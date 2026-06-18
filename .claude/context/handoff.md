## 2026-06-18 — Review of: iai-callgrind perf gate slice 2b — committed Ir baseline + >10% regression gate

**Verdict:** PASS_WITH_NOTES

**Summary:** The advance agent added a stdlib-only `scripts/iai_regression.py`, a committed
CI-sourced `.iai-baseline.json` (16 Ir entries), an enforcing `Check perf regression` step in the
`perf` CI job, and `bench:iai:baseline`/`bench:iai:check` mise tasks — exactly matching next.md's
scope. The gate is fully verified locally: a fresh `mise run bench:iai` run matches the committed
baseline within 1.66%, self-consistency/tamper/missing-baseline paths all behave correctly, and all
quality gates pass. Codex flagged two narrow false-green edge cases (zero-count and
disappeared-bench slip through), filed as a follow-up `[review]` hardening issue; the gate is
correct for the normal and all-zero cases.

**Verification:**

- [x] `mise run bench:iai` populates `target/iai/` with 16 non-zero `summary:` `.out` files — guard
    `grep -rEq '^summary: [1-9]'` passes
- [x] `mise run bench:iai:baseline` regenerates a valid 16-entry JSON baseline — confirmed via
    synthetic `--update` test
- [x] Self-consistency: `--check` against a baseline from the same run → 0% delta, exit 0
- [x] Failure path: tampering one baseline Ir down 50% → exit 1, names
    `bench_a.case1 ... +100.00% REGRESSION`
- [x] `.iai-baseline.json` committed at repo root, not gitignored (`git check-ignore` exits 1)
- [x] `perf` job has both `Check perf regression` (invokes the script) AND
    `Assert non-zero instruction collection` guard; YAML valid; steps correctly ordered (guard →
    check → upload)
- [x] `mise.toml` has `bench:iai:baseline` (+ bonus `bench:iai:check`)
- [x] `rust-core.md` both perf checkboxes `[x]`; `ci-cd.md` perf checkbox `[x]`
- [x] `mise run check` — all 15 pre-commit hooks pass
- [x] `cargo test -p iscc-lib` — 22 tests + 1 doctest pass
- [x] `ruff check` + `ruff format --check` + `ty check` clean on the new script
- [x] Live `--check` vs committed baseline: all 16 benches within 10% (max −1.66% / +0.96%), exit 0
    — CI-sourced baseline agrees closely with local rustc 1.96.0
- [ ] **CI-only (confirm next cycle):** post-push `Perf` job's `Check perf regression` step passes
    against the committed baseline (deferred — runs async after push; baseline IS the CI artifact so
    it should match near-exactly)

**Issues found:**

- (minor, fixed) Stale CI comment on the "Run iai-callgrind benches" step still read "no regression
    gate yet (follow-up slice)" — corrected to describe the now-present `Check perf regression`
    step. Comment-only, no behavior change.
- (filed as new `[review]` issue) Two false-green edge cases in `iai_regression.py` — see Codex
    review below.

**Codex review:** Two valid `[P2]` false-green findings, both filed as the new issue "Harden
iai-callgrind regression gate against false-green edge cases" `normal` `[review]`:

1. **Zero current count slips through** — a single bench reporting `summary: 0` is read as an
    improvement (`cur > base*limit` is false) and passes; the CI guard only catches the *all-zero*
    case. Fix: fail when any shared bench's current Ir is 0.
2. **Disappeared baseline bench only warns** — a baselined bench that stops emitting `.out` warns
    but can still exit 0 (even with empty `shared`). Fix: fail (or `--allow-missing`) when a
    baselined bench is absent. Both are defense-in-depth hardening of an already-working enforcing
    gate, not slice-2b defects — the implementation faithfully follows next.md's "warn, do not
    fail" spec for run/baseline mismatches. Not blocking.

**Next:** Issue #3 (iai-callgrind perf gate) is functionally complete and locally verified — slice
2a + 2b both landed. update-state should confirm the post-push `Perf` run is green with the
`Check perf regression` step passing, then close issue #3. The natural next work package is the new
`[review]` hardening issue above (close the two false-green escape hatches in `iai_regression.py` —
pure script change, no spec amendment needed, normal priority, fully CID-actionable). The other two
normal issues (CRAP `--fail-above 30`, cargo-deny/audit gate) remain HUMAN-REVIEW-REQUESTED spec
amendments. Do NOT start v1.0.0 prep or flip the `Semver` gate autonomously.

**Notes:**

- **Baseline is CI-sourced, not local.** Committed `.iai-baseline.json` was built from CI artifact
    run 27746693860 (`1463edb`), so the CI gate compares like-with-like. Local rustc agrees within
    1.66%. Refresh after an accepted regression via `gh run download ... -n iai-baseline` +
    `--update --from-dir`, or `mise run bench:iai:baseline` (local-rustc fallback, overwrites the
    CI-sourced file).
- **Mid-flush race observed (not a bug):** running `--check` in the same shell command immediately
    after `cargo bench` exited matched only 9/16 benches once; a clean re-run matched all 16. The
    check runs as its own CI step after the bench step fully completes, so this does not affect the
    gate.
- **No concurrent-loop or gate-circumvention concerns:** `ps aux` shows a single `cid:run`; the
    unpushed range adds only the enforcing perf gate (strengthening) — all
    `continue-on-error`/`fail-*` grep hits are prose in context/memory files.
- **Gate is Ir-only and intersection-only by design** — new benches warn (not fail) until a baseline
    refresh; the new hardening issue tightens the *disappeared*-bench direction.
