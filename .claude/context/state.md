<!-- assessed-at: b7ff4c25ee011f2c97a0eff30a42f551485e98cd -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.4.0 hardening toward v1.0.0 stability commitment

v0.4.0 is released across all registries; all 12 language bindings are functionally met. Incremental
review since `b1127ed`: the only code change was iteration 86's narrowing of 5 internal modules to
`pub(crate)` (commit `3f6a61d`), which closes one of the v1.0.0 backlog items in code. The remaining
v1.0.0 hardening backlog (SemVer + perf gates, PyO3 bump, npm fix, SumHasher, GIL release, coverage
gate) is still open. Note: local `develop` is **7 commits ahead of `origin/develop`** and the
module-narrowing code change has **not yet been CI-verified**.

## Rust Core Crate

**Status**: partially met

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing on last green CI run.
- **Resolved in code (this iteration)**: internal module visibility narrowed. `lib.rs` now declares
    `pub(crate) mod cdc / conformance / minhash / simhash / utils` (joining `dct` and `wtahash`);
    only `codec`, `types`, `streaming` remain `pub mod`. All 10 crate-root `pub use` re-exports are
    intact (Tier 1 surface preserved); redundant module-path tests were dropped, and no crate or
    package still imports via the now-private module paths (grep-verified). The matching "Narrow
    internal module visibility" issue is still listed in issues.md pending review-agent verification
    \+ deletion.
- **Gap (normal)**: no `cargo-semver-checks` public-API backward-compat gate against the last
    release.
- **Gap (normal)**: no `iai-callgrind` instruction-count perf gate with committed baseline (> 10%
    regression must fail CI).
- Workspace version is `0.4.0`; next target is v1.0.0 (stability-committed under strict SemVer).

## Python Bindings

**Status**: partially met

- Existing criteria met: all symbols exported, Python 3.10 + 3.14 CI jobs green, ruff clean.
- **Gap (normal)**: PyO3 still pinned to `0.23` in root `Cargo.toml` (`workspace.dependencies`);
    issue requires migration to `0.29.0` to clear two RustSec advisories shipped inside the wheel.
- **Gap (normal)**: no streaming `SumHasher` — `grep SumHasher crates/iscc-py/src` returns nothing.
    Requires a core `streaming::SumHasher` first, then the PyO3 wrapper.
- **Gap (normal)**: GIL held for the entire hash duration — `grep allow_threads crates/iscc-py/`
    returns nothing. Threaded consumers serialize on CPU-bound work.

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
- **Gap (normal)**: no streaming `SumHasher` class — same gap as Python. Spec (`wasm-bindings.md` →
    "Streaming Hashers") requires it over the shared core struct.

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
    benchmarks page with speedup factors all present.
- **Gap (low, CID skips)**: language logos in `docs/index.md` and howto headers — cosmetic only.

## Benchmarks

**Status**: met (CI perf gate is a separate v1.0.0 item)

- Criterion benches for all 10 `gen_*_v0` (+2) functions, Bench (compile check) CI job green,
    pytest-benchmark 18 functions, speedup factors published (1.3x–158x) in docs/benchmarks.md.
- Note: the `iai-callgrind` *regression gate* (distinct from criterion local profiling) is tracked
    under Rust Core / CI/CD.

## CI/CD and Publishing

**Status**: partially met

- **LATEST CI RUN** — run 27629376698 (sha `63523ba` on develop): **16/16 jobs SUCCESS** (all
    green). URL: https://github.com/iscc/iscc-lib/actions/runs/27629376698
- **Caveat**: HEAD (`b7ff4c2`) is **7 commits ahead of `origin/develop`** (which is still at
    `63523ba`). The local commits — including the module-narrowing code change `3f6a61d` (touches
    `lib.rs` + removes two test files) and the mise task migration `b1127ed` — are **unpushed and
    not yet CI-verified**. The code change should compile (no remaining module-path refs), but CI
    has not run on it. CI is not failing; it simply hasn't seen the latest work.
- v0.4.0 published; 9-registry release workflow + version sync (16 targets) in place.
- **Gap (normal)**: no Rust coverage / CRAP gate — `.cargo-crap.toml` absent, no `cargo llvm-cov` /
    `cargo crap` step in ci.yml, no `mise run coverage` / `mise run crap` tasks.
- **Gap (normal)**: no `cargo-semver-checks` API backward-compat CI job.
- **Gap (normal)**: no `iai-callgrind` perf-regression CI job with committed baseline.
- **Gap (normal)**: npm `optionalDependencies` injection bug in release.yml (see Node.js above).

## Open Issues (10 total — 0 critical, 8 normal, 2 low)

Normal (CID-actionable):

1. **Remove dangling npm `optionalDependencies`** (#38) — breaks downstream `npm ci`.
2. **Update PyO3 0.23 → 0.29** — security advisories in shipped wheel; incremental migration.
3. **Add streaming `SumHasher` to Python + WASM** (#37) — core struct first, then wrappers.
4. **Release the GIL during Python hashing** (#39) — `py.allow_threads(...)`.
5. **Add Rust coverage + CRAP-metric quality gate** — phased CI job.
6. **Add `cargo-semver-checks` API backward-compat CI gate**.
7. **Add `iai-callgrind` performance-regression CI gate**.
8. **Narrow internal module visibility to `pub(crate)`** — code change landed in `3f6a61d`; awaiting
    CI verification + review-agent deletion. No longer actionable as feature work.

Low (human-directed, CID skips):

- **Release core as v1.0.0** — human-driven via `/release` skill; land semver + perf gates first.
- **Add programming language logos to docs site** — cosmetic.

## Next Milestone

CI is green on the last run, but the loop should **push the 7 local commits** so CI verifies the
module-narrowing change (`3f6a61d`) before further work stacks on top of it. With that confirmed,
proceed on the `normal` backlog toward v1.0.0, suggested order by impact:

1. **Remove dangling npm `optionalDependencies`** (#38) — actively breaks downstream `npm ci` in
    production; highest user impact, contained fix in release.yml.
2. **Update PyO3 to 0.29** — clears two RustSec advisories shipped inside the published wheel; large
    but well-scoped six-minor-version migration.
3. **v1.0.0 stability gates** — `cargo-semver-checks` job and `iai-callgrind` perf gate + baseline.
    These must land before v1.0.0 so the (now-narrowed) API is locked under enforcement.
4. **Rust coverage + CRAP gate** — phased report-only → regression CI job.
5. **Streaming `SumHasher` (core + Python + WASM) and Python GIL release** — DX/perf improvements;
    the SumHasher core struct unblocks both bindings.

The two `low` issues (v1.0.0 release cut, docs logos) are human-directed and remain out of CID
scope.
