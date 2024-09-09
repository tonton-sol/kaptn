pub use kaptn_attribute_transfer_hook::transfer_hook;
pub use kaptn_derive_extrametas::ExtraMetas;
pub use kaptn_macros::{declare_id, declare_mint};

pub use solana_program;

pub mod context;
pub mod execute;
pub mod initialize;
pub mod update;

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
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        stake_history::StakeHistory,
        system_instruction,
        sysvar::Sysvar,
    };

    pub use spl_tlv_account_resolution::account::ExtraAccountMeta;
    pub use spl_transfer_hook_interface::{
        error::TransferHookError, instruction::TransferHookInstruction,
    };
}
