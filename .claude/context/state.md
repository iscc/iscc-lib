<!-- assessed-at: 3912039a9631bb50c0649ffa8fa3e4c068ac7d10 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.4.0 hardening toward v1.0.0 stability commitment

v0.4.0 is released across all registries; all 12 language bindings are functionally met. Incremental
review since `150c778`: the **CRAP gate Phase 3** (enforcing `--fail-regression` against a committed
`.crap-baseline.json`) was implemented in commit `3912039` — the only code change. It is committed
**locally but not yet pushed**: HEAD is 4 commits ahead of `origin/develop` (`cbc0d14`), and the new
enforcing gate has **never run in CI**. The latest CI run is green but reflects only Phase 2.

## Rust Core Crate

**Status**: partially met

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing on the latest green CI run (`cbc0d14`).
- Reusable `pub struct SumHasher` in `streaming.rs` (`new()` / `update(&[u8])` /
    `finalize(bits, wide, add_units) -> SumCodeResult` / `Default`) runs the Data-Code and
    Instance-Code algorithms in a single pass; `gen_sum_code_v0` (lib.rs:997) drives it. Reachable
    as `iscc_lib::streaming::SumHasher` (module is `pub mod`) but intentionally **not** a crate-root
    Tier 1 re-export — only `DataHasher` / `InstanceHasher` are (`lib.rs:24`). It backs the core
    path-based `gen_sum_code_v0`, the Python `SumHasher` wrapper, and the WASM `SumHasher` class.
- Internal module visibility is narrowed: `lib.rs` declares
    `pub(crate) mod cdc / conformance / dct / minhash / simhash / utils / wtahash`; only `codec`,
    `streaming`, `types` are `pub mod`. All 10 crate-root `pub use` re-exports intact.
- **Semver gate present (informational)**: a `Semver (cargo-semver-checks)` CI job
    (`obi1kenobi/cargo-semver-checks-action@v2`, `package: iscc-lib`, `continue-on-error: true`,
    ci.yml:280) checks the public API against the last release. It currently *reports* 2 expected
    breaking changes (the post-0.4.0 `pub(crate)` narrowing) without failing the run. The target's
    **enforcing** criterion stays unmet — `rust-core.md` "verified when" still `[ ]` because it
    requires the gate to *fail* on unsanctioned breaks AND the crate to be >= 1.0.0. Flip
    `continue-on-error` off only at the v1.0.0 cut.
- **Gap (normal)**: no `iai-callgrind` instruction-count perf gate with committed baseline (> 10%
    regression must fail CI). This is now the only outstanding v1.0.0 CI quality gate that has no
    implementation at all.
- Workspace version is `0.4.0`; next target is v1.0.0 (stability-committed under strict SemVer).

## Python Bindings

**Status**: partially met

- Existing criteria met: all symbols exported, Python 3.10 + 3.14 CI jobs green, ruff clean,
    streaming `SumHasher` wrapper present (`__init__.py:349`, exported in `__all__` at line 402).
- GIL release done (closed #39): `py.allow_threads(...)` wraps the pure-Rust CPU-bound compute at
    all 7 call sites in `crates/iscc-py/src/lib.rs`. `tests/test_gil.py` adds 7 concurrency tests.
- **Gap (normal)**: PyO3 still pinned to `0.23` in root `Cargo.toml` (`workspace.dependencies`);
    issue requires incremental migration to `0.29.0` to clear two RustSec advisories shipped inside
    the wheel.

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols exported with TypeScript declarations, Node.js CI job green.
- Issue #38 RESOLVED: the `napi prepublish` step is removed from `release.yml`; source
    `crates/iscc-napi/package.json` uses the bundled model (`files: ["*.node"]`, no
    `optionalDependencies`). No open issues.

## WASM Bindings

**Status**: met

- All 32 Tier 1 symbols via `#[wasm_bindgen]`, WASM CI job green.
- Streaming `SumHasher` class present (lib.rs:533, finalize-once via `inner.take()`), 8
    `test_sum_hasher_*` tests pass under `wasm-pack test --node`; documented in
    `docs/howto/wasm.md`.

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
    `docs/howto/wasm.md` document the streaming `SumHasher`.
- **Gap (low, CID skips)**: language logos in `docs/index.md` and howto headers — cosmetic only.

## Benchmarks

**Status**: met (CI perf gate is a separate v1.0.0 item)

- Criterion benches for all 10 `gen_*_v0` (+2) functions, Bench (compile check) CI job green,
    pytest-benchmark 18 functions, speedup factors published (1.3x–158x) in docs/benchmarks.md.
- Note: the `iai-callgrind` *regression gate* (distinct from criterion local profiling) is tracked
    under Rust Core / CI/CD and remains unimplemented.

## CI/CD and Publishing

**Status**: partially met

- **LATEST CI RUN** — run 27675312942 (sha `cbc0d14` on `origin/develop`): **overall SUCCESS**. URL:
    https://github.com/iscc/iscc-lib/actions/runs/27675312942 — this is the most recent run and it
    reflects only Phase 2. No failing CI exists.
- **CRAP gate Phase 3 IMPLEMENTED LOCALLY (iter 97, commit `3912039`) — NOT YET CI-VERIFIED**: the
    `Coverage + CRAP` job gains a final enforcing step `CRAP regression gate`
    (`cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression`, ci.yml:335-336,
    NOT `continue-on-error`). A committed `.crap-baseline.json` sits at the repo root (envelope
    `{$schema, version: "0.2.2", entries:[...]}`, 97 entries across 10 `crates/iscc-lib/src/` files,
    NOT gitignored — only `lcov.info` + `crap.sarif` are). A `mise run crap:baseline`
    (`depends=["coverage"]`) task regenerates it. ci-cd.md Phase 3 checkbox flipped `[x]`; the
    rollout text now describes a reviewed-commit baseline refresh (not CI auto-commit). The
    report-only Phase 2 steps (GitHub annotations + SARIF upload) are unchanged.
- **PUSH/VERIFY GAP (top priority)**: HEAD (`3912039`) is **4 commits ahead of `origin/develop`
    (`cbc0d14`)**, all unpushed (`4840a40` update-state, `e1ad69e` define-next, `3912039` advance,
    plus the older `150c778` log commit). No CI run exists for any of them. The new enforcing CRAP
    regression gate has therefore **never executed in CI**, and there is a known cross-environment
    determinism risk: the baseline was generated in the devcontainer while CI regenerates coverage
    on `@stable`. If the first CI run flaps, the documented fix is to regenerate
    `.crap-baseline.json` from CI's `lcov` artifact — NOT to widen `--epsilon`.
- v0.4.0 published; release workflow with 8 registry input toggles (crates.io, PyPI, npm, Maven,
    FFI, RubyGems, NuGet, Maven-Kotlin; Swift XCFramework built in `prepare-release`) + version sync
    (16 targets) in place.
- **Gap (normal)**: no `iai-callgrind` perf-regression CI job with committed baseline — the last
    v1.0.0 CI gate with zero implementation.
- Note: `cargo-semver-checks` CI gate EXISTS (informational); becomes enforcing at v1.0.0.

## Open Issues (5 total — 0 critical, 3 normal, 2 low)

Normal (CID-actionable):

1. **Update PyO3 0.23 → 0.29** — security advisories in shipped wheel; incremental migration.
2. **Add Rust coverage + CRAP-metric quality gate** — all three phases now implemented locally
    (Phase 1 LCOV, Phase 2 report-only crap+SARIF, Phase 3 enforcing `--fail-regression`). Stays
    OPEN until the Phase 3 work is pushed, the enforcing gate runs green in CI, and the review
    agent verifies + deletes it.
3. **Add `iai-callgrind` performance-regression CI gate** — committed baseline, > 10% fails CI. Not
    started.

Low (human-directed, CID skips):

- **Release core as v1.0.0** — human-driven via `/release` skill; land perf gate + flip semver gate
    to enforcing first.
- **Add programming language logos to docs site** — cosmetic.

## Next Milestone

CI is green on the last pushed commit, but the just-completed CRAP Phase 3 work (a NEW enforcing CI
gate) is committed locally and unverified. The immediate priorities, in order:

1. **Push the Phase 3 work and confirm CI green.** The enforcing `CRAP regression gate` step has
    never run in CI. Because the baseline was captured in the devcontainer and CI regenerates
    coverage on `@stable`, the first run could flap on float noise; if it does, regenerate
    `.crap-baseline.json` from CI's `lcov` artifact (do not widen `--epsilon`). This must be
    verified before the CRAP issue can be closed.
2. **`iai-callgrind` perf-regression CI gate + committed baseline** — the last remaining v1.0.0
    stability gate, mirroring the reviewed-baseline pattern just established for CRAP.
    Deterministic instruction counts on shared runners; > 10% regression fails CI.
3. **Update PyO3 to 0.29** — clears two RustSec advisories shipped inside the published wheel; a
    six-minor-version jump with breaking changes per minor, so split it (start 0.23 → 0.24) and
    scope to `crates/iscc-py/` only (core has no PyO3 dep).

The two `low` issues (v1.0.0 release cut, docs logos) are human-directed and remain out of CID
scope. The semver gate becoming *enforcing* (drop `continue-on-error`) is a deliberate one-line
follow-up tied to the v1.0.0 cut — do not flip it before then.
