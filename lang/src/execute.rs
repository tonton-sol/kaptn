use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use spl_tlv_account_resolution::state::ExtraAccountMetaList;
use spl_token_2022::extension::transfer_hook::TransferHookAccount;
use spl_token_2022::extension::{BaseStateWithExtensions, StateWithExtensions};
use spl_token_2022::state::Account;
use spl_transfer_hook_interface::error::TransferHookError;
use spl_transfer_hook_interface::get_extra_account_metas_address;
use spl_transfer_hook_interface::instruction::{ExecuteInstruction, TransferHookInstruction};

use crate::{context::ExtraMetas, context::TransferContext};

pub fn process_execute<'info, E: ExtraMetas<'info>>(
    program_id: &Pubkey,
    accounts: &[AccountInfo<'info>],
    amount: u64,
    process_transfer: fn(TransferContext<E>) -> ProgramResult,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let source_account_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let destination_account_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let extra_account_metas_info = next_account_info(account_info_iter)?;

    check_token_account_is_transferring(source_account_info)?;
    check_token_account_is_transferring(destination_account_info)?;

    let expected_validation_address = get_extra_account_metas_address(mint_info.key, program_id);
    if expected_validation_address != *extra_account_metas_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let data = extra_account_metas_info.try_borrow_data()?;
    if !data.is_empty() {
        ExtraAccountMetaList::check_account_infos::<ExecuteInstruction>(
            accounts,
            &TransferHookInstruction::Execute { amount }.pack(),
            program_id,
            &data,
        )?;
    }

    let extra_metas = E::from_accounts(accounts)?;

    let ctx = TransferContext {
        program_id,
        source_account: source_account_info,
        mint: mint_info,
        destination_account: destination_account_info,
        authority: authority_info,
        extra_account_metas: extra_account_metas_info,
        amount,
        extra_metas,
    };

    process_transfer(ctx)
}

fn check_token_account_is_transferring(account_info: &AccountInfo) -> Result<(), ProgramError> {
    let account_data = account_info.try_borrow_data()?;
    let token_account = StateWithExtensions::<Account>::unpack(&account_data)?;
    let extension = token_account.get_extension::<TransferHookAccount>()?;
    if bool::from(extension.transferring) {
        Ok(())
    } else {
        Err(TransferHookError::ProgramCalledOutsideOfTransfer.into())
    }
}
