//! phoscat - Minimal pipe-only log colorizer
//!
//! Usage: `command | phoscat <program>`
//!        `PHOS_PROGRAM=docker command | phoscat`

use anyhow::{Context, Result};
use phos::{programs, Colorizer, Theme};
use std::env;

fn main() -> Result<()> {
    // Get program name from arg or env
    let program_name = env::args()
        .nth(1)
        .or_else(|| env::var("PHOS_PROGRAM").ok())
        .unwrap_or_else(|| {
            eprintln!("Usage: command | phoscat <program>");
            eprintln!("   or: PHOS_PROGRAM=<program> command | phoscat");
            eprintln!("\nRun 'phos list' to see available programs.");
            std::process::exit(1);
        });

    // Get theme from env
    let theme_name = env::var("PHOS_THEME").unwrap_or_else(|_| "default-dark".into());

    // Load rules
    let registry = programs::default_registry();
    let rules = registry
        .get(&program_name)
        .map(|p| p.rules())
        .unwrap_or_else(|| {
            eprintln!("Unknown program: {program_name}");
            eprintln!("Run 'phos list' to see available programs.");
            std::process::exit(1);
        });

    // Get theme
    let theme = Theme::builtin(&theme_name).unwrap_or_else(Theme::default_dark);

    // Colorize stdin to stdout
    let color_enabled = atty::is(atty::Stream::Stdout);
    let mut colorizer = Colorizer::new(rules)
        .with_theme(theme)
        .with_color_enabled(color_enabled);

    colorizer.process_stdio().context("Failed to process stdin")?;
    Ok(())
}
