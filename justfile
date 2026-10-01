set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

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