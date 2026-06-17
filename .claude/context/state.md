<!-- assessed-at: 9cf84be898d8632216ba43d7fe15bed492fcd113 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.4.0 hardening toward v1.0.0 — CI RED (cargo-crap install flake)

v0.4.0 is released across all registries; all 12 language bindings are functionally met. **CI is
currently FAILING** on the latest pushed commit: the `Coverage + CRAP` job dies at the report step
with `error: no such command: crap` — a `cargo-binstall` / `Swatinem/rust-cache` cache-poisoning
flake, NOT a code regression. The only code change this cycle was the PyO3 migration advancing
`0.24 → 0.25` (CI-green locally; the CRAP failure is unrelated to it). Fixing CI is the top
priority.

## Rust Core Crate

**Status**: partially met

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing — the `Rust (fmt, clippy, test)` job is GREEN on
    the latest run.
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
- **PyO3 migration in progress (normal, #1)**: this cycle the workspace `pyo3` pin advanced
    `0.24 → 0.25` (`Cargo.toml:35` now `version = "0.25"`, `Cargo.lock` `pyo3 0.25.1`). `abi3-py310`
    preserved on the workspace dep; `crates/iscc-py/Cargo.toml` still layers `extension-module`.
    Zero source changes needed; review PASS confirmed build/clippy(`-D warnings`)/fmt/maturin abi3
    wheel/286 pytest all green locally. Four more minor steps remain to reach `0.29.0`, where the
    two RustSec advisories shipped inside the wheel clear. The predicted `IntoPyObject`/lifetime
    breaks have NOT yet materialized at 0.24 or 0.25.

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

**Status**: partially met — **CI RED (must fix first)**

- **LATEST CI RUN — FAILURE.** Run 27683405546 (sha `e5ff328`, the PyO3 0.25 commit): **conclusion
    `failure`**. URL: https://github.com/iscc/iscc-lib/actions/runs/27683405546 — 16 of 18 jobs
    green; 2 jobs red:
    - **`Coverage + CRAP (cargo llvm-cov + cargo crap)` — REAL FAILURE (flips the run).** This job is
        NOT `continue-on-error`. Root cause: an infra/cache flake, NOT a code regression. The
        `Install cargo-crap` step (ci.yml:313-314, `cargo binstall -y cargo-crap@0.2.2`, **no
        `--force`**) logged `cargo-crap v0.2.2 is already installed, use --force to override` and
        skipped the install — but `Swatinem/rust-cache@v2` (ci.yml:304) had restored cargo's `.crates`
        metadata WITHOUT the `~/.cargo/bin/cargo-crap` binary, so the next step
        `cargo crap --lcov lcov.info --format github` (step 10, report-only) died with
        `error: no such command: crap` (exit 101). The SARIF report, SARIF upload, and the enforcing
        `CRAP regression gate` (step 13) were all **SKIPPED** — so Phase 3 enforcement never even ran
        on this commit. The first green run `cb8b7e9` populated the poisoned cache, so this will
        **recur on every run** until fixed (e.g. add `--force` to the binstall, or stop trusting the
        cached install record). The lcov coverage build itself succeeded and uploaded fine.
    - `Semver (cargo-semver-checks)` — reports job-level `failure` (2 expected post-0.4.0 breaks) but
        is `continue-on-error: true`, so it does NOT flip the run. Informational; not a blocker.
- HEAD `9cf84be` adds only `iterations.jsonl` on top of `e5ff328`, so this failing run fully covers
    HEAD's code state. There is no newer (green) re-run.
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

The review sweep this cycle deleted the resolved "Add Rust coverage + CRAP-metric quality gate"
issue (all 3 phases were implemented and had run CI-green once on `cb8b7e9`).

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

**CI is RED — fixing it is the top priority before any feature work.**

1. **Fix the `Coverage + CRAP` job's cargo-crap install flake.** The cached cargo metadata records
    cargo-crap as installed while the binary is missing, so binstall skips it and `cargo crap` is
    not on PATH. The minimal robust fix is to make the install deterministic — e.g. add `--force`
    to `cargo binstall -y cargo-crap@0.2.2` (ci.yml:314) so it always installs the binary
    regardless of the cached record, or exclude the binstall record from the restored cache. Re-run
    CI to confirm green on the current code. (This is infra, not a code regression — the PyO3 0.25
    bump is fine.)
2. **Continue the PyO3 migration: 0.25 → 0.26.** Same self-contained recipe (bump the pin in root
    `Cargo.toml` → `cargo update -p pyo3` → build/clippy(`-D warnings`)/fmt →
    `uv run maturin  develop` → `uv run pytest`). Watch `-D warnings` for `IntoPyObject`/lifetime
    deprecations. RustSec advisories clear only at 0.29 — keep going one minor per step, scoped to
    `crates/iscc-py/` (core has no PyO3 dep).
3. **Harden the CRAP gate with `--fail-above 30`** (review-sourced) so new uncovered high-complexity
    functions also fail CI. Confirm `cargo-crap 0.2.2` accepts `--fail-above` and
    `--fail-regression` together; HUMAN REVIEW REQUESTED before amending the spec.
4. **`iai-callgrind` perf-regression CI gate + committed baseline** — the last v1.0.0 stability gate
    with zero implementation, mirroring the reviewed-baseline pattern established for CRAP.

The two `low` issues (v1.0.0 release cut, docs logos) are human-directed and out of CID scope. The
semver gate becoming *enforcing* is a deliberate one-line follow-up tied to the v1.0.0 cut — do not
flip it before then.
