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

## Add Rust coverage + CRAP-metric quality gate `normal` [human]

Stand up Rust test-coverage measurement (`cargo llvm-cov` → LCOV) for the core `iscc-lib` crate and
gate it with the CRAP (Change Risk Anti-Patterns) metric via
[`cargo-crap`](https://github.com/minikin/cargo-crap). Today Rust complexity is gated (clippy
`cognitive-complexity-threshold = 15`) but coverage is unmeasured; CRAP combines both to surface
functions that are complex *and* undertested. Implement as a CI job — not a pre-push hook, since
instrumented coverage runs would roughly double local push time. Phase it in: (1) `cargo llvm-cov`
LCOV artifact, (2) report-only `cargo crap` with GitHub annotations + SARIF upload, (3)
`--fail-regression` against a baseline refreshed on merges to `develop`. Pin `cargo-crap` (pre-1.0),
install via `cargo binstall`, configure via `.cargo-crap.toml`, and add `mise run coverage` /
`mise run crap` local tasks.

**Spec:** `.claude/context/specs/ci-cd.md` → "Rust Coverage and CRAP Quality Gate"

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
