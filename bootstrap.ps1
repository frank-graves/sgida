param (
    [string]$Path = "."
)

$ErrorActionPreference = "Stop"

$resolvedPath = Resolve-Path $Path
if (Test-Path (Join-Path $resolvedPath 'Cargo.toml')) {
    Write-Error "Directory is not empty. Aborting."
    exit 1
}

Write-Host "Creating directory structure..."

function Write-File {
    param([string]$RelativePath, [string]$Content)
    $fullPath = Join-Path $resolvedPath $RelativePath
    $dir = Split-Path $fullPath -Parent
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
    }
    $normalizedContent = $Content -replace "`r`n", "`n"
    [System.IO.File]::WriteAllText($fullPath, $normalizedContent, [System.Text.UTF8Encoding]::new($false))
    Write-Host "Created $RelativePath"
}

$workspaceCargo = @'
[workspace]
members = [
    "crates/sgida-ports",
    "crates/sgida-identity",
    "crates/sgida-isolation",
    "crates/sgida-orchestrator",
    "crates/sgidad",
    "crates/sgida-network",
    "crates/sgida-target",
    "crates/sgida-stealth",
    "crates/sgida-behavior",
    "crates/sgida-email",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT"
rust-version = "1.85"
authors = ["Plutonium-Software"]
repository = "https://github.com/frank-graves/sgida"

[workspace.dependencies]
# async
# tokio = { version = "1", features = ["rt-multi-thread", "macros", "signal", "process", "time", "sync", "net"] }
# async-trait = "0.1"
# tokio-util = "0.7"
# serialization
# serde = { version = "1", features = ["derive"] }
# serde_json = "1"
# api
# axum = "0.7"
# tower = "0.4"
# tower-http = { version = "0.5", features = ["trace", "limit"] }
# storage
# rusqlite = { version = "0.31", features = ["bundled"] }
# dashmap = "5"
# logging
# tracing = "0.1"
# tracing-subscriber = { version = "0.3", features = ["json"] }
# cli
# clap = { version = "4", features = ["derive"] }
# config = "0.14"
# errors
# thiserror = "1"
# anyhow = "1"
# crypto
# rand = "0.8"
# rand_chacha = "0.3"
# hkdf = "0.12"
# sha2 = "0.10"
# secrecy = { version = "0.8", features = ["alloc"] }
# zeroize = "1"
# utilities
# uuid = { version = "1", features = ["v4", "serde"] }
# bytes = "1"
# chrono = { version = "0.4", features = ["serde"] }
# chrono-tz = "0.9"
# petname = "2"
# tempfile = "3"
# nix = { version = "0.28", features = ["sched", "mount", "hostname", "user"] }
# rtnetlink = "0.14"
# testing
# proptest = "1"
# mockall = "0.12"
# criterion = { version = "0.5", features = ["html_reports"] }

[workspace.lints.rust]
unsafe_code = "warn"
missing_debug_implementations = "warn"

[workspace.lints.clippy]
all = "warn"
pedantic = "warn"
nursery = "warn"
unwrap_used = "deny"
expect_used = "warn"
panic = "deny"

[profile.release]
lto = "thin"
codegen-units = 1
panic = "abort"
strip = "symbols"
opt-level = 3
'@
$portsCargo = @'
# sgida-ports: traits only; no tokio, no axum, no I/O
[package]
name = "sgida-ports"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true
authors.workspace = true

[dependencies]
# async-trait.workspace = true
# thiserror.workspace = true
# uuid.workspace = true
# bytes = { workspace = true, optional = true }

[lints]
workspace = true
'@
$identityCargo = @'
# sgida-identity: deterministic profile generation
[package]
name = "sgida-identity"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true
authors.workspace = true

[dependencies]
# rand.workspace = true
# rand_chacha.workspace = true
# serde.workspace = true
# serde_json.workspace = true
# chrono.workspace = true
# chrono-tz.workspace = true
# uuid.workspace = true
# hkdf.workspace = true
# sha2.workspace = true
# secrecy.workspace = true
# zeroize.workspace = true
# petname.workspace = true

[dev-dependencies]
# proptest.workspace = true

[lints]
workspace = true
'@
$isolationCargo = @'
# sgida-isolation: Linux namespace backend
[package]
name = "sgida-isolation"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true
authors.workspace = true

[dependencies]
# nix.workspace = true
# tokio.workspace = true
# tempfile.workspace = true
# uuid.workspace = true
# serde.workspace = true
# rtnetlink = { workspace = true, optional = true }

[lints]
workspace = true
'@
$orchestratorCargo = @'
# sgida-orchestrator: FSM, API, scheduler, state store
[package]
name = "sgida-orchestrator"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true
authors.workspace = true

[dependencies]
# tokio.workspace = true
# axum.workspace = true
# tower.workspace = true
# tower-http.workspace = true
# serde.workspace = true
# serde_json.workspace = true
# thiserror.workspace = true
# anyhow.workspace = true
# uuid.workspace = true
# rusqlite.workspace = true
# tracing.workspace = true
# tracing-subscriber.workspace = true
# dashmap.workspace = true
# tokio-util.workspace = true
# clap.workspace = true
# config.workspace = true

[dev-dependencies]
# mockall.workspace = true

[lints]
workspace = true
'@
$sgidadCargo = @'
# sgidad: the single binary that wires everything
[package]
name = "sgidad"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true
authors.workspace = true

[[bin]]
name = "sgidad"
path = "src/main.rs"

[dependencies]
# tokio.workspace = true
# tracing-subscriber.workspace = true
# clap.workspace = true
# config.workspace = true
# sgida-ports = { path = "../sgida-ports" }
# sgida-identity = { path = "../sgida-identity" }
# sgida-isolation = { path = "../sgida-isolation" }
# sgida-orchestrator = { path = "../sgida-orchestrator" }

[lints]
workspace = true
'@
$networkCargo = @'
# sgida-network: Proxies, DNS, WireGuard
[package]
name = "sgida-network"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true
authors.workspace = true

[dependencies]
# tokio.workspace = true
# hickory-resolver = "0.24"
# tokio-socks = "0.5"
# boringtun = "0.5"
# ipnet = "2"
# reqwest = { version = "0.12", default-features = false, features = ["rustls-tls"] }
# serde.workspace = true

[lints]
workspace = true
'@
$targetCargo = @'
# sgida-target: Chromium control via CDP
[package]
name = "sgida-target"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true
authors.workspace = true

[dependencies]
# chromiumoxide = "0.6"
# tokio.workspace = true
# scraper = "0.20"
# url = "2"
# serde.workspace = true

[lints]
workspace = true
'@
$stealthCargo = @'
# sgida-stealth: JS injection, TLS fingerprint
[package]
name = "sgida-stealth"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true
authors.workspace = true

[dependencies]
# rquickjs = "0.6"
# boring = "4"
# serde.workspace = true

[lints]
workspace = true
'@
$behaviorCargo = @'
# sgida-behavior: Mouse, scroll, typing models
[package]
name = "sgida-behavior"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true
authors.workspace = true

[dependencies]
# nalgebra = "0.33"
# rand_distr = "0.4"
# kurbo = "0.11"
# tokio.workspace = true

[lints]
workspace = true
'@
$emailCargo = @'
# sgida-email: IMAP/SMTP, OTP extraction
[package]
name = "sgida-email"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true
authors.workspace = true

[dependencies]
# async-imap = "0.28"
# lettre = "0.11"
# mailparse = "0.15"
# regex = "1"
# reqwest = { version = "0.12", default-features = false, features = ["rustls-tls"] }
# serde.workspace = true

[lints]
workspace = true
'@
$portsLib = @'
//! Traits for identity provision, isolation, and state storage.
#![forbid(unsafe_code)]
'@
$identityLib = @'
//! Deterministic profile generation and coherence validators.
#![forbid(unsafe_code)]
'@
$isolationLib = @'
//! Linux namespace backend for ephemeral environments.
'@
$orchestratorLib = @'
//! FSM, priority scheduler, REST API, and SQLite state store.
#![forbid(unsafe_code)]
'@
$sgidadMain = @'
//! The single binary that wires concrete types into traits and runs the daemon.
#![forbid(unsafe_code)]

fn main() {}
'@
$networkLib = @'
//! Proxies, DNS, and WireGuard integration.
#![forbid(unsafe_code)]
'@
$targetLib = @'
//! Chromium control via CDP.
#![forbid(unsafe_code)]
'@
$stealthLib = @'
//! JS injection and TLS fingerprint spoofing.
#![forbid(unsafe_code)]
'@
$behaviorLib = @'
//! Mouse, scroll, and typing models.
#![forbid(unsafe_code)]
'@
$emailLib = @'
//! IMAP/SMTP handling and OTP extraction.
#![forbid(unsafe_code)]
'@
$rustfmt = @'
edition = "2024"
max_width = 100
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
'@
$clippy = @'
msrv = "1.85"
'@
$editorconfig = @'
root = true

[*]
charset = utf-8
end_of_line = lf
indent_style = space
indent_size = 4
insert_final_newline = true
trim_trailing_whitespace = true

[*.md]
trim_trailing_whitespace = false
'@
$gitignore = @'
/target
**/*.rs.bk
.DS_Store
*.sublime-workspace
.coverage/
coverage/
lcov.info
*.profraw
benches/baseline.json.tmp
'@
$license = @'
MIT License

Copyright (c) 2026 Plutonium-Software

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
'@
$readme = @'
# SGIDA

A Rust workspace for orchestrating isolated browsing environments in controlled research settings.

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```

## Verification

```bash
just verify-1
just verify-2
just verify-3
just verify-4
just verify-5
```

## License

MIT License. Copyright (c) 2026 Plutonium-Software.
'@
$justfile = @'
verify-1:
    @echo "=== Level 1: fmt + clippy + check ==="
    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo check --workspace

verify-2: verify-1
    @echo "=== Level 2: unit + contract tests ==="
    cargo test --workspace --lib --bins --tests

verify-3: verify-2
    @echo "=== Level 3: coverage ==="
    cargo llvm-cov --workspace

verify-4: verify-3
    @echo "=== Level 4: mutation + deny ==="
    cargo mutants --workspace
    cargo deny check

verify-5: verify-4
    @echo "=== Level 5: fuzz + audit ==="
    cargo fuzz smoke
    cargo audit
'@
$preCommit = @'
#!/bin/sh
set -e

echo "Running pre-commit checks..."

if ! cargo fmt --check; then
    echo "ERROR: cargo fmt failed. Run 'cargo fmt' to fix."
    exit 1
fi

if ! cargo clippy --workspace --all-targets -- -D warnings; then
    echo "ERROR: cargo clippy failed."
    exit 1
fi

echo "Pre-commit checks passed."
'@
$prePush = @'
#!/bin/sh
set -e

echo "Running pre-push checks..."

if ! cargo fmt --check; then
    echo "ERROR: cargo fmt failed."
    exit 1
fi

if ! cargo clippy --workspace --all-targets -- -D warnings; then
    echo "ERROR: cargo clippy failed."
    exit 1
fi

if ! cargo test --workspace --lib --bins --tests; then
    echo "ERROR: cargo test failed."
    exit 1
fi

echo "Pre-push checks passed."
'@
$adr = @'
# 0001 Single Binary Architecture

## Status
Accepted

## Context
The system must orchestrate isolated browsing environments for privacy research.
We need to manage identities, namespaces, and state without external daemon overhead.

## Decision
The entire system is compiled into a single binary (`sgidad`).
All modules communicate via in-process traits defined in `sgida-ports`.
No RPC, no IPC, no subprocesses for the modules themselves.

## Consequences
- No Node.js, Go, or C FFI allowed.
- Playwright is rejected in favor of `chromiumoxide` (Rust-native CDP).
- Frida is rejected in favor of `rquickjs` (embedded JS).
- `utls` is rejected in favor of `boring` (Apache-2.0 TLS).
- Simplifies deployment and eliminates network overhead between components.
'@
Write-File "Cargo.toml" $workspaceCargo
Write-File "rustfmt.toml" $rustfmt
Write-File "clippy.toml" $clippy
Write-File ".editorconfig" $editorconfig
Write-File ".gitignore" $gitignore
Write-File "LICENSE" $license
Write-File "README.md" $readme
Write-File "justfile" $justfile
Write-File ".githooks/pre-commit" $preCommit
Write-File ".githooks/pre-push" $prePush
Write-File "docs/decisions/0001-single-binary.md" $adr

Write-File "crates/sgida-ports/Cargo.toml" $portsCargo
Write-File "crates/sgida-ports/src/lib.rs" $portsLib

Write-File "crates/sgida-identity/Cargo.toml" $identityCargo
Write-File "crates/sgida-identity/src/lib.rs" $identityLib

Write-File "crates/sgida-isolation/Cargo.toml" $isolationCargo
Write-File "crates/sgida-isolation/src/lib.rs" $isolationLib

Write-File "crates/sgida-orchestrator/Cargo.toml" $orchestratorCargo
Write-File "crates/sgida-orchestrator/src/lib.rs" $orchestratorLib

Write-File "crates/sgidad/Cargo.toml" $sgidadCargo
Write-File "crates/sgidad/src/main.rs" $sgidadMain

Write-File "crates/sgida-network/Cargo.toml" $networkCargo
Write-File "crates/sgida-network/src/lib.rs" $networkLib

Write-File "crates/sgida-target/Cargo.toml" $targetCargo
Write-File "crates/sgida-target/src/lib.rs" $targetLib

Write-File "crates/sgida-stealth/Cargo.toml" $stealthCargo
Write-File "crates/sgida-stealth/src/lib.rs" $stealthLib

Write-File "crates/sgida-behavior/Cargo.toml" $behaviorCargo
Write-File "crates/sgida-behavior/src/lib.rs" $behaviorLib

Write-File "crates/sgida-email/Cargo.toml" $emailCargo
Write-File "crates/sgida-email/src/lib.rs" $emailLib

Write-Host "Configuring git hooks..."
Push-Location $resolvedPath
git init -q
git config core.hooksPath .githooks
Pop-Location

Write-Host "Verifying skeleton..."
Push-Location $resolvedPath
cargo check --workspace
cargo fmt --check
Pop-Location

Write-Host "Final tree:"
Get-ChildItem -Path $resolvedPath -Recurse -Name -Force
# Script already lives in the target directory