pub use kaptn_attribute_transfer_hook::transfer_hook;
pub use kaptn_derive_extrametas::ExtraMetas;
pub use kaptn_macros::{declare_id, declare_mint};

pub mod solana_program {
    pub use solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        program::invoke_signed,
        program_error::ProgramError,
        pubkey::Pubkey,
        system_instruction,
    };
}

pub mod spl_token {
    pub use spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount, BaseStateWithExtensions, StateWithExtensions,
        },
        state::{Account, Mint},
    };
}

pub mod spl_tlv_account_resolution {
    pub use spl_tlv_account_resolution::{account::ExtraAccountMeta, state::ExtraAccountMetaList};
}

pub mod spl_transfer_hook_interface {
    pub use spl_transfer_hook_interface::{
        collect_extra_account_metas_signer_seeds,
        error::TransferHookError,
        get_extra_account_metas_address, get_extra_account_metas_address_and_bump_seed,
        instruction::{ExecuteInstruction, TransferHookInstruction},
    };
}

pub mod context;
pub mod execute;
pub mod initialize;
pub mod processor;
pub mod update;

/// The prelude contains all commonly used components of the crate.
/// All programs should include it via `use kaptn_lang::prelude::*;`.
pub mod prelude {
    pub use super::context::{ExtraMetas, TransferContext};
    pub use super::{declare_id, declare_mint, transfer_hook};
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
