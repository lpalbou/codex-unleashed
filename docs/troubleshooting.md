# Troubleshooting

## `codex-unleashed` is not found

Check that the npm global binary directory or Cargo install directory is on
`PATH`.

```bash
npm bin -g
cargo install --list | grep codex
```

## npm install reports a missing optional dependency

Reinstall the package from the public registry:

```bash
npm install -g @lpalbou/codex-unleashed
```

The meta package depends on platform-specific payload versions of the same npm
package. If a payload version is missing from npm, run the maintainer workflow
documented in [npm publishing](npm-publishing.md).

## Configuration appears to be shared with upstream Codex

Codex Unleashed should use `~/.codex-unleashed` by default. Check whether an
override is set:

```bash
printenv CODEX_UNLEASHED_HOME
printenv CODEX_HOME
```

Prefer `CODEX_UNLEASHED_HOME` for this fork. `CODEX_HOME` is accepted only as a
legacy override.
