# phos

> Greek: φως "light"

A fast, universal log colorizer. 98 programs built-in, 13 themes, webhook alerts.

## Install

```bash
# Homebrew (macOS/Linux)
brew install Hydepwns/phos/phos

# Cargo (crates.io)
cargo install phoscat

# Arch Linux (AUR)
yay -S phos

# Nix
nix run github:Hydepwns/phos

# Docker
docker run --rm -i mfdroo/phos

# From source
cargo install --git https://github.com/Hydepwns/phos
```

## Usage

```bash
phos -p docker -- docker logs mycontainer    # Wrap command
docker logs myapp | phos -p docker           # Pipe mode
phos -- docker logs mycontainer              # Auto-detect program
phos -c lodestar -t dracula -- docker logs   # Ethereum client + theme
```

## Shell Integration

Add to your shell config for automatic colorization of common commands:

```bash
# bash (~/.bashrc)
eval "$(phos shell-init bash)"

# zsh (~/.zshrc)
eval "$(phos shell-init zsh)"

# fish (~/.config/fish/config.fish)
phos shell-init fish | source
```

This gives you:
- `p` alias for phos (e.g., `p -p docker -- docker logs`)
- Auto-wrapped commands: git, cargo, npm, docker, kubectl, terraform, etc.
- Disable with `export PHOS_NO_ALIASES=1`

## Pipe with Auto-Detect

```bash
# phoscat auto-detects the program from log content
docker logs mycontainer | phoscat
cargo build 2>&1 | phoscat

# Or specify explicitly
docker logs mycontainer | phoscat docker
```

## Commands

```bash
phos list                    # List all 98 programs
phos list -c ethereum        # List by category
phos themes                  # List 13 themes
phos info docker             # Program details
phos preview                 # Preview themes
```

## Programs (98)

| Category | Count | Examples |
|----------|-------|----------|
| Ethereum | 15 | lighthouse, geth, lodestar, prysm, reth |
| System | 26 | systemd, syslog, dmesg, ps, df, iptables |
| Network | 21 | ping, curl, nginx, dig, tcpdump |
| Dev | 15 | git, cargo, npm, go, make, gcc |
| DevOps | 8 | docker, kubectl, terraform, helm, ansible |
| Data | 5 | postgres, redis, mysql, mongodb, elasticsearch |
| Monitoring | 4 | prometheus, grafana, datadog |
| Messaging | 2 | kafka, rabbitmq |
| CI/CD | 2 | github-actions, jenkins |

See [ETHEREUM.md](ETHEREUM.md) for Ethereum-specific guides.

## Themes

```bash
phos -t dracula      # Dracula
phos -t nord         # Nord
phos -t catppuccin   # Catppuccin Mocha
phos -t gruvbox      # Gruvbox
phos -t tokyo-night  # Tokyo Night
phos -t matrix       # Green monochrome
```

All 13: default-dark, dracula, nord, catppuccin, synthwave84, gruvbox, monokai, solarized, matrix, phosphor, tokyo-night, horizon, high-contrast

## Alerting

```bash
# Discord/Telegram webhooks on errors, peer drops, sync stalls
phos -c lodestar --alert "https://discord.com/api/webhooks/xxx/yyy" \
  --alert-on error --alert-on "peer-drop:10" -- docker logs -f lodestar
```

Conditions: `error`, `error-threshold:N`, `peer-drop:N`, `sync-stall`, `pattern:REGEX`

## Statistics

```bash
phos -p docker --stats -- docker logs mycontainer
phos -p docker --stats-export json -- docker logs mycontainer
docker logs -f myapp | phos -p docker --stats-interval 30
```

## Custom Programs

Create `~/.config/phos/programs/myapp.yaml`:

```yaml
name: myapp
category: custom
detect: [myapp]
rules:
  - regex: '\[ERROR\]'
    colors: [error]
    bold: true
```

## Custom Themes

Create `~/.config/phos/themes/mytheme.yaml`:

```yaml
name: mytheme
palette:
  red: "#FF5555"
  green: "#50FA7B"
  cyan: "#8BE9FD"
```

## Library

```rust
use phos::{Colorizer, Theme, programs};

let registry = programs::default_registry();
let program = registry.get("docker").unwrap();
let mut colorizer = Colorizer::new(program.rules()).with_theme(Theme::dracula());
println!("{}", colorizer.colorize("container abc123 started"));
```

## Performance

| Metric | Value |
|--------|-------|
| Binary | 4.6 MB |
| Throughput | >370k lines/sec |
| Memory | ~2 MB |

## Acknowledgments

Inspired by [grc](https://github.com/garabik/grc) (Generic Colouriser) by Radovan Garabik.

## License

MIT OR Apache-2.0
