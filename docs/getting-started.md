# Getting started with Codex Unleashed

Codex Unleashed installs as `codex-unleashed` and stores its own state under
`~/.codex-unleashed`, so it can live beside upstream `codex`.

```bash
npm install -g @lpalbou/codex-unleashed
codex-unleashed login
codex-unleashed
```

The fork keeps the 0.87-era terminal workflow while updating model discovery for
current OpenAI models. Start with the default model catalog, or pin a model for
a specific run:

```bash
codex-unleashed --model gpt-5.4-mini
```

For the design stance behind the fork, see [Rationale](rationale.md). For source
builds and side-by-side configuration details, see [Installing](install.md) and
[Configuration](config.md).
