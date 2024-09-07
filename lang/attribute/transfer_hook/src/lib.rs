use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn transfer_hook(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    let expanded = quote! {
        #input_fn

        kaptn_lang::solana_program::entrypoint!(__process_instruction);

        pub fn __process_instruction(
            program_id: &kaptn_lang::solana_program::Pubkey,
            accounts: &[kaptn_lang::solana_program::AccountInfo],
            instruction_data: &[u8],
        ) -> kaptn_lang::solana_program::entrypoint::ProgramResult {
            kaptn_lang::processor::process_instruction(program_id, accounts, instruction_data, #fn_name)
        }


    };

    TokenStream::from(expanded)
}
