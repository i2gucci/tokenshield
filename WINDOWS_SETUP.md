# Token Shield - Windows Development Setup

Complete guide for setting up the Anchor development environment on Windows.

## Prerequisites

- Windows 10/11 (64-bit)
- Administrator privileges
- ~10 GB disk space

## Installation Steps

### 1. Install WSL2 (Recommended)

Anchor works best on Linux. WSL2 provides a Linux environment on Windows.

```powershell
# Run in PowerShell as Administrator
wsl --install
```

This installs Ubuntu by default. Restart your computer when prompted.

After restart, open Ubuntu from Start menu and create a username/password.

### 2. Install Rust (in WSL2)

```bash
# In WSL2 Ubuntu terminal
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow prompts, choose default installation
# Then source the environment
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### 3. Install Solana CLI

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y pkg-config build-essential libudev-dev libssl-dev

# Install Solana CLI (version 1.17.x for Anchor 0.29.0)
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"

# Add to PATH (add to ~/.bashrc for persistence)
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Verify installation
solana --version
```

### 4. Install Node.js and Yarn

```bash
# Install Node.js 18.x
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install Yarn
npm install -g yarn

# Verify installations
node --version
yarn --version
```

### 5. Install Anchor CLI

```bash
# Install Anchor version manager (avm)
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

# Install Anchor 0.29.0
avm install 0.29.0
avm use 0.29.0

# Verify installation
anchor --version
```

### 6. Clone and Setup Project

```bash
# Navigate to project directory (Windows drives are at /mnt/c, /mnt/d, etc.)
cd /mnt/c/Users/taylo/Desktop/token-shield

# Install Node dependencies
yarn install

# Build the project
anchor build
```

### 7. Configure Solana for Local Development

```bash
# Set localhost cluster
solana config set --url localhost

# Create a new keypair (or use existing)
solana-keygen new

# Start local validator (in separate terminal)
solana-test-validator

# In another terminal, check validator is running
solana cluster-version
```

### 8. Run Tests

```bash
# Make sure validator is running
# In project directory
anchor test
```

## Alternative: Native Windows Installation (Not Recommended)

### Using Native Rust on Windows

1. Install Rust from https://rustup.rs/
2. Install Visual Studio C++ Build Tools
3. Install Solana CLI (Windows builds available but limited)
4. Note: Anchor CLI has limited Windows support

**⚠️ Warning**: Many Anchor features don't work properly on native Windows. WSL2 is strongly recommended.

## Common Issues

### "anchor: command not found"

```bash
# Ensure Anchor is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Or reinstall Anchor
avm install 0.29.0
avm use 0.29.0
```

### "Error: cargo is not recognized"

This means you're in Windows PowerShell, not WSL2. Open "Ubuntu" from Start menu.

### Build fails with missing dependencies

```bash
# Install all required packages
sudo apt-get update
sudo apt-get install -y \
    pkg-config \
    build-essential \
    libudev-dev \
    libssl-dev \
    python3 \
    git
```

### "Unable to connect to validator"

```bash
# Start local validator
solana-test-validator

# In another terminal, check connection
solana cluster-version
```

## Development Workflow

### 1. Start Local Validator

```bash
# Terminal 1
solana-test-validator
```

### 2. Build and Deploy

```bash
# Terminal 2
anchor build
anchor deploy
```

### 3. Run Tests

```bash
anchor test
```

### 4. Watch Mode (auto-rebuild on changes)

```bash
# Install cargo-watch
cargo install cargo-watch

# Run in watch mode
cargo watch -x "build-bpf"
```

## VS Code Setup

### Recommended Extensions

Install these in VS Code:

- **rust-analyzer** - Rust language support
- **Anchor Snippets** - Anchor code snippets
- **Better TOML** - TOML file support
- **WSL** - Remote development in WSL2

### Open Project in WSL

1. Install "WSL" extension in VS Code
2. Click green icon in bottom-left corner
3. Select "Connect to WSL"
4. File > Open Folder > `/mnt/c/Users/taylo/Desktop/token-shield`

### Configure rust-analyzer

Create `.vscode/settings.json`:

```json
{
  "rust-analyzer.cargo.target": "bpfel-unknown-unknown",
  "rust-analyzer.checkOnSave.allTargets": false,
  "rust-analyzer.check.targets": ["bpfel-unknown-unknown"]
}
```

## Environment Variables

Add to `~/.bashrc` or `~/.zshrc`:

```bash
# Solana
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Rust
export PATH="$HOME/.cargo/bin:$PATH"

# Anchor
export ANCHOR_WALLET="$HOME/.config/solana/id.json"

# Optional: Set cluster
export SOLANA_CLUSTER="devnet"  # or "localhost" or "mainnet-beta"
```

Reload shell:
```bash
source ~/.bashrc
```

## Next Steps

- Run `anchor test` to verify setup
- Review smart contract code in `programs/token-shield/src/`
- Modify tests in `tests/token-shield.ts`
- Check frontend integration guide in main README

## Resources

- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [WSL2 Setup Guide](https://learn.microsoft.com/en-us/windows/wsl/install)
- [Rust Book](https://doc.rust-lang.org/book/)

## Troubleshooting

If you encounter issues:

1. Check Anchor version: `anchor --version` (should be 0.29.0)
2. Check Solana version: `solana --version` (should be 1.17.x)
3. Ensure validator is running: `solana cluster-version`
4. Check logs: `tail -f test-ledger/validator.log`
5. Clean build: `anchor clean && anchor build`

## Support

- GitHub Issues: https://github.com/your-repo/token-shield/issues
- Anchor Discord: https://discord.gg/anchor
