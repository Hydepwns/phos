# phos

> Greek: phos -- "light"

A fast, universal log colorizer for programs.
Inspired by [grc](https://github.com/garabik/grc) by Radovan Garabik.

## Features

- **Fast**: ~2MB binary, instant startup
- **99 programs built-in**: Ethereum, DevOps, Dev, Network, Data, System, CI/CD, Messaging, Monitoring
- **Theme system**: 13 themes including Dracula, Nord, Catppuccin
- **Auto-detection**: Detects program from command
- **Alerting**: Webhook notifications to Discord/Telegram on errors, peer drops, sync stalls
- **Extensible**: Add custom programs via YAML configs
- **Zero dependencies**: Single static binary

## Installation

### Homebrew (macOS/Linux)

```bash
brew tap Hydepwns/phos
brew install phos
```

### From Source

```bash
# Build from source
cargo build --release

# Install
cargo install --path .
```

### Pre-built Binaries

```bash
# Linux
curl -sSL https://github.com/Hydepwns/phos/releases/latest/download/phos-linux-amd64 -o phos
chmod +x phos
sudo mv phos /usr/local/bin/

# macOS (Intel)
curl -sSL https://github.com/Hydepwns/phos/releases/latest/download/phos-darwin-amd64 -o phos
chmod +x phos
sudo mv phos /usr/local/bin/

# macOS (Apple Silicon)
curl -sSL https://github.com/Hydepwns/phos/releases/latest/download/phos-darwin-arm64 -o phos
chmod +x phos
sudo mv phos /usr/local/bin/
```

## Quick Start

```bash
# DevOps
phos -p docker -- docker logs mycontainer
phos -p kubectl -- kubectl get pods
phos -p terraform -- terraform plan

# Development
phos -p cargo -- cargo test
phos -p git -- git diff
phos -p npm -- npm install

# Network
phos -p ping -- ping google.com
phos -p curl -- curl -v https://api.example.com

# System
phos -p systemd -- journalctl -f
phos -p syslog -- tail -f /var/log/syslog

# Ethereum
phos -c lodestar -- docker logs lodestar
phos -c geth -t dracula -- journalctl -u geth -f

# Pipe mode
docker logs myapp | phos -p docker
cargo test 2>&1 | phos -p cargo

# Auto-detect (no -p needed)
phos -- docker logs mycontainer
phos -- cargo test

# Statistics mode
docker logs myapp | phos -p docker --stats
phos -p cargo --stats -- cargo test
```

## phoscat (Pipe-only Binary)

For pure pipe workflows, `phoscat` is a minimal binary with faster startup:

```bash
# Basic usage
docker logs myapp | phoscat docker
cargo test 2>&1 | phoscat cargo
kubectl get pods | phoscat kubectl

# With environment variables
export PHOS_PROGRAM=docker
export PHOS_THEME=dracula
docker logs myapp | phoscat

# One-liner with env
PHOS_PROGRAM=geth journalctl -u geth | phoscat
```

## Commands

```bash
# List all programs
phos list
phos list --format json              # JSON output for scripting

# List by category
phos list --category devops
phos list --category ethereum

# List themes
phos themes

# Show program info
phos info docker
phos info lodestar --format json     # JSON output

# Show Ethereum brand colors
phos colors

# Generate shell completions
phos completions bash > ~/.local/share/bash-completion/completions/phos
phos completions zsh > ~/.zfunc/_phos
phos completions fish > ~/.config/fish/completions/phos.fish

# Shell integration (auto-colorize commands)
eval "$(phos shell-init bash)"       # Add to ~/.bashrc
eval "$(phos shell-init zsh)"        # Add to ~/.zshrc
phos shell-init fish | source        # Add to config.fish

# Configuration management
phos config path                     # Show config directory paths
phos config init                     # Initialize config with example
phos config validate                 # Validate all configs
phos config validate myapp.yaml      # Validate specific file

# Generate man pages
phos man                             # Print man page to stdout
phos man -o ./man                    # Generate all man pages to directory

# Preview themes
phos preview                         # Preview all themes with sample output
phos preview -t dracula              # Preview specific theme
phos preview -q                      # Compact output
phos preview -c                      # Category-centric view
```

## Built-in Programs

### Ethereum (15 clients)

| Layer | Clients |
|-------|---------|
| Consensus | Lighthouse, Prysm, Teku, Nimbus, Lodestar, Grandine, Lambda |
| Execution | Geth, Nethermind, Besu, Erigon, Reth |
| Full Node | Mana |
| Middleware | Charon, MEV-Boost |

See [ETHEREUM.md](ETHEREUM.md) for eth-docker integration, usage examples, and tooling guides.

### DevOps (8)

| Program | Description |
|---------|-------------|
| docker | Container logs, ps, images |
| kubectl | Kubernetes pods, events, logs |
| terraform | Plan output with +/-/~ highlighting |
| k9s | K9s Kubernetes CLI logs |
| helm | Helm package manager output |
| ansible | Ansible playbook output |
| docker-compose | Docker Compose logs and output |
| aws | AWS CLI output |

### Development (15)

| Program | Description |
|---------|-------------|
| git | Diff, status, log output |
| cargo | Rust build and test output |
| npm | Node.js package manager output |
| go | Go build and test output |
| make | Make build output |
| yarn | Yarn package manager output |
| pnpm | pnpm package manager output |
| elixir | Elixir/Mix build and test output |
| diff | File comparison output |
| wdiff | Word diff output |
| gcc | GCC/G++/Clang compiler output |
| configure | Autoconf configure script output |
| ant | Apache Ant build output |
| mvn | Apache Maven build output |
| php | PHP interpreter and composer output |

### Network (21)

| Program | Description |
|---------|-------------|
| ping | Latency and packet loss |
| curl | HTTP status codes and timing |
| dig | DNS query results |
| nginx | Nginx access and error logs |
| caddy | Caddy web server logs |
| apache | Apache/httpd web server logs |
| haproxy | HAProxy load balancer logs |
| traefik | Traefik reverse proxy logs |
| traceroute | Network path tracing |
| nmap | Network scanner output |
| netstat | Network statistics |
| ss | Socket statistics |
| sockstat | BSD socket statistics |
| ifconfig | Network interface config |
| ip | Linux ip command output |
| iwconfig | Wireless interface config |
| arp | ARP table output |
| mtr | My traceroute output |
| tcpdump | Packet capture output |
| whois | Domain lookup output |
| ntpdate | NTP time sync output |

### Data (5)

| Program | Description |
|---------|-------------|
| postgres | PostgreSQL database logs |
| redis | Redis server logs |
| mysql | MySQL/MariaDB database logs |
| mongodb | MongoDB database logs |
| elasticsearch | Elasticsearch cluster logs |

### System (26)

| Program | Description |
|---------|-------------|
| systemd | Journalctl and systemd unit logs |
| syslog | Traditional syslog format |
| fail2ban | Fail2ban intrusion prevention logs |
| dmesg | Kernel ring buffer messages |
| cron | Cron scheduled task logs |
| auditd | Linux audit daemon logs |
| iptables | iptables/nftables firewall logs |
| ls | Directory listing output |
| df | Disk space usage |
| du | Disk usage output |
| stat | File statistics output |
| mount | Filesystem mount output |
| ps | Process status output |
| free | Memory usage output |
| top | Process monitor output |
| uptime | System uptime output |
| lsof | List open files output |
| lsmod | List kernel modules |
| lspci | List PCI devices |
| vmstat | Virtual memory statistics |
| iostat | I/O statistics output |
| env | Environment variables |
| blkid | Block device attributes |
| fdisk | Partition table output |
| lsblk | List block devices |
| dnf | DNF package manager output |

### CI/CD (2)

| Program | Description |
|---------|-------------|
| github-actions | GitHub Actions workflow output |
| jenkins | Jenkins build output |

### Messaging (2)

| Program | Description |
|---------|-------------|
| kafka | Apache Kafka broker logs |
| rabbitmq | RabbitMQ message broker logs |

### Monitoring (4)

| Program | Description |
|---------|-------------|
| prometheus | Prometheus server logs |
| grafana | Grafana server logs |
| datadog | Datadog agent logs |
| signoz | SigNoz observability platform logs |

## Themes

```bash
phos -t default-dark  # Default dark theme
phos -t dracula       # Dracula color scheme
phos -t nord          # Nord arctic theme
phos -t catppuccin    # Catppuccin Mocha
phos -t synthwave84   # Retro neon aesthetic
phos -t gruvbox       # Retro earthy tones
phos -t monokai       # Classic editor theme
phos -t solarized     # Precision colors
phos -t matrix        # Green monochrome
phos -t phosphor      # Amber CRT terminal
phos -t tokyo-night   # City lights aesthetic
phos -t horizon       # Warm sunset colors
phos -t high-contrast # Maximum readability
```

## Statistics Mode

Use `--stats` to collect and display log statistics after processing:

```bash
phos -p docker --stats -- docker logs mycontainer
cat /var/log/syslog | phos -p syslog --stats
```

Statistics include:
- Total lines processed and match percentage
- Time range (first and last timestamp)
- Log level distribution (ERROR, WARN, INFO, DEBUG, TRACE)
- Top error messages
- Error rate

## Alerting

Send webhook notifications to Discord or Telegram when specific conditions are detected:

```bash
# Discord webhook - alert on errors
phos -c lodestar --alert "https://discord.com/api/webhooks/xxx/yyy" \
  -- docker logs -f lodestar

# Telegram webhook - alert on errors
phos -c geth --alert "https://api.telegram.org/botTOKEN/sendMessage" \
  --telegram-chat-id "-1001234567890" \
  -- journalctl -u geth -f

# Multiple conditions with custom cooldown
phos -c lighthouse --alert "https://discord.com/api/webhooks/xxx/yyy" \
  --alert-on error \
  --alert-on "peer-drop:10" \
  --alert-on sync-stall \
  --alert-cooldown 300 \
  -- docker logs -f lighthouse
```

### Alert Conditions

| Condition | Description |
|-----------|-------------|
| `error` | Fire on first ERROR/FATAL/PANIC/CRIT detection |
| `error-threshold:N` | Fire when error count exceeds N |
| `peer-drop:N` | Fire when peer count drops below N |
| `sync-stall` | Fire when no sync progress is detected |
| `pattern:REGEX` | Fire on custom regex pattern match |

### Supported Services

- **Discord**: Rich embeds with color-coded severity (auto-detected from URL)
- **Telegram**: MarkdownV2 formatted messages (requires `--telegram-chat-id`)
- **Generic**: Simple JSON POST to any webhook URL

## Custom Programs

Create a YAML config in your programs directory:
- **Linux**: `~/.config/phos/programs/myapp.yaml`
- **macOS**: `~/Library/Application Support/phos/programs/myapp.yaml`
- **Windows**: `%APPDATA%\phos\programs\myapp.yaml`

```yaml
name: myapp
description: My application logs
category: custom

detect:
  - myapp
  - "docker.*myapp"

rules:
  - regex: '\[ERROR\]'
    colors: [error]
    bold: true
  - regex: '\[WARN\]'
    colors: [warn]
  - regex: '\[INFO\]'
    colors: [info]
  - regex: 'request_id=([a-f0-9-]+)'
    colors: [identifier]
  - regex: '\d+ms'
    colors: [metric]
```

Use with:
```bash
phos -p myapp -- tail -f /var/log/myapp.log
# Or auto-detect:
phos -- myapp --serve
```

## Semantic Colors

Rules can use semantic color names resolved by themes:

| Category | Colors |
|----------|--------|
| Log levels | error, warn, info, debug, trace |
| Data types | number, string, boolean |
| Identifiers | identifier, label, metric |
| Status | success, failure |
| Structure | timestamp, key, value |

Domain-specific colors (e.g., Ethereum's slot, epoch, hash) are defined as hex colors in each program's rules rather than as universal semantic colors.

## Library Usage

```rust
use phos::{Colorizer, Theme, programs};

// Use program registry
let registry = programs::default_registry();
if let Some(program) = registry.get("docker") {
    let mut colorizer = Colorizer::new(program.rules())
        .with_theme(Theme::dracula());
    println!("{}", colorizer.colorize("container abc123 started"));
}

// Auto-detect program from command
let cmd = "docker logs mycontainer";
if let Some(program) = registry.detect(cmd) {
    let colorizer = Colorizer::new(program.rules());
    // ...
}

// Custom rules
use phos::{Rule, SemanticColor};

let rules = vec![
    Rule::new(r"\d+").unwrap()
        .semantic(SemanticColor::Number)
        .build(),
];
let mut colorizer = Colorizer::new(rules);
```

## Performance

| Metric | Value |
|--------|-------|
| Binary size | ~2 MB |
| Startup time | <5ms |
| Memory | ~2 MB |
| Throughput | >100k lines/sec |

## Building

```bash
cargo build              # Debug build
cargo build --release    # Release build
cargo test               # Run tests
cargo clippy             # Lint
```

## License

MIT OR Apache-2.0
