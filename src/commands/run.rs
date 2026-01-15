//! Command execution with colorization.

use anyhow::{Context, Result};
use phos::{AlertManager, Colorizer, StatsCollector};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

#[cfg(unix)]
use nix::libc;
#[cfg(unix)]
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
#[cfg(unix)]
use nix::unistd::{close, dup2, execvp, fork, setsid, ForkResult, Pid};
#[cfg(unix)]
use phos::pty::{create_pty, poll_hup, poll_read, setup_sigwinch_handler, RawModeGuard, TermSize};
#[cfg(unix)]
use std::ffi::CString;
#[cfg(unix)]
use std::io::Read;
#[cfg(unix)]
use std::os::fd::AsRawFd;

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

// ============================================================================
// PTY-based Execution (Unix only)
// ============================================================================

#[cfg(unix)]
mod pty_helpers {
    use super::*;

    /// Convert WaitStatus to an exit code.
    pub fn wait_status_to_exit_code(status: WaitStatus) -> Option<i32> {
        match status {
            WaitStatus::Exited(_, code) => Some(code),
            WaitStatus::Signaled(_, sig, _) => Some(128 + sig as i32),
            _ => None,
        }
    }

    /// Check child process status, returning exit code if terminated.
    pub fn check_child_status(child: Pid) -> Result<Option<i32>> {
        waitpid(child, Some(WaitPidFlag::WNOHANG))
            .map(|status| match status {
                WaitStatus::StillAlive => None,
                other => wait_status_to_exit_code(other),
            })
            .map_err(Into::into)
    }

    /// Process a line: colorize, record stats, check alerts.
    pub fn process_line(
        line: &str,
        colorizer: &mut Colorizer,
        stdout: &mut std::io::Stdout,
        stats: &mut Option<&mut StatsCollector>,
        alert_manager: &mut Option<&mut AlertManager>,
    ) -> std::io::Result<()> {
        let colored = colorizer.colorize(line);
        writeln!(stdout, "{}", colored)?;

        if let Some(ref mut s) = stats {
            s.process_line(line, true);
        }

        if let Some(ref mut a) = alert_manager {
            let (err_count, peer_count, slot) = stats
                .as_ref()
                .map(|s| (s.error_count(), s.peer_count(), s.slot()))
                .unwrap_or((0, None, None));
            a.check_line(line, err_count, peer_count, slot);
        }

        Ok(())
    }
}

/// Run a command with PTY support for interactive programs.
///
/// This function creates a pseudo-terminal so that child processes see a TTY,
/// allowing interactive programs like vim, less, and editors to work correctly.
#[cfg(unix)]
pub fn run_command_pty(
    colorizer: &mut Colorizer,
    args: &[String],
    mut stats: Option<&mut StatsCollector>,
    mut alert_manager: Option<&mut AlertManager>,
) -> Result<()> {
    let (cmd, cmd_args) = args.split_first().context("No command specified")?;

    // Create PTY pair
    let pty_pair = create_pty().context("Failed to create PTY")?;
    let master_fd = pty_pair.master.as_raw_fd();
    let slave_fd = pty_pair.slave.as_raw_fd();

    // Set PTY size to match current terminal
    if let Ok(size) = TermSize::from_env() {
        pty_pair.master.set_size(size)?;
    }

    // Prepare args for execvp - properly propagate null byte errors
    let c_cmd = CString::new(cmd.as_str()).context("Command contains null byte")?;
    let c_args: Vec<CString> = std::iter::once(Ok(c_cmd.clone()))
        .chain(cmd_args.iter().map(|s| CString::new(s.as_str())))
        .collect::<std::result::Result<Vec<_>, _>>()
        .context("Argument contains null byte")?;

    // Fork
    match unsafe { fork() }.context("Failed to fork")? {
        ForkResult::Child => {
            // Child process: set up PTY slave as controlling terminal
            let _ = close(master_fd);
            let _ = setsid();

            // Set controlling terminal
            unsafe {
                libc::ioctl(slave_fd, libc::TIOCSCTTY as libc::c_ulong, 0);
            }

            // Redirect stdin/stdout/stderr to PTY slave
            let _ = dup2(slave_fd, 0);
            let _ = dup2(slave_fd, 1);
            let _ = dup2(slave_fd, 2);

            if slave_fd > 2 {
                let _ = close(slave_fd);
            }

            // Execute the command (never returns on success)
            let _ = execvp(&c_cmd, &c_args);
            eprintln!("phos: exec failed: {}", std::io::Error::last_os_error());
            std::process::exit(127);
        }
        ForkResult::Parent { child } => {
            // Parent: close slave, run I/O loop
            drop(pty_pair.slave);

            // Set up SIGWINCH handler for terminal resize
            let _sigwinch_handle = setup_sigwinch_handler(master_fd);

            // Run I/O loop
            let exit_code = run_pty_io_loop(
                colorizer,
                pty_pair.master,
                child,
                &mut stats,
                &mut alert_manager,
            )?;

            if exit_code != 0 {
                std::process::exit(exit_code);
            }

            Ok(())
        }
    }
}

/// Main I/O loop: forward stdin to PTY, colorize PTY output to stdout.
#[cfg(unix)]
fn run_pty_io_loop(
    colorizer: &mut Colorizer,
    mut pty: phos::pty::PtyMaster,
    child: Pid,
    stats: &mut Option<&mut StatsCollector>,
    alert_manager: &mut Option<&mut AlertManager>,
) -> Result<i32> {
    use pty_helpers::{check_child_status, process_line, wait_status_to_exit_code};

    let _raw_guard = RawModeGuard::new()?;

    let stdin_fd = std::io::stdin().as_raw_fd();
    let pty_fd = pty.as_raw_fd();
    let mut stdout = std::io::stdout();
    let mut read_buf = [0u8; 4096];
    let mut line_buffer = String::new();

    loop {
        // Check if child has terminated
        if let Some(exit_code) = check_child_status(child)? {
            drain_pty_output(
                &mut pty,
                colorizer,
                &mut stdout,
                &mut line_buffer,
                stats,
                alert_manager,
            )?;
            return Ok(exit_code);
        }

        // Forward stdin to PTY
        if poll_read(stdin_fd, 0)? {
            let n = std::io::stdin().read(&mut read_buf)?;
            if n > 0 {
                pty.write_all(&read_buf[..n])?;
            }
        }

        // Read and process PTY output
        if poll_read(pty_fd, 10)? {
            let n = match pty.read(&mut read_buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                Err(e) if e.raw_os_error() == Some(libc::EIO) => break,
                Err(e) => return Err(e.into()),
            };

            // Process bytes with inline closures
            for &byte in &read_buf[..n] {
                match byte {
                    b'\n' => {
                        process_line(&line_buffer, colorizer, &mut stdout, stats, alert_manager)?;
                        line_buffer.clear();
                    }
                    b'\r' => {
                        if !line_buffer.is_empty() {
                            let colored = colorizer.colorize(&line_buffer);
                            write!(stdout, "\r{}", colored)?;
                        } else {
                            write!(stdout, "\r")?;
                        }
                        stdout.flush()?;
                    }
                    _ => {
                        line_buffer.push(byte as char);
                    }
                }
            }
        }

        // Check for PTY hangup
        if poll_hup(pty_fd, 0)? {
            break;
        }
    }

    // Flush remaining content
    if !line_buffer.is_empty() {
        process_line(&line_buffer, colorizer, &mut stdout, stats, alert_manager)?;
    }

    // Wait for child to fully exit
    waitpid(child, None)
        .map(|status| wait_status_to_exit_code(status).unwrap_or(0))
        .map_err(Into::into)
}

/// Drain remaining output from PTY after child exits.
#[cfg(unix)]
fn drain_pty_output(
    pty: &mut phos::pty::PtyMaster,
    colorizer: &mut Colorizer,
    stdout: &mut std::io::Stdout,
    line_buffer: &mut String,
    stats: &mut Option<&mut StatsCollector>,
    alert_manager: &mut Option<&mut AlertManager>,
) -> Result<()> {
    use pty_helpers::process_line;

    let mut read_buf = [0u8; 4096];
    let pty_fd = pty.as_raw_fd();

    while poll_read(pty_fd, 10)? {
        let n = match pty.read(&mut read_buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => n,
        };

        for &byte in &read_buf[..n] {
            match byte {
                b'\n' => {
                    process_line(line_buffer, colorizer, stdout, stats, alert_manager)?;
                    line_buffer.clear();
                }
                b'\r' => {}
                _ => {
                    line_buffer.push(byte as char);
                }
            }
        }
    }

    // Flush remaining content
    if !line_buffer.is_empty() {
        process_line(line_buffer, colorizer, stdout, stats, alert_manager)?;
        line_buffer.clear();
    }

    Ok(())
}
