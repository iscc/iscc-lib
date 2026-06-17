<!-- assessed-at: e95f57b30053e6350d65419995d15263f282a9f2 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.4.0 hardening toward v1.0.0 stability commitment

v0.4.0 is released across all registries; all 12 language bindings are functionally met. Incremental
review since `140aeba`: **no code changed** — the only commits are CID context/log/define-next
bookkeeping. The `define-next` agent scoped CRAP gate **Phase 2** (report-only `cargo crap` + SARIF)
as the next work package, but no `advance` has implemented it yet. The latest pushed code
(`f61efa0`, which carries the iteration-94 Phase 1 coverage job) is CI-green. The remaining v1.0.0
backlog (PyO3 bump, iai-callgrind perf gate, CRAP Phases 2–3) is all `normal`/`low` tooling — no
functional binding gaps remain.

## Rust Core Crate

**Status**: partially met

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing on the latest green CI run (`f61efa0`).
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
    ci.yml:280) checks the public API against the last release. It currently *reports* 2 expected
    breaking changes (the post-0.4.0 `pub(crate)` narrowing) without failing the run. The target's
    **enforcing** criterion stays unmet — `rust-core.md` "verified when" still `[ ]` because it
    requires the gate to *fail* on unsanctioned breaks AND the crate to be >= 1.0.0. Flip
    `continue-on-error` off only at the v1.0.0 cut.
- **Gap (normal)**: no `iai-callgrind` instruction-count perf gate with committed baseline (> 10%
    regression must fail CI).
- Workspace version is `0.4.0`; next target is v1.0.0 (stability-committed under strict SemVer).

## Python Bindings

**Status**: partially met

- Existing criteria met: all symbols exported, Python 3.10 + 3.14 CI jobs green, ruff clean,
    streaming `SumHasher` wrapper present (`__init__.py:349`, exported in `__all__` at line 402).
- GIL release done (closed #39): `py.allow_threads(...)` wraps the pure-Rust CPU-bound compute at
    all 7 call sites in `crates/iscc-py/src/lib.rs` — 4 one-shot byte functions plus the 3 streaming
    `update()` methods. `tests/test_gil.py` adds 7 concurrency-correctness tests; output
    byte-identical, no Python-facing signature changed.
- **Gap (normal)**: PyO3 still pinned to `0.23` in root `Cargo.toml` (`workspace.dependencies`);
    issue requires incremental migration to `0.29.0` to clear two RustSec advisories shipped inside
    the wheel.

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols exported with TypeScript declarations, Node.js CI job green.
- Issue #38 RESOLVED: the `Prepare npm packages` (`npx napi prepublish -t npm`) step is removed from
    `release.yml` — it was the only source of the five unpublished `@iscc/lib-<triple>`
    `optionalDependencies`. Source `crates/iscc-napi/package.json` uses the bundled model
    (`files: ["*.node"]`, no `optionalDependencies`). No open issues.

## WASM Bindings

**Status**: met

- All 32 Tier 1 symbols via `#[wasm_bindgen]`, WASM CI job green.
- Streaming `SumHasher` class present: `pub struct SumHasher` at lib.rs:533, `impl` at 545,
    finalize-once via `inner.take()`, reusing `WasmSumCodeResult`. 8 `test_sum_hasher_*` tests pass
    under `wasm-pack test --node` (78 total); `docs/howto/wasm.md` documents it. Not promoted to
    Tier 1 (no crate-root re-export).

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
    `docs/howto/wasm.md` document the streaming `SumHasher`. iscc-napi docs reflect the bundled
    single-package npm model.
- **Gap (low, CID skips)**: language logos in `docs/index.md` and howto headers — cosmetic only.

## Benchmarks

**Status**: met (CI perf gate is a separate v1.0.0 item)

- Criterion benches for all 10 `gen_*_v0` (+2) functions, Bench (compile check) CI job green,
    pytest-benchmark 18 functions, speedup factors published (1.3x–158x) in docs/benchmarks.md.
- Note: the `iai-callgrind` *regression gate* (distinct from criterion local profiling) is tracked
    under Rust Core / CI/CD.

## CI/CD and Publishing

**Status**: partially met

- **LATEST CI RUN** — run 27666337100 (sha `f61efa0` on develop): **overall SUCCESS**. URL:
    https://github.com/iscc/iscc-lib/actions/runs/27666337100 — this is the most recent run because
    every commit since (`140aeba`, `6f90953`, `59ec67f`, `e95f57b`) is context/log-only and
    unpushed; no newer code has reached CI.
- **18 actual jobs** (17 YAML entries; `python-test` matrix expands 3.10 + 3.14): 16 functional jobs
    all green (version-check, rust, python-test x2, python, nodejs, wasm, c-ffi, dotnet, java, go,
    ruby, cpp, swift, kotlin, bench) + the `Coverage (cargo llvm-cov)` job (green, artifact-only) +
    the non-blocking `Semver` job. The Semver job reports individual `failure` (2 expected breaking
    changes from the post-0.4.0 `pub(crate)` narrowing), but `continue-on-error: true` keeps the run
    green — by design during the 0.4.0 → 1.0.0 transition.
- **Coverage gate Phase 1 landed (iter 94)**: the `coverage` job (no `needs:`, no
    `continue-on-error`) runs `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info`
    (ci.yml:307) and uploads the LCOV as the `lcov` artifact; mirrored locally by
    `mise run coverage`; `lcov.info` gitignored. This is Phase 1 (measurement) of the 3-phase "Add
    Rust coverage + CRAP-metric quality gate" issue.
- **CRAP Phase 2 scoped, NOT implemented**: `next.md` defines Phase 2 (report-only `cargo crap` +
    GitHub annotations + SARIF upload, `.cargo-crap.toml`, `mise run crap`), but VERIFIED absent —
    `.cargo-crap.toml` does not exist, ci.yml has 0 `cargo crap` references, mise.toml has no `crap`
    task. The coverage/CRAP issue stays open (Phases 2–3 remain); the CRAP *gate* is still unmet.
- **Push state**: HEAD (`e95f57b`) is **4 commits ahead** of `origin/develop` (`f61efa0`). All four
    leading commits — `140aeba` (log), `6f90953` (update-state), `59ec67f` (define-next), `e95f57b`
    (log) — touch only `.claude/` context, memory, and iteration-log files. No source/CI/config code
    is unpushed; the latest code is fully pushed and CI-verified at `f61efa0`.
- v0.4.0 published; release workflow with 8 registry input toggles (crates.io, PyPI, npm, Maven,
    FFI, RubyGems, NuGet, Maven-Kotlin; Swift XCFramework built in `prepare-release`) + version sync
    (16 targets) in place.
- **Gap (normal)**: CRAP *gate* not yet active — `.cargo-crap.toml` absent, no `cargo crap` step in
    ci.yml, no `mise run crap` task (Phase 2/3 of the coverage issue).
- **Gap (normal)**: no `iai-callgrind` perf-regression CI job with committed baseline.
- Note: `cargo-semver-checks` CI gate now EXISTS (informational); becomes enforcing at v1.0.0.

## Open Issues (5 total — 0 critical, 3 normal, 2 low)

Normal (CID-actionable):

1. **Update PyO3 0.23 → 0.29** — security advisories in shipped wheel; incremental migration.
2. **Add Rust coverage + CRAP-metric quality gate** — Phase 1 (LCOV artifact) DONE; Phase 2
    (`cargo crap` report-only + SARIF, currently scoped in next.md) and Phase 3
    (`--fail-regression` baseline) remain.
3. **Add `iai-callgrind` performance-regression CI gate** — committed baseline, > 10% fails CI.

Low (human-directed, CID skips):

- **Release core as v1.0.0** — human-driven via `/release` skill; land perf gate + flip semver gate
    to enforcing first.
- **Add programming language logos to docs site** — cosmetic.

## Next Milestone

CI is overall green and the latest code is verified at `f61efa0`. The immediate next step is already
scoped in `next.md` and awaiting an `advance`:

1. **CRAP Phase 2 (report-only `cargo crap`)** — builds directly on the `lcov.info` artifact from
    Phase 1: add `.cargo-crap.toml`, install pinned `cargo-crap` via `cargo binstall`, run
    report-only with `--format github` annotations + SARIF upload to Code Scanning (add
    `permissions` for SARIF), and add the `mise run crap` task (non-failing). Most self-contained
    next step.
2. **`iai-callgrind` perf-regression CI gate + committed baseline** — the remaining v1.0.0 stability
    gate; pairs with the now-present semver gate. Deterministic instruction counts on shared
    runners,
    > 10% regression fails CI.
3. **Update PyO3 to 0.29** — clears two RustSec advisories shipped inside the published wheel;
    security-relevant but a six-minor-version jump with breaking changes per minor, so split it
    (start 0.23 → 0.24) and scope to `crates/iscc-py/` only (core has no PyO3 dep).

The two `low` issues (v1.0.0 release cut, docs logos) are human-directed and remain out of CID
scope. The semver gate becoming *enforcing* (drop `continue-on-error`) is a deliberate one-line
follow-up tied to the v1.0.0 cut — do not flip it before then.
