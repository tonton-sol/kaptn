#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use kaptn_lang::prelude::*;
const ID_ARRAY: [u8; 32] = kaptn_lang::prelude::Pubkey::new_from_array([
        63u8,
        139u8,
        208u8,
        147u8,
        20u8,
        28u8,
        123u8,
        69u8,
        87u8,
        43u8,
        207u8,
        47u8,
        88u8,
        240u8,
        224u8,
        92u8,
        201u8,
        31u8,
        58u8,
        131u8,
        110u8,
        241u8,
        61u8,
        146u8,
        243u8,
        159u8,
        68u8,
        45u8,
        0u8,
        148u8,
        139u8,
        78u8,
    ])
    .to_bytes();
/// The static ID
pub static ID: kaptn_lang::prelude::Pubkey = kaptn_lang::prelude::Pubkey::new_from_array(
    ID_ARRAY,
);
/// Const version of the ID
pub const ID_CONST: kaptn_lang::prelude::Pubkey = kaptn_lang::prelude::Pubkey::new_from_array(
    ID_ARRAY,
);
/// Confirms that a given pubkey is equivalent to the ID
pub fn check_id(id: &kaptn_lang::prelude::Pubkey) -> bool {
    id.to_bytes() == ID_ARRAY
}
/// Returns the ID
pub fn id() -> kaptn_lang::prelude::Pubkey {
    kaptn_lang::prelude::Pubkey::new_from_array(ID_ARRAY)
}
/// Const version of the ID function
pub const fn id_const() -> kaptn_lang::prelude::Pubkey {
    ID_CONST
}
const MINT_ARRAY: [u8; 32] = kaptn_lang::prelude::Pubkey::new_from_array([
        214u8,
        18u8,
        251u8,
        103u8,
        28u8,
        91u8,
        33u8,
        14u8,
        118u8,
        41u8,
        148u8,
        118u8,
        29u8,
        218u8,
        68u8,
        180u8,
        171u8,
        28u8,
        55u8,
        221u8,
        238u8,
        183u8,
        180u8,
        39u8,
        59u8,
        33u8,
        105u8,
        141u8,
        18u8,
        191u8,
        189u8,
        89u8,
    ])
    .to_bytes();
/// The static ID
pub static MINT: kaptn_lang::prelude::Pubkey = kaptn_lang::prelude::Pubkey::new_from_array(
    MINT_ARRAY,
);
/// Const version of the ID
pub const MINT_CONST: kaptn_lang::prelude::Pubkey = kaptn_lang::prelude::Pubkey::new_from_array(
    MINT_ARRAY,
);
/// Confirms that a given pubkey is equivalent to the ID
pub fn check_mint(id: &kaptn_lang::prelude::Pubkey) -> bool {
    id.to_bytes() == MINT_ARRAY
}
/// Returns the ID
pub fn mint() -> kaptn_lang::prelude::Pubkey {
    kaptn_lang::prelude::Pubkey::new_from_array(MINT_ARRAY)
}
/// Const version of the ID function
pub const fn mint_const() -> kaptn_lang::prelude::Pubkey {
    MINT_CONST
}
pub fn ahoy_world(ctx: TransferContext<MyExtraMetas>) -> ProgramResult {
    ::solana_program::log::sol_log(
        &::alloc::__export::must_use({
            let res = ::alloc::fmt::format(
                format_args!("Ahoy from transfer-hook program: {0:?}", ctx.program_id),
            );
            res
        }),
    );
    Ok(())
}
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) = unsafe {
        ::solana_program::entrypoint::deserialize(input)
    };
    match __process_instruction(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
pub fn __process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    process_instruction(program_id, accounts, instruction_data, ahoy_world)
}
fn process_instruction<'info, E: ExtraMetas<'info>>(
    program_id: &Pubkey,
    accounts: &[AccountInfo<'info>],
    instruction_data: &[u8],
    process_transfer: fn(TransferContext<'_, 'info, E>) -> ProgramResult,
) -> ProgramResult {
    let instruction = TransferHookInstruction::unpack(instruction_data)?;
    match instruction {
        TransferHookInstruction::Execute { amount } => {
            ::solana_program::log::sol_log("Instruction: Execute");
            process_execute(program_id, accounts, amount, process_transfer)
        }
        TransferHookInstruction::InitializeExtraAccountMetaList {
            extra_account_metas: _,
        } => {
            ::solana_program::log::sol_log(
                "Instruction: InitializeExtraAccountMetaList",
            );
            let user_extra_metas = E::to_extra_account_metas();
            process_initialize_extra_account_meta_list(
                program_id,
                accounts,
                &user_extra_metas,
            )
        }
        TransferHookInstruction::UpdateExtraAccountMetaList {
            extra_account_metas: _,
        } => {
            ::solana_program::log::sol_log("Instruction: UpdateExtraAccountMetaList");
            let user_extra_metas = E::to_extra_account_metas();
            process_update_extra_account_meta_list(
                program_id,
                accounts,
                &user_extra_metas,
            )
        }
    }
}
pub fn process_execute<'info, E: ExtraMetas<'info>>(
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
    let expected_validation_address = get_extra_account_metas_address(
        mint_info.key,
        program_id,
    );
    if expected_validation_address != *extra_account_metas_info.key {
        return Err(ProgramError::InvalidSeeds);
    }
    let data = extra_account_metas_info.try_borrow_data()?;
    if !data.is_empty() {
        ExtraAccountMetaList::check_account_infos::<
            ExecuteInstruction,
        >(
            accounts,
            &TransferHookInstruction::Execute {
                amount,
            }
                .pack(),
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
fn check_token_account_is_transferring(
    account_info: &AccountInfo,
) -> Result<(), ProgramError> {
    let account_data = account_info.try_borrow_data()?;
    let token_account = StateWithExtensions::<Account>::unpack(&account_data)?;
    let extension = token_account.get_extension::<TransferHookAccount>()?;
    if bool::from(extension.transferring) {
        Ok(())
    } else {
        Err(TransferHookError::ProgramCalledOutsideOfTransfer.into())
    }
}
pub fn process_initialize_extra_account_meta_list(
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
    let (expected_validation_address, bump_seed) = get_extra_account_metas_address_and_bump_seed(
        mint_info.key,
        program_id,
    );
    if expected_validation_address != *extra_account_metas_info.key {
        return Err(ProgramError::InvalidSeeds);
    }
    let bump_seed = [bump_seed];
    let signer_seeds = collect_extra_account_metas_signer_seeds(
        mint_info.key,
        &bump_seed,
    );
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
pub fn process_update_extra_account_meta_list(
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
    let expected_validation_address = get_extra_account_metas_address(
        mint_info.key,
        program_id,
    );
    if expected_validation_address != *extra_account_metas_info.key {
        return Err(ProgramError::InvalidSeeds);
    }
    let min_account_size = ExtraAccountMetaList::size_of(0)?;
    let original_account_size = extra_account_metas_info.data_len();
    if program_id != extra_account_metas_info.owner
        || original_account_size < min_account_size
    {
        return Err(ProgramError::UninitializedAccount);
    }
    let length = extra_account_metas.len();
    let account_size = ExtraAccountMetaList::size_of(length)?;
    if account_size >= original_account_size {
        extra_account_metas_info.realloc(account_size, false)?;
        let mut data = extra_account_metas_info.try_borrow_mut_data()?;
        ExtraAccountMetaList::update::<
            ExecuteInstruction,
        >(&mut data, extra_account_metas)?;
    } else {
        {
            let mut data = extra_account_metas_info.try_borrow_mut_data()?;
            ExtraAccountMetaList::update::<
                ExecuteInstruction,
            >(&mut data, extra_account_metas)?;
        }
        extra_account_metas_info.realloc(account_size, false)?;
    }
    Ok(())
}
pub struct MyExtraMetas {}
impl ExtraMetas<'_> for MyExtraMetas {
    fn from_accounts(accounts: &[AccountInfo]) -> Result<Self, ProgramError> {
        let mut iter = accounts.iter().skip(5);
        Ok(Self {})
    }
    fn to_extra_account_metas() -> Vec<ExtraAccountMeta> {
        ::alloc::vec::Vec::new()
    }
}
