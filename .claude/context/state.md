<!-- assessed-at: c4417400cfc5033c110b773bb4836802f7e7272e -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — CI GREEN; dependency refresh underway (5 of ~8 slices done)

v0.5.0 is released across all registries and all 12 language bindings meet their core criteria.
Iteration 128 landed **slice 5 of the dependency refresh**: 11 JVM manifest pins bumped in
`crates/iscc-jni/java/pom.xml` and `packages/kotlin/build.gradle.kts`. **CI is fully green on the
develop tip (41 check-runs, 0 non-success, 0 running.)** The slice surfaced one release-blocking
side effect that is now an open `[review]` issue with HUMAN REVIEW REQUESTED: the Kotlin plugin bump
raised the published consumer Kotlin floor to 2.3.

## Rust Core Crate

**Status**: partially met — CI GREEN; only the held semver-enforcing / v1.0.0 criterion is unmet

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing — `Rust (fmt, clippy, test)` job GREEN. Workspace
    version `0.5.0` (unchanged). 11 source modules in `crates/iscc-lib/src/`, **320** `#[test]`
    functions (re-counted). **No Rust source touched this iteration** — the only non-context changes
    were two JVM manifests plus 4 doc/CLAUDE.md files.
- **Rust pin hold-backs documented (iter 126)**: `grep -c '# held' Cargo.toml` → **4** (criterion
    0.8 needs rustc 1.86 vs declared `rust-version = "1.85"`; jni 0.22 is a wholesale API rework;
    magnus 0.8 drops `old-api` and deprecates `exception::runtime_error()`; uniffi 0.32 needs
    Swift+Kotlin regeneration) plus a `# note:` on `pyo3` tying bumps to issue #41. These comments
    are the authoritative record of why each pin sits below latest.
- **Cargo.lock refreshed (iter 124, slice 1)** — ~100 transitive crates at latest semver-compatible
    versions; still standing; cleared cargo-deny.
- **Trailing-byte hardening intact (iter 121)**: `iscc_decode` (`crates/iscc-lib/src/lib.rs`) has
    both "too short" (~line 235) and "too long" (~line 241) rejection branches. ISCC-IDv1 is
    rejected at the header level in the Rust core (`codec::Version` is V0-only) — IDv1 support
    remains Go-only.
- **CRAP regression gate green** (iter 122 baseline refresh holds): max CRAP well below the 30.0
    `--fail-above` cap. NOTE: the `--fail-regression` gate is CI-only (not in `mise run check`/
    pre-commit) — any source change adding a branch/loop to a covered function must refresh
    `.crap-baseline.json` in the same step.
- **Perf gate — COMPLETE, ENFORCING**: `Perf (iai-callgrind)` GREEN against the unmodified baseline.
- **Semver gate present (informational)**: the `semver` job carries `continue-on-error: true` and
    reports `success`; the enforcing-at-v1.0.0 criterion (`rust-core.md` semver box `[ ]`) stays
    unmet — deliberately held by Titusz until the v1.0.0 cut.
- **Known constraint (not a regression)**: the `proc-macro-error2 v2.0.1` future-incompat warning on
    `cargo test`/`cargo bench` comes from `iai-callgrind-macros` → `iai-callgrind` (dev-only). No
    fixed upstream release; re-check when bumping `iai-callgrind` (must stay in lockstep with the
    CI-installed runner).

## Python Bindings

**Status**: met

- Core met: all 32 symbols exported, `Python 3.10` and `Python 3.14` matrix jobs GREEN (plus the
    `if: always()` aggregator gate job `Python (ruff, pytest)`), ruff clean, streaming `SumHasher`
    wrapper present. PyO3 pinned `0.29` (`abi3-py310`). Untouched this iteration.
- **GIL release COMPLETE (issue #41 RESOLVED)**: `grep -c '\.detach('` in
    `crates/iscc-py/src/lib.rs` = **12** (re-verified) — data/instance/image/sum/text/video plus 3
    streaming `update()` paths. Video detach opens strictly after frame-signature extraction;
    meta/audio/mixed stay attached by design.
- **`uv.lock` refreshed (iter 125, slice 2)**: 40 packages bumped incl. iscc-core `1.3.0` (matches
    the vendored `data.json` vectors, zero output drift), ty 0.0.63, maturin 1.14.1, prek 0.4.11.
- **Documented hold-back**: `pyproject.toml` pins `ruff<0.16` with an inline `# held:` comment (0.16
    expands default lint rules → 104 new errors, mostly `_lowlevel.pyi`). Deferred adoption, not
    gate weakening.
- **aarch64 wheels wired (issue #49 DONE, iter 123)**: `release.yml` `build-wheels` has the
    `ubuntu-24.04-arm`/`aarch64` entry; first real aarch64 wheel ships at v0.6.0.

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols with TypeScript declarations; `Node.js (napi build, test)` job GREEN
    (`actions/setup-node@v7`, node 20). `crates/iscc-napi/package.json` is the **only**
    hand-maintained JS manifest and carries a single `@napi-rs/cli: ^3` devDependency — already
    caret-covered, so the remaining manifest slice is a one-line confirmation here.

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

- **47** `#[unsafe(no_mangle)]` extern entry points (re-verified); cbindgen header committed +
    freshness check, C test passes, csbindgen generates `NativeMethods.g.cs`;
    `C FFI (cbindgen, gcc, test)` job GREEN.

## Java Bindings

**Status**: met — JVM manifest refresh (slice 5) landed iteration 128

- 32 Tier 1 symbols via JNI, native libs bundled in JAR; `Java (JNI build, mvn test)` job GREEN
    (`actions/setup-java@v5`, temurin 17). `jni` stays pinned at 0.21 with a documented hold-back.
- **`pom.xml` pins now current (verified in file)**: junit-jupiter `5.14.4`, gson `2.14.0`,
    maven-compiler-plugin `3.15.0`, maven-surefire-plugin `3.5.6`, and in the release-only profile
    maven-source-plugin `3.4.0`, maven-javadoc-plugin `3.12.0`, maven-gpg-plugin `3.2.8`.
    `central-publishing-maven-plugin` is deliberately **held at 0.7.0** with an inline `held:` XML
    comment — its `deploy` goal only runs during a real Maven Central publish, so no local or CI
    check can validate a bump. Local run was mvn 69/69 green; CI `Java` job confirms.
- Doc sync verified: `crates/iscc-jni/CLAUDE.md` now names 5.14.4/2.14.0 (no `5.11.4`/`2.11.0`
    strings remain anywhere in the repo).

## Go Bindings

**Status**: met — ISCC-IDv1 (#43) landed iter 119, trailing-byte hardening landed iter 120

- Pure Go (no CGO), all 32 Tier 1 symbols, `Go (go test, go vet)` job GREEN (`actions/setup-go@v7`,
    version from `go.mod`), `CGO_ENABLED=0` holds.
- `packages/go/iscc_id.go` provides `EncodeIsccID` / `DecodeIsccID` + `IsccIDv1Result`; `codec.go`
    adds `VSV1` and a MainType-ID-only Version=1 relaxation in `decodeHeader`.
- `IsccDecode` (`codec.go`) has both "too short" (~line 594) and "too long" (~line 597) rejection
    branches; `DecodeIsccID` inherits via delegation; `IsccDecompose` untouched.
- Manifest state for a future slice: `go.mod` declares `go 1.26.1` with
    `github.com/zeebo/blake3 v0.2.4`, `golang.org/x/text v0.34.0`, indirect
    `github.com/klauspost/cpuid/v2 v2.0.12`. Caution: the `go` directive is a **consumer floor**,
    not a dependency pin — same support-policy class as the Kotlin issue below.

## Ruby Bindings

**Status**: met

- 32 Tier 1 symbols via Magnus; `Ruby (magnus build, test)` job GREEN; version synced. `magnus`
    stays pinned at 0.7 with a documented hold-back (0.8 needs an `exception::runtime_error()`
    call-site refactor across 5 sites).
- Manifest state: `crates/iscc-rb/Gemfile` uses pessimistic constraints only (minitest `~> 5.0`,
    rake `~> 13.0`, rake-compiler `~> 1.2`, rb_sys `~> 0.9`, standard `~> 1.0`, rubocop-minitest
    `~> 0.36`); the gemspec declares `required_ruby_version >= 3.1.0` (a consumer floor — leave it
    alone in a refresh step). Any rb_sys move must stay matched to the
    `oxidize-rb/actions/cross-gem` Docker image tag.

## C# / .NET Bindings

**Status**: met

- 32 public symbols via P/Invoke over the C FFI; `C# / .NET (dotnet build, test)` job GREEN
    (`actions/setup-dotnet@v6`, SDK 8.0). Manifest state: both projects target `net8.0`; the test
    project's package refs already use floating wildcards (`Microsoft.NET.Test.Sdk 17.*`,
    `xunit 2.*`, `xunit.runner.visualstudio 2.*`), so the .NET manifest slice is near-empty.

## C++ Bindings

**Status**: met

- C++17 header-only wrapper, all 32 Tier 1 symbols, ASAN clean, vcpkg + Conan;
    `C++ (cmake, ASAN, test)` job GREEN.

## UniFFI Scaffolding Crate

**Status**: complete (internal, not published)

- **32** `#[uniffi::export]` annotations (re-verified); shared by Swift + Kotlin. Pinned at 0.31
    with a documented hold-back (0.32 requires Swift/Kotlin regeneration, unverifiable in the Linux
    devcontainer).

## Swift Bindings

**Status**: met

- SPM package with UniFFI bindings, all 32 Tier 1 symbols, XCFramework build;
    `Swift (swift build, swift test)` job GREEN on macos-14. XCFramework checksum current for
    v0.5.0.

## Kotlin Bindings

**Status**: met on target criteria (CI GREEN) — but one **release-blocking** open `[review]` issue

- `packages/kotlin/` with JNA-loaded UniFFI bindings, 9 desktop + Android targets;
    `Kotlin (gradle build, test)` job GREEN (gradle 9/9 tests locally). Generated bindings and the
    Gradle wrapper were untouched by the refresh.
- **Refreshed pins (verified in `build.gradle.kts`)**: `kotlin("jvm") 2.4.10`, JNA `5.19.1`,
    junit-jupiter `5.14.4`, gson `2.14.0`, plus a required
    `testRuntimeOnly org.junit.platform:junit-platform-launcher:1.14.4` (Gradle 8.12.1 injects a
    launcher predating platform 1.12). JUnit 6.x is held with an inline `held:` comment. The
    launcher does not leak into the published POM (verified via
    `generatePomFileForMavenPublication`).
- **OPEN, release-blocking**: the `kotlin("jvm")` 2.1.10 → 2.4.10 bump stamps metadata `mv=[2,4,0]`
    into the published jar and puts `kotlin-stdlib:2.4.10` in the published POM's compile scope.
    Empirically verified against throwaway consumers: Kotlin 2.1.10 FAILS, 2.2.21 FAILS, 2.3.21
    PASSES — so the consumer floor silently moved to **Kotlin 2.3+**. Nothing has shipped (only the
    human-dispatched `release.yml` publishes `io.iscc:iscc-lib-kotlin`), so this must be settled
    before the next Maven Central publish. Filed as `normal` `[review]` with HUMAN REVIEW REQUESTED;
    rationale for approving the slice anyway is in `decisions.md` 2026-07-25.
- **Doc/spec drift from this change (unresolved, small)**:
    `.claude/context/specs/kotlin-bindings.md` still shows `net.java.dev.jna:jna:5.16.0@aar` at
    lines 150 and 165 (human-owned file, out of the slice's scope), and no spec or README states a
    consumer Kotlin floor.

## README

**Status**: met

- Polyglot README with CI + registry badges, per-language install/quick start for all 12 languages,
    architecture + MainTypes. Kotlin snippet updated to JNA 5.19.1 this iteration.

## Per-Crate READMEs

**Status**: partially met — one stale install snippet found

- READMEs present for all 12 crates/packages; registry metadata references them.
- **NEW FINDING (small, CID-doable, not yet in issues.md)**: `packages/kotlin/README.md:13` still
    advertises `implementation("io.iscc:iscc-lib-kotlin:0.3.1")` while the workspace is at `0.5.0`.
    Root cause: `packages/kotlin/README.md` is **absent from the `TARGETS` list in
    `scripts/version_sync.py`** (which does cover `README.md`, `crates/iscc-jni/README.md`,
    `packages/swift/README.md`, `docs/howto/kotlin.md`, and 16 other targets), so
    `mise run version:check` passes while the file rots. Fix is two lines: correct the version and
    add the file to `TARGETS`. It is the only remaining `0.3.1` string in the repo.

## Documentation

**Status**: met (one low-priority cosmetic gap)

- Docs site, **11** `docs/howto/*.md` language guides, tabbed examples, llms-full.txt (22 pages),
    benchmarks page with speedup factors all present. `docs/howto/kotlin.md` JNA snippet refreshed
    to 5.19.1 this iteration; no other docs content changed.
- **`docs.yml` action bumps are still statically verified only** — the workflow triggers on push to
    `main`, so `upload-pages-artifact@v5` + `deploy-pages@v5` get their first real exercise on the
    next develop→main merge (PR #44 "Release 0.6.0" is open). If the docs deploy breaks after that
    merge, slice 4 is the first suspect.
- **Gap (low, CID skips)**: language logos in `docs/index.md` / howto headers — cosmetic only.

## Benchmarks

**Status**: met

- Criterion benches for all 10 `gen_*_v0` (+2) — **12** `bench_*` functions in `benchmarks.rs`;
    `Bench (compile check)` job GREEN on criterion 0.7 with `black_box` imported from `std::hint`.
    18 pytest-benchmark functions, speedup factors published (1.3x-158x) in `docs/benchmarks.md`.
- Second harness `iai_benches.rs` (iai-callgrind 0.16, **11** `bench_*` → 16 cases) enforcing the
    > 10% Ir regression gate — GREEN against the unmodified baseline.

## CI/CD and Publishing

**Status**: partially met — **CI GREEN**; one CID-doable gap remains (dependency freshness, in
progress)

- **LATEST CI RUN — SUCCESS.** origin/develop tip `4a93fe9` (the iteration-128 review commit): **41
    check-runs, 0 non-success, 0 still running** — verified via
    `gh api repos/iscc/iscc-lib/commits/4a93fe9.../check-runs?per_page=100`. All 21 distinct check
    names are `success`, including `Java (JNI build, mvn test)` and `Kotlin (gradle build, test)`.
    (41 rather than ~20 because every develop commit triggers both a `push` run and a `pull_request`
    run from the open develop→main PR #44.) HEAD `c441740` is a +1 log-only commit touching
    `iterations.jsonl` only, unpushed — no code sits outside CI coverage.
- ci.yml has **19 job entries → 20 jobs** (`python-test` is a 3.10/3.14 matrix, `python` is an
    `if: always()` aggregator gate). Untouched this iteration.
- **Dependency refresh progress**: ✅ slice 1 Rust `Cargo.lock` (124), ✅ slice 2 Python `uv.lock`
    (125), ✅ slice 3 Rust direct pins (126), ✅ slice 4 GitHub Actions in ci.yml + docs.yml (127), ✅
    slice 5 JVM manifests — pom.xml + build.gradle.kts (128). Remaining under the `normal` `[human]`
    "Dependency review and refresh" issue: rb `Gemfile`/gemspec + `Gemfile.lock`, go `go.mod`, napi
    `package.json` (one-liner), dotnet `.csproj` (wildcards — confirmation only), `release.yml`
    action refs, the Gradle wrapper 8.12.1 and JUnit 6.x majors, plus three migration steps (ruff
    0.16, `magnus` 0.8, `jni` 0.22).
- **Standing correction for planning**: `release.yml` contains **no `astral-sh/setup-uv` step** —
    the claim in issues.md that its setup-uv needs `@v9.0.0` is wrong. Its actual lag is
    `actions/checkout@v4` (23), `actions/download-artifact@v4` (20), `actions/upload-artifact@v4`
    (11, must move as a pair with download), `actions/setup-java@v4` (6), `actions/setup-node@v5`
    (5), `actions/setup-dotnet@v4` (3), `actions/setup-python@v5` (2), `actions/cache@v4` (1).
    Nothing in release.yml is exercised by a CID push, so it stays human-timed.
- **cargo-deny gate enforcing** (unchanged): root `deny.toml`, `Audit (cargo-deny)` job GREEN. A
    fresh live advisory can flip this red on any push with no code change (prefer
    `cargo update -p <crate>` over a `deny.toml` ignore).
- **Gap (v0.6.0)**: no automated dependency updates — `.github/dependabot.yml` and `renovate.json`
    both absent (deliberate; refresh is a manual per-release pass).
- v0.5.0 published; release workflow with 8 per-registry toggles + version sync (20 `TARGETS`
    entries) in place. Two release-workflow reliability issues remain open (npm OIDC migration,
    single-registry re-trigger bug) — both human-gated `normal`.

## Open Issues (issues.md lists 6 — 0 critical, 4 normal, 2 low)

CI is green — no open issue is CI-blocking. Any open issue keeps the project IN_PROGRESS. One issue
is **new this iteration** (the Kotlin consumer floor, source tag `[review]`).

CID-doable now (`normal`, `[human]`, spec'd):

- Dependency review and refresh across the project (spec: `.claude/context/specs/ci-cd.md` →
    "Dependency Freshness"). Slices 1-5 DONE; remaining slices listed above.

Needs a human policy decision (`normal`, `[review]`, HUMAN REVIEW REQUESTED):

- Kotlin binding silently raised the consumer Kotlin floor to 2.3 (spec:
    `.claude/context/specs/kotlin-bindings.md`, which documents no floor). Two options: accept and
    document "requires Kotlin 2.3+" (review agent's recommendation, docs-only), or hold
    `kotlin("jvm")` at 2.1.x — note that pinning `languageVersion` alone is insufficient because the
    transitive `kotlin-stdlib:2.4.10` triggers the same error. Release-blocking for
    `io.iscc:iscc-lib-kotlin`, harmless on develop.

Release-workflow reliability (`normal`, `[human]`, human-gated):

- Migrate npm publishing to OIDC Trusted Publishing (needs npm-side trusted-publisher config first)
- Fix broken single-registry re-trigger in `release.yml`

Low (human-directed, CID skips):

- Release core as v1.0.0 — **held** by Titusz (stay on 0.5.x; flip Semver to enforcing at the cut).
- Add programming language logos to docs site — cosmetic.

## Next Milestone

**CI is green — no CI fix needed.** The Kotlin consumer-floor issue is the most consequential open
item, but its central question (which Kotlin versions we support) is a support-policy call reserved
for Titusz, so the loop should not decide it. Recommended order:

1. **Dependency-refresh slice 6 — one small non-JVM ecosystem.** Best candidate:
    `crates/iscc-napi/package.json` + `packages/dotnet/*/*.csproj` bundled as a single
    confirmation-and-bump step (both are near-empty: one `@napi-rs/cli: ^3` devDependency, and
    csproj refs already float on `17.*`/`2.*` wildcards), or the Ruby manifests
    (`crates/iscc-rb/Gemfile` + gemspec + `Gemfile.lock`) if a more substantial step is wanted —
    Ruby is fully exercised by the `Ruby (magnus build, test)` job on every develop push. **Apply
    the slice-5 lesson**: do not move a consumer floor inside a refresh step —
    `required_ruby_version` (Ruby), the `go` directive in `go.mod`, `engines`/`net8.0` targets, and
    any rb_sys bump (which must match the `oxidize-rb/actions/cross-gem` Docker tag) are
    support-policy decisions, not pins. Cheap detectors for that class of break:
    `javap -v -p <class> | grep mv=` on a built jar, and a throwaway consumer project.
2. **Fold in the two-line `packages/kotlin/README.md` fix** (stale `0.3.1` → `0.5.0`, and add the
    file to `TARGETS` in `scripts/version_sync.py` so `version:check` catches it next time). It is
    self-contained, verifiable with `mise run version:check`, and the only stale version string
    left in the repo.
3. **Kotlin floor follow-up — only once Titusz picks an option.** If option 1 (accept and document)
    is chosen, the work is docs-only: state "requires Kotlin 2.3+" in `packages/kotlin/README.md`,
    `docs/howto/kotlin.md`, and the root README Kotlin section, and refresh the stale
    `jna:5.16.0@aar` lines (150, 165) in `specs/kotlin-bindings.md`.
4. **ruff 0.16 adoption** — `ruff check --fix` (≈60 auto-fixable), hand-fix the rest (mostly
    `_lowlevel.pyi` stub-style PIE790/PYI048/RUF022), then drop the `ruff<0.16` pin.
5. **`magnus` 0.8 and `jni` 0.22 migrations** — each its own dedicated step with a source rewrite
    (`crates/iscc-rb/src/lib.rs`, `crates/iscc-jni/src/lib.rs`); update the corresponding `# held:`
    comment in `Cargo.toml` when a hold-back is lifted.

Deferred, not autonomous: `release.yml` action refs, the Gradle wrapper 8.12.1 major, and JUnit 6.x
— none is exercised by a CID push in a way that proves the change, and the first is best opened
once, by a human, bundled with the single-registry re-trigger fix near the next release.

Do NOT cut v1.0.0, flip the `Semver` gate to enforcing, or raise the workspace MSRV to 1.86 (which
would unlock criterion 0.8) — all three are human policy calls tied to the v1.0.0 cut. Guards: any
source change that adds a branch/loop to a covered function must refresh `.crap-baseline.json` in
the same step (the CRAP regression gate is CI-only, not in `mise run check`); watch for the
enforcing `Audit (cargo-deny)` gate turning red on a fresh live advisory; before bumping any action
to `@vN`, confirm the floating major tag exists via
`gh api repos/<owner>/<repo>/git/matching-refs/tags/v<N>`; and re-run Gradle after `./gradlew clean`
before believing a Kotlin build failure (this bind mount flakes on incremental state).
