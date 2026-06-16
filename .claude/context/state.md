<!-- assessed-at: b1127ed6c517fae431f3aaecf00007c93f2bd25d -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.4.0 hardening toward v1.0.0 stability commitment

v0.4.0 is released across all registries and all 12 language bindings are green in CI (16/16 jobs).
However, the project is **no longer near-complete**: the human triaged GitHub issues into the
backlog, adding **7 new `normal` issues** and tightening `target.md`/specs with new v1.0.0
acceptance criteria (API-stability + performance CI gates, Rust coverage/CRAP gate). Several
sections that were "met" for v0.4.0 are now "partially met" against the raised bar. CI is green, so
work can proceed directly on the backlog.

## Rust Core Crate

**Status**: partially met

- Core API met: all 10 `gen_*_v0` functions present (including `gen_sum_code_v0`), 32 Tier 1
    symbols, conformance against `iscc-core/data.json` passing (Rust CI job green).
- Workspace version is `0.4.0`; next target is v1.0.0 (stability-committed under strict SemVer).
- **Gap (normal)**: new `target.md` "verified when" criteria not yet met — no `cargo-semver-checks`
    API backward-compat gate, no `iai-callgrind` instruction-count perf gate with committed
    baseline.
- **Gap (normal)**: internal modules still over-exposed. `crates/iscc-lib/src/lib.rs` declares
    `pub mod cdc / conformance / minhash / simhash / utils` — these should be `pub(crate) mod`
    before v1.0.0 locks the API surface (only `codec`, `types`, `streaming` stay public). `dct` and
    `wtahash` are already `pub(crate)`.

## Python Bindings

**Status**: partially met

- Existing criteria met: all symbols exported, both Python 3.10 and 3.14 CI jobs green, ruff clean.
- **Gap (normal)**: PyO3 still pinned to `0.23` in root `Cargo.toml` (`workspace.dependencies`);
    issue requires migration to `0.29.0` to clear two RustSec advisories shipped inside the wheel.
- **Gap (normal)**: no streaming `SumHasher` — `grep SumHasher crates/iscc-py/src` returns nothing.
    Requires a core `streaming::SumHasher` first, then the PyO3 wrapper.
- **Gap (normal)**: GIL is held for the entire hash duration — `grep allow_threads crates/iscc-py/`
    returns nothing. Threaded consumers serialize on CPU-bound work.

## Node.js Bindings

**Status**: partially met

- Existing criteria met: all 32 Tier 1 symbols exported, Node.js CI job green.
- **Gap (normal, external bug report #38)**: the release workflow injects broken
    `optionalDependencies`. Source `crates/iscc-napi/package.json` uses the bundled model
    (`files: ["*.node"]`, no `optionalDependencies`), but `release.yml:378` runs
    `npx napi prepublish -t npm`, which injects five per-platform `@iscc/lib-<triple>` deps that are
    never published — causing `npm ci` (`EUSAGE`) failures for downstream consumers of `@iscc/lib`.
    Fix: drop the prepublish injection, keep the single bundled package.

## WASM Bindings

**Status**: partially met

- Existing criteria met: all 32 Tier 1 symbols via `#[wasm_bindgen]`, WASM CI job green.
- **Gap (normal)**: no streaming `SumHasher` class — same gap as Python. Spec (`wasm-bindings.md` →
    "Streaming Hashers") now requires it over the shared core struct.

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
- Note: the new `iai-callgrind` *regression gate* (distinct from criterion local profiling) is
    tracked under Rust Core / CI/CD below.

## CI/CD and Publishing

**Status**: partially met

- **LATEST CI RUN** — run 27629376698 (sha 63523ba on develop): **16/16 jobs SUCCESS** (all green).
    URL: https://github.com/iscc/iscc-lib/actions/runs/27629376698
- HEAD (b1127ed) is 2 commits ahead of that run, but both commits
    (`docs(cid): triage GitHub   issues`, `fix(mise): migrate task args`) touch only
    context/dev-tooling files — no CI-tested code changed, so green status holds.
- v0.4.0 published; 9-registry release workflow + version sync (16 targets) in place.
- **Gap (normal)**: no Rust coverage / CRAP gate — `.cargo-crap.toml` absent, no `cargo llvm-cov` or
    `cargo crap` step in ci.yml, no `mise run coverage` / `mise run crap` tasks.
- **Gap (normal)**: no `cargo-semver-checks` API backward-compat CI job.
- **Gap (normal)**: no `iai-callgrind` perf-regression CI job with committed baseline.
- **Gap (normal)**: npm `optionalDependencies` injection bug in release.yml (see Node.js above).

## Open Issues (9 total — 0 critical, 7 normal, 2 low)

Normal (CID-actionable):

1. **Update PyO3 0.23 → 0.29** — security advisories in shipped wheel; incremental migration.
2. **Remove dangling npm `optionalDependencies`** (#38) — breaks downstream `npm ci`.
3. **Add streaming `SumHasher` to Python + WASM** (#37) — core struct first, then wrappers.
4. **Release the GIL during Python hashing** (#39) — `py.allow_threads(...)`.
5. **Add Rust coverage + CRAP-metric quality gate** — phased CI job.
6. **Add `cargo-semver-checks` API backward-compat CI gate**.
7. **Add `iai-callgrind` performance-regression CI gate**.
8. **Narrow internal module visibility to `pub(crate)`** before v1.0.0 (lib.rs).

(That is 8 normal entries; #2 and #4 share the same DX cluster — count them as listed in issues.md.)

Low (human-directed, CID skips):

- **Release core as v1.0.0** — human-driven via `/release` skill; land semver + perf gates first.
- **Add programming language logos to docs site** — cosmetic.

## Next Milestone

CI is green, so no CI-fix priority — proceed directly on the `normal` backlog toward v1.0.0.
Suggested order by impact:

1. **Remove dangling npm `optionalDependencies`** — actively breaks downstream `npm ci` in
    production (external consumer report #38); highest user impact, contained fix in release.yml.
2. **Update PyO3 to 0.29** — clears two RustSec advisories shipped inside the published wheel; large
    but well-scoped six-minor-version migration.
3. **v1.0.0 stability gates** — `cargo-semver-checks` job, `iai-callgrind` perf gate + baseline, and
    narrow internal module visibility to `pub(crate)`. These must land before v1.0.0 so the API is
    locked under enforcement; the module-visibility change is itself a (last allowed) breaking
    change that belongs in the pre-1.0 window.
4. **Rust coverage + CRAP gate** — phased report-only → regression CI job.
5. **Streaming `SumHasher` (core + Python + WASM) and Python GIL release** — DX/perf improvements
    for streaming consumers; the SumHasher core struct unblocks both bindings.

The two `low` issues (v1.0.0 release cut, docs logos) are human-directed and remain out of CID
scope.
