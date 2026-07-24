<!-- assessed-at: 9845cf58246ce2994ba4397f034b0c55eea45a3a -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — CI GREEN; remaining work is v0.6.0 (aarch64 wheels + dependency refresh)

v0.5.0 is released across all registries and all 12 language bindings meet their core criteria. The
iter-121/122 CI regression is **resolved**: iteration 122 refreshed the `.crap-baseline.json` entry
for `iscc_decode` (cyclomatic 4→5, coverage 80.95→85.19, crap 4.11→5.08), so the enforcing
`Coverage + CRAP` job is green while the iter-121 trailing-byte robustness fix stays in place. **CI
is now fully green (30/30 check-runs) on the develop tip.** No source changed since iter 121; only
the committed CRAP baseline moved. Remaining gaps are the v0.6.0 targets — aarch64 Python wheels and
a project-wide dependency refresh — plus human-held items (v1.0.0 cut, npm OIDC, release re-trigger
bug).

## Rust Core Crate

**Status**: partially met — CI GREEN; only the held semver-enforcing / v1.0.0 criterion is unmet

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing — `Rust (fmt, clippy, test)` job GREEN.
- **Trailing-byte hardening intact (iter 121)**: `iscc_decode` (`crates/iscc-lib/src/lib.rs`) has
    both a "too short" (`tail.len() < nbytes`, ~line 235) and a "too long" (`tail.len() > nbytes`,
    ~line 241) rejection branch, so trailing padding errors instead of aliasing; new test
    `test_..._rejects_trailing_bytes`. ISCC-IDv1 is rejected earlier at the header level in the Rust
    core (`codec::Version` is V0-only) — ISCC-IDv1 support remains Go-only.
- **CRAP regression RESOLVED (iter 122)**: the `Coverage + CRAP` job's `--fail-regression` gate is
    green again after the `.crap-baseline.json` `iscc_decode` entry was refreshed to the new,
    legitimate complexity (crap 5.08, far below the 30.0 `--fail-above` cap). No source revert; the
    robustness fix stays. Root cause of the earlier RED: the CRAP regression gate is CI-only (not in
    `mise run check`/pre-commit), so the branch-adding fix slipped through green locally — recorded
    in learnings.md and agent memories as a discipline rule (refresh the baseline in the same step).
- **Perf gate — COMPLETE, ENFORCING**: `iai_benches.rs` (iai-callgrind 0.16, 11 `bench_*` → 16
    cases), `Perf (iai-callgrind)` job GREEN.
- **Semver gate present (informational)**: `Semver (cargo-semver-checks)` is
    `continue-on-error:   true`, reports `success`; the enforcing v1.0.0 criterion (`rust-core.md`
    semver box `[ ]`) stays unmet — deliberately held by Titusz until the v1.0.0 cut. `decisions.md`
    (2026-07-24) records that tightening decode input-validation is not a SemVer break.
- Workspace version is `0.5.0`.

## Python Bindings

**Status**: partially met — one v0.6.0 target gap open (aarch64 wheels)

- Core met: all symbols exported, Python 3.10 + 3.14 CI jobs GREEN, ruff clean, streaming
    `SumHasher` wrapper present. PyO3 pinned `0.29` (`abi3-py310`), single lockfile resolution.
- **GIL release COMPLETE for all heavyweight compute paths (issue #41 RESOLVED)**: `py.detach` wraps
    pure-Rust compute in all entry points (data/instance/image/sum + text/video + 3 streaming
    `update()`), **12 total detach sites**. Video detach opens strictly AFTER frame-signature
    extraction. Meta/audio/mixed stay attached by design.
- **Gap (v0.6.0, issue #49)**: `linux/aarch64` (`manylinux_2_17_aarch64`) wheels are not built —
    only x86_64/universal2/win_amd64 ship. Target requires Linux x86_64 **and aarch64** wheels. Plan
    at `.claude/plans/restore-linux-aarch64-python-wheels.md`.

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
    unification — no `use blake3` in source, must not be pruned as "unused"). `simd128` RUSTFLAGS
    (ci.yml + release.yml) and `--enable-simd` wasm-opt flag remain in place.

## C FFI

**Status**: met

- cbindgen header committed + freshness check, C test passes, csbindgen generates
    `NativeMethods.g.cs`; C FFI CI job GREEN.

## Java Bindings

**Status**: met

- 32 Tier 1 symbols via JNI, native libs bundled in JAR; Java (Maven) CI job GREEN.

## Go Bindings

**Status**: met — ISCC-IDv1 (#43) landed iter 119, trailing-byte hardening landed iter 120

- Core met: pure Go (no CGO), all 32 Tier 1 symbols, `Go (go test, go vet)` CI job GREEN,
    `CGO_ENABLED=0` holds.
- **ISCC-IDv1 support DONE (issue #43)**: `packages/go/iscc_id.go` adds `EncodeIsccID` /
    `DecodeIsccID` + `IsccIDv1Result`, `codec.go` adds `VSV1` const and a MainType-ID-only Version=1
    relaxation in `decodeHeader`.
- **Trailing-byte hardening DONE (iter 120)**: `IsccDecode` (`codec.go`) has both a "too short"
    (~line 594) and a "too long" (~line 597) rejection branch; `DecodeIsccID` inherits via
    delegation; `IsccDecompose` untouched. Independent of the Rust-core fix (Go reimplements the
    codec natively, not via FFI).

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

**Status**: partially met — **CI GREEN**; two v0.6.0 CI/CD gaps remain

- **LATEST CI RUN — SUCCESS.** develop tip `e76a22b` (HEAD `9845cf5` is a +1 log-only commit;
    origin/develop == `e76a22b`). All **30 check-runs report `success`** — including the previously
    red `Coverage + CRAP (cargo llvm-cov + cargo crap)` job, now green after the iter-122 baseline
    refresh. Verified via `gh api repos/iscc/iscc-lib/commits/e76a22b.../check-runs` (0
    non-success).
- **cargo-deny gate enforcing** (unchanged): root `deny.toml`, `Audit (cargo-deny)` CI job GREEN.
    NOTE: a future live advisory can flip this red on any push with no code change (prefer
    `cargo update -p <crate>` over a `deny.toml` ignore when a patch exists).
- **Gap (v0.6.0, CI/CD)**: dependency freshness — no Dependabot/Renovate config
    (`.github/dependabot.yml`, `renovate.json` both absent).
- **Gap (v0.6.0, CI/CD)**: Python wheel matrix must cover aarch64 (see Python section, issue #49).
- v0.5.0 published; release workflow with per-registry toggles + version sync (16 targets) in place.
    Two release-workflow reliability issues remain open (npm OIDC migration, single-registry
    re-trigger bug) — both human-gated / `normal`.

## Open Issues (issues.md lists 6 — 0 critical, 4 normal, 2 low; all `[human]`)

CI is green — no open issue is CI-blocking. There is currently **no CID-actionable `[review]` or
`[audit]` issue**. Any open issue keeps the project IN_PROGRESS.

v0.6.0-scoped (`normal`, `[human]`, each with a spec — CID-doable):

- Restore linux/aarch64 Python wheels (#49) — plan at
    `.claude/plans/restore-linux-aarch64-python-wheels.md`
- Dependency review and refresh across the project

Release-workflow reliability (`normal`, `[human]`, human-gated):

- Migrate npm publishing to OIDC Trusted Publishing (needs npm-side trusted-publisher config first)
- Fix broken single-registry re-trigger in `release.yml`

Low (human-directed, CID skips):

- Release core as v1.0.0 — **held** by Titusz (stay on 0.5.x; flip Semver to enforcing at the cut).
- Add programming language logos to docs site — cosmetic.

## Next Milestone

**CI is green — no CI fix needed.** The immediate CID-doable v0.6.0 targets are:

1. **#49 — Restore linux/aarch64 Python wheels** (plan at
    `.claude/plans/restore-linux-aarch64-python-wheels.md`): add a native-ARM `ubuntu-24.04-arm`
    entry to the `build-wheels` matrix in `.github/workflows/release.yml` (not QEMU), verify the
    `before-script-linux` cp310 PATH prepend in the aarch64 manylinux container, and extend the
    wheel test job to install/import-test the aarch64 wheel on ARM before publish.
2. **Project-wide dependency review/refresh** (root `Cargo.toml`/lock, `pyproject.toml`/`uv.lock`,
    binding manifests, tooling pins): patch/minor by default, majors evaluated individually. Mind
    the documented pinning constraints — PyO3 bumps with re-verifying `gil_used`/`py.detach` (#41),
    rb_sys must match the cross-gem Docker tag, wheels stay `abi3-py310`, and quality-gate tool
    pins (e.g. cargo-crap) bump together with their baselines.

Do NOT cut v1.0.0 or flip the `Semver` gate to enforcing — both deliberately held by Titusz. The npm
OIDC migration and single-registry re-trigger fixes stay human-gated. Guard: any source change that
adds a branch/loop to a covered function must refresh `.crap-baseline.json` in the same step (the
CRAP regression gate is CI-only, not in `mise run check`). Watch for the enforcing
`Audit (cargo-deny)` gate turning red on a fresh live advisory (prefer `cargo update -p <crate>`).
