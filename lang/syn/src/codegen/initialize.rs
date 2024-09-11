use crate::TransferHookInput;
use quote::quote;

pub fn generate(_program: &TransferHookInput) -> proc_macro2::TokenStream {
    quote! {
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

            if !check_mint(mint_info.key) {
                return Err(ProgramError::InvalidArgument);
            }

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


    }
}
