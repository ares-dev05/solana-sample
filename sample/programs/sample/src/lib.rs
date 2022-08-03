use anchor_lang::prelude::*;

declare_id!("7RZ3zNg49LrzAtesYLTKPH9jUyTNCoQvD8u6pKar9Dgk");

#[program]
pub mod sample {
    use super::*;

    pub fn create(_ctx: Context<Create>) -> Result<()> {
        let base_account = &mut _ctx.accounts.base_account;
        base_account.count = 0;

        Ok(())
    }

    pub fn increment(_ctx: Context<Create>) -> Result<()> {
        let base_account = &mut _ctx.accounts.base_account;
        base_account.count += 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 32)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increase<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
#[derive(Default)]
pub struct BaseAccount {
    count: u8,
}

