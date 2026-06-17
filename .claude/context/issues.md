# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The
review agent deletes resolved issues after verification (history in git).

<!-- Add issues below this line -->

## Update PyO3 to latest release (security fixes) `normal` [human]

Bump PyO3 from the pinned `0.23` (`Cargo.toml` `workspace.dependencies`) to the latest stable
**`0.29.0`** (released 2026-06-11). The 0.29.0 release closes two RustSec advisories — a missing
`Sync` bound on `PyCFunction::new_closure` closures, and a possible out-of-bounds read in
`BoundTupleIterator::nth_back` / `BoundListIterator::nth_back` — plus several minor breaking changes
that close soundness holes. PyO3 ships inside the published Python wheel, so keep it current.

Scope: this is a six-minor-version jump (0.23 → 0.24 → … → 0.29), and each PyO3 minor release
carries breaking API changes. Migrate incrementally, following the PyO3 migration guide
(https://pyo3.rs/main/migration), rather than jumping the version pin in one step. Touch points are
`crates/iscc-py/` (`Cargo.toml` `pyo3/extension-module`, `pyproject.toml` `pyo3/extension-module`,
and `src/`); the pure-Rust core carries no PyO3 dependency.

Constraints / verification:

- Preserve `abi3-py310` (one wheel per platform, Python 3.10+) — confirm 0.29.0 still supports it.
- Conformance vectors and `pytest` must stay green; abi3 wheels must build on all four CI targets
    (linux x86_64/aarch64, macos universal2, windows x64).
- Update `Cargo.lock`; re-run the supply-chain check (`cargo deny check` / `cargo audit`, per
    `notes/07-security-versioning.md`) to confirm the advisories clear.

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

## Add `iai-callgrind` performance-regression CI gate `normal` [human]

Add a Linux CI job with [`iai-callgrind`](https://github.com/iai-callgrind/iai-callgrind)
instruction-count benches for the hot `gen_*_v0` / hashing / CDC / MinHash paths. Instruction counts
(valgrind) are deterministic, so the gate is stable on shared runners — unlike wall-clock criterion.
Commit a baseline to the repo and fail CI on a > 10% regression; baseline refreshes are deliberate
reviewed commits. Keep the existing `criterion` benches for local profiling. Add `mise` tasks to run
and to refresh the baseline.

**Spec:** `.claude/context/specs/rust-core.md` → "API Stability & Performance Invariants";
`.claude/context/specs/ci-cd.md` → "API Stability and Performance Gates"

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
