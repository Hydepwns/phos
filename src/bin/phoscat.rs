//! phoscat - Minimal pipe-only log colorizer
//!
//! Usage: `command | phoscat <program>`
//!        `command | phoscat`              (auto-detect program)
//!        `PHOS_PROGRAM=docker command | phoscat`

use anyhow::{Context, Result};
use phos::{programs, Colorizer, Theme};
use std::env;
use std::io::{self, BufRead, Write};

/// Number of lines to buffer for auto-detection
const AUTO_DETECT_LINES: usize = 10;

fn main() -> Result<()> {
    // Get program name from arg or env (optional now)
    let program_name = env::args()
        .nth(1)
        .or_else(|| env::var("PHOS_PROGRAM").ok());

    // Get theme from env
    let theme_name = env::var("PHOS_THEME").unwrap_or_else(|_| "default-dark".into());
    let theme = Theme::builtin(&theme_name).unwrap_or_else(Theme::default_dark);

    // Load registry
    let registry = programs::default_registry();

    // Determine rules - explicit program or auto-detect
    let rules = match program_name {
        Some(name) => registry.get(&name).map(|p| p.rules()).unwrap_or_else(|| {
            eprintln!("Unknown program: {name}");
            eprintln!("Run 'phos list' to see available programs.");
            std::process::exit(1);
        }),
        None => {
            // Auto-detect: buffer initial lines
            let stdin = io::stdin();
            let stdout = io::stdout();
            let mut stdout = stdout.lock();
            let mut buffer: Vec<String> = Vec::with_capacity(AUTO_DETECT_LINES);

            for line in stdin.lock().lines().take(AUTO_DETECT_LINES) {
                buffer.push(line.context("Failed to read stdin")?);
            }

            // Try to detect program from buffered content
            let line_refs: Vec<&str> = buffer.iter().map(|s| s.as_str()).collect();
            let detected = registry.detect_from_lines(&line_refs);

            let rules = match detected {
                Some(program) => {
                    eprintln!("phoscat: auto-detected program: {}", program.info().id);
                    program.rules()
                }
                None => {
                    // Fall back to generic log coloring
                    eprintln!("phoscat: no program detected, using generic coloring");
                    registry.get("cargo").map(|p| p.rules()).unwrap_or_default()
                }
            };

            // Create colorizer and process buffered lines
            let color_enabled = atty::is(atty::Stream::Stdout);
            let mut colorizer = Colorizer::new(rules.clone())
                .with_theme(theme.clone())
                .with_color_enabled(color_enabled);

            for line in &buffer {
                writeln!(stdout, "{}", colorizer.colorize(line))?;
            }

            // Continue processing rest of stdin
            for line in stdin.lock().lines() {
                let line = line.context("Failed to read stdin")?;
                writeln!(stdout, "{}", colorizer.colorize(&line))?;
            }

            return Ok(());
        }
    };

    // Explicit program: colorize stdin to stdout
    let color_enabled = atty::is(atty::Stream::Stdout);
    let mut colorizer = Colorizer::new(rules)
        .with_theme(theme)
        .with_color_enabled(color_enabled);

    colorizer.process_stdio().context("Failed to process stdin")
}
