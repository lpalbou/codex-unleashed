# Codex Unleashed

Codex Unleashed is a local fork of OpenAI Codex CLI kept close to the 0.87-era
CLI while remaining usable with current OpenAI models.

It installs as `codex-unleashed`, not `codex`, and stores its own user state in
`~/.codex-unleashed` by default. This lets it live next to the regular Codex CLI
without sharing auth, config, logs, sessions, caches, or helper files.

## Install

From npm, once a release has been published:

```sh
npm install -g @lpalbou/codex-unleashed
```

macOS/Linux:

```sh
curl -fsSL https://raw.githubusercontent.com/lpalbou/codex-unleashed/main/scripts/install/codex-unleashed.sh | sh
```

Windows PowerShell:

```powershell
irm https://raw.githubusercontent.com/lpalbou/codex-unleashed/main/scripts/install/codex-unleashed.ps1 | iex
```

The installer builds from source, so it requires `git`, `cargo`, and a Rust
toolchain. To install from this checkout instead:

```sh
cd codex-rs
cargo install --locked --path cli --bin codex-unleashed
```

## Run

```sh
codex-unleashed
codex-unleashed login
codex-unleashed --model gpt-5.5
```

Global configuration lives at `~/.codex-unleashed/config.toml`. Override the
home directory with `CODEX_UNLEASHED_HOME=/path/to/home`.

The bundled fallback model catalog prefers `gpt-5.5`, then `gpt-5.4`, then
`gpt-5.4-mini`. Local/source builds use a compatibility client-version floor for
OpenAI `/models` discovery so the fork can continue to see current model
catalog entries.

## Docs

- [Installing and building](./docs/install.md)
- [Documentation index](./docs/README.md)
- [npm publishing](./docs/npm-publishing.md)
- [Configuration](./docs/config.md)
- [Authentication](./docs/authentication.md)
- [Sandboxing](./docs/sandbox.md)
- [Contributing](./docs/contributing.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
