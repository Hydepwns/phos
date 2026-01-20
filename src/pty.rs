//! PTY (pseudo-terminal) support for interactive command execution.
//!
//! This module provides PTY-based command execution that allows interactive
//! programs (vim, less, etc.) to work correctly while phos colorizes their output.

use nix::libc;
use nix::poll::PollFlags;
use nix::pty::{openpty, OpenptyResult};
use nix::sys::termios::{self, SetArg, Termios};
use std::io::{self, Read, Write};
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd, RawFd};

// ============================================================================
// Error Handling Helpers
// ============================================================================

/// Extension trait for converting nix::Error to io::Error.
trait NixResultExt<T> {
    fn to_io(self) -> io::Result<T>;
}

impl<T> NixResultExt<T> for Result<T, nix::Error> {
    fn to_io(self) -> io::Result<T> {
        self.map_err(|e| io::Error::from_raw_os_error(e as i32))
    }
}

/// Convert ioctl return value to Result.
fn ioctl_result(ret: libc::c_int) -> io::Result<()> {
    if ret == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

// ============================================================================
// Terminal Size
// ============================================================================

/// Terminal size (rows x columns).
#[derive(Debug, Clone, Copy)]
pub struct TermSize {
    pub rows: u16,
    pub cols: u16,
}

impl TermSize {
    /// Get current terminal size from stdout.
    pub fn from_env() -> io::Result<Self> {
        let mut ws = libc::winsize::from(Self { rows: 0, cols: 0 });
        ioctl_result(unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) })?;
        Ok(Self {
            rows: ws.ws_row,
            cols: ws.ws_col,
        })
    }
}

impl From<TermSize> for libc::winsize {
    fn from(size: TermSize) -> Self {
        Self {
            ws_row: size.rows,
            ws_col: size.cols,
            ws_xpixel: 0,
            ws_ypixel: 0,
        }
    }
}

// ============================================================================
// PTY Master
// ============================================================================

/// PTY master handle for reading/writing to the pseudo-terminal.
pub struct PtyMaster {
    fd: OwnedFd,
}

impl PtyMaster {
    /// Set the terminal size of the PTY.
    pub fn set_size(&self, size: TermSize) -> io::Result<()> {
        let ws: libc::winsize = size.into();
        ioctl_result(unsafe { libc::ioctl(self.fd.as_raw_fd(), libc::TIOCSWINSZ, &ws) })
    }
}

impl AsRawFd for PtyMaster {
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}

impl AsFd for PtyMaster {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}

impl Read for PtyMaster {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        nix::unistd::read(self.fd.as_raw_fd(), buf).to_io()
    }
}

impl Write for PtyMaster {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        nix::unistd::write(&self.fd, buf).to_io()
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// ============================================================================
// Raw Mode Guard
// ============================================================================

/// RAII guard to restore terminal to cooked mode on drop.
pub struct RawModeGuard {
    original: Termios,
    fd: RawFd,
}

impl RawModeGuard {
    /// Enter raw mode on stdin.
    pub fn new() -> io::Result<Self> {
        let fd = libc::STDIN_FILENO;
        let borrowed_fd = unsafe { BorrowedFd::borrow_raw(fd) };
        let original = termios::tcgetattr(borrowed_fd).to_io()?;

        let mut raw = original.clone();
        termios::cfmakeraw(&mut raw);
        termios::tcsetattr(borrowed_fd, SetArg::TCSANOW, &raw).to_io()?;

        Ok(Self { original, fd })
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let borrowed_fd = unsafe { BorrowedFd::borrow_raw(self.fd) };
        let _ = termios::tcsetattr(borrowed_fd, SetArg::TCSANOW, &self.original);
    }
}

// ============================================================================
// PTY Creation
// ============================================================================

/// PTY pair containing master and slave file descriptors.
pub struct PtyPair {
    pub master: PtyMaster,
    pub slave: OwnedFd,
}

/// Create a PTY pair.
pub fn create_pty() -> io::Result<PtyPair> {
    let OpenptyResult { master, slave } = openpty(None, None).to_io()?;
    Ok(PtyPair {
        master: PtyMaster { fd: master },
        slave,
    })
}

// ============================================================================
// Signal Handling
// ============================================================================

/// Set up SIGWINCH handler to forward terminal resize to PTY.
/// Returns a thread handle that monitors for resize signals.
pub fn setup_sigwinch_handler(pty_fd: RawFd) -> io::Result<std::thread::JoinHandle<()>> {
    use signal_hook::consts::SIGWINCH;
    use signal_hook::iterator::Signals;

    let mut signals = Signals::new([SIGWINCH])?;

    let handle = std::thread::spawn(move || {
        for _ in signals.forever() {
            if let Ok(size) = TermSize::from_env() {
                let ws: libc::winsize = size.into();
                unsafe {
                    libc::ioctl(pty_fd, libc::TIOCSWINSZ, &ws);
                }
            }
        }
    });

    Ok(handle)
}

/// Signal forwarding for job control signals.
///
/// Forwards these signals to the child process group:
/// - SIGINT (Ctrl+C) - interrupt
/// - SIGTERM - terminate
/// - SIGTSTP (Ctrl+Z) - suspend
/// - SIGCONT - resume after suspend
/// - SIGQUIT (Ctrl+\) - quit with core dump
pub struct SignalForwarder {
    _handle: std::thread::JoinHandle<()>,
}

impl SignalForwarder {
    /// Set up signal forwarding to a child process.
    ///
    /// The forwarder runs in a background thread and forwards job control
    /// signals to the child process group (negative pid), ensuring the
    /// entire process tree receives the signal.
    pub fn new(child_pid: i32) -> io::Result<Self> {
        use signal_hook::consts::{SIGCONT, SIGINT, SIGQUIT, SIGTERM, SIGTSTP};
        use signal_hook::iterator::Signals;

        let mut signals = Signals::new([SIGINT, SIGTERM, SIGTSTP, SIGCONT, SIGQUIT])?;

        let handle = std::thread::spawn(move || {
            for sig in signals.forever() {
                // Forward signal to child process group (negative pid)
                // This ensures the entire process group receives the signal
                unsafe {
                    libc::kill(-child_pid, sig);
                }
            }
        });

        Ok(Self { _handle: handle })
    }
}

// ============================================================================
// Polling Helpers
// ============================================================================

/// Poll a file descriptor for specific events (retries on EINTR).
fn poll_for(fd: RawFd, flags: PollFlags, timeout_ms: i32) -> io::Result<bool> {
    use nix::poll::{poll, PollFd, PollTimeout};

    let mut poll_fds = [PollFd::new(unsafe { BorrowedFd::borrow_raw(fd) }, flags)];

    let timeout = match timeout_ms {
        t if t < 0 => PollTimeout::NONE,
        t => PollTimeout::from(t.min(i32::from(u16::MAX)) as u16),
    };

    loop {
        match poll(&mut poll_fds, timeout) {
            Ok(ready) => {
                return Ok(ready > 0 && poll_fds[0].revents().is_some_and(|r| r.intersects(flags)))
            }
            Err(nix::Error::EINTR) => continue,
            Err(e) => return Err(io::Error::from_raw_os_error(e as i32)),
        }
    }
}

/// Check if a file descriptor has data ready to read.
pub fn poll_read(fd: RawFd, timeout_ms: i32) -> io::Result<bool> {
    poll_for(fd, PollFlags::POLLIN, timeout_ms)
}

/// Check if a file descriptor has hung up (EOF/closed).
pub fn poll_hup(fd: RawFd, timeout_ms: i32) -> io::Result<bool> {
    poll_for(fd, PollFlags::POLLHUP, timeout_ms)
}

/// Poll result with detailed event information.
#[derive(Debug, Clone, Copy, Default)]
pub struct PollResult {
    pub readable: bool,
    pub hangup: bool,
    pub error: bool,
}

impl PollResult {
    /// Returns true if data is available to read.
    #[inline]
    pub const fn is_readable(&self) -> bool {
        self.readable
    }

    /// Returns true if the other end has closed (hangup) with no pending data.
    #[inline]
    pub const fn is_eof(&self) -> bool {
        self.hangup && !self.readable
    }

    /// Returns true if we should terminate the I/O loop.
    #[inline]
    pub const fn should_terminate(&self) -> bool {
        self.error || self.is_eof()
    }

    /// Returns true if no events occurred (timeout).
    #[inline]
    pub const fn is_timeout(&self) -> bool {
        !self.readable && !self.hangup && !self.error
    }
}

/// Poll a file descriptor and return detailed event information.
///
/// This is more useful than `poll_read`/`poll_hup` when you need to distinguish
/// between "data ready" and "hangup only" states, which is critical for proper
/// PTY drain after child exit.
pub fn poll_events(fd: RawFd, timeout_ms: i32) -> io::Result<PollResult> {
    use nix::poll::{poll, PollFd, PollTimeout};

    let flags = PollFlags::POLLIN | PollFlags::POLLHUP | PollFlags::POLLERR;
    let mut poll_fds = [PollFd::new(unsafe { BorrowedFd::borrow_raw(fd) }, flags)];

    let timeout = match timeout_ms {
        t if t < 0 => PollTimeout::NONE,
        t => PollTimeout::from(t.min(i32::from(u16::MAX)) as u16),
    };

    loop {
        match poll(&mut poll_fds, timeout) {
            Ok(0) => return Ok(PollResult::default()),
            Ok(_) => {
                let revents = poll_fds[0].revents().unwrap_or(PollFlags::empty());
                return Ok(PollResult {
                    readable: revents.contains(PollFlags::POLLIN),
                    hangup: revents.contains(PollFlags::POLLHUP),
                    error: revents.contains(PollFlags::POLLERR),
                });
            }
            Err(nix::Error::EINTR) => continue,
            Err(e) => return Err(io::Error::from_raw_os_error(e as i32)),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // PollResult Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_poll_result_default() {
        let result = PollResult::default();
        assert!(!result.readable);
        assert!(!result.hangup);
        assert!(!result.error);
    }

    #[test]
    fn test_poll_result_is_readable() {
        let result = PollResult {
            readable: true,
            hangup: false,
            error: false,
        };
        assert!(result.is_readable());
    }

    #[test]
    fn test_poll_result_is_eof_requires_hangup_without_readable() {
        // EOF is hangup without pending data
        let eof = PollResult {
            readable: false,
            hangup: true,
            error: false,
        };
        assert!(eof.is_eof());

        // Hangup with readable data is not EOF yet
        let not_eof = PollResult {
            readable: true,
            hangup: true,
            error: false,
        };
        assert!(!not_eof.is_eof());
    }

    #[test]
    fn test_poll_result_should_terminate_on_error() {
        let result = PollResult {
            readable: false,
            hangup: false,
            error: true,
        };
        assert!(result.should_terminate());
    }

    #[test]
    fn test_poll_result_should_terminate_on_eof() {
        let result = PollResult {
            readable: false,
            hangup: true,
            error: false,
        };
        assert!(result.should_terminate());
    }

    #[test]
    fn test_poll_result_should_not_terminate_when_readable() {
        let result = PollResult {
            readable: true,
            hangup: true,
            error: false,
        };
        assert!(!result.should_terminate());
    }

    #[test]
    fn test_poll_result_is_timeout() {
        let timeout = PollResult::default();
        assert!(timeout.is_timeout());

        let not_timeout = PollResult {
            readable: true,
            hangup: false,
            error: false,
        };
        assert!(!not_timeout.is_timeout());
    }

    // -------------------------------------------------------------------------
    // TermSize Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_term_size_to_winsize() {
        let size = TermSize { rows: 24, cols: 80 };
        let ws: libc::winsize = size.into();
        assert_eq!(ws.ws_row, 24);
        assert_eq!(ws.ws_col, 80);
        assert_eq!(ws.ws_xpixel, 0);
        assert_eq!(ws.ws_ypixel, 0);
    }

    // -------------------------------------------------------------------------
    // PTY Creation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_create_pty_succeeds() {
        let result = create_pty();
        assert!(result.is_ok());
    }

    #[test]
    fn test_pty_master_set_size() {
        let pty = create_pty().expect("Failed to create PTY");
        let size = TermSize {
            rows: 40,
            cols: 120,
        };
        let result = pty.master.set_size(size);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pty_master_as_raw_fd() {
        let pty = create_pty().expect("Failed to create PTY");
        let fd = pty.master.as_raw_fd();
        assert!(fd >= 0);
    }

    // -------------------------------------------------------------------------
    // Poll Helpers Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_poll_read_returns_on_timeout() {
        // Create a PTY and poll with short timeout - should return false (no data)
        let pty = create_pty().expect("Failed to create PTY");
        let result = poll_read(pty.master.as_raw_fd(), 1);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // No data available
    }

    #[test]
    fn test_poll_events_returns_timeout_result() {
        let pty = create_pty().expect("Failed to create PTY");
        let result = poll_events(pty.master.as_raw_fd(), 1);
        assert!(result.is_ok());
        let events = result.unwrap();
        assert!(events.is_timeout());
    }
}
