<!-- assessed-at: dd843c56476f162ca81e85661e6d6ff86b980f84 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — CI GREEN; dependency refresh underway (Rust Cargo.lock slice done)

v0.5.0 is released across all registries and all 12 language bindings meet their core criteria. This
iteration (124) landed **slice 1 of the dependency refresh**: `Cargo.lock` was regenerated via
`cargo update` (~100 transitive crates to latest semver-compatible versions; all `Cargo.toml` pins
held; no source changes). **CI is fully green on the develop tip.** The remaining CID-doable v0.6.0
work is the rest of the dependency refresh (Rust direct-pin evaluation, Python `uv.lock`,
per-binding manifests, tooling pins). Everything else open is human-gated (v1.0.0 cut, npm OIDC,
release re-trigger bug) or cosmetic (docs logos).

## Rust Core Crate

**Status**: partially met — CI GREEN; only the held semver-enforcing / v1.0.0 criterion is unmet

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing — `Rust (fmt, clippy, test)` job GREEN.
- **Cargo.lock refreshed (iter 124, dependency-refresh slice 1)**: `cargo update` bumped ~100
    transitive crates to latest semver-compatible versions; no `Cargo.toml` direct pins changed and
    no source touched. All quality gates (Rust, Coverage+CRAP, Perf, Audit, Semver) stayed GREEN on
    the refreshed lockfile.
- **Trailing-byte hardening intact (iter 121)**: `iscc_decode` (`crates/iscc-lib/src/lib.rs`) has
    both a "too short" (`tail.len() < nbytes`, ~line 235) and a "too long" (`tail.len() > nbytes`,
    ~line 241) rejection branch, so trailing padding errors instead of aliasing. ISCC-IDv1 is
    rejected earlier at the header level in the Rust core (`codec::Version` is V0-only) — ISCC-IDv1
    support remains Go-only.
- **CRAP regression gate green (iter 122 baseline refresh holds)**: the `Coverage + CRAP` job's
    `--fail-regression` gate remains green; `.crap-baseline.json` `iscc_decode` entry reflects the
    legitimate post-hardening complexity (crap 5.08, far below the 30.0 `--fail-above` cap). NOTE:
    the CRAP regression gate is CI-only (not in `mise run check`/pre-commit) — any source change
    adding a branch/loop to a covered function must refresh the baseline in the same step.
- **Perf gate — COMPLETE, ENFORCING**: `iai_benches.rs` (iai-callgrind 0.16, 11 `bench_*` → 16
    cases), `Perf (iai-callgrind)` job GREEN.
- **Semver gate present (informational)**: `Semver (cargo-semver-checks)` is
    `continue-on-error: true`, reports `success`; the enforcing v1.0.0 criterion (`rust-core.md`
    semver box `[ ]`) stays unmet — deliberately held by Titusz until the v1.0.0 cut. `decisions.md`
    (2026-07-24) records that tightening decode input-validation is not a SemVer break.
- Workspace version is `0.5.0`.

## Python Bindings

**Status**: met — aarch64 wheel matrix wired iter 123 (#49 resolved)

- Core met: all symbols exported, Python 3.10 + 3.14 CI jobs GREEN, ruff clean, streaming
    `SumHasher` wrapper present. PyO3 pinned `0.29` (`abi3-py310`), single lockfile resolution.
- **GIL release COMPLETE for all heavyweight compute paths (issue #41 RESOLVED)**: `py.detach` wraps
    pure-Rust compute in all entry points (data/instance/image/sum + text/video + 3 streaming
    `update()`), **12 total detach sites**. Video detach opens strictly AFTER frame-signature
    extraction. Meta/audio/mixed stay attached by design.
- **aarch64 wheels wired (issue #49 DONE, iter 123)**: `.github/workflows/release.yml`
    `build-wheels` matrix has a fourth entry (`ubuntu-24.04-arm` / `aarch64` / `python3.10`, native
    ARM runner — not QEMU), and `test-wheels` is matrixified to install/import-test both the x86_64
    and aarch64 wheels before publish. Matches the `ci-cd.md` "Build Matrices" spec (lines 285-300).
    NOTE: this build/test path runs only under a `workflow_dispatch` release with `pypi` selected —
    NOT exercised by CID pushes, so the first real aarch64 wheel builds/tests at the next v0.6.0
    release. Python `uv.lock` refresh is still pending (dependency-refresh remaining slice).

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols with TypeScript declarations; Node.js CI job GREEN. Bundled model in
    `package.json`. napi `package.json` dependency refresh still pending (remaining slice).

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

**Status**: partially met — **CI GREEN**; one CID-doable CI/CD gap remains (dependency freshness, in
progress)

- **LATEST CI RUN — SUCCESS.** origin/develop tip `a5eb561` (HEAD `dd843c5` is a +1 log-only commit
    touching only `iterations.jsonl`, unpushed). All check-runs report `success` (41 check-runs
    incl. re-run duplicates, 0 non-success) — verified via
    `gh api repos/iscc/iscc-lib/commits/a5eb561.../check-runs`. The Cargo.lock refresh did not
    regress any gate.
- **Dependency refresh — slice 1 (Rust Cargo.lock) DONE (iter 124)**: `cargo update` regenerated the
    lockfile; all `Cargo.toml` direct pins held; all gates green. Remaining slices (still open under
    the "Dependency review and refresh" `normal` `[human]` issue): Rust direct-pin major evaluation
    (document hold-backs for uniffi 0.32 / pyo3 #41 / criterion / iai-callgrind / magnus / jni /
    napi), Python `uv.lock`, per-binding manifests (napi `package.json`, rb `Gemfile`/gemspec, jni
    `pom.xml`, kotlin `build.gradle.kts`, dotnet `.csproj`, go `go.mod`), and tooling pins
    (`mise.toml`, `.pre-commit-config.yaml`, GHA versions).
- **Python wheel matrix covers aarch64 (iter 123)**: `ubuntu-24.04-arm`/`aarch64` build + matrixed
    `test-wheels` in `release.yml`, matching the target's "Linux x86_64 and aarch64" wheel
    criterion. First real aarch64 wheel ships at the v0.6.0 release.
- **cargo-deny gate enforcing** (unchanged): root `deny.toml`, `Audit (cargo-deny)` CI job GREEN.
    NOTE: a future live advisory can flip this red on any push with no code change (prefer
    `cargo update -p <crate>` over a `deny.toml` ignore when a patch exists).
- **Gap (v0.6.0, CI/CD)**: no automated dependency updates — `.github/dependabot.yml` /
    `renovate.json` both absent (deliberately; refresh is a manual per-release pass). The
    "Dependency review and refresh" issue (`normal` `[human]`) stays open until all slices land.
- v0.5.0 published; release workflow with per-registry toggles + version sync (16 targets) in place.
    Two release-workflow reliability issues remain open (npm OIDC migration, single-registry
    re-trigger bug) — both human-gated / `normal`.

## Open Issues (issues.md lists 5 — 0 critical, 3 normal, 2 low; all `[human]`)

CI is green — no open issue is CI-blocking. There is currently **no CID-actionable `[review]` or
`[audit]` issue**. Any open issue keeps the project IN_PROGRESS.

CID-doable now (`normal`, `[human]`, spec'd):

- Dependency review and refresh across the project (spec: `ci-cd.md` → "Dependency Freshness").
    **Slice 1 (Rust Cargo.lock) DONE iter 124**; remaining slices listed above.

Release-workflow reliability (`normal`, `[human]`, human-gated):

- Migrate npm publishing to OIDC Trusted Publishing (needs npm-side trusted-publisher config first)
- Fix broken single-registry re-trigger in `release.yml`

Low (human-directed, CID skips):

- Release core as v1.0.0 — **held** by Titusz (stay on 0.5.x; flip Semver to enforcing at the cut).
- Add programming language logos to docs site — cosmetic.

## Next Milestone

**CI is green — no CI fix needed.** Continue the **project-wide dependency refresh** where slice 1
(Rust `Cargo.lock`) left off. Scope as several smaller per-ecosystem steps (this spans many
manifests and does NOT cite an `[audit]` issue, so no 8-file escape valve applies):

1. Rust direct-pin evaluation — review workspace `Cargo.toml` majors (uniffi, pyo3, criterion,
    iai-callgrind, magnus, jni, napi); bump the safe ones, document each deliberate hold-back next
    to its pin.
2. Python `uv.lock` refresh (keep `abi3-py310`; PyO3 bumps only with re-verifying `gil_used`/
    `py.detach`, #41).
3. Per-binding manifests: napi `package.json`, rb `Gemfile`/gemspec (rb_sys must match the
    `oxidize-rb/actions/cross-gem` Docker tag), jni `pom.xml`, kotlin `build.gradle.kts`, dotnet
    `.csproj`, go `go.mod`.
4. Tooling pins: `mise.toml`, `.pre-commit-config.yaml`, GHA action versions (quality-gate tool pins
    like `cargo-crap` bump together with their committed baselines).

Do NOT cut v1.0.0 or flip the `Semver` gate to enforcing — both deliberately held by Titusz. The npm
OIDC migration and single-registry re-trigger fixes stay human-gated. Guard: any source change that
adds a branch/loop to a covered function must refresh `.crap-baseline.json` in the same step (the
CRAP regression gate is CI-only, not in `mise run check`). Watch for the enforcing
`Audit (cargo-deny)` gate turning red on a fresh live advisory (prefer `cargo update -p <crate>`).
