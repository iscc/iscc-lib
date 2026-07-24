<!-- assessed-at: 618bb15f62b810a506c898c5b548760888c1ba2e -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — CI GREEN; sole remaining CID-doable v0.6.0 target is the dependency refresh

v0.5.0 is released across all registries and all 12 language bindings meet their core criteria. This
iteration (123) resolved issue #49: `release.yml` now builds and import-tests a native-ARM
`manylinux_2_17_aarch64` Python wheel, closing the last aarch64 gap at the workflow level. **CI is
fully green (30/30 check-runs) on the develop tip.** The only remaining CID-doable v0.6.0 target is
the project-wide dependency review/refresh; everything else open is human-gated (v1.0.0 cut, npm
OIDC, release re-trigger bug) or cosmetic (docs logos).

## Rust Core Crate

**Status**: partially met — CI GREEN; only the held semver-enforcing / v1.0.0 criterion is unmet

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing — `Rust (fmt, clippy, test)` job GREEN.
- **Trailing-byte hardening intact (iter 121)**: `iscc_decode` (`crates/iscc-lib/src/lib.rs`) has
    both a "too short" (`tail.len() < nbytes`, ~line 235) and a "too long" (`tail.len() > nbytes`,
    ~line 241) rejection branch, so trailing padding errors instead of aliasing. ISCC-IDv1 is
    rejected earlier at the header level in the Rust core (`codec::Version` is V0-only) — ISCC-IDv1
    support remains Go-only.
- **CRAP regression RESOLVED (iter 122)**: the `Coverage + CRAP` job's `--fail-regression` gate is
    green after the `.crap-baseline.json` `iscc_decode` entry was refreshed to the new, legitimate
    complexity (crap 5.08, far below the 30.0 `--fail-above` cap). No source revert; the robustness
    fix stays. Root cause of the earlier RED: the CRAP regression gate is CI-only (not in
    `mise run check`/pre-commit), so a branch-adding fix slipped through green locally — recorded in
    learnings.md and agent memories as a discipline rule (refresh the baseline in the same step).
- **Perf gate — COMPLETE, ENFORCING**: `iai_benches.rs` (iai-callgrind 0.16, 11 `bench_*` → 16
    cases), `Perf (iai-callgrind)` job GREEN.
- **Semver gate present (informational)**: `Semver (cargo-semver-checks)` is
    `continue-on-error: true`, reports `success`; the enforcing v1.0.0 criterion (`rust-core.md`
    semver box `[ ]`) stays unmet — deliberately held by Titusz until the v1.0.0 cut. `decisions.md`
    (2026-07-24) records that tightening decode input-validation is not a SemVer break.
- Workspace version is `0.5.0`.

## Python Bindings

**Status**: met — aarch64 wheel matrix wired this iteration (#49 resolved)

- Core met: all symbols exported, Python 3.10 + 3.14 CI jobs GREEN, ruff clean, streaming
    `SumHasher` wrapper present. PyO3 pinned `0.29` (`abi3-py310`), single lockfile resolution.
- **GIL release COMPLETE for all heavyweight compute paths (issue #41 RESOLVED)**: `py.detach` wraps
    pure-Rust compute in all entry points (data/instance/image/sum + text/video + 3 streaming
    `update()`), **12 total detach sites**. Video detach opens strictly AFTER frame-signature
    extraction. Meta/audio/mixed stay attached by design.
- **aarch64 wheels wired (issue #49 DONE, iter 123)**: `.github/workflows/release.yml`
    `build-wheels` matrix now has a fourth entry (`ubuntu-24.04-arm` / `aarch64` / `python3.10`,
    native ARM runner — not QEMU), and `test-wheels` is matrixified to install/import-test both the
    x86_64 and aarch64 wheels before publish. Matches the `ci-cd.md` "Build Matrices" spec (lines
    285-300). NOTE: this build/test path runs only under a `workflow_dispatch` release with `pypi`
    selected — it is NOT exercised by CID pushes, so the first real aarch64 wheel builds/tests at
    the next v0.6.0 release (verification bar is static per next.md).

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

**Status**: partially met — **CI GREEN**; one CID-doable CI/CD gap remains (dependency freshness)

- **LATEST CI RUN — SUCCESS.** origin/develop tip `cbec132` (HEAD `618bb15` is a +1 log-only commit
    touching only `iterations.jsonl`). All **30 check-runs report `success`** — verified via
    `gh api repos/iscc/iscc-lib/commits/cbec132.../check-runs` (0 non-success). The iter-123
    `release.yml` aarch64 edit does not run under ci.yml (release-time only), so it did not alter
    the CI surface.
- **Python wheel matrix now covers aarch64 (iter 123)**: the `ubuntu-24.04-arm`/`aarch64` build +
    matrixed `test-wheels` are in `release.yml`, matching the target's "Linux x86_64 and aarch64"
    wheel criterion. First real aarch64 wheel ships at the v0.6.0 release.
- **cargo-deny gate enforcing** (unchanged): root `deny.toml`, `Audit (cargo-deny)` CI job GREEN.
    NOTE: a future live advisory can flip this red on any push with no code change (prefer
    `cargo update -p <crate>` over a `deny.toml` ignore when a patch exists).
- **Gap (v0.6.0, CI/CD)**: dependency freshness — no Dependabot/Renovate config
    (`.github/dependabot.yml`, `renovate.json` both absent, deliberately; refresh is a manual
    per-release pass). The "Dependency review and refresh" issue (`normal` `[human]`) remains open.
- v0.5.0 published; release workflow with per-registry toggles + version sync (16 targets) in place.
    Two release-workflow reliability issues remain open (npm OIDC migration, single-registry
    re-trigger bug) — both human-gated / `normal`.

## Open Issues (issues.md lists 5 — 0 critical, 3 normal, 2 low; all `[human]`)

CI is green — no open issue is CI-blocking. There is currently **no CID-actionable `[review]` or
`[audit]` issue**. Any open issue keeps the project IN_PROGRESS.

CID-doable now (`normal`, `[human]`, spec'd):

- Dependency review and refresh across the project (spec: `ci-cd.md` → "Dependency Freshness")

Release-workflow reliability (`normal`, `[human]`, human-gated):

- Migrate npm publishing to OIDC Trusted Publishing (needs npm-side trusted-publisher config first)
- Fix broken single-registry re-trigger in `release.yml`

Low (human-directed, CID skips):

- Release core as v1.0.0 — **held** by Titusz (stay on 0.5.x; flip Semver to enforcing at the cut).
- Add programming language logos to docs site — cosmetic.

## Next Milestone

**CI is green — no CI fix needed.** With #49 (aarch64 wheels) resolved, the sole remaining
CID-doable v0.6.0 target is the **project-wide dependency review/refresh** (root `Cargo.toml`/lock,
`pyproject.toml`/`uv.lock`, binding manifests: napi `package.json`, rb `Gemfile`/gemspec, jni
`pom.xml`, kotlin `build.gradle.kts`, dotnet `.csproj`, go `go.mod`; tooling pins: `mise.toml`,
`.pre-commit-config.yaml`, GHA versions). Patch/minor by default; evaluate majors individually and
document any deliberate hold-back next to its pin. Mind the documented pinning constraints: PyO3
bumps only with re-verifying `gil_used`/`py.detach` (#41); rb_sys must match the
`oxidize-rb/actions/cross-gem` Docker tag; wheels stay `abi3-py310`; quality-gate tool pins (e.g.
`cargo-crap`) bump together with their committed baselines. This spans many manifests and does NOT
cite an `[audit]` issue (no 8-file escape valve), so define-next should scope it as several smaller
per-ecosystem steps (Rust deps; Python deps; each binding-manifest group; tooling pins) rather than
one mega-diff.

Do NOT cut v1.0.0 or flip the `Semver` gate to enforcing — both deliberately held by Titusz. The npm
OIDC migration and single-registry re-trigger fixes stay human-gated. Guard: any source change that
adds a branch/loop to a covered function must refresh `.crap-baseline.json` in the same step (the
CRAP regression gate is CI-only, not in `mise run check`). Watch for the enforcing
`Audit (cargo-deny)` gate turning red on a fresh live advisory (prefer `cargo update -p <crate>`).
