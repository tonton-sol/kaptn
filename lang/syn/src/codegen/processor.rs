use crate::TransferHookInput;
use quote::quote;

pub fn generate(_program: &TransferHookInput) -> proc_macro2::TokenStream {
    quote! {
        fn process_instruction<'info, E: kaptn_lang::context::ExtraMetas<'info>>(
            program_id: &kaptn_lang::solana_program::Pubkey,
            accounts: &[kaptn_lang::solana_program::AccountInfo<'info>],
            instruction_data: &[u8],
            process_transfer: fn(kaptn_lang::context::TransferContext<'_, 'info, E>) -> kaptn_lang::solana_program::entrypoint::ProgramResult,
        ) -> kaptn_lang::solana_program::entrypoint::ProgramResult {
            let instruction = kaptn_lang::spl_transfer_hook_interface::TransferHookInstruction::unpack(instruction_data)?;

            match instruction {
                kaptn_lang::spl_transfer_hook_interface::TransferHookInstruction::Execute { amount } => {
                    kaptn_lang::solana_program::msg!("Instruction: Execute");
                    kaptn_lang::execute::process_execute(program_id, accounts, amount, process_transfer)
                }
                kaptn_lang::spl_transfer_hook_interface::TransferHookInstruction::InitializeExtraAccountMetaList { extra_account_metas: _ } => {
                    kaptn_lang::solana_program::msg!("Instruction: InitializeExtraAccountMetaList");
                    let user_extra_metas = E::to_extra_account_metas();
                    kaptn_lang::initialize::process_initialize_extra_account_meta_list(
                        program_id,
                        accounts,
                        &user_extra_metas,
                    )
                }
                kaptn_lang::spl_transfer_hook_interface::TransferHookInstruction::UpdateExtraAccountMetaList { extra_account_metas: _ } => {
                    kaptn_lang::solana_program::msg!("Instruction: UpdateExtraAccountMetaList");
                    let user_extra_metas = E::to_extra_account_metas();
                    kaptn_lang::update::process_update_extra_account_meta_list(program_id, accounts, &user_extra_metas)
                }
            }
        }
    }
}
