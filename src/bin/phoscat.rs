//! phoscat - Minimal pipe-only log colorizer
//!
//! Usage: `command | phoscat <program>`
//!        `command | phoscat`              (auto-detect program)
//!        `PHOS_PROGRAM=docker command | phoscat`

use anyhow::{Context, Result};
use is_terminal::IsTerminal;
use phos::{programs, Colorizer, Theme};
use std::env;
use std::io::{self, BufRead, Write};

/// Number of lines to buffer for auto-detection
const AUTO_DETECT_LINES: usize = 10;

/// Process lines through a colorizer, writing to the given output.
fn colorize_lines<'a, I, W>(colorizer: &mut Colorizer, lines: I, out: &mut W) -> Result<()>
where
    I: Iterator<Item = &'a str>,
    W: Write,
{
    for line in lines {
        writeln!(out, "{}", colorizer.colorize(line))?;
    }
    Ok(())
}

fn main() -> Result<()> {
    // Get program name from arg or env (optional now)
    let program_name = env::args().nth(1).or_else(|| env::var("PHOS_PROGRAM").ok());

    // Get theme from env
    let theme_name = env::var("PHOS_THEME").unwrap_or_else(|_| "default-dark".into());
    let theme = Theme::builtin(&theme_name).unwrap_or_else(Theme::default_dark);

    // Load registry
    let registry = programs::default_registry();

    // Determine rules - explicit program or auto-detect
    let rules = if let Some(name) = program_name {
        registry.get(&name).map_or_else(
            || {
                eprintln!("Unknown program: {name}");
                eprintln!("Run 'phos list' to see available programs.");
                std::process::exit(1);
            },
            |p| p.rules(),
        )
    } else {
        // Auto-detect: buffer initial lines
        let stdin = io::stdin();
        let mut buffer: Vec<String> = Vec::with_capacity(AUTO_DETECT_LINES);

        for line in stdin.lock().lines().take(AUTO_DETECT_LINES) {
            buffer.push(line.context("Failed to read stdin")?);
        }

        // Try to detect program from buffered content
        let line_refs: Vec<&str> = buffer.iter().map(String::as_str).collect();
        let detected = registry.detect_from_lines(&line_refs);

        let rules = detected.map_or_else(
            || {
                eprintln!("phoscat: no program detected, using generic coloring");
                registry.get("cargo").map(|p| p.rules()).unwrap_or_default()
            },
            |program| {
                eprintln!("phoscat: auto-detected program: {}", program.info().id);
                program.rules()
            },
        );

        // Create colorizer and process buffered + remaining lines
        let color_enabled = io::stdout().is_terminal();
        let mut colorizer = Colorizer::new(rules)
            .with_theme(theme)
            .with_color_enabled(color_enabled);

        let stdout = io::stdout();
        let mut out = stdout.lock();

        // Process buffered lines
        colorize_lines(&mut colorizer, buffer.iter().map(String::as_str), &mut out)?;

        // Process remaining stdin
        for line in stdin.lock().lines() {
            let line = line.context("Failed to read stdin")?;
            writeln!(out, "{}", colorizer.colorize(&line))?;
        }

        return Ok(());
    };

    // Explicit program: colorize stdin to stdout
    let color_enabled = io::stdout().is_terminal();
    let mut colorizer = Colorizer::new(rules)
        .with_theme(theme)
        .with_color_enabled(color_enabled);

    colorizer.process_stdio().context("Failed to process stdin")
}
