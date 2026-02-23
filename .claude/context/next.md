# Next Work Package

## Step: Add WASM CI job to workflow

## Goal

Add a WASM job to `.github/workflows/ci.yml` that runs `wasm-pack test --node` against the
`crates/iscc-wasm` crate. This is the last piece needed to consider the WASM bindings target
criterion complete — all 4 binding crates will then be under CI.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/ci.yml` — add a `wasm` job
- **Reference**: existing `rust`, `python`, `nodejs` jobs in `ci.yml` for structural patterns;
    `crates/iscc-wasm/Cargo.toml` for crate config; learnings about CI tooling

## Implementation Notes

Follow the existing job structure pattern (checkout → rust-toolchain → rust-cache → tool setup →
build → test). Specifically:

1. Add a new job named `wasm` with display name `WASM (wasm-pack test)`.
2. Use `ubuntu-latest` runner (consistent with other jobs).
3. Steps:
    - `actions/checkout@v4`
    - `dtolnay/rust-toolchain@stable`
    - `Swatinem/rust-cache@v2`
    - Install wasm-pack via `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
        (this is the official installer and avoids needing a dedicated action). Alternatively, use
        `cargo install wasm-pack` but the curl installer is faster.
    - Run tests: `wasm-pack test --node crates/iscc-wasm`
4. No `needs:` dependency on other jobs — all 4 jobs should run in parallel.
5. Do NOT use `mise` in CI (per learnings).

The handoff notes that wasm-pack 0.13.1 is locally installed and 0.14.0 is available. The curl
installer will pull the latest stable version, which is fine for CI.

## Verification

- `ci.yml` has a `wasm` job that installs wasm-pack and runs
    `wasm-pack test --node crates/iscc-wasm`
- YAML is valid (no syntax errors)
- Job follows the same structural pattern as existing jobs (checkout, rust-toolchain, rust-cache)
- The 4 jobs (rust, python, nodejs, wasm) all run independently (no `needs:` dependencies)
- `cargo clippy --workspace --all-targets -- -D warnings` still passes (no Rust changes)
- `cargo fmt --all --check` still passes (no Rust changes)

## Done When

The advance agent is done when `ci.yml` contains a valid WASM job following the established CI
patterns, and all existing quality gates (`cargo clippy`, `cargo fmt`) still pass.
