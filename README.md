# Token Shield 🛡️

**DeFi Insurance for Solana Degens**

Token Shield is a decentralized insurance protocol that protects traders from catastrophic losses in high-risk Solana tokens. We enable degens to hold high-conviction positions with capped downside risk.

## What We Do

Token Shield provides automated, oracle-verified coverage for meme coins, Pump.fun launches, and other volatile Solana tokens. When your insured token dumps >40%, liquidity drains, or dev wallets dump—you get paid out automatically.

## How It Works

1. **Pool Premiums** - Insurance buyers pay premiums into a shared liquidity pool
2. **Monitor Positions** - Pyth oracles track token prices and liquidity in real-time  
3. **Trigger Events** - Smart contracts detect qualifying dump/rug events automatically
4. **Execute Payouts** - Coverage is paid directly from the pool to affected policyholders

## Recent Updates (Feb 28, 2026)

### Token Eligibility Requirements
- **Minimum Token Age:** 7 days from launch (teams can apply pre-launch via Team Shield)
- **Minimum Market Cap:** $500k USD (adjustable based on SOL price)
- **Full criteria:** See [eligibility-requirements.md](.internal-docs/eligibility-requirements.md)

### Custom Coverage Levels
- Users can now select **any percentage between 30-70%** (not just preset 30%, 50%, 70%)
- Example: Choose 35%, 47%, 63%, or any custom level
- More flexibility in risk/reward balance

## Learn More

- **Website:** [tokenshield.fun](https://tokenshield.fun)
- **Documentation:** [tokenshield.fun/docs](https://tokenshield.fun/docs)
- **Try Interface:** [tokenshield.fun/app](https://tokenshield.fun/app) *(demo only)*

## Repository Structure

```
token-shield/
├── web/                  # Frontend website files
│   ├── index.html        # Landing page
│   ├── docs.html         # Documentation
│   ├── app.html          # Policy interface
│   ├── styles.css        # Styling
│   ├── script.js         # UI interactions
│   └── app.js           # Policy logic
├── programs/             # Solana smart contracts (Anchor/Rust)
├── tests/               # Smart contract tests
├── CNAME                # Custom domain config
└── README.md            # This file
```

## Current Status

**⚠️ Pre-Alpha Development - Not Production Ready**

Token Shield is in active development. Smart contracts have **NOT been audited**. Do not use with real funds.

- ✅ Concept & Documentation
- ✅ Website & Landing Page  
- ⏳ Smart Contract Development (Q2 2026)
- ⏳ Testnet Launch (Q3 2026)
- ⏳ Security Audits (Q3 2026)
- ⏳ Mainnet Launch (Q4 2026)

## Disclaimer

This is an experimental DeFi protocol. Smart contracts are unaudited. Use at your own risk. Not financial advice. DYOR.

---

**Built with 💜 for the trenches** 
