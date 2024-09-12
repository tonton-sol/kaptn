pub use kaptn_attribute_transfer_hook::transfer_hook;
pub use kaptn_derive_extrametas::ExtraMetas;
pub use kaptn_macros::{declare_id, declare_mint};

pub use solana_program;

pub mod context;

/// The prelude contains all commonly used components of the crate.
/// All programs should include it via `use kaptn_lang::prelude::*;`.
pub mod prelude {

    pub use super::{
        context::{ExtraMetas, TransferContext},
        declare_id, declare_mint, transfer_hook,
    };

    pub use solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        epoch_schedule::EpochSchedule,
        msg,
        program::invoke_signed,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        stake_history::StakeHistory,
        system_instruction,
        sysvar::Sysvar,
    };

    pub use spl_tlv_account_resolution::{
        account::ExtraAccountMeta, error::AccountResolutionError, seeds::Seed,
        state::ExtraAccountMetaList,
    };
    pub use spl_token_2022;
    pub use spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount, BaseStateWithExtensions,
            BaseStateWithExtensionsMut, ExtensionType, StateWithExtensions, StateWithExtensionsMut,
        },
        state::{Account, AccountState, Mint},
    };
    pub use spl_transfer_hook_interface::{
        collect_extra_account_metas_signer_seeds,
        error::TransferHookError,
        get_extra_account_metas_address, get_extra_account_metas_address_and_bump_seed,
        instruction::{
            execute_with_extra_account_metas, initialize_extra_account_meta_list,
            update_extra_account_meta_list, ExecuteInstruction, TransferHookInstruction,
        },
        onchain,
    };
}
