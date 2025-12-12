# phos

> Greek: phos -- "light"

A fast, universal log colorizer for programs.
Inspired by [grc](https://github.com/garabik/grc) by Radovan Garabik.

## Features

- **Fast**: ~2MB binary, instant startup
- **59 programs built-in**: Ethereum, DevOps, Dev, Network, Data, System, CI/CD, Messaging, Monitoring
- **Theme system**: 13 themes including Dracula, Nord, Catppuccin
- **Auto-detection**: Detects program from command
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

## Commands

```bash
# List all programs
phos list

# List by category
phos list --category devops
phos list --category ethereum

# List themes
phos themes

# Show program info
phos info docker
phos info lodestar

# Show Ethereum brand colors
phos colors

# Generate shell completions
phos completions bash > ~/.local/share/bash-completion/completions/phos
phos completions zsh > ~/.zfunc/_phos
phos completions fish > ~/.config/fish/completions/phos.fish
```

## Built-in Programs

### Ethereum (15 clients)

| Layer | Clients |
|-------|---------|
| Consensus | Lighthouse, Prysm, Teku, Nimbus, Lodestar, Grandine, Lambda |
| Execution | Geth, Nethermind, Besu, Erigon, Reth |
| Full Node | Mana |
| Middleware | Charon, MEV-Boost |

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

### Development (8)

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

### Network (8)

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

### Data (5)

| Program | Description |
|---------|-------------|
| postgres | PostgreSQL database logs |
| redis | Redis server logs |
| mysql | MySQL/MariaDB database logs |
| mongodb | MongoDB database logs |
| elasticsearch | Elasticsearch cluster logs |

### System (7)

| Program | Description |
|---------|-------------|
| systemd | Journalctl and systemd unit logs |
| syslog | Traditional syslog format |
| fail2ban | Fail2ban intrusion prevention logs |
| dmesg | Kernel ring buffer messages |
| cron | Cron scheduled task logs |
| auditd | Linux audit daemon logs |
| iptables | iptables/nftables firewall logs |

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
| Ethereum | hash, address, slot, epoch, validator, pubkey |

## Library Usage

```rust
use phos::{Colorizer, Client, Theme};
use phos::programs;

// Use program registry
let registry = programs::default_registry();
if let Some(program) = registry.get("docker") {
    let mut colorizer = Colorizer::new(program.rules())
        .with_theme(Theme::dracula());
    println!("{}", colorizer.colorize("container abc123 started"));
}

// Ethereum client (legacy API still works)
let mut colorizer = Colorizer::for_client(Client::Lodestar)
    .with_theme(Theme::nord());
println!("{}", colorizer.colorize("INFO: Synced slot 12345"));

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
