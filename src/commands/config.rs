//! Configuration management commands.

use anyhow::Result;
use clap::Subcommand;
use phos::program::loader;

/// Configuration subcommand actions.
#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show configuration directory paths
    #[command(name = "path")]
    Path,

    /// Validate user program configurations
    #[command(name = "validate")]
    Validate {
        /// Specific file to validate (validates all if not specified)
        file: Option<String>,
    },

    /// Initialize config directory with example files
    #[command(name = "init")]
    Init,
}

/// Handle configuration subcommands.
pub fn handle_config_action(action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Path => {
            println!("Configuration paths:\n");
            println!(
                "  Config directory:   {}",
                loader::config_dir()
                    .map_or_else(|| "(unavailable)".to_string(), |p| p.display().to_string())
            );
            println!(
                "  Programs directory: {}",
                loader::programs_dir()
                    .map_or_else(|| "(unavailable)".to_string(), |p| p.display().to_string())
            );
            println!(
                "  Themes directory:   {}",
                loader::themes_dir()
                    .map_or_else(|| "(unavailable)".to_string(), |p| p.display().to_string())
            );

            // Show if directories exist
            println!();
            if let Some(programs_dir) = loader::programs_dir() {
                if programs_dir.exists() {
                    let files = loader::list_program_files();
                    println!("  Programs found: {}", files.len());
                } else {
                    println!("  Programs directory does not exist yet.");
                    println!("  Run 'phos config init' to create it.");
                }
            }
            Ok(())
        }

        ConfigAction::Validate { file } => {
            if let Some(file_path) = file {
                // Validate specific file
                let path = std::path::PathBuf::from(&file_path);
                if !path.exists() {
                    anyhow::bail!("File not found: {file_path}");
                }
                match loader::validate_program_file(&path) {
                    Ok(info) => {
                        println!("Valid: {info}");
                        Ok(())
                    }
                    Err(result) => {
                        eprintln!("Error: {}", result.format());
                        std::process::exit(1);
                    }
                }
            } else {
                // Validate all files in programs directory
                let files = loader::list_program_files();
                if files.is_empty() {
                    println!("No program configuration files found.");
                    if let Some(dir) = loader::programs_dir() {
                        println!("Expected location: {}", dir.display());
                    }
                    return Ok(());
                }

                println!("Validating {} configuration file(s):\n", files.len());
                let mut errors = 0;
                for path in &files {
                    let filename = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    match loader::validate_program_file(path) {
                        Ok(info) => println!("  [OK] {filename}: {info}"),
                        Err(result) => {
                            eprintln!("  [ERROR] {}", result.format());
                            errors += 1;
                        }
                    }
                }
                println!();
                if errors > 0 {
                    eprintln!("{errors} error(s) found.");
                    std::process::exit(1);
                } else {
                    println!("All configurations valid.");
                }
                Ok(())
            }
        }

        ConfigAction::Init => {
            // Create config directories
            loader::ensure_config_dirs()?;

            let config_dir = loader::config_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
            let programs_dir = loader::programs_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
            let themes_dir = loader::themes_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;

            println!("Created configuration directories:\n");
            println!("  Config:   {}", config_dir.display());
            println!("  Programs: {}", programs_dir.display());
            println!("  Themes:   {}", themes_dir.display());
            println!();

            // Create global config.yaml
            let global_config_path = config_dir.join("config.yaml");
            if global_config_path.exists() {
                println!("Global config exists: {}", global_config_path.display());
            } else {
                let global_config_content = r#"# phos global configuration
# Settings here apply to all phos invocations

# Default theme (run 'phos themes' to see all 13 options)
# theme: dracula

# Force color output even when not a TTY
# color: true

# Statistics settings
# stats: false
# stats_export: human  # human, json, prometheus
# stats_interval: 0    # Print stats every N seconds (0 = end only)

# Alerting defaults
# alerts:
#   url: https://discord.com/api/webhooks/xxx/yyy
#   cooldown: 60
#   conditions:
#     - error
#     - peer-drop:10
#   # telegram_chat_id: "123456789"  # Required for Telegram webhooks
"#;
                std::fs::write(&global_config_path, global_config_content)?;
                println!("Created: {}", global_config_path.display());
            }
            println!();

            // Create example program config
            let example_path = programs_dir.join("example.yaml");
            if example_path.exists() {
                println!("Example program exists: {}", example_path.display());
            } else {
                let example_content = r#"# Example phos program configuration
# Rename this file to match your application (e.g., myapp.yaml)

name: MyApp
description: Example custom application colorization
category: custom

# Patterns for auto-detection (matched against command line)
detect:
  - myapp
  - docker.*myapp

# Custom semantic colors for your domain
semantic_colors:
  request_id: '#88AAFF'
  user_id: '#FFAA88'

# Colorization rules (applied in order)
rules:
  # Log levels (use semantic colors: error, warn, info, debug, trace)
  - regex: '\[ERROR\]'
    colors: [error]
    bold: true

  - regex: '\[WARN\]'
    colors: [warn]

  - regex: '\[INFO\]'
    colors: [info]

  - regex: '\[DEBUG\]'
    colors: [debug]

  # Custom patterns using domain colors defined above
  - regex: 'request_id=([a-f0-9-]+)'
    colors: [request_id]

  - regex: 'user_id=(\d+)'
    colors: [user_id]

  # Use semantic colors: timestamp, number, string, identifier, etc.
  - regex: '\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}'
    colors: [timestamp]

  - regex: '\b\d+(\.\d+)?\b'
    colors: [number]
"#;
                std::fs::write(&example_path, example_content)?;
                println!("Created: {}", example_path.display());
            }

            // Create example theme config
            let theme_path = themes_dir.join("example.yaml");
            if theme_path.exists() {
                println!("Example theme exists: {}", theme_path.display());
            } else {
                let theme_content = r#"# Example phos theme configuration
# Rename this file to use your theme (e.g., mytheme.yaml)
# Then use: phos -t mytheme -- your-command

name: mytheme

# Color palette (hex colors)
palette:
  # Log levels
  error: '#FF5555'
  warn: '#FFAA00'
  info: '#88FF88'
  debug: '#888888'
  trace: '#666666'

  # Data types
  number: '#BD93F9'
  string: '#F1FA8C'
  boolean: '#FF79C6'

  # Structure
  timestamp: '#8BE9FD'
  key: '#FFB86C'
  value: '#F8F8F2'

  # Status
  success: '#50FA7B'
  failure: '#FF5555'

  # Identifiers
  identifier: '#8BE9FD'
  label: '#FF79C6'
  metric: '#BD93F9'
"#;
                std::fs::write(&theme_path, theme_content)?;
                println!("Created: {}", theme_path.display());
            }

            println!();
            println!("Quick start:");
            println!("  1. Set default theme: edit config.yaml, uncomment 'theme: dracula'");
            println!("  2. Create custom program: copy example.yaml, rename to myapp.yaml");
            println!("  3. Create custom theme: copy themes/example.yaml, rename it");
            println!();
            println!("Commands:");
            println!("  phos themes           # List available themes");
            println!("  phos preview          # Preview all themes");
            println!("  phos config validate  # Validate your configurations");
            println!("  phos list             # List all programs");

            Ok(())
        }
    }
}
