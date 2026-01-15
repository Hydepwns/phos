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

    /// Check child process status, returning exit code if terminated (handles EINTR).
    pub fn check_child_status(child: Pid) -> Result<Option<i32>> {
        loop {
            match waitpid(child, Some(WaitPidFlag::WNOHANG)) {
                Ok(WaitStatus::StillAlive) => return Ok(None),
                Ok(other) => return Ok(wait_status_to_exit_code(other)),
                Err(nix::Error::EINTR) => continue,
                Err(e) => return Err(e.into()),
            }
        }
    }
}

/// Run a command with PTY support for interactive programs.
///
/// This function creates a pseudo-terminal so that child processes see a TTY,
/// allowing interactive programs like vim, less, and editors to work correctly.
///
/// If `raw` is true, output is passed through without colorization (for vim, etc.).
#[cfg(unix)]
pub fn run_command_pty(
    colorizer: &mut Colorizer,
    args: &[String],
    mut stats: Option<&mut StatsCollector>,
    mut alert_manager: Option<&mut AlertManager>,
    raw: bool,
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
                raw,
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
    raw: bool,
) -> Result<i32> {
    use pty_helpers::{check_child_status, wait_status_to_exit_code};

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
                raw,
            )?;
            return Ok(exit_code);
        }

        // Forward stdin to PTY (retry on EINTR)
        if poll_read(stdin_fd, 0)? {
            let n = loop {
                match std::io::stdin().read(&mut read_buf) {
                    Ok(n) => break n,
                    Err(e) if e.raw_os_error() == Some(libc::EINTR) => continue,
                    Err(e) => return Err(e.into()),
                }
            };
            if n > 0 {
                pty.write_all(&read_buf[..n])?;
            }
        }

        // Read and process PTY output (retry on EINTR)
        if poll_read(pty_fd, 10)? {
            let n = match pty.read(&mut read_buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                Err(e) if e.raw_os_error() == Some(libc::EIO) => break,
                Err(e) if e.raw_os_error() == Some(libc::EINTR) => continue,
                Err(e) => return Err(e.into()),
            };

            if raw {
                // Raw mode: pass through without colorization
                stdout.write_all(&read_buf[..n])?;
                stdout.flush()?;

                // Track lines for stats/alerts (strip ANSI for accurate matching)
                for &byte in &read_buf[..n] {
                    if byte == b'\n' {
                        let stripped = phos::strip_ansi(&line_buffer);
                        if let Some(ref mut s) = stats {
                            s.process_line(&stripped, !stripped.is_empty());
                        }
                        if let Some(ref mut a) = alert_manager {
                            let (err_count, peer_count, slot) = stats
                                .as_ref()
                                .map(|s| (s.error_count(), s.peer_count(), s.slot()))
                                .unwrap_or((0, None, None));
                            a.check_line(&stripped, err_count, peer_count, slot);
                        }
                        line_buffer.clear();
                    } else if byte != b'\r' {
                        line_buffer.push(byte as char);
                    }
                }
            } else {
                // Colorize mode: strip ANSI, colorize, output
                for &byte in &read_buf[..n] {
                    match byte {
                        b'\n' => {
                            // Strip existing ANSI and colorize
                            let stripped = phos::strip_ansi(&line_buffer);
                            let colored = colorizer.colorize(&stripped);
                            writeln!(stdout, "{}", colored)?;

                            if let Some(ref mut s) = stats {
                                s.process_line(&stripped, true);
                            }
                            if let Some(ref mut a) = alert_manager {
                                let (err_count, peer_count, slot) = stats
                                    .as_ref()
                                    .map(|s| (s.error_count(), s.peer_count(), s.slot()))
                                    .unwrap_or((0, None, None));
                                a.check_line(&stripped, err_count, peer_count, slot);
                            }
                            line_buffer.clear();
                        }
                        b'\r' => {
                            // Carriage return: clear line and display colorized content
                            if !line_buffer.is_empty() {
                                let stripped = phos::strip_ansi(&line_buffer);
                                let colored = colorizer.colorize(&stripped);
                                write!(stdout, "\x1b[2K\r{}", colored)?;
                                stdout.flush()?;
                                line_buffer.clear();
                            } else {
                                write!(stdout, "\x1b[2K\r")?;
                                stdout.flush()?;
                            }
                        }
                        _ => {
                            line_buffer.push(byte as char);
                        }
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
        let stripped = phos::strip_ansi(&line_buffer);
        if raw {
            write!(stdout, "{}", line_buffer)?;
        } else {
            let colored = colorizer.colorize(&stripped);
            write!(stdout, "{}", colored)?;
        }
        stdout.flush()?;
    }

    // Wait for child to fully exit (handles EINTR)
    loop {
        match waitpid(child, None) {
            Ok(status) => return Ok(wait_status_to_exit_code(status).unwrap_or(0)),
            Err(nix::Error::EINTR) => continue,
            Err(e) => return Err(e.into()),
        }
    }
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
    raw: bool,
) -> Result<()> {
    let mut read_buf = [0u8; 4096];
    let pty_fd = pty.as_raw_fd();

    while poll_read(pty_fd, 10)? {
        let n = match pty.read(&mut read_buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => n,
        };

        if raw {
            // Raw mode: pass through
            stdout.write_all(&read_buf[..n])?;

            for &byte in &read_buf[..n] {
                if byte == b'\n' {
                    let stripped = phos::strip_ansi(line_buffer);
                    if let Some(ref mut s) = stats {
                        s.process_line(&stripped, !stripped.is_empty());
                    }
                    if let Some(ref mut a) = alert_manager {
                        let (err_count, peer_count, slot) = stats
                            .as_ref()
                            .map(|s| (s.error_count(), s.peer_count(), s.slot()))
                            .unwrap_or((0, None, None));
                        a.check_line(&stripped, err_count, peer_count, slot);
                    }
                    line_buffer.clear();
                } else if byte != b'\r' {
                    line_buffer.push(byte as char);
                }
            }
        } else {
            // Colorize mode
            for &byte in &read_buf[..n] {
                if byte == b'\n' {
                    let stripped = phos::strip_ansi(line_buffer);
                    let colored = colorizer.colorize(&stripped);
                    writeln!(stdout, "{}", colored)?;

                    if let Some(ref mut s) = stats {
                        s.process_line(&stripped, true);
                    }
                    if let Some(ref mut a) = alert_manager {
                        let (err_count, peer_count, slot) = stats
                            .as_ref()
                            .map(|s| (s.error_count(), s.peer_count(), s.slot()))
                            .unwrap_or((0, None, None));
                        a.check_line(&stripped, err_count, peer_count, slot);
                    }
                    line_buffer.clear();
                } else if byte != b'\r' {
                    line_buffer.push(byte as char);
                }
            }
        }
    }

    stdout.flush()?;

    Ok(())
}
