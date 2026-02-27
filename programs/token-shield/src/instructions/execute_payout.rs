use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{state::*, constants::*, errors::*};

#[derive(Accounts)]
#[instruction(policy_id: [u8; 32])]
pub struct ExecutePayout<'info> {
    #[account(
        mut,
        seeds = [POLICY_SEED, &policy_id],
        bump = policy.bump,
        constraint = policy.status == PolicyStatus::Triggered @ TokenShieldError::TriggerNotMet
    )]
    pub policy: Account<'info, Policy>,
    
    #[account(
        mut,
        seeds = [POOL_SEED],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,
    
    #[account(
        mut,
        seeds = [TRIGGER_EVENT_SEED, &policy_id],
        bump = trigger_event.bump,
        constraint = !trigger_event.payout_executed @ TokenShieldError::PolicyAlreadyPaidOut
    )]
    pub trigger_event: Account<'info, TriggerEvent>,
    
    /// Pool's USDC account (source of payout)
    #[account(mut)]
    pub pool_usdc_account: Account<'info, TokenAccount>,
    
    /// Covered wallet's USDC account (destination)
    #[account(
        mut,
        constraint = covered_wallet_usdc_account.owner == policy.covered_wallet
    )]
    pub covered_wallet_usdc_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

pub fn execute_payout(
    ctx: Context<ExecutePayout>,
    policy_id: [u8; 32],
) -> Result<()> {
    let policy = &mut ctx.accounts.policy;
    let pool = &mut ctx.accounts.pool;
    let trigger_event = &mut ctx.accounts.trigger_event;
    
    let payout_amount = trigger_event.payout_amount;
    
    // Verify pool has sufficient reserves
    require!(
        pool.liquid_reserves >= payout_amount,
        TokenShieldError::InsufficientLiquidity
    );
    
    // Transfer payout from pool to covered wallet (pool PDA signs)
    let pool_seeds = &[
        POOL_SEED,
        &[pool.bump],
    ];
    let signer = &[&pool_seeds[..]];
    
    let cpi_accounts = Transfer {
        from: ctx.accounts.pool_usdc_account.to_account_info(),
        to: ctx.accounts.covered_wallet_usdc_account.to_account_info(),
        authority: pool.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    token::transfer(cpi_ctx, payout_amount)?;
    
    // Update policy status
    policy.status = PolicyStatus::PaidOut;
    
    // Update pool state
    pool.total_payouts = pool.total_payouts
        .checked_add(payout_amount)
        .ok_or(TokenShieldError::MathOverflow)?;
    pool.liquid_reserves = pool.liquid_reserves
        .checked_sub(payout_amount)
        .ok_or(TokenShieldError::InsufficientLiquidity)?;
    pool.total_coverage_value = pool.total_coverage_value
        .checked_sub(policy.max_payout)
        .ok_or(TokenShieldError::MathOverflow)?;
    pool.active_policy_count = pool.active_policy_count
        .checked_sub(1)
        .ok_or(TokenShieldError::MathOverflow)?;
    
    // Mark payout as executed
    trigger_event.payout_executed = true;
    
    msg!(
        "Payout executed for policy {:?}: {} USDC sent to {:?}",
        policy_id,
        payout_amount,
        policy.covered_wallet
    );
    
    Ok(())
}
