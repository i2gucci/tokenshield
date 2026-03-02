# Token Shield 🛡️

**Decentralized Insurance Protocol for Solana**

Token Shield is a DeFi insurance protocol built on Solana that protects traders from catastrophic losses in high-risk tokens. Get automated, oracle-verified payouts when your positions dump, rug, or lose liquidity.

[![Built with Anchor](https://img.shields.io/badge/Built%20with-Anchor-blueviolet)](https://www.anchor-lang.com/)
[![Solana](https://img.shields.io/badge/Solana-1.16+-blue)](https://solana.com)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## What We Do

Token Shield provides automated coverage for meme coins, Pump.fun launches, and volatile Solana tokens. When your insured token dumps >40%, liquidity drains >50%, or dev wallets sell—you get paid out automatically via smart contract.

### Key Features

- **Automated Triggers** - Oracle-verified price/liquidity monitoring  
- **Instant Payouts** - CPI transfers execute within seconds of trigger  
- **No Middlemen** - 100% on-chain, no claims departments  
- **Yield-Generating** - Premiums deployed to Marinade/Jito/Marginfi for 6.2% APY
- **Custom Coverage** - Choose any protection level from 30-70%
- **Risk Scoring** - Dynamic premiums based on token age, liquidity, volatility

## How It Works

1. **Pool Premiums** - Insurance buyers pay premiums into a shared liquidity pool
2. **Monitor Positions** - Pyth/Switchboard oracles track token prices and liquidity  
3. **Trigger Events** - Smart contracts detect qualifying dump/rug events automatically
4. **Execute Payouts** - Coverage is paid directly from the pool to affected policyholders

## Technical Architecture

### Smart Contract (Anchor/Rust)

- **12 Instructions:** `initialize_pool`, `create_policy`, `check_trigger`, `execute_payout`, etc.
- **State Accounts:** Pool, Policy, TriggerEvent, OracleData
- **Oracle Integration:** Pyth & Switchboard for price/liquidity data
- **Token Standard:** SPL tokens for policy NFTs
- **Security:** PDA-based authority, re-entrancy guards

**Program ID:** `Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS`

### Key Instructions

```rust
// Create individual coverage policy
pub fn create_policy(
    ctx: Context<CreatePolicy>,
    token_mint: Pubkey,
    position_size: u64,
    coverage_level_bps: u16,  // 3000-7000 (30-70%)
    duration_days: u16,
) -> Result<()>

// Check trigger conditions and initiate payout
pub fn check_and_trigger_payout(
    ctx: Context<CheckTrigger>,
) -> Result<()>

// Execute verified payout
pub fn execute_payout(
    ctx: Context<ExecutePayout>,
) -> Result<()>
```

### Eligibility Requirements

- **Minimum Token Age:** 7 days (or Team Shield pre-launch)
- **Minimum Market Cap:** $500k USD
- **Minimum Position Value:** $1,000 USD
- **Supported Coverage:** 30-70% of position value
- **Maximum Duration:** 90 days

Full details: [eligibility-requirements.md](.internal-docs/eligibility-requirements.md)

## Learn More

- **Website:** [tokenshield.fun](https://tokenshield.fun)
- **Documentation:** [tokenshield.fun/docs](https://tokenshield.fun/docs)
- **Try Interface:** [tokenshield.fun/app](https://tokenshield.fun/app) *(demo only)*

## Repository Structure

```
token-shield/
├── programs/token-shield/src/
│   ├── lib.rs                      # Program entry point
│   ├── state.rs                    # Account structures (Pool, Policy, etc.)
│   ├── errors.rs                   # Custom error types
│   ├── constants.rs                # Protocol parameters
│   └── instructions/
│       ├── initialize_pool.rs      # Pool setup
│       ├── create_policy.rs        # Policy creation & premium calc
│       ├── check_trigger.rs        # Event detection
│       ├── execute_payout.rs       # Payout execution
│       └── ... (8 more instructions)
├── tests/
│   └── token-shield.ts             # Anchor integration tests
├── web/
│   ├── index.html                  # Landing page
│   ├── docs.html                   # Documentation
│   ├── app.html                    # Policy interface
│   └── app.js                      # Frontend logic
├── .internal-docs/                 # Design docs (20+ files)
├── Anchor.toml                     # Anchor config
└── README.md                       # This file
```

## Development

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) 1.16+
- [Anchor](https://www.anchor-lang.com/docs/installation) 0.29.0
- Node.js 16+

### Build

```bash
# Install dependencies
anchor build

# Run tests
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

### Test Coverage

- ✅ Pool initialization
- ✅ Oracle data setup
- ✅ Policy creation
- ✅ Premium calculation
- ⏳ Trigger detection (needs oracle integration)
- ⏳ Payout execution (needs funded pool)

## Current Status

**🚧 Active Development - Devnet Testing Phase**

Token Shield smart contracts are implemented and undergoing testing. **NOT audited** - do not use with real funds.

- ✅ Smart Contract Implementation (2,000+ lines Rust)
- ✅ 12 Instructions fully implemented  
- ✅ Oracle integration (Pyth & Switchboard)
- ✅ Frontend interface & documentation
- ✅ Token eligibility requirements  
- 🔄 Devnet deployment & testing (in progress)
- ⏳ Security audit (planned Q2 2026)
- ⏳ Mainnet launch (Q3 2026)

### Recent Activity (Feb 28, 2026)

- ✅ Fixed Solana dependency conflicts (1.16.0)
- ✅ Built Anchor program successfully
- ✅ Custom coverage levels (30-70% range)
- ✅ Added token eligibility (7-day age, $500k mcap)
- 🔄 Running integration tests

## Disclaimer

This is an experimental DeFi protocol. Smart contracts are unaudited. Use at your own risk. Not financial advice. DYOR.

---

**Built with 💜 for the trenches** 
