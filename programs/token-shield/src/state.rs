use anchor_lang::prelude::*;

/// Insurance pool state - holds all protocol funds and configuration
#[account]
pub struct Pool {
    /// Authority that can update pool parameters
    pub authority: Pubkey,
    
    /// Total premiums collected (in USDC/SOL lamports)
    pub total_premiums: u64,
    
    /// Total payouts distributed
    pub total_payouts: u64,
    
    /// Current liquid reserves (not deployed to yield)
    pub liquid_reserves: u64,
    
    /// Funds deployed to yield strategies
    pub deployed_yield: u64,
    
    /// Target collateral ratio (basis points, e.g., 15000 = 150%)
    pub target_collateral_ratio: u16,
    
    /// Base premium rate (basis points, e.g., 200 = 2%)
    pub base_rate_bps: u16,
    
    /// Total active coverage value
    pub total_coverage_value: u64,
    
    /// Number of active policies
    pub active_policy_count: u32,
    
    /// Emergency pause flag
    pub paused: bool,
    
    /// Bump seed for PDA
    pub bump: u8,
}

impl Pool {
    pub const LEN: usize = 8 // discriminator
        + 32 // authority
        + 8 // total_premiums
        + 8 // total_payouts
        + 8 // liquid_reserves
        + 8 // deployed_yield
        + 2 // target_collateral_ratio
        + 2 // base_rate_bps
        + 8 // total_coverage_value
        + 4 // active_policy_count
        + 1 // paused
        + 1; // bump
}

/// Individual coverage policy
#[account]
pub struct Policy {
    /// Unique policy ID (derived from VRF)
    pub policy_id: [u8; 32],
    
    /// Wallet address being covered
    pub covered_wallet: Pubkey,
    
    /// Token mint being insured
    pub token_mint: Pubkey,
    
    /// Position size at snapshot (in token native units)
    pub position_size: u64,
    
    /// USD value at enrollment
    pub position_value_usd: u64,
    
    /// Coverage level in basis points (3000=30%, 5000=50%, 7000=70%)
    pub coverage_level_bps: u16,
    
    /// Policy start timestamp
    pub start_time: i64,
    
    /// Policy expiry timestamp
    pub expiry_time: i64,
    
    /// Premium paid (in USDC lamports)
    pub premium_paid: u64,
    
    /// Maximum payout amount
    pub max_payout: u64,
    
    /// Risk score assigned at enrollment (1-10)
    pub risk_score: u8,
    
    /// Entry price (from oracle at enrollment)
    pub entry_price: u64,
    
    /// Policy status
    pub status: PolicyStatus,
    
    /// Whether this is part of team coverage
    pub is_team_policy: bool,
    
    /// Team policy account if applicable
    pub team_policy: Option<Pubkey>,
    
    /// Policy token mint address (SPL token representing coverage)
    pub policy_token_mint: Pubkey,
    
    /// Whether holder has opted out (for team policies)
    pub opted_out: bool,
    
    /// Bump seed for PDA
    pub bump: u8,
}

impl Policy {
    pub const LEN: usize = 8 // discriminator
        + 32 // policy_id
        + 32 // covered_wallet
        + 32 // token_mint
        + 8 // position_size
        + 8 // position_value_usd
        + 2 // coverage_level_bps
        + 8 // start_time
        + 8 // expiry_time
        + 8 // premium_paid
        + 8 // max_payout
        + 1 // risk_score
        + 8 // entry_price
        + 1 // status
        + 1 // is_team_policy
        + 33 // team_policy (Option<Pubkey>)
        + 32 // policy_token_mint
        + 1 // opted_out
        + 1; // bump
}

/// Team-sponsored coverage policy
#[account]
pub struct TeamPolicy {
    /// Team authority (multisig or verified creator)
    pub team_authority: Pubkey,
    
    /// Token mint being covered
    pub token_mint: Pubkey,
    
    /// Number of holders covered
    pub holder_count: u32,
    
    /// Total coverage value
    pub total_coverage_value: u64,
    
    /// Coverage level (applies to all holders)
    pub coverage_level_bps: u16,
    
    /// Policy start time
    pub start_time: i64,
    
    /// Policy expiry time
    pub expiry_time: i64,
    
    /// Total premium paid by team
    pub total_premium: u64,
    
    /// Bulk discount applied (basis points)
    pub bulk_discount_bps: u16,
    
    /// Team surcharge applied (basis points)
    pub team_surcharge_bps: u16,
    
    /// Whether team has locked liquidity
    pub liquidity_locked: bool,
    
    /// Active status
    pub active: bool,
    
    /// Bump seed
    pub bump: u8,
}

impl TeamPolicy {
    pub const LEN: usize = 8 // discriminator
        + 32 // team_authority
        + 32 // token_mint
        + 4 // holder_count
        + 8 // total_coverage_value
        + 2 // coverage_level_bps
        + 8 // start_time
        + 8 // expiry_time
        + 8 // total_premium
        + 2 // bulk_discount_bps
        + 2 // team_surcharge_bps
        + 1 // liquidity_locked
        + 1 // active
        + 1; // bump
}

/// Oracle data for a specific token
#[account]
pub struct OracleData {
    /// Token mint
    pub token_mint: Pubkey,
    
    /// Current price (in USD, scaled by 1e6)
    pub price: u64,
    
    /// Total liquidity across DEXs
    pub liquidity: u64,
    
    /// Price 24h ago (for dump detection)
    pub price_24h_ago: u64,
    
    /// Liquidity 24h ago (for drain detection)
    pub liquidity_24h_ago: u64,
    
    /// Last update timestamp
    pub last_update: i64,
    
    /// Oracle source (Pyth, Jupiter, Manual)
    pub oracle_source: OracleSource,
    
    /// Bump seed
    pub bump: u8,
}

impl OracleData {
    pub const LEN: usize = 8 // discriminator
        + 32 // token_mint
        + 8 // price
        + 8 // liquidity
        + 8 // price_24h_ago
        + 8 // liquidity_24h_ago
        + 8 // last_update
        + 1 // oracle_source
        + 1; // bump
}

/// Trigger event record
#[account]
pub struct TriggerEvent {
    /// Policy that triggered
    pub policy_id: [u8; 32],
    
    /// Trigger type
    pub trigger_type: TriggerType,
    
    /// Trigger timestamp
    pub timestamp: i64,
    
    /// Price at trigger (if price dump)
    pub trigger_price: u64,
    
    /// Percentage change that caused trigger
    pub percentage_change: i16, // signed, in basis points
    
    /// Payout amount calculated
    pub payout_amount: u64,
    
    /// Whether payout has been executed
    pub payout_executed: bool,
    
    /// Bump seed
    pub bump: u8,
}

impl TriggerEvent {
    pub const LEN: usize = 8 // discriminator
        + 32 // policy_id
        + 1 // trigger_type
        + 8 // timestamp
        + 8 // trigger_price
        + 2 // percentage_change
        + 8 // payout_amount
        + 1 // payout_executed
        + 1; // bump
}

/// Policy status enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PolicyStatus {
    Active,
    Triggered,
    PaidOut,
    Expired,
    Cancelled,
}

/// Trigger type enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum TriggerType {
    PriceDump,      // >40% drop in 24h
    LiquidityDrain, // >50% liquidity loss in 24h
    DevWalletDump,  // >10% supply dump
}

/// Oracle source enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum OracleSource {
    Pyth,
    Jupiter,
    Manual, // For testing
}
