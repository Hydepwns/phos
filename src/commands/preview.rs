//! Theme preview command.

use anyhow::Result;
use phos::{Colorizer, ProgramRegistry, Theme};
use std::sync::Arc;

/// Sample log lines for preview.
struct PreviewSample {
    program_id: &'static str,
    label: &'static str,
    lines: &'static [&'static str],
}

const PREVIEW_SAMPLES: &[PreviewSample] = &[
    PreviewSample {
        program_id: "ethereum.lighthouse",
        label: "Ethereum (Lighthouse)",
        lines: &[
            "Dec 05 00:12:36.557 INFO Synced slot: 12345, epoch: 385, peers: 47",
            "Dec 05 00:12:37.123 WARN Attestation delayed validator_index: 42",
            "Dec 05 00:12:38.456 ERRO Failed to connect to peer: 0x4f6a8b2c1d3e5f7a",
        ],
    },
    PreviewSample {
        program_id: "ethereum.geth",
        label: "Ethereum (Geth)",
        lines: &[
            "INFO [12-05|10:30:45.123] Imported new chain segment number=19630289 hash=0x4f6a8b2c",
            "WARN [12-05|10:30:46.456] Peer connection dropped id=abc123 err=\"timeout\"",
            "ERROR[12-05|10:30:47.789] Transaction pool full txs=5000 limit=5000",
        ],
    },
    PreviewSample {
        program_id: "devops.docker",
        label: "Docker",
        lines: &[
            "2024-01-15T10:30:45.123Z INFO Container started id=abc123def456",
            "2024-01-15T10:30:46.456Z WARN Memory usage high: 85%",
            "2024-01-15T10:30:47.789Z ERROR Container exited code=1",
        ],
    },
    PreviewSample {
        program_id: "dev.cargo",
        label: "Cargo",
        lines: &[
            "   Compiling phos v0.1.0 (/home/user/phos)",
            "warning: unused variable: `foo`",
            "error[E0382]: borrow of moved value: `x`",
        ],
    },
    PreviewSample {
        program_id: "system.systemd",
        label: "systemd",
        lines: &[
            "Dec 15 10:30:45 hostname systemd[1]: Started My Service.",
            "Dec 15 10:30:46 hostname myapp[1234]: INFO Connected to database",
            "Dec 15 10:30:47 hostname myapp[1234]: ERROR Failed to bind port 8080",
        ],
    },
    PreviewSample {
        program_id: "network.ping",
        label: "ping",
        lines: &[
            "PING 8.8.8.8 (8.8.8.8): 56 data bytes",
            "64 bytes from 8.8.8.8: icmp_seq=1 ttl=117 time=14.2 ms",
            "64 bytes from 8.8.8.8: icmp_seq=2 ttl=117 time=13.8 ms",
        ],
    },
    PreviewSample {
        program_id: "dev.git",
        label: "git",
        lines: &[
            "On branch main",
            "Changes not staged for commit:",
            "  modified:   src/main.rs",
        ],
    },
    PreviewSample {
        program_id: "dev.npm",
        label: "npm",
        lines: &[
            "npm WARN deprecated package@1.0.0: This package is deprecated",
            "npm ERR! 404 Not Found - GET https://registry.npmjs.org/nonexistent",
            "added 150 packages in 5.2s",
        ],
    },
];

const QUICK_SAMPLE: &str = "INFO slot=12345 Synced | WARN timeout | ERROR 0x4f6a8b2c1d";

/// Preview themes with colorized sample output.
pub fn preview_themes(
    registry: &ProgramRegistry,
    theme_filter: Option<&str>,
    quick: bool,
    categories: bool,
) -> Result<()> {
    let themes: Vec<String> = if let Some(name) = theme_filter {
        if Theme::builtin(name).is_none() {
            anyhow::bail!("Unknown theme: {name}. Run 'phos themes' to see available themes.");
        }
        vec![name.to_string()]
    } else {
        Theme::list_builtin()
            .iter()
            .map(ToString::to_string)
            .collect()
    };

    if categories {
        // Category-centric view: for each category, show samples across themes
        for sample in PREVIEW_SAMPLES {
            let program = registry.get(sample.program_id);
            let rules = program.map_or_else(|| Arc::from([]), |p| p.rules());

            println!("\n{}", sample.label);
            println!("{}", "-".repeat(sample.label.len()));

            for theme_name in &themes {
                let theme = Theme::builtin(theme_name).unwrap_or_else(Theme::default_dark);
                let mut colorizer = Colorizer::new(rules.clone())
                    .with_theme(theme)
                    .with_color_enabled(true);

                print!("  {theme_name:12} ");
                if quick {
                    println!("{}", colorizer.colorize(QUICK_SAMPLE));
                } else {
                    // Show first line inline, rest indented
                    for (i, line) in sample.lines.iter().enumerate() {
                        if i == 0 {
                            println!("{}", colorizer.colorize(line));
                        } else {
                            println!("  {:12} {}", "", colorizer.colorize(line));
                        }
                    }
                }
            }
        }
    } else {
        // Theme-centric view (default): for each theme, show samples from categories
        for theme_name in &themes {
            let theme = Theme::builtin(theme_name).unwrap_or_else(Theme::default_dark);

            println!("\n{theme_name}");
            println!("{}", "-".repeat(theme_name.len()));

            for sample in PREVIEW_SAMPLES {
                let program = registry.get(sample.program_id);
                let rules = program.map_or_else(|| Arc::from([]), |p| p.rules());
                let mut colorizer = Colorizer::new(rules.clone())
                    .with_theme(theme.clone())
                    .with_color_enabled(true);

                print!("  {:12} ", sample.label);
                if quick {
                    println!("{}", colorizer.colorize(QUICK_SAMPLE));
                } else {
                    // Show first line only in quick mode
                    println!("{}", colorizer.colorize(sample.lines[0]));
                }
            }
        }
    }

    println!();
    Ok(())
}
