//! phos - Universal log colorizer
//!
//! Usage:
//!   phos -p docker -- docker logs mycontainer
//!   phos -c lodestar -- docker logs -f lodestar
//!   phos -p cargo -- cargo test
//!   echo "error at slot 12345" | phos -c lodestar

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Shell};
use phos::programs;
use phos::programs::ethereum;
use phos::shell::{self, ShellType};
use phos::{Colorizer, Config, StatsCollector, Theme};
use serde::Serialize;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

/// Version string with git hash
fn version_string() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION"),
        " (",
        env!("PHOS_GIT_HASH"),
        " ",
        env!("PHOS_BUILD_DATE"),
        ")"
    )
}

#[derive(Parser)]
#[command(
    name = "phos",
    version = version_string(),
    about = "Universal log colorizer with built-in support for 98 programs",
    long_about = "A fast, portable log colorizer with built-in support for:\n\n\
                  Ethereum:   Lighthouse, Prysm, Teku, Nimbus, Lodestar, Grandine, Lambda,\n\
                              Geth, Nethermind, Besu, Erigon, Reth, Mana, Charon, MEV-Boost\n\n\
                  DevOps:     Docker, kubectl, Terraform, k9s, Helm, Ansible, docker-compose, AWS\n\
                  System:     systemd, syslog, fail2ban, dmesg, cron, auditd, iptables\n\
                  Dev:        git, cargo, npm, go, make, yarn, pnpm, elixir\n\
                  Network:    ping, curl, dig, nginx, caddy, apache, haproxy, traefik\n\
                  Data:       postgres, redis, mysql, mongodb, elasticsearch\n\
                  Monitoring: prometheus, grafana, datadog, signoz\n\
                  Messaging:  kafka, rabbitmq\n\
                  CI/CD:      github-actions, jenkins"
)]
struct Cli {
    /// Program to use for colorization (e.g., docker, kubectl, cargo)
    #[arg(short, long, value_name = "PROGRAM")]
    program: Option<String>,

    /// Client configuration to use (alias for --program, for Ethereum clients)
    #[arg(short, long, value_name = "CLIENT")]
    client: Option<String>,

    /// Config file path
    #[arg(long, value_name = "FILE")]
    config: Option<String>,

    /// Theme to use (run 'themes' to see all 13 options)
    #[arg(short, long, default_value = "default-dark")]
    theme: String,

    /// Force color output even when not a TTY
    #[arg(long)]
    color: bool,

    /// Show log statistics after processing
    #[arg(long)]
    stats: bool,

    /// Subcommand or command to run
    #[command(subcommand)]
    command: Option<Commands>,

    /// Command to run and colorize (after --)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

/// Output format for commands
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum OutputFormat {
    /// Human-readable table format
    #[default]
    Table,
    /// JSON format for scripting
    Json,
}

#[derive(Subcommand)]
enum Commands {
    /// List available programs
    #[command(name = "list")]
    List {
        /// Filter by category (ethereum, devops, system, dev, network, data, monitoring, messaging, ci)
        #[arg(long)]
        category: Option<String>,
        /// Output format
        #[arg(long, short = 'f', value_enum, default_value = "table")]
        format: OutputFormat,
    },

    /// List available clients (alias for 'list --category ethereum')
    #[command(name = "list-clients")]
    ListClients,

    /// List available themes
    #[command(name = "themes")]
    Themes,

    /// List available themes (deprecated alias)
    #[command(name = "list-themes", hide = true)]
    ListThemes,

    /// Show program/client info
    #[command(name = "info")]
    Info {
        /// Program or client name
        name: String,
        /// Output format
        #[arg(long, short = 'f', value_enum, default_value = "table")]
        format: OutputFormat,
    },

    /// Show brand colors for Ethereum clients
    #[command(name = "colors")]
    Colors,

    /// Generate shell completions
    #[command(name = "completions")]
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Generate shell integration script for automatic command colorization
    #[command(name = "shell-init")]
    ShellInit {
        /// Shell to generate integration for (bash, zsh, fish)
        shell: String,
    },

    /// Manage configuration
    #[command(name = "config")]
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Generate man page
    #[command(name = "man")]
    Man {
        /// Output directory (prints to stdout if not specified)
        #[arg(long, short = 'o')]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Create program registry with built-ins and user programs
    let mut registry = programs::default_registry();

    // Load user programs from config directory
    let load_errors = phos::program::loader::load_user_programs(&mut registry);
    for err in &load_errors {
        eprintln!("Warning: {}", err.format());
    }

    // Handle subcommands
    if let Some(cmd) = cli.command {
        return match cmd {
            Commands::List { category, format } => list_programs(&registry, category.as_deref(), format),
            Commands::ListClients => list_programs(&registry, Some("ethereum"), OutputFormat::Table),
            Commands::Themes | Commands::ListThemes => list_themes(),
            Commands::Info { name, format } => show_info(&registry, &name, format),
            Commands::Colors => show_colors(),
            Commands::Completions { shell } => generate_completions(shell),
            Commands::ShellInit { shell } => generate_shell_init(&registry, &shell),
            Commands::Config { action } => handle_config_action(action),
            Commands::Man { output } => generate_man_page(output),
        };
    }

    // Determine if we're reading from stdin or running a command
    let is_pipe = !atty::is(atty::Stream::Stdin);

    // Get the theme
    let theme = Theme::builtin(&cli.theme).unwrap_or_else(Theme::default_dark);

    // Get rules - check program first, then config, then auto-detect
    let rules = if let Some(ref program_name) = cli.program.as_ref().or(cli.client.as_ref()) {
        // Look up program in registry
        if let Some(program) = registry.get(program_name) {
            program.rules()
        } else {
            anyhow::bail!("Unknown program: {program_name}. Run 'phos list' to see available programs.");
        }
    } else if let Some(ref config_path) = cli.config {
        // Load from config file
        Config::load(config_path)?.to_rules()?
    } else if !cli.args.is_empty() {
        // Try to auto-detect from command
        let cmd_str = cli.args.join(" ");
        if let Some(program) = registry.detect(&cmd_str) {
            program.rules()
        } else {
            // No program detected, use empty rules
            Vec::new()
        }
    } else {
        Vec::new()
    };

    // Enable colors if: --color flag set OR stdout is a TTY
    let color_enabled = cli.color || atty::is(atty::Stream::Stdout);

    let mut colorizer = Colorizer::new(rules)
        .with_theme(theme)
        .with_color_enabled(color_enabled);
    let mut stats = cli.stats.then(StatsCollector::new);

    if is_pipe {
        // Read from stdin
        if let Some(ref mut stats) = stats {
            colorizer.process_stdio_with_stats(stats)?;
        } else {
            colorizer.process_stdio()?;
        }
    } else if !cli.args.is_empty() {
        // Run the command
        run_command(&mut colorizer, &cli.args, stats.as_mut())?;
    } else {
        // No input - show help
        eprintln!("Usage: phos -p <program> -- <command>");
        eprintln!("       phos -c <client> -- <command>  (for Ethereum clients)");
        eprintln!("       echo 'log line' | phos -p <program>");
        eprintln!();
        eprintln!("Run 'phos list' to see available programs.");
        std::process::exit(1);
    }

    // Print stats if enabled
    if let Some(stats) = stats {
        stats.print_summary();
    }

    Ok(())
}

fn run_command(
    colorizer: &mut Colorizer,
    args: &[String],
    stats: Option<&mut StatsCollector>,
) -> Result<()> {
    let (cmd, cmd_args) = args.split_first().context("No command specified")?;

    let mut child = Command::new(cmd)
        .args(cmd_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context(format!("Failed to run: {cmd}"))?;

    let stdout = child.stdout.take().expect("stdout");
    let stderr = child.stderr.take().expect("stderr");

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    // Clone colorizer for threads
    let colorizer_clone = colorizer.clone();

    // Wrap stats in Arc<Mutex> for thread safety if enabled
    let stats_arc = stats.is_some().then(|| Arc::new(Mutex::new(StatsCollector::new())));

    // Process stdout and stderr in separate threads
    let stdout_handle = std::thread::spawn({
        let mut colorizer = colorizer_clone.clone();
        let stats_arc = stats_arc.clone();
        move || {
            let out = std::io::stdout();
            let mut out = out.lock();
            stdout_reader.lines().map_while(Result::ok).for_each(|line| {
                let (colored, had_match) = colorizer.colorize_with_match_info(&line);
                if let Some(ref stats) = stats_arc {
                    if let Ok(mut s) = stats.lock() {
                        s.process_line(&line, had_match);
                    }
                }
                let _ = writeln!(out, "{colored}");
            });
        }
    });

    let stderr_handle = std::thread::spawn({
        let mut colorizer = colorizer_clone;
        let stats_arc = stats_arc.clone();
        move || {
            let err = std::io::stderr();
            let mut err = err.lock();
            stderr_reader.lines().map_while(Result::ok).for_each(|line| {
                let (colored, had_match) = colorizer.colorize_with_match_info(&line);
                if let Some(ref stats) = stats_arc {
                    if let Ok(mut s) = stats.lock() {
                        s.process_line(&line, had_match);
                    }
                }
                let _ = writeln!(err, "{colored}");
            });
        }
    });

    stdout_handle.join().ok();
    stderr_handle.join().ok();

    // Merge thread-collected stats back into the caller's collector
    if let (Some(stats), Some(stats_arc)) = (stats, stats_arc) {
        if let Ok(thread_stats) = Arc::try_unwrap(stats_arc) {
            if let Ok(thread_stats) = thread_stats.into_inner() {
                stats.stats_mut().merge(thread_stats.stats());
            }
        }
    }

    child.wait()?;
    Ok(())
}

/// Serializable program info for JSON output
#[derive(Serialize)]
struct ProgramJson {
    id: String,
    name: String,
    description: String,
    category: String,
    rules: usize,
}

#[derive(Serialize)]
struct ProgramListJson {
    total: usize,
    programs: Vec<ProgramJson>,
}

fn list_programs(registry: &phos::ProgramRegistry, category: Option<&str>, format: OutputFormat) -> Result<()> {
    let categories = category
        .map(|c| vec![c.to_string()])
        .unwrap_or_else(|| registry.categories());

    match format {
        OutputFormat::Json => {
            let programs: Vec<ProgramJson> = categories
                .iter()
                .flat_map(|cat| registry.list_by_category(cat))
                .map(|info| {
                    let program = registry.get(&info.id);
                    ProgramJson {
                        id: info.id.clone(),
                        name: info.name.clone(),
                        description: info.description.clone(),
                        category: info.category.clone(),
                        rules: program.map(|p| p.rules().len()).unwrap_or(0),
                    }
                })
                .collect();

            let output = ProgramListJson {
                total: programs.len(),
                programs,
            };
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        OutputFormat::Table => {
            println!("Available programs ({} total):\n", registry.len());

            categories
                .iter()
                .filter_map(|cat| {
                    let programs = registry.list_by_category(cat);
                    (!programs.is_empty()).then_some((cat, programs))
                })
                .for_each(|(cat, programs)| {
                    let cat_display = match cat.as_str() {
                        "ethereum" => "Ethereum",
                        "devops" => "DevOps",
                        "system" => "System",
                        "dev" => "Development",
                        "network" => "Network",
                        "ci" => "CI/CD",
                        "data" => "Data",
                        "messaging" => "Messaging",
                        "monitoring" => "Monitoring",
                        _ => cat,
                    };

                    println!("{cat_display}:");
                    programs.iter().for_each(|info| {
                        let name = info.id.split('.').next_back().unwrap_or(&info.id);
                        println!("  {:12} - {}", name, info.description);
                    });
                    println!();
                });

            // Also show Ethereum layer info if showing ethereum category
            if category == Some("ethereum") {
                println!("Ethereum clients by layer:");
                println!("  Consensus:  Lighthouse, Prysm, Teku, Nimbus, Lodestar, Grandine, Lambda");
                println!("  Execution:  Geth, Nethermind, Besu, Erigon, Reth");
                println!("  Full Node:  Mana");
                println!("  Middleware: Charon, MEV-Boost");
            }
        }
    }

    Ok(())
}

fn list_themes() -> Result<()> {
    println!("Available themes:\n");
    Theme::list_builtin()
        .iter()
        .filter_map(|name| Theme::builtin(name).map(|t| (name, t)))
        .for_each(|(name, theme)| {
            println!("  {:12} - {}", name, theme.description);
        });
    Ok(())
}

/// Extended program info for JSON output (includes Ethereum metadata when applicable)
#[derive(Serialize)]
struct ProgramInfoJson {
    id: String,
    name: String,
    description: String,
    category: String,
    rules: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    layer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brand_color: Option<String>,
}

fn show_info(registry: &phos::ProgramRegistry, name: &str, format: OutputFormat) -> Result<()> {
    let program = registry.get(name)
        .ok_or_else(|| anyhow::anyhow!("Unknown program: {name}. Run 'phos list' to see available programs."))?;

    let info = program.info();

    match format {
        OutputFormat::Json => {
            let meta = if info.category == "ethereum" {
                ethereum::client_meta(&info.name)
            } else {
                None
            };

            let output = ProgramInfoJson {
                id: info.id.clone(),
                name: info.name.clone(),
                description: info.description.clone(),
                category: info.category.clone(),
                rules: program.rules().len(),
                layer: meta.map(|m| format!("{:?}", m.layer)),
                language: meta.map(|m| m.language.to_string()),
                website: meta.map(|m| m.website.to_string()),
                brand_color: meta.map(|m| m.brand_color.to_string()),
            };
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        OutputFormat::Table => {
            println!("{}", info.name);
            println!("  ID:          {}", info.id);
            println!("  Description: {}", info.description);
            println!("  Category:    {}", info.category);
            println!("  Rules:       {}", program.rules().len());

            // Show extra info for Ethereum clients
            if info.category == "ethereum" {
                if let Some(meta) = ethereum::client_meta(&info.name) {
                    println!("  Layer:       {:?}", meta.layer);
                    println!("  Language:    {}", meta.language);
                    println!("  Website:     {}", meta.website);
                    println!("  Brand color: {}", meta.brand_color);
                }
            }
        }
    }

    Ok(())
}

fn show_colors() -> Result<()> {
    println!("Ethereum Client Brand Colors:\n");

    ethereum::all_client_names().iter().for_each(|name| {
        let name_lower = name.to_lowercase();
        let hex = ethereum::brand_color(&name_lower).unwrap_or("#888888");
        let (r, g, b) = phos::parse_hex_rgb(hex).unwrap_or((128, 128, 128));

        println!("  \x1b[38;2;{r};{g};{b}m##\x1b[0m {name_lower:12} {hex}");
    });

    Ok(())
}

fn generate_completions(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    generate(shell, &mut cmd, name, &mut std::io::stdout());
    Ok(())
}

fn generate_shell_init(registry: &phos::ProgramRegistry, shell_name: &str) -> Result<()> {
    let shell_type = ShellType::parse(shell_name).ok_or_else(|| {
        anyhow::anyhow!(
            "Unknown shell: {shell_name}. Supported shells: {}",
            ShellType::supported().join(", ")
        )
    })?;

    let script = shell::generate_script(shell_type, registry);
    print!("{script}");
    Ok(())
}

fn handle_config_action(action: ConfigAction) -> Result<()> {
    use phos::program::loader;

    match action {
        ConfigAction::Path => {
            println!("Configuration paths:\n");
            println!("  Config directory:   {}",
                loader::config_dir().map(|p| p.display().to_string()).unwrap_or_else(|| "(unavailable)".to_string()));
            println!("  Programs directory: {}",
                loader::programs_dir().map(|p| p.display().to_string()).unwrap_or_else(|| "(unavailable)".to_string()));
            println!("  Themes directory:   {}",
                loader::themes_dir().map(|p| p.display().to_string()).unwrap_or_else(|| "(unavailable)".to_string()));

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
                    let filename = path.file_name()
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

fn generate_man_page(output: Option<String>) -> Result<()> {
    use clap_mangen::Man;
    use std::fs;
    use std::path::PathBuf;

    let cmd = Cli::command();
    let man = Man::new(cmd.clone());

    match output {
        Some(dir) => {
            let out_dir = PathBuf::from(&dir);
            fs::create_dir_all(&out_dir)?;

            // Generate main man page
            let main_path = out_dir.join("phos.1");
            let mut file = fs::File::create(&main_path)?;
            man.render(&mut file)?;
            println!("Generated: {}", main_path.display());

            // Generate man pages for subcommands
            for subcommand in cmd.get_subcommands() {
                let name = subcommand.get_name();
                if name == "help" {
                    continue;
                }
                let sub_path = out_dir.join(format!("phos-{name}.1"));
                let mut file = fs::File::create(&sub_path)?;
                let sub_man = Man::new(subcommand.clone());
                sub_man.render(&mut file)?;
                println!("Generated: {}", sub_path.display());
            }

            println!("\nInstall with:");
            println!("  sudo cp {dir}/*.1 /usr/local/share/man/man1/");
            println!("  sudo mandb  # Linux only");
        }
        None => {
            // Print to stdout
            man.render(&mut std::io::stdout())?;
        }
    }

    Ok(())
}
