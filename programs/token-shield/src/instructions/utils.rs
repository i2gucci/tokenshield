use anchor_lang::prelude::*;
use solana_program::keccak;
use crate::{state::*, constants::*, errors::*};

/// Derive policy ID from VRF proof
pub fn derive_policy_id_from_vrf(vrf_proof: &[u8; 64]) -> [u8; 32] {
    let hash = keccak::hash(vrf_proof);
    hash.to_bytes()
}

/// Calculate position value in USD
pub fn calculate_position_value(
    position_size: u64,
    price: u64, // Price in USD scaled by 1e6
) -> Result<u64> {
    // Simple calculation: position_size * price / token_decimals_scale
    // For now, assume token has 6 decimals like USDC
    let value = (position_size as u128)
        .checked_mul(price as u128)
        .ok_or(TokenShieldError::MathOverflow)?
        .checked_div(USD_SCALE as u128)
        .ok_or(TokenShieldError::MathOverflow)? as u64;
    
    Ok(value)
}

/// Calculate risk score (1-10) based on oracle data
pub fn calculate_risk_score(oracle_data: &OracleData) -> Result<u8> {
    let mut score: u8 = 5; // Base score
    
    // Check liquidity (lower liquidity = higher risk)
    if oracle_data.liquidity < 500_000 * USD_SCALE {
        score = score.saturating_add(2);
    } else if oracle_data.liquidity < 2_000_000 * USD_SCALE {
        score = score.saturating_add(1);
    }
    
    // Check price volatility (24h change)
    if oracle_data.price_24h_ago > 0 {
        let price_change_bps = calculate_percentage_change(
            oracle_data.price_24h_ago as i64,
            oracle_data.price as i64,
        )?;
        
        if price_change_bps.abs() > 5000 { // >50% volatility
            score = score.saturating_add(2);
        } else if price_change_bps.abs() > 2000 { // >20% volatility
            score = score.saturating_add(1);
        }
    }
    
    // Cap at 10
    Ok(score.min(10).max(1))
}

/// Calculate premium based on position, coverage, duration, and risk
pub fn calculate_premium(
    position_value_usd: u64,
    coverage_level_bps: u16,
    duration_days: u8,
    risk_score: u8,
    base_rate_bps: u16,
) -> Result<u64> {
    // Formula: PositionValue × CoverageLevel × (Duration/30) × RiskScore × BaseRate
    
    let premium = (position_value_usd as u128)
        .checked_mul(coverage_level_bps as u128)
        .ok_or(TokenShieldError::PremiumCalculationError)?
        .checked_div(BPS_DENOMINATOR as u128)
        .ok_or(TokenShieldError::PremiumCalculationError)?
        .checked_mul(duration_days as u128)
        .ok_or(TokenShieldError::PremiumCalculationError)?
        .checked_div(30u128)
        .ok_or(TokenShieldError::PremiumCalculationError)?
        .checked_mul(risk_score as u128)
        .ok_or(TokenShieldError::PremiumCalculationError)?
        .checked_mul(base_rate_bps as u128)
        .ok_or(TokenShieldError::PremiumCalculationError)?
        .checked_div(BPS_DENOMINATOR as u128)
        .ok_or(TokenShieldError::PremiumCalculationError)? as u64;
    
    Ok(premium)
}

/// Calculate team premium with bulk discount and surcharge
pub fn calculate_team_premium(
    total_coverage_value: u64,
    coverage_level_bps: u16,
    duration_days: u8,
    risk_score: u8,
    base_rate_bps: u16,
    holder_count: u32,
    liquidity_locked: bool,
) -> Result<u64> {
    // Base premium
    let base_premium = calculate_premium(
        total_coverage_value,
        coverage_level_bps,
        duration_days,
        risk_score,
        base_rate_bps,
    )?;
    
    // Calculate bulk discount (more holders = bigger discount)
    let bulk_discount_bps = if holder_count >= 100 {
        TEAM_MAX_DISCOUNT_BPS
    } else if holder_count >= 50 {
        2500 // 25%
    } else if holder_count >= 25 {
        2000 // 20%
    } else {
        TEAM_MIN_DISCOUNT_BPS // 15%
    };
    
    // Apply bulk discount
    let discounted = (base_premium as u128)
        .checked_mul((BPS_DENOMINATOR - bulk_discount_bps as u64) as u128)
        .ok_or(TokenShieldError::PremiumCalculationError)?
        .checked_div(BPS_DENOMINATOR as u128)
        .ok_or(TokenShieldError::PremiumCalculationError)? as u64;
    
    // Calculate moral hazard surcharge
    let surcharge_bps = if liquidity_locked {
        TEAM_MIN_SURCHARGE_BPS // Lower surcharge if LP locked
    } else {
        TEAM_MAX_SURCHARGE_BPS // Higher surcharge if no lock
    };
    
    // Apply surcharge
    let final_premium = (discounted as u128)
        .checked_mul((BPS_DENOMINATOR as u64 + surcharge_bps as u64) as u128)
        .ok_or(TokenShieldError::PremiumCalculationError)?
        .checked_div(BPS_DENOMINATOR as u128)
        .ok_or(TokenShieldError::PremiumCalculationError)? as u64;
    
    Ok(final_premium)
}

/// Calculate percentage change in basis points
pub fn calculate_percentage_change(old_value: i64, new_value: i64) -> Result<i16> {
    if old_value == 0 {
        return Ok(0);
    }
    
    let change = new_value - old_value;
    let percentage_bps = (change as i128)
        .checked_mul(BPS_DENOMINATOR as i128)
        .ok_or(TokenShieldError::MathOverflow)?
        .checked_div(old_value as i128)
        .ok_or(TokenShieldError::MathOverflow)? as i16;
    
    Ok(percentage_bps)
}

/// Check if price dump trigger is met
pub fn check_price_dump_trigger(
    entry_price: u64,
    current_price: u64,
) -> Result<bool> {
    let change_bps = calculate_percentage_change(
        entry_price as i64,
        current_price as i64,
    )?;
    
    Ok(change_bps <= PRICE_DUMP_THRESHOLD_BPS)
}

/// Check if liquidity drain trigger is met
pub fn check_liquidity_drain_trigger(
    liquidity_24h_ago: u64,
    current_liquidity: u64,
) -> Result<bool> {
    let change_bps = calculate_percentage_change(
        liquidity_24h_ago as i64,
        current_liquidity as i64,
    )?;
    
    Ok(change_bps <= LIQUIDITY_DRAIN_THRESHOLD_BPS)
}

/// Calculate actual payout based on loss and coverage
pub fn calculate_payout(
    entry_price: u64,
    trigger_price: u64,
    position_size: u64,
    coverage_level_bps: u16,
    max_payout: u64,
) -> Result<u64> {
    // Calculate actual loss
    let loss_per_token = entry_price.saturating_sub(trigger_price);
    let total_loss_value = (position_size as u128)
        .checked_mul(loss_per_token as u128)
        .ok_or(TokenShieldError::MathOverflow)?
        .checked_div(USD_SCALE as u128)
        .ok_or(TokenShieldError::MathOverflow)? as u64;
    
    // Apply coverage level
    let covered_loss = (total_loss_value as u128)
        .checked_mul(coverage_level_bps as u128)
        .ok_or(TokenShieldError::MathOverflow)?
        .checked_div(BPS_DENOMINATOR as u128)
        .ok_or(TokenShieldError::MathOverflow)? as u64;
    
    // Cap at max payout
    Ok(covered_loss.min(max_payout))
}
