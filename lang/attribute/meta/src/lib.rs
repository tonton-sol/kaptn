use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Attribute, Expr, Meta, NestedMeta, Token};

struct Seeds(Punctuated<Expr, Token![,]>);

impl syn::parse::Parse for Seeds {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::bracketed!(content in input);
        Ok(Seeds(content.parse_terminated(Expr::parse)?))
    }
}

#[proc_macro_attribute]
pub fn meta(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemStruct);
    let struct_name = &input.ident;

    // Extract the fields from the struct
    let fields = match &input.fields {
        syn::Fields::Named(named) => &named.named,
        _ => {
            return syn::Error::new_spanned(input, "Expected a struct with named fields")
                .to_compile_error()
                .into();
        }
    };

    let field_metas = fields.iter().map(|field| {
        let field_name = &field.ident;
        let meta_attr = field.attrs.iter().find(|attr| attr.path.is_ident("meta"));

        if let Some(attr) = meta_attr {
            match parse_meta_attribute(field_name.as_ref().unwrap(), attr) {
                Ok(meta) => meta,
                Err(err) => return err.to_compile_error(),
            }
        } else {
            quote! {}
        }
    });

    // Generate the struct with the field metadata logic embedded
    let expanded = quote! {
        #input

        impl #struct_name {
            pub fn get_account_metas() -> Vec<ExtraAccountMeta> {
                vec![
                    #(#field_metas),*
                ]
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_meta_attribute(
    _ident: &syn::Ident,
    attr: &Attribute,
) -> syn::Result<proc_macro2::TokenStream> {
    let meta = attr.parse_meta()?;
    if let Meta::List(list) = meta {
        let mut pubkey = None;
        let mut signer = false;
        let mut writable = false;
        let mut seeds = None;

        for nested in list.nested.iter() {
            match nested {
                NestedMeta::Meta(Meta::NameValue(nv)) => {
                    match nv.path.get_ident().map(|i| i.to_string()).as_deref() {
                        Some("pubkey") => {
                            if let syn::Lit::Str(lit) = &nv.lit {
                                pubkey = Some(lit.value());
                            }
                        }
                        Some("signer") => {
                            if let syn::Lit::Bool(lit) = &nv.lit {
                                signer = lit.value();
                            }
                        }
                        Some("writable") => {
                            if let syn::Lit::Bool(lit) = &nv.lit {
                                writable = lit.value();
                            }
                        }
                        _ => {}
                    }
                }
                NestedMeta::Meta(Meta::List(list)) if list.path.is_ident("seeds") => {
                    seeds = Some(syn::parse2::<Seeds>(list.nested.to_token_stream())?);
                }
                _ => {}
            }
        }

        if pubkey.is_some() && seeds.is_some() {
            return Err(syn::Error::new_spanned(
                attr,
                "Cannot specify both pubkey and seeds",
            ));
        }

        if pubkey.is_none() && seeds.is_none() {
            return Err(syn::Error::new_spanned(
                attr,
                "Must specify either pubkey or seeds",
            ));
        }

        // Generate appropriate ExtraAccountMeta based on pubkey or seeds
        Ok(if let Some(pk) = pubkey {
            quote! {
                ExtraAccountMeta::new_with_pubkey(&#pk.parse().unwrap(), #signer, #writable).unwrap()
            }
        } else if let Some(Seeds(seeds)) = seeds {
            let seed_exprs = seeds.iter();
            quote! {
                ExtraAccountMeta::new_external_pda_with_seeds(
                    0, // Associated token program index, adjust as needed
                    &[#(#seed_exprs),*],
                    #signer,
                    #writable
                ).unwrap()
            }
        } else {
            unreachable!()
        })
    } else {
        Err(syn::Error::new_spanned(
            attr,
            "Expected list-style attribute",
        ))
    }
}
