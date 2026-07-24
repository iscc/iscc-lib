<!-- assessed-at: 4edfb0a164e3b190c1e9960fa5c52f57d868f83a -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — v0.6.0 backlog; #42 (WASM SIMD) attempted but incomplete (NEEDS_WORK)

v0.5.0 is released across all registries and all 12 language bindings meet their core criteria.
Since the last assessment, iteration 117 attempted issue #42 (enable WASM SIMD) — the `simd128`
RUSTFLAGS + `wasm-opt --enable-simd` flags landed, but review found them insufficient to activate
BLAKE3's SIMD backend, so #42 stays open (NEEDS_WORK, unpushed). **CI is green on the last pushed
develop tip (`2ffc8f9`); the #42 partial batch is unpushed and not yet CI-verified.** The project
stays IN_PROGRESS: four spec'd v0.6.0 work packages (one partially advanced) and two
release-workflow reliability issues remain open.

## Rust Core Crate

**Status**: partially met

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing — `Rust (fmt, clippy, test)` job GREEN.
- Reusable `SumHasher` in `streaming.rs` drives `gen_sum_code_v0` (single-pass Data+Instance);
    reachable as `iscc_lib::streaming::SumHasher`, intentionally not a Tier 1 re-export.
- **Perf gate — COMPLETE, ENFORCING, HARDENED** (unchanged): `iai_benches.rs` (iai-callgrind 0.16,
    11 `bench_*` → 16 cases), enforcing `Perf (iai-callgrind)` job
    (`scripts/iai_regression.py   --check`, >10% Ir gate vs `.iai-baseline.json`), 11 fixture tests.
    Job GREEN.
- **Semver gate present (informational)**: `Semver (cargo-semver-checks)` job is
    `continue-on-error: true`; against the 0.5.0 baseline it currently reports `success`. The
    target's **enforcing** criterion for v1.0.0 stays unmet (`rust-core.md` semver box `[ ]`) — flip
    to enforcing only at the human-gated v1.0.0 cut, which is held.
- Workspace version is `0.5.0`. Only remaining Rust Core gap is the held v1.0.0 semver enforcement.

## Python Bindings

**Status**: partially met — one v0.6.0 target gap open (aarch64 wheels)

- Core met: all symbols exported, Python 3.10 + 3.14 CI jobs GREEN, ruff clean, streaming
    `SumHasher` wrapper present. PyO3 pinned `0.29` (`abi3-py310`), single lockfile resolution.
- **GIL release COMPLETE for all heavyweight compute paths (issue #41 RESOLVED, unchanged)**:
    `py.detach` wraps the pure-Rust compute in all entry points (data/instance/image/sum +
    text/video
    - 3 streaming `update()`), **12 total detach sites**. Every video detach opens strictly AFTER
        frame-signature extraction. Meta/audio/mixed stay attached by design.
- **Gap (v0.6.0, issue #49)**: `linux/aarch64` (`manylinux_2_17_aarch64`) wheels are not built — the
    `build-wheels` matrix in `release.yml` was dropped around 0.2.0; only
    x86_64/universal2/win_amd64 ship. Target requires Linux x86_64 **and aarch64** wheels.

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols with TypeScript declarations; Node.js CI job GREEN. Bundled model in
    `package.json`.

## WASM Bindings

**Status**: partially met — v0.6.0 target gap open (#42, partially advanced this cycle)

- Core met: all 32 Tier 1 symbols via `#[wasm_bindgen]`, streaming `SumHasher` class,
    `WASM (wasm-pack test)` job GREEN (on `2ffc8f9`).
- **Gap (v0.6.0, issue #42) — attempted iter 117, review verdict NEEDS_WORK**: the SIMD flags LANDED
    but are insufficient. Committed (unpushed, `b5e3767`): `RUSTFLAGS: -C target-feature=+simd128`
    on the wasm CI test step (`ci.yml`) and the release build step (`release.yml`), plus
    `--enable-simd` in the `wasm-opt` array (`crates/iscc-wasm/Cargo.toml`). Under the locked
    `blake3 1.8.3` these do NOT activate BLAKE3's `wasm32` SIMD backend — that backend is gated
    behind the `blake3/wasm32_simd` **Cargo feature** (build.rs emits the `blake3_wasm32_simd` cfg
    only from `CARGO_FEATURE_WASM32_SIMD`), so `Platform::detect()` stays `Portable`. The observed
    `v128` opcodes are LLVM auto-vectorization of the portable path, not the SIMD backend.
- **Remaining fix (spec'd in handoff + issues.md review note)**: add a direct
    `blake3 = { workspace = true, features = ["wasm32_simd"] }` dep to `crates/iscc-wasm/Cargo.toml`
    (feature-unifies for the wasm-only build; keep the already-landed RUSTFLAGS + `--enable-simd`).
    Verify with a before/after `SumHasher` throughput measurement on a few-MB buffer — NOT `v128`
    opcode counting. The four `specs/wasm-bindings.md` "Verified when" boxes remain unchecked.

## C FFI

**Status**: met

- cbindgen header committed + freshness check, C test passes, csbindgen generates
    `NativeMethods.g.cs`; C FFI CI job GREEN.

## Java Bindings

**Status**: met

- 32 Tier 1 symbols via JNI, native libs bundled in JAR; Java (Maven) CI job GREEN.

## Go Bindings

**Status**: partially met — v0.6.0 target gap open

- Core met: pure Go (no CGO), all 32 Tier 1 symbols, `Go (go test, go vet)` CI job GREEN.
- **Gap (v0.6.0, issue #43)**: experimental ISCC-IDv1 not supported — `codec.go` hard-rejects any
    `Version > 0` (`iscc: invalid Version` at lines 269/438) and there are no `EncodeIsccID` /
    `DecodeIsccID` functions. Target requires `IsccDecode` to accept MainType ID Version 1 plus
    dedicated encode/decode exposing realm, hub-id, timestamp.

## Ruby Bindings

**Status**: met

- 32 Tier 1 symbols via Magnus; Ruby CI job GREEN; version synced.

## C# / .NET Bindings

**Status**: met

- 32 public symbols via P/Invoke over C FFI; C#/.NET CI job GREEN.

## C++ Bindings

**Status**: met

- C++17 header-only wrapper, all 32 Tier 1 symbols, ASAN clean, vcpkg + Conan; C++ CI job GREEN.

## UniFFI Scaffolding Crate

**Status**: complete (internal, not published)

- 32 `#[uniffi::export]` annotations; shared by Swift + Kotlin.

## Swift Bindings

**Status**: met

- SPM package with UniFFI bindings, all 32 Tier 1 symbols, XCFramework build; Swift CI job GREEN.
    XCFramework checksum updated for v0.5.0.

## Kotlin Bindings

**Status**: met

- packages/kotlin/ with JNA-loaded UniFFI bindings, 9 desktop+Android targets; Kotlin CI job GREEN.

## README

**Status**: met

- Polyglot README with CI + registry badges, per-language install/quick start for all 12 languages,
    architecture + MainTypes.

## Per-Crate READMEs

**Status**: met

- READMEs present for all 12 crates/packages; registry metadata references them.

## Documentation

**Status**: met (one low-priority cosmetic gap)

- Docs site, 11 language howto guides, tabbed examples, llms-full.txt, benchmarks page with speedup
    factors all present.
- **Gap (low, CID skips)**: language logos in `docs/index.md` / howto headers — cosmetic only.

## Benchmarks

**Status**: met

- Criterion benches for all 10 `gen_*_v0` (+2), Bench (compile check) CI job GREEN, 18
    pytest-benchmark functions, speedup factors published (1.3x-158x) in docs/benchmarks.md.
- Second iai-callgrind harness enforcing >10% Ir regression gate (green).

## CI/CD and Publishing

**Status**: partially met — **CI GREEN** on last pushed tip; two v0.6.0 target gaps remain

- **LATEST CI RUN — SUCCESS.** origin/develop tip == `2ffc8f9`. All check-runs (Rust, all 12
    bindings, Coverage+CRAP, cargo-crap, Perf, **Audit (cargo-deny)**, Semver, Version consistency)
    report `success` via the check-runs API.
- **Unpushed #42 batch not yet CI-verified**: `origin/develop..HEAD` is 6 commits (iter 116+117,
    NEEDS_WORK — no push). It includes the partial simd128 flags in `ci.yml`/`release.yml`/
    `Cargo.toml`. The review agent verified `wasm-pack test --node ... --features conformance`
    passes locally under `RUSTFLAGS=-C target-feature=+simd128` (conformance byte-identical) and
    `wasm-pack build` succeeds, so the flag is safe; but the CI jobs have NOT run on it. It will
    ride out with the #42 follow-up fix.
- **cargo-deny gate enforcing** (unchanged): root `deny.toml`, `Audit (cargo-deny)` CI job
    (`ci.yml`, `cargo-deny@0.19.9`, no `continue-on-error`), `mise run audit`. NOTE: a future
    live-advisory can flip this red on any push with no code change — normal, not a regression
    (prefer `cargo update -p <crate>` over a `deny.toml` ignore when a patch exists).
- **Gap (v0.6.0, CI/CD)**: dependency freshness — no Dependabot/Renovate config
    (`.github/dependabot.yml`, `renovate.json` both absent). Target requires no manifest/Action/tool
    pin to lag a major version without a documented hold-back.
- **Gap (v0.6.0, CI/CD)**: Python wheel matrix must cover aarch64 (see Python section, issue #49).
- CRAP gate (Phases 1-3 + `--fail-above`) and Perf gate both enforcing and green.
- v0.5.0 published; release workflow with per-registry toggles + version sync (16 targets) in place.
    Two release-workflow reliability issues remain open (npm OIDC migration, single-registry
    re-trigger bug) — both human-gated / `normal`.

## Open Issues (issues.md lists 8 — 0 critical, 6 normal, 2 low; all `[human]`)

No open issue is CI-blocking. Any open issue keeps the project IN_PROGRESS.

v0.6.0-scoped (`normal`, `[human]`, each with a spec):

1. Enable WASM `simd128` in the `@iscc/wasm` release build (#42) — **partially advanced (iter
    117)**; flags landed, blake3 SIMD backend still not active. Remaining fix =
    `blake3/wasm32_simd` feature.
2. Go bindings: experimental ISCC-IDv1 encode/decode (#43)
3. Restore linux/aarch64 Python wheels (#49)
4. Dependency review and refresh across the project

Release-workflow reliability (`normal`, `[human]`):

- Migrate npm publishing to OIDC Trusted Publishing
- Fix broken single-registry re-trigger in `release.yml`

Low (human-directed, CID skips):

- Release core as v1.0.0 — **held** by Titusz (stay on 0.5.x; flip Semver to enforcing at the cut).
- Add programming language logos to docs site — cosmetic.

## Next Milestone

**Finish issue #42 (WASM SIMD) — the recommended next pick, already partially advanced.** The
`RUSTFLAGS=-C target-feature=+simd128` + `wasm-opt --enable-simd` flags are already committed; the
precise remaining fix (from the iter-117 review + Codex, verified against the blake3 1.8.3 source)
is to add a direct `blake3 = { workspace = true, features = ["wasm32_simd"] }` dependency to
`crates/iscc-wasm/Cargo.toml`. Because features unify across the build graph and iscc-wasm only
compiles to `wasm32` (no other binding depends on it), this activates `blake3_wasm32_simd` for the
wasm build only, without touching native builds. Verify with a **before/after `SumHasher` throughput
measurement** on a few-MB buffer — NOT `v128` opcode counting (LLVM auto-vectorizes the portable
path too). Then the review agent can check the four `specs/wasm-bindings.md` "Verified when" boxes
and delete issue #42. Do NOT revert the already-landed flags — they remain required.

After #42: #43 (Go ISCC-IDv1), #49 (aarch64 wheels), dependency refresh, plus the two
release-workflow reliability fixes — one per iteration. Do NOT cut v1.0.0 or flip the `Semver` gate
to enforcing — both deliberately held by Titusz. Watch for the enforcing `Audit (cargo-deny)` gate
turning red on a fresh live advisory (prefer `cargo update -p <crate>` over a `deny.toml` ignore).
