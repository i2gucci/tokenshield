/// Protocol constants

/// Minimum coverage duration in days
pub const MIN_DURATION_DAYS: u8 = 7;

/// Maximum coverage duration in days
pub const MAX_DURATION_DAYS: u8 = 30;

/// Minimum coverage level in basis points (30%)
pub const MIN_COVERAGE_LEVEL_BPS: u16 = 3000;

/// Maximum coverage level in basis points (70%)
pub const MAX_COVERAGE_LEVEL_BPS: u16 = 7000;

/// Suggested coverage levels for UI (users can select any value between min/max)
pub const SUGGESTED_COVERAGE_LEVELS: [u16; 3] = [3000, 5000, 7000]; // 30%, 50%, 70%

/// Minimum position value in USD (scaled by 1e6)
pub const MIN_POSITION_USD: u64 = 1_000_000_000; // $1k

/// Maximum position size in USD (scaled by 1e6)
pub const MAX_POSITION_USD: u64 = 50_000_000_000; // $50k

/// Minimum pool liquid reserve ratio (20%)
pub const MIN_LIQUID_RESERVE_BPS: u16 = 2000;

/// Price dump trigger threshold (-40%)
pub const PRICE_DUMP_THRESHOLD_BPS: i16 = -4000;

/// Liquidity drain trigger threshold (-50%)
pub const LIQUIDITY_DRAIN_THRESHOLD_BPS: i16 = -5000;

/// Dev wallet dump trigger threshold (10% of supply)
pub const DEV_DUMP_THRESHOLD_BPS: u16 = 1000;

/// Oracle staleness limit (5 minutes)
pub const ORACLE_STALENESS_SECONDS: i64 = 300;

/// Team bulk discount range (15-30%)
pub const TEAM_MIN_DISCOUNT_BPS: u16 = 1500;
pub const TEAM_MAX_DISCOUNT_BPS: u16 = 3000;

/// Team surcharge for moral hazard (20-30%)
pub const TEAM_MIN_SURCHARGE_BPS: u16 = 2000;
pub const TEAM_MAX_SURCHARGE_BPS: u16 = 3000;

/// Basis points denominator
pub const BPS_DENOMINATOR: u64 = 10_000;

/// Token Eligibility Requirements

/// Minimum token age in days (for regular users)
pub const MIN_TOKEN_AGE_DAYS: u64 = 7;

/// Minimum market cap in USD (scaled by 1e6) - subject to change based on SOL price
pub const MIN_MARKET_CAP_USD: u64 = 500_000_000_000; // $500k

/// Note: Teams can apply for coverage even before token launch (Team Shield pre-approval)

/// Seconds per day
pub const SECONDS_PER_DAY: i64 = 86_400;

/// USD scale factor (1e6 for 6 decimals like USDC)
pub const USD_SCALE: u64 = 1_000_000;

/// Policy token decimals (0 = always exactly 1 token)
pub const POLICY_TOKEN_DECIMALS: u8 = 0;

/// Seeds for PDAs
pub const POOL_SEED: &[u8] = b"pool";
pub const POLICY_SEED: &[u8] = b"policy";
pub const TEAM_POLICY_SEED: &[u8] = b"team_policy";
pub const ORACLE_DATA_SEED: &[u8] = b"oracle_data";
pub const TRIGGER_EVENT_SEED: &[u8] = b"trigger_event";
