#!/bin/sh

set -eu

REPO_URL="${CODEX_UNLEASHED_REPO:-https://github.com/lpalbou/codex.git}"
REF="${CODEX_UNLEASHED_REF:-main}"
BIN_NAME="codex-unleashed"

step() {
  printf '==> %s\n' "$1"
}

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "$1 is required to install $BIN_NAME." >&2
    exit 1
  fi
}

require_command git
require_command cargo
require_command mktemp

tmp_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT INT TERM

checkout_dir="$tmp_dir/codex-unleashed"

step "Cloning $REPO_URL ($REF)"
git clone --depth 1 --branch "$REF" "$REPO_URL" "$checkout_dir"

step "Installing $BIN_NAME with cargo"
cargo install --locked --path "$checkout_dir/codex-rs/cli" --bin "$BIN_NAME"

step "$BIN_NAME installed"
step "Run: $BIN_NAME"
step "Config home: \${CODEX_UNLEASHED_HOME:-\$HOME/.codex-unleashed}"
