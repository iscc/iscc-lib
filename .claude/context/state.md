<!-- assessed-at: a3db8acd20fd9cd72b7c6e9058f8f98a73a97606 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — v0.6.0 backlog; WASM SIMD (#42) DONE, three feature packages remain

v0.5.0 is released across all registries and all 12 language bindings meet their core criteria.
Iteration 118 finished issue #42 (WASM SIMD): a direct `blake3 = { features = ["wasm32_simd"] }`
dependency on `iscc-wasm` now activates BLAKE3's `wasm32` SIMD backend, verified via cargo-tree
feature graph + conformance + a 1993→5370 `v128`-opcode jump. **CI is GREEN on the pushed develop
tip (`6571c1b`)**, including the SIMD change. The project stays IN_PROGRESS: three spec'd v0.6.0
feature packages and two release-workflow reliability issues remain open.

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
    text/video + 3 streaming `update()`), **12 total detach sites**. Every video detach opens
    strictly AFTER frame-signature extraction. Meta/audio/mixed stay attached by design.
- **Gap (v0.6.0, issue #49)**: `linux/aarch64` (`manylinux_2_17_aarch64`) wheels are not built — the
    `build-wheels` matrix in `release.yml` was dropped around 0.2.0; only
    x86_64/universal2/win_amd64 ship. Target requires Linux x86_64 **and aarch64** wheels.

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols with TypeScript declarations; Node.js CI job GREEN. Bundled model in
    `package.json`.

## WASM Bindings

**Status**: met — issue #42 (SIMD) resolved iteration 118, CI-verified

- All 32 Tier 1 symbols via `#[wasm_bindgen]`, streaming `SumHasher` class, `WASM (wasm-pack test)`
    job GREEN on the pushed tip `6571c1b`.
- **SIMD backend now active (issue #42 DONE)**: `crates/iscc-wasm/Cargo.toml` carries a direct
    `blake3 = { workspace = true, features = ["wasm32_simd"] }` dep (exists solely for feature
    unification — no `use blake3` in source, must not be pruned as "unused"). This feature-unifies
    onto iscc-lib's blake3 for the wasm-only build, making blake3's build.rs emit the gating
    `blake3_wasm32_simd` cfg so `Platform::detect()` returns `WASM32_SIMD`. The already-landed
    `RUSTFLAGS=-C target-feature=+simd128` (ci.yml + release.yml) and `--enable-simd` wasm-opt flag
    remain in place (broaden simd128 across the crate + let wasm-opt accept `v128`). Verified via
    cargo-tree feature graph, byte-identical conformance on the SIMD build, and a 1993→5370
    `v128`-opcode jump in the released `.wasm`. Native builds unaffected (build.rs skips the cfg
    off-wasm). All four `specs/wasm-bindings.md` "WASM SIMD" boxes are checked.

## C FFI

**Status**: met

- cbindgen header committed + freshness check, C test passes, csbindgen generates
    `NativeMethods.g.cs`; C FFI CI job GREEN.

## Java Bindings

**Status**: met

- 32 Tier 1 symbols via JNI, native libs bundled in JAR; Java (Maven) CI job GREEN.

## Go Bindings

**Status**: partially met — v0.6.0 target gap open (recommended next pick)

- Core met: pure Go (no CGO), all 32 Tier 1 symbols, `Go (go test, go vet)` CI job GREEN.
- **Gap (v0.6.0, issue #43)**: experimental ISCC-IDv1 not supported — `codec.go` hard-rejects any
    `Version > 0` (`iscc: invalid Version` at lines 269/438) and there are no `EncodeIsccID` /
    `DecodeIsccID` functions. Target requires `IsccDecode` to accept MainType ID Version 1 plus
    dedicated encode/decode exposing realm, hub-id, timestamp. Known conformance vector:
    `ISCC:MAIGHFECJMOPMIAB` → realm 0, hub_id 1, timestamp 1751831876325218. Unblocks iscc-monitor's
    ADR-0011.

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

**Status**: partially met — **CI GREEN** on pushed tip; two v0.6.0 target gaps remain

- **LATEST CI RUN — SUCCESS.** origin/develop tip == `6571c1b` (iter-118 review commit). All
    check-runs (Rust, all 12 bindings, Coverage+CRAP, cargo-crap, Perf, **Audit (cargo-deny)**,
    Semver, Version consistency, WASM incl. the SIMD change) report `success` via the check-runs
    API.
- **HEAD (`a3db8ac`) is a +1 log-only commit, unpushed** (`origin/develop..HEAD` = 1 commit,
    `cid(log): iteration 118`). No source changes ride on it — nothing to CI-verify.
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

## Open Issues (issues.md lists 7 — 0 critical, 5 normal, 2 low; all `[human]`)

No open issue is CI-blocking. Any open issue keeps the project IN_PROGRESS.

v0.6.0-scoped (`normal`, `[human]`, each with a spec):

1. Go bindings: experimental ISCC-IDv1 encode/decode (#43) — recommended next pick
2. Restore linux/aarch64 Python wheels (#49)
3. Dependency review and refresh across the project

Release-workflow reliability (`normal`, `[human]`):

- Migrate npm publishing to OIDC Trusted Publishing
- Fix broken single-registry re-trigger in `release.yml`

Low (human-directed, CID skips):

- Release core as v1.0.0 — **held** by Titusz (stay on 0.5.x; flip Semver to enforcing at the cut).
- Add programming language logos to docs site — cosmetic.

## Next Milestone

**Issue #43 (Go bindings ISCC-IDv1) is the recommended next pick.** It is a concrete, spec'd feature
(`specs/go-bindings.md` → "ISCC-IDv1 Support (Experimental)") with a known conformance vector
(`ISCC:MAIGHFECJMOPMIAB` → realm 0, hub_id 1, timestamp 1751831876325218), and it unblocks
`iscc/iscc-monitor` deleting its interim in-repo codec port (their ADR-0011). Scope: add
`EncodeIsccID` / `DecodeIsccID` to `packages/go`, accept `Version = 1` for MainType ID only in
`decodeHeader` (`codec.go` lines 269/438), and mark the API experimental (ISCC-IDv1 is not part of
ISO 24138) — at parity with iscc-core's `iscc_id.py`.

After #43: #49 (aarch64 wheels), dependency review/refresh, plus the two release-workflow
reliability fixes — one per iteration. Do NOT cut v1.0.0 or flip the `Semver` gate to enforcing —
both deliberately held by Titusz. Watch for the enforcing `Audit (cargo-deny)` gate turning red on a
fresh live advisory (prefer `cargo update -p <crate>` over a `deny.toml` ignore).
