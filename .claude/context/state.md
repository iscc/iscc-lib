<!-- assessed-at: 2a0e78d705637e89fe3941219fcab46efca2df70 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.4.0 hardening toward v1.0.0 stability commitment

v0.4.0 is released across all registries; all 12 language bindings are functionally met. Incremental
review since `af05564`: iteration 89 added the PyO3 streaming `SumHasher` wrapper to the Python
bindings (closing the Python half of issue #37), with a `.pyi` stub, docs, and 11 new tests. The
review PASSED and rescoped #37 to WASM-only. CI is green 16/16 on the latest pushed code
(`d333c56`), so this work is verified. The remaining v1.0.0 hardening backlog (npm fix, WASM
SumHasher, PyO3 bump, GIL release, SemVer + perf + coverage gates) is still open.

## Rust Core Crate

**Status**: partially met

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing on the latest green CI run (`d333c56`).
- Reusable `pub struct SumHasher` in `streaming.rs` (`new()` / `update(&[u8])` /
    `finalize(bits, wide, add_units) -> SumCodeResult` / `Default`) runs the Data-Code and
    Instance-Code algorithms in a single pass; `gen_sum_code_v0` (lib.rs:997) drives it. Reachable
    as `iscc_lib::streaming::SumHasher` (module is `pub mod`) but intentionally **not** a crate-root
    Tier 1 re-export — only `DataHasher` / `InstanceHasher` are (`lib.rs:24`). This now backs both
    the core path-based `gen_sum_code_v0` and the new Python `SumHasher` wrapper; the WASM wrapper
    remains the only open consumer (issue #37).
- Internal module visibility is narrowed: `lib.rs` declares
    `pub(crate) mod cdc / conformance / dct /   minhash / simhash / utils / wtahash`; only `codec`,
    `streaming`, `types` are `pub mod`. All 10 crate-root `pub use` re-exports intact (Tier 1
    surface preserved).
- **Gap (normal)**: no `cargo-semver-checks` public-API backward-compat gate against the last
    release.
- **Gap (normal)**: no `iai-callgrind` instruction-count perf gate with committed baseline (> 10%
    regression must fail CI).
- Workspace version is `0.4.0`; next target is v1.0.0 (stability-committed under strict SemVer).

## Python Bindings

**Status**: partially met

- Existing criteria met: all symbols exported, Python 3.10 + 3.14 CI jobs green, ruff clean.
- **New this iteration (`742759b`)**: streaming `SumHasher` wrapper landed.
    `#[pyclass(name =   "SumHasher")]` (`PySumHasher`, lib.rs:615) over the core
    `iscc_lib::streaming::SumHasher`, plus a Pythonic `SumHasher` wrapper class (`__init__.py:349`)
    mirroring `DataHasher`/`InstanceHasher`, exported in `__all__` (line 402), with a `.pyi` stub
    and 11 new tests (`tests/test_streaming.py`, 19 SumHasher references). Closes the **Python half
    of #37**.
- **Gap (normal)**: PyO3 still pinned to `0.23` in root `Cargo.toml` (`workspace.dependencies`);
    issue requires migration to `0.29.0` to clear two RustSec advisories shipped inside the wheel.
- **Gap (normal, #39)**: GIL held for the entire hash duration —
    `grep allow_threads crates/iscc-py/src/` returns nothing. Threaded consumers serialize on
    CPU-bound work. When this lands it should also wrap the new `PySumHasher.update()`.

## Node.js Bindings

**Status**: partially met

- Existing criteria met: all 32 Tier 1 symbols exported, Node.js CI job green.
- **Gap (normal, external bug #38)**: the release workflow injects broken `optionalDependencies`.
    Source `crates/iscc-napi/package.json` uses the bundled model (`files: ["*.node"]`, no
    `optionalDependencies`), but `release.yml:378` runs `npx napi prepublish -t npm`, injecting five
    per-platform `@iscc/lib-<triple>` deps that are never published — causing `npm ci` (`EUSAGE`)
    failures for downstream consumers. Fix: drop the prepublish injection, keep the single bundled
    package.

## WASM Bindings

**Status**: partially met

- Existing criteria met: all 32 Tier 1 symbols via `#[wasm_bindgen]`, WASM CI job green.
- **Gap (normal, #37 — now WASM-only)**: no streaming `SumHasher` class —
    `grep SumHasher crates/iscc-wasm/src` returns nothing. The shared core struct exists and the
    Python wrapper landed this iteration; the wasm-bindgen wrapper class is the **only remaining
    half of #37**. Mirror the WASM `DataHasher`/`InstanceHasher` finalize-once pattern; verify with
    `wasm-pack test --node` against `gen_sum_code_v0` and the two-hasher path.

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
    benchmarks page with speedup factors all present. `docs/howto/python.md` now documents the
    Python `SumHasher` (reviewer also added the `units?` optional field to the result-types table).
- **Gap (low, CID skips)**: language logos in `docs/index.md` and howto headers — cosmetic only.

## Benchmarks

**Status**: met (CI perf gate is a separate v1.0.0 item)

- Criterion benches for all 10 `gen_*_v0` (+2) functions, Bench (compile check) CI job green,
    pytest-benchmark 18 functions, speedup factors published (1.3x–158x) in docs/benchmarks.md.
- Note: the `iai-callgrind` *regression gate* (distinct from criterion local profiling) is tracked
    under Rust Core / CI/CD.

## CI/CD and Publishing

**Status**: partially met

- **LATEST CI RUN** — run 27654149923 (sha `d333c56` on develop): **16/16 jobs SUCCESS** (all
    green). URL: https://github.com/iscc/iscc-lib/actions/runs/27654149923 — this run **includes**
    the Python SumHasher binding (advance `742759b`), so it is CI-verified.
- **Push state**: HEAD (`2a0e78d`) is only 1 commit ahead of `origin/develop` (`d333c56`); that
    commit is `cid(log): iteration 89` (touches only `iterations.jsonl`). All code is pushed and
    verified.
- v0.4.0 published; 9-registry release workflow + version sync (16 targets) in place.
- **Gap (normal)**: no Rust coverage / CRAP gate — `.cargo-crap.toml` absent, no `cargo llvm-cov` /
    `cargo crap` step in ci.yml, no `mise run coverage` / `mise run crap` tasks.
- **Gap (normal)**: no `cargo-semver-checks` API backward-compat CI job.
- **Gap (normal)**: no `iai-callgrind` perf-regression CI job with committed baseline.
- **Gap (normal)**: npm `optionalDependencies` injection bug in release.yml:378 (see Node.js above).

## Open Issues (9 total — 0 critical, 7 normal, 2 low)

Normal (CID-actionable):

1. **Remove dangling npm `optionalDependencies`** (#38) — breaks downstream `npm ci`.
2. **Update PyO3 0.23 → 0.29** — security advisories in shipped wheel; incremental migration.
3. **Add streaming `SumHasher` to WASM** (#37) — Python half done (`742759b`); only the wasm-bindgen
    wrapper class remains.
4. **Release the GIL during Python hashing** (#39) — `py.allow_threads(...)`; also cover new
    `PySumHasher.update()`.
5. **Add Rust coverage + CRAP-metric quality gate** — phased CI job.
6. **Add `cargo-semver-checks` API backward-compat CI gate**.
7. **Add `iai-callgrind` performance-regression CI gate**.

Low (human-directed, CID skips):

- **Release core as v1.0.0** — human-driven via `/release` skill; land semver + perf gates first.
- **Add programming language logos to docs site** — cosmetic.

## Next Milestone

CI is green and the latest code is verified, so the loop can proceed directly on the `normal`
backlog toward v1.0.0. Suggested order by impact:

1. **Add streaming `SumHasher` to WASM** (#37) — the only remaining half of a tracked issue; the
    core struct and the Python wrapper both exist, so this is a cheap, well-patterned wasm-bindgen
    wrapper that fully closes #37 across all bindings.
2. **Remove dangling npm `optionalDependencies`** (#38) — actively breaks downstream `npm ci` in
    production; highest user impact, contained fix in release.yml.
3. **Update PyO3 to 0.29** — clears two RustSec advisories shipped inside the published wheel; large
    but well-scoped six-minor-version migration.
4. **v1.0.0 stability gates** — `cargo-semver-checks` job and `iai-callgrind` perf gate + baseline.
    These must land before v1.0.0 so the (now-narrowed) API is locked under enforcement.
5. **Rust coverage + CRAP gate** — phased report-only → regression CI job.
6. **Python GIL release** (#39) — DX/perf improvement for threaded consumers (now also covers
    `PySumHasher.update()`).

The two `low` issues (v1.0.0 release cut, docs logos) are human-directed and remain out of CID
scope.
