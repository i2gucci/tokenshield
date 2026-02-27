use anchor_lang::prelude::*;
use crate::{state::*, constants::*, errors::*};

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = authority,
        space = Pool::LEN,
        seeds = [POOL_SEED],
        bump
    )]
    pub pool: Account<'info, Pool>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_pool(
    ctx: Context<InitializePool>,
    target_collateral_ratio: u16,
    base_rate_bps: u16,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    
    pool.authority = ctx.accounts.authority.key();
    pool.total_premiums = 0;
    pool.total_payouts = 0;
    pool.liquid_reserves = 0;
    pool.deployed_yield = 0;
    pool.target_collateral_ratio = target_collateral_ratio;
    pool.base_rate_bps = base_rate_bps;
    pool.total_coverage_value = 0;
    pool.active_policy_count = 0;
    pool.paused = false;
    pool.bump = ctx.bumps.pool;
    
    msg!("Pool initialized with {}% collateral ratio and {}bps base rate",
        target_collateral_ratio / 100,
        base_rate_bps
    );
    
    Ok(())
}
