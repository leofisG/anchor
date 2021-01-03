#![feature(proc_macro_hygiene)]

use anchor::prelude::*;

#[program]
mod basic {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> ProgramResult {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub my_account: ProgramAccount<'info, MyAccount>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MyAccount {
    pub data: u64,
}