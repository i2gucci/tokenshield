use anchor_lang::prelude::*;
use crate::{state::*, constants::*};

#[derive(Accounts)]
#[instruction(token_mint: Pubkey)]
pub struct InitializeOracleData<'info> {
    #[account(
        init,
        payer = authority,
        space = OracleData::LEN,
        seeds = [ORACLE_DATA_SEED, token_mint.as_ref()],
        bump
    )]
    pub oracle_data: Account<'info, OracleData>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_oracle_data(
    ctx: Context<InitializeOracleData>,
    token_mint: Pubkey,
) -> Result<()> {
    let oracle_data = &mut ctx.accounts.oracle_data;
    let clock = Clock::get()?;
    
    oracle_data.token_mint = token_mint;
    oracle_data.price = 0;
    oracle_data.liquidity = 0;
    oracle_data.price_24h_ago = 0;
    oracle_data.liquidity_24h_ago = 0;
    oracle_data.last_update = clock.unix_timestamp;
    oracle_data.oracle_source = OracleSource::Manual;
    oracle_data.bump = ctx.bumps.oracle_data;
    
    msg!("Oracle data initialized for token: {}", token_mint);
    
    Ok(())
}
