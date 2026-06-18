<!-- assessed-at: 54bbddc6c3bb57d3e729d2171c0200fdfa875e0f -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.4.0 hardening toward v1.0.0 — CI GREEN; iai-callgrind perf gate enforcing AND hardened against false greens; at a human-handoff point

v0.4.0 is released across all registries; all 12 language bindings are functionally met. **CI is
GREEN** on the pushed tip (`6d6c594`, run 27753395707, overall conclusion `success`). This iteration
landed the iai-callgrind regression-gate hardening: `scripts/iai_regression.py` now fails on
zero-count benches and on disappeared baselined benches (with `--allow-missing` as a deliberate
escape hatch), covered by 11 synthetic-fixture pytest tests. Issue #3 (the perf gate itself) and the
false-green hardening issue were both verified PASS and swept from issues.md. Only 4 issues remain —
2 `normal` [review] items, both **HUMAN REVIEW REQUESTED** spec amendments, plus 2 `low` [human]
items — so there is no fully-autonomous CID work package left; the loop sits at a natural human
handoff point. No critical issues open.

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
- **Perf gate — COMPLETE, ENFORCING, AND HARDENED.** `crates/iscc-lib/benches/iai_benches.rs` is an
    `iai-callgrind` 0.16 instruction-count harness (11 `bench_*` fns → 16 parametrized runtime
    cases). `[profile.bench] strip = false, debug = true` (Cargo.toml:61) preserves the
    `__iai_callgrind_wrapper` toggle symbols so callgrind collects real counts. The
    `Perf (iai-callgrind)` CI job (ci.yml:281-333, no `continue-on-error`) installs valgrind +
    `iai-callgrind-runner@0.16.1`, runs the benches (env `IAI_CALLGRIND_ALLOW_ASLR=true`), asserts
    non-zero instruction collection, then runs the enforcing `Check perf regression` step
    (`python3 scripts/iai_regression.py --check`, ci.yml:324) which fails CI if any bench's Ir
    exceeds the committed `.iai-baseline.json` (16 Ir entries, metric `Ir`, 10% tolerance, repo
    root) by >10%. This iteration hardened the gate script (248 lines) against two false-green
    edges: it now fails when any shared bench reports a zero Ir count (independent of
    `--allow-missing`) and when a baselined bench disappears from the run (`--allow-missing`
    downgrades that to a warning); new benches still warn only. Satisfies target.md "No > 10%
    performance regression ... gated by `iai-callgrind` vs a committed baseline" — `rust-core.md`
    perf "verified when" boxes are `[x]`.
- **Semver gate present (informational, still unmet for v1.0.0)**: the
    `Semver (cargo-semver-checks)` job (ci.yml:334-344, `continue-on-error: true`) reports job-level
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
    older entries. The explicit `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697,
    documented) is preserved (PyO3 0.28 silently flipped that default `true`->`false`).
- Note: advisory clearance was confirmed only by the mechanical lockfile proxy (single
    `pyo3 0.29.0`, no older entries) — `cargo deny`/`cargo audit` absent. Tracked as a separate
    `[review]` CI/CD issue (HUMAN REVIEW REQUESTED).

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
    collects genuine non-zero Ir counts, and is gated by an **enforcing >10% regression check**
    against the committed `.iai-baseline.json`. The gate script (`scripts/iai_regression.py`, 248
    lines, stdlib-only) is now hardened against zero-count and disappeared-bench false greens
    (`--allow-missing` escape hatch), covered by 11 synthetic-fixture pytest tests
    (`tests/test_iai_regression.py`). Results upload as the `iai-baseline` artifact
    (`if: always()`).

## CI/CD and Publishing

**Status**: partially met — **CI GREEN on pushed tip**

- **LATEST PUSHED CI RUN — SUCCESS.** Run 27753395707 on sha `6d6c594` (= `origin/develop` tip):
    overall conclusion `success`; all 19 functional jobs green (confirmed via check-runs API on the
    actual tip SHA), including `Perf (iai-callgrind)`, `Coverage + CRAP`, and the enforcing Phase 3
    CRAP gate. The only job-level `failure` is `Semver (cargo-semver-checks)`, which is
    `continue-on-error: true` (informational, does not flip the run). URL:
    https://github.com/iscc/iscc-lib/actions/runs/27753395707
- **`Perf (iai-callgrind)` gate ENFORCING and HARDENED.** Pipeline (ci.yml:281-333, no `needs:`, no
    `continue-on-error`): install valgrind + `iai-callgrind-runner@0.16.1` (binstall `--force`) →
    run benches (env `IAI_CALLGRIND_ALLOW_ASLR=true`) → `Assert non-zero instruction collection`
    guard → `Check perf regression` (`python3 scripts/iai_regression.py --check`, fails on >10% Ir
    regression, zero-count, or disappeared baselined bench vs committed `.iai-baseline.json`) →
    upload `iai-baseline` artifact (`if: always()`). Baseline refresh via
    `mise run bench:iai:baseline` (reviewed commit).
- **HEAD is 1 commit ahead of origin/develop, code-clean.** HEAD (`54bbddc`) is the iteration-110
    log commit (`iterations.jsonl` only); origin/develop is `6d6c594`. No code is unpushed — the CI
    result on `6d6c594` reflects the current code.
- **Coverage + CRAP GREEN.** The enforcing Phase 3 `CRAP regression gate`
    (`cargo crap --baseline .crap-baseline.json --fail-regression`, ci.yml) runs on every commit and
    concludes `success`.
- **CRAP gate hardening gap (open, normal, [review], HUMAN REVIEW REQUESTED)**: Phase 3 is
    regression-only — a brand-new/renamed function has no baseline entry, reports `★ N new`, and the
    step exits 0 (Codex verified a new uncovered CC=21 fn at CRAP 462 bypassed it). Fix: add
    `--fail-above 30` alongside `--fail-regression` (baseline max ~22.3 < 30). HUMAN REVIEW
    REQUESTED before mandating in spec.
- **Supply-chain audit gap (open, normal, [review], HUMAN REVIEW REQUESTED)**:
    `notes/07-security-versioning.md` mandates `cargo deny check` "Run in CI" (licenses + RustSec
    advisories + duplicate versions) via a workspace-root `deny.toml`, plus `cargo audit`. None
    exists — no `deny.toml`, no CI job, no `mise` task, neither tool installed. HUMAN REVIEW
    REQUESTED (requirement lives only in design notes, not the CID specs).
- v0.4.0 published; release workflow with 8 registry input toggles (crates.io, PyPI, npm, Maven,
    FFI, RubyGems, NuGet, Maven-Kotlin; Swift XCFramework built in `prepare-release`) + version sync
    (16 targets) in place.

## Open Issues (issues.md lists 4 — 0 critical, 2 normal, 2 low)

Issue #3 (iai-callgrind perf gate, [human]) and the iai false-green hardening issue ([review]) were
both verified PASS and **deleted** from issues.md this cycle. The 2 remaining `normal` issues are
both **HUMAN REVIEW REQUESTED** spec amendments — neither is fully CID-autonomous:

1. **CRAP gate does not fail on new high-CRAP functions** [review] — regression-only gate lets new
    uncovered high-complexity functions through; fix is `--fail-above 30`. Requires amending
    `ci-cd.md` first; confirm `cargo-crap 0.2.2` accepts `--fail-above` + `--fail-regression`
    together. HUMAN REVIEW REQUESTED.
2. **Wire up `cargo deny`/`cargo audit` supply-chain gate** [review] — design notes mandate it but
    no `deny.toml`/CI job/`mise` task exists; the requirement lives only in `notes/07`, not the CID
    specs. HUMAN REVIEW REQUESTED before amending the spec.

Low (human-directed, CID skips):

- **Release core as v1.0.0** — human-driven via `/release` skill; flip semver gate to enforcing
    first.
- **Add programming language logos to docs site** — cosmetic.

## Next Milestone

**CI is GREEN on the pushed tip and the iai-callgrind perf gate is committed, enforcing, hardened,
and CI-verified. There is no fully-autonomous CID `normal` work package remaining** — the loop is at
a natural human-handoff point.

- Both open `normal` [review] issues are **HUMAN REVIEW REQUESTED** spec amendments. define-next
    must NOT start either autonomously:
    1. **CRAP gate `--fail-above 30`** — amend `ci-cd.md` and confirm `cargo-crap 0.2.2` accepts both
        flags together first.
    2. **`cargo deny`/`cargo audit` supply-chain gate** — `deny.toml` at the workspace root, a
        `Security audit` CI job, and a `mise run audit` task; spec amendment required (requirement
        lives only in `notes/07`).
- The two `low` issues (v1.0.0 release cut, docs logos) are human-directed and out of CID scope. The
    semver gate becoming *enforcing* is a deliberate follow-up tied to the v1.0.0 cut — do not flip
    it before then.
- If define-next finds no autonomous work, the next no-op advance should let review signal IDLE. Do
    not start v1.0.0 prep or flip the `Semver` gate autonomously.
