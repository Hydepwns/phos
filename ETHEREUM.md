# Ethereum Tooling Guide

This guide covers using phos with Ethereum node management tools and clients.

## Overview

phos provides built-in colorization for **15 Ethereum clients** across all layers:

| Layer | Clients |
|-------|---------|
| Consensus | Lighthouse, Prysm, Teku, Nimbus, Lodestar, Grandine, Lambda |
| Execution | Geth, Nethermind, Besu, Erigon, Reth |
| Full Node | Mana |
| Middleware | Charon, MEV-Boost |

### Why phos for Ethereum logs?

- **Instant visual feedback**: Errors, warnings, slots, epochs, and hashes are highlighted
- **Client-aware**: Each client has tailored patterns for its log format
- **Theme support**: 13 themes including Dracula, Nord, and Catppuccin
- **Zero config**: Auto-detects clients from container/service names

## eth-docker Integration

[eth-docker](https://eth-docker.net/) is a popular Docker-based setup for running Ethereum nodes.

### Quick Start

```bash
# Install phos
brew install Hydepwns/phos/phos

# View consensus client logs with colorization
phos -c lighthouse -- docker logs eth-docker-lighthouse-bn-1 -f

# View execution client logs
phos -c geth -- docker logs eth-docker-geth-1 -f

# Auto-detect client from container name
phos -- docker logs eth-docker-lighthouse-bn-1 -f
```

### Using with ethd CLI

eth-docker provides the `ethd` CLI wrapper. Pipe logs through phos:

```bash
# Consensus client logs
./ethd logs consensus | phos -c lighthouse

# Execution client logs
./ethd logs execution | phos -c geth

# Follow logs with tail
./ethd logs -f consensus | phos -c lodestar
```

### Container Name Patterns

phos recognizes common eth-docker container naming patterns:

| Pattern | Example | Client |
|---------|---------|--------|
| `<client>-bn` | `lighthouse-bn-1` | Beacon node |
| `<client>-vc` | `lighthouse-vc-1` | Validator client |
| `<client>-el` | `geth-el-1` | Execution layer |
| `<client>_beacon` | `prysm_beacon` | Beacon (underscore) |
| `<client>_validator` | `prysm_validator` | Validator (underscore) |

### Shell Aliases

Add these to your `~/.bashrc` or `~/.zshrc`:

```bash
# eth-docker + phos aliases
alias ethd-cl='./ethd logs -f consensus | phos -c'
alias ethd-el='./ethd logs -f execution | phos -c'

# Usage: ethd-cl lighthouse
# Usage: ethd-el geth
```

Or for auto-detection:

```bash
# Auto-detect client from eth-docker
ethd-logs() {
    local service="${1:-consensus}"
    ./ethd logs -f "$service" | phos
}
```

### Docker Compose Integration

To always colorize logs, you can wrap Docker commands:

```bash
# Add to ~/.bashrc or ~/.zshrc
docker-logs() {
    docker logs "$@" 2>&1 | phos
}

# Usage: docker-logs -f eth-docker-lighthouse-bn-1
```

## Other Tooling

### DAppNode

Install the **phos** package from the DAppNode store. Smart alerts are pre-configured - just add a webhook URL!

**Quick Start**:
1. Install phos from DAppNode store
2. Add Discord/Telegram webhook URL in settings
3. Done - you're protected!

**Pre-configured alerts**: errors, peer drops (<10), sync stalls, reorgs, slashing

**Additional patterns you can add**:
- `pattern:missed.*head` - Missed head vote
- `pattern:attestable_delay` - Late block (>4s = missed)
- `pattern:optimistic` - EL not synced
- `pattern:MissingBeaconBlock` - DB corruption

**Filter by client**: `*geth*`, `*lighthouse*`, `*prysm*`, etc.

UI: http://phos.dappnode:8180

Technical: Connects to dappmanager via Socket.IO API for container logs.

### Sedge

*Coming soon* - Integration with Nethermind's Sedge CLI

### Stereum

*Coming soon* - Integration with Stereum node management

## Client Reference

### Detection Patterns

phos auto-detects clients from command strings. Each client recognizes these patterns:

| Client | Patterns |
|--------|----------|
| Lighthouse | `lighthouse`, `lighthouse-bn`, `lighthouse-vc`, `lighthouse.log` |
| Prysm | `prysm`, `beacon-chain`, `validator`, `prysm-bn`, `prysm-vc` |
| Teku | `teku`, `teku-bn`, `teku-vc`, `teku.log` |
| Nimbus | `nimbus`, `nimbus-bn`, `nimbus-vc`, `nimbus-eth2` |
| Lodestar | `lodestar`, `lodestar-bn`, `lodestar-vc` |
| Grandine | `grandine`, `grandine-bn`, `grandine-vc` |
| Lambda | `lambda_ethereum`, `lambda-bn` |
| Geth | `geth`, `geth-el`, `go-ethereum`, `geth.log` |
| Nethermind | `nethermind`, `nethermind-el`, `nethermind.log` |
| Besu | `besu`, `besu-el`, `hyperledger-besu` |
| Erigon | `erigon`, `erigon-el`, `erigon.log` |
| Reth | `reth`, `reth-el`, `reth.log` |
| Mana | `mana`, `mana-el`, `mana-cl` |
| Charon | `charon`, `charon-dv`, `obol-charon` |
| MEV-Boost | `mev-boost`, `mev_boost`, `mevboost` |

### Brand Colors

Each client has a distinctive brand color used in `phos colors` output:

```bash
# Show all Ethereum client brand colors
phos colors
```

### Sample Log Formats

**Lighthouse** (Rust/slog):
```
Dec 05 00:12:36.557 INFO Synced slot: 12345, epoch: 385, peers: 47
```

**Lodestar** (TypeScript):
```
Dec 05 00:12:36.557[] info: Synced - Peers 47 - Eph 167991/6 - slot 5375712
```

**Geth** (Go):
```
INFO [12-05|00:12:36.557] Imported new chain segment number=19630289 hash=0x4f6a...
```

**Prysm** (Go/logrus):
```
time="2024-12-05T00:12:36Z" level=info msg="Synced to slot" slot=12345
```

## Themes for Ethereum

### Recommended Themes

These themes work well with Ethereum logs:

```bash
# Dracula - popular dark theme
phos -c lighthouse -t dracula -- docker logs lighthouse

# Nord - arctic color palette
phos -c geth -t nord -- docker logs geth

# Catppuccin - pastel colors
phos -c lodestar -t catppuccin -- docker logs lodestar

# Matrix - green monochrome (for that crypto vibe)
phos -c prysm -t matrix -- docker logs prysm
```

### Preview All Themes

```bash
# List available themes
phos themes

# Preview themes with Ethereum logs
echo "INFO slot=12345 epoch=385 peers=47" | phos -c lighthouse -t dracula
echo "INFO slot=12345 epoch=385 peers=47" | phos -c lighthouse -t nord
echo "INFO slot=12345 epoch=385 peers=47" | phos -c lighthouse -t catppuccin
```

## Statistics Mode

Use `--stats` to collect log statistics after processing:

```bash
# Get log statistics from eth-docker
./ethd logs consensus | phos -c lighthouse --stats

# Statistics include:
# - Total lines and match percentage
# - Log level distribution (ERROR, WARN, INFO, DEBUG)
# - Top error messages
# - Time range
```

## Troubleshooting

### Client Not Detected

If auto-detection fails, specify the client explicitly:

```bash
# Instead of:
phos -- docker logs custom-container-name

# Use:
phos -c lighthouse -- docker logs custom-container-name
```

### No Colors in Pipe

Ensure your terminal supports colors and phos isn't detecting a non-TTY:

```bash
# Force colors even in pipes
docker logs mycontainer | phos -c geth --color always
```

### Performance with High-Volume Logs

For very high-volume logs, phos processes >100k lines/sec. If needed:

```bash
# Sample logs instead of processing all
docker logs mycontainer --tail 1000 | phos -c lighthouse
```
