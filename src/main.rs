//! phos - Universal log colorizer
//!
//! Usage:
//!   phos -p docker -- docker logs mycontainer
//!   phos -c lodestar -- docker logs -f lodestar
//!   phos -p cargo -- cargo test
//!   echo "error at slot 12345" | phos -c lodestar

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use phos::programs;
use phos::{Client, Colorizer, Config, StatsCollector, Theme};
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
    about = "Universal log colorizer with built-in support for 59 programs",
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

#[derive(Subcommand)]
enum Commands {
    /// List available programs
    #[command(name = "list")]
    List {
        /// Filter by category (ethereum, devops, system, dev, network, data, monitoring, messaging, ci)
        #[arg(long)]
        category: Option<String>,
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Create program registry with built-ins and user programs
    let mut registry = programs::default_registry();

    // Load user programs from config directory
    let load_errors = phos::program::loader::load_user_programs(&mut registry);
    for err in load_errors {
        eprintln!("Warning: Failed to load user program: {err}");
    }

    // Handle subcommands
    if let Some(cmd) = cli.command {
        return match cmd {
            Commands::List { category } => list_programs(&registry, category.as_deref()),
            Commands::ListClients => list_programs(&registry, Some("ethereum")),
            Commands::Themes | Commands::ListThemes => list_themes(),
            Commands::Info { name } => show_info(&registry, &name),
            Commands::Colors => show_colors(),
            Commands::Completions { shell } => generate_completions(shell),
        };
    }

    // Determine if we're reading from stdin or running a command
    let is_pipe = !atty::is(atty::Stream::Stdin);

    // Get the theme
    let theme = Theme::builtin(&cli.theme).unwrap_or_else(Theme::default_dark);

    // Get rules - check program first, then client (for backward compat), then config
    let rules = if let Some(ref program_name) = cli.program.as_ref().or(cli.client.as_ref()) {
        // Try program registry first
        if let Some(program) = registry.get(program_name) {
            program.rules()
        } else if let Some(client) = Client::parse(program_name) {
            // Fall back to legacy client parsing
            client.rules()
        } else {
            anyhow::bail!("Unknown program: {program_name}. Run 'phos list' to see available programs.");
        }
    } else if let Some(ref config_path) = cli.config {
        // Load from config file
        Config::load(config_path)?.to_rules()?
    } else if !cli.args.is_empty() {
        // Try to auto-detect from command using new registry
        let cmd_str = cli.args.join(" ");
        if let Some(program) = registry.detect(&cmd_str) {
            program.rules()
        } else if let Some(client) = phos::colorizer::detect_client(&cli.args[0], &cli.args[1..]) {
            // Fall back to legacy client detection
            client.rules()
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

fn list_programs(registry: &phos::ProgramRegistry, category: Option<&str>) -> Result<()> {
    let categories = category
        .map(|c| vec![c.to_string()])
        .unwrap_or_else(|| registry.categories());

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

fn show_info(registry: &phos::ProgramRegistry, name: &str) -> Result<()> {
    // Try program registry first
    if let Some(program) = registry.get(name) {
        let info = program.info();
        println!("{}", info.name);
        println!("  ID:          {}", info.id);
        println!("  Description: {}", info.description);
        println!("  Category:    {}", info.category);
        println!("  Rules:       {}", program.rules().len());

        // Show brand color for Ethereum clients
        if info.category == "ethereum" {
            let brand = phos::colors::brands::color(&info.name.to_lowercase()).unwrap_or("#888888");
            println!("  Brand color: {brand}");

            // Also show layer info if it's an Ethereum client
            if let Some(client) = Client::parse(&info.name) {
                let client_info = client.info();
                println!("  Layer:       {:?}", client_info.layer);
                println!("  Language:    {}", client_info.language);
                println!("  Website:     {}", client_info.website);
            }
        }
        return Ok(());
    }

    // Fall back to legacy client lookup
    if let Some(client) = Client::parse(name) {
        let info = client.info();
        let brand = phos::colors::brands::color(name).unwrap_or("#888888");

        println!("{}", info.name);
        println!("  Description: {}", info.description);
        println!("  Layer:       {:?}", info.layer);
        println!("  Language:    {}", info.language);
        println!("  Website:     {}", info.website);
        println!("  Brand color: {brand}");
        println!("  Rules:       {}", client.rules().len());
        return Ok(());
    }

    anyhow::bail!("Unknown program: {name}. Run 'phos list' to see available programs.");
}

fn show_colors() -> Result<()> {
    println!("Ethereum Client Brand Colors:\n");

    Client::all().iter().for_each(|client| {
        let name = format!("{client:?}").to_lowercase();
        let hex = phos::colors::brands::color(&name).unwrap_or("#888888");
        let (r, g, b) = phos::parse_hex_rgb(hex).unwrap_or((128, 128, 128));

        println!("  \x1b[38;2;{r};{g};{b}m##\x1b[0m {name:12} {hex}");
    });

    Ok(())
}

fn generate_completions(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    generate(shell, &mut cmd, name, &mut std::io::stdout());
    Ok(())
}
