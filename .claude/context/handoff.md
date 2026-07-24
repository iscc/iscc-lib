# Handoff

## 2026-07-24 — Refresh `.crap-baseline.json` for the iter-121 `iscc_decode` change (fix RED CI)

**Done:** Regenerated the committed CRAP regression baseline via the documented path
(`mise run coverage` → `cargo crap --lcov lcov.info --format json --output .crap-baseline.json`,
cargo-llvm-cov 0.8.7 + cargo-crap 0.2.2, same versions CI installs). The `iscc_decode` entry now
records the legitimate complexity from the iter-121 trailing-byte guard; no source files changed.

**Files changed:**

- `.crap-baseline.json`: `iscc_decode` entry updated — `cyclomatic 4.0 → 5.0`,
    `coverage 80.95 → 85.19`, `crap 4.110571… → 5.081288…`, `line 225 → 226` (entry also moved
    position in the file because output is sorted by CRAP score descending). All other lib.rs
    entries got pure `line:` shifts of +7 (matching the 7 lines iter 121 added); NO other entry's
    `cyclomatic`/`coverage`/`crap` changed — verified by reading the full diff.

**Verification:**

- `mise run coverage` produced a fresh `lcov.info`, exit 0 (lcov.info is gitignored, not staged)
- Enforcing gate command exactly as CI runs it:
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above` →
    exit 0, summary `↑ 0 regressed  ↓ 0 improved  ★ 0 new  ↔ 0 moved  · 97 unchanged  — 0 removed`,
    nothing above the 30.0 cap
- `cargo test -p iscc-lib` — 321 passed (270 unit + 28 + 22 integration + 1 doctest), 0 failed
- `cargo clippy -p iscc-lib --all-features --all-targets -- -D warnings` — exit 0 (only the
    pre-existing transitive `proc-macro-error2` future-incompat note, unrelated)
- `mise run check` — all 15 pre-commit hooks Passed, exit 0

**Next:** CI should go green on this push (the `Coverage + CRAP` job was the only red job per
state.md). Then the queue per prior review: #49 aarch64 Python wheels
(`.claude/plans/restore-linux-aarch64-python-wheels.md`) and the project-wide dependency
review/refresh — both `normal` `[human]`, both unblocked once this lands green.

**Notes:**

- Tooling surprise: next.md said cargo-crap 0.2.2 / cargo-llvm-cov 0.8.7 are installed in the
    devcontainer — neither was on PATH in this session. Installed cargo-llvm-cov 0.8.7 via
    `cargo binstall`; the binstall'd cargo-crap binary failed with `GLIBC_2.39 not found` (container
    glibc is older), so cargo-crap 0.2.2 was built from source with `cargo install` (~1.5 min). This
    is environment-only — the committed baseline was generated locally as required (not pulled from
    a CI artifact).
- Residual risk (flagged in next.md's fallback note): if devcontainer coverage drifts from CI
    coverage, the gate could still trip in CI despite the local exit 0. The local run showed
    `97 unchanged / 0 regressed`, so drift is unlikely, but the reviewer should confirm the CI
    `Coverage + CRAP` job on this push.
- Guard gap worth remembering (also noted in next.md): the CRAP regression gate is CI-only — not in
    `mise run check`. Any future change adding a branch/loop to a covered function must refresh
    `.crap-baseline.json` in the same step. Recorded in agent memory (ci-gates.md).
- `.claude/context/iterations.jsonl` shows as modified in the working tree (runner-managed); left
    unstaged per protocol.
