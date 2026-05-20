# Workflows

This public repository only runs fork-specific workflows:

- `ci.yml`: package/docs validation and one Rust CLI smoke check.
- `docs.yml`: GitHub Pages documentation generation.
- `native-artifacts.yml`: manual native binary artifact build for npm packaging.
- `npm-publish-codex-unleashed.yml`: manual npm staging/publishing for
  `@lpalbou/codex-unleashed`.

The upstream OpenAI Bazel, full Rust matrix, stale issue, labeler, and release
workflows are intentionally not enabled here.
