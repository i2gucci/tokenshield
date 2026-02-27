use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};
use crate::{state::*, constants::*, errors::*};

#[derive(Accounts)]
#[instruction(policy_id: [u8; 32])]
pub struct BurnPolicyToken<'info> {
    #[account(
        mut,
        seeds = [POLICY_SEED, &policy_id],
        bump = policy.bump,
        constraint = policy.status == PolicyStatus::PaidOut || policy.status == PolicyStatus::Expired
    )]
    pub policy: Account<'info, Policy>,
    
    /// Policy token mint
    #[account(
        mut,
        constraint = policy_token_mint.key() == policy.policy_token_mint
    )]
    pub policy_token_mint: Account<'info, Mint>,
    
    /// Holder's policy token account
    #[account(
        mut,
        constraint = policy_token_account.mint == policy.policy_token_mint,
        constraint = policy_token_account.amount == 1
    )]
    pub policy_token_account: Account<'info, TokenAccount>,
    
    /// Token account authority (covered wallet owner)
    pub token_authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

pub fn burn_policy_token(
    ctx: Context<BurnPolicyToken>,
    policy_id: [u8; 32],
) -> Result<()> {
    let policy = &ctx.accounts.policy;
    
    // Burn the policy token
    let cpi_accounts = Burn {
        mint: ctx.accounts.policy_token_mint.to_account_info(),
        from: ctx.accounts.policy_token_account.to_account_info(),
        authority: ctx.accounts.token_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::burn(cpi_ctx, 1)?; // Burn exactly 1 token
    
    msg!("Policy token burned for policy {:?}", policy_id);
    
    Ok(())
}
