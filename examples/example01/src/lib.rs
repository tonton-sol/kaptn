use kaptn_lang::prelude::*;

declare_id!("5H4LbTCzkudomL3ocLttgLFtHWvpbiadS1DhPGvo2XYh");
declare_mint!("FQf33CHwMZY4TYo6RP5CuTXUCVs8YFJH1MreMYtHiPhi");

#[transfer_hook]
pub fn hello_world(ctx: TransferContext<MyExtraMetas>) -> ProgramResult {
    msg!("Ahoy from transfer-hook program: {:?}", ctx.program_id);
    Ok(())
}

#[derive(ExtraMetas)]
pub struct MyExtraMetas {}
