//! # phos - High-performance universal log colorizer
//!
//! A fast, portable log colorizer with support for 98 programs across 9 categories.
//!
//! ## Features
//!
//! - 98 built-in program configurations (Ethereum, DevOps, Dev, Network, Data, etc.)
//! - Theme system with 13 themes and semantic colors
//! - Regex-based pattern matching
//! - Zero-copy output with ANSI escape codes
//!
//! ## Quick Start
//!
//! ```rust
//! use phos::{Colorizer, programs};
//!
//! let registry = programs::default_registry();
//! let program = registry.get("lodestar").unwrap();
//! let mut colorizer = Colorizer::new(program.rules());
//! let colored = colorizer.colorize("INFO: Synced slot 12345");
//! println!("{}", colored);
//! ```
//!
//! ## Supported Ethereum Clients (15)
//!
//! ### Consensus Layer
//! - Lighthouse, Prysm, Teku, Nimbus, Lodestar, Grandine, Lambda
//!
//! ### Execution Layer
//! - Geth, Nethermind, Besu, Erigon, Reth
//!
//! ### Full Node
//! - Mana (EL+CL in Elixir)
//!
//! ### Middleware
//! - Charon (Obol DVT), MEV-Boost

pub mod colorizer;
pub mod colors;
pub mod config;
pub mod program;
pub mod programs;
pub mod rule;
pub mod shell;
pub mod stats;
pub mod theme;

pub use colorizer::Colorizer;
pub use colors::{parse_hex_rgb, Color, ColorSpec, SemanticColor};
pub use config::Config;
pub use program::{Program, ProgramInfo, ProgramRegistry};
pub use rule::Rule;
pub use stats::{Stats, StatsCollector};
pub use theme::Theme;

/// Prelude for convenient imports.
pub mod prelude {
    pub use crate::{
        Color, ColorSpec, Colorizer, Config, Program, ProgramInfo, ProgramRegistry, Rule,
        SemanticColor, Stats, StatsCollector, Theme,
    };
}
