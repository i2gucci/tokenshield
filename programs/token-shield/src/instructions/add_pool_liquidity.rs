use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{state::*, constants::*, errors::*};

#[derive(Accounts)]
pub struct AddPoolLiquidity<'info> {
    #[account(
        mut,
        seeds = [POOL_SEED],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,
    
    /// Liquidity provider's USDC account
    #[account(mut)]
    pub provider_usdc_account: Account<'info, TokenAccount>,
    
    /// Pool's USDC account
    #[account(mut)]
    pub pool_usdc_account: Account<'info, TokenAccount>,
    
    pub provider: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

pub fn add_pool_liquidity(
    ctx: Context<AddPoolLiquidity>,
    amount: u64,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    
    // Transfer USDC from provider to pool
    let cpi_accounts = Transfer {
        from: ctx.accounts.provider_usdc_account.to_account_info(),
        to: ctx.accounts.pool_usdc_account.to_account_info(),
        authority: ctx.accounts.provider.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;
    
    // Update pool reserves
    pool.liquid_reserves = pool.liquid_reserves
        .checked_add(amount)
        .ok_or(TokenShieldError::MathOverflow)?;
    
    msg!("Liquidity added to pool: {} USDC", amount);
    
    Ok(())
}
