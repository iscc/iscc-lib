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

## Remove dangling npm `optionalDependencies` from `@iscc/lib` `normal` [human]

GitHub: https://github.com/iscc/iscc-lib/issues/38 (reported by an external consumer)

Published `@iscc/lib@0.4.0` declares five per-platform `optionalDependencies` (`@iscc/lib-<triple>`)
that were **never published to npm** — they 404 on install, leave `{ "optional": true }`
placeholders in consumer lockfiles, and (on recent npm) cause `npm ci` failures (`EUSAGE` /
`Missing: @iscc/lib-darwin-arm64 from lock file`) for any downstream project. Root cause: the source
`crates/iscc-napi/package.json` bundles all binaries via `files: ["*.node"]`, but the release
workflow runs `npx napi prepublish -t npm` (`.github/workflows/release.yml:378`), which **injects**
those `optionalDependencies` while the workflow only ever publishes the main package — never the
sibling packages. The published artifact is thus a broken hybrid of two distribution models.

Fix (decision recorded: keep the single bundled-package model — each stripped addon is ~1.1 MB, so
bundling all five costs little and removes the whole `optionalDependencies` fragility class):

- Stop injecting `optionalDependencies`: drop the `napi prepublish -t npm` step (or strip the
    injected block from `package.json` before `npm publish`). Keep `files: ["*.node"]` and the
    runtime `index.js` platform loader.
- Publish only the single `@iscc/lib` package; do not publish per-platform siblings.
- Verify `npm ci` succeeds in a consumer project and the published tarball ships all five `.node`
    binaries with no `optionalDependencies` in its `package.json`.

**Spec:** `.claude/context/specs/nodejs-bindings.md` → "Native Binary Distribution" (rewritten to
the bundled-package model with a revisit trigger if the bundle grows past ~30 MB)

## Add streaming `SumHasher` to WASM bindings `normal` [human]

GitHub: https://github.com/iscc/iscc-lib/issues/37 (Titusz +1'd extending it to WASM in the thread)

Streaming consumers that need an ISCC-SUM code must currently run two hashers (`DataHasher` +
`InstanceHasher`), feed every chunk to both, and combine with `gen_iscc_code_v0` — crossing the
language→Rust boundary twice per chunk (in WASM, copying each chunk into linear memory twice).

**Progress:** Core `streaming::SumHasher` landed in iteration 88
(`crates/iscc-lib/src/streaming.rs`; `gen_sum_code_v0` now drives it). The PyO3 `SumHasher` wrapper
landed in iteration 89 (`crates/iscc-py`, exported in `__all__`, 11 tests). **Remaining work is the
WASM half only.**

- **wasm-bindgen wrapper** (`crates/iscc-wasm`): expose a `SumHasher` class over the shared core
    `iscc_lib::streaming::SumHasher`, mirroring the existing WASM `DataHasher`/`InstanceHasher`
    finalize-once pattern.
- Verify WASM output matches the two-hasher pattern and the path-based `gen_sum_code_v0` for
    identical data, with finalize-once semantics (`wasm-pack test --node`).

**Spec:** `.claude/context/specs/wasm-bindings.md` → "Streaming Hashers"

## Release the GIL during Python hashing (`allow_threads`) `normal` [human]

GitHub: https://github.com/iscc/iscc-lib/issues/39

`crates/iscc-py/src/lib.rs` holds the GIL for the entire duration of hashing —
`grep allow_threads crates/iscc-py/src/` returns nothing (confirmed). Two Python threads each
driving a hasher therefore serialize on the CPU-bound hash work instead of running in parallel,
bottlenecking threaded consumers (e.g. `iscc-sdk`'s `ThreadPoolExecutor` overlap of
`code_sum`/`code_meta`/`code_image`). Wrap the pure-Rust compute in `py.allow_threads(...)` for the
streaming `update()` methods (`PyDataHasher` ~:541, `PyInstanceHasher` ~:585, and the new
`SumHasher`) and the one-shot byte-data functions (`gen_data_code_v0`, `gen_instance_code_v0`,
`gen_image_code_v0`, `gen_sum_code_v0`). Output is byte-identical, so conformance is unaffected.
Caveats: the `data: &[u8]` borrow of the Python buffer must satisfy PyO3's `Ungil`/`Send` bounds
across the release (confirm soundness for immutable `bytes`, else copy the slice first); consider a
small-input size threshold so releasing the GIL doesn't regress tiny `update()` calls. Injecting
`py: Python<'_>` does not change the Python-facing signature (non-breaking). Validate with a
2-thread multi-GB microbenchmark (expect ~2× on the streaming hashers) and an unchanged self-test
suite.

**Spec:** `.claude/context/specs/python-bindings.md` → "GIL Release During Hashing"

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
loop cut this release autonomously. Ideally land both the `cargo-semver-checks` and `iai-callgrind`
gate issues above first so 1.0.0 ships with enforcement active.

**Spec:** `.claude/context/specs/rust-core.md` → "API Stability & Performance Invariants"

## Add programming language logos to docs site `low` [human]

README language logos added (iteration 3). Consider adding matching logos to `docs/index.md` and
howto guide headers on the documentation site for visual consistency. Purely cosmetic follow-up.
