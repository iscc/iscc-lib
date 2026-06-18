# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The
review agent deletes resolved issues after verification (history in git).

<!-- Add issues below this line -->

## CRAP gate does not fail on new high-CRAP functions `normal` [review]

The Phase 3 CRAP gate (`cargo crap --baseline .crap-baseline.json --fail-regression`, ci.yml) only
exits non-zero when an *existing* baseline entry's score worsens. A brand-new (or renamed) function
has no baseline entry, so it is reported as `★ N new` and the step still exits 0 — a new uncovered,
high-complexity function bypasses the enforced gate entirely. Verified by Codex (iter 97): a new
uncovered CC=21 function reported `★ 1 new` at CRAP 462 and the gate returned success; the
report-only `--format github`/`--format sarif` steps are non-failing too, so nothing blocks it.

This was a deliberate scope choice for Phase 3 (regression-only, no absolute `--fail-above`), so it
is a follow-up hardening item, not a Phase 3 defect. Fix: add `--fail-above 30` alongside
`--fail-regression` so new code above the configured threshold also fails. The current baseline max
is ~22.3 (well below 30), so this would not break existing code. Confirm `cargo-crap 0.2.2` accepts
both flags together before wiring it in.

**Spec:** `.claude/context/specs/ci-cd.md` → "Rust Coverage and CRAP Quality Gate" — HUMAN REVIEW
REQUESTED before amending the spec to mandate `--fail-above` (review-sourced).

## Wire up `cargo deny`/`cargo audit` supply-chain gate `normal` [review]

`notes/07-security-versioning.md` mandates `cargo deny check` "Run in CI" (licenses + RustSec
advisories + duplicate versions) configured via a workspace-root `deny.toml`, plus `cargo audit` as
a complement. None of this exists: no `deny.toml`, no CI job, no `mise` task, and neither tool is
installed in the devcontainer. This gap surfaced concretely in iteration 105 — the PyO3 0.29 bump
(issue #1) targeted two RustSec advisories but the clearance could only be verified by the
mechanical proxy "lockfile resolves a single `pyo3 0.29.0`, no older entries", not by an actual
advisory scan. Fix: add `deny.toml` at the workspace root, a `Security audit` CI job running
`cargo deny check` (advisories + bans + licenses), and a `mise run audit` task; install via
`taiki-e/install-action` or `cargo binstall -y --force` (note the rust-cache poisoning gotcha — see
learnings "cargo binstall + Swatinem/rust-cache"). Optionally add `npm audit` for the napi package
per the same note.

**Spec:** `.claude/context/specs/ci-cd.md` — HUMAN REVIEW REQUESTED before amending the spec to
mandate a supply-chain audit gate (review-sourced; the requirement currently lives only in the
design notes, not the CID specs).

## Add `iai-callgrind` performance-regression CI gate `normal` [human]

Add a Linux CI job with [`iai-callgrind`](https://github.com/iai-callgrind/iai-callgrind)
instruction-count benches for the hot `gen_*_v0` / hashing / CDC / MinHash paths. Instruction counts
(valgrind) are deterministic, so the gate is stable on shared runners — unlike wall-clock criterion.
Commit a baseline to the repo and fail CI on a > 10% regression; baseline refreshes are deliberate
reviewed commits. Keep the existing `criterion` benches for local profiling. Add `mise` tasks to run
and to refresh the baseline.

**Spec:** `.claude/context/specs/rust-core.md` → "API Stability & Performance Invariants";
`.claude/context/specs/ci-cd.md` → "API Stability and Performance Gates"

## Harden iai-callgrind regression gate against false-green edge cases `normal` [review]

`scripts/iai_regression.py` (iter 109, slice 2b of the perf gate) has two narrow false-green gaps
Codex flagged:

1. **Zero current count slips through.** A single benchmark reporting `summary: 0` (e.g. a partial
    strip or a harness regression) is read as a large *improvement*, so `cur > base * limit` is
    false and `--check` passes. The CI `Assert non-zero instruction collection` guard only catches
    the *all-zero* case (`grep -rEq '^summary: [1-9]'` matches as long as ANY bench is non-zero).
    Fix: in `check_regressions`, fail when any shared bench's current Ir is 0.
2. **Disappeared baseline bench only warns.** If a benchmark in `.iai-baseline.json` stops emitting
    a `.out` (renamed/removed harness case), it is reported via `only_baseline` as a warning and
    the script can still exit 0 — even when `shared` is empty. Fix: fail (or require an explicit
    `--allow-missing`) when a baselined bench is absent from the run, so the committed baseline
    only shrinks via a deliberate `--update` refresh.

Both are defense-in-depth hardening of an already-working enforcing gate (the normal case and the
all-zero case are correctly handled), not a slice-2b defect. No spec change needed — `ci-cd.md`
already says the gate "fails on a > 10% regression"; this just closes the zero/missing escape
hatches. Confirm `cargo test -p iscc-lib` is unaffected (script-only change).

## Release core as v1.0.0 (stability commitment) `low` [human]

Human-driven release: cut **v1.0.0** as the first stability-committed release of the lockstep
workspace (per the 1.0.0 decision). This is the one release allowed to break the 0.4.0 API freely;
afterward 1.x is locked under strict SemVer. Drive via the `/release` skill — do NOT let the CID
loop cut this release autonomously. Ideally land both the `cargo-semver-checks` and `iai-callgrind`
gate issues above first so 1.0.0 ships with enforcement active.

**Spec:** `.claude/context/specs/rust-core.md` → "API Stability & Performance Invariants"

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.
