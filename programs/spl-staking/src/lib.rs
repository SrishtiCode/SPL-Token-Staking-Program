use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

declare_id!("E4ujhyaQNbyw7epzKuSwEDVP6GmpLanYuPmPaahL3eSn");

#[program]
pub mod spl_staking {
    use super::*;
    pub fn stake(ctx: Context<StakeAccounts>, amount: u64) -> Result<()> {
        let _ = ctx.bumps.stake_entry;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StakeAccounts<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 64,
        seeds = [b"stake_entry", user.key().as_ref()],
        bump,
    )]
    pub stake_entry: Account<'info, StakeEntry>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(InitSpace)]
pub struct StakeEntry {
    pub amount: u64,
}
