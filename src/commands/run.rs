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

// ============================================================================
// Helpers
// ============================================================================

/// Retry an operation while it returns EINTR.
#[cfg(unix)]
fn retry_eintr<T, F: FnMut() -> std::io::Result<T>>(mut f: F) -> std::io::Result<T> {
    loop {
        match f() {
            Err(e) if e.raw_os_error() == Some(libc::EINTR) => continue,
            result => return result,
        }
    }
}

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
            for line in stdout_reader.lines().map_while(Result::ok) {
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
            }
        }
    });

    let stderr_handle = std::thread::spawn({
        let mut colorizer = colorizer_clone;
        let stats_arc = stats_arc.clone();
        let alert_tx = alert_tx;
        move || {
            let err = std::io::stderr();
            let mut err = err.lock();
            for line in stderr_reader.lines().map_while(Result::ok) {
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
            }
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

    // Log thread panics (should not happen in normal operation)
    if let Err(e) = stdout_handle.join() {
        eprintln!("phos: warning: stdout handler panicked: {e:?}");
    }
    if let Err(e) = stderr_handle.join() {
        eprintln!("phos: warning: stderr handler panicked: {e:?}");
    }

    // Merge thread-collected stats back into the caller's collector
    if let (Some(stats), Some(stats_arc)) = (stats, stats_arc) {
        let _ = Arc::try_unwrap(stats_arc)
            .ok()
            .and_then(|mutex| mutex.into_inner().ok())
            .map(|thread_stats| stats.stats_mut().merge(thread_stats.stats()));
    }

    child.wait()?;
    Ok(())
}

// ============================================================================
// PTY-based Execution (Unix only)
// ============================================================================

/// Process stats and alerts for a completed line.
#[cfg(unix)]
#[inline]
fn process_line_stats(
    line: &str,
    stats: &mut Option<&mut StatsCollector>,
    alerts: &mut Option<&mut AlertManager>,
    had_match: bool,
) {
    if let Some(s) = stats.as_mut() {
        s.process_line(line, had_match);
    }
    if let Some(a) = alerts.as_mut() {
        let (err, peer, slot) = stats.as_ref().map_or((0, None, None), |s| {
            (s.error_count(), s.peer_count(), s.slot())
        });
        a.check_line(line, err, peer, slot);
    }
}

/// Process PTY bytes into lines, handling CR/LF and outputting colorized content.
#[cfg(unix)]
fn process_pty_bytes(
    buf: &[u8],
    line_buffer: &mut String,
    colorizer: &mut Colorizer,
    stdout: &mut std::io::Stdout,
    stats: &mut Option<&mut StatsCollector>,
    alerts: &mut Option<&mut AlertManager>,
    raw: bool,
) -> std::io::Result<()> {
    if raw {
        stdout.write_all(buf)?;
    }

    for (i, &byte) in buf.iter().enumerate() {
        match byte {
            b'\n' => {
                let stripped = phos::strip_ansi(line_buffer);
                if !raw {
                    writeln!(stdout, "{}", colorizer.colorize(&stripped))?;
                }
                process_line_stats(&stripped, stats, alerts, !stripped.is_empty());
                line_buffer.clear();
            }
            b'\r' if buf.get(i + 1) != Some(&b'\n') => line_buffer.clear(),
            b'\r' => {}
            _ => line_buffer.push(byte as char),
        }
    }
    stdout.flush()
}

/// Flush remaining line buffer content.
#[cfg(unix)]
fn flush_line_buffer(
    line_buffer: &str,
    colorizer: &mut Colorizer,
    stdout: &mut std::io::Stdout,
    raw: bool,
) -> std::io::Result<()> {
    let trimmed = line_buffer.trim();
    if !trimmed.is_empty() {
        let stripped = phos::strip_ansi(trimmed);
        if raw {
            writeln!(stdout, "{}", stripped)?;
        } else {
            writeln!(stdout, "{}", colorizer.colorize(&stripped))?;
        }
    }
    stdout.flush()
}

#[cfg(unix)]
mod pty_helpers {
    use super::*;

    /// Convert a `WaitStatus` to a process exit code.
    pub fn wait_status_to_exit_code(status: WaitStatus) -> Option<i32> {
        match status {
            WaitStatus::Exited(_, code) => Some(code),
            WaitStatus::Signaled(_, sig, _) => Some(128i32.saturating_add(sig as i32)),
            _ => None,
        }
    }

    /// Check if child has terminated (non-blocking, retries on EINTR).
    pub fn check_child_status(child: Pid) -> Result<Option<i32>> {
        loop {
            match waitpid(child, Some(WaitPidFlag::WNOHANG)) {
                Ok(WaitStatus::StillAlive) => return Ok(None),
                Ok(status) => return Ok(wait_status_to_exit_code(status)),
                Err(nix::Error::EINTR) => continue,
                Err(e) => return Err(e.into()),
            }
        }
    }

    /// Wait for child to fully exit (blocking, retries on EINTR).
    pub fn wait_child(child: Pid) -> Result<WaitStatus> {
        loop {
            match waitpid(child, None) {
                Ok(status) => return Ok(status),
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
            // Note: ioctl request type differs by platform (c_ulong on macOS, c_int on Linux)
            #[cfg(target_os = "macos")]
            unsafe {
                libc::ioctl(slave_fd, libc::TIOCSCTTY as libc::c_ulong, 0);
            }
            #[cfg(target_os = "linux")]
            unsafe {
                libc::ioctl(slave_fd, libc::TIOCSCTTY, 0);
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
    alerts: &mut Option<&mut AlertManager>,
    raw: bool,
) -> Result<i32> {
    use is_terminal::IsTerminal;
    use pty_helpers::{check_child_status, wait_child, wait_status_to_exit_code};

    let stdin_is_tty = std::io::stdin().is_terminal();
    let _raw_guard = stdin_is_tty.then(RawModeGuard::new).transpose()?;

    let stdin_fd = std::io::stdin().as_raw_fd();
    let pty_fd = pty.as_raw_fd();
    let mut stdout = std::io::stdout();
    let mut buf = [0u8; 4096];
    let mut line_buffer = String::new();

    loop {
        // Child terminated - drain remaining output and return
        if let Some(code) = check_child_status(child)? {
            drain_pty_to_stdout(
                &mut pty,
                &mut line_buffer,
                colorizer,
                &mut stdout,
                stats,
                alerts,
                raw,
            )?;
            return Ok(code);
        }

        // Forward stdin to PTY if available
        if stdin_is_tty && poll_read(stdin_fd, 0)? {
            let n = retry_eintr(|| std::io::stdin().read(&mut buf))?;
            if n > 0 {
                pty.write_all(&buf[..n])?;
            }
        }

        // Process PTY output
        if poll_read(pty_fd, 10)? {
            match pty.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => process_pty_bytes(
                    &buf[..n],
                    &mut line_buffer,
                    colorizer,
                    &mut stdout,
                    stats,
                    alerts,
                    raw,
                )?,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                Err(e) if e.raw_os_error() == Some(libc::EIO) => break,
                Err(e) if e.raw_os_error() == Some(libc::EINTR) => continue,
                Err(e) => return Err(e.into()),
            }
        }

        if poll_hup(pty_fd, 0)? {
            break;
        }
    }

    flush_line_buffer(&line_buffer, colorizer, &mut stdout, raw)?;
    wait_child(child).map(|s| wait_status_to_exit_code(s).unwrap_or(0))
}

/// Drain remaining PTY output after child exits.
#[cfg(unix)]
fn drain_pty_to_stdout(
    pty: &mut phos::pty::PtyMaster,
    line_buffer: &mut String,
    colorizer: &mut Colorizer,
    stdout: &mut std::io::Stdout,
    stats: &mut Option<&mut StatsCollector>,
    alerts: &mut Option<&mut AlertManager>,
    raw: bool,
) -> Result<()> {
    let mut buf = [0u8; 4096];
    let pty_fd = pty.as_raw_fd();

    while poll_read(pty_fd, 100)? {
        match pty.read(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => process_pty_bytes(
                &buf[..n],
                line_buffer,
                colorizer,
                stdout,
                stats,
                alerts,
                raw,
            )?,
        }
    }
    flush_line_buffer(line_buffer, colorizer, stdout, raw)?;
    Ok(())
}
