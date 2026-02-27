use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{state::*, constants::*, errors::*, instructions::utils::*};

#[derive(Accounts)]
#[instruction(token_mint: Pubkey, holder_addresses: Vec<Pubkey>)]
pub struct CreateTeamPolicy<'info> {
    #[account(
        init,
        payer = team_authority,
        space = TeamPolicy::LEN,
        seeds = [
            TEAM_POLICY_SEED,
            token_mint.as_ref(),
            team_authority.key().as_ref(),
        ],
        bump
    )]
    pub team_policy: Account<'info, TeamPolicy>,
    
    #[account(
        mut,
        seeds = [POOL_SEED],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,
    
    #[account(
        seeds = [ORACLE_DATA_SEED, token_mint.as_ref()],
        bump = oracle_data.bump
    )]
    pub oracle_data: Account<'info, OracleData>,
    
    /// Team's USDC account for premium payment
    #[account(mut)]
    pub team_usdc_account: Account<'info, TokenAccount>,
    
    /// Pool's USDC account
    #[account(mut)]
    pub pool_usdc_account: Account<'info, TokenAccount>,
    
    /// Team authority (multisig or verified creator)
    #[account(mut)]
    pub team_authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn create_team_policy(
    ctx: Context<CreateTeamPolicy>,
    token_mint: Pubkey,
    holder_addresses: Vec<Pubkey>,
    coverage_level_bps: u16,
    duration_days: u8,
) -> Result<()> {
    let team_policy = &mut ctx.accounts.team_policy;
    let pool = &mut ctx.accounts.pool;
    let oracle_data = &ctx.accounts.oracle_data;
    let clock = Clock::get()?;
    
    // Validate inputs
    require!(
        VALID_COVERAGE_LEVELS.contains(&coverage_level_bps),
        TokenShieldError::InvalidCoverageLevel
    );
    
    require!(
        duration_days >= MIN_DURATION_DAYS && duration_days <= MAX_DURATION_DAYS,
        TokenShieldError::InvalidDuration
    );
    
    require!(
        !holder_addresses.is_empty() && holder_addresses.len() <= 1000,
        TokenShieldError::InvalidOracleData
    );
    
    // Check oracle freshness
    require!(
        clock.unix_timestamp - oracle_data.last_update < ORACLE_STALENESS_SECONDS,
        TokenShieldError::StaleOracleData
    );
    
    // Calculate risk score
    let risk_score = calculate_risk_score(oracle_data)?;
    
    // Calculate total coverage value (would need to snapshot each holder's position)
    // For now, simplified - in production, would iterate through holder_addresses
    // and sum their token balances * price
    let total_coverage_value = 1_000_000 * USD_SCALE; // Placeholder $1M
    
    // Determine if team has locked liquidity (would check on-chain LP locks)
    let liquidity_locked = false; // Placeholder - implement LP lock verification
    
    // Calculate team premium with discounts and surcharges
    let total_premium = calculate_team_premium(
        total_coverage_value,
        coverage_level_bps,
        duration_days,
        risk_score,
        pool.base_rate_bps,
        holder_addresses.len() as u32,
        liquidity_locked,
    )?;
    
    // Check pool has sufficient capacity
    let max_payout = (total_coverage_value as u128)
        .checked_mul(coverage_level_bps as u128)
        .ok_or(TokenShieldError::MathOverflow)?
        .checked_div(BPS_DENOMINATOR as u128)
        .ok_or(TokenShieldError::MathOverflow)? as u64;
    
    let required_reserves = (max_payout as u128)
        .checked_mul(pool.target_collateral_ratio as u128)
        .ok_or(TokenShieldError::MathOverflow)?
        .checked_div(BPS_DENOMINATOR as u128)
        .ok_or(TokenShieldError::MathOverflow)? as u64;
    
    require!(
        pool.liquid_reserves >= required_reserves,
        TokenShieldError::InsufficientLiquidity
    );
    
    // Transfer premium from team to pool
    let cpi_accounts = Transfer {
        from: ctx.accounts.team_usdc_account.to_account_info(),
        to: ctx.accounts.pool_usdc_account.to_account_info(),
        authority: ctx.accounts.team_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, total_premium)?;
    
    // Initialize team policy
    team_policy.team_authority = ctx.accounts.team_authority.key();
    team_policy.token_mint = token_mint;
    team_policy.holder_count = holder_addresses.len() as u32;
    team_policy.total_coverage_value = total_coverage_value;
    team_policy.coverage_level_bps = coverage_level_bps;
    team_policy.start_time = clock.unix_timestamp;
    team_policy.expiry_time = clock.unix_timestamp + (duration_days as i64 * SECONDS_PER_DAY);
    team_policy.total_premium = total_premium;
    team_policy.bulk_discount_bps = 2000; // Calculated discount
    team_policy.team_surcharge_bps = 2500; // Calculated surcharge
    team_policy.liquidity_locked = liquidity_locked;
    team_policy.active = true;
    team_policy.bump = ctx.bumps.team_policy;
    
    // Update pool state
    pool.total_premiums = pool.total_premiums
        .checked_add(total_premium)
        .ok_or(TokenShieldError::MathOverflow)?;
    pool.liquid_reserves = pool.liquid_reserves
        .checked_add(total_premium)
        .ok_or(TokenShieldError::MathOverflow)?;
    pool.total_coverage_value = pool.total_coverage_value
        .checked_add(max_payout)
        .ok_or(TokenShieldError::MathOverflow)?;
    
    msg!(
        "Team policy created for {} holders: Premium: {}, Coverage: {}",
        holder_addresses.len(),
        total_premium,
        total_coverage_value
    );
    
    Ok(())
}
