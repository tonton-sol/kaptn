use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Attribute, Data, DeriveInput, Expr, Fields, GenericParam, Lit, Meta, NestedMeta, Token,
};

struct Seeds(Punctuated<Expr, Token![,]>);

impl Parse for Seeds {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::bracketed!(content in input);
        Ok(Seeds(content.parse_terminated(Expr::parse)?))
    }
}

#[proc_macro_derive(ExtraMetas, attributes(meta))]
pub fn derive_extra_metas(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extract lifetimes
    let lifetimes: Vec<_> = input
        .generics
        .params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Lifetime(lt) = param {
                Some(lt.lifetime.clone())
            } else {
                None
            }
        })
        .collect();

    // Determine if the 'info lifetime is present
    let has_info_lifetime = lifetimes.iter().any(|lt| lt.to_string() == "'info");

    // Extract fields
    let fields = if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields) = &data_struct.fields {
            fields.named.iter().collect::<Vec<_>>()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    // Process account metas: pubkey or seeds
    let account_metas = fields
        .iter()
        .filter_map(|f| {
            let ident = f.ident.as_ref()?;
            let meta_attr = f.attrs.iter().find(|attr| attr.path.is_ident("meta"))?;
            parse_meta_attribute(ident, meta_attr).ok()
        })
        .collect::<Vec<_>>();

    let field_names = fields
        .iter()
        .filter_map(|f| f.ident.as_ref())
        .collect::<Vec<_>>();

    // Generate from_accounts method
    let from_accounts = if has_info_lifetime {
        quote! {
            fn from_accounts(accounts: &[AccountInfo<'info>]) -> Result<Self, ProgramError> {
                let mut iter = accounts.iter().skip(5); // Adjust skip as needed
                Ok(Self {
                    #(#field_names: iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?.clone(),)*
                })
            }
        }
    } else {
        quote! {
            fn from_accounts(accounts: &[AccountInfo]) -> Result<Self, ProgramError> {
                let mut iter = accounts.iter().skip(5); // Skip till the extra account metas
                Ok(Self {
                    #(#field_names: iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?.clone(),)*
                })
            }
        }
    };

    // Generate trait implementation for ExtraMetas
    let gen = if has_info_lifetime {
        quote! {
            impl<#(#lifetimes),*> ExtraMetas<#(#lifetimes),*> for #name<#(#lifetimes),*> {
                #from_accounts

                fn to_extra_account_metas() -> Vec<ExtraAccountMeta> {
                    vec![
                        #(#account_metas),*
                    ]
                }
            }
        }
    } else if !lifetimes.is_empty() {
        quote! {
            impl<#(#lifetimes),*> ExtraMetas<'_, #(#lifetimes),*> for #name<#(#lifetimes),*> {
                #from_accounts

                fn to_extra_account_metas() -> Vec<ExtraAccountMeta> {
                    vec![
                        #(#account_metas),*
                    ]
                }
            }
        }
    } else {
        quote! {
            impl ExtraMetas<'_> for #name {
                #from_accounts

                fn to_extra_account_metas() -> Vec<ExtraAccountMeta> {
                    vec![
                        #(#account_metas),*
                    ]
                }
            }
        }
    };

    gen.into()
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
                            if let Lit::Str(lit) = &nv.lit {
                                pubkey = Some(lit.value());
                            }
                        }
                        Some("signer") => {
                            if let Lit::Bool(lit) = &nv.lit {
                                signer = lit.value();
                            }
                        }
                        Some("writable") => {
                            if let Lit::Bool(lit) = &nv.lit {
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
