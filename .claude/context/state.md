<!-- assessed-at: 571ed7c76cc17529ed122b3e4da8f2c9370b7687 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — CI GREEN; dependency refresh underway (4 of ~8 slices done)

v0.5.0 is released across all registries and all 12 language bindings meet their core criteria.
Iteration 127 landed **slice 4 of the dependency refresh**: 9 distinct GitHub Actions refs bumped to
current majors in `ci.yml` and `docs.yml` (with `astral-sh/setup-uv` pinned to the exact tag
`@v9.0.0` because upstream publishes no floating major past v7). **CI is fully green on the develop
tip (41 check-runs, 0 non-success.)** Remaining CID-doable v0.6.0 work is the rest of the dependency
refresh: per-binding manifests, `release.yml` action refs (human-timed), and three separate
migration steps (ruff 0.16, jni 0.22, magnus 0.8).

## Rust Core Crate

**Status**: partially met — CI GREEN; only the held semver-enforcing / v1.0.0 criterion is unmet

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing — `Rust (fmt, clippy, test)` job GREEN. Workspace
    version `0.5.0` (unchanged). 11 source modules in `crates/iscc-lib/src/`, 320 `#[test]`
    functions. **No library source touched this iteration** — the only non-context changes were
    `.github/workflows/ci.yml` and `docs.yml`.
- **Rust pin hold-backs documented (iter 126)**: `grep -c '# held' Cargo.toml` → **4** (criterion
    0.8 needs rustc 1.86 vs declared `rust-version = "1.85"`; jni 0.22 is a wholesale API rework;
    magnus 0.8 drops `old-api` and deprecates `exception::runtime_error()`; uniffi 0.32 needs
    Swift+Kotlin regeneration) plus a `# note:` on `pyo3` tying bumps to issue #41. These comments
    are the authoritative record of why each pin sits below latest.
- **Cargo.lock refreshed (iter 124, slice 1)** — ~100 transitive crates at latest semver-compatible
    versions; still standing after the criterion 0.7 subtree change; cleared cargo-deny.
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
    wrapper present. PyO3 pinned `0.29` (`abi3-py310`). Untouched this iteration apart from the
    `actions/setup-python@v7` + `astral-sh/setup-uv@v9.0.0` bumps in the job definition.
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
    hand-maintained JS manifest (re-verified) and carries a single `@napi-rs/cli: ^3` devDependency
    — already caret-covered, so the manifest slice is a one-line check here.

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

- 47 `#[unsafe(no_mangle)]` extern entry points; cbindgen header committed + freshness check, C test
    passes, csbindgen generates `NativeMethods.g.cs`; `C FFI (cbindgen, gcc, test)` job GREEN.

## Java Bindings

**Status**: met

- 32 Tier 1 symbols via JNI, native libs bundled in JAR; `Java (JNI build, mvn test)` job GREEN
    (`actions/setup-java@v5`, temurin 17). `jni` stays pinned at 0.21 with a documented hold-back.
- Manifest state for the next slice: `crates/iscc-jni/java/pom.xml` pins junit-jupiter `5.11.4`,
    gson `2.11.0`, maven-compiler-plugin `3.13.0`, maven-surefire-plugin `3.5.2`, plus release-only
    profile plugins (source `3.3.1`, javadoc `3.11.2`, gpg `3.2.7`, central-publishing `0.7.0`) that
    CI does **not** exercise.

## Go Bindings

**Status**: met — ISCC-IDv1 (#43) landed iter 119, trailing-byte hardening landed iter 120

- Pure Go (no CGO), all 32 Tier 1 symbols, `Go (go test, go vet)` job GREEN (`actions/setup-go@v7`,
    version from `go.mod`), `CGO_ENABLED=0` holds.
- `packages/go/iscc_id.go` provides `EncodeIsccID` / `DecodeIsccID` + `IsccIDv1Result`; `codec.go`
    adds `VSV1` and a MainType-ID-only Version=1 relaxation in `decodeHeader`.
- `IsccDecode` (`codec.go`) has both "too short" (~line 594) and "too long" (~line 597) rejection
    branches; `DecodeIsccID` inherits via delegation; `IsccDecompose` untouched.
- Manifest state: `go.mod` declares `go 1.26.1` with `github.com/zeebo/blake3 v0.2.4`,
    `golang.org/x/text v0.34.0`, indirect `github.com/klauspost/cpuid/v2 v2.0.12`.

## Ruby Bindings

**Status**: met

- 32 Tier 1 symbols via Magnus; `Ruby (magnus build, test)` job GREEN; version synced. `magnus`
    stays pinned at 0.7 with a documented hold-back (0.8 needs an `exception::runtime_error()`
    call-site refactor across 5 sites).
- Manifest state: `crates/iscc-rb/Gemfile` uses pessimistic constraints only (minitest `~> 5.0`,
    rake `~> 13.0`, rake-compiler `~> 1.2`, rb_sys `~> 0.9`, standard `~> 1.0`, rubocop-minitest
    `~> 0.36`); the gemspec declares `required_ruby_version >= 3.1.0`. Any rb_sys move must stay
    matched to the `oxidize-rb/actions/cross-gem` Docker image tag.

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

- 32 `#[uniffi::export]` annotations (re-verified); shared by Swift + Kotlin. Pinned at 0.31 with a
    documented hold-back (0.32 requires Swift/Kotlin regeneration, unverifiable in the Linux
    devcontainer).

## Swift Bindings

**Status**: met

- SPM package with UniFFI bindings, all 32 Tier 1 symbols, XCFramework build;
    `Swift (swift build, swift test)` job GREEN on macos-14. XCFramework checksum current for
    v0.5.0.

## Kotlin Bindings

**Status**: met

- `packages/kotlin/` with JNA-loaded UniFFI bindings, 9 desktop + Android targets;
    `Kotlin (gradle build, test)` job GREEN. Manifest state: Kotlin JVM plugin `2.1.10`, JNA
    `5.16.0`.

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
    benchmarks page with speedup factors all present. No docs content changed this iteration.
- **`docs.yml` action bumps are statically verified only** — the workflow triggers on push to
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

- **LATEST CI RUN — SUCCESS.** origin/develop tip `b071517`: **41 check-runs, 0 non-success, 0 still
    running** — verified via `gh api repos/iscc/iscc-lib/commits/b071517/check-runs?per_page=100`.
    (41 rather than ~20 because every develop commit triggers both a `push` run and a `pull_request`
    run from the open develop→main PR #44.) HEAD `571ed7c` is a +1 log-only commit touching
    `iterations.jsonl` only, unpushed — no code sits outside CI coverage.
- ci.yml has **19 job entries → 20 jobs** (`python-test` is a 3.10/3.14 matrix, `python` is an
    `if: always()` aggregator gate). Job structure, steps, inputs, permissions and runtime versions
    were untouched by slice 4 — only `uses:` lines changed.
- **Slice 4 verified in place**: ci.yml `uses:` inventory is now `actions/checkout@v7` (18),
    `actions/setup-python@v7` (2), `actions/setup-node@v7` (1), `actions/setup-java@v5` (2),
    `actions/setup-go@v7` (1), `actions/setup-dotnet@v6` (1), `actions/upload-artifact@v7` (2),
    `github/codeql-action/upload-sarif@v4` (1), `astral-sh/setup-uv@v9.0.0` (1, exact tag with an
    inline explanatory comment), plus the deliberately-current `dtolnay/rust-toolchain@stable` (16),
    `Swatinem/rust-cache@v2` (16), `taiki-e/install-action@v2` (4), `ruby/setup-ruby@v1` (1),
    `obi1kenobi/cargo-semver-checks-action@v2` (1). docs.yml carries checkout v7, setup-python v7,
    setup-uv v9.0.0, upload-pages-artifact v5, deploy-pages v5. `.pre-commit-config.yaml` still has
    exactly 2 pinned revs (`v6.0.0`, `1.0.0`, both current) and `mise.toml` still has no `[tools]`
    section.
- **Dependency refresh progress**: ✅ slice 1 Rust `Cargo.lock` (124), ✅ slice 2 Python `uv.lock`
    (125), ✅ slice 3 Rust direct pins (126), ✅ slice 4 GitHub Actions in ci.yml + docs.yml (127).
    Remaining under the `normal` `[human]` "Dependency review and refresh" issue: per-binding
    manifests (jni `pom.xml`, kotlin `build.gradle.kts`, rb `Gemfile`/gemspec + `Gemfile.lock`,
    dotnet `.csproj`, go `go.mod`; napi `package.json` is a one-liner), `release.yml` action refs,
    and three migration steps (ruff 0.16, `magnus` 0.8, `jni` 0.22).
- **Correction for planning**: `release.yml` contains **no `astral-sh/setup-uv` step** — the claim
    in issues.md/handoff that its setup-uv needs `@v9.0.0` is wrong. Its actual lag is
    `actions/checkout@v4` (23), `actions/download-artifact@v4` (20), `actions/upload-artifact@v4`
    (11, must move as a pair with download), `actions/setup-java@v4` (6), `actions/setup-node@v5`
    (5), `actions/setup-dotnet@v4` (3), `actions/setup-python@v5` (2), `actions/cache@v4` (1).
    Nothing in release.yml is exercised by a CID push, so it stays human-timed.
- **cargo-deny gate enforcing** (unchanged): root `deny.toml`, `Audit (cargo-deny)` job GREEN. A
    fresh live advisory can flip this red on any push with no code change (prefer
    `cargo update -p <crate>` over a `deny.toml` ignore).
- **Gap (v0.6.0)**: no automated dependency updates — `.github/dependabot.yml` and `renovate.json`
    both absent (deliberate; refresh is a manual per-release pass).
- v0.5.0 published; release workflow with 8 per-registry toggles + version sync (16 targets) in
    place. Two release-workflow reliability issues remain open (npm OIDC migration, single-registry
    re-trigger bug) — both human-gated `normal`.

## Open Issues (issues.md lists 5 — 0 critical, 3 normal, 2 low; all `[human]`)

CI is green — no open issue is CI-blocking. There is currently **no CID-actionable `[review]` or
`[audit]` issue**. Any open issue keeps the project IN_PROGRESS. Issue headers are unchanged since
iteration 125; only the dependency issue's progress body grew (slice 4 detail).

CID-doable now (`normal`, `[human]`, spec'd):

- Dependency review and refresh across the project (spec: `.claude/context/specs/ci-cd.md` →
    "Dependency Freshness"). Slices 1-4 DONE; remaining slices listed above.

Release-workflow reliability (`normal`, `[human]`, human-gated):

- Migrate npm publishing to OIDC Trusted Publishing (needs npm-side trusted-publisher config first)
- Fix broken single-registry re-trigger in `release.yml`

Low (human-directed, CID skips):

- Release core as v1.0.0 — **held** by Titusz (stay on 0.5.x; flip Semver to enforcing at the cut).
- Add programming language logos to docs site — cosmetic.

## Next Milestone

**CI is green — no CI fix needed.** Continue the **project-wide dependency refresh** where slice 4
(GitHub Actions in ci.yml + docs.yml) left off, one small per-ecosystem step at a time (this issue
cites no `[audit]` issue, so no 8-file escape valve applies). Recommended order:

1. **JVM binding manifests (slice 5)** — `crates/iscc-jni/java/pom.xml` +
    `packages/kotlin/build.gradle.kts`, 2 files, both fully exercised on every develop push by the
    `Java (JNI build, mvn test)` and `Kotlin (gradle build, test)` jobs. Concrete pins to evaluate:
    junit-jupiter 5.11.4, gson 2.11.0, maven-compiler-plugin 3.13.0, maven-surefire-plugin 3.5.2,
    Kotlin JVM plugin 2.1.10, JNA 5.16.0. Cautions: the release-only profile plugins (source,
    javadoc, gpg, central-publishing) are **not** CI-exercised — exclude them or mark them
    statically-verified-only; `java-version: '17'` and the Kotlin jvmToolchain are support-policy
    decisions, not dependency pins, so they stay out of scope.
2. **Remaining small manifests** — rb `Gemfile`/gemspec + `Gemfile.lock` (rb_sys must match the
    `oxidize-rb/actions/cross-gem` Docker tag), go `go.mod`, napi `package.json`; the dotnet
    `.csproj` files already float on wildcards and need only a confirmation pass.
3. **ruff 0.16 adoption** — `ruff check --fix` (≈60 auto-fixable), hand-fix the rest (mostly
    `_lowlevel.pyi` stub-style PIE790/PYI048/RUF022), then drop the `ruff<0.16` pin.
4. **`magnus` 0.8 and `jni` 0.22 migrations** — each its own dedicated step with a source rewrite
    (`crates/iscc-rb/src/lib.rs`, `crates/iscc-jni/src/lib.rs`); update the corresponding `# held:`
    comment in `Cargo.toml` when a hold-back is lifted.

Deferred, not autonomous: `release.yml` action refs — nothing in that file is exercised by a CID
push, and it is best opened once, by a human, bundled with the existing single-registry re-trigger
fix near the next release.

Do NOT cut v1.0.0, flip the `Semver` gate to enforcing, or raise the workspace MSRV to 1.86 (which
would unlock criterion 0.8) — all three are human policy calls tied to the v1.0.0 cut. Guards: any
source change that adds a branch/loop to a covered function must refresh `.crap-baseline.json` in
the same step (the CRAP regression gate is CI-only, not in `mise run check`); watch for the
enforcing `Audit (cargo-deny)` gate turning red on a fresh live advisory; and before bumping any
action to `@vN`, confirm the floating major tag actually exists via
`gh api repos/<owner>/<repo>/git/matching-refs/tags/v<N>` — `releases/latest` is not proof (that
mistake reddened CI mid-iteration 127).
