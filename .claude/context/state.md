<!-- assessed-at: ea284c0323113a46803001b3955ea0e6e4b00a9a -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.4.0 hardening toward (held) v1.0.0 — CI GREEN; CRAP `--fail-above` gate landed; one authorized gate left (`cargo-deny` supply-chain audit)

v0.4.0 is released across all registries; all 12 language bindings are functionally met and CI is
GREEN on the pushed tip (`cee130a`, runs `27759986239`/`27759984667`, overall `success`). Since the
last assessment the first of the two human-authorized hardening gates landed: the CRAP
`--fail-above` absolute gate is wired into the enforcing Phase 3 CRAP CI step, verified green, and
its `ci-cd.md` "verified when" box is `[x]`. One autonomous CID work package remains — the
`cargo-deny` supply-chain audit gate (authorized, spec'd, **not yet built**). v1.0.0 stays on hold
(stay on 0.4.x).

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
- **Perf gate — COMPLETE, ENFORCING, AND HARDENED** (unchanged this cycle).
    `crates/iscc-lib/benches/iai_benches.rs` is an `iai-callgrind` 0.16 instruction-count harness
    (11 `bench_*` fns → 16 parametrized cases). `[profile.bench] strip = false, debug = true`
    (Cargo.toml:61) preserves `__iai_callgrind_wrapper` toggle symbols. The `Perf (iai-callgrind)`
    job (no `continue-on-error`) installs valgrind + `iai-callgrind-runner@0.16.1`, runs benches
    (env `IAI_CALLGRIND_ALLOW_ASLR=true`), asserts non-zero collection, then runs the enforcing
    `Check perf regression` step (`python3 scripts/iai_regression.py --check`) which fails CI on
    \>10% Ir regression, zero-count benches, or a disappeared baselined bench vs the committed
    `.iai-baseline.json` (16 Ir entries, 10% tolerance, repo root). `--allow-missing` is a
    deliberate escape hatch; covered by 11 synthetic-fixture tests (`tests/test_iai_regression.py`).
    `rust-core.md` perf "verified when" boxes are `[x]`.
- **Semver gate present (informational, still unmet for v1.0.0)**: the
    `Semver (cargo-semver-checks)` job (`continue-on-error: true`) reports job-level `failure` (2
    expected breaking changes from the post-0.4.0 `pub(crate)` narrowing) but does NOT flip the run
    conclusion. The target's **enforcing** criterion stays unmet (`rust-core.md` semver "verified
    when" still `[ ]`) — flip `continue-on-error` off only at the v1.0.0 cut, which is held.
- Workspace version is `0.4.0`. The semver/v1.0.0 enforcement is the only remaining Rust Core gap;
    everything else met.

## Python Bindings

**Status**: met

- All symbols exported, both Python 3.10 and 3.14 CI jobs GREEN, ruff clean, streaming `SumHasher`
    wrapper present and exported.
- GIL release done (closed #39): `Python::detach` at all 7 call sites in
    `crates/iscc-py/src/lib.rs`; `tests/test_gil.py` adds 7 concurrency tests. 0 `allow_threads`
    remain.
- **PyO3 migration COMPLETE (closed #1)**: workspace `pyo3` pin is `0.29` with `abi3-py310`;
    `Cargo.lock` resolves a single `pyo3 0.29.0`, no older entries. Explicit
    `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697) preserved (PyO3 0.28 silently
    flipped that default `true`->`false`).
- Note: PyO3 advisory clearance was confirmed only by the mechanical lockfile proxy — exactly what
    the still-pending `cargo deny`/`cargo audit` gate will replace.

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
    (`scripts/iai_regression.py`, 248 lines, stdlib-only, hardened against zero-count and
    disappeared-bench false greens with an `--allow-missing` escape hatch; 11 fixture tests).
    Results upload as the `iai-baseline` artifact (`if: always()`).

## CI/CD and Publishing

**Status**: partially met — **CI GREEN on pushed tip; CRAP `--fail-above` landed; one authorized
gate (cargo-deny) not yet built**

- **LATEST PUSHED CI RUN — SUCCESS.** origin/develop tip `cee130a`; completed runs `27759986239` /
    `27759984667` (overall `success`); all functional jobs green (confirmed via check-runs API on
    the actual tip SHA), incl. `Perf (iai-callgrind)`, `Coverage + CRAP`, the `cargo-crap` action,
    and the enforcing Phase 3 CRAP gate (now with `--fail-above`). Only
    `Semver (cargo-semver-checks)` reports `failure` and it is `continue-on-error: true`
    (informational, does not flip the run). URL:
    https://github.com/iscc/iscc-lib/actions/runs/27759986239
- **HEAD is 1 commit ahead of origin/develop, log-only.** HEAD (`ea284c0`,
    `cid(log): iteration   113`) touches only `.claude/context/iterations.jsonl`. The green CI
    result reflects the current code.
- **CRAP `--fail-above` gate — DONE & GREEN (authorized issue #1 closed this cycle).** The enforcing
    Phase 3 CRAP step (ci.yml:392-393) now runs
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`.
    `--fail-above` is a boolean keyed off `.cargo-crap.toml threshold = 30.0` (no numeric arg); it
    closes the regression-only blind spot where a brand-new/renamed CC-heavy function reported
    `★ N new` and exited 0. Current max CRAP ~22.3 < 30, so the absolute gate passes on the current
    code — `Coverage + CRAP` is green. `.cargo-crap.toml` comment updated to describe the enforcing
    `--fail-above`; `ci-cd.md` "verified when" box (line 445) is `[x]`. Reviewed PASS (`cee130a`,
    Codex confirmed the YAML parses to the intended command).
- **`cargo-deny`/`cargo audit` supply-chain gate (authorized, UNIMPLEMENTED — last work package):**
    `ci-cd.md` (Audit row at line 33, "Supply chain — `cargo-deny`" section at line 139) mandates an
    Audit job running `cargo deny check` (advisories + bans + licenses) over the workspace via a
    root `deny.toml`, plus a `mise run audit` task (optionally `cargo audit` / `npm audit`).
    **Verified absent today:** no `deny.toml`, no Audit/deny job in `ci.yml`, no `audit`/`deny` task
    in `mise.toml`, neither tool installed. The `ci-cd.md` "verified when" box (line 448) is `[ ]`.
- **Perf gate ENFORCING + HARDENED** (no `continue-on-error`) — unchanged; green.
- v0.4.0 published; release workflow with 8 registry input toggles (crates.io, PyPI, npm, Maven,
    FFI, RubyGems, NuGet, Maven-Kotlin; Swift XCFramework built in `prepare-release`) + version sync
    (16 targets) in place.

## Open Issues (issues.md lists 3 — 0 critical, 1 normal, 2 low)

1. **Wire up `cargo deny`/`cargo audit` supply-chain gate** `normal` `[review]` — **AUTHORIZED** by
    Titusz (`9770332`), CID may implement autonomously. Add root `deny.toml`, a `Security audit` CI
    job (`cargo deny check` advisories + bans + licenses), a `mise run audit` task; install via
    `taiki-e/install-action` or `cargo binstall -y --force` (mind the `Swatinem/rust-cache`
    poisoning gotcha — `--force` is load-bearing). Optionally add `npm audit` for the napi package.

Low (human-directed, CID skips):

- **Release core as v1.0.0** — **held** by Titusz (2026-06-18): stay on 0.4.x, land the cargo-deny
    hardening gate first, then flip the `cargo-semver-checks` gate to enforcing as part of the
    eventual cut. Human-driven via `/release`; CID must not cut it autonomously.
- **Add programming language logos to docs site** — cosmetic.

## Next Milestone

CI is GREEN and the CRAP `--fail-above` gate has landed. **The single remaining autonomous work
package is the authorized `cargo-deny`/`cargo audit` supply-chain gate.** define-next should pick it
up:

- Add a workspace-root `deny.toml` (advisories + bans + licenses policy classes).
- Add a `Security audit` CI job running `cargo deny check`; install the tool carefully in CI
    (`taiki-e/install-action` or `cargo binstall -y --force` — the `--force` is load-bearing against
    the `Swatinem/rust-cache` poisoning gotcha; see learnings).
- Add a `mise run audit` task that reproduces `cargo deny check` locally. Optionally complement with
    `cargo audit` / `npm audit` (napi).
- Flip the `ci-cd.md` "verified when" box (line 448) only once the new CI job is confirmed green —
    `cargo-deny`/`cargo-audit` are NOT installed in the devcontainer, so the green CI job is the
    real confirmation (local verification limited to `deny.toml` parse + task wiring).

Do NOT cut v1.0.0 or flip the `Semver` gate to enforcing — both are deliberately held by Titusz
until after the cargo-deny gate lands.
