# Codex Unleashed

Codex Unleashed is a public fork of OpenAI Codex CLI kept close to the
`rust-v0.87.x` CLI lineage while remaining usable with current OpenAI models.

It installs as `codex-unleashed`, not `codex`, and stores its own user state in
`~/.codex-unleashed` by default. This lets it live next to the regular Codex CLI
without sharing auth, config, logs, sessions, caches, or helper files.

## Why this fork exists

This fork preserves a style of Codex use that became harder to keep stable as
later upstream versions moved toward faster iteration, more automatic
orchestration, and broader multi-agent workflows.

Those directions are useful, but they change the operating model. Deep agentic
coding often depends on one long-running agent maintaining judgment, context,
and accountability over a task. Splitting work across agents is not free:
shared context has to be selected or summarized, ownership has to stay visible,
and orchestration has to avoid silently changing model, reasoning effort, or
task state. When those boundaries are unclear, the result can feel less agentic
even if the system is faster.

The degradation this fork is designed around is not simply "newer is worse" or
"multiple agents are bad." It is the practical loss of continuity that shows up
when a coding assistant optimizes for dispatch and turnaround before context
ownership and orchestration semantics are mature enough for deep work.

Codex Unleashed takes the opposite default stance: depth over speed, explicit
configuration over hidden routing, and predictable long-running behavior over
automatic handoff. The public branch is intentionally conservative. It keeps the
0.87-era CLI recognizable, isolates the fork from upstream `codex`, suppresses
upstream update prompts, and updates model discovery so current OpenAI models
remain available.

The fork also keeps model cost and reasoning tradeoffs visible. You can pin a
frontier model for hard work, choose a cheaper model for routine tasks, and set
reasoning effort explicitly instead of treating model choice as an opaque
default.

See [Rationale](./docs/rationale.md) for the longer design note.

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

This public branch does not publish private experimental branches. Private work
stays outside this repository; public releases are cut from the public `main`
branch only.

## Docs

- [Rationale](./docs/rationale.md)
- [Installing and building](./docs/install.md)
- [Documentation index](./docs/README.md)
- [npm publishing](./docs/npm-publishing.md)
- [Configuration](./docs/config.md)
- [Authentication](./docs/authentication.md)
- [Sandboxing](./docs/sandbox.md)
- [Contributing](./docs/contributing.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
