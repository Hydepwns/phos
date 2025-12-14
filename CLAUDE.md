# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

phos is a universal log colorizer with built-in support for 98 programs across multiple domains:

- **Ethereum**: Lighthouse, Prysm, Teku, Nimbus, Lodestar, Grandine, Lambda, Geth, Nethermind, Besu, Erigon, Reth, Mana, Charon, MEV-Boost (15)
- **System**: systemd/journalctl, syslog, fail2ban, dmesg, cron, auditd, iptables/nftables, ls, df, du, stat, mount, ps, free, top, uptime, lsof, lsmod, lspci, vmstat, iostat, env, blkid, fdisk, lsblk, dnf (26)
- **Network**: ping, curl, dig, nginx, caddy, Apache, HAProxy, Traefik, traceroute, netstat, nmap, ss, sockstat, ifconfig, ip, iwconfig, arp, mtr, tcpdump, whois, ntpdate (21)
- **Dev**: git, cargo, npm, go, make, yarn, pnpm, elixir/mix, diff, gcc, wdiff, configure, ant, mvn, php (15)
- **DevOps**: Docker, kubectl, Terraform, k9s, Helm, Ansible, docker-compose, AWS CLI (8)
- **Data**: PostgreSQL, Redis, MySQL/MariaDB, MongoDB, Elasticsearch (5)
- **Monitoring**: Prometheus, Grafana, Datadog, SigNoz (4)
- **Messaging**: Kafka, RabbitMQ (2)
- **CI/CD**: GitHub Actions, Jenkins (2)

**Repository**: https://github.com/Hydepwns/phos

## Build and Test Commands

```bash
cargo build                    # Debug build
cargo build --release          # Optimized release (~1.9MB binary)
cargo test                     # Run all tests
cargo test test_name           # Run specific test
cargo clippy                   # Lint
cargo fmt                      # Format code

# Run the CLI
cargo run -- -p docker -- docker logs -f mycontainer
cargo run -- -c lodestar -- docker logs -f lodestar  # -c for Ethereum clients
cargo run -- list              # List all programs
cargo run -- list-clients      # List Ethereum clients only
cargo run -- themes            # List all 13 themes
echo "ERROR slot=12345" | cargo run -- -c lodestar
echo "error: build failed" | cargo run -- -p cargo
```

## Architecture

```
src/
  lib.rs             # Public API, module exports
  main.rs            # CLI binary (phos)
  colorizer.rs       # Core colorization engine
  colors.rs          # Color enum, SemanticColor, ANSI codes
  rule.rs            # Rule struct, RuleBuilder, CountMode
  theme.rs           # Theme system (13 built-in themes)
  config.rs          # YAML/JSON configuration loading
  clients/
    mod.rs           # Client enum, Layer enum, ethereum_patterns module
  program/
    mod.rs           # Program trait, ProgramRegistry, SimpleProgram
    config.rs        # User program config parsing (YAML/JSON)
    loader.rs        # Config directory discovery, user program loading
  programs/
    mod.rs           # default_registry(), program registration
    common.rs        # Shared rule builders (log levels, IPs, timestamps, HTTP)
    ethereum/        # Ethereum client programs (15)
    devops/          # Docker, kubectl, Terraform, k9s, Helm, Ansible, docker-compose, AWS (8)
    system/          # System utilities (26)
      mod.rs         # Registration
      core.rs        # systemd, syslog, dmesg, cron
      security.rs    # fail2ban, auditd, iptables
      files.rs       # ls, df, du, stat, mount
      processes.rs   # ps, free, top, uptime, lsof
      hardware.rs    # lsmod, lspci, vmstat, iostat, env, blkid, fdisk, lsblk
      packages.rs    # dnf
    dev/             # Development tools (15)
      mod.rs         # Registration
      vcs.rs         # git, diff, wdiff
      rust.rs        # cargo
      node.rs        # npm, yarn, pnpm
      build.rs       # make, gcc, configure, ant, mvn
      lang.rs        # go, elixir, php
    network/         # Network tools (21)
      mod.rs         # Registration
      tools.rs       # ping, curl, dig, traceroute, nmap
      servers.rs     # nginx, caddy, apache, haproxy, traefik
      sockets.rs     # netstat, ss, sockstat
      interfaces.rs  # ifconfig, ip, iwconfig, arp
      diagnostics.rs # mtr, tcpdump, whois, ntpdate
    data/            # PostgreSQL, Redis, MySQL, MongoDB, Elasticsearch (5)
    monitoring/      # Prometheus, Grafana, Datadog, SigNoz (4)
    messaging/       # Kafka, RabbitMQ (2)
    ci/              # GitHub Actions, Jenkins (2)
```

## Data Flow

```bash
USER INPUT (phos -c lodestar -- docker logs)
         |
         v
    CLI PARSING (main.rs)
    - Parse --client, --theme, --config
    - Detect stdin pipe vs command execution
         |
         v
    RULE RESOLUTION (clients/mod.rs + config.rs)
    - Load client-specific rules OR config file
    - Auto-detect client from command if not specified
         |
         v
    THEME RESOLUTION (theme.rs)
    - Load built-in theme
    - Map semantic colors to RGB/ANSI values
         |
         v
    COLORIZATION (colorizer.rs)
    - Apply rules in order
    - Match regex patterns
    - Wrap matches in ANSI escape codes
         |
         v
    OUTPUT (colorized stdout/stderr)
```

## Key Types

```rust
// Core
Rule { regex, colors: Vec<Color>, count_mode, bold }
Color::Named("red") | Color::Hex("#FF5555") | Color::Semantic(SemanticColor::Error)
SemanticColor { Error, Warn, Info, Slot, Epoch, Hash, Address, ... }
Theme { name, colors: HashMap<SemanticColor, Color> }
Colorizer { rules, theme }

// Program system
Program                    // Trait: info(), rules(), detect_patterns(), domain_colors()
ProgramRegistry            // Holds all registered programs, detect(), get(), list_by_category()
SimpleProgram              // Convenience struct implementing Program trait
ProgramInfo { id, name, description, category }

// Ethereum (legacy, still supported)
Client { Lighthouse, Prysm, Geth, Lodestar, Mana, ... } // 15 total
```

## Key Design Decisions

1. **Semantic Colors**: Rules use `SemanticColor::Error` that themes resolve to actual colors. One ruleset works across all themes.

2. **Zero-Copy**: Minimize allocations. Use `&str` over `String`. Consider `Cow<'_, str>`.

3. **Compiled Regex**: All patterns compiled once at rule creation. Never in hot paths.

4. **Shared Rule Builders**: Common patterns in `programs/common.rs` reduce duplication:
   - `log_level_rules()` - ERROR, WARN, INFO, DEBUG, TRACE
   - `ip_rules()` - IPv4, IPv6 addresses
   - `timestamp_rules()` - ISO8601, syslog, etc.
   - `metric_rules()` - durations, percentages, bytes

5. **Ethereum Pattern Helpers**: In `clients/mod.rs::ethereum_patterns`:
   - `rust_log_levels()`, `lighthouse_log_levels()`, `prysm_log_levels()`, etc.
   - `consensus_patterns()`, `execution_patterns()`, `mev_patterns()`

## Ethereum Domain Knowledge

Key concepts:
- **Slot**: 12-second time unit. **Epoch**: 32 slots (~6.4 minutes)
- **Attestation**: Validator vote. **Proposal**: Creating a block
- **Finality**: Irreversible checkpoint. **Forkchoice**: Canonical chain selection

Log patterns:
- Hashes: `0x` + 64 hex chars
- Addresses: `0x` + 40 hex chars
- Slot/Epoch: `slot=12345 epoch=385` or `Eph 385/12` (Lodestar)

## Adding a New Program

**Option 1: Built-in program (recommended for common tools)**

In `src/programs/<category>/mod.rs`:
1. Create a function returning `Arc<dyn Program>` using `SimpleProgram::new()`
2. Register in `register_<category>_programs()` function
3. Add detection patterns via `.with_detect_patterns()`

Example:
```rust
pub fn mytool_program() -> Arc<dyn Program> {
    Arc::new(SimpleProgram::new(
        "devops.mytool", "MyTool", "Description here", "devops",
        vec![/* rules */]
    ).with_detect_patterns(vec!["mytool"]))
}
```

**Option 2: User-defined program (YAML/JSON config)**

Create `~/.config/phos/programs/myprogram.yaml`:
```yaml
name: MyProgram
description: My custom program
category: custom
detect: ["myprogram", "myprog"]
rules:
  - regex: '\bERROR\b'
    colors: [error]
    bold: true
```

## Adding a New Ethereum Client (Legacy)

In `src/clients/mod.rs`:
1. Add variant to `Client` enum
2. Add to `Client::all()` vector
3. Implement `Client::info()` match arm
4. Create `client_rules()` function using `ethereum_patterns` helpers
5. Add parse case in `Client::parse()`
6. Add brand color in `src/colors.rs` `brands::color()`

## Adding a New Theme

In `src/theme.rs`:
1. Add `pub fn my_theme() -> Self` method
2. Add to `Theme::builtin()` match
3. Add to `Theme::list_builtin()` vector

## Common Pitfalls

**Regex Backtracking**:
```rust
// BAD: Can hang on long strings
r"(a+)+"
// GOOD: Rewrite without nested quantifiers
r"a+"
```

**ANSI Nesting**:
```rust
// BAD: Nested colors break
"\x1b[31m red \x1b[32m green \x1b[0m"
// GOOD: Reset between colors
"\x1b[31m red \x1b[0m\x1b[32m green \x1b[0m"
```

**Terminal Compatibility**: Not all terminals support 24-bit color. Consider fallbacks.

**Log Format Evolution**: Clients update formats between versions. Test against real logs.

## Sample Log Lines for Testing

```bash
# Ethereum - Lodestar
Dec 05 00:12:36.557[] info: Synced - Peers 47 - Eph 167991/6 - slot 5375712

# Ethereum - Geth
INFO [12-05|00:12:36.557] Imported new chain segment number=19630289 hash=0x4f6a...

# Ethereum - Lighthouse
Dec 05 00:12:36.557 INFO Synced slot: 12345, epoch: 385

# DevOps - Docker
2024-01-15T10:30:45.123Z INFO Container started id=abc123

# DevOps - kubectl
error: the server doesn't have a resource type "pods"

# Dev - Cargo
error[E0382]: borrow of moved value: `x`
warning: unused variable: `foo`

# Dev - npm
npm ERR! 404 Not Found - GET https://registry.npmjs.org/nonexistent

# System - systemd
Dec 15 10:30:45 hostname systemd[1]: Started My Service.

# Network - ping
64 bytes from 8.8.8.8: icmp_seq=1 ttl=117 time=14.2 ms
```

## Performance Guidelines

- **Regex**: Avoid catastrophic backtracking. Test with long strings.
- **Rule ordering**: Most common matches first (log levels before rare patterns)
- **Memory**: Stream processing, don't load entire files

## Code Style

- `rustfmt` defaults
- `thiserror` for library errors, `anyhow` in binaries
- Document public APIs with `///` doc comments
