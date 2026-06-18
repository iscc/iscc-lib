<!-- assessed-at: bb9f02e5f80e6345f8eeed8574b144405be5ca58 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.4.0 hardening toward v1.0.0 — CI GREEN; iai-callgrind perf regression gate now committed and enforcing

v0.4.0 is released across all registries; all 12 language bindings are functionally met. **CI is
GREEN** on the pushed tip (`a5ce73c`, run 27750907410, overall conclusion `success`). This iteration
landed slice 2b of the `iai-callgrind` perf gate: a committed `.iai-baseline.json` (16 Ir entries,
10% tolerance), a stdlib-only `scripts/iai_regression.py`, an enforcing `Check perf regression` step
in the `Perf` CI job, and `bench:iai:baseline` / `bench:iai:check` mise tasks. The post-push `Perf`
job is GREEN with the regression step passing against the committed baseline — the deferred CI-only
verification from the handoff is now confirmed, so issue #3 is functionally complete (awaiting
review-agent deletion). No critical issues open.

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
- **Perf gate (was normal #3) — NOW COMPLETE & ENFORCING.** `crates/iscc-lib/benches/iai_benches.rs`
    is an `iai-callgrind` 0.16 instruction-count harness (11 `bench_*` fns → 16 parametrized runtime
    cases). `[profile.bench] strip = false, debug = true` (Cargo.toml:61) keeps the
    `__iai_callgrind_wrapper` toggle symbols so callgrind collects real counts (the earlier false
    green). The `Perf (iai-callgrind)` CI job installs valgrind + `iai-callgrind-runner@0.16.1`,
    runs the benches (env `IAI_CALLGRIND_ALLOW_ASLR=true`), passes an
    `Assert non-zero instruction   collection` guard, then runs the enforcing
    `Check perf regression` step (`python3 scripts/iai_regression.py --check`) which fails CI if any
    bench's Ir exceeds the committed `.iai-baseline.json` by >10%. Baseline = 16 Ir entries, metric
    `Ir`, tolerance 10%, committed at repo root (NOT gitignored). `bench:iai`, `bench:iai:check`,
    `bench:iai:baseline` mise tasks present. Job is GREEN on the pushed tip with the regression step
    passing. This satisfies target.md "No > 10% performance regression ... gated by `iai-callgrind`
    vs a committed baseline" — `rust-core.md` perf "verified when" boxes are now `[x]`.
- **Semver gate present (informational, still unmet for v1.0.0)**: the
    `Semver (cargo-semver-checks)` job (`continue-on-error: true`, ci.yml) reports job-level
    `failure` (2 expected breaking changes from the post-0.4.0 `pub(crate)` narrowing) but does NOT
    flip the run conclusion. The target's **enforcing** criterion stays unmet (`rust-core.md` semver
    "verified when" still `[ ]`) — flip `continue-on-error` off only at the v1.0.0 cut.
- Workspace version is `0.4.0`; next target is v1.0.0 (stability-committed under strict SemVer). The
    semver/v1.0.0 enforcement is the only remaining Rust Core gap; everything else met.

## Python Bindings

**Status**: met

- All symbols exported, both Python 3.10 and 3.14 CI jobs GREEN, ruff clean, streaming `SumHasher`
    wrapper present and exported.
- GIL release done (closed #39): GIL-release wraps the pure-Rust CPU-bound compute at all 7 call
    sites in `crates/iscc-py/src/lib.rs` via the `Python::detach` API; `tests/test_gil.py` adds 7
    concurrency tests. 0 `allow_threads` remain.
- **PyO3 migration COMPLETE (closed #1)**: the workspace `pyo3` pin is `0.29` (`Cargo.toml`
    `version = "0.29"` with `abi3-py310`); `Cargo.lock` resolves a single `pyo3 0.29.0` with no
    older entries — the version where both targeted RustSec advisories shipped in the wheel clear.
    The explicit `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697, documented) is
    preserved (PyO3 0.28 silently flipped that default `true`->`false`).
- Note: advisory clearance was confirmed only by the mechanical lockfile proxy (single
    `pyo3 0.29.0`, no older entries) — `cargo deny`/`cargo audit` absent. Tracked as a separate
    `[review]` CI/CD issue.

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
    GREEN. Release provenance guard + root Package.swift dump-package smoke test present.

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

**Status**: met

- Criterion benches for all 10 `gen_*_v0` (+2) functions, Bench (compile check) CI job GREEN,
    pytest-benchmark 18 functions, speedup factors published (1.3x-158x) in docs/benchmarks.md.
- Second harness `iai_benches.rs` (iai-callgrind 0.16, 11 `bench_*` fns) runs in CI under valgrind,
    collects genuine non-zero Ir counts, and is now gated by an **enforcing >10% regression check**
    against the committed `.iai-baseline.json` (issue #3, slices 2a+2b, complete). Results upload as
    the `iai-baseline` artifact (`if: always()`).

## CI/CD and Publishing

**Status**: partially met — **CI GREEN on pushed tip**

- **LATEST PUSHED CI RUN — SUCCESS.** Run 27750907410 on sha `a5ce73c` (= `origin/develop` tip):
    overall conclusion `success`; all 19 functional jobs green (confirmed via check-runs API),
    including the `Perf (iai-callgrind)` job (its enforcing `Check perf regression` step passes
    against the committed baseline), the `Coverage + CRAP` job, and the enforcing Phase 3 CRAP gate.
    The only job-level `failure` is `Semver (cargo-semver-checks)`, which is
    `continue-on-error:   true` (informational, does not flip the run). URL:
    https://github.com/iscc/iscc-lib/actions/runs/27750907410
- **`Perf (iai-callgrind)` gate now ENFORCING (this iteration, slice 2b).** Pipeline: install
    valgrind + `iai-callgrind-runner@0.16.1` (binstall `--force`) → run benches (env
    `IAI_CALLGRIND_ALLOW_ASLR=true`) → `Assert non-zero instruction collection` guard →
    `Check perf regression` (`python3 scripts/iai_regression.py --check`, stdlib-only, fails on >10%
    Ir regression vs committed `.iai-baseline.json`) → upload `iai-baseline` artifact
    (`if: always()`). No `needs:`, no `continue-on-error` — it enforces. Baseline refresh via
    `mise run   bench:iai:baseline` (reviewed commit).
- **HEAD is 1 commit ahead of origin/develop, code-clean.** HEAD (`bb9f02e`) is the iteration-109
    log commit (`iterations.jsonl` only); origin/develop is `a5ce73c`. No code is unpushed — the CI
    result on `a5ce73c` reflects the current code.
- **Coverage + CRAP GREEN.** The enforcing Phase 3 `CRAP regression gate`
    (`cargo crap --baseline .crap-baseline.json --fail-regression`, ci.yml) runs on every commit and
    concludes `success`.
- **CRAP gate hardening gap (open, normal, [review])**: Phase 3 is regression-only — a
    brand-new/renamed function has no baseline entry, reports `★ N new`, and the step exits 0 (Codex
    verified a new uncovered CC=21 fn at CRAP 462 bypassed it). Fix: add `--fail-above 30` alongside
    `--fail-regression` (baseline max ~22.3 < 30). HUMAN REVIEW REQUESTED before mandating in spec.
- **Supply-chain audit gap (open, normal, [review])**: `notes/07-security-versioning.md` mandates
    `cargo deny check` "Run in CI" (licenses + RustSec advisories + duplicate versions) via a
    workspace-root `deny.toml`, plus `cargo audit`. None exists — no `deny.toml`, no CI job, no
    `mise` task, neither tool installed. HUMAN REVIEW REQUESTED (requirement lives only in design
    notes, not the CID specs).
- **iai_regression.py false-green hardening (open, normal, [review], NEW this cycle)**: two narrow
    escape hatches Codex flagged in the new gate script — (1) a single bench reporting `summary: 0`
    is read as an improvement and passes (the CI guard only catches the all-zero case); (2) a
    baselined bench that stops emitting a `.out` only warns and can still exit 0. Both are
    defense-in-depth on an already-working enforcing gate, not slice-2b defects. No spec change
    needed — pure script change, fully CID-actionable.
- v0.4.0 published; release workflow with 8 registry input toggles (crates.io, PyPI, npm, Maven,
    FFI, RubyGems, NuGet, Maven-Kotlin; Swift XCFramework built in `prepare-release`) + version sync
    (16 targets) in place.

## Open Issues (issues.md lists 6 — 0 critical, 4 normal, 2 low)

Issue #3 (iai-callgrind perf gate, [human]) is **functionally COMPLETE and CI-verified** this cycle
(Perf job green, regression step passing) — still listed in issues.md, awaiting review-agent
deletion. Remaining normal issues:

1. **Harden iai-callgrind regression gate against false-green edge cases** [review] (NEW) — zero
    current count + disappeared-baseline-bench escape hatches in `scripts/iai_regression.py`. Pure
    script change, no spec amendment, **fully CID-actionable** — the natural next work package.
2. **CRAP gate does not fail on new high-CRAP functions** [review] — regression-only gate lets new
    uncovered high-complexity functions through; add `--fail-above 30`. HUMAN REVIEW REQUESTED.
3. **Wire up `cargo deny`/`cargo audit` supply-chain gate** [review] — design notes mandate it but
    no `deny.toml`/CI job/`mise` task exists. HUMAN REVIEW REQUESTED (spec gap — requirement only
    in notes/07, not CID specs).

Low (human-directed, CID skips):

- **Release core as v1.0.0** — human-driven via `/release` skill; flip semver gate to enforcing
    first.
- **Add programming language logos to docs site** — cosmetic.

## Next Milestone

**CI is GREEN on the pushed tip and the iai-callgrind perf gate is now committed, enforcing, and
CI-verified.** The obvious next CID-actionable slice is the one issue with no review gate:

1. **Harden `scripts/iai_regression.py` against the two false-green edge cases** (review-sourced,
    NEW) — fail `check_regressions` when any shared bench's current Ir is 0, and fail (or require
    `--allow-missing`) when a baselined bench is absent from the run. Pure script change, no spec
    amendment, fully CID-actionable. Confirm `cargo test -p iscc-lib` is unaffected.
2. **Wire up the `cargo deny`/`cargo audit` supply-chain gate** (review-sourced) — `deny.toml` at
    the workspace root, a `Security audit` CI job, and a `mise run audit` task. **HUMAN REVIEW
    REQUESTED before amending the spec** — the requirement lives only in `notes/07`, not the CID
    specs.
3. **Harden the CRAP gate with `--fail-above 30`** (review-sourced). Confirm `cargo-crap 0.2.2`
    accepts `--fail-above` and `--fail-regression` together; HUMAN REVIEW REQUESTED before amending
    the spec.

The two `low` issues (v1.0.0 release cut, docs logos) are human-directed and out of CID scope. The
semver gate becoming *enforcing* is a deliberate one-line follow-up tied to the v1.0.0 cut — do not
flip it before then. define-next should not start v1.0.0 prep or flip the `Semver` gate
autonomously.
