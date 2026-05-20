## Installing & building Codex Unleashed

### System requirements

| Requirement       | Details                                                         |
| ----------------- | --------------------------------------------------------------- |
| Operating systems | macOS 12+, Ubuntu 20.04+/Debian 10+, or Windows 11 **via WSL2** |
| Git               | 2.23+ for installer and built-in PR helpers                     |
| Rust toolchain    | Stable Rust with `cargo`                                        |
| RAM               | 4-GB minimum (8-GB recommended)                                 |

### Install from npm

From npm, once a release has been published:

```bash
npm install -g @lpalbou/codex-unleashed
```

The npm package installs the `codex-unleashed` command and keeps Codex
Unleashed state in `~/.codex-unleashed`, so it can live beside upstream
`codex`.

### Install from source

macOS/Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/lpalbou/codex/main/scripts/install/codex-unleashed.sh | sh
```

Windows PowerShell:

```powershell
irm https://raw.githubusercontent.com/lpalbou/codex/main/scripts/install/codex-unleashed.ps1 | iex
```

The installer clones this repository and runs:

```bash
cargo install --locked --path codex-rs/cli --bin codex-unleashed
```

It installs only `codex-unleashed`. It does not overwrite an existing `codex`
binary from the upstream OpenAI CLI.

### Build from source

```bash
# Clone the repository and navigate to the root of the Cargo workspace.
git clone https://github.com/lpalbou/codex.git
cd codex/codex-rs

# Install the Rust toolchain, if necessary.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustup component add rustfmt
rustup component add clippy
# Install helper tools used by the workspace justfile:
cargo install just
# Optional: install nextest for the `just test` helper
cargo install --locked cargo-nextest

# Build Codex Unleashed.
cargo build

# Source and local builds keep their package version, but OpenAI model catalog
# discovery uses a compatibility version floor so this 0.87-era fork can see
# current models.

# Launch the TUI with a sample prompt.
cargo run --bin codex-unleashed -- "explain this codebase to me"

# Install from this checkout.
cargo install --locked --path cli --bin codex-unleashed

# After making changes, use the root justfile helpers (they default to codex-rs):
just fmt
just fix -p <crate-you-touched>

# Run the relevant tests (project-specific is fastest), for example:
cargo test -p codex-tui
# If you have cargo-nextest installed, `just test` runs the test suite via nextest:
just test
# Avoid `--all-features` for routine local runs because it increases build
# time and `target/` disk usage by compiling additional feature combinations.
# If you specifically want full feature coverage, use:
cargo test --all-features
```

## Tracing / verbose logging

Codex is written in Rust, so it honors the `RUST_LOG` environment variable to configure its logging behavior.

The TUI defaults to `RUST_LOG=codex_core=info,codex_tui=info,codex_rmcp_client=info` and log messages are written to `~/.codex-unleashed/log/codex-tui.log` by default. For a single run, you can override the log directory with `-c log_dir=...` (for example, `-c log_dir=./.codex-log`).

```bash
tail -F ~/.codex-unleashed/log/codex-tui.log
```

By comparison, the non-interactive mode (`codex-unleashed exec`) defaults to `RUST_LOG=error`, but messages are printed inline, so there is no need to monitor a separate file.

See the Rust documentation on [`RUST_LOG`](https://docs.rs/env_logger/latest/env_logger/#enabling-logging) for more information on the configuration options.

See [npm publishing](./npm-publishing.md) for the maintainer release workflow.
