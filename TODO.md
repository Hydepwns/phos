# TODO - phos Roadmap

## Phase 1: Production Ready (Priority: HIGH)

### Testing
- [x] Unit tests for `colors.rs` (parse, to_ansi, brands) - 2 tests
- [x] Unit tests for `rule.rs` (matching, count modes) - 2 tests
- [x] Unit tests for `theme.rs` (builtin themes, resolve) - 2 tests
- [x] Unit tests for `config.rs` (load YAML/JSON, to_rules) - 2 tests
- [x] Unit tests for `colorizer.rs` (colorize, block mode) - 5 tests
- [x] Unit tests for all program categories - 39 tests (59 total)
- [ ] Integration tests for each client (verify rules work on sample logs)
- [ ] CLI integration tests (argument parsing, pipe behavior)
- [ ] Property tests: colorization is idempotent
- [ ] Property tests: output never contains malformed ANSI

### CI/CD
- [ ] GitHub Actions: `cargo test` on push/PR
- [ ] GitHub Actions: `cargo clippy` lint check
- [ ] GitHub Actions: `cargo fmt --check`
- [ ] GitHub Actions: Build release binaries (linux/macos/windows)
- [ ] GitHub Actions: Publish to crates.io on tag
- [ ] Dependabot configuration
- [ ] Code coverage with codecov

### Documentation
- [ ] Rustdoc for all public types
- [ ] Rustdoc examples for common use cases
- [ ] CHANGELOG.md
- [ ] CONTRIBUTING.md
- [ ] LICENSE files (MIT + Apache-2.0)

### Polish
- [ ] Better error messages (config parsing, regex errors)
- [x] `--version` shows git commit hash
- [x] `--help` is comprehensive and pretty
- [x] Shell completions (bash, zsh, fish) - `phos completions <shell>`
- [ ] Man page generation

---

## Phase 2: Distribution (Priority: MEDIUM)

### Package Managers
- [ ] crates.io publish
- [x] Homebrew tap (`brew install Hydepwns/phos/phos`)
- [ ] Homebrew core (submit after community feedback/30+ stars)
- [ ] AUR package (Arch Linux)
- [ ] Nix flake
- [ ] Docker image

### Pre-built Binaries
- [ ] Linux x86_64 (musl for static linking)
- [ ] Linux aarch64
- [ ] macOS x86_64
- [ ] macOS aarch64 (Apple Silicon)
- [ ] Windows x86_64

---

## Phase 3: Integrations (Priority: MEDIUM)

### WASM
- [ ] Feature flag for WASM build
- [ ] wasm-bindgen bindings
- [ ] npm package structure
- [ ] Example HTML page
- [ ] Size optimization (<500KB target)

### Editor Extensions
- [ ] VS Code extension (syntax highlighting for log files)
- [ ] Neovim plugin (lua)

### Ethereum Tooling
- [ ] eth-docker integration guide
- [ ] DAppNode package
- [ ] Sedge integration
- [ ] Stereum integration

---

## Phase 4: Advanced Features (Priority: LOW)

### Structured Output
- [ ] `--extract` mode that outputs JSON
- [ ] Extract: slot, epoch, block number, hashes, addresses
- [ ] Extract: log level counts
- [ ] Extract: error messages

### Analysis
- [x] `--stats` mode for log statistics
- [ ] Error rate over time
- [ ] Peer connection tracking
- [ ] Sync progress parsing

### Real-time Features
- [ ] `--alert` mode with webhook notifications
- [ ] Discord webhook integration
- [ ] Telegram bot
- [ ] PagerDuty integration

### Web Dashboard
- [ ] REST API for colorization
- [ ] WebSocket streaming
- [ ] React frontend
- [ ] Log file upload

---

## Backlog (Ideas)

- [ ] Grafana/Loki plugin for log labeling
- [ ] OpenTelemetry trace ID correlation
- [ ] AI-powered log summarization (via Claude API)
- [ ] Log anomaly detection
- [ ] Multi-language support (i18n)
- [ ] Config hot-reload
- [ ] Terminal capability detection (fallback to 16 colors)
- [ ] Regex syntax highlighting in config files
- [ ] Visual regex debugger
- [ ] Performance profiling mode

---

## Known Issues

- [ ] Block coloring state not reset between files
- [ ] Some regex patterns may be slow on very long lines
- [ ] No validation that theme covers all semantic colors
- [ ] Client auto-detection may have false positives

---

## Notes

### Adding Clients
When new Ethereum clients emerge, add them following CLAUDE.md guidelines.
Candidates to watch:
- Helios (light client)
- Portal Network clients
- New L2-specific clients

### Maintenance
- Regularly test against latest client versions
- Update regex patterns when log formats change
- Keep brand colors synced with clientdiversity.org
