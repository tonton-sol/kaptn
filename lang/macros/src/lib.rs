extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result},
    Expr, LitByte, LitStr,
};

fn parse_id(input: ParseStream) -> Result<proc_macro2::TokenStream> {
    let pubkey_type = quote! { kaptn_lang::solana_program::pubkey::Pubkey };

    let id = if input.peek(syn::LitStr) {
        let id_literal: LitStr = input.parse()?;
        parse_pubkey(&id_literal, &pubkey_type)?
    } else {
        let expr: Expr = input.parse()?;
        quote! { #expr }
    };

    if !input.is_empty() {
        let stream: proc_macro2::TokenStream = input.parse()?;
        return Err(syn::Error::new_spanned(stream, "unexpected token"));
    }
    Ok(id)
}

fn generate_id_tokens(id: &proc_macro2::TokenStream, name: &str) -> proc_macro2::TokenStream {
    let pubkey_type = quote! { kaptn_lang::solana_program::pubkey::Pubkey };
    let name_upper = syn::Ident::new(&name.to_uppercase(), Span::call_site());
    let name_lower = syn::Ident::new(name, Span::call_site());
    let check_fn = syn::Ident::new(&format!("check_{}", name), Span::call_site());
    let const_fn = syn::Ident::new(&format!("{}_const", name), Span::call_site());
    let name_upper_const =
        syn::Ident::new(&format!("{}_CONST", name.to_uppercase()), Span::call_site());
    let array_const = syn::Ident::new(&format!("{}_ARRAY", name.to_uppercase()), Span::call_site());

    quote! {
        const #array_const: [u8; 32] = #id.to_bytes();

        /// The static ID
        pub static #name_upper: #pubkey_type = #pubkey_type::new_from_array(#array_const);

        /// Const version of the ID
        pub const #name_upper_const: #pubkey_type = #pubkey_type::new_from_array(#array_const);

        /// Confirms that a given pubkey is equivalent to the ID
        pub fn #check_fn(id: &#pubkey_type) -> bool {
            id.to_bytes() == #array_const
        }

        /// Returns the ID
        pub fn #name_lower() -> #pubkey_type {
            #pubkey_type::new_from_array(#array_const)
        }

        /// Const version of the ID function
        pub const fn #const_fn() -> #pubkey_type {
            #name_upper_const
        }
    }
}

fn parse_pubkey(
    id_literal: &LitStr,
    pubkey_type: &proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream> {
    let id_vec = bs58::decode(id_literal.value())
        .into_vec()
        .map_err(|_| syn::Error::new_spanned(id_literal, "failed to decode base58 string"))?;
    let id_array = <[u8; 32]>::try_from(<&[u8]>::clone(&&id_vec[..])).map_err(|_| {
        syn::Error::new_spanned(
            id_literal,
            format!("pubkey array is not 32 bytes long: len={}", id_vec.len()),
        )
    })?;
    let bytes = id_array.iter().map(|b| LitByte::new(*b, Span::call_site()));
    Ok(quote! {
        #pubkey_type::new_from_array(
            [#(#bytes,)*]
        )
    })
}

struct Id(proc_macro2::TokenStream);

impl Parse for Id {
    fn parse(input: ParseStream) -> Result<Self> {
        parse_id(input).map(Self)
    }
}

#[proc_macro]
pub fn declare_id(input: TokenStream) -> TokenStream {
    let id = syn::parse_macro_input!(input as Id);
    TokenStream::from(generate_id_tokens(&id.0, "id"))
}

#[proc_macro]
pub fn declare_mint(input: TokenStream) -> TokenStream {
    let id = syn::parse_macro_input!(input as Id);
    TokenStream::from(generate_id_tokens(&id.0, "mint"))
}
