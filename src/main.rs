use leptos::prelude::*;

// LedgerFlow Configuration
struct LedgerFlowConfig {
    brand_name: &'static str,
    hero_title: &'static str,
    hero_subtitle: &'static str,
    telegram_bot_url: &'static str,
    github_url: &'static str,
    testnet_contract_url: &'static str,
}

const CONFIG: LedgerFlowConfig = LedgerFlowConfig {
    brand_name: "LedgerFlow",
    hero_title: "Make Value Flow Like Information.",
    hero_subtitle: "The open souce, permissionless, self-custody payment gateway for modern creators and developers. Accept global crypto payments without barriers, frozen accounts, or hidden fees.",
    telegram_bot_url: "https://t.me/LedgerFlowBot",
    github_url: "https://github.com/longcipher/ledgerflow",
    testnet_contract_url: "https://sepolia.uniscan.xyz/address/0x8b6f22009ae835795b9b33d75ad218c730db039b",
};

// Problems Configuration
const PROBLEMS: &[(&str, &str, &str)] = &[
    (
        "🏢",
        "High Barriers",
        "Forced to register a company just to receive payments.",
    ),
    (
        "🔒",
        "Frozen Funds",
        "Your money is held hostage by platforms with opaque rules.",
    ),
    (
        "💸",
        "Hidden Fees",
        "Unpredictable costs from currency conversion and cross-border fees.",
    ),
    (
        "🌍",
        "Geographic Walls",
        "Locked out of the global market by banking and country restrictions.",
    ),
];

// Solutions Configuration
const SOLUTIONS: &[(&str, &str, &str)] = &[
    (
        "⚡",
        "Truly Permissionless",
        "All you need is a crypto address. Start in seconds.",
    ),
    (
        "�",
        "Self-Custody",
        "Funds go directly to your on-chain vault. Only you control them.",
    ),
    (
        "💎",
        "Radical Transparency",
        "Pay only for blockchain gas. No hidden platform fees.",
    ),
    (
        "🌐",
        "Genuinely Global",
        "Accept payments from anyone, anywhere in the world with internet.",
    ),
];

// How It Works Configuration
const HOW_IT_WORKS: &[(&str, &str, &str)] = &[
    (
        "1",
        "Create Link",
        "Generate a unique payment request via our Telegram Bot.",
    ),
    (
        "2",
        "Customer Pays",
        "The payer sends USDC to the secure PaymentVault contract.",
    ),
    (
        "3",
        "Receive Funds",
        "Payment is confirmed on-chain and funds arrive in your vault instantly.",
    ),
];

// Core Features Configuration
const FEATURES: &[(&str, &str, &str)] = &[
    (
        "🏛️",
        "Non-Custodial Vault",
        "A single, audited smart contract secures all funds, eliminating platform risk.",
    ),
    (
        "🔗",
        "Seamless Multi-Chain",
        "Deploy on any EVM chain. Accept payments on Ethereum, Polygon, Base, and more.",
    ),
    (
        "⚙️",
        "Programmable & Composable",
        "Ready for subscriptions, DeFi integrations, and multi-token payments.",
    ),
    (
        "⛽",
        "Gasless Experience",
        "Support for permit signatures means a smoother, user-friendly payment flow.",
    ),
];

#[component]
fn App() -> impl IntoView {
    view! {
      <div class="app">
        <Navbar />
        <HeroSection />
        <ProblemSection />
        <SolutionSection />
        <HowItWorksSection />
        <FeaturesSection />
        <FinalCTASection />
        <Footer />
      </div>
    }
}

#[component]
fn Navbar() -> impl IntoView {
    let (is_scrolled, _set_is_scrolled) = signal(false);

    view! {
      <nav class=move || format!("navbar {}", if is_scrolled.get() { "scrolled" } else { "" })>
        <div class="nav-container">
          <a href="#" class="logo">
            {CONFIG.brand_name}
          </a>
          <ul class="nav-links">
            <li>
              <a href="#problems" class="nav-link">
                "Why LedgerFlow?"
              </a>
            </li>
            <li>
              <a href="#features" class="nav-link">
                "Features"
              </a>
            </li>
            <li>
              <a href="#how-it-works" class="nav-link">
                "How it Works"
              </a>
            </li>
            <li>
              <a href=CONFIG.github_url target="_blank" class="nav-link">
                "GitHub"
              </a>
            </li>
          </ul>
          <div class="nav-buttons">
            <a href=CONFIG.github_url target="_blank" class="btn btn-nav btn-secondary">
              "GitHub"
            </a>
            <a href=CONFIG.telegram_bot_url target="_blank" class="btn btn-nav">
              "Get Started"
            </a>
          </div>
          <button class="mobile-menu-toggle">"☰"</button>
        </div>
      </nav>
    }
}

#[component]
fn HeroSection() -> impl IntoView {
    view! {
      <section class="hero-section">
        <div class="hero-background"></div>
        <div class="hero-content fade-in-up">
          <h1 class="hero-title">{CONFIG.hero_title}</h1>
          <p class="hero-subtitle">{CONFIG.hero_subtitle}</p>
          <div class="cta-buttons fade-in-up-delay-1">
            <a href=CONFIG.github_url target="_blank" class="btn btn-secondary">
              "View on GitHub"
            </a>
            <a href=CONFIG.telegram_bot_url target="_blank" class="btn btn-primary">
              "Start Receiving Payments via Telegram"
            </a>
          </div>
          <p class="hero-sub-text fade-in-up-delay-2">
            "No registration, no borders, just pure value flow."
          </p>
        </div>
      </section>
    }
}

#[component]
fn ProblemSection() -> impl IntoView {
    view! {
      <section id="problems" class="problem-section">
        <div class="container">
          <h2 class="section-title fade-in-up">"Tired of Traditional Payment Rails?"</h2>
          <div class="problem-grid">
            {PROBLEMS
              .iter()
              .enumerate()
              .map(|(i, (icon, title, description))| {
                let delay_class = format!("fade-in-up-delay-{}", i + 1);
                view! {
                  <div class=format!("problem-card {}", delay_class)>
                    <div class="problem-icon">{*icon}</div>
                    <h3 class="problem-title">{*title}</h3>
                    <p class="problem-description">{*description}</p>
                  </div>
                }
              })
              .collect_view()}
          </div>
        </div>
      </section>
    }
}

#[component]
fn SolutionSection() -> impl IntoView {
    view! {
      <section class="solution-section">
        <div class="container">
          <h2 class="section-title fade-in-up">"Welcome to the Future of Payments."</h2>
          <div class="solution-grid">
            {SOLUTIONS
              .iter()
              .enumerate()
              .map(|(i, (icon, title, description))| {
                let delay_class = format!("fade-in-up-delay-{}", i + 1);
                view! {
                  <div class=format!("solution-card {}", delay_class)>
                    <div class="solution-icon">{*icon}</div>
                    <h3 class="solution-title">{*title}</h3>
                    <p class="solution-description">{*description}</p>
                  </div>
                }
              })
              .collect_view()}
          </div>
        </div>
      </section>
    }
}

#[component]
fn HowItWorksSection() -> impl IntoView {
    view! {
      <section id="how-it-works" class="how-it-works-section">
        <div class="container">
          <h2 class="section-title fade-in-up">"Simple & Secure in 3 Steps"</h2>
          <div class="steps-container">
            {HOW_IT_WORKS
              .iter()
              .enumerate()
              .map(|(i, (step, title, description))| {
                let delay_class = format!("fade-in-up-delay-{}", i + 1);
                view! {
                  <div class=format!("step-card {}", delay_class)>
                    <div class="step-number">{*step}</div>
                    <h3 class="step-title">{*title}</h3>
                    <p class="step-description">{*description}</p>
                  </div>
                }
              })
              .collect_view()}
          </div>
        </div>
      </section>
    }
}

#[component]
fn FeaturesSection() -> impl IntoView {
    view! {
      <section id="features" class="features-section">
        <div class="container">
          <h2 class="section-title fade-in-up">"Built for the Web3 Economy"</h2>
          <div class="features-grid">
            {FEATURES
              .iter()
              .enumerate()
              .map(|(i, (icon, title, description))| {
                let delay_class = format!("fade-in-up-delay-{}", i + 1);
                view! {
                  <div class=format!("feature-card {}", delay_class)>
                    <div class="feature-icon">{*icon}</div>
                    <h3 class="feature-title">{*title}</h3>
                    <p class="feature-description">{*description}</p>
                  </div>
                }
              })
              .collect_view()}
          </div>
        </div>
      </section>
    }
}

#[component]
fn FinalCTASection() -> impl IntoView {
    view! {
      <section class="final-cta-section">
        <div class="container">
          <h2 class="cta-title fade-in-up">"Ready to Join the Open Financial Network?"</h2>
          <p class="cta-subtitle fade-in-up-delay-1">
            "Ditch the old system. Embrace a more open, transparent, and efficient way to get paid."
          </p>
          <div class="cta-buttons fade-in-up-delay-2">
            <a href=CONFIG.github_url target="_blank" class="btn btn-secondary btn-large">
              "View on GitHub"
            </a>
            <a href=CONFIG.telegram_bot_url target="_blank" class="btn btn-primary btn-large">
              "Launch the Telegram Bot"
            </a>
          </div>
          <p class="cta-sub-link fade-in-up-delay-3">
            <a href=CONFIG.testnet_contract_url target="_blank" class="testnet-link">
              "View our Testnet Contract on Uniscan"
            </a>
          </p>
        </div>
      </section>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
      <footer class="footer">
        <div class="container">
          <p>"© 2025 " {CONFIG.brand_name} ". Built with Leptos & Rust for the Web3 Future."</p>
        </div>
      </footer>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
