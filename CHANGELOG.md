# Changelog

## Unreleased

This is the first fork-specific changelog entry for Codex Unleashed. The
original `main` branch only linked to upstream OpenAI Codex release notes.

### Added

- Add a separate `codex-unleashed` CLI binary target so this fork can be
  installed beside upstream `codex`.
- Add source-build installer scripts for macOS/Linux and Windows PowerShell:
  `scripts/install/codex-unleashed.sh` and
  `scripts/install/codex-unleashed.ps1`.
- Add `CODEX_UNLEASHED_HOME` as the primary user-state override.
- Add a main-only npm publishing workflow for
  `@lpalbou/codex-unleashed`, using npm trusted publishing/OIDC and an
  explicit confirmation gate.
- Add npm publishing maintainer documentation for the `@lpalbou` scoped
  package.
- Add fork branding constants so the TUI header identifies the project as
  `Open Codex Unleashed`.
- Add public rationale documentation explaining the fork's depth-first
  0.87-lineage stance, side-by-side install policy, and explicit model/reasoning
  tradeoffs.

### Changed

- Move the default user home from `~/.codex` to `~/.codex-unleashed`, keeping
  auth, config, logs, sessions, caches, sandbox helpers, themes, skills, MCP
  credentials, and history separate from upstream Codex.
- Keep `CODEX_HOME` as a legacy fallback only when `CODEX_UNLEASHED_HOME` is
  unset, for explicit migrations and existing tests.
- Update CLI help, login hints, resume hints, MCP hints, TUI status text, update
  prompts, npm package metadata, and the Node wrapper to use
  `codex-unleashed`.
- Publish the npm package under the `@lpalbou` scope while keeping the installed
  command name `codex-unleashed`.
- Keep inherited upstream npm publishing disabled for this fork; the dedicated
  Codex Unleashed workflow is the only npm publication path here.
- Set the fork package/workspace version to `0.1.0`.
- Suppress upstream Codex update prompts, update banners, startup tooltips, and
  remote announcement prewarming for Codex Unleashed.
- Update the root README, install docs, config docs, schema docs, generated
  `config.schema.json`, and related TUI snapshots to match the fork identity
  and isolated home directory.
- Refresh the bundled fallback model catalog to prefer `gpt-5.5`, `gpt-5.4`,
  and `gpt-5.4-mini`, and route deprecated model migration prompts to
  `gpt-5.5`.
- Keep 0.87-era source and local builds compatible with current OpenAI model
  discovery by sending a `/models` client-version compatibility floor while
  preserving the user-visible package version.
- Build Linux npm native payloads for the GNU Linux targets used by the Rust
  dependency graph. The first npm release publishes Linux x64, macOS x64/ARM64,
  and Windows x64/ARM64 payloads; Linux ARM64 is deferred until its native build
  is reliable enough for release automation.

### Compatibility Notes

- A regular upstream Codex install continues to use `codex` and `~/.codex`.
  Codex Unleashed uses `codex-unleashed` and `~/.codex-unleashed` by default.
- The old `codex` Rust binary target is still present for compatibility with
  existing local tooling and tests, but docs and install paths now point users
  to `codex-unleashed`.

Older upstream release notes are available on the
[releases page](https://github.com/openai/codex/releases).
