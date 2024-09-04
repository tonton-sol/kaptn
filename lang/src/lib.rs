use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};
use spl_tlv_account_resolution::{account::ExtraAccountMeta, state::ExtraAccountMetaList};
use spl_token_2022::{
    extension::{transfer_hook::TransferHookAccount, BaseStateWithExtensions, StateWithExtensions},
    state::{Account, Mint},
};
use spl_transfer_hook_interface::{
    collect_extra_account_metas_signer_seeds,
    error::TransferHookError,
    get_extra_account_metas_address, get_extra_account_metas_address_and_bump_seed,
    instruction::{ExecuteInstruction, TransferHookInstruction},
};

pub use kaptn_macros::{declare_id, declare_mint, transfer_hook, ExtraMetas};

pub struct TransferContext<'a, 'info, E = ()> {
    pub program_id: &'a Pubkey,
    pub source_account: &'a AccountInfo<'info>,
    pub mint: &'a AccountInfo<'info>,
    pub destination_account: &'a AccountInfo<'info>,
    pub authority: &'a AccountInfo<'info>,
    pub extra_account_metas: &'a AccountInfo<'info>,
    pub amount: u64,
    pub extra_metas: E,
}

pub trait ExtraMetas<'info>: Sized {
    fn from_accounts(accounts: &[AccountInfo<'info>]) -> Result<Self, ProgramError>;
    fn to_extra_account_metas() -> Vec<ExtraAccountMeta>;
}

impl<'info> ExtraMetas<'info> for () {
    fn from_accounts(_accounts: &[AccountInfo<'info>]) -> Result<Self, ProgramError> {
        Ok(())
    }

    fn to_extra_account_metas() -> Vec<ExtraAccountMeta> {
        vec![]
    }
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

fn process_execute<'info, E: ExtraMetas<'info>>(
    program_id: &Pubkey,
    accounts: &[AccountInfo<'info>],
    amount: u64,
    process_transfer: fn(TransferContext<'_, 'info, E>) -> ProgramResult,
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

fn process_initialize_extra_account_meta_list(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    extra_account_metas: &[ExtraAccountMeta],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let extra_account_metas_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let _system_program_info = next_account_info(account_info_iter)?;

    let mint_data = mint_info.try_borrow_data()?;
    let mint = StateWithExtensions::<Mint>::unpack(&mint_data)?;
    let mint_authority = mint
        .base
        .mint_authority
        .ok_or(TransferHookError::MintHasNoMintAuthority)?;

    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if *authority_info.key != mint_authority {
        return Err(TransferHookError::IncorrectMintAuthority.into());
    }

    let (expected_validation_address, bump_seed) =
        get_extra_account_metas_address_and_bump_seed(mint_info.key, program_id);
    if expected_validation_address != *extra_account_metas_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let bump_seed = [bump_seed];
    let signer_seeds = collect_extra_account_metas_signer_seeds(mint_info.key, &bump_seed);
    let length = extra_account_metas.len();
    let account_size = ExtraAccountMetaList::size_of(length)?;
    invoke_signed(
        &system_instruction::allocate(extra_account_metas_info.key, account_size as u64),
        &[extra_account_metas_info.clone()],
        &[&signer_seeds],
    )?;
    invoke_signed(
        &system_instruction::assign(extra_account_metas_info.key, program_id),
        &[extra_account_metas_info.clone()],
        &[&signer_seeds],
    )?;

    let mut data = extra_account_metas_info.try_borrow_mut_data()?;
    ExtraAccountMetaList::init::<ExecuteInstruction>(&mut data, extra_account_metas)?;

    Ok(())
}

fn process_update_extra_account_meta_list(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    extra_account_metas: &[ExtraAccountMeta],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let extra_account_metas_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;

    let mint_data = mint_info.try_borrow_data()?;
    let mint = StateWithExtensions::<Mint>::unpack(&mint_data)?;
    let mint_authority = mint
        .base
        .mint_authority
        .ok_or(TransferHookError::MintHasNoMintAuthority)?;

    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if *authority_info.key != mint_authority {
        return Err(TransferHookError::IncorrectMintAuthority.into());
    }

    let expected_validation_address = get_extra_account_metas_address(mint_info.key, program_id);
    if expected_validation_address != *extra_account_metas_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let min_account_size = ExtraAccountMetaList::size_of(0)?;
    let original_account_size = extra_account_metas_info.data_len();
    if program_id != extra_account_metas_info.owner || original_account_size < min_account_size {
        return Err(ProgramError::UninitializedAccount);
    }

    let length = extra_account_metas.len();
    msg!("length: {}", length);
    let account_size = ExtraAccountMetaList::size_of(length)?;
    if account_size >= original_account_size {
        extra_account_metas_info.realloc(account_size, false)?;
        let mut data = extra_account_metas_info.try_borrow_mut_data()?;
        ExtraAccountMetaList::update::<ExecuteInstruction>(&mut data, extra_account_metas)?;
    } else {
        {
            let mut data = extra_account_metas_info.try_borrow_mut_data()?;
            ExtraAccountMetaList::update::<ExecuteInstruction>(&mut data, extra_account_metas)?;
        }
        extra_account_metas_info.realloc(account_size, false)?;
    }

    Ok(())
}

#[doc(hidden)]
pub fn __process_instruction<'info, E: ExtraMetas<'info>>(
    program_id: &Pubkey,
    accounts: &[AccountInfo<'info>],
    instruction_data: &[u8],
    process_transfer: fn(TransferContext<E>) -> ProgramResult,
) -> ProgramResult {
    let instruction = TransferHookInstruction::unpack(instruction_data)?;

    match instruction {
        TransferHookInstruction::Execute { amount } => {
            msg!("Instruction: Execute");
            process_execute(program_id, accounts, amount, process_transfer)
        }
        TransferHookInstruction::InitializeExtraAccountMetaList {
            extra_account_metas: _,
        } => {
            msg!("Instruction: InitializeExtraAccountMetaList");

            // Directly use the extra metas from the user's struct, bypassing accounts
            let user_extra_metas = E::to_extra_account_metas(); // Generate metas from user struct
            process_initialize_extra_account_meta_list(program_id, accounts, &user_extra_metas)
        }
        TransferHookInstruction::UpdateExtraAccountMetaList {
            extra_account_metas: _,
        } => {
            msg!("Instruction: UpdateExtraAccountMetaList");

            // Directly use the extra metas from the user's struct, bypassing accounts
            let user_extra_metas = E::to_extra_account_metas(); // Generate metas from user struct
            process_update_extra_account_meta_list(program_id, accounts, &user_extra_metas)
        }
    }
}

/// The prelude contains all commonly used components of the crate.
/// All programs should include it via `use kaptn_lang::prelude::*;`.
pub mod prelude {
    pub use super::{declare_id, declare_mint, transfer_hook, ExtraMetas, TransferContext};
    pub use solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        epoch_schedule::EpochSchedule,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        stake_history::StakeHistory,
        system_instruction,
        sysvar::Sysvar,
    };
    pub use spl_tlv_account_resolution::account::ExtraAccountMeta;
    pub use spl_transfer_hook_interface::error::TransferHookError;
    pub use std::str::FromStr;
}
