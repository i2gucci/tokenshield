# Token Shield 🛡️

**DeFi Insurance for Solana Degens**

A decentralized insurance protocol for high-risk Solana tokens (meme coins, Pump.fun launches). Protect your bags from rugs, dumps, and flash crashes with automated, oracle-verified coverage.

## 🎯 Project Overview

Token Shield protects traders from catastrophic losses in volatile Solana tokens by:
- Pooling insurance premiums into a yield-generating liquidity pool
- Monitoring positions with Pyth oracles for trigger events (>40% dumps, liquidity drains, dev dumps)
- Automatically paying out coverage when triggers are confirmed
- Enabling degens to hold high-conviction positions with capped downside

## 📁 Project Structure

```
token-shield/
├── programs/
│   └── token-shield/         # Anchor smart contracts
│       ├── src/
│       │   ├── lib.rs        # Program entry point
│       │   ├── state.rs      # Data structures
│       │   ├── errors.rs     # Error definitions
│       │   ├── constants.rs  # Protocol constants
│       │   └── instructions/ # Instruction handlers
│       ├── Cargo.toml
│       └── README.md
├── tests/
│   └── token-shield.ts       # Anchor tests
├── index.html                # Landing page with hero, features
├── docs.html                 # Complete protocol documentation
├── app.html                  # VRF-based policy interface
├── styles.css                # Terminal-themed styling
├── script.js                 # Interactive functionality
├── app.js                    # Policy enrollment logic
├── Anchor.toml               # Anchor configuration
├── package.json              # NPM dependencies
└── README.md                 # This file
```

## 🚀 Quick Start

### Local Preview

1. **Open in Browser:**
   - Simply double-click `index.html` to open in your default browser
   - Or right-click → "Open with" → Choose your browser

2. **Live Server (Recommended):**
   ```powershell
   # If you have Python installed:
   python -m http.server 8000
   
   # Then visit: http://localhost:8000
   ```

3. **VS Code Live Server:**
   - Install "Live Server" extension in VS Code
   - Right-click `index.html` → "Open with Live Server"

### Smart Contract Development

**Prerequisites:**
- [Rust](https://www.rust-lang.org/tools/install) 1.70+
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) 1.17+
- [Anchor](https://www.anchor-lang.com/docs/installation) 0.29+
- [Node.js](https://nodejs.org/) 18+

**Setup:**
```bash
# Install dependencies
yarn install

# Build the Anchor program
anchor build

# Run tests
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

**Local Development:**
```bash
# Start local validator (in one terminal)
solana-test-validator

# Deploy to local cluster (in another terminal)
anchor deploy

# Run tests against local validator
anchor test --skip-local-validator
```

See [Smart Contract README](./programs/token-shield/README.md) for detailed architecture documentation.

## 📋 Recent Updates

**Code Improvements** (see [CODE_FIXES.md](CODE_FIXES.md)):
- ✅ Added missing `initialize_oracle_data` instruction
- ✅ Fixed PDA constraints across all instructions
- ✅ Corrected CPI authority in payout execution
- ✅ Enhanced test suite with proper setup sequence
- ✅ Added GitHub Actions CI/CD workflow

## ✨ Features

### Landing Page (`index.html`)
- **Hero Section:** Gradient headline, animated chart visualization, stats
- **Features Grid:** 6 key protocol features with hover effects
- **How It Works:** 4-step process with code examples
- **Use Cases:** Target audiences (meme traders, Pump.fun enjoyers, bag holders)
- **FAQ Accordion:** Common questions with expandable answers
- **CTA Section:** Email waitlist signup
- **Waitlist Modal:** Full signup form with risk acceptance

### Documentation (`docs.html`)
- **Complete Protocol Specs:** 2500+ word technical documentation
- **Sticky Sidebar Nav:** Quick navigation to all sections
- **Coverage Topics:**
  - Introduction & Problem Statement
  - Solution Architecture
  - Token Eligibility Criteria
  - Premium Pricing Math
  - Claim Triggers & Payouts
  - Pool Sustainability & Yield
  - Oracle Integration (Pyth)
  - Smart Contract Design
  - ZK Privacy Features
  - Risks & Limitations (transparent!)
  - DAO Governance
  - Roadmap (Q2-Q4 2026)
  - Team & Community

### Styling (`styles.css`)
- **Solana Brand Colors:**
  - Purple: `#9945FF`
  - Teal: `#14F195`
  - Blue: `#00D4FF`
- **Dark Theme:** Modern dark UI optimized for crypto audiences
- **Responsive Design:** Mobile-friendly breakpoints
- **Animations:** Smooth transitions, hover effects, chart animations
- **Typography:** Inter font family for clean readability

### JavaScript (`script.js`)
- FAQ accordion functionality
- Waitlist modal (open/close, form validation)
- Email signup handling
- Smooth scroll for anchor links
- Active nav highlighting in docs
- Mobile menu toggle
- Scroll effects (navbar shadow, parallax)
- Keyboard accessibility (Escape key)
- Utility functions (formatters, validators)

## 🎨 Design Philosophy

- **Edgy but Professional:** Meme culture meets DeFi polish
- **Degen-Friendly:** Language that resonates with high-risk traders
- **Transparent:** Honest about risks and limitations
- **Visual Hierarchy:** Clear CTAs and information flow
- **Solana Ecosystem:** Brand-consistent colors and vibes

## 🛠️ Customization

### Change Colors
Edit CSS variables in `styles.css`:
```css
:root {
    --solana-purple: #9945FF;
    --solana-teal: #14F195;
    --bg-dark: #0a0a0f;
    /* ... */
}
```

### Update Copy
- Hero headlines: Lines 37-43 in `index.html`
- Feature descriptions: Lines 95-138 in `index.html`
- Documentation content: Throughout `docs.html`

### Add Sections
Use existing patterns:
```html
<section class="your-section">
    <div class="container">
        <div class="section-header">
            <h2 class="section-title">Your Title</h2>
            <p class="section-subtitle">Your subtitle</p>
        </div>
        <!-- Your content -->
    </div>
</section>
```

## 📱 Responsive Design

Fully responsive with breakpoints:
- Desktop: 1200px+
- Tablet: 768px - 1199px
- Mobile: < 768px

Mobile menu automatically activates on smaller screens.

## 🔗 Integration Points

For production deployment, update these placeholders:

### Forms (`script.js`)
- Line 43-56: Waitlist modal form submission → Connect to backend API
- Line 62-78: Email CTA form → Connect to email service (Mailchimp, SendGrid)

### Social Links (`index.html`)
- Footer links (lines 280-287): Add real URLs for Twitter, Discord, Telegram, GitHub

### Analytics
Add tracking scripts before closing `</body>` tag:
```html
<!-- Google Analytics -->
<script async src="https://www.googletagmanager.com/gtag/js?id=YOUR-ID"></script>

<!-- Wallet integrations -->
<!-- Add Phantom/Solflare wallet adapters -->
```

## 🚢 Deployment

### GitHub Pages
```bash
git init
git add .
git commit -m "Initial Token Shield website"
git branch -M main
git remote add origin https://github.com/yourusername/token-shield.git
git push -u origin main

# Enable GitHub Pages in repo settings → Pages → Source: main branch
```

### Netlify / Vercel
1. Push code to GitHub
2. Import repo to Netlify/Vercel
3. Deploy (automatic builds on push)

### IPFS (Decentralized)
```bash
# Install IPFS
# Add folder to IPFS
ipfs add -r token-shield/
# Get CID and pin to service (Pinata, Filebase)
```

## ⚠️ Disclaimer

This is an **experimental DeFi protocol** in active development. The smart contracts are in **pre-alpha** stage and have **NOT been audited**. 

**Do NOT use with real funds until:**
- ✅ Smart contracts are formally audited (2+ independent firms)
- ✅ Economic model stress-tested
- ✅ Bug bounty program completed
- ✅ Mainnet beta phase successful

Use at your own risk. Not financial advice. DYOR.

**Important Notes:**
- Not financial advice
- DYOR (Do Your Own Research)
- High experimental risk
- Regulatory uncertainty
- See full risks in documentation

## 📋 Roadmap Status

- ✅ Concept & Documentation
- ✅ Website & Landing Page
- ⏳ Smart Contract Development (Q2 2026)
- ⏳ Testnet Launch (Q3 2026)
- ⏳ Audits & Security (Q3 2026)
- ⏳ Mainnet Launch (Q4 2026)

## 🤝 Contributing

Interested in contributing to Token Shield?

**We're looking for:**
- Smart contract developers (Anchor/Rust)
- Frontend developers (React/Next.js)
- Quantitative analysts (premium pricing models)
- Security researchers (auditing)
- Community managers
- DeFi partnerships

**Coming Soon:**
- Discord server
- GitHub org
- Contribution guidelines
- Bug bounty program

## 📄 License

[To be determined - likely MIT or Apache 2.0 for open source]

## 🙏 Acknowledgments

- **Solana Foundation** for ecosystem support
- **Pyth Network** for oracle inspiration
- **DeFi insurance protocols** (Nexus Mutual, Unslashed) for pioneering the space
- **Degen community** for being the target audience that keeps crypto fun

---

**Built with 💜 for the Solana degen community**

Stay based, stay protected. 🛡️
