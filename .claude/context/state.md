<!-- assessed-at: 77a5fb36e213531e65c86a6666dee93fd5ad0225 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.4.0 hardening toward v1.0.0 — CI GREEN; iai-callgrind Perf job runs in CI (regression gate pending)

v0.4.0 is released across all registries; all 12 language bindings are functionally met. **CI is
GREEN** on the pushed tip (`55ff177`, run 27742285656), including the new `Perf (iai-callgrind)`
job, the `Coverage + CRAP` job, and its enforcing Phase 3 gate. This iteration's predecessor landed
the CI `Perf` job that now runs the iai-callgrind benches under valgrind and uploads a baseline
artifact — but the *committed* baseline + >10% regression gate (slice 2b) is still pending, so the
perf gate is not yet enforcing. No critical issues open.

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
- **Perf gate (normal) — harness + CI job done, regression gate still missing**:
    `crates/iscc-lib/benches/iai_benches.rs` is an `iai-callgrind` 0.16 instruction-count harness
    (11 benches: 9 `gen_*_v0` hot paths + `alg_cdc_chunks` + `alg_minhash_256`). The CI
    `Perf   (iai-callgrind)` job now installs valgrind + `iai-callgrind-runner@0.16.1`, runs
    `cargo bench -p iscc-lib --bench iai_benches`, and uploads `target/iai/` as `iai-baseline` —
    **GREEN on the pushed tip**. A `bench:iai` mise task exists (mise.toml:126). Still missing
    (slice 2b): a **committed** baseline file and a **>10% regression gate** that fails CI. Issue #3
    stays open until slice 2b lands.
- Workspace version is `0.4.0`; next target is v1.0.0 (stability-committed under strict SemVer).

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
    preserved (PyO3 0.28 silently flipped that default `true`→`false`; the explicit setting keeps
    the raw-FFI `extract_frame_sigs` path free-threading-safe).
- Note: the advisory clearance was confirmed only by the mechanical lockfile proxy (single
    `pyo3 0.29.0`, no older entries) — `cargo deny`/`cargo audit` are absent from both the
    devcontainer and CI. That tooling gap is tracked as a separate `[review]` CI/CD issue.

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

**Status**: met (CI perf *regression gate* is a separate v1.0.0 item)

- Criterion benches for all 10 `gen_*_v0` (+2) functions, Bench (compile check) CI job GREEN,
    pytest-benchmark 18 functions, speedup factors published (1.3x–158x) in docs/benchmarks.md.
- A second bench harness `iai_benches.rs` (iai-callgrind 0.16, 11 benches) sits alongside criterion
    `benchmarks.rs`. As of the pushed tip it is **run in CI** by the `Perf (iai-callgrind)` job
    under valgrind and the results are uploaded as the `iai-baseline` artifact (job GREEN). Still
    missing: a **committed** baseline and the >10% **regression gate** — that slice (issue #3, slice
    2b) is not yet implemented and remains tracked under Rust Core / CI/CD.

## CI/CD and Publishing

**Status**: partially met — **CI GREEN on pushed tip**

- **LATEST PUSHED CI RUN — SUCCESS.** Run 27742285656 on sha `55ff177` (= `origin/develop` tip):
    confirmed `success` via the check-runs/check-suites API — all functional jobs green, including
    the **new `Perf (iai-callgrind)` job**, the `Coverage + CRAP` job, and the enforcing Phase 3
    gate. The only job-level `failure` is `Semver (cargo-semver-checks)`, which is
    `continue-on-error: true` (informational, does not flip the run). URL:
    https://github.com/iscc/iscc-lib/actions/runs/27742285656
- **`Perf (iai-callgrind)` job ADDED and GREEN (ci.yml).** Steps: apt valgrind → cargo-binstall →
    `cargo binstall -y --force iai-callgrind-runner@0.16.1` →
    `cargo bench -p iscc-lib --bench iai_benches` → upload `target/iai/` as `iai-baseline`. No
    `needs:`, no `continue-on-error`. The CI-only criterion deferred at review time (job concludes
    success + uploads a usable artifact under valgrind) is now **confirmed**. This completes slice
    2a of issue #3; the regression gate (slice 2b) is the remaining work.
- **HEAD is 1 commit ahead of origin/develop, code-clean.** HEAD (`77a5fb3`) is the iteration-107
    log commit (`iterations.jsonl` only); origin/develop is `55ff177`. No code is unpushed/
    unverified — the CI result on `55ff177` reflects the current code.
- **Coverage + CRAP GREEN.** The earlier `cargo-binstall` / `Swatinem/rust-cache` cache-poisoning
    flake was fixed via `--force` on the install step (ci.yml:314
    `cargo binstall -y --force cargo-crap@0.2.2`). The enforcing Phase 3 `CRAP regression gate` runs
    on every commit and concludes `success`.
- **CRAP gate hardening gap (open, normal, [review])**: Phase 3 is regression-only — a
    brand-new/renamed function has no baseline entry, reports `★ N new`, and the step exits 0 (Codex
    verified a new uncovered CC=21 fn at CRAP 462 bypassed it). Fix: add `--fail-above 30` alongside
    `--fail-regression` (baseline max ~22.3 < 30). HUMAN REVIEW REQUESTED before mandating in spec.
- **Supply-chain audit gap (open, normal, [review])**: `notes/07-security-versioning.md` mandates
    `cargo deny check` "Run in CI" (licenses + RustSec advisories + duplicate versions) via a
    workspace-root `deny.toml`, plus `cargo audit`. None exists — no `deny.toml`, no CI job, no
    `mise` task, neither tool installed in the devcontainer. Surfaced concretely during the PyO3
    0.29 bump (clearance confirmable only by a lockfile proxy). HUMAN REVIEW REQUESTED (requirement
    lives only in design notes, not the CID specs).
- **Perf regression gate gap (open, normal)**: the `Perf` job now runs the benches and uploads a
    baseline artifact, but there is no **committed** baseline and no **>10% regression** failure
    condition yet (issue #3, slice 2b).
- v0.4.0 published; release workflow with 8 registry input toggles (crates.io, PyPI, npm, Maven,
    FFI, RubyGems, NuGet, Maven-Kotlin; Swift XCFramework built in `prepare-release`) + version sync
    (16 targets) in place.

## Open Issues (5 total — 0 critical, 3 normal, 2 low)

Normal (CID-actionable, but all three are constrained — see Next Milestone):

1. **CRAP gate does not fail on new high-CRAP functions** [review] — regression-only gate lets new
    uncovered high-complexity functions through; add `--fail-above 30`. HUMAN REVIEW REQUESTED.
2. **Wire up `cargo deny`/`cargo audit` supply-chain gate** [review] — design notes mandate it but
    no `deny.toml`/CI job/`mise` task exists. HUMAN REVIEW REQUESTED (spec gap — requirement only
    in notes/07, not CID specs).
3. **Add `iai-callgrind` performance-regression CI gate** [human] — *CI Perf job landed and is GREEN
    this iteration*; remaining work (slice 2b) is the **committed baseline** + the **>10%
    regression gate** that fails CI, plus a baseline-refresh `mise` task. Verifiable only via CI
    (valgrind is absent in the devcontainer).

Low (human-directed, CID skips):

- **Release core as v1.0.0** — human-driven via `/release` skill; land perf regression gate + flip
    semver gate to enforcing first.
- **Add programming language logos to docs site** — cosmetic.

## Next Milestone

**CI is GREEN on the pushed tip and the iai-callgrind `Perf` job now runs under valgrind in CI.**
The obvious next slice closes the perf gate; the other two normal issues are review-gated:

1. **Finish the `iai-callgrind` perf gate (slice 2b)** — inspect the uploaded `iai-baseline`
    artifact to learn the on-disk layout/summary format, commit a known-good baseline file, add the
    > 10% instruction-count **regression failure** condition (via `LibraryBenchmarkConfig` tolerance
    > or an iai-callgrind `--baseline`/`--fail-*` flow), and add a deliberate `bench:iai:baseline`
    > refresh `mise` task. This is the slice that flips `rust-core.md` / `ci-cd.md` perf-gate
    > checkboxes and lets issue #3 close. Verifiable only via CI (no valgrind in the devcontainer);
    > the `Perf` job already has no `continue-on-error`, so it will enforce once the gate exists.
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
