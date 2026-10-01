PHASE 1 — Workspace and tooling.

CONTEXT
Read the 5 files loaded in this project. We are at v1 kickoff, phase 1.
No product code exists yet. This phase creates the repository skeleton.

OBJECTIVE
Generate a single PowerShell script (`bootstrap.ps1`) that, when run in an
empty directory, produces a complete Rust workspace ready to build, with
local verification tooling wired up.

SCOPE — the script must create exactly these files and directories:

ROOT/
  Cargo.toml
  rustfmt.toml
  clippy.toml
  .editorconfig
  .gitignore
  LICENSE
  README.md
  justfile
  bootstrap.ps1  (the script itself, for reproducibility)
  .githooks/
    pre-commit
    pre-push
  docs/
    decisions/
      0001-single-binary.md
  crates/
    sgida-ports/           Cargo.toml + src/lib.rs
    sgida-identity/        Cargo.toml + src/lib.rs
    sgida-isolation/       Cargo.toml + src/lib.rs
    sgida-orchestrator/    Cargo.toml + src/lib.rs
    sgidad/                Cargo.toml + src/main.rs
    sgida-network/         Cargo.toml + src/lib.rs  (placeholder)
    sgida-target/          Cargo.toml + src/lib.rs  (placeholder)
    sgida-stealth/         Cargo.toml + src/lib.rs  (placeholder)
    sgida-behavior/        Cargo.toml + src/lib.rs  (placeholder)
    sgida-email/           Cargo.toml + src/lib.rs  (placeholder)

CONTENT SPECIFICATIONS

1. Workspace Cargo.toml:
   - resolver = "2", members = all 10 crates
   - [workspace.package] with version, edition = "2024", license = "MIT",
     rust-version = "1.85", authors = ["Plutonium-Software"],
     repository = "https://github.com/frank-graves/sgida"
   - [workspace.dependencies] with EVERY dependency the project will need
     across all phases, but ALL LINES COMMENTED OUT with a `#` prefix.
     Group by category (async, serialization, api, storage, logging, cli,
     errors, crypto, testing). The user will uncomment per phase.
   - [workspace.lints.rust] with unsafe_code = "warn",
     missing_debug_implementations = "warn"
   - [workspace.lints.clippy] with all, pedantic, nursery = "warn";
     unwrap_used = "deny"; expect_used = "warn"; panic = "deny"
   - [profile.release] with lto = "thin", codegen-units = 1,
     panic = "abort", strip = "symbols", opt-level = 3

2. Each crate Cargo.toml:
   - Inherits from workspace: version, edition, license, rust-version, authors
   - Uses workspace lints: lints.workspace = true
   - `sgidad` is a [[bin]], the rest are [lib]
   - Declares its dependencies COMMENTED OUT (same rationale as workspace)
   - Each crate's Cargo.toml has a comment header with one line explaining
     that crate's role (from 04_context.md)

3. Every src/lib.rs or src/main.rs:
   - A single doc comment at the top describing the crate's purpose
   - `#![forbid(unsafe_code)]` for the pure-Rust crates (sgida-ports,
     sgida-identity, sgida-orchestrator, sgidad) — NOT for sgida-isolation
     which needs unsafe for syscalls
   - Empty otherwise. No stubs. No TODOs.

4. rustfmt.toml:
   edition = "2024", max_width = 100, imports_granularity = "Crate",
   group_imports = "StdExternalCrate"

5. clippy.toml:
   msrv = "1.85"

6. .editorconfig:
   root = true, [*] with charset utf-8, end_of_line lf, indent_style space,
   indent_size 4, insert_final_newline true, trim_trailing_whitespace true,
   and [*.md] trim_trailing_whitespace = false

7. .gitignore:
   /target, **/*.rs.bk, Cargo.lock (NO — we commit Cargo.lock; do not ignore),
   .DS_Store, *.sublime-workspace, .coverage/, coverage/, lcov.info,
   *.profraw, benches/baseline.json.tmp

8. LICENSE:
   Standard MIT license text, Copyright (c) 2026 Plutonium-Software

9. README.md:
   Project name, one-paragraph description pointing to the 5 files,
   build instructions (`cargo build`), test instructions (`cargo test`),
   verification instructions (`just verify-1` .. `just verify-5`),
   license footer.

10. justfile:
    Recipes: verify-1 (fmt + clippy + cargo check), verify-2 (verify-1 +
    cargo test), verify-3 (verify-2 + cargo llvm-cov), verify-4 (verify-3
    + cargo mutants + cargo deny), verify-5 (verify-4 + cargo fuzz smoke +
    cargo audit). Each recipe prints a banner before running. Use `@` on
    lines to suppress command echo where noisy.

11. .githooks/pre-commit:
    POSIX sh. Runs `cargo fmt --check` and `cargo clippy --workspace
    --all-targets -- -D warnings`. Exits non-zero on failure. Prints a
    clear message when a check fails. Should run in under 60 seconds.

12. .githooks/pre-push:
    POSIX sh. Runs the pre-commit checks, then `cargo test --workspace
    --lib --bins --tests`. Should run in under 5 minutes.

13. docs/decisions/0001-single-binary.md:
    ADR format: Title, Status (Accepted), Context, Decision, Consequences.
    Decision: single-binary for the whole system, traits in-process, no RPC.
    Consequences: no Node/Go/C FFI; Playwright replaced by chromiumoxide;
    Frida replaced by rquickjs; utls replaced by boring. Reference
    04_context.md for the full decision log.

14. The bootstrap.ps1 script itself must:
    - Take an optional -Path parameter (default: current directory)
    - Refuse to run if the directory is not empty (safety)
    - Print each action as it happens
    - At the end, run `cargo check --workspace` and `cargo fmt --check` to
      verify the skeleton compiles
    - Print the final tree of what was created
    - Be idempotent-safe: if interrupted, safe to delete and rerun
    - Use only PowerShell built-ins (New-Item, Set-Content, Out-File,
      Test-Path). No external tools beyond `git`, `cargo`, `rustc`.

CONSTRAINTS
- No emojis anywhere. No decorative ASCII art. Plain text only.
- Comments in English.
- No placeholders like "TODO: fill this". Everything must be complete.
- Do not invent crates not mentioned in 03_stack.md.
- If something is ambiguous, ASK. Do not guess.

OUTPUT FORMAT
Return ONLY the complete content of `bootstrap.ps1`. No preamble, no
explanation, no closing remarks. The user will save it as `bootstrap.ps1`
and run it.

ACCEPTANCE CRITERIA
The script is correct when, after running it in an empty directory:
1. `cargo check --workspace` succeeds.
2. `cargo fmt --check` succeeds.
3. `git status` shows all created files as untracked.
4. `git config core.hooksPath` returns `.githooks`.
5. `just verify-1` runs and passes.
