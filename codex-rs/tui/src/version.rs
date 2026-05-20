/// The current Codex CLI version as embedded at compile time.
pub const CODEX_CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Fork display name shown in the TUI.
pub const FORK_NAME: &str = "Open Codex Unleashed";

/// Fork version shown in the TUI.
pub const FORK_VERSION: &str = CODEX_CLI_VERSION;

/// Fork URL shown in the TUI.
pub const FORK_URL: &str = "https://github.com/lpalbou/codex";

/// Fork author shown in the TUI.
pub const FORK_AUTHOR: &str = "Laurent-Philippe Albou";

/// Upstream project name used in copy.
pub const UPSTREAM_NAME: &str = "OpenAI Codex";

/// Upstream Codex CLI lineage for this fork.
pub const UPSTREAM_VERSION: &str = "0.87.0";
