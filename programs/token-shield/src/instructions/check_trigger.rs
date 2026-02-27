use anchor_lang::prelude::*;
use crate::{state::*, constants::*, errors::*, instructions::utils::*};

#[derive(Accounts)]
#[instruction(policy_id: [u8; 32])]
pub struct CheckTrigger<'info> {
    #[account(
        mut,
        seeds = [POLICY_SEED, &policy_id],
        bump = policy.bump,
        constraint = policy.status == PolicyStatus::Active @ TokenShieldError::PolicyAlreadyPaidOut
    )]
    pub policy: Account<'info, Policy>,
    
    #[account(
        seeds = [ORACLE_DATA_SEED, policy.token_mint.as_ref()],
        bump = oracle_data.bump
    )]
    pub oracle_data: Account<'info, OracleData>,
    
    #[account(
        init,
        payer = payer,
        space = TriggerEvent::LEN,
        seeds = [TRIGGER_EVENT_SEED, &policy_id],
        bump
    )]
    pub trigger_event: Account<'info, TriggerEvent>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn check_and_trigger_payout(
    ctx: Context<CheckTrigger>,
    policy_id: [u8; 32],
) -> Result<()> {
    let policy = &mut ctx.accounts.policy;
    let oracle_data = &ctx.accounts.oracle_data;
    let trigger_event = &mut ctx.accounts.trigger_event;
    let clock = Clock::get()?;
    
    // Check policy hasn't expired
    require!(
        clock.unix_timestamp <= policy.expiry_time,
        TokenShieldError::PolicyExpired
    );
    
    // Check if holder opted out (for team policies)
    require!(
        !policy.opted_out,
        TokenShieldError::HolderOptedOut
    );
    
    // Verify oracle data is fresh
    require!(
        clock.unix_timestamp - oracle_data.last_update < ORACLE_STALENESS_SECONDS,
        TokenShieldError::StaleOracleData
    );
    
    // Check each trigger condition
    let mut triggered = false;
    let mut trigger_type = TriggerType::PriceDump;
    let mut percentage_change: i16 = 0;
    
    // 1. Check price dump (>40% drop from entry)
    if check_price_dump_trigger(policy.entry_price, oracle_data.price)? {
        triggered = true;
        trigger_type = TriggerType::PriceDump;
        percentage_change = calculate_percentage_change(
            policy.entry_price as i64,
            oracle_data.price as i64,
        )?;
        msg!("Price dump trigger met: {}bps change", percentage_change);
    }
    
    // 2. Check liquidity drain (>50% drop in 24h)
    if !triggered && oracle_data.liquidity_24h_ago > 0 {
        if check_liquidity_drain_trigger(
            oracle_data.liquidity_24h_ago,
            oracle_data.liquidity,
        )? {
            triggered = true;
            trigger_type = TriggerType::LiquidityDrain;
            percentage_change = calculate_percentage_change(
                oracle_data.liquidity_24h_ago as i64,
                oracle_data.liquidity as i64,
            )?;
            msg!("Liquidity drain trigger met: {}bps change", percentage_change);
        }
    }
    
    // Require at least one trigger condition is met
    require!(triggered, TokenShieldError::TriggerNotMet);
    
    // Calculate payout amount
    let payout_amount = calculate_payout(
        policy.entry_price,
        oracle_data.price,
        policy.position_size,
        policy.coverage_level_bps,
        policy.max_payout,
    )?;
    
    // Update policy status
    policy.status = PolicyStatus::Triggered;
    
    // Record trigger event
    trigger_event.policy_id = policy_id;
    trigger_event.trigger_type = trigger_type;
    trigger_event.timestamp = clock.unix_timestamp;
    trigger_event.trigger_price = oracle_data.price;
    trigger_event.percentage_change = percentage_change;
    trigger_event.payout_amount = payout_amount;
    trigger_event.payout_executed = false;
    trigger_event.bump = ctx.bumps.trigger_event;
    
    msg!(
        "Trigger activated for policy {:?}: type={:?}, payout={}",
        policy_id,
        trigger_type,
        payout_amount
    );
    
    Ok(())
}
