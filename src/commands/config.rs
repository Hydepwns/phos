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
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "(unavailable)".to_string())
            );
            println!(
                "  Programs directory: {}",
                loader::programs_dir()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "(unavailable)".to_string())
            );
            println!(
                "  Themes directory:   {}",
                loader::themes_dir()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "(unavailable)".to_string())
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

            let programs_dir = loader::programs_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;

            println!("Created configuration directories.");
            println!();
            println!("Programs directory: {}", programs_dir.display());
            println!();

            // Create example program config
            let example_path = programs_dir.join("example.yaml");
            if !example_path.exists() {
                let example_content = r#"# Example phos program configuration
# Rename this file and customize for your application

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
  # Log levels
  - regex: '\[ERROR\]'
    colors: [error]
    bold: true

  - regex: '\[WARN\]'
    colors: [warn]
    bold: true

  - regex: '\[INFO\]'
    colors: [info]

  - regex: '\[DEBUG\]'
    colors: [debug]

  # Custom patterns using domain colors
  - regex: 'request_id=([a-f0-9-]+)'
    colors: [request_id]

  - regex: 'user_id=(\d+)'
    colors: [user_id]

  # Timestamps
  - regex: '\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}'
    colors: [timestamp]

  # Numbers
  - regex: '\b\d+(\.\d+)?\b'
    colors: [number]
"#;
                std::fs::write(&example_path, example_content)?;
                println!("Created example config: {}", example_path.display());
                println!();
                println!("Edit this file and rename it to match your application.");
                println!("Run 'phos config validate' to check your configuration.");
            } else {
                println!("Example config already exists: {}", example_path.display());
            }

            Ok(())
        }
    }
}
