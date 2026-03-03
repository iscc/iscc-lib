# Next Work Package

## Step: Add CI feature matrix testing for issue #16

## Goal

Add feature combination test steps to `.github/workflows/ci.yml` so CI verifies that `iscc-lib`
compiles and passes tests under `--no-default-features`, `--all-features`, and
`--no-default-features --features text-processing`. This is the final sub-task to close issue #16.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/ci.yml`
- **Reference**: `crates/iscc-lib/Cargo.toml` (feature definitions)

## Not In Scope

- Modifying any Rust source code — this is a YAML-only change
- Adding a separate CI job — add steps to the existing `rust` job to avoid duplicating
    checkout/toolchain/cache setup
- Testing feature combinations for binding crates — only `iscc-lib` has feature flags
- Changing the existing `cargo test --workspace` or `cargo clippy --workspace` steps — those stay
    as-is for full workspace coverage

## Implementation Notes

Add new steps to the existing `rust` job. After the current "Run clippy" step, add two clippy
feature-flag checks:

- `cargo clippy -p iscc-lib --no-default-features -- -D warnings`
- `cargo clippy -p iscc-lib --all-features -- -D warnings`

After the current "Run tests" step, add three test steps:

- `cargo test -p iscc-lib --no-default-features`
- `cargo test -p iscc-lib --all-features`
- `cargo test -p iscc-lib --no-default-features --features text-processing`

Keep step names descriptive (e.g., "Test iscc-lib (no default features)"). No `shell: bash` needed
since this job runs on `ubuntu-latest` only. Do NOT use `mise` in CI — call cargo directly.

The existing workspace-level clippy and test steps remain unchanged. The new steps are additive and
target only `iscc-lib` to verify feature-gated code paths.

## Verification

- The YAML is valid: `python -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"`
    exits 0
- `grep -c 'no-default-features' .github/workflows/ci.yml` returns at least 2 (test + clippy)
- `grep -c 'all-features' .github/workflows/ci.yml` returns at least 2 (test + clippy)
- `grep 'text-processing' .github/workflows/ci.yml` shows the text-processing-only test step
- No Rust source files modified: `git diff --name-only` shows only `.github/workflows/ci.yml`

## Done When

All verification criteria pass and the only file modified is `.github/workflows/ci.yml` with five
new steps covering feature combination testing for `iscc-lib`.
