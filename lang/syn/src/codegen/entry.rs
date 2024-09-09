use quote::quote;
use crate::TransferHookInput;

pub fn generate(program: &TransferHookInput) -> proc_macro2::TokenStream {
    let fn_name = &program.fn_name;
    quote! {
        kaptn_lang::solana_program::entrypoint!(__process_instruction);

        pub fn __process_instruction(
            program_id: &kaptn_lang::solana_program::Pubkey,
            accounts: &[kaptn_lang::solana_program::AccountInfo],
            instruction_data: &[u8],
        ) -> kaptn_lang::solana_program::entrypoint::ProgramResult {
            process_instruction(program_id, accounts, instruction_data, #fn_name)
        }
    }
}