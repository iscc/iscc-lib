# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The
review agent deletes resolved issues after verification (history in git).

<!-- Add issues below this line -->

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

## Add `cargo-semver-checks` API backward-compat CI gate `normal` [human]

Add a CI job that runs [`cargo-semver-checks`](https://github.com/obi1kenobi/cargo-semver-checks) on
the public API of the `iscc-lib` core crate, comparing the working tree against the last published
release. Catches renamed/removed/retyped public items (Tier 1 + Tier 2 `codec`) that conformance
vectors cannot see. Install via `cargo binstall cargo-semver-checks`. During the 0.4.0 → 1.0.0
transition the check is informational; from v1.0.0 it must fail the build on any breaking change not
matched by a major bump. Add a `mise` task to run it locally.

**Spec:** `.claude/context/specs/rust-core.md` → "API Stability & Performance Invariants";
`.claude/context/specs/ci-cd.md` → "API Stability and Performance Gates"

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
loop cut this release autonomously. Ideally land both gate issues above first so 1.0.0 ships with
enforcement active.

**Spec:** `.claude/context/specs/rust-core.md` → "API Stability & Performance Invariants"

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.
