# LedgerFlow - Web3 Payment Gateway

**Make Value Flow Like Information.**

LedgerFlow is a modern, dark-themed landing page for a Web3 payment gateway built with Leptos and Rust. It showcases a permissionless, self-custody payment solution for creators and developers in the Web3 ecosystem.

## 🌟 Features

- 🌙 **Dark Theme**: Modern, technological design with blue and purple glowing accents
- ⚡ **Web3 Native**: Built for the decentralized economy
- 📱 **Responsive**: Perfect on desktop and mobile devices
- 🚀 **High Performance**: Powered by Rust and Leptos
- 🎨 **Animated**: Smooth animations and glowing effects
- � **Self-Custody Focus**: Emphasizes user control and security

## 🏗️ Page Structure

### 🧭 Navigation
- **Logo**: LedgerFlow with gradient text
- **Menu**: Why LedgerFlow? | Features | How it Works
- **CTA Button**: Get Started (links to Telegram Bot)

### 🚀 Hero Section
- **Headline**: "Make Value Flow Like Information."
- **Subtitle**: Explains the permissionless, self-custody value proposition
- **Primary CTA**: "Start Receiving Payments via Telegram"
- **Sub-text**: "No registration, no borders, just pure value flow."

### ❌ Problem Section
- **Title**: "Tired of Traditional Payment Rails?"
- **Problems**: High Barriers, Frozen Funds, Hidden Fees, Geographic Walls

### ✅ Solution Section  
- **Title**: "Welcome to the Future of Payments."
- **Solutions**: Permissionless, Self-Custody, Transparent, Global

### 🔄 How It Works
- **Title**: "Simple & Secure in 3 Steps"
- **Steps**: Create Link → Customer Pays → Receive Funds

### 🛠️ Features Section
- **Title**: "Built for the Web3 Economy"
- **Features**: Non-Custodial Vault, Multi-Chain, Programmable, Gasless

### 📞 Final CTA
- **Title**: "Ready to Join the Open Financial Network?"
- **Primary Button**: "Launch the Telegram Bot"
- **Secondary Link**: "View our Testnet Contract on Uniscan"

## 🚀 Development and Deployment

### Prerequisites

- Rust toolchain
- Trunk (for building WASM applications)

### Install Dependencies

```bash
# Install Trunk
cargo install trunk

# Or using npm
npm install -g trunk
```

### Run Development Server

```bash
trunk serve
```

Visit http://127.0.0.1:8080 to view the page

### Build for Production

```bash
trunk build --release
```

## 🎨 Design Features

### Color Palette

- **Primary**: Blue (#3b82f6) - Technology and trust
- **Secondary**: Purple (#8b5cf6) - Innovation and Web3
- **Accent**: Cyan (#06b6d4) - Flow and movement
- **Background**: Deep blacks (#0a0a0a, #111111) - Sophistication
- **Text**: White to gray gradient - Clarity and readability

### Visual Effects

- **Glow Effects**: Blue and purple shadows on interactive elements
- **Gradient Text**: Primary headlines use blue-to-purple gradients
- **Floating Animation**: Subtle background element movement
- **Hover Transforms**: Cards lift and glow on interaction
- **Smooth Transitions**: All interactions have 0.3s ease timing

### Typography

- **Font**: Inter - Modern, clean, and highly readable
- **Hierarchy**: Clear distinction between titles, subtitles, and body text
- **Weights**: Strategic use of font weights (400-800) for emphasis

## 🛠️ Customization Guide

### Quick Content Updates

Edit the configuration constants in `src/main.rs`:

```rust
const CONFIG: LedgerFlowConfig = LedgerFlowConfig {
    brand_name: "YourBrand",
    hero_title: "Your Custom Headline",
    hero_subtitle: "Your value proposition...",
    telegram_bot_url: "https://t.me/YourBot",
    testnet_contract_url: "https://explorer.com/contract/0x...",
};
```

### Updating Problems and Solutions

Modify the arrays in `src/main.rs`:

```rust
const PROBLEMS: &[(&str, &str, &str)] = &[
    ("�", "Your Problem", "Problem description..."),
    // Add more problems...
];

const SOLUTIONS: &[(&str, &str, &str)] = &[
    ("⚡", "Your Solution", "Solution description..."),
    // Add more solutions...
];
```

### Color Theme Customization

Update CSS variables in `style.css`:
```css
:root {
  --color-primary: #3758f9;      /* 主要颜色 */
  --color-dark: #111928;         /* 深色文本 */
  --gradient-primary: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  --gradient-secondary: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
}
```

### 3. 添加新功能特性
要添加更多功能特性，只需在 `FEATURES` 数组中添加新的元组：
```rust
const FEATURES: &[(&str, &str, &str)] = &[
    ("🚀", "Lightning Fast", "描述..."),
    ("🛡️", "Secure & Reliable", "描述..."),
    ("�", "Cross Platform", "描述..."),
    ("🎨", "Beautiful Design", "新特性描述..."),  // 新增
    ("⚡", "High Performance", "另一个新特性..."), // 新增
];
```

### 4. 修改页面内容
- **导航栏**: 在 `Navbar` 组件中修改链接和文本
- **英雄区域**: 在 `HeroSection` 组件中修改按钮文本
- **CTA区域**: 在 `CTASection` 组件中修改文案和表单
- **页脚**: 在 `Footer` 组件中修改版权信息

### 5. 品牌定制
- 修改 `CONFIG.brand_name` 更新品牌名称
- 更新 `index.html` 中的页面标题和元数据
- 替换 favicon (当前使用火箭 emoji)

## 技术栈

## 📦 Tech Stack

- **Frontend Framework**: Leptos (Rust-based reactive UI)
- **Build Tool**: Trunk (WASM application bundler)
- **Styling**: Custom CSS with CSS variables
- **Icons**: Unicode emojis for lightweight design
- **Fonts**: Google Fonts (Inter)
- **Deployment**: Static hosting compatible

## 🏗️ Project Structure

```
leptos-tutorial/
├── src/
│   └── main.rs          # Main Leptos components and configuration
├── style.css            # Dark theme CSS with Web3 styling
├── index.html           # HTML template with LedgerFlow branding
├── Cargo.toml           # Rust dependencies
├── .gitignore           # Comprehensive ignore patterns
└── README.md            # Project documentation
```

## 🚀 Deployment

### Static Deployment

Build for production:

```bash
trunk build --release
```

Deploy the `dist/` folder to:

- **Vercel**: Import your repository and deploy
- **Netlify**: Drag and drop the dist folder
- **GitHub Pages**: Push to gh-pages branch
- **IPFS**: For decentralized hosting
- **Any static host**: Upload the dist contents

### Environment Setup

1. **Domain Configuration**: Update any absolute URLs
2. **Analytics**: Add your tracking codes to `index.html`
3. **SEO**: Update meta tags and structured data
4. **Performance**: Enable compression and CDN

## 🎯 Target Audience

This landing page is designed for:

- **Web3 Developers**: Technical documentation and clear integration steps
- **Crypto-native Creators**: Self-custody and permissionless messaging
- **DeFi Enthusiasts**: Emphasis on transparency and composability
- **Global Users**: Borderless and accessible design

## 🔗 Key Links

- **Telegram Bot**: [https://t.me/LedgerFlowBot](https://t.me/LedgerFlowBot)
- **Testnet Contract**: Placeholder for Uniscan explorer link
- **Documentation**: Can be added to navigation
- **GitHub**: Repository for open-source credibility

## 🤝 Contributing

We welcome issues and pull requests to improve this Web3 landing page template.

## 📄 License

MIT License - Feel free to use this template for your own Web3 projects.

---

**Built with ❤️ using Leptos and Rust for the decentralized future.**
