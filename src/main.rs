// Pedantic lints allowed for consistency with lib.rs
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::too_many_lines)]

//! phos - Universal log colorizer
//!
//! Usage:
//!   phos -p docker -- docker logs mycontainer
//!   phos -c lodestar -- docker logs -f lodestar
//!   phos -p cargo -- cargo test
//!   echo "error at slot 12345" | phos -c lodestar

mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;
use phos::alert::AlertManagerBuilder;
use phos::programs;
use phos::{Colorizer, Config, GlobalConfig, StatsCollector, StatsExportFormat, Theme};
use std::sync::Arc;

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
pub struct Cli {
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

    /// Export format for statistics (implies --stats)
    #[arg(long, value_enum, value_name = "FORMAT")]
    stats_export: Option<StatsExportFormat>,

    /// Write statistics to file instead of stderr
    #[arg(long, value_name = "PATH")]
    stats_output: Option<String>,

    /// Print statistics every N seconds during processing (0 = end only)
    #[arg(long, value_name = "SECONDS", default_value = "0")]
    stats_interval: u64,

    /// Webhook URL for alerts (auto-detects Discord/Telegram)
    #[arg(long, value_name = "URL")]
    alert: Option<String>,

    /// Alert conditions: error, error-threshold:N, peer-drop:N, sync-stall
    #[arg(long = "alert-on", value_name = "CONDITION")]
    alert_on: Vec<String>,

    /// Telegram chat ID (required for Telegram webhooks)
    #[arg(long, value_name = "CHAT_ID")]
    telegram_chat_id: Option<String>,

    /// Alert cooldown in seconds (default: 60)
    #[arg(long, default_value = "60")]
    alert_cooldown: u64,

    /// Subcommand or command to run
    #[command(subcommand)]
    command: Option<Commands>,

    /// Command to run and colorize (after --)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

/// Output format for commands
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
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
        action: commands::ConfigAction,
    },

    /// Generate man page
    #[command(name = "man")]
    Man {
        /// Output directory (prints to stdout if not specified)
        #[arg(long, short = 'o')]
        output: Option<String>,
    },

    /// Preview themes with colorized sample output
    #[command(name = "preview")]
    Preview {
        /// Show only this theme (shows all if not specified)
        #[arg(short, long)]
        theme: Option<String>,
        /// Compact output with fewer samples
        #[arg(short, long)]
        quick: bool,
        /// Category-centric view instead of theme-centric
        #[arg(short, long)]
        categories: bool,
    },
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

    // Load global configuration (~/.config/phos/config.yaml)
    let global_config = match GlobalConfig::load() {
        Ok(config) => config.unwrap_or_default(),
        Err(e) => {
            eprintln!("Warning: failed to load global config: {e}");
            GlobalConfig::default()
        }
    };

    // Handle subcommands
    if let Some(cmd) = cli.command {
        return match cmd {
            Commands::List { category, format } => {
                commands::list_programs(&registry, category.as_deref(), format)
            }
            Commands::ListClients => {
                commands::list_programs(&registry, Some("ethereum"), OutputFormat::Table)
            }
            Commands::Themes | Commands::ListThemes => commands::list_themes(),
            Commands::Info { name, format } => commands::show_info(&registry, &name, format),
            Commands::Colors => commands::show_colors(),
            Commands::Completions { shell } => commands::generate_completions(shell),
            Commands::ShellInit { shell } => commands::generate_shell_init(&registry, &shell),
            Commands::Config { action } => commands::handle_config_action(action),
            Commands::Man { output } => commands::generate_man_page(output),
            Commands::Preview {
                theme,
                quick,
                categories,
            } => commands::preview_themes(&registry, theme.as_deref(), quick, categories),
        };
    }

    // Determine if we're reading from stdin or running a command
    let is_pipe = !atty::is(atty::Stream::Stdin);

    // Get the theme: CLI > global config > default
    let theme_name = (cli.theme != "default-dark")
        .then_some(&cli.theme)
        .or(global_config.theme.as_ref())
        .unwrap_or(&cli.theme);
    let theme = Theme::get(theme_name).unwrap_or_else(Theme::default_dark);

    // Get rules - check program first, then config, then auto-detect
    let rules = if let Some(program_name) = cli.program.as_ref().or(cli.client.as_ref()) {
        // Look up program in registry
        if let Some(program) = registry.get(program_name) {
            program.rules()
        } else {
            anyhow::bail!(
                "Unknown program: {program_name}. Run 'phos list' to see available programs."
            );
        }
    } else if let Some(ref config_path) = cli.config {
        // Load from config file
        Arc::from(Config::load(config_path)?.to_rules()?)
    } else if !cli.args.is_empty() {
        // Try to auto-detect from command
        let cmd_str = cli.args.join(" ");
        if let Some(program) = registry.detect(&cmd_str) {
            program.rules()
        } else {
            // No program detected, use empty rules
            Arc::from([])
        }
    } else {
        Arc::from([])
    };

    // Enable colors if: --color flag set OR global config color OR stdout is a TTY
    let color_enabled = cli.color || global_config.color || atty::is(atty::Stream::Stdout);

    let mut colorizer = Colorizer::new(rules)
        .with_theme(theme)
        .with_color_enabled(color_enabled);

    // Merge stats settings: CLI > global config > default
    // --stats-export and --stats-interval > 0 imply --stats
    let stats_interval = if cli.stats_interval > 0 {
        cli.stats_interval
    } else {
        global_config.stats_interval
    };
    let stats_enabled = cli.stats
        || global_config.stats
        || cli.stats_export.is_some()
        || global_config.stats_export.is_some()
        || stats_interval > 0;
    let mut stats = stats_enabled.then(StatsCollector::new);

    // Set up alert manager if --alert is provided (CLI or global config)
    let program_name = cli.program.as_ref().or(cli.client.as_ref()).cloned();
    let alert_url = cli.alert.as_ref().or(global_config.alerts.url.as_ref());
    let mut alert_manager = if let Some(url) = alert_url {
        let cooldown = match cli.alert_cooldown {
            60 => global_config.alerts.cooldown,
            custom => custom,
        };
        let mut builder = AlertManagerBuilder::new().url(url).cooldown_secs(cooldown);

        // Chat ID: CLI > global config
        let chat_id = cli
            .telegram_chat_id
            .as_ref()
            .or(global_config.alerts.telegram_chat_id.as_ref());
        if let Some(chat_id) = chat_id {
            builder = builder.chat_id(chat_id);
        }

        if let Some(ref name) = program_name {
            builder = builder.program(name);
        }

        // Add conditions from CLI first, then global config
        let conditions = if cli.alert_on.is_empty() {
            &global_config.alerts.conditions
        } else {
            &cli.alert_on
        };
        if !conditions.is_empty() {
            match builder.conditions(conditions) {
                Ok(b) => builder = b,
                Err(e) => {
                    eprintln!("phos: invalid alert condition: {e}");
                    std::process::exit(1);
                }
            }
        }

        builder.build()
    } else {
        None
    };

    // If alert is enabled, we need stats for tracking
    if alert_manager.is_some() && stats.is_none() {
        stats = Some(StatsCollector::new());
    }

    if is_pipe {
        // Read from stdin
        match (&mut stats, &mut alert_manager, stats_interval > 0) {
            (Some(stats), Some(alerts), true) => {
                colorizer.process_stdio_with_alerts_interval(stats, alerts, stats_interval)?;
            }
            (Some(stats), Some(alerts), false) => {
                colorizer.process_stdio_with_alerts(stats, alerts)?;
            }
            (Some(stats), None, true) => {
                colorizer.process_stdio_with_stats_interval(stats, stats_interval)?;
            }
            (Some(stats), None, false) => {
                colorizer.process_stdio_with_stats(stats)?;
            }
            _ => {
                colorizer.process_stdio()?;
            }
        }
    } else if !cli.args.is_empty() {
        // Run the command
        commands::run_command(
            &mut colorizer,
            &cli.args,
            stats.as_mut(),
            alert_manager.as_mut(),
        )?;
    } else {
        // No input - show help
        eprintln!("Usage: phos -p <program> -- <command>");
        eprintln!("       phos -c <client> -- <command>  (for Ethereum clients)");
        eprintln!("       echo 'log line' | phos -p <program>");
        eprintln!();
        eprintln!("Run 'phos list' to see available programs.");
        std::process::exit(1);
    }

    // Output stats if enabled
    if let Some(ref stats) = stats {
        // Merge stats export format: CLI > global config > default
        let format = cli.stats_export.unwrap_or_else(|| {
            global_config
                .stats_export
                .as_deref()
                .and_then(|s| match s {
                    "json" => Some(StatsExportFormat::Json),
                    "prometheus" => Some(StatsExportFormat::Prometheus),
                    "human" => Some(StatsExportFormat::Human),
                    _ => None,
                })
                .unwrap_or(StatsExportFormat::Human)
        });
        let program = program_name.as_deref();

        if let Some(ref path) = cli.stats_output {
            // Write to file
            let mut file = std::fs::File::create(path)?;
            stats.write_export(&mut file, format, program)?;
        } else {
            // Write to stderr
            let mut stderr = std::io::stderr().lock();
            stats.write_export(&mut stderr, format, program)?;
        }
    }

    Ok(())
}
