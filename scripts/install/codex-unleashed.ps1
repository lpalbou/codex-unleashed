param(
    [string]$RepoUrl = $(if ($env:CODEX_UNLEASHED_REPO) { $env:CODEX_UNLEASHED_REPO } else { "https://github.com/lpalbou/codex-unleashed.git" }),
    [string]$Ref = $(if ($env:CODEX_UNLEASHED_REF) { $env:CODEX_UNLEASHED_REF } else { "main" })
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

$BinName = "codex-unleashed"

function Write-Step {
    param([string]$Message)
    Write-Host "==> $Message"
}

function Require-Command {
    param([string]$Name)
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "$Name is required to install $BinName."
    }
}

Require-Command git
Require-Command cargo

$TempDir = Join-Path ([System.IO.Path]::GetTempPath()) ("codex-unleashed-install-" + [System.Guid]::NewGuid().ToString("N"))
$CheckoutDir = Join-Path $TempDir "codex-unleashed"

try {
    New-Item -ItemType Directory -Force -Path $TempDir | Out-Null

    Write-Step "Cloning $RepoUrl ($Ref)"
    git clone --depth 1 --branch $Ref $RepoUrl $CheckoutDir
    if ($LASTEXITCODE -ne 0) {
        throw "git clone failed"
    }

    Write-Step "Installing $BinName with cargo"
    cargo install --locked --path (Join-Path $CheckoutDir "codex-rs\cli") --bin $BinName
    if ($LASTEXITCODE -ne 0) {
        throw "cargo install failed"
    }

    Write-Step "$BinName installed"
    Write-Step "Run: $BinName"
    Write-Step 'Config home: $env:CODEX_UNLEASHED_HOME or $HOME\.codex-unleashed'
} finally {
    Remove-Item -Recurse -Force $TempDir -ErrorAction SilentlyContinue
}
