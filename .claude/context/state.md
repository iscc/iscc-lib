<!-- assessed-at: 96558ca9be3bbab8df475fcc16257d857455d19d -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.4.0 hardening toward v1.0.0 stability commitment

v0.4.0 is released across all registries; all 12 language bindings are functionally met. Incremental
review since `a8f1ffc`: iteration 93 added an **informational** `cargo-semver-checks` API
backward-compat CI gate (advance `9d42077`, reviewed `758fc31`) plus a mirrored `mise run semver`
task, closing one of the four `normal` backlog issues. CI is overall green on the latest pushed code
(`758fc31`); the remaining v1.0.0 hardening backlog (PyO3 bump, iai-callgrind perf gate, CRAP
coverage gate) is all `normal`/`low` tooling work — no functional binding gaps remain.

## Rust Core Crate

**Status**: partially met

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing on the latest green CI run (`758fc31`).
- Reusable `pub struct SumHasher` in `streaming.rs` (`new()` / `update(&[u8])` /
    `finalize(bits, wide, add_units) -> SumCodeResult` / `Default`) runs the Data-Code and
    Instance-Code algorithms in a single pass; `gen_sum_code_v0` (lib.rs:997) drives it. Reachable
    as `iscc_lib::streaming::SumHasher` (module is `pub mod`) but intentionally **not** a crate-root
    Tier 1 re-export — only `DataHasher` / `InstanceHasher` are (`lib.rs:24`). It backs the core
    path-based `gen_sum_code_v0`, the Python `SumHasher` wrapper, and the WASM `SumHasher` class.
- Internal module visibility is narrowed: `lib.rs` declares
    `pub(crate) mod cdc / conformance / dct / minhash / simhash / utils / wtahash`; only `codec`,
    `streaming`, `types` are `pub mod`. All 10 crate-root `pub use` re-exports intact.
- **Semver gate now present (informational)**: a `Semver (cargo-semver-checks)` CI job
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

- **LATEST CI RUN** — run 27664286163 (sha `758fc31` on develop): **overall SUCCESS**. URL:
    https://github.com/iscc/iscc-lib/actions/runs/27664286163 — covers the semver-gate addition
    (advance `9d42077`, review `758fc31`).
- **17 jobs total**: 16 functional jobs all green (version-check, rust, python-test x2, python,
    nodejs, wasm, c-ffi, dotnet, java, go, ruby, cpp, swift, kotlin, bench) + the **new non-blocking
    `Semver` job**. The Semver job reports individual `failure` (2 expected breaking changes from
    the post-0.4.0 `pub(crate)` narrowing), but `continue-on-error: true` keeps the run green — by
    design during the 0.4.0 → 1.0.0 transition.
- **Push state**: HEAD (`96558ca`) == `origin/develop`; it is `cid(log): iteration 93` (touches only
    `iterations.jsonl` / memory / context). All code is pushed and CI-verified.
- v0.4.0 published; release workflow with 8 registry input toggles (crates.io, PyPI, npm, Maven,
    FFI, RubyGems, NuGet, Maven-Kotlin; Swift XCFramework built in `prepare-release`) + version sync
    (16 targets) in place.
- **Gap (normal)**: no Rust coverage / CRAP gate — `.cargo-crap.toml` absent, no `cargo llvm-cov` /
    `cargo crap` step in ci.yml, no `mise run coverage` / `mise run crap` tasks.
- **Gap (normal)**: no `iai-callgrind` perf-regression CI job with committed baseline.
- Note: `cargo-semver-checks` CI gate now EXISTS (informational); becomes enforcing at v1.0.0.

## Open Issues (5 total — 0 critical, 3 normal, 2 low)

Normal (CID-actionable):

1. **Update PyO3 0.23 → 0.29** — security advisories in shipped wheel; incremental migration.
2. **Add Rust coverage + CRAP-metric quality gate** — phased CI job.
3. **Add `iai-callgrind` performance-regression CI gate** — committed baseline, > 10% fails CI.

Low (human-directed, CID skips):

- **Release core as v1.0.0** — human-driven via `/release` skill; land perf gate + flip semver gate
    to enforcing first.
- **Add programming language logos to docs site** — cosmetic.

## Next Milestone

CI is overall green and the latest code is verified. The `cargo-semver-checks` gate landed
(informational), leaving three `normal` v1.0.0-hardening items. Suggested order by impact:

1. **`iai-callgrind` perf-regression CI gate + committed baseline** — the remaining v1.0.0 stability
    gate; pairs with the now-present semver gate. Deterministic instruction counts on shared
    runners,
    > 10% regression fails CI. Most self-contained next step.
2. **Update PyO3 to 0.29** — clears two RustSec advisories shipped inside the published wheel;
    security-relevant but a six-minor-version jump with breaking changes per minor, so split it
    (start 0.23 → 0.24) and scope to `crates/iscc-py/` only (core has no PyO3 dep).
3. **Rust coverage + CRAP gate** — phased report-only → regression CI job; largest build-out.

The two `low` issues (v1.0.0 release cut, docs logos) are human-directed and remain out of CID
scope. The semver gate becoming *enforcing* (drop `continue-on-error`) is a deliberate one-line
follow-up tied to the v1.0.0 cut — do not flip it before then.
