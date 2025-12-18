//! Command execution with colorization.

use anyhow::{Context, Result};
use phos::{AlertManager, Colorizer, StatsCollector};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

/// Run a command and colorize its output.
pub fn run_command(
    colorizer: &mut Colorizer,
    args: &[String],
    stats: Option<&mut StatsCollector>,
    mut alert_manager: Option<&mut AlertManager>,
) -> Result<()> {
    let (cmd, cmd_args) = args.split_first().context("No command specified")?;

    let mut child = Command::new(cmd)
        .args(cmd_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context(format!("Failed to run: {cmd}"))?;

    let stdout = child
        .stdout
        .take()
        .context("Failed to capture stdout pipe")?;
    let stderr = child
        .stderr
        .take()
        .context("Failed to capture stderr pipe")?;

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    // Clone colorizer for threads
    let colorizer_clone = colorizer.clone();

    // Wrap stats in Arc<Mutex> for thread safety if enabled
    let stats_arc = stats
        .is_some()
        .then(|| Arc::new(Mutex::new(StatsCollector::new())));

    // For alerting: collect lines that need alert checking in a channel
    let (alert_tx, alert_rx) = if alert_manager.is_some() {
        let (tx, rx) = std::sync::mpsc::channel::<String>();
        (Some(tx), Some(rx))
    } else {
        (None, None)
    };

    // Process stdout and stderr in separate threads
    let stdout_handle = std::thread::spawn({
        let mut colorizer = colorizer_clone.clone();
        let stats_arc = stats_arc.clone();
        let alert_tx = alert_tx.clone();
        move || {
            let out = std::io::stdout();
            let mut out = out.lock();
            stdout_reader
                .lines()
                .map_while(Result::ok)
                .for_each(|line| {
                    let (colored, had_match) = colorizer.colorize_with_match_info(&line);
                    if let Some(ref stats) = stats_arc {
                        if let Ok(mut s) = stats.lock() {
                            s.process_line(&line, had_match);
                        }
                    }
                    // Send line for alert processing
                    if let Some(ref tx) = alert_tx {
                        let _ = tx.send(line);
                    }
                    let _ = writeln!(out, "{colored}");
                });
        }
    });

    let stderr_handle = std::thread::spawn({
        let mut colorizer = colorizer_clone;
        let stats_arc = stats_arc.clone();
        let alert_tx = alert_tx;
        move || {
            let err = std::io::stderr();
            let mut err = err.lock();
            stderr_reader
                .lines()
                .map_while(Result::ok)
                .for_each(|line| {
                    let (colored, had_match) = colorizer.colorize_with_match_info(&line);
                    if let Some(ref stats) = stats_arc {
                        if let Ok(mut s) = stats.lock() {
                            s.process_line(&line, had_match);
                        }
                    }
                    // Send line for alert processing
                    if let Some(ref tx) = alert_tx {
                        let _ = tx.send(line);
                    }
                    let _ = writeln!(err, "{colored}");
                });
        }
    });

    // Process alerts in main thread while other threads handle colorization
    if let (Some(alerts), Some(rx)) = (&mut alert_manager, alert_rx) {
        if let Some(ref stats_arc) = stats_arc {
            // Process lines for alerting
            for line in rx {
                // Get current stats for alert evaluation
                let (error_count, peer_count, slot) = if let Ok(s) = stats_arc.lock() {
                    (s.error_count(), s.peer_count(), s.slot())
                } else {
                    (0, None, None)
                };

                alerts.check_line(&line, error_count, peer_count, slot);
            }
        }
    }

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
