pub use kaptn_derive_extrametas::ExtraMetas;
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};
use spl_tlv_account_resolution::account::ExtraAccountMeta;

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
