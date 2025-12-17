// Pedantic lints we've chosen to allow:
// - missing_panics_doc: Many functions use .expect() for internal invariants
// - missing_errors_doc: Error types are self-documenting
// - too_many_lines: Some complex functions benefit from being cohesive
// - cast_precision_loss/cast_sign_loss/cast_possible_wrap: Safe in context
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]

//! # phos - High-performance universal log colorizer
//!
//! A fast, portable log colorizer with support for 99+ programs across 9 categories.
//!
//! ## Features
//!
//! - **99 built-in programs**: Ethereum clients, DevOps tools, dev tools, databases, etc.
//! - **13 themes**: Dracula, Nord, Catppuccin, Gruvbox, Tokyo Night, and more
//! - **Semantic colors**: Rules use abstract colors (Error, Warn, Info) that themes resolve
//! - **Regex-based**: Powerful pattern matching with capture groups and replacements
//! - **Zero-copy output**: Efficient ANSI escape code generation
//! - **Extensible**: Define custom programs via YAML/JSON configuration
//!
//! ## Quick Start
//!
//! ```rust
//! use phos::{Colorizer, programs};
//!
//! // Get a program from the built-in registry
//! let registry = programs::default_registry();
//! let program = registry.get("lodestar").unwrap();
//!
//! // Create a colorizer with the program's rules
//! let mut colorizer = Colorizer::new(program.rules());
//! let colored = colorizer.colorize("INFO: Synced slot 12345");
//! println!("{}", colored);
//! ```
//!
//! ## Custom Rules
//!
//! ```rust
//! use phos::{Colorizer, Rule, SemanticColor, Theme};
//!
//! // Define custom rules
//! let rules = vec![
//!     Rule::new(r"\bERROR\b").unwrap()
//!         .semantic(SemanticColor::Error)
//!         .bold()
//!         .build(),
//!     Rule::new(r"\bWARN\b").unwrap()
//!         .semantic(SemanticColor::Warn)
//!         .build(),
//!     Rule::new(r"\d{4}-\d{2}-\d{2}").unwrap()
//!         .semantic(SemanticColor::Timestamp)
//!         .build(),
//! ];
//!
//! // Use with a theme
//! let mut colorizer = Colorizer::new(rules)
//!     .with_theme(Theme::dracula());
//! ```
//!
//! ## Program Categories
//!
//! | Category   | Count | Examples                                    |
//! |------------|-------|---------------------------------------------|
//! | Ethereum   | 15    | Lighthouse, Geth, Prysm, Lodestar           |
//! | System     | 26    | systemd, syslog, dmesg, ps, top             |
//! | Network    | 21    | nginx, curl, ping, tcpdump                  |
//! | Dev        | 15    | cargo, git, npm, go, make                   |
//! | DevOps     | 8     | docker, kubectl, terraform, ansible         |
//! | Data       | 5     | postgres, redis, mysql, mongodb             |
//! | Monitoring | 4     | prometheus, grafana, datadog                |
//! | Messaging  | 2     | kafka, rabbitmq                             |
//! | CI/CD      | 2     | github-actions, jenkins                     |
//!
//! ## Themes
//!
//! Built-in themes: `default-dark`, `dracula`, `nord`, `catppuccin`, `gruvbox`,
//! `monokai`, `solarized`, `synthwave84`, `tokyo-night`, `horizon`, `matrix`,
//! `phosphor`, `high-contrast`
//!
//! ```rust
//! use phos::Theme;
//!
//! let theme = Theme::builtin("dracula").unwrap();
//! // Or use convenience methods:
//! let theme = Theme::nord();
//! ```

pub mod aggregator;
pub mod alert;
pub mod category;
pub mod colorizer;
pub mod colors;
pub mod config;
pub mod program;
pub mod programs;
pub mod rule;
pub mod shell;
pub mod stats;
pub mod theme;

pub use alert::{AlertCondition, AlertManager, AlertManagerBuilder, AlertSeverity};
pub use category::{Category, ParseCategoryError};
pub use colorizer::Colorizer;
pub use colors::{parse_hex_rgb, Color, ColorSpec, SemanticColor};
pub use config::{AlertsConfig, Config, GlobalConfig, RuleConfig};
pub use program::{Program, ProgramInfo, ProgramRegistry};
pub use rule::Rule;
pub use stats::{Stats, StatsCollector, StatsExportFormat, StatsJson};
pub use theme::{Theme, ThemeConfig, ThemeLoadError};

/// Prelude for convenient imports.
pub mod prelude {
    pub use crate::{
        Category, Color, ColorSpec, Colorizer, Config, Program, ProgramInfo, ProgramRegistry,
        Rule, SemanticColor, Stats, StatsCollector, Theme,
    };
}
