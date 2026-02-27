use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

pub mod instructions;
pub mod state;
pub mod errors;
pub mod constants;

use instructions::*;
use state::*;
use errors::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod token_shield {
    use super::*;

    /// Initialize the protocol pool
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        target_collateral_ratio: u16,
        base_rate_bps: u16,
    ) -> Result<()> {
        instructions::initialize_pool(ctx, target_collateral_ratio, base_rate_bps)
    }

    /// Initialize oracle data for a token
    pub fn initialize_oracle_data(
        ctx: Context<InitializeOracleData>,
        token_mint: Pubkey,
    ) -> Result<()> {
        instructions::initialize_oracle_data(ctx, token_mint)
    }

    /// Create a new coverage policy (VRF-based, no wallet connection required)
    pub fn create_policy(
        ctx: Context<CreatePolicy>,
        wallet_address: Pubkey,
        token_mint: Pubkey,
        position_size: u64,
        coverage_level_bps: u16, // 3000=30%, 5000=50%, 7000=70%
        duration_days: u8,
        vrf_proof: [u8; 64], // VRF proof for policy ID generation
    ) -> Result<()> {
        instructions::create_policy(
            ctx,
            wallet_address,
            token_mint,
            position_size,
            coverage_level_bps,
            duration_days,
            vrf_proof,
        )
    }

    /// Create team-sponsored coverage for multiple holders
    pub fn create_team_policy(
        ctx: Context<CreateTeamPolicy>,
        token_mint: Pubkey,
        holder_addresses: Vec<Pubkey>,
        coverage_level_bps: u16,
        duration_days: u8,
    ) -> Result<()> {
        instructions::create_team_policy(
            ctx,
            token_mint,
            holder_addresses,
            coverage_level_bps,
            duration_days,
        )
    }

    /// Update oracle data (price, liquidity, holder positions)
    pub fn update_oracle_data(
        ctx: Context<UpdateOracleData>,
        price: u64,
        liquidity: u64,
        timestamp: i64,
    ) -> Result<()> {
        instructions::update_oracle_data(ctx, price, liquidity, timestamp)
    }

    /// Check trigger conditions and initiate payout if met
    pub fn check_and_trigger_payout(
        ctx: Context<CheckTrigger>,
        policy_id: [u8; 32],
    ) -> Result<()> {
        instructions::check_and_trigger_payout(ctx, policy_id)
    }

    /// Execute payout to covered wallet
    pub fn execute_payout(
        ctx: Context<ExecutePayout>,
        policy_id: [u8; 32],
    ) -> Result<()> {
        instructions::execute_payout(ctx, policy_id)
    }

    /// Burn policy token after payout or expiry
    pub fn burn_policy_token(
        ctx: Context<BurnPolicyToken>,
        policy_id: [u8; 32],
    ) -> Result<()> {
        instructions::burn_policy_token(ctx, policy_id)
    }

    /// Add liquidity to insurance pool
    pub fn add_pool_liquidity(
        ctx: Context<AddPoolLiquidity>,
        amount: u64,
    ) -> Result<()> {
        instructions::add_pool_liquidity(ctx, amount)
    }

    /// Opt-out of team-sponsored coverage (privacy)
    pub fn opt_out_team_coverage(
        ctx: Context<OptOutTeamCoverage>,
        policy_id: [u8; 32],
    ) -> Result<()> {
        instructions::opt_out_team_coverage(ctx, policy_id)
    }
}
