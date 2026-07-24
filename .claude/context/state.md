<!-- assessed-at: 1d28684830d665c028c15024fea237edef77c669 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — v0.6.0 backlog; Go trailing-byte fix DONE, Rust-core parallel gap + two feature packages + reliability fixes remain

v0.5.0 is released across all registries and all 12 language bindings meet their core criteria.
Iteration 120 landed the Go `IsccDecode` trailing-byte rejection (`packages/go/codec.go`), closing
the alias gap where `ISCC:MAIGHFECJMOPMIABAA` decoded identically to canonical
`ISCC:MAIGHFECJMOPMIAB`; the review agent then filed the **identical gap in the Rust core**
`iscc_decode` as a new `normal` `[review]` issue. **CI is fully GREEN on the pushed develop tip
(`1d28684`)** — all 21 check-runs `success`. The project stays IN_PROGRESS: one CID-actionable
robustness fix, two spec'd v0.6.0 feature packages, two release-reliability issues, and dependency
freshness remain open.

## Rust Core Crate

**Status**: partially met

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing — `Rust (fmt, clippy, test)` job GREEN.
- Reusable `SumHasher` in `streaming.rs` drives `gen_sum_code_v0` (single-pass Data+Instance);
    reachable as `iscc_lib::streaming::SumHasher`, intentionally not a Tier 1 re-export.
- **Perf gate — COMPLETE, ENFORCING, HARDENED** (unchanged): `iai_benches.rs` (iai-callgrind 0.16,
    11 `bench_*` → 16 cases), enforcing `Perf (iai-callgrind)` job
    (`scripts/iai_regression.py --check`, >10% Ir gate vs `.iai-baseline.json`), 11 fixture tests.
    Job GREEN.
- **Semver gate present (informational)**: `Semver (cargo-semver-checks)` job is
    `continue-on-error: true`; against the 0.5.0 baseline it reports `success`. The target's
    **enforcing** criterion for v1.0.0 stays unmet (`rust-core.md` semver box `[ ]`) — flip to
    enforcing only at the human-gated v1.0.0 cut, which is held.
- **NEW open robustness issue (`normal` `[review]`, NOT a target-gap)**: `iscc_decode`
    (`crates/iscc-lib/src/lib.rs`, line 234) checks only `tail.len() < nbytes` ("too short"), then
    copies `tail[..nbytes]` (line 245) and silently drops any trailing bytes — so
    `iscc_decode("ISCC:MAIGHFECJMOPMIABAA")` aliases the canonical form. Codec-wide (every
    MainType), pre-existing, inherited by all 11 delegating bindings. Verified still open (only the
    "too short" branch present). `iscc_decode` is Tier 1 but the fix is a signature-neutral
    stricter-input- validation change, not an API break. **Recommended next pick** (see Next
    Milestone).
- Workspace version is `0.5.0`. Aside from the new robustness issue, the only remaining Rust Core
    gap is the held v1.0.0 semver enforcement.

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
    x86_64/universal2/win_amd64 ship. Target requires Linux x86_64 **and aarch64** wheels. Plan at
    `.claude/plans/restore-linux-aarch64-python-wheels.md`.

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols with TypeScript declarations; Node.js CI job GREEN. Bundled model in
    `package.json`.

## WASM Bindings

**Status**: met — issue #42 (SIMD) resolved iteration 118, CI-verified

- All 32 Tier 1 symbols via `#[wasm_bindgen]`, streaming `SumHasher` class, `WASM (wasm-pack test)`
    job GREEN.
- **SIMD backend active (issue #42 DONE)**: `crates/iscc-wasm/Cargo.toml` carries a direct
    `blake3 = { workspace = true, features = ["wasm32_simd"] }` dep (exists solely for feature
    unification — no `use blake3` in source, must not be pruned as "unused"), making
    `Platform::detect()` return `WASM32_SIMD` on the wasm-only build. The landed
    `RUSTFLAGS=-C target-feature=+simd128` (ci.yml + release.yml) and `--enable-simd` wasm-opt flag
    remain in place. All four `specs/wasm-bindings.md` "WASM SIMD" boxes checked.

## C FFI

**Status**: met

- cbindgen header committed + freshness check, C test passes, csbindgen generates
    `NativeMethods.g.cs`; C FFI CI job GREEN.

## Java Bindings

**Status**: met

- 32 Tier 1 symbols via JNI, native libs bundled in JAR; Java (Maven) CI job GREEN.

## Go Bindings

**Status**: met — ISCC-IDv1 (#43) landed iter 119, trailing-byte hardening landed iter 120, both
CI-verified

- Core met: pure Go (no CGO), all 32 Tier 1 symbols, `Go (go test, go vet)` CI job GREEN,
    `CGO_ENABLED=0` holds.
- **ISCC-IDv1 support DONE (issue #43)**: `packages/go/iscc_id.go` adds `EncodeIsccID` /
    `DecodeIsccID` + `IsccIDv1Result`, `codec.go` adds `VSV1` const and a MainType-ID-only Version=1
    relaxation in `decodeHeader`. Verified against `ISCC:MAIGHFECJMOPMIAB` (realm 0/hub 1/ts
    1751831876325218). All 5 `specs/go-bindings.md` "verified when" boxes checked.
- **Trailing-byte hardening DONE (iter 120, prior `[review]` issue RESOLVED)**: `IsccDecode`
    (`codec.go`) now has both a "too short" (`len(tail) < nbytes`, line 594) and a NEW "too long"
    (`len(tail) > nbytes`, line 597) rejection branch, so `ISCC:MAIGHFECJMOPMIABAA` now errors
    instead of aliasing canonical. `DecodeIsccID` inherits the fix via delegation; `IsccDecompose`
    (own body loop) untouched. Two focused tests added; review verdict PASS. The equivalent gap in
    the Rust core `iscc_decode` was filed separately (see Rust Core section) and is independent (Go
    reimplements the codec natively, not via FFI).

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

- **LATEST CI RUN — SUCCESS.** origin/develop tip == HEAD == `1d28684`. All 21 check-runs (Rust, all
    12 bindings incl. Go, Coverage+CRAP, cargo-crap, Perf, **Audit (cargo-deny)**, Semver, Version
    consistency, WASM, Bench) report `success` via the check-runs API. Run:
    https://github.com/iscc/iscc-lib/actions/runs/30109824628
- **HEAD is pushed** — `git log origin/develop..HEAD` is empty (no unpushed log-only commit this
    time). The last three commits (`feat(cid)`/`docs(cid)`) are CID-loop infrastructure (audit role,
    metrics tooling, decisions.md, scope escape valve) — no product/target source changes; the only
    target-relevant source change since the previous assessment is the Go `codec.go` trailing-byte
    fix, which is CI-verified GREEN.
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

## Open Issues (issues.md lists 7 — 0 critical, 5 normal, 2 low)

No open issue is CI-blocking. Any open issue keeps the project IN_PROGRESS.

CID-actionable now:

- Rust core `iscc_decode` accepts trailing bytes (`normal`, `[review]`) — recommended next pick;
    concrete scoped fix, no human gating required. Same two-branch pattern as the just-landed Go
    fix.

v0.6.0-scoped (`normal`, `[human]`, each with a spec):

- Restore linux/aarch64 Python wheels (#49)
- Dependency review and refresh across the project

Release-workflow reliability (`normal`, `[human]`, human-gated):

- Migrate npm publishing to OIDC Trusted Publishing (needs npm-side trusted-publisher config first)
- Fix broken single-registry re-trigger in `release.yml`

Low (human-directed, CID skips):

- Release core as v1.0.0 — **held** by Titusz (stay on 0.5.x; flip Semver to enforcing at the cut).
- Add programming language logos to docs site — cosmetic.

## Next Milestone

**The Rust core `iscc_decode` trailing-byte hardening issue (`normal`, `[review]`) is the
recommended next pick.** It is concrete, well-scoped, CID-actionable (no human gating), and closes
the same robustness gap just fixed in Go — but for the stability-committed core and all 11 bindings
that delegate to it. Scope (mirroring the Go fix): add a `tail.len() > nbytes` rejection branch
after the existing "too short" check at `crates/iscc-lib/src/lib.rs:234` (keep the "too short"
message/test intact so `test_*` at ~line 1547 stays green), update the `iscc_decode` docstring
(currently only mentions "too short"), then `cargo test -p iscc-lib` + the conformance suite to
confirm no vendored vector relies on trailing padding, and verify the composite-decompose path (its
own body loop at ~line 955) is untouched.

After that: #49 (aarch64 wheels) and the project-wide dependency review/refresh are the remaining
CID-doable v0.6.0 targets (release-workflow verification is limited — release.yml only exercises on
a real release). The two release-reliability issues (npm OIDC, single-registry re-trigger) are
human-gated. Do NOT cut v1.0.0 or flip the `Semver` gate to enforcing — both deliberately held by
Titusz. Watch for the enforcing `Audit (cargo-deny)` gate turning red on a fresh live advisory
(prefer `cargo update -p <crate>` over a `deny.toml` ignore).
