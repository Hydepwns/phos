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
use phos::{Colorizer, Config, StatsCollector, Theme};
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
            } => commands::preview_themes(&registry, theme, quick, categories),
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

    // Enable colors if: --color flag set OR stdout is a TTY
    let color_enabled = cli.color || atty::is(atty::Stream::Stdout);

    let mut colorizer = Colorizer::new(rules)
        .with_theme(theme)
        .with_color_enabled(color_enabled);
    let mut stats = cli.stats.then(StatsCollector::new);

    // Set up alert manager if --alert is provided
    let program_name = cli.program.as_ref().or(cli.client.as_ref()).cloned();
    let mut alert_manager = if let Some(ref url) = cli.alert {
        let mut builder = AlertManagerBuilder::new()
            .url(url)
            .cooldown_secs(cli.alert_cooldown);

        if let Some(ref chat_id) = cli.telegram_chat_id {
            builder = builder.chat_id(chat_id);
        }

        if let Some(ref name) = program_name {
            builder = builder.program(name);
        }

        // Add conditions from CLI
        if !cli.alert_on.is_empty() {
            match builder.conditions(&cli.alert_on) {
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
        match (&mut stats, &mut alert_manager) {
            (Some(stats), Some(alerts)) => {
                colorizer.process_stdio_with_alerts(stats, alerts)?;
            }
            (Some(stats), None) => {
                colorizer.process_stdio_with_stats(stats)?;
            }
            _ => {
                colorizer.process_stdio()?;
            }
        }
    } else if !cli.args.is_empty() {
        // Run the command
        commands::run_command(&mut colorizer, &cli.args, stats.as_mut(), alert_manager.as_mut())?;
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
