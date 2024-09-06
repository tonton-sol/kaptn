use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn transfer_hook(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    let expanded = quote! {
        #input_fn

        use solana_program::entrypoint;
        entrypoint!(process_instruction);

        pub fn process_instruction(
            program_id: &solana_program::pubkey::Pubkey,
            accounts: &[solana_program::account_info::AccountInfo],
            instruction_data: &[u8],
        ) -> solana_program::entrypoint::ProgramResult {
            kaptn_lang::__process_instruction(program_id, accounts, instruction_data, #fn_name)
        }
    };

    TokenStream::from(expanded)
}
