# Codex Unleashed

Codex Unleashed is a fork of OpenAI Codex CLI that keeps the 0.87-era command
line experience while using current OpenAI model discovery.

The public package installs as `codex-unleashed`, not `codex`, and stores
user-scoped state in `~/.codex-unleashed` by default.

## Why this exists

Codex Unleashed preserves a depth-first terminal workflow for long-running
agentic coding. It keeps model and reasoning choices explicit, avoids replacing
upstream `codex`, and treats multi-agent orchestration as something that must
preserve context and accountability rather than silently fragmenting work.
The concern is weaker continuity in hard tasks, not lower raw model capability.

Read the full [Rationale](rationale.md).

## Install

```bash
npm install -g @lpalbou/codex-unleashed
```

Source builds are documented in [Installing](install.md).

## Maintainer workflows

- [npm publishing](npm-publishing.md) documents the exact release flow.
- GitHub Pages is generated from this `docs/` directory by the `docs` workflow.
- Only the public `main` branch is used for CI and publication.
