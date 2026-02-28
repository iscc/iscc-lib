# Next Work Package

## Step: Add benchmark compile-check job to CI

## Goal

Add a `bench` job to `ci.yml` that compiles all criterion benchmarks (`cargo bench --no-run`) on
every push and PR, preventing benchmark bitrot. This closes the state.md gap "CI does not run
benchmarks automatically" and ensures benchmarks stay buildable as the codebase evolves.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/ci.yml` — add a new `bench` job
- **Reference**: `crates/iscc-lib/benches/benchmarks.rs` (to understand what gets compiled),
    `.github/workflows/ci.yml` (existing job structure to follow)

## Not In Scope

- Running benchmarks and uploading results as CI artifacts (future step — adds complexity with
    criterion JSON output, artifact storage, comparison tooling)
- Publishing benchmark speedup factors in documentation (separate target gap)
- pytest-benchmark comparison infrastructure (Python-side benchmarks are a separate concern)
- Updating PR #10 title/description (human task — PR is already passing and mergeable)
- Any publishing infrastructure changes (OIDC, npm, Maven Central are human tasks)

## Implementation Notes

Follow the existing CI job pattern — same toolchain setup, same cache action. The job structure:

```yaml
bench:
  name: Bench (compile check)
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - uses: Swatinem/rust-cache@v2
    - name: Compile benchmarks
      run: cargo bench --no-run
```

Key points:

- `cargo bench --no-run` compiles all benchmark targets without executing them. This catches
    compilation errors (broken imports, removed APIs, type mismatches) without the noise and
    variability of running benchmarks on shared CI runners.
- Use `Swatinem/rust-cache@v2` for caching (consistent with all other Rust-based jobs).
- Place the job after the existing `go` job to keep logical ordering (supplementary checks after
    binding jobs).
- Do NOT add `--workspace` — `cargo bench` already discovers workspace bench targets.
- The benchmark file (`crates/iscc-lib/benches/benchmarks.rs`) imports 10 symbols from `iscc_lib`
    and defines 10 benchmark functions. A compile-only check validates all imports and types.

## Verification

- `grep -q 'cargo bench --no-run' .github/workflows/ci.yml` exits 0
- `grep -q 'Bench (compile check)' .github/workflows/ci.yml` exits 0
- The CI YAML is valid: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"`
    exits 0
- `mise run format` produces no unformatted changes (run format, then `git diff` shows no changes)
- `cargo bench --no-run` succeeds locally (exits 0)

## Done When

All verification criteria pass: the `bench` job is present in `ci.yml`, the YAML is valid and
formatted, benchmarks compile locally, and the file is ready to trigger the new job on next push.
