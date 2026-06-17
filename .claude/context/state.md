<!-- assessed-at: f7f3689222ee9c27de9a727266a51d1d4c794ce3 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.4.0 hardening toward v1.0.0 — CI GREEN

v0.4.0 is released across all registries; all 12 language bindings are functionally met. **CI is now
fully GREEN** — last cycle's `Coverage + CRAP` cargo-crap install flake was fixed by adding
`--force` to the `cargo binstall` step (ci.yml:314), and the job (including the enforcing Phase 3
`CRAP regression gate`) now runs and passes. Remaining work is v1.0.0 hardening: continue the PyO3
minor migration (now at 0.25, advancing toward 0.29 to clear two RustSec advisories), add the
`iai-callgrind` perf-regression gate, and harden the CRAP gate. No critical issues open.

## Rust Core Crate

**Status**: partially met

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing — the `Rust (fmt, clippy, test)` job is GREEN.
- Reusable `pub struct SumHasher` in `streaming.rs` (`new()` / `update(&[u8])` /
    `finalize(bits, wide, add_units) -> SumCodeResult` / `Default`) runs Data-Code and Instance-Code
    in a single pass; `gen_sum_code_v0` (lib.rs:997) drives it. Reachable as
    `iscc_lib::streaming::SumHasher` but intentionally **not** a crate-root Tier 1 re-export — only
    `DataHasher` / `InstanceHasher` are (`lib.rs:24`).
- Internal module visibility narrowed: `lib.rs` declares
    `pub(crate) mod cdc / conformance / dct / minhash / simhash / utils / wtahash`; only `codec`,
    `streaming`, `types` are `pub mod`. All 10 crate-root `pub use` re-exports intact.
- **Semver gate present (informational)**: the `Semver (cargo-semver-checks)` job
    (`continue-on-error: true`, ci.yml) reports job-level `failure` (2 expected breaking changes
    from the post-0.4.0 `pub(crate)` narrowing) but does NOT flip the run conclusion. The target's
    **enforcing** criterion stays unmet (`rust-core.md` "verified when" still `[ ]`) — flip
    `continue-on-error` off only at the v1.0.0 cut.
- **Gap (normal)**: no `iai-callgrind` instruction-count perf gate with committed baseline — the
    only outstanding v1.0.0 CI quality gate with zero implementation.
- Workspace version is `0.4.0`; next target is v1.0.0 (stability-committed under strict SemVer).

## Python Bindings

**Status**: partially met

- Existing criteria met: all symbols exported, both Python 3.10 and 3.14 CI jobs GREEN, ruff clean,
    streaming `SumHasher` wrapper present and exported.
- GIL release done (closed #39): `py.allow_threads(...)` wraps the pure-Rust CPU-bound compute at
    all 7 call sites in `crates/iscc-py/src/lib.rs`; `tests/test_gil.py` adds 7 concurrency tests.
- **PyO3 migration in progress (normal, #1)**: the workspace `pyo3` pin is at `0.25`
    (`Cargo.toml:35` `version = "0.25"`, `Cargo.lock` `pyo3 0.25.1`). `abi3-py310` preserved on the
    workspace dep; `crates/iscc-py/Cargo.toml` still layers `extension-module`. The 0.24→0.25 bump
    (last cycle) needed zero source changes. Four more minor steps remain to reach `0.29.0`, where
    the two RustSec advisories shipped inside the wheel clear. The predicted `IntoPyObject`/lifetime
    breaks have NOT materialized at 0.24 or 0.25 — treat the prediction skeptically for 0.26.

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols exported with TypeScript declarations; Node.js CI job GREEN.
- Issue #38 RESOLVED: `napi prepublish` step removed from `release.yml`; source `package.json` uses
    the bundled model (`files: ["*.node"]`, no `optionalDependencies`). No open issues.

## WASM Bindings

**Status**: met

- All 32 Tier 1 symbols via `#[wasm_bindgen]`; WASM CI job GREEN.
- Streaming `SumHasher` class present (lib.rs:533, finalize-once via `inner.take()`), 8
    `test_sum_hasher_*` tests pass under `wasm-pack test --node`; documented in
    `docs/howto/wasm.md`.

## C FFI

**Status**: met

- cbindgen header committed + freshness check in CI; C test program passes; csbindgen generates
    `NativeMethods.g.cs`. C FFI CI job GREEN. No new issues.

## Java Bindings

**Status**: met

- All 32 Tier 1 symbols via JNI; Java (Maven) CI job GREEN; native libs bundled in JAR. No issues.

## Go Bindings

**Status**: met

- Pure Go (no CGO), all 32 Tier 1 symbols, go vet clean, Go CI job GREEN. No issues.

## Ruby Bindings

**Status**: met

- 32 Tier 1 symbols via Magnus; Ruby CI job GREEN; version synced. No issues.

## C# / .NET Bindings

**Status**: met

- 32 public symbols via P/Invoke over C FFI; C#/.NET CI job GREEN. No issues.

## C++ Bindings

**Status**: met

- C++17 header-only wrapper, all 32 Tier 1 symbols, ASAN clean, vcpkg + Conan; C++ CI job GREEN.

## UniFFI Scaffolding Crate

**Status**: complete (internal, not published)

- 32 `#[uniffi::export]` annotations, proc-macro approach; shared by Swift + Kotlin. No issues.

## Swift Bindings

**Status**: met

- SPM package with UniFFI-generated bindings, all 32 Tier 1 symbols, XCFramework build, Swift CI job
    GREEN on macos-14. Release provenance guard + root Package.swift dump-package smoke test
    present.

## Kotlin Bindings

**Status**: met

- packages/kotlin/ with JNA-loaded UniFFI bindings, 9 desktop+Android targets in release workflow,
    Kotlin CI job GREEN. No issues.

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

- Criterion benches for all 10 `gen_*_v0` (+2) functions, Bench (compile check) CI job GREEN,
    pytest-benchmark 18 functions, speedup factors published (1.3x–158x) in docs/benchmarks.md.
- The `iai-callgrind` *regression gate* (distinct from criterion local profiling) is tracked under
    Rust Core / CI/CD and remains unimplemented.

## CI/CD and Publishing

**Status**: partially met — **CI GREEN**

- **LATEST CI RUN — SUCCESS.** Run 27685108728 (sha `b3c064b`): **conclusion `success`**. URL:
    https://github.com/iscc/iscc-lib/actions/runs/27685108728 — all 16 functional jobs + the
    Coverage
    - CRAP job green; the only `failure` is `Semver (cargo-semver-checks)`, which is
        `continue-on-error: true` (informational, does not flip the run). HEAD `f7f3689` adds only
        `iterations.jsonl` on top of `b3c064b`, so this green run fully covers HEAD's code state.
- **Coverage + CRAP flake FIXED.** Last cycle's `error: no such command: crap` failure was a
    `cargo-binstall` / `Swatinem/rust-cache` cache-poisoning flake. The fix (commit `628c5d9`, the
    only code change this cycle) added `--force` to the install step (ci.yml:314 now
    `cargo binstall -y --force cargo-crap@0.2.2`). Verified on the green run: the
    `Install cargo-crap` step succeeds and ALL CRAP steps ran (not skipped) — CRAP annotations
    report, SARIF report, SARIF upload to Code Scanning, AND the enforcing Phase 3
    `CRAP regression gate` step all conclude `success`. Phase 3 enforcement is back online on every
    commit.
- **CRAP gate hardening gap (open, normal, [review])**: even when it runs, Phase 3 is
    regression-only — a brand-new/renamed function has no baseline entry, reports `★ N new`, and the
    step exits 0 (Codex verified a new uncovered CC=21 fn at CRAP 462 bypassed it). Fix: add
    `--fail-above 30` alongside `--fail-regression` (baseline max ~22.3 < 30). HUMAN REVIEW
    REQUESTED before mandating in the spec.
- v0.4.0 published; release workflow with 8 registry input toggles (crates.io, PyPI, npm, Maven,
    FFI, RubyGems, NuGet, Maven-Kotlin; Swift XCFramework built in `prepare-release`) + version sync
    (16 targets) in place.
- **Gap (normal)**: no `iai-callgrind` perf-regression CI job with committed baseline — the last
    v1.0.0 CI gate with zero implementation.
- Note: `cargo-semver-checks` CI gate EXISTS (informational); becomes enforcing at v1.0.0.

## Open Issues (5 total — 0 critical, 3 normal, 2 low)

Normal (CID-actionable):

1. **Update PyO3 0.23 → 0.29** — security advisories in shipped wheel; incremental migration **in
    progress** (now at **0.25**; advisories clear only at 0.29). Next increment 0.25 → 0.26.
2. **CRAP gate does not fail on new high-CRAP functions** [review] — regression-only gate lets new
    uncovered high-complexity functions through; add `--fail-above 30`. HUMAN REVIEW REQUESTED.
3. **Add `iai-callgrind` performance-regression CI gate** — committed baseline, > 10% fails CI. Not
    started.

Low (human-directed, CID skips):

- **Release core as v1.0.0** — human-driven via `/release` skill; land perf gate + flip semver gate
    to enforcing first.
- **Add programming language logos to docs site** — cosmetic.

## Next Milestone

**CI is GREEN.** With the cargo-crap flake fixed, resume v1.0.0 hardening:

1. **Continue the PyO3 migration: 0.25 → 0.26.** Self-contained recipe (bump the pin in root
    `Cargo.toml` → `cargo update -p pyo3` → build/clippy(`-D warnings`)/fmt →
    `uv run maturin  develop` → `uv run pytest`). Watch `-D warnings` for `IntoPyObject`/lifetime
    deprecations. RustSec advisories clear only at 0.29 — keep going one minor per step, scoped to
    `crates/iscc-py/` (core has no PyO3 dep).
2. **Harden the CRAP gate with `--fail-above 30`** (review-sourced) so new uncovered high-complexity
    functions also fail CI. Confirm `cargo-crap 0.2.2` accepts `--fail-above` and
    `--fail-regression` together; HUMAN REVIEW REQUESTED before amending the spec.
3. **`iai-callgrind` perf-regression CI gate + committed baseline** — the last v1.0.0 stability gate
    with zero implementation, mirroring the reviewed-baseline pattern established for CRAP.

The two `low` issues (v1.0.0 release cut, docs logos) are human-directed and out of CID scope. The
semver gate becoming *enforcing* is a deliberate one-line follow-up tied to the v1.0.0 cut — do not
flip it before then.
