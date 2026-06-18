## 2026-06-18 — Review of: Harden iai-callgrind regression gate against zero-count and disappeared-bench false greens

**Verdict:** PASS

**Summary:** The advance agent closed both Codex-flagged false-green escape hatches in
`scripts/iai_regression.py`: `check_regressions` now fails when any shared benchmark reports a zero
Ir count (independent of `--allow-missing`) and when a baselined benchmark disappears from the run
(downgradable to a warning with the new `--allow-missing` flag). The change is clean, stricter (not
weaker), scope-disciplined, and covered by 11 deterministic synthetic-fixture pytest tests. All
verification passes.

**Verification:**

- [x] `uv run pytest tests/test_iai_regression.py -q` — 11 passed (>= 6 required, covers cases a–f +
    boundary/roundtrip/parse_ir)
- [x] `uv run pytest -q` full suite — 297 passed, 1 warning (pre-existing benchmark legend,
    unrelated)
- [x] `uv run ruff check` — clean on both files
- [x] `uv run ruff format --check` — 2 files already formatted
- [x] `uv run ty check scripts/iai_regression.py` — clean
- [x] `cargo test -p iscc-lib` — 22 unit + doctest pass (script-only change, core unaffected)
- [x] `mise run check` — all 15 pre-commit hooks pass
- [x] Manual sanity — matching synthetic run (16 baseline benches) exits 0; one `.out` tampered to
    `summary: 0` exits 1 with `FAIL: 1 benchmark(s) collected a zero Ir count`; removing a bench dir
    exits 1 by default and 0 with `--allow-missing`

**Issues found:**

- (none) — diff touches only `scripts/iai_regression.py`, the new `tests/test_iai_regression.py`,
    and context/memory files. No CRAP/cargo-deny/CI-YAML/baseline/semver changes (all correctly out
    of scope). No gate circumvention across the 4 unpushed commits — the `continue-on-error` grep
    hits are all prose in context/memory files, and the new failure paths make the gate stricter.

**Codex review:** Clean — "The commit correctly tightens the iai regression gate for shared
zero-count results and missing baselined benchmarks, adds the intended allow-missing escape hatch,
and covers the behavior with synthetic tests." No actionable findings.

**Issues swept this cycle (deleted from issues.md):**

- "Add `iai-callgrind` performance-regression CI gate" `normal` [human] (issue #3) — functionally
    complete and CI-verified GREEN on `a5ce73c` (per update-state / state.md), specs already marked
    `[x]` (rust-core.md 374–376, ci-cd.md 424). Deleted; no further spec edit needed.
- "Harden iai-callgrind regression gate against false-green edge cases" `normal` [review] — resolved
    by this iteration. Deleted.

**Next:** No fully-autonomous `normal` work package remains. The two open `normal` [review] issues
are both **HUMAN REVIEW REQUESTED spec amendments** — define-next must NOT start either
autonomously:

1. **CRAP gate `--fail-above 30`** — add an absolute threshold so new uncovered high-CRAP functions
    fail (current regression-only gate lets them through). Requires amending `ci-cd.md`; confirm
    `cargo-crap 0.2.2` accepts `--fail-above` + `--fail-regression` together first.
2. **Wire up `cargo deny`/`cargo audit` supply-chain gate** — requirement lives only in `notes/07`,
    not the CID specs; needs a spec amendment plus `deny.toml` + CI job + `mise run audit` task.

The remaining `low` [human] issues (v1.0.0 release cut, docs logos) are human-directed and out of
CID scope. **The loop is at a natural human-handoff point**: if define-next finds no autonomous
work, the next no-op advance should let review signal IDLE. Do not flip the `Semver` gate to
enforcing or start v1.0.0 prep autonomously.

**Notes:**

- The hardened gate is correct by construction: `zero_benches` fails independently of
    `--allow-missing`; `only_baseline` fails unless `--allow-missing`; `only_run` (new bench) still
    only warns. The committed `.iai-baseline.json` (16 Ir entries) can only shrink via a deliberate
    `--update` refresh in a reviewed commit.
- Mid-flush race (prior handoff): running `--check` in the same shell command immediately after
    `cargo bench` can match fewer benches before the flush completes. In CI `--check` is a separate
    step after the bench step fully finishes, so the now-failing disappeared-bench path is safe.
    `--allow-missing` is the deliberate escape hatch for any future transient CI flake — do not
    weaken the default.
- State to confirm next cycle: CI `Perf` job stays GREEN on the pushed tip after this batch (the
    `Check perf regression` step is unchanged in behavior for the all-present, within-tolerance case
    it sees in CI; the new failure paths only trigger on zero/missing).
