use anchor_lang::prelude::*;
use crate::{state::*, constants::*, errors::*};

#[derive(Accounts)]
pub struct UpdateOracleData<'info> {
    #[account(
        mut,
        seeds = [ORACLE_DATA_SEED, oracle_data.token_mint.as_ref()],
        bump = oracle_data.bump
    )]
    pub oracle_data: Account<'info, OracleData>,
    
    /// Oracle authority (could be Pyth, Switchboard, or trusted updater)
    pub oracle_authority: Signer<'info>,
}

pub fn update_oracle_data(
    ctx: Context<UpdateOracleData>,
    price: u64,
    liquidity: u64,
    timestamp: i64,
) -> Result<()> {
    let oracle_data = &mut ctx.accounts.oracle_data;
    let clock = Clock::get()?;
    
    // Validate timestamp is not in the future
    require!(
        timestamp <= clock.unix_timestamp,
        TokenShieldError::InvalidOracleData
    );
    
    // Update 24h historical data if needed
    if clock.unix_timestamp - oracle_data.last_update >= SECONDS_PER_DAY {
        oracle_data.price_24h_ago = oracle_data.price;
        oracle_data.liquidity_24h_ago = oracle_data.liquidity;
    }
    
    // Update current data
    oracle_data.price = price;
    oracle_data.liquidity = liquidity;
    oracle_data.last_update = timestamp;
    
    msg!("Oracle updated: price={}, liquidity={}", price, liquidity);
    
    Ok(())
}
