# LedgerFlow - Web3 Payment Gateway

**Make Value Flow Like Information.**

A modern, dark-themed landing page for a Web3 payment gateway built with Leptos and Rust. LedgerFlow showcases a permissionless, self-custody payment solution for creators and developers in the Web3 ecosystem.

## 🌟 Features

- 🌙 **Dark Web3 Theme**: Modern UI with blue and purple glowing accents
- ⚡ **Rust Performance**: Built with Leptos for lightning-fast WASM performance
- 📱 **Fully Responsive**: Optimized for desktop and mobile devices
- 🎨 **Smooth Animations**: CSS animations with fade-in effects and hover states
- 🔧 **Easy Configuration**: Simple config constants for quick customization
- 🌐 **Static Deployment**: Zero server requirements, deploy anywhere

## 🏗️ Page Structure

The landing page is built with modular Leptos components:

### 🧭 Navigation (`Navbar`)
- Logo with brand name
- Navigation links: "Why LedgerFlow?", "Features", "How it Works"
- Action buttons: "GitHub" and "Get Started"
- Mobile-responsive menu toggle

### 🚀 Hero Section (`HeroSection`)
- Primary headline with animated background
- Value proposition subtitle
- Dual CTAs: "View on GitHub" and "Start Receiving Payments via Telegram"
- Supporting text with fade-in animations

### ❌ Problem Section (`ProblemSection`)
- Title: "Tired of Traditional Payment Rails?"
- Four problem cards:
  - 🏢 **High Barriers**: Company registration requirements
  - 🔒 **Frozen Funds**: Platform control over user funds
  - 💸 **Hidden Fees**: Unpredictable costs and conversion fees
  - 🌍 **Geographic Walls**: Banking and country restrictions

### ✅ Solution Section (`SolutionSection`)  
- Title: "Welcome to the Future of Payments."
- Four solution cards:
  - ⚡ **Truly Permissionless**: Only crypto address needed
  - 🔐 **Self-Custody**: Direct on-chain vault control
  - 💎 **Radical Transparency**: Only blockchain gas fees
  - 🌐 **Genuinely Global**: Internet-only requirements

### 🔄 How It Works Section (`HowItWorksSection`)
- Title: "Simple & Secure in 3 Steps"
- Three-step process:
  1. **Create Link**: Generate payment request via Telegram Bot
  2. **Customer Pays**: USDC sent to PaymentVault contract
  3. **Receive Funds**: On-chain confirmation and instant vault arrival

### 🛠️ Features Section (`FeaturesSection`)
- Title: "Built for the Web3 Economy"
- Four core features:
  - 🏛️ **Non-Custodial Vault**: Single audited smart contract
  - 🔗 **Seamless Multi-Chain**: EVM chain compatibility
  - ⚙️ **Programmable & Composable**: DeFi integration ready
  - ⛽ **Gasless Experience**: Permit signature support

### 📞 Final CTA Section (`FinalCTASection`)
- Title: "Ready to Join the Open Financial Network?"
- Primary action: "Launch the Telegram Bot"
- Secondary action: "View on GitHub"
- Testnet contract link on Uniscan

### 🏠 Footer (`Footer`)

- Copyright notice with current year
- Built with Leptos & Rust branding

## 🚀 Quick Start

### Prerequisites

- **Rust**: Latest stable version
- **Trunk**: WASM web application bundler

### Installation

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Trunk
cargo install trunk

# Or install via npm
npm install -g trunk
```

### Development

```bash
# Clone the repository
git clone https://github.com/longcipher/ledgerflow-homepage.git
cd ledgerflow-homepage

# Start development server
trunk serve

# Open browser to http://127.0.0.1:8080
```

### Production Build

```bash
# Build for production
trunk build --release

# Files will be in dist/ directory
```

## 🎨 Design System

### Color Palette

- **Primary Blue**: `#3b82f6` - Technology and trust
- **Secondary Purple**: `#8b5cf6` - Innovation and Web3
- **Accent Cyan**: `#06b6d4` - Flow and movement
- **Background**: `#0a0a0a` / `#111111` - Dark sophistication
- **Text**: `#ffffff` to `#71717a` - Clarity gradient

### Typography

- **Font Family**: Inter (Google Fonts)
- **Weights**: 100-900 available
- **Hierarchy**: Clear distinction between titles, subtitles, and body text

### Visual Effects

- **Glow Effects**: Blue and purple shadows on interactive elements
- **Gradient Text**: Primary headlines use blue-to-purple gradients
- **Hover States**: Cards lift and glow on interaction
- **Animations**: Fade-in effects with staggered delays

## 🛠️ Configuration

### Core Settings

Edit the `CONFIG` struct in `src/main.rs`:

```rust
const CONFIG: LedgerFlowConfig = LedgerFlowConfig {
    brand_name: "LedgerFlow",
    hero_title: "Make Value Flow Like Information.",
    hero_subtitle: "The open source, permissionless, self-custody payment gateway...",
    telegram_bot_url: "https://t.me/LedgerFlowBot",
    github_url: "https://github.com/longcipher/ledgerflow",
    testnet_contract_url: "https://sepolia.uniscan.xyz/address/0x...",
};
```

### Content Arrays

Modify the problem/solution/feature arrays:

```rust
// Problems users face
const PROBLEMS: &[(&str, &str, &str)] = &[
    ("🏢", "High Barriers", "Forced to register a company..."),
    // Add more problems...
];

// Solutions LedgerFlow provides
const SOLUTIONS: &[(&str, &str, &str)] = &[
    ("⚡", "Truly Permissionless", "All you need is a crypto address..."),
    // Add more solutions...
];

// Core features
const FEATURES: &[(&str, &str, &str)] = &[
    ("🏛️", "Non-Custodial Vault", "A single, audited smart contract..."),
    // Add more features...
];
```

### Styling Customization

Update CSS variables in `style.css`:

```css
:root {
  --color-primary: #3b82f6;
  --color-secondary: #8b5cf6;
  --color-accent: #06b6d4;
  --color-bg-primary: #0a0a0a;
  --color-bg-secondary: #111111;
  /* ... more variables */
}
```

## 📦 Tech Stack

- **Frontend Framework**: [Leptos](https://leptos.dev/) - Rust-based reactive UI
- **Build Tool**: [Trunk](https://trunkrs.dev/) - WASM application bundler
- **Styling**: Custom CSS with CSS variables and animations
- **Icons**: Unicode emojis for lightweight, universal compatibility
- **Fonts**: Inter from Google Fonts
- **Deployment**: Static files, compatible with any hosting

## 🏗️ Project Structure

```text
ledgerflow-homepage/
├── src/
│   └── main.rs                 # Main Leptos components and config
├── dist/                       # Built files (generated)
├── target/                     # Rust build artifacts
├── index.html                  # HTML template
├── style.css                   # Dark theme CSS
├── Cargo.toml                  # Rust dependencies
├── Trunk.toml                  # Trunk configuration
├── Justfile                    # Development commands
├── leptosfmt.toml             # Leptos formatter config
└── README.md                   # This file
```

## 🌐 Deployment

### Static Hosting

The app builds to static files, deployable anywhere:

```bash
trunk build --release
```

Deploy the `dist/` folder to:

- **Vercel**: Connect GitHub repo for automatic deployments
- **Netlify**: Drag and drop dist folder or connect repo
- **GitHub Pages**: Deploy from gh-pages branch
- **AWS S3**: Static website hosting
- **IPFS**: Decentralized hosting for Web3 alignment

### Development Commands

Using Just (justfile):

```bash
# Format code
just format

# Lint code
just lint

# Run tests
just test
```

Or using Cargo directly:

```bash
# Format Rust code
cargo fmt

# Check for issues
cargo clippy

# Run tests
cargo test
```

## 🎯 Target Audience

This landing page is designed for:

- **Web3 Developers**: Clear technical messaging and integration focus
- **Crypto Creators**: Self-custody and permissionless value propositions
- **DeFi Users**: Emphasis on transparency and composability
- **Global Entrepreneurs**: Borderless payment solutions

## 🔗 External Links

- **Live Demo**: [LedgerFlow Homepage](https://ledgerflow.dev)
- **Telegram Bot**: [@LedgerFlowBot](https://t.me/LedgerFlowBot)
- **GitHub Repository**: [longcipher/ledgerflow](https://github.com/longcipher/ledgerflow)
- **Testnet Contract**: [Sepolia Uniscan](https://sepolia.uniscan.xyz/address/0x8b6f22009ae835795b9b33d75ad218c730db039b)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `just lint` to ensure code quality
5. Submit a pull request

## 📄 License

MIT License - Feel free to use this template for your Web3 projects.

---

**Built with ❤️ using Leptos and Rust for the decentralized future.**
