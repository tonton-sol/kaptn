use crate::TransferHookInput;
use quote::quote;

pub fn generate(_program: &TransferHookInput) -> proc_macro2::TokenStream {
    quote! {
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



    }
}
