# Token Shield - Smart Contract Architecture

## Overview

Token Shield is a decentralized insurance protocol for high-risk Solana tokens. This directory contains the core Anchor programs that implement the protocol's functionality.

## Architecture

### Core Programs

The protocol is built as a single Anchor program with multiple instruction handlers:

#### 1. **Pool Management**
- `initialize_pool` - Set up the insurance pool with initial parameters
- `add_pool_liquidity` - Add funds to the insurance pool

#### 2. **Policy Creation**
- `create_policy` - Individual coverage enrollment (VRF-based, no wallet connection)
- `create_team_policy` - Team-sponsored bulk coverage for holders

#### 3. **Oracle Integration**
- `update_oracle_data` - Update price/liquidity data for monitored tokens

#### 4. **Trigger & Payout System**
- `check_and_trigger_payout` - Check if trigger conditions are met
- `execute_payout` - Execute payment to covered wallet
- `burn_policy_token` - Burn SPL policy token after payout/expiry

#### 5. **Privacy & Opt-Out**
- `opt_out_team_coverage` - Allow holders to opt out of team coverage

## Data Structures

### Pool
```rust
pub struct Pool {
    pub authority: Pubkey,
    pub total_premiums: u64,
    pub total_payouts: u64,
    pub liquid_reserves: u64,
    pub deployed_yield: u64,
    pub target_collateral_ratio: u16,
    pub base_rate_bps: u16,
    pub total_coverage_value: u64,
    pub active_policy_count: u32,
    pub paused: bool,
}
```

### Policy
```rust
pub struct Policy {
    pub policy_id: [u8; 32],
    pub covered_wallet: Pubkey,
    pub token_mint: Pubkey,
    pub position_size: u64,
    pub coverage_level_bps: u16,
    pub start_time: i64,
    pub expiry_time: i64,
    pub premium_paid: u64,
    pub max_payout: u64,
    pub risk_score: u8,
    pub entry_price: u64,
    pub status: PolicyStatus,
    pub policy_token_mint: Pubkey,
}
```

### TeamPolicy
```rust
pub struct TeamPolicy {
    pub team_authority: Pubkey,
    pub token_mint: Pubkey,
    pub holder_count: u32,
    pub total_coverage_value: u64,
    pub coverage_level_bps: u16,
    pub total_premium: u64,
    pub bulk_discount_bps: u16,
    pub team_surcharge_bps: u16,
}
```

## Key Features

### VRF-Based Policy IDs
Policies use Chainlink VRF (or similar) to generate verifiable random policy IDs. This enables:
- No wallet connection required for enrollment
- Bearer bond model (policy ID + secret)
- Maximum privacy for users

### SPL Policy Tokens
Instead of NFTs, the protocol mints standard SPL tokens:
- Format: `TS-POLICY-####`
- 0 decimals (exactly 1 token = 1 policy)
- Non-transferable via freeze authority
- Automatically burned after payout or expiry

### Trigger System
Automated monitoring for three trigger types:
1. **Price Dump**: >40% drop in 24h
2. **Liquidity Drain**: >50% liquidity loss in 24h
3. **Dev Wallet Dump**: >10% supply dump (planned)

### Premium Calculation
```
Premium = PositionValue × CoverageLevel × (Duration/30) × RiskScore × BaseRate
```

For team policies:
```
TeamPremium = BasePremium × (1 - BulkDiscount) × (1 + MoralHazardSurcharge)
```

## Development

### Build
```bash
anchor build
```

### Test
```bash
anchor test
```

### Deploy to Devnet
```bash
anchor deploy --provider.cluster devnet
```

## Security Considerations

### Implemented
- ✅ Arithmetic overflow checks
- ✅ Authority validation
- ✅ Oracle staleness checks
- ✅ Policy status constraints
- ✅ Collateral ratio enforcement

### TODO
- [ ] Rate limiting for policy creation
- [ ] Emergency pause mechanism
- [ ] Timelock for parameter updates
- [ ] Integration with Pyth/Switchboard oracles
- [ ] Dev wallet tracking for dump detection
- [ ] Formal security audit

## Constants

Key protocol parameters (defined in `constants.rs`):

- **Coverage Levels**: Any percentage between 30% and 70% (e.g., 30%, 45%, 50%, 65%, 70%)
- **Duration Range**: 7-30 days
- **Max Position**: $50K per policy
- **Price Dump Threshold**: -40% in 24h
- **Liquidity Drain Threshold**: -50% in 24h
- **Oracle Staleness**: 5 minutes

## License

MIT

## Status

⚠️ **DEVELOPMENT**: This is pre-alpha code. Not audited. Do not use in production.
