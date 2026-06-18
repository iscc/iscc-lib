<!-- assessed-at: 40fa239e00e7564934a02cf48d0ca60d9b1addb1 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.4.0 hardening toward v1.0.0 — CI GREEN, PyO3 migration COMPLETE

v0.4.0 is released across all registries; all 12 language bindings are functionally met. **CI is
fully GREEN** — the latest run (sha `103fe3d`) passes, including the `Coverage + CRAP` job and its
enforcing Phase 3 gate. The PyO3 security migration is **finished** (pin now `0.29`, both targeted
RustSec advisories clear) and issue #1 is closed. Remaining v1.0.0 hardening: an `iai-callgrind`
perf-regression gate, CRAP gate hardening, and a `cargo deny`/`cargo audit` supply-chain gate. No
critical issues open.

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

**Status**: met

- All symbols exported, both Python 3.10 and 3.14 CI jobs GREEN, ruff clean, streaming `SumHasher`
    wrapper present and exported.
- GIL release done (closed #39): GIL-release wraps the pure-Rust CPU-bound compute at all 7 call
    sites in `crates/iscc-py/src/lib.rs` via the `Python::detach` API; `tests/test_gil.py` adds 7
    concurrency tests. 0 `allow_threads` remain.
- **PyO3 migration COMPLETE (closed #1)**: the workspace `pyo3` pin is `0.29` (`Cargo.toml:35`
    `version = "0.29"` with `abi3-py310`); `Cargo.lock` resolves a single `pyo3 0.29.0` with no
    older entries — the version where both targeted RustSec advisories shipped in the wheel clear.
    The 0.28→0.29 hop needed no source edits; `crates/iscc-py/src/lib.rs` is unchanged and the
    explicit `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697, documented) is
    preserved (PyO3 0.28 silently flipped that default `true`→`false`; the explicit setting keeps
    the raw-FFI `extract_frame_sigs` path free-threading-safe). 286 pytest pass.
- Note: the advisory clearance was confirmed only by the mechanical lockfile proxy (single
    `pyo3 0.29.0`, no older entries) — `cargo deny`/`cargo audit` are absent from both the
    devcontainer and CI. That tooling gap is now tracked as a separate `[review]` CI/CD issue, not a
    Python-binding defect.

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

- **LATEST CI RUN — SUCCESS.** Run 27728337913 (sha `103fe3d`, the review commit): **conclusion
    `success`**. URL: https://github.com/iscc/iscc-lib/actions/runs/27728337913 — all 17 functional
    jobs + the `Coverage + CRAP` job green; the only job-level `failure` is
    `Semver (cargo-semver-checks)`, which is `continue-on-error: true` (informational, does not flip
    the run). HEAD `40fa239` adds only one `iterations.jsonl` log commit on top of `103fe3d`, so
    this green run fully covers HEAD's code state.
- **Coverage + CRAP GREEN.** The `cargo-binstall` / `Swatinem/rust-cache` cache-poisoning flake was
    fixed earlier by adding `--force` to the install step (ci.yml:314
    `cargo binstall -y --force cargo-crap@0.2.2`). Verified on the latest run: the `Coverage + CRAP`
    job concludes `success` with the enforcing Phase 3 `CRAP regression gate` step running on every
    commit.
- **CRAP gate hardening gap (open, normal, [review])**: even when it runs, Phase 3 is
    regression-only — a brand-new/renamed function has no baseline entry, reports `★ N new`, and the
    step exits 0 (Codex verified a new uncovered CC=21 fn at CRAP 462 bypassed it). Fix: add
    `--fail-above 30` alongside `--fail-regression` (baseline max ~22.3 < 30). HUMAN REVIEW
    REQUESTED before mandating in the spec.
- **Supply-chain audit gap (open, normal, [review], NEW iter 105)**:
    `notes/07-security-versioning.md` mandates `cargo deny check` "Run in CI" (licenses + RustSec
    advisories + duplicate versions) via a workspace-root `deny.toml`, plus `cargo audit`. None
    exists — no `deny.toml`, no CI job, no `mise` task, neither tool installed in the devcontainer.
    This surfaced concretely during the PyO3 0.29 bump: advisory clearance could only be confirmed
    by a lockfile proxy, not an actual advisory scan. Fix: add `deny.toml`, a `Security audit` CI
    job, and a `mise run audit` task. HUMAN REVIEW REQUESTED (requirement currently lives only in
    design notes, not the CID specs).
- v0.4.0 published; release workflow with 8 registry input toggles (crates.io, PyPI, npm, Maven,
    FFI, RubyGems, NuGet, Maven-Kotlin; Swift XCFramework built in `prepare-release`) + version sync
    (16 targets) in place.
- **Gap (normal)**: no `iai-callgrind` perf-regression CI job with committed baseline.
- Note: `cargo-semver-checks` CI gate EXISTS (informational); becomes enforcing at v1.0.0.

## Open Issues (5 total — 0 critical, 3 normal, 2 low)

Normal (CID-actionable, but ALL THREE are constrained — see Next Milestone):

1. **CRAP gate does not fail on new high-CRAP functions** [review] — regression-only gate lets new
    uncovered high-complexity functions through; add `--fail-above 30`. HUMAN REVIEW REQUESTED.
2. **Wire up `cargo deny`/`cargo audit` supply-chain gate** [review] — design notes mandate it but
    no `deny.toml`/CI job/`mise` task exists; would have let the PyO3 bump tool-confirm advisories.
    HUMAN REVIEW REQUESTED (spec gap — requirement only in notes/07, not CID specs).
3. **Add `iai-callgrind` performance-regression CI gate** [human] — committed baseline, > 10% fails
    CI. Not started; blocked by valgrind being unavailable in the devcontainer.

Low (human-directed, CID skips):

- **Release core as v1.0.0** — human-driven via `/release` skill; land perf gate + flip semver gate
    to enforcing first.
- **Add programming language logos to docs site** — cosmetic.

## Next Milestone

**CI is GREEN and the PyO3 security migration arc (issue #1) is complete.** All three remaining
`normal` issues are constrained, so this is a natural pause point pending human direction:

1. **Wire up the `cargo deny`/`cargo audit` supply-chain gate** (review-sourced) — the most
    self-contained, mechanical work: add `deny.toml` at the workspace root, a `Security audit` CI
    job (`cargo deny check` advisories + bans + licenses), and a `mise run audit` task; install via
    `taiki-e/install-action` or `cargo binstall -y --force` (heed the rust-cache poisoning gotcha).
    This would also have let the PyO3 bump tool-confirm advisory clearance. **HUMAN REVIEW
    REQUESTED before amending the spec** — the requirement currently lives only in `notes/07`, not
    the CID specs.
2. **Harden the CRAP gate with `--fail-above 30`** (review-sourced) so new uncovered high-complexity
    functions also fail CI. Confirm `cargo-crap 0.2.2` accepts `--fail-above` and
    `--fail-regression` together; HUMAN REVIEW REQUESTED before amending the spec.
3. **`iai-callgrind` perf-regression CI gate + committed baseline** — the last v1.0.0 stability gate
    with zero implementation; currently blocked by valgrind being unavailable in the devcontainer.

The two `low` issues (v1.0.0 release cut, docs logos) are human-directed and out of CID scope. The
semver gate becoming *enforcing* is a deliberate one-line follow-up tied to the v1.0.0 cut — do not
flip it before then. define-next should not start v1.0.0 prep or flip the `Semver` gate
autonomously.
