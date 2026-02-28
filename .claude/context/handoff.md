## 2026-02-28 — Review of: Add benchmark compile-check job to CI

**Verdict:** PASS

**Summary:** Clean, minimal 9-line YAML addition that adds a `bench` job to CI compiling all
criterion benchmarks with `cargo bench --no-run`. The job follows existing CI patterns exactly (same
toolchain, cache actions). All 5 verification criteria pass, `mise run check` passes all 14 hooks,
benchmarks compile locally (7 targets). No scope violations, no quality gate changes.

**Verification:**

- [x] `grep -q 'cargo bench --no-run' .github/workflows/ci.yml` — exits 0
- [x] `grep -q 'Bench (compile check)' .github/workflows/ci.yml` — exits 0
- [x] YAML valid — `yaml.safe_load()` succeeds
- [x] `mise run format` — no unformatted changes (only `iterations.jsonl` touched, expected)
- [x] `cargo bench --no-run` — succeeds locally, compiled 7 bench targets
- [x] `mise run check` — all 14 hooks pass

**Issues found:**

- (none)

**Codex review:** One P2 finding: `cargo bench --no-run` compiles the full workspace (including
binding crates) rather than just `iscc-lib`. Codex suggested scoping with `-p iscc-lib`. Assessment:
the existing Rust CI job already runs `cargo test --workspace` and
`cargo clippy --workspace --all-targets`, so workspace-wide compilation is the established pattern.
Compiling binding bench targets also prevents bitrot there. Not actionable.

**Next:** All 7 bindings at 30/30, CI has 8 jobs (all green), Go docs are clean. The develop branch
is ready for merge to main. Suggested priorities:

1. PR from develop → main — this is a solid milestone (all bindings, clean CI, benchmark check)
2. Publishing infrastructure (OIDC, npm, Maven Central) — human tasks requiring credentials
3. Benchmark speedup documentation (criterion results → docs)

Note: PR #10 may already exist. Check with `gh pr list` before creating a new one.

**Notes:** This was iteration 10, a straightforward CI-only change. The CID loop has completed all
binding work and cleanup. Remaining target gaps are primarily human tasks (publishing credentials,
Maven Central setup) or require running benchmarks and documenting results. The develop → main merge
is the natural next milestone.
