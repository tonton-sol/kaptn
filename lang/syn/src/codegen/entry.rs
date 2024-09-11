use crate::TransferHookInput;
use quote::quote;

pub fn generate(program: &TransferHookInput) -> proc_macro2::TokenStream {
    let fn_name = &program.fn_name;
    quote! {
        kaptn_lang::solana_program::entrypoint!(__process_instruction);

        pub fn __process_instruction(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            instruction_data: &[u8],
        ) -> ProgramResult {

            if !check_mint(program_id) {
                return Err(ProgramError::InvalidArgument);
            }

            process_instruction(program_id, accounts, instruction_data, #fn_name)
        }
    }
}
