use kaptn_lang::prelude::*;

declare_id!("5ybRi28gZYes7sxvEgCrskTq2yzBzTwQaHznVPKMuSCT");
declare_mint!("4KXsXhiy8pc6hzPXTZECeRhZKQwbqnmpmuXsXP7rmLEx");

#[transfer_hook]
pub fn hello_world(ctx: TransferContext<MyExtraMetas>) -> ProgramResult {
    msg!("Ahoy from transfer-hook program: {:?}", ctx.program_id);
    Ok(())
}

#[derive(ExtraMetas)]
pub struct MyExtraMetas {}

// #[derive(ExtraMetas)]
// pub struct MyExtraMetas<'info> {
//     #[meta(
//         pubkey = "So11111111111111111111111111111111111111112",
//         signer = false,
//         writable = false
//     )]
//     wsol_mint: AccountInfo<'info>,
// }
