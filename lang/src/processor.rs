use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};
use spl_transfer_hook_interface::instruction::TransferHookInstruction;

use crate::{
    context::{ExtraMetas, TransferContext},
    execute, initialize,
    update::process_update_extra_account_meta_list,
};

pub fn process_instruction<'info, E: ExtraMetas<'info>>(
    program_id: &Pubkey,
    accounts: &[AccountInfo<'info>],
    instruction_data: &[u8],
    process_transfer: fn(TransferContext<'_, 'info, E>) -> ProgramResult,
) -> ProgramResult {
    let instruction = TransferHookInstruction::unpack(instruction_data)?;

    match instruction {
        TransferHookInstruction::Execute { amount } => {
            msg!("Instruction: Execute");
            execute::process_execute(program_id, accounts, amount, process_transfer)
        }
        TransferHookInstruction::InitializeExtraAccountMetaList {
            extra_account_metas: _,
        } => {
            msg!("Instruction: InitializeExtraAccountMetaList");
            let user_extra_metas = E::to_extra_account_metas();
            initialize::process_initialize_extra_account_meta_list(
                program_id,
                accounts,
                &user_extra_metas,
            )
        }
        TransferHookInstruction::UpdateExtraAccountMetaList {
            extra_account_metas: _,
        } => {
            msg!("Instruction: UpdateExtraAccountMetaList");
            let user_extra_metas = E::to_extra_account_metas();
            process_update_extra_account_meta_list(program_id, accounts, &user_extra_metas)
        }
    }
}
