## 2026-02-28 — Add benchmark compile-check job to CI

**Done:** Added a `bench` job to `.github/workflows/ci.yml` that compiles all criterion benchmarks
with `cargo bench --no-run` on every push and PR. The job follows the existing CI pattern (checkout,
rust-toolchain, rust-cache) and is placed after the `go` job.

**Files changed:**

- `.github/workflows/ci.yml`: Added `bench` job with 4 steps (checkout, toolchain, cache, compile
    benchmarks)

**Verification:**

- [x] `grep -q 'cargo bench --no-run' .github/workflows/ci.yml` — exits 0
- [x] `grep -q 'Bench (compile check)' .github/workflows/ci.yml` — exits 0
- [x] YAML validation via `python3 -c "import yaml; yaml.safe_load(...)"` — valid
- [x] `mise run format` — no unformatted changes
- [x] `cargo bench --no-run` — succeeds locally (compiled 7 bench targets)
- [x] `mise run check` — all 14 hooks pass

**Next:** All CI gaps are now closed for the current codebase. Suggested next priorities from
target.md:

1. PR from develop → main — Go pure rewrite + CI benchmark check is a solid milestone
2. Publishing infrastructure (OIDC, npm, Maven Central) — human tasks requiring credentials
3. Benchmark speedup documentation (criterion results → docs)

**Notes:** No surprises. This was a minimal, clean addition. The `cargo bench --no-run` compiles all
7 workspace bench targets (iscc-lib benchmarks.rs + lib.rs benches for each binding crate). No tests
needed — this is a CI-only change verified by YAML validation and local compilation.
