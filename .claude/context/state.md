<!-- assessed-at: 0b05b77a110e5662fabe9f3726c798f9586126f5 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — CI GREEN; dependency refresh underway (Rust + Python lockfile slices done)

v0.5.0 is released across all registries and all 12 language bindings meet their core criteria. This
iteration (125) landed **slice 2 of the dependency refresh**: the root Python `uv.lock` was
regenerated via `uv lock --upgrade` (40 packages, incl. iscc-core 1.2.2→1.3.0 and ty 0.0.18→0.0.63),
with one documented `ruff<0.16` hold-back in `pyproject.toml`. **CI is fully green on the develop
tip.** Remaining CID-doable v0.6.0 work is the rest of the dependency refresh (Rust direct-pin
evaluation, per-binding manifests, tooling pins) plus a dedicated ruff 0.16 adoption step.

## Rust Core Crate

**Status**: partially met — CI GREEN; only the held semver-enforcing / v1.0.0 criterion is unmet

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing — `Rust (fmt, clippy, test)` job GREEN. Workspace
    version `0.5.0`. No Rust source or `Cargo.toml`/`Cargo.lock` change this iteration.
- **Cargo.lock refreshed (iter 124, dependency-refresh slice 1)**: ~100 transitive crates at latest
    semver-compatible versions; all direct pins held; all gates stayed green. Still standing.
- **Trailing-byte hardening intact (iter 121)**: `iscc_decode` (`crates/iscc-lib/src/lib.rs`) has
    both "too short" (~line 235) and "too long" (~line 241) rejection branches. ISCC-IDv1 is
    rejected at the header level in the Rust core (`codec::Version` is V0-only) — IDv1 support
    remains Go-only.
- **CRAP regression gate green** (iter 122 baseline refresh holds): `.crap-baseline.json` matches
    the post-hardening complexity; max CRAP well below the 30.0 `--fail-above` cap. NOTE: the
    `--fail-regression` gate is CI-only (not in `mise run check`/pre-commit) — any source change
    adding a branch/loop to a covered function must refresh the baseline in the same step.
- **Perf gate — COMPLETE, ENFORCING**: `iai_benches.rs` (iai-callgrind 0.16, 11 `bench_*` → 16
    cases); `Perf (iai-callgrind)` job GREEN.
- **Semver gate present (informational)**: `Semver (cargo-semver-checks)` is
    `continue-on-error:   true`, reports `success`; the enforcing-at-v1.0.0 criterion
    (`rust-core.md` semver box `[ ]`) stays unmet — deliberately held by Titusz until the v1.0.0
    cut.

## Python Bindings

**Status**: met — dev/lock dependencies refreshed iter 125 (dependency-refresh slice 2)

- Core met: all 32 symbols exported, `Python 3.10` and `Python 3.14` CI jobs GREEN (plus the
    aggregator `Python (ruff, pytest)` gate job), ruff clean, streaming `SumHasher` wrapper present.
    PyO3 pinned `0.29` (`abi3-py310`), single lockfile resolution.
- **GIL release COMPLETE (issue #41 RESOLVED)**: `grep -c '\.detach('` in
    `crates/iscc-py/src/lib.rs` = **12** — data/instance/image/sum/text/video plus 3 streaming
    `update()` paths. Video detach opens strictly after frame-signature extraction; meta/audio/mixed
    stay attached by design. Unchanged this iteration (no binding source touched).
- **`uv.lock` refreshed (iter 125, slice 2)**: 40 packages bumped — verified in the lockfile:
    iscc-core `1.3.0`, ruff `0.15.22` (held), plus ty 0.0.63 / maturin 1.14.1 / prek 0.4.11 / pytest
    9.1.1 / zensical 0.0.51. `data.json` conformance vectors match iscc-core 1.3.0 (ISO 24138
    frozen, zero output drift). No Rust or Python source file was modified.
- **Documented hold-back**: `pyproject.toml` dev group pins `ruff<0.16` with an inline `# held:`
    comment (0.16 expands default lint rules → 104 new errors, mostly in `_lowlevel.pyi`). This is
    deferred adoption, not gate weakening — the locked ruff 0.15.22 enforces the same rule set as
    before. A dedicated ruff 0.16 adoption step is tracked in the dependency issue.
- **aarch64 wheels wired (issue #49 DONE, iter 123)**: `release.yml` `build-wheels` has the
    `ubuntu-24.04-arm`/`aarch64` entry (native ARM runner) and `test-wheels` is matrixified. Runs
    only on a `workflow_dispatch` release with `pypi` selected — first real aarch64 wheel ships at
    v0.6.0.

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols with TypeScript declarations; `Node.js (napi build, test)` job GREEN.
    Bundled model in `package.json`. napi `package.json` dependency refresh still pending (remaining
    dependency-refresh slice).

## WASM Bindings

**Status**: met — issue #42 (SIMD) resolved iteration 118, CI-verified

- All 32 Tier 1 symbols via `#[wasm_bindgen]`, streaming `SumHasher` class, `WASM (wasm-pack test)`
    job GREEN.
- **SIMD backend active**: `crates/iscc-wasm/Cargo.toml` carries a direct
    `blake3 = { features = ["wasm32_simd"] }` dep that exists solely for feature unification (no
    `use blake3` in source — must not be pruned as "unused"). `simd128` RUSTFLAGS (ci.yml +
    release.yml) and the `--enable-simd` wasm-opt flag remain in place.

## C FFI

**Status**: met

- cbindgen header committed + freshness check, C test passes, csbindgen generates
    `NativeMethods.g.cs`; `C FFI (cbindgen, gcc, test)` job GREEN.

## Java Bindings

**Status**: met

- 32 Tier 1 symbols via JNI, native libs bundled in JAR; `Java (JNI build, mvn test)` job GREEN.

## Go Bindings

**Status**: met — ISCC-IDv1 (#43) landed iter 119, trailing-byte hardening landed iter 120

- Pure Go (no CGO), all 32 Tier 1 symbols, `Go (go test, go vet)` job GREEN, `CGO_ENABLED=0` holds.
- `packages/go/iscc_id.go` provides `EncodeIsccID` / `DecodeIsccID` + `IsccIDv1Result`; `codec.go`
    adds `VSV1` and a MainType-ID-only Version=1 relaxation in `decodeHeader`.
- `IsccDecode` (`codec.go`) has both "too short" (~line 594) and "too long" (~line 597) rejection
    branches; `DecodeIsccID` inherits via delegation; `IsccDecompose` untouched.

## Ruby Bindings

**Status**: met

- 32 Tier 1 symbols via Magnus; `Ruby (magnus build, test)` job GREEN; version synced.

## C# / .NET Bindings

**Status**: met

- 32 public symbols via P/Invoke over the C FFI; `C# / .NET (dotnet build, test)` job GREEN.

## C++ Bindings

**Status**: met

- C++17 header-only wrapper, all 32 Tier 1 symbols, ASAN clean, vcpkg + Conan;
    `C++ (cmake, ASAN,   test)` job GREEN.

## UniFFI Scaffolding Crate

**Status**: complete (internal, not published)

- 32 `#[uniffi::export]` annotations; shared by Swift + Kotlin.

## Swift Bindings

**Status**: met

- SPM package with UniFFI bindings, all 32 Tier 1 symbols, XCFramework build;
    `Swift (swift build,   swift test)` job GREEN. XCFramework checksum current for v0.5.0.

## Kotlin Bindings

**Status**: met

- `packages/kotlin/` with JNA-loaded UniFFI bindings, 9 desktop + Android targets;
    `Kotlin (gradle   build, test)` job GREEN.

## README

**Status**: met

- Polyglot README with CI + registry badges, per-language install/quick start for all 12 languages,
    architecture + MainTypes.

## Per-Crate READMEs

**Status**: met

- READMEs present for all 12 crates/packages; registry metadata references them.

## Documentation

**Status**: met (one low-priority cosmetic gap)

- Docs site, **11** `docs/howto/*.md` language guides, tabbed examples, llms-full.txt (22 pages),
    benchmarks page with speedup factors all present.
- **Anchor fix (iter 125)**: `docs/howto/c-cpp.md:9` intra-page link corrected to
    `#c-wrapper-iscchpp` (matches the
    `## C++ wrapper (\`iscc.hpp\`)`heading slug at line 328). The   stale extra hyphen was a pre-existing defect surfaced by zensical 0.0.51's new anchor checker;  `zensical
    build\` now reports "No issues found".
- **Gap (low, CID skips)**: language logos in `docs/index.md` / howto headers — cosmetic only.

## Benchmarks

**Status**: met

- Criterion benches for all 10 `gen_*_v0` (+2), `Bench (compile check)` job GREEN, 18
    pytest-benchmark functions, speedup factors published (1.3x-158x) in `docs/benchmarks.md`.
- Second iai-callgrind harness enforcing the >10% Ir regression gate (green).

## CI/CD and Publishing

**Status**: partially met — **CI GREEN**; one CID-doable gap remains (dependency freshness, in
progress)

- **LATEST CI RUN — SUCCESS.** origin/develop tip `61f031f` (HEAD `0b05b77` is a +1 log-only commit
    touching only `iterations.jsonl`, unpushed). **41 check-runs, 0 non-success** (duplicates are
    re-runs) — verified via `gh api repos/iscc/iscc-lib/commits/61f031f/check-runs`. The `uv.lock`
    refresh regressed no gate; notably `Coverage + CRAP`, `Perf (iai-callgrind)` and
    `Audit   (cargo-deny)` all GREEN, and the Python type-check passed on the ty 0.0.18→0.0.63 jump.
- ci.yml has 19 job entries → 20 jobs (`python-test` is a 3.10/3.14 matrix, `python` is an
    `if: always()` aggregator gate).
- **Dependency refresh progress**: ✅ slice 1 Rust `Cargo.lock` (iter 124), ✅ slice 2 Python
    `uv.lock` (iter 125). Remaining under the `normal` `[human]` "Dependency review and refresh"
    issue: Rust direct-pin evaluation (document hold-backs for uniffi 0.32 / pyo3 #41 / criterion /
    iai-callgrind / magnus / jni / napi), per-binding manifests (napi `package.json`, rb
    `Gemfile`/gemspec, jni `pom.xml`, kotlin `build.gradle.kts`, dotnet `.csproj`, go `go.mod`),
    tooling pins (`mise.toml`, `.pre-commit-config.yaml`, GHA action versions), and the dedicated
    ruff 0.16 adoption step.
- **cargo-deny gate enforcing** (unchanged): root `deny.toml`, `Audit (cargo-deny)` job GREEN. A
    fresh live advisory can flip this red on any push with no code change (prefer
    `cargo update -p <crate>` over a `deny.toml` ignore).
- **Gap (v0.6.0)**: no automated dependency updates — `.github/dependabot.yml` and `renovate.json`
    both verified absent (deliberate; refresh is a manual per-release pass).
- v0.5.0 published; release workflow with 8 per-registry toggles + version sync (16 targets) in
    place. Two release-workflow reliability issues remain open (npm OIDC migration, single-registry
    re-trigger bug) — both human-gated `normal`.

## Open Issues (issues.md lists 5 — 0 critical, 3 normal, 2 low; all `[human]`)

CI is green — no open issue is CI-blocking. There is currently **no CID-actionable `[review]` or
`[audit]` issue**. Any open issue keeps the project IN_PROGRESS.

CID-doable now (`normal`, `[human]`, spec'd):

- Dependency review and refresh across the project (spec: `.claude/context/specs/ci-cd.md` →
    "Dependency Freshness"). Slices 1 (Rust `Cargo.lock`) and 2 (Python `uv.lock`) DONE; remaining
    slices listed above.

Release-workflow reliability (`normal`, `[human]`, human-gated):

- Migrate npm publishing to OIDC Trusted Publishing (needs npm-side trusted-publisher config first)
- Fix broken single-registry re-trigger in `release.yml`

Low (human-directed, CID skips):

- Release core as v1.0.0 — **held** by Titusz (stay on 0.5.x; flip Semver to enforcing at the cut).
- Add programming language logos to docs site — cosmetic.

## Next Milestone

**CI is green — no CI fix needed.** Continue the **project-wide dependency refresh** where slice 2
(Python `uv.lock`) left off. Keep scoping one small per-ecosystem step at a time (this issue spans
many manifests and cites no `[audit]` issue, so no 8-file escape valve applies). Recommended order:

1. **Rust direct-pin evaluation** — review workspace `Cargo.toml` majors (uniffi, pyo3, criterion,
    iai-callgrind, magnus, jni, napi); bump the safe ones and document each deliberate hold-back
    inline next to its pin (pyo3 stays pinned per #41 `gil_used`/`py.detach`; uniffi 0.32 would
    require Swift/Kotlin binding regeneration).
2. **ruff 0.16 adoption** — `ruff check --fix` (≈60 auto-fixable), hand-fix the rest (mostly
    `_lowlevel.pyi` stub-style PIE790/PYI048/RUF022), then drop the `ruff<0.16` pin from
    `pyproject.toml`.
3. **Per-binding manifests** — napi `package.json`, rb `Gemfile`/gemspec (rb_sys must match the
    `oxidize-rb/actions/cross-gem` Docker tag), jni `pom.xml`, kotlin `build.gradle.kts`, dotnet
    `.csproj`, go `go.mod` — one small step each.
4. **Tooling pins** — `mise.toml`, `.pre-commit-config.yaml`, GHA action versions (quality-gate tool
    pins such as `cargo-crap` bump together with their committed baselines).

Do NOT cut v1.0.0 or flip the `Semver` gate to enforcing — both deliberately held by Titusz. The npm
OIDC migration and single-registry re-trigger fixes stay human-gated. Guards: any source change that
adds a branch/loop to a covered function must refresh `.crap-baseline.json` in the same step (the
CRAP regression gate is CI-only, not in `mise run check`); and watch for the enforcing
`Audit (cargo-deny)` gate turning red on a fresh live advisory.
