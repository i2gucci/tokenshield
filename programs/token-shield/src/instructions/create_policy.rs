use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};
use crate::{state::*, constants::*, errors::*, instructions::utils::*};

#[derive(Accounts)]
#[instruction(wallet_address: Pubkey, token_mint: Pubkey, vrf_proof: [u8; 64])]
pub struct CreatePolicy<'info> {
    #[account(
        init,
        payer = payer,
        space = Policy::LEN,
        seeds = [
            POLICY_SEED,
            &derive_policy_id_from_vrf(&vrf_proof)
        ],
        bump
    )]
    pub policy: Account<'info, Policy>,
    
    #[account(
        mut,
        seeds = [POOL_SEED],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,
    
    /// Oracle data for the token being insured
    #[account(
        seeds = [ORACLE_DATA_SEED, token_mint.as_ref()],
        bump = oracle_data.bump
    )]
    pub oracle_data: Account<'info, OracleData>,
    
    /// Policy token mint (SPL token representing coverage)
    #[account(
        init,
        payer = payer,
        mint::decimals = POLICY_TOKEN_DECIMALS,
        mint::authority = policy,
    )]
    pub policy_token_mint: Account<'info, Mint>,
    
    /// Token account to receive policy token (covered wallet)
    #[account(mut)]
    pub policy_token_account: Account<'info, TokenAccount>,
    
    /// Premium payment account (USDC)
    #[account(mut)]
    pub premium_payment_account: Account<'info, TokenAccount>,
    
    /// Pool's USDC account
    #[account(mut)]
    pub pool_usdc_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_policy(
    ctx: Context<CreatePolicy>,
    wallet_address: Pubkey,
    token_mint: Pubkey,
    position_size: u64,
    coverage_level_bps: u16,
    duration_days: u8,
    vrf_proof: [u8; 64],
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let policy = &mut ctx.accounts.policy;
    let oracle_data = &ctx.accounts.oracle_data;
    
    // Validate inputs
    require!(
        coverage_level_bps >= MIN_COVERAGE_LEVEL_BPS && coverage_level_bps <= MAX_COVERAGE_LEVEL_BPS,
        TokenShieldError::InvalidCoverageLevel
    );
    
    require!(
        duration_days >= MIN_DURATION_DAYS && duration_days <= MAX_DURATION_DAYS,
        TokenShieldError::InvalidDuration
    );
    
    require!(!pool.paused, TokenShieldError::InsufficientLiquidity);
    
    // Verify oracle data is fresh
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp - oracle_data.last_update < ORACLE_STALENESS_SECONDS,
        TokenShieldError::StaleOracleData
    );
    
    // Calculate position value in USD
    let position_value_usd = calculate_position_value(
        position_size,
        oracle_data.price,
    )?;
    
    require!(
        position_value_usd <= MAX_POSITION_USD,
        TokenShieldError::PositionTooLarge
    );
    
    // Calculate risk score based on oracle data
    let risk_score = calculate_risk_score(oracle_data)?;
    
    // Calculate premium
    let premium = calculate_premium(
        position_value_usd,
        coverage_level_bps,
        duration_days,
        risk_score,
        pool.base_rate_bps,
    )?;
    
    // Calculate max payout
    let max_payout = (position_value_usd as u128)
        .checked_mul(coverage_level_bps as u128)
        .ok_or(TokenShieldError::MathOverflow)?
        .checked_div(BPS_DENOMINATOR as u128)
        .ok_or(TokenShieldError::MathOverflow)? as u64;
    
    // Check pool has sufficient liquidity
    let required_reserves = (max_payout as u128)
        .checked_mul(pool.target_collateral_ratio as u128)
        .ok_or(TokenShieldError::MathOverflow)?
        .checked_div(BPS_DENOMINATOR as u128)
        .ok_or(TokenShieldError::MathOverflow)? as u64;
    
    require!(
        pool.liquid_reserves >= required_reserves,
        TokenShieldError::InsufficientLiquidity
    );
    
    // Transfer premium from user to pool
    let cpi_accounts = token::Transfer {
        from: ctx.accounts.premium_payment_account.to_account_info(),
        to: ctx.accounts.pool_usdc_account.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, premium)?;
    
    // Generate policy ID from VRF proof
    let policy_id = derive_policy_id_from_vrf(&vrf_proof);
    
    // Initialize policy
    policy.policy_id = policy_id;
    policy.covered_wallet = wallet_address;
    policy.token_mint = token_mint;
    policy.position_size = position_size;
    policy.position_value_usd = position_value_usd;
    policy.coverage_level_bps = coverage_level_bps;
    policy.start_time = clock.unix_timestamp;
    policy.expiry_time = clock.unix_timestamp + (duration_days as i64 * SECONDS_PER_DAY);
    policy.premium_paid = premium;
    policy.max_payout = max_payout;
    policy.risk_score = risk_score;
    policy.entry_price = oracle_data.price;
    policy.status = PolicyStatus::Active;
    policy.is_team_policy = false;
    policy.team_policy = None;
    policy.policy_token_mint = ctx.accounts.policy_token_mint.key();
    policy.opted_out = false;
    policy.bump = ctx.bumps.policy;
    
    // Mint policy token to covered wallet
    let seeds = &[
        POLICY_SEED,
        &policy_id,
        &[policy.bump],
    ];
    let signer = &[&seeds[..]];
    
    let mint_accounts = MintTo {
        mint: ctx.accounts.policy_token_mint.to_account_info(),
        to: ctx.accounts.policy_token_account.to_account_info(),
        authority: policy.to_account_info(),
    };
    let mint_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        mint_accounts,
        signer,
    );
    token::mint_to(mint_ctx, 1)?; // Mint exactly 1 policy token
    
    // Update pool state
    pool.total_premiums = pool.total_premiums
        .checked_add(premium)
        .ok_or(TokenShieldError::MathOverflow)?;
    pool.liquid_reserves = pool.liquid_reserves
        .checked_add(premium)
        .ok_or(TokenShieldError::MathOverflow)?;
    pool.total_coverage_value = pool.total_coverage_value
        .checked_add(max_payout)
        .ok_or(TokenShieldError::MathOverflow)?;
    pool.active_policy_count = pool.active_policy_count
        .checked_add(1)
        .ok_or(TokenShieldError::MathOverflow)?;
    
    msg!("Policy created: ID {:?}, Premium: {}, Max Payout: {}",
        policy_id,
        premium,
        max_payout
    );
    
    Ok(())
}
