<!-- assessed-at: 4ddaa0410b89eb48c5fe2af3290d29e98c64a31b -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 release; **CI back GREEN** (RUSTSEC-2026-0204 patched) — v0.6.0 backlog now open

v0.5.0 is released across all registries and all 12 language bindings meet their core criteria.
Since the last assessment the enforcing `cargo-deny` gate's fresh-advisory failure was fixed: the
advance agent bumped the dev-only `crossbeam-epoch` lockfile entry `0.9.18 → 0.9.20`, clearing
**RUSTSEC-2026-0204**. **CI is now fully green on the pushed develop tip (`cb7bed9`).** The project
remains IN_PROGRESS: five spec'd v0.6.0 work packages and two release-workflow reliability issues
are still open.

## Rust Core Crate

**Status**: partially met

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing — `Rust (fmt, clippy, test)` job GREEN.
- Reusable `SumHasher` in `streaming.rs` drives `gen_sum_code_v0` (single-pass Data+Instance);
    reachable as `iscc_lib::streaming::SumHasher`, intentionally not a Tier 1 re-export.
- **Perf gate — COMPLETE, ENFORCING, HARDENED** (unchanged): `iai_benches.rs` (iai-callgrind 0.16,
    11 `bench_*` → 16 cases), `[profile.bench] strip=false debug=true`, enforcing
    `Check perf regression` (`scripts/iai_regression.py --check`, >10% Ir gate vs
    `.iai-baseline.json`), 11 fixture tests. `Perf (iai-callgrind)` job GREEN. `rust-core.md` perf
    boxes `[x]`.
- **Semver gate present (informational)**: `Semver (cargo-semver-checks)` job is
    `continue-on-error: true`; against the 0.5.0 baseline it currently reports `success`. The
    target's **enforcing** criterion for v1.0.0 stays unmet (`rust-core.md` semver box `[ ]`) — flip
    to enforcing only at the human-gated v1.0.0 cut, which is held.
- Workspace version is `0.5.0`. Only remaining Rust Core gap is the held v1.0.0 semver enforcement.

## Python Bindings

**Status**: partially met — v0.6.0 target gaps open

- Core met: all symbols exported, Python 3.10 + 3.14 CI jobs GREEN, ruff clean, streaming
    `SumHasher` wrapper present. PyO3 pinned `0.29` (`abi3-py310`), single lockfile resolution.
- GIL release done for the heavyweight data/instance/image/sum paths (`py.detach` at the streaming
    hasher update sites, lib.rs:554/603/654).
- **Gap (v0.6.0, issue #41)**: GIL is NOT yet released for the text/video compute paths —
    `gen_text_code_v0` (lib.rs:136), `gen_video_code_v0` (lib.rs:178), `soft_hash_video_v0`
    (lib.rs:513) still hold the GIL for the full Rust compute. Target criterion requires `py.detach`
    around text/video compute (video detach must open strictly AFTER `extract_frame_sigs`).
- **Gap (v0.6.0, issue #49)**: `linux/aarch64` (`manylinux_2_17_aarch64`) wheels are not built — the
    `build-wheels` matrix in `release.yml` was dropped around 0.2.0; only
    x86_64/universal2/win_amd64 ship. Target now requires Linux x86_64 **and aarch64** wheels.

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols with TypeScript declarations; Node.js CI job GREEN. Issue #38 (napi
    prepublish) remains resolved; bundled model in `package.json`.

## WASM Bindings

**Status**: partially met — v0.6.0 target gap open

- Core met: all 32 Tier 1 symbols via `#[wasm_bindgen]`, streaming `SumHasher` class,
    `WASM (wasm-pack test)` job GREEN.
- **Gap (v0.6.0, issue #42)**: WASM SIMD is NOT enabled — no `simd128` / `enable-simd` anywhere in
    `crates/iscc-wasm/Cargo.toml`, `release.yml`, or `ci.yml`. Target now requires `simd128` target
    feature + `wasm-opt --enable-simd` in the published build (BLAKE3 SIMD backend).

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
    `DecodeIsccID` functions. Target now requires `IsccDecode` to accept MainType ID Version 1 plus
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

**Status**: partially met — **CI GREEN again**; two v0.6.0 target gaps remain

- **LATEST CI RUN — SUCCESS.** origin/develop tip == `cb7bed9`. All 22 check-runs (Rust, all 12
    bindings, Coverage+CRAP, cargo-crap, Perf, **Audit (cargo-deny)**, Semver, Version consistency)
    report `success` via the check-runs API. HEAD (`4ddaa04`) adds only a log-only `iteration 115`
    commit (unpushed) — no code delta, so the verified-green state holds.
- **RUSTSEC-2026-0204 RESOLVED**: the advance agent ran `cargo update -p crossbeam-epoch`, bumping
    the dev-only lockfile entry `0.9.18 → 0.9.20` (dev-only:
    `criterion → rayon → crossbeam-deque →   crossbeam-epoch`, never shipped). No `deny.toml`
    suppression added; `cargo deny check` now exits 0 and the `Audit (cargo-deny)` job is green.
    Clean root-cause fix — 1-package Cargo.lock diff.
- **cargo-deny gate fully built and enforcing**: root `deny.toml` (65 lines; advisories + licenses +
    bans + sources, config v2, 2 justified dev-only ignores), `Audit (cargo-deny)` CI job
    (ci.yml:395, `taiki-e/install-action` → `cargo-deny@0.19.9` → `cargo deny check`, no
    `continue-on-error`), `mise run audit` task (mise.toml:149). `ci-cd.md` Audit box `[x]`. NOTE: a
    future live-advisory can flip this red on any push with no code change — normal, not a
    regression.
- **Gap (v0.6.0, CI/CD)**: dependency freshness — no Dependabot/Renovate config
    (`.github/dependabot.yml`, `renovate.json` both absent). Target requires no manifest/Action/tool
    pin to lag a major version without a documented hold-back (issue: "Dependency review and
    refresh").
- **Gap (v0.6.0, CI/CD)**: Python wheel matrix must cover aarch64 (see Python section, issue #49).
- CRAP gate (Phases 1-3 + `--fail-above`) and Perf gate both enforcing and green.
- v0.5.0 published; release workflow with per-registry toggles + version sync (16 targets) in place.
    Two release-workflow reliability issues remain open (npm OIDC migration, single-registry
    re-trigger bug) — both human-gated / `normal`.

## Open Issues (issues.md lists 9 — 0 critical, 7 normal, 2 low; all `[human]`)

The RUSTSEC-2026-0204 CI blocker is resolved and was never filed as a formal issue. No open issue is
CI-blocking. Any open issue keeps the project IN_PROGRESS.

v0.6.0-scoped (`normal`, `[human]`, each with a spec):

1. Release GIL for text/video Python compute paths (#41)
2. Enable WASM `simd128` in the `@iscc/wasm` release build (#42)
3. Go bindings: experimental ISCC-IDv1 encode/decode (#43)
4. Restore linux/aarch64 Python wheels (#49)
5. Dependency review and refresh across the project

Release-workflow reliability (`normal`, `[human]`):

- Migrate npm publishing to OIDC Trusted Publishing
- Fix broken single-registry re-trigger in `release.yml`

Low (human-directed, CID skips):

- Release core as v1.0.0 — **held** by Titusz (stay on 0.5.x; flip Semver to enforcing at the cut).
- Add programming language logos to docs site — cosmetic.

## Next Milestone

**CI is green — start the v0.6.0 backlog.** With the supply-chain gate cleared, the actionable work
is the five spec'd `normal` v0.6.0 packages, one per iteration:

- **Recommended first pick: #41 (Python text/video GIL)** — most self-contained and lowest-risk: a
    single-file change in `crates/iscc-py/src/lib.rs` mirroring the already-landed
    data/instance/image/ sum `py.detach` pattern, conformance-neutral. Caveat: the video detach
    window must open strictly AFTER `extract_frame_sigs` (borrowed `PyList_GetItem` pointers are not
    free-threading-safe).
- Then #42 (WASM simd128), #43 (Go ISCC-IDv1), #49 (aarch64 wheels), dependency refresh — plus the
    two release-workflow reliability fixes as capacity allows.

Do NOT cut v1.0.0 or flip the `Semver` gate to enforcing — both deliberately held by Titusz. Watch
for the enforcing `Audit (cargo-deny)` gate turning red again on a fresh live advisory (prefer
`cargo update -p <crate>` to a patched release over a `deny.toml` ignore when a patch exists).
