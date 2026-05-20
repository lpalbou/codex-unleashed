# Rationale

Codex Unleashed exists to preserve a specific Codex CLI operating model:
long-running, depth-first agentic work where the same agent can maintain context,
judgment, and accountability over a task.

## Design stance

The fork is based on the `rust-v0.87.x` Codex CLI lineage because that era is a
useful base for focused terminal work. Later upstream versions have increasingly
emphasized faster iteration, richer orchestration, and more multi-agent
coordination. Those are legitimate goals, but they introduce tradeoffs that are
especially visible in deep coding sessions.

Multi-agent orchestration is difficult because shared context is difficult. A
system has to decide what each agent sees, what is summarized, who owns which
part of the work, how to recover when a child task stalls, and how to avoid
silently changing model, effort, permissions, or working directory. If those
decisions are hidden or too automatic, the user can get speed at the cost of
continuity. The tool may do more things in parallel while feeling less like one
coherent agent pursuing one goal.

That is the observed agency degradation this fork is meant to resist: not lower
raw model capability, but weaker continuity. If the system is too eager to
dispatch, retry, summarize, or hand off before the context contract is clear, it
can lose the thread of a hard task while still appearing busy and responsive.

Codex Unleashed therefore favors:

- Long-running continuity over short automatic retries.
- Explicit model and reasoning choices over hidden defaults.
- Side-by-side installation over replacing upstream `codex`.
- User-visible state over surprise update banners or remote startup tips.
- Conservative 0.87-native backports over wholesale upstream rewrites.

This is not a claim that upstream Codex should avoid orchestration. It is a
different product stance: keep a dependable single-agent terminal workflow
available while newer Codex lines explore broader coordination.

## Model and reasoning policy

The 0.87-era CLI no longer works well if it cannot discover current OpenAI
models. Codex Unleashed keeps the user-visible fork version separate from the
client-version compatibility floor used for `/models` discovery, so source and
local builds can still receive current catalog entries.

The bundled fallback catalog prefers current frontier models first, while still
letting users pin cheaper or faster models for routine work:

```toml
model = "gpt-5.4-mini"
model_reasoning_effort = "medium"
```

For one-off runs:

```bash
codex-unleashed --model gpt-5.4-mini
```

The point is not to force one default. The point is to make capability, latency,
and cost tradeoffs visible.

## Public scope

This public repository is the installable Codex Unleashed base. It includes the
fork identity, side-by-side command and state directory, current model discovery,
documentation, and public release automation for `@lpalbou/codex-unleashed`.

Private experimental branches are not published here. Work on memory graphs,
local-model routing experiments, and deeper orchestration research remains
separate unless it is deliberately reviewed, documented, and promoted to public
`main`.

## Practical guarantees

Codex Unleashed is intended to:

- Install as `codex-unleashed`, leaving any upstream `codex` command alone.
- Store user state in `~/.codex-unleashed` by default.
- Suppress upstream Codex update prompts and startup tips that do not apply to
  this fork.
- Keep public CI and publishing scoped to the public `main` branch.
- Publish npm releases only under `@lpalbou/codex-unleashed`.
