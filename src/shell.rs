//! Shell integration for automatic command colorization.
//!
//! Generates shell scripts that wrap supported commands with phos.

use crate::ProgramRegistry;

/// Programs suitable for shell aliasing (direct command wrappers).
/// These are commands users run directly that benefit from colorization.
const ALIASABLE_PROGRAMS: &[(&str, &[&str])] = &[
    // Network tools
    ("ping", &["ping"]),
    ("dig", &["dig"]),
    ("curl", &["curl"]),
    // Dev tools
    ("git", &["git"]),
    ("cargo", &["cargo"]),
    ("npm", &["npm"]),
    ("yarn", &["yarn"]),
    ("pnpm", &["pnpm"]),
    ("make", &["make", "gmake"]),
    ("go", &["go"]),
    // DevOps tools
    ("docker", &["docker"]),
    ("docker-compose", &["docker-compose"]),
    ("kubectl", &["kubectl"]),
    ("helm", &["helm"]),
    ("terraform", &["terraform", "tofu"]),
    ("ansible", &["ansible", "ansible-playbook"]),
    ("aws", &["aws"]),
    // System tools
    ("dmesg", &["dmesg"]),
    ("journalctl", &["journalctl"]),
    ("systemctl", &["systemctl"]),
];

/// Shell types supported for integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
}

impl ShellType {
    /// Parse shell type from string.
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "bash" => Some(Self::Bash),
            "zsh" => Some(Self::Zsh),
            "fish" => Some(Self::Fish),
            _ => None,
        }
    }

    /// Get list of supported shell names.
    pub fn supported() -> &'static [&'static str] {
        &["bash", "zsh", "fish"]
    }
}

/// Generate shell integration script.
pub fn generate_script(shell: ShellType, registry: &ProgramRegistry) -> String {
    match shell {
        ShellType::Bash => generate_bash(registry),
        ShellType::Zsh => generate_zsh(registry),
        ShellType::Fish => generate_fish(registry),
    }
}

/// Generate bash shell integration script.
fn generate_bash(registry: &ProgramRegistry) -> String {
    let mut script = String::from(
        r#"# phos shell integration for bash
# Add to ~/.bashrc: eval "$(phos shell-init bash)"

# Skip if not interactive or no TTY
[[ $- != *i* ]] && return
[[ ! -t 1 ]] && return

# Skip if PHOS_NO_ALIASES is set
[[ -n "${PHOS_NO_ALIASES:-}" ]] && return

# Wrapper function that preserves exit codes
__phos_wrap() {
    local prog="$1"
    shift
    phos -p "$prog" -- "$prog" "$@"
}

"#,
    );

    // Generate aliases for each program
    ALIASABLE_PROGRAMS
        .iter()
        .filter(|(program, _)| registry.get(program).is_some())
        .flat_map(|(program, commands)| commands.iter().map(move |cmd| (program, cmd)))
        .for_each(|(program, cmd)| {
            script.push_str(&format!(
                r#"if command -v {cmd} &>/dev/null && ! alias {cmd} &>/dev/null 2>&1; then
    alias {cmd}='__phos_wrap {program}'
fi
"#
            ));
        });

    script
}

/// Generate zsh shell integration script.
fn generate_zsh(registry: &ProgramRegistry) -> String {
    let mut script = String::from(
        r#"# phos shell integration for zsh
# Add to ~/.zshrc: eval "$(phos shell-init zsh)"

# Skip if not interactive or no TTY
[[ ! -o interactive ]] && return
[[ ! -t 1 ]] && return

# Skip if PHOS_NO_ALIASES is set
[[ -n "${PHOS_NO_ALIASES:-}" ]] && return

"#,
    );

    // Generate functions for each program (zsh functions preserve completions better)
    ALIASABLE_PROGRAMS
        .iter()
        .filter(|(program, _)| registry.get(program).is_some())
        .flat_map(|(program, commands)| commands.iter().map(move |cmd| (program, cmd)))
        .for_each(|(program, cmd)| {
            script.push_str(&format!(
                r#"if (( $+commands[{cmd}] )) && ! (( $+functions[{cmd}] )) && ! (( $+aliases[{cmd}] )); then
    {cmd}() {{ phos -p {program} -- $commands[{cmd}] "$@" }}
fi
"#
            ));
        });

    script
}

/// Generate fish shell integration script.
fn generate_fish(registry: &ProgramRegistry) -> String {
    let mut script = String::from(
        r#"# phos shell integration for fish
# Add to ~/.config/fish/config.fish: phos shell-init fish | source

# Skip if not interactive or no TTY
not status is-interactive; and return
not isatty stdout; and return

# Skip if PHOS_NO_ALIASES is set
set -q PHOS_NO_ALIASES; and return

"#,
    );

    // Generate functions for each program
    ALIASABLE_PROGRAMS
        .iter()
        .filter(|(program, _)| registry.get(program).is_some())
        .flat_map(|(program, commands)| commands.iter().map(move |cmd| (program, cmd)))
        .for_each(|(program, cmd)| {
            script.push_str(&format!(
                r#"if type -q {cmd}; and not functions -q {cmd}
    function {cmd} --wraps={cmd} --description 'phos-wrapped {cmd}'
        phos -p {program} -- (command -v {cmd}) $argv
    end
end
"#
            ));
        });

    script
}

/// List all programs that would be aliased.
pub fn list_aliasable(registry: &ProgramRegistry) -> Vec<(&'static str, &'static [&'static str])> {
    ALIASABLE_PROGRAMS
        .iter()
        .filter(|(program, _)| registry.get(program).is_some())
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::programs;

    #[test]
    fn test_shell_type_parse() {
        assert_eq!(ShellType::parse("bash"), Some(ShellType::Bash));
        assert_eq!(ShellType::parse("BASH"), Some(ShellType::Bash));
        assert_eq!(ShellType::parse("zsh"), Some(ShellType::Zsh));
        assert_eq!(ShellType::parse("fish"), Some(ShellType::Fish));
        assert_eq!(ShellType::parse("unknown"), None);
    }

    #[test]
    fn test_bash_script_generation() {
        let registry = programs::default_registry();
        let script = generate_bash(&registry);

        assert!(script.contains("__phos_wrap"));
        assert!(script.contains("docker"));
        assert!(script.contains("kubectl"));
    }

    #[test]
    fn test_zsh_script_generation() {
        let registry = programs::default_registry();
        let script = generate_zsh(&registry);

        assert!(script.contains("$+commands"));
        assert!(script.contains("docker"));
    }

    #[test]
    fn test_fish_script_generation() {
        let registry = programs::default_registry();
        let script = generate_fish(&registry);

        assert!(script.contains("--wraps"));
        assert!(script.contains("docker"));
    }

    #[test]
    fn test_list_aliasable() {
        let registry = programs::default_registry();
        let aliasable = list_aliasable(&registry);

        // Should have at least the core programs
        assert!(aliasable.iter().any(|(p, _)| *p == "docker"));
        assert!(aliasable.iter().any(|(p, _)| *p == "git"));
    }
}
