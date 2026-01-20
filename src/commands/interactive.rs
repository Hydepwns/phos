//! Interactive command detection for PTY execution.
//!
//! Detects commands that require pseudo-terminal (PTY) support,
//! including interactive programs, editors, REPLs, and TUI applications.

/// Commands that require PTY for proper operation (interactive/TUI programs).
const INTERACTIVE_COMMANDS: &[&str] = &[
    // Editors
    "vim",
    "vi",
    "nvim",
    "nano",
    "emacs",
    "pico",
    "ed",
    // Pagers
    "less",
    "more",
    "most",
    // Interactive tools
    "top",
    "htop",
    "btop",
    "ncdu",
    "mc",
    "tig",
    // Shells
    "bash",
    "zsh",
    "fish",
    "sh",
    // TUI apps
    "tmux",
    "screen",
    "k9s",
    "lazygit",
    "lazydocker",
    // System editors (always spawn $EDITOR)
    "visudo",
    "vipw",
    "vigr",
    "edquota",
    // Database CLIs (interactive by default)
    "psql",
    "mysql",
    "sqlite3",
    "redis-cli",
    "mongosh",
];

/// Commands that are interactive only when run without arguments (REPLs).
const REPL_COMMANDS: &[&str] = &[
    "python", "python3", "python2", "node", "nodejs", "irb", "pry",  // Ruby
    "ghci", // Haskell
    "erl", "iex", // Erlang/Elixir
    "lua", "R",
];

/// Commands that pass through to another command (check the inner command).
const PASSTHROUGH_COMMANDS: &[&str] = &[
    "sudo", "doas", "su", "env", "nice", "nohup", "time", "timeout", "strace",
];

/// Git subcommands that spawn an editor and require PTY.
const GIT_INTERACTIVE_SUBCOMMANDS: &[&str] =
    &["commit", "merge", "rebase", "tag", "notes", "filter-branch"];

/// Git subcommand flags that trigger interactive mode.
const GIT_INTERACTIVE_FLAGS: &[&str] = &["-p", "--patch", "-i", "--interactive", "-e", "--edit"];

/// Kubectl subcommands that spawn an editor.
const KUBECTL_INTERACTIVE_SUBCOMMANDS: &[&str] = &["edit", "exec"];

/// Extract the base command name from a path.
fn base_command(cmd: &str) -> &str {
    std::path::Path::new(cmd)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(cmd)
}

/// Check if a command is known to be interactive (requires PTY).
pub fn is_interactive_command(cmd: &str) -> bool {
    INTERACTIVE_COMMANDS.contains(&base_command(cmd))
}

/// Check if args contain any of the given flags.
fn has_flag(args: &[String], flags: &[&str]) -> bool {
    args.iter()
        .any(|arg| flags.iter().any(|f| arg.starts_with(f)))
}

/// Check git-specific interactive patterns.
fn is_git_interactive(args: &[String]) -> bool {
    args.iter()
        .any(|arg| GIT_INTERACTIVE_SUBCOMMANDS.contains(&arg.as_str()))
        || has_flag(args, GIT_INTERACTIVE_FLAGS)
}

/// Check kubectl-specific interactive patterns.
fn is_kubectl_interactive(args: &[String]) -> bool {
    args.iter()
        .any(|arg| KUBECTL_INTERACTIVE_SUBCOMMANDS.contains(&arg.as_str()))
        || has_flag(args, &["-it", "-ti"])
}

/// Check docker-specific interactive patterns.
fn is_docker_interactive(args: &[String]) -> bool {
    let has_run_or_exec = args.iter().any(|arg| arg == "run" || arg == "exec");
    let has_interactive = has_flag(args, &["-it", "-ti", "--interactive"]);
    has_run_or_exec && has_interactive
}

/// Check if command is a REPL run without script arguments.
fn is_repl_interactive(base: &str, args: &[String]) -> bool {
    if !REPL_COMMANDS.contains(&base) {
        return false;
    }
    // Interactive if no file arguments (heuristic: no args or only flags)
    args.iter().skip(1).all(|arg| arg.starts_with('-'))
}

/// Check crontab -e pattern.
fn is_crontab_edit(base: &str, args: &[String]) -> bool {
    base == "crontab" && has_flag(args, &["-e"])
}

/// Check ssh without remote command (interactive shell).
fn is_ssh_interactive(base: &str, args: &[String]) -> bool {
    if base != "ssh" {
        return false;
    }
    // ssh is interactive if there's no command after the host
    !args
        .iter()
        .skip(1)
        .any(|arg| arg.contains(' ') || arg.contains('/'))
}

/// Check if a command with arguments requires PTY.
///
/// Detects interactive commands including:
/// - Direct interactive commands (vim, less, htop, etc.)
/// - Commands that spawn editors (git commit, crontab -e, kubectl edit)
/// - Docker/kubectl with -it flags
/// - REPLs without script arguments (python, node, etc.)
/// - Commands wrapped with sudo/su/env
pub fn needs_pty(args: &[String]) -> bool {
    if args.is_empty() {
        return false;
    }

    let base = base_command(&args[0]);

    // Handle passthrough commands (sudo, su, env, etc.) - check inner command
    if PASSTHROUGH_COMMANDS.contains(&base) {
        for (i, arg) in args.iter().enumerate().skip(1) {
            if arg.starts_with('-') {
                continue;
            }
            let remaining: Vec<String> = args.iter().skip(i).cloned().collect();
            if needs_pty(&remaining) {
                return true;
            }
            if arg.contains('/') || is_interactive_command(arg) {
                break;
            }
        }
    }

    // Direct interactive commands
    if is_interactive_command(&args[0]) {
        return true;
    }

    // Program-specific detection
    match base {
        "git" if args.len() > 1 => is_git_interactive(&args[1..]),
        "kubectl" if args.len() > 1 => is_kubectl_interactive(&args[1..]),
        "docker" | "podman" if args.len() > 1 => is_docker_interactive(&args[1..]),
        "crontab" => is_crontab_edit(base, args),
        "ssh" => is_ssh_interactive(base, args),
        _ => is_repl_interactive(base, args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_interactive_command_editors() {
        assert!(is_interactive_command("vim"));
        assert!(is_interactive_command("vi"));
        assert!(is_interactive_command("nvim"));
        assert!(is_interactive_command("nano"));
        assert!(is_interactive_command("emacs"));
    }

    #[test]
    fn test_is_interactive_command_pagers() {
        assert!(is_interactive_command("less"));
        assert!(is_interactive_command("more"));
        assert!(is_interactive_command("most"));
    }

    #[test]
    fn test_is_interactive_command_tui_apps() {
        assert!(is_interactive_command("htop"));
        assert!(is_interactive_command("top"));
        assert!(is_interactive_command("btop"));
        assert!(is_interactive_command("k9s"));
        assert!(is_interactive_command("lazygit"));
    }

    #[test]
    fn test_is_interactive_command_shells() {
        assert!(is_interactive_command("bash"));
        assert!(is_interactive_command("zsh"));
        assert!(is_interactive_command("fish"));
        assert!(is_interactive_command("sh"));
    }

    #[test]
    fn test_is_interactive_command_with_path() {
        assert!(is_interactive_command("/usr/bin/vim"));
        assert!(is_interactive_command("/usr/local/bin/nvim"));
        assert!(is_interactive_command("./vim"));
    }

    #[test]
    fn test_is_interactive_command_non_interactive() {
        assert!(!is_interactive_command("ls"));
        assert!(!is_interactive_command("cat"));
        assert!(!is_interactive_command("grep"));
        assert!(!is_interactive_command("git"));
        assert!(!is_interactive_command("cargo"));
        assert!(!is_interactive_command("docker"));
    }

    #[test]
    fn test_is_interactive_command_empty() {
        assert!(!is_interactive_command(""));
    }

    #[test]
    fn test_needs_pty_interactive_commands() {
        assert!(needs_pty(&["vim".to_string()]));
        assert!(needs_pty(&["less".to_string(), "file.txt".to_string()]));
        assert!(needs_pty(&["/usr/bin/htop".to_string()]));
    }

    #[test]
    fn test_needs_pty_git_commit() {
        assert!(needs_pty(&["git".to_string(), "commit".to_string()]));
        assert!(needs_pty(&[
            "git".to_string(),
            "commit".to_string(),
            "-m".to_string(),
            "msg".to_string()
        ]));
        assert!(needs_pty(&[
            "git".to_string(),
            "commit".to_string(),
            "-a".to_string()
        ]));
    }

    #[test]
    fn test_needs_pty_git_rebase() {
        assert!(needs_pty(&["git".to_string(), "rebase".to_string()]));
        assert!(needs_pty(&[
            "git".to_string(),
            "rebase".to_string(),
            "-i".to_string(),
            "HEAD~3".to_string()
        ]));
    }

    #[test]
    fn test_needs_pty_git_merge() {
        assert!(needs_pty(&[
            "git".to_string(),
            "merge".to_string(),
            "feature".to_string()
        ]));
    }

    #[test]
    fn test_needs_pty_git_interactive_flags() {
        assert!(needs_pty(&[
            "git".to_string(),
            "add".to_string(),
            "-p".to_string()
        ]));
        assert!(needs_pty(&[
            "git".to_string(),
            "add".to_string(),
            "--patch".to_string()
        ]));
        assert!(needs_pty(&[
            "git".to_string(),
            "add".to_string(),
            "-i".to_string()
        ]));
        assert!(needs_pty(&[
            "git".to_string(),
            "add".to_string(),
            "--interactive".to_string()
        ]));
    }

    #[test]
    fn test_needs_pty_git_non_interactive() {
        assert!(!needs_pty(&["git".to_string(), "status".to_string()]));
        assert!(!needs_pty(&["git".to_string(), "log".to_string()]));
        assert!(!needs_pty(&["git".to_string(), "diff".to_string()]));
        assert!(!needs_pty(&["git".to_string(), "push".to_string()]));
        assert!(!needs_pty(&["git".to_string(), "pull".to_string()]));
        assert!(!needs_pty(&["git".to_string(), "fetch".to_string()]));
        assert!(!needs_pty(&[
            "git".to_string(),
            "clone".to_string(),
            "url".to_string()
        ]));
    }

    #[test]
    fn test_needs_pty_non_git_commands() {
        assert!(!needs_pty(&["ls".to_string()]));
        assert!(!needs_pty(&["cargo".to_string(), "build".to_string()]));
        assert!(!needs_pty(&["docker".to_string(), "ps".to_string()]));
    }

    #[test]
    fn test_needs_pty_empty() {
        assert!(!needs_pty(&[]));
    }

    #[test]
    fn test_needs_pty_sudo_wrapping() {
        assert!(needs_pty(&[
            "sudo".to_string(),
            "vim".to_string(),
            "file".to_string()
        ]));
        assert!(needs_pty(&[
            "sudo".to_string(),
            "-u".to_string(),
            "root".to_string(),
            "htop".to_string()
        ]));
        assert!(needs_pty(&[
            "sudo".to_string(),
            "git".to_string(),
            "commit".to_string()
        ]));
        assert!(!needs_pty(&["sudo".to_string(), "ls".to_string()]));
    }

    #[test]
    fn test_needs_pty_docker_interactive() {
        assert!(needs_pty(&[
            "docker".to_string(),
            "run".to_string(),
            "-it".to_string(),
            "ubuntu".to_string()
        ]));
        assert!(needs_pty(&[
            "docker".to_string(),
            "exec".to_string(),
            "-it".to_string(),
            "container".to_string(),
            "bash".to_string()
        ]));
        assert!(!needs_pty(&[
            "docker".to_string(),
            "run".to_string(),
            "ubuntu".to_string(),
            "ls".to_string()
        ]));
        assert!(!needs_pty(&["docker".to_string(), "ps".to_string()]));
    }

    #[test]
    fn test_needs_pty_kubectl() {
        assert!(needs_pty(&[
            "kubectl".to_string(),
            "edit".to_string(),
            "pod".to_string(),
            "mypod".to_string()
        ]));
        assert!(needs_pty(&[
            "kubectl".to_string(),
            "exec".to_string(),
            "-it".to_string(),
            "pod".to_string()
        ]));
        assert!(!needs_pty(&[
            "kubectl".to_string(),
            "get".to_string(),
            "pods".to_string()
        ]));
    }

    #[test]
    fn test_needs_pty_crontab() {
        assert!(needs_pty(&["crontab".to_string(), "-e".to_string()]));
        assert!(!needs_pty(&["crontab".to_string(), "-l".to_string()]));
    }

    #[test]
    fn test_needs_pty_system_editors() {
        assert!(needs_pty(&["visudo".to_string()]));
        assert!(needs_pty(&["vipw".to_string()]));
        assert!(needs_pty(&["vigr".to_string()]));
    }

    #[test]
    fn test_needs_pty_database_clis() {
        assert!(needs_pty(&["psql".to_string()]));
        assert!(needs_pty(&["mysql".to_string()]));
        assert!(needs_pty(&["sqlite3".to_string()]));
        assert!(needs_pty(&["redis-cli".to_string()]));
    }

    #[test]
    fn test_needs_pty_repls() {
        assert!(needs_pty(&["python".to_string()]));
        assert!(needs_pty(&["python3".to_string()]));
        assert!(needs_pty(&["node".to_string()]));
        assert!(needs_pty(&["irb".to_string()]));
        assert!(!needs_pty(&["python".to_string(), "script.py".to_string()]));
        assert!(!needs_pty(&["node".to_string(), "app.js".to_string()]));
    }
}
