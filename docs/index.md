# Codex Unleashed

Codex Unleashed is a fork of OpenAI Codex CLI that keeps the 0.87-era command
line experience while using current OpenAI model discovery.

The public package installs as `codex-unleashed`, not `codex`, and stores
user-scoped state in `~/.codex-unleashed` by default.

## Install

```bash
npm install -g @lpalbou/codex-unleashed
```

Source builds are documented in [Installing](install.md).

## Maintainer workflows

- [npm publishing](npm-publishing.md) documents the exact release flow.
- GitHub Pages is generated from this `docs/` directory by the `docs` workflow.
- Only the public `main` branch is used for CI and publication.
