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
use phos::pty::{
    create_pty, poll_events, poll_read, setup_sigwinch_handler, RawModeGuard, SignalForwarder,
    TermSize,
};
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

/// I/O error classification for PTY reads.
#[cfg(unix)]
enum ReadOutcome {
    /// Successful read with data
    Data(usize),
    /// End of file (read returned 0 or EIO)
    Eof,
    /// Transient error, should retry (EINTR, EAGAIN, WouldBlock)
    Retry,
    /// Fatal error that should propagate
    Error(std::io::Error),
}

#[cfg(unix)]
impl ReadOutcome {
    /// Classify a read result into an outcome.
    fn from_read_result(result: std::io::Result<usize>) -> Self {
        match result {
            Ok(0) => Self::Eof,
            Ok(n) => Self::Data(n),
            Err(e) if e.raw_os_error() == Some(libc::EIO) => Self::Eof,
            Err(e) if e.raw_os_error() == Some(libc::EINTR) => Self::Retry,
            Err(e) if e.raw_os_error() == Some(libc::EAGAIN) => Self::Retry,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Self::Retry,
            Err(e) => Self::Error(e),
        }
    }
}

// ============================================================================
// PTY Constants
// ============================================================================

/// Poll timeout in milliseconds during active I/O.
#[cfg(unix)]
const PTY_POLL_TIMEOUT_MS: i32 = 10;

// ============================================================================
// PTY Child Setup Helpers
// ============================================================================

/// Set up the PTY slave as the controlling terminal for the child process.
/// Logs warnings on failure but continues execution (some environments may not support these).
#[cfg(unix)]
fn setup_controlling_terminal(slave_fd: i32) {
    // Create new session - required for controlling terminal
    setsid()
        .map_err(|_| eprintln!("phos: warning: setsid failed"))
        .ok();

    // Set controlling terminal - required for interactive programs
    let tiocsctty_result = {
        #[cfg(target_os = "macos")]
        {
            unsafe { libc::ioctl(slave_fd, libc::TIOCSCTTY as libc::c_ulong, 0) }
        }
        #[cfg(target_os = "linux")]
        {
            unsafe { libc::ioctl(slave_fd, libc::TIOCSCTTY, 0) }
        }
    };

    if tiocsctty_result == -1 {
        eprintln!("phos: warning: TIOCSCTTY failed");
    }
}

/// Redirect standard file descriptors to the PTY slave.
#[cfg(unix)]
fn redirect_stdio_to_slave(slave_fd: i32) {
    [0, 1, 2].iter().for_each(|&fd| {
        let _ = dup2(slave_fd, fd);
    });

    // Close original slave fd if it's not one of the standard fds
    if slave_fd > 2 {
        let _ = close(slave_fd);
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

    // Wait for child and propagate exit code
    let status = child.wait()?;
    if let Some(code) = status.code().filter(|&code| code != 0) {
        std::process::exit(code);
    }

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

/// Write bytes directly to stdout (passthrough mode) while tracking lines for stats/alerts.
///
/// Bytes are written unchanged to preserve terminal escape sequences.
/// Line tracking is done separately for optional stats/alerting without modifying output.
#[cfg(unix)]
fn write_passthrough(
    buf: &[u8],
    line_buffer: &mut String,
    stdout: &mut std::io::Stdout,
    stats: &mut Option<&mut StatsCollector>,
    alerts: &mut Option<&mut AlertManager>,
) -> std::io::Result<()> {
    // Write bytes unchanged
    stdout.write_all(buf)?;

    // Track lines for stats/alerts if enabled (without modifying output)
    if stats.is_some() || alerts.is_some() {
        buf.iter().for_each(|&byte| match byte {
            b'\n' => {
                let stripped = phos::strip_ansi(line_buffer);
                process_line_stats(&stripped, stats, alerts, !stripped.is_empty());
                line_buffer.clear();
            }
            b'\r' => line_buffer.clear(),
            _ => line_buffer.push(byte as char),
        });
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
/// Creates a pseudo-terminal so child processes see a TTY, allowing interactive
/// programs (vim, less, editors, TUI apps) to work correctly. Output is passed
/// through unchanged (raw passthrough mode) to preserve escape sequences for
/// cursor control, screen clearing, and other terminal operations.
///
/// The `pty_config` provides configurable timeouts and other PTY settings.
#[cfg(unix)]
pub fn run_command_pty(
    args: &[String],
    mut stats: Option<&mut StatsCollector>,
    mut alert_manager: Option<&mut AlertManager>,
    pty_config: &phos::PtyConfig,
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
            setup_controlling_terminal(slave_fd);
            redirect_stdio_to_slave(slave_fd);

            // Execute the command (never returns on success)
            let _ = execvp(&c_cmd, &c_args);
            eprintln!("phos: exec failed: {}", std::io::Error::last_os_error());
            std::process::exit(127);
        }
        ForkResult::Parent { child } => {
            // Parent: close slave, run I/O loop
            drop(pty_pair.slave);

            // Set up signal handlers
            let _sigwinch_handle = setup_sigwinch_handler(master_fd);
            let _signal_forwarder = SignalForwarder::new(child.as_raw());

            // Run I/O loop with raw passthrough (no colorization)
            let exit_code = run_pty_io_loop(
                pty_pair.master,
                child,
                &mut stats,
                &mut alert_manager,
                pty_config,
            )?;

            if exit_code != 0 {
                std::process::exit(exit_code);
            }

            Ok(())
        }
    }
}

/// Main I/O loop: forward stdin to PTY, pass PTY output to stdout unchanged.
///
/// Operates in raw passthrough mode - bytes flow through without modification
/// to preserve terminal escape sequences for TUI applications.
///
/// Reads until EOF/EIO from the PTY (not just until child exits) to prevent
/// output truncation when the child produces output and exits quickly.
#[cfg(unix)]
fn run_pty_io_loop(
    mut pty: phos::pty::PtyMaster,
    child: Pid,
    stats: &mut Option<&mut StatsCollector>,
    alerts: &mut Option<&mut AlertManager>,
    pty_config: &phos::PtyConfig,
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

    // Track child exit state - we continue reading PTY even after child exits
    let mut exit_code: Option<i32> = None;

    loop {
        // Check child status (non-blocking) - capture exit code but continue reading
        let child_exited = exit_code.is_some()
            || check_child_status(child)?
                .inspect(|code| exit_code = Some(*code))
                .is_some();

        // Forward stdin to PTY only if child is still running
        if !child_exited && stdin_is_tty && poll_read(stdin_fd, 0)? {
            retry_eintr(|| std::io::stdin().read(&mut buf))
                .ok()
                .filter(|&n| n > 0)
                .map(|n| pty.write_all(&buf[..n]))
                .transpose()?;
        }

        // Poll with longer timeout after child exits to ensure drain completes
        let timeout = if child_exited {
            pty_config.drain_timeout_ms
        } else {
            PTY_POLL_TIMEOUT_MS
        };
        let events = poll_events(pty_fd, timeout)?;

        if events.is_readable() {
            match ReadOutcome::from_read_result(pty.read(&mut buf)) {
                ReadOutcome::Data(n) => {
                    write_passthrough(&buf[..n], &mut line_buffer, &mut stdout, stats, alerts)?;
                }
                ReadOutcome::Eof => break,
                ReadOutcome::Retry => continue,
                ReadOutcome::Error(e) => return Err(e.into()),
            }
        } else if events.is_eof() {
            // POLLHUP without POLLIN - do final read attempt
            if let ReadOutcome::Data(n) = ReadOutcome::from_read_result(pty.read(&mut buf)) {
                write_passthrough(&buf[..n], &mut line_buffer, &mut stdout, stats, alerts)?;
            }
            break;
        } else if events.error || (child_exited && events.is_timeout()) {
            // Error or child exited with no more events - drain and exit
            if child_exited {
                drain_pty_output(
                    &mut pty,
                    &mut line_buffer,
                    &mut stdout,
                    stats,
                    alerts,
                    pty_config,
                )?;
            }
            break;
        }
        // else: timeout with no events and child running, continue loop
    }

    // Return captured exit code or wait for child
    exit_code.map_or_else(
        || wait_child(child).map(|s| wait_status_to_exit_code(s).unwrap_or(0)),
        Ok,
    )
}

/// Drain remaining PTY output after child exits.
///
/// Uses `poll_events` for proper POLLIN/POLLHUP detection and retries on
/// transient errors. Gives up after configured max retries consecutive timeouts.
#[cfg(unix)]
fn drain_pty_output(
    pty: &mut phos::pty::PtyMaster,
    line_buffer: &mut String,
    stdout: &mut std::io::Stdout,
    stats: &mut Option<&mut StatsCollector>,
    alerts: &mut Option<&mut AlertManager>,
    pty_config: &phos::PtyConfig,
) -> Result<()> {
    let mut buf = [0u8; 4096];
    let pty_fd = pty.as_raw_fd();
    let retry_timeout = pty_config.drain_timeout_ms / 2;
    let mut consecutive_timeouts = 0u32;

    loop {
        let events = poll_events(pty_fd, retry_timeout)?;

        if events.is_readable() {
            consecutive_timeouts = 0;
            match ReadOutcome::from_read_result(pty.read(&mut buf)) {
                ReadOutcome::Data(n) => {
                    write_passthrough(&buf[..n], line_buffer, stdout, stats, alerts)?;
                }
                ReadOutcome::Eof | ReadOutcome::Error(_) => break,
                ReadOutcome::Retry => continue,
            }
        } else if events.hangup {
            // POLLHUP - do final read attempt and exit
            if let ReadOutcome::Data(n) = ReadOutcome::from_read_result(pty.read(&mut buf)) {
                write_passthrough(&buf[..n], line_buffer, stdout, stats, alerts)?;
            }
            break;
        } else {
            consecutive_timeouts += 1;
            if consecutive_timeouts >= pty_config.drain_max_retries {
                break;
            }
        }
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    #[cfg(unix)]
    use super::*;

    // ReadOutcome tests (require private access)
    #[cfg(unix)]
    #[test]
    fn test_read_outcome_data() {
        let outcome = ReadOutcome::from_read_result(Ok(100));
        assert!(matches!(outcome, ReadOutcome::Data(100)));
    }

    #[cfg(unix)]
    #[test]
    fn test_read_outcome_eof_on_zero() {
        let outcome = ReadOutcome::from_read_result(Ok(0));
        assert!(matches!(outcome, ReadOutcome::Eof));
    }

    #[cfg(unix)]
    #[test]
    fn test_read_outcome_eof_on_eio() {
        let err = std::io::Error::from_raw_os_error(libc::EIO);
        let outcome = ReadOutcome::from_read_result(Err(err));
        assert!(matches!(outcome, ReadOutcome::Eof));
    }

    #[cfg(unix)]
    #[test]
    fn test_read_outcome_retry_on_eintr() {
        let err = std::io::Error::from_raw_os_error(libc::EINTR);
        let outcome = ReadOutcome::from_read_result(Err(err));
        assert!(matches!(outcome, ReadOutcome::Retry));
    }

    #[cfg(unix)]
    #[test]
    fn test_read_outcome_retry_on_eagain() {
        let err = std::io::Error::from_raw_os_error(libc::EAGAIN);
        let outcome = ReadOutcome::from_read_result(Err(err));
        assert!(matches!(outcome, ReadOutcome::Retry));
    }

    #[cfg(unix)]
    #[test]
    fn test_read_outcome_error_on_other() {
        let err = std::io::Error::from_raw_os_error(libc::EBADF);
        let outcome = ReadOutcome::from_read_result(Err(err));
        assert!(matches!(outcome, ReadOutcome::Error(_)));
    }
}
