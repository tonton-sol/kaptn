use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn declare_id(input: TokenStream) -> TokenStream {
    let id_str = parse_macro_input!(input as LitStr);
    let id_bytes: Vec<u8> = id_str.value().as_bytes().iter().take(32).copied().collect();

    if id_bytes.len() != 32 {
        panic!("The provided ID must be exactly 32 bytes long.");
    }

    let id_array = quote! {
        [#(#id_bytes),*]
    };

    let expanded = quote! {
        pub static ID: kaptn_lang::prelude::Pubkey = kaptn_lang::prelude::Pubkey::new_from_array(#id_array);
        pub const ID_CONST: kaptn_lang::prelude::Pubkey = kaptn_lang::prelude::Pubkey::new_from_array(#id_array);

        pub fn check_id(id: &kaptn_lang::prelude::Pubkey) -> bool {
            id == &ID
        }

        pub fn id() -> kaptn_lang::prelude::Pubkey {
            ID
        }

        pub const fn id_const() -> kaptn_lang::prelude::Pubkey {
            ID_CONST
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn declare_mint(input: TokenStream) -> TokenStream {
    let mint_str = parse_macro_input!(input as LitStr);
    let mint_bytes: Vec<u8> = mint_str
        .value()
        .as_bytes()
        .iter()
        .take(32)
        .copied()
        .collect();

    if mint_bytes.len() != 32 {
        panic!("The provided mint must be exactly 32 bytes long.");
    }

    let mint_array = quote! {
        [#(#mint_bytes),*]
    };

    let expanded = quote! {
        pub static MINT: kaptn_lang::prelude::Pubkey = kaptn_lang::prelude::Pubkey::new_from_array(#mint_array);
        pub const MINT_CONST: kaptn_lang::prelude::Pubkey = kaptn_lang::prelude::Pubkey::new_from_array(#mint_array);

        pub fn check_mint(mint: &kaptn_lang::prelude::Pubkey) -> bool {
            mint == &MINT
        }

        pub fn mint() -> kaptn_lang::prelude::Pubkey {
            MINT
        }

        pub const fn mint_const() -> kaptn_lang::prelude::Pubkey {
            MINT_CONST
        }
    };

    TokenStream::from(expanded)
}
