<!-- assessed-at: eead1d6a95f90a549774e4744f29523d2748f49f -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.4.0 hardening toward v1.0.0 stability commitment

v0.4.0 is released across all registries; all 12 language bindings are functionally met. Incremental
review since `3912039`: the **CRAP gate Phase 3** enforcing step was pushed and is now **CI-verified
green** (run `27680364506`, sha `cb8b7e9`), and the **PyO3 migration started** (0.23 → 0.24, CI
green). Remaining v1.0.0 hardening: finish the PyO3 bump to 0.29, add the `iai-callgrind` perf gate,
add a `--fail-above` guard to the CRAP gate, and flip the semver gate to enforcing at the 1.0.0 cut.

## Rust Core Crate

**Status**: partially met

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing on the latest green CI run (`cb8b7e9`).
- Reusable `pub struct SumHasher` in `streaming.rs` (`new()` / `update(&[u8])` /
    `finalize(bits, wide, add_units) -> SumCodeResult` / `Default`) runs the Data-Code and
    Instance-Code algorithms in a single pass; `gen_sum_code_v0` (lib.rs:997) drives it. Reachable
    as `iscc_lib::streaming::SumHasher` (module is `pub mod`) but intentionally **not** a crate-root
    Tier 1 re-export — only `DataHasher` / `InstanceHasher` are (`lib.rs:24`). It backs the core
    path-based `gen_sum_code_v0`, the Python `SumHasher` wrapper, and the WASM `SumHasher` class.
- Internal module visibility is narrowed: `lib.rs` declares
    `pub(crate) mod cdc / conformance / dct / minhash / simhash / utils / wtahash`; only `codec`,
    `streaming`, `types` are `pub mod`. All 10 crate-root `pub use` re-exports intact.
- **Semver gate present (informational)**: a `Semver (cargo-semver-checks)` CI job
    (`obi1kenobi/cargo-semver-checks-action@v2`, `package: iscc-lib`, `continue-on-error: true`,
    ci.yml:280) checks the public API against the last release. On the latest run it *reports*
    `failure` (2 expected breaking changes from the post-0.4.0 `pub(crate)` narrowing) but the
    run-level conclusion stays SUCCESS — NOT a CI failure. The target's **enforcing** criterion
    stays unmet — `rust-core.md` "verified when" still `[ ]` because it requires the gate to *fail*
    on unsanctioned breaks AND the crate to be >= 1.0.0. Flip `continue-on-error` off only at the
    v1.0.0 cut.
- **Gap (normal)**: no `iai-callgrind` instruction-count perf gate with committed baseline (> 10%
    regression must fail CI). This is now the only outstanding v1.0.0 CI quality gate that has no
    implementation at all.
- Workspace version is `0.4.0`; next target is v1.0.0 (stability-committed under strict SemVer).

## Python Bindings

**Status**: partially met

- Existing criteria met: all symbols exported, Python 3.10 + 3.14 CI jobs green, ruff clean,
    streaming `SumHasher` wrapper present (`__init__.py:349`, exported in `__all__` at line 402).
- GIL release done (closed #39): `py.allow_threads(...)` wraps the pure-Rust CPU-bound compute at
    all 7 call sites in `crates/iscc-py/src/lib.rs`. `tests/test_gil.py` adds 7 concurrency tests.
- **PyO3 migration in progress (normal, was #1)**: the workspace `pyo3` pin advanced `0.23 → 0.24`
    (`Cargo.toml`, `abi3-py310` preserved; CI green, 286 pytest pass, zero source changes). Five
    more minor steps remain to reach `0.29.0`, which is where the two RustSec advisories shipped
    inside the wheel actually clear. PyO3 0.25+ tightens `IntoPyObject`/lifetime rules, so real
    source work is expected to begin at the 0.24 → 0.25 increment.

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols exported with TypeScript declarations, Node.js CI job green.
- Issue #38 RESOLVED: the `napi prepublish` step is removed from `release.yml`; source
    `crates/iscc-napi/package.json` uses the bundled model (`files: ["*.node"]`, no
    `optionalDependencies`). No open issues.

## WASM Bindings

**Status**: met

- All 32 Tier 1 symbols via `#[wasm_bindgen]`, WASM CI job green.
- Streaming `SumHasher` class present (lib.rs:533, finalize-once via `inner.take()`), 8
    `test_sum_hasher_*` tests pass under `wasm-pack test --node`; documented in
    `docs/howto/wasm.md`.

## C FFI

**Status**: met

- cbindgen header committed + freshness check in CI; C test program passes; csbindgen generates
    `NativeMethods.g.cs`. C FFI CI job green. No new issues.

## Java Bindings

**Status**: met

- All 32 Tier 1 symbols via JNI; Java (Maven) CI job green; native libs bundled in JAR. No issues.

## Go Bindings

**Status**: met

- Pure Go (no CGO), all 32 Tier 1 symbols, go vet clean, Go CI job green. No issues.

## Ruby Bindings

**Status**: met

- 32 Tier 1 symbols via Magnus; Ruby CI job green; version synced. No issues.

## C# / .NET Bindings

**Status**: met

- 32 public symbols via P/Invoke over C FFI; C#/.NET CI job green. No issues.

## C++ Bindings

**Status**: met

- C++17 header-only wrapper, all 32 Tier 1 symbols, ASAN clean, vcpkg + Conan; C++ CI job green.

## UniFFI Scaffolding Crate

**Status**: complete (internal, not published)

- 32 `#[uniffi::export]` annotations, proc-macro approach; shared by Swift + Kotlin. No issues.

## Swift Bindings

**Status**: met

- SPM package with UniFFI-generated bindings, all 32 Tier 1 symbols, XCFramework build, Swift CI job
    green on macos-14. Release provenance guard + root Package.swift dump-package smoke test
    present.

## Kotlin Bindings

**Status**: met

- packages/kotlin/ with JNA-loaded UniFFI bindings, 9 desktop+Android targets in release workflow,
    Kotlin CI job green. No issues.

## README

**Status**: met

- Polyglot public README with CI + registry badges, per-language install + quick start for all 12
    languages, architecture section, MainTypes table. No issues.

## Per-Crate READMEs

**Status**: met

- READMEs present for all 12 crates/packages; registry metadata references them. No issues.

## Documentation

**Status**: met (one low-priority cosmetic gap)

- Docs site, 11 language howto guides, tabbed multi-language examples, llms-full.txt generation,
    benchmarks page with speedup factors all present. `docs/howto/python.md` and
    `docs/howto/wasm.md` document the streaming `SumHasher`.
- **Gap (low, CID skips)**: language logos in `docs/index.md` and howto headers — cosmetic only.

## Benchmarks

**Status**: met (CI perf gate is a separate v1.0.0 item)

- Criterion benches for all 10 `gen_*_v0` (+2) functions, Bench (compile check) CI job green,
    pytest-benchmark 18 functions, speedup factors published (1.3x–158x) in docs/benchmarks.md.
- Note: the `iai-callgrind` *regression gate* (distinct from criterion local profiling) is tracked
    under Rust Core / CI/CD and remains unimplemented.

## CI/CD and Publishing

**Status**: partially met

- **LATEST CI RUN** — run 27680364506 (sha `cb8b7e9` on `origin/develop`): **overall SUCCESS**. URL:
    https://github.com/iscc/iscc-lib/actions/runs/27680364506 — 18 jobs: 16 functional (all green) +
    `Coverage + CRAP` (green) + non-blocking `Semver` (reports `failure` but `continue-on-error`
    keeps the run green — NOT a CI failure). HEAD (`eead1d6`) adds only `iterations.jsonl` beyond
    `cb8b7e9`, so this run fully covers HEAD's code state.
- **CRAP gate Phase 3 PUSHED + CI-VERIFIED GREEN (was the top open risk; now resolved)**: the
    `Coverage + CRAP` job's final enforcing step `CRAP regression gate`
    (`cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression`, ci.yml:335-336,
    NOT `continue-on-error`) ran for the first time in CI on `cb8b7e9` and **passed**. The committed
    `.crap-baseline.json` (repo root, 97 entries across 10 `crates/iscc-lib/src/` files) held up on
    `@stable` — no cross-environment determinism flap occurred. A `mise run crap:baseline`
    (`depends=["coverage"]`) task regenerates it; baseline refreshes are reviewed commits. The
    push/verify gap noted in the prior state is closed.
- **CRAP gate hardening gap (NEW issue, normal, [review])**: the Phase 3 gate is regression-only — a
    brand-new or renamed function has no baseline entry, so it reports `★ N new` and the step still
    exits 0. Codex (iter 97) verified a new uncovered CC=21 function at CRAP 462 bypassed the gate.
    Fix is to add `--fail-above 30` alongside `--fail-regression` (current baseline max ~22.3 is
    well below 30, so it would not break existing code). Flagged HUMAN REVIEW REQUESTED before
    mandating in the spec.
- v0.4.0 published; release workflow with 8 registry input toggles (crates.io, PyPI, npm, Maven,
    FFI, RubyGems, NuGet, Maven-Kotlin; Swift XCFramework built in `prepare-release`) + version sync
    (16 targets) in place.
- **Gap (normal)**: no `iai-callgrind` perf-regression CI job with committed baseline — the last
    v1.0.0 CI gate with zero implementation.
- Note: `cargo-semver-checks` CI gate EXISTS (informational); becomes enforcing at v1.0.0.

## Open Issues (6 total — 0 critical, 4 normal, 2 low)

Normal (CID-actionable):

1. **Update PyO3 0.23 → 0.29** — security advisories in shipped wheel; incremental migration **in
    progress** (now at 0.24; advisories clear only at 0.29). Next increment 0.24 → 0.25.
2. **Add Rust coverage + CRAP-metric quality gate** — all three phases implemented and now
    CI-verified green (Phase 1 LCOV, Phase 2 report-only crap+SARIF, Phase 3 enforcing
    `--fail-regression`). Still listed open in issues.md; awaiting review-agent deletion (the
    follow-up hardening below is tracked separately).
3. **CRAP gate does not fail on new high-CRAP functions** [review] — regression-only gate lets new
    uncovered high-complexity functions through; add `--fail-above 30`. HUMAN REVIEW REQUESTED.
4. **Add `iai-callgrind` performance-regression CI gate** — committed baseline, > 10% fails CI. Not
    started.

Low (human-directed, CID skips):

- **Release core as v1.0.0** — human-driven via `/release` skill; land perf gate + flip semver gate
    to enforcing first.
- **Add programming language logos to docs site** — cosmetic.

## Next Milestone

CI is green on the latest pushed commit, the CRAP Phase 3 enforcing gate is now CI-verified, and the
duplicate-CID-loop concurrency hazard is cleared. No CI failures to fix. The immediate priorities,
in order:

1. **Continue the PyO3 migration: 0.24 → 0.25.** Same self-contained recipe (bump the pin in root
    `Cargo.toml` → `cargo update -p pyo3` → build/clippy(`-D warnings`)/fmt →
    `uv run maturin develop` → `uv run pytest`). Watch for real source work starting at 0.25
    (tightened `IntoPyObject`/lifetime rules). RustSec advisories only clear at 0.29 — keep going
    one minor per step, scoped to `crates/iscc-py/` (core has no PyO3 dep).
2. **Harden the CRAP gate with `--fail-above 30`** (review-sourced follow-up) so new uncovered
    high-complexity functions also fail CI, closing the regression-only bypass. Confirm
    `cargo-crap 0.2.2` accepts `--fail-above` and `--fail-regression` together; HUMAN REVIEW
    REQUESTED before amending the spec.
3. **`iai-callgrind` perf-regression CI gate + committed baseline** — the last remaining v1.0.0
    stability gate with zero implementation, mirroring the reviewed-baseline pattern established
    for CRAP. Deterministic instruction counts on shared runners; > 10% regression fails CI.

The two `low` issues (v1.0.0 release cut, docs logos) are human-directed and remain out of CID
scope. The semver gate becoming *enforcing* (drop `continue-on-error`) is a deliberate one-line
follow-up tied to the v1.0.0 cut — do not flip it before then.
