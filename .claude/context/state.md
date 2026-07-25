<!-- assessed-at: b83b1ee176f0e2bf9aa1dcefcbdcb12dde130ab9 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — CI GREEN; dependency refresh underway (6 of ~8 slices done)

v0.5.0 is released across all registries and all 12 language bindings meet their core criteria.
Iteration 129 landed **slice 6 of the dependency refresh** (Go module: `x/text` 0.34.0 → 0.40.0,
`cpuid/v2` 2.0.12 → 2.4.0, new indirect `x/sys` 0.47.0) plus the stale `packages/kotlin/README.md`
version string, now managed by `scripts/version_sync.py`. **CI is fully green on the develop tip (41
check-runs, 0 non-success, 0 running.)** Establishing the refresh's output-neutrality baseline
uncovered a **pre-existing conformance divergence**: the Rust core disagrees with `iscc-core` and
the Go package on characters assigned after Unicode 15 — now an open `[review]` issue with HUMAN
REVIEW REQUESTED.

## Rust Core Crate

**Status**: partially met — CI GREEN, but **two** unmet items: the held semver/v1.0.0 criterion and
a newly-identified Unicode conformance divergence

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols, all
    vendored `iscc-core/data.json` vectors passing — `Rust (fmt, clippy, test)` job GREEN. Workspace
    version `0.5.0` (unchanged). 11 source modules in `crates/iscc-lib/src/`, **320** `#[test]`
    functions. **No Rust source touched this iteration** — the only non-context changes were
    `packages/go/go.{mod,sum}`, `packages/kotlin/README.md`, and one line in
    `scripts/version_sync.py`.
- **NEW, unmet: Unicode-version divergence from the reference (iter 129, open `[review]` issue).**
    Target criterion "All Tier 1 functions produce output matching their iscc-core counterparts" is
    violated outside the vendored vectors. Independently confirmed this iteration: the devcontainer
    Python is 3.13.14 with `unicodedata` **15.1.0**, where `Ɤ` (U+A7CB, added in Unicode 16) has
    category `Cn`; `crates/iscc-lib/src/utils.rs:28-32` strips `GeneralCategory::Unassigned` using
    `unicode-general-category` **1.1.0 (Unicode 16)** — which classifies U+A7CB as `Lu`, so the Rust
    core *keeps* a character Go and `iscc-core` *strip*. `unicode-normalization` is at 0.1.25
    (Unicode 17). The iter-129 review swept the code space: **5,813** `text_clean` and **5,750**
    `text_collapse` disagreements, reproduced end-to-end (Meta-Code and Text-Code differ). All 10
    non-Go bindings inherit the core's behaviour; `packages/go` matches the reference. No gate
    catches it — every vendored vector predates Unicode 16. Choosing a Unicode version is a
    conformance-policy call reserved for Titusz (it also flips direction once CPython 3.14 ships
    Unicode 16), so the loop must not pick unilaterally.
- **Rust pin hold-backs documented (iter 126)**: `grep -c '# held' Cargo.toml` → **4** (criterion
    0.8 needs rustc 1.86 vs declared `rust-version = "1.85"`; jni 0.22 is a wholesale API rework;
    magnus 0.8 drops `old-api`; uniffi 0.32 needs Swift+Kotlin regeneration) plus a `# note:` on
    `pyo3` tying bumps to issue #41.
- **Cargo.lock refreshed (iter 124, slice 1)** — still standing; clears cargo-deny.
- **Trailing-byte hardening intact (iter 121)**: `iscc_decode` has both "too short" and "too long"
    rejection branches. ISCC-IDv1 stays rejected at the header level in Rust (Go-only feature).
- **CRAP regression gate green**; max CRAP well below the 30.0 `--fail-above` cap. NOTE: the
    `--fail-regression` gate is CI-only (not in `mise run check`/pre-commit) — any source change
    adding a branch/loop to a covered function must refresh `.crap-baseline.json` in the same step.
- **Perf gate — COMPLETE, ENFORCING**: `Perf (iai-callgrind)` GREEN against the unmodified baseline.
- **Semver gate present (informational)**: the `semver` job carries `continue-on-error: true`; the
    enforcing-at-v1.0.0 criterion (`rust-core.md` semver box `[ ]`) stays unmet — held by Titusz.
- **Known constraint (not a regression)**: the `proc-macro-error2 v2.0.1` future-incompat warning on
    `cargo test`/`cargo bench` comes from `iai-callgrind-macros` → `iai-callgrind` (dev-only).

## Python Bindings

**Status**: met (with the cross-cutting Unicode caveat inherited from the core)

- Core met: all 32 symbols exported, `Python 3.10` and `Python 3.14` matrix jobs GREEN (plus the
    `if: always()` aggregator gate job `Python (ruff, pytest)`), ruff clean, streaming `SumHasher`
    wrapper present. PyO3 pinned `0.29` (`abi3-py310`). Untouched this iteration.
- **Caveat**: the criterion "All functions return `dict` with the same keys and values as iscc-core"
    holds for every vendored vector but not for text containing post-Unicode-15 characters — the
    binding faithfully re-exports the core's behaviour (see Rust Core above). Fixing it belongs in
    the core, not here.
- **GIL release COMPLETE (issue #41 RESOLVED)**: `grep -c '\.detach('` in
    `crates/iscc-py/src/lib.rs` = **12** (re-verified) — data/instance/image/sum/text/video plus 3
    streaming `update()` paths.
- **`uv.lock` refreshed (iter 125, slice 2)**: 40 packages incl. iscc-core `1.3.0`, ty 0.0.63,
    maturin 1.14.1, prek 0.4.11.
- **Documented hold-back**: `pyproject.toml` pins `ruff<0.16` with an inline `# held:` comment (0.16
    expands default lint rules → 104 new errors, mostly `_lowlevel.pyi`).
- **aarch64 wheels wired (issue #49 DONE, iter 123)**; first real aarch64 wheel ships at v0.6.0.

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols with TypeScript declarations; `Node.js (napi build, test)` job GREEN
    (`actions/setup-node@v7`, node 20). `crates/iscc-napi/package.json` re-checked iter 129: its
    lone `@napi-rs/cli: ^3` devDependency already floats over the 3.x line (covers 3.7.4) — editing
    it would be churn, so the napi manifest slice is closed as "verified current".

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
- `pom.xml` pins current: junit-jupiter `5.14.4`, gson `2.14.0`, maven-compiler-plugin `3.15.0`,
    surefire `3.5.6`, plus release-profile source `3.4.0` / javadoc `3.12.0` / gpg `3.2.8`.
    `central-publishing-maven-plugin` deliberately **held at 0.7.0** (its `deploy` goal only runs in
    a real Maven Central publish, so no local or CI check can validate a bump).

## Go Bindings

**Status**: met — dependency refresh (slice 6) landed iteration 129

- Pure Go (no CGO), all 32 Tier 1 symbols, `Go (go test, go vet)` job GREEN (`actions/setup-go@v7`,
    version from `go.mod`), `CGO_ENABLED=0` holds.
- **`go.mod` verified current (this iteration)**: `golang.org/x/text v0.40.0` (direct),
    `github.com/zeebo/blake3 v0.2.4` (unchanged, latest), indirect
    `github.com/klauspost/cpuid/v2 v2.4.0` and new indirect `golang.org/x/sys v0.47.0` (required by
    cpuid 2.4.0). The `go 1.26.1` **consumer floor** was deliberately untouched — slice-5's lesson
    applied. No hold-backs needed; all three modules are at their latest published versions.
- The `x/text` bump was proven **output-neutral, not merely vector-green**:
    `TextClean`/`TextCollapse` are byte-identical across all 1,112,032 code points under 0.34.0 and
    0.40.0 (the `unicode/norm` tables files are unchanged between releases and `tables17.0.0.go` is
    gated `//go:build go1.27`).
- `packages/go/iscc_id.go` provides `EncodeIsccID`/`DecodeIsccID` + `IsccIDv1Result`; `codec.go`
    adds `VSV1` and a MainType-ID-only Version=1 relaxation in `decodeHeader`. `IsccDecode` retains
    both "too short" and "too long" rejection branches.
- **Note**: Go currently matches `iscc-core` on post-Unicode-15 text only because Go's stdlib tables
    are at 15.0.0. That alignment is incidental and will invert when Go 1.27 lands (or when CPython
    3.14 becomes the reference runtime) — see the Rust Core Unicode item.

## Ruby Bindings

**Status**: met

- 32 Tier 1 symbols via Magnus; `Ruby (magnus build, test)` job GREEN; version synced. `magnus`
    stays pinned at 0.7 with a documented hold-back (0.8 needs an `exception::runtime_error()`
    call-site refactor across 5 sites).
- Manifest state (next refresh slice): `crates/iscc-rb/Gemfile` uses pessimistic constraints only
    (minitest `~> 5.0`, rake `~> 13.0`, rake-compiler `~> 1.2`, rb_sys `~> 0.9`, standard `~> 1.0`,
    rubocop-minitest `~> 0.36`); the gemspec declares `required_ruby_version >= 3.1.0` (a consumer
    floor — leave it alone). Any rb_sys move must stay matched to the `oxidize-rb/actions/cross-gem`
    Docker image tag.

## C# / .NET Bindings

**Status**: met

- 32 public symbols via P/Invoke over the C FFI; `C# / .NET (dotnet build, test)` job GREEN
    (`actions/setup-dotnet@v6`, SDK 8.0). Manifest re-checked iter 129: both projects target
    `net8.0` and the test project's package refs already float (`Microsoft.NET.Test.Sdk 17.*`,
    `xunit 2.*`, `xunit.runner.visualstudio 2.*`) — the .NET refresh slice is closed as "verified
    current"; only the xunit 3.x / Test.Sdk 18.x majors remain, each its own step.

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
    `Kotlin (gradle build, test)` job GREEN. Untouched this iteration apart from the README version.
- Refreshed pins (slice 5): `kotlin("jvm") 2.4.10`, JNA `5.19.1`, junit-jupiter `5.14.4`, gson
    `2.14.0`, plus a required `testRuntimeOnly junit-platform-launcher:1.14.4`. JUnit 6.x held with
    an inline `held:` comment.
- **OPEN, release-blocking**: the `kotlin("jvm")` 2.1.10 → 2.4.10 bump stamps metadata `mv=[2,4,0]`
    into the published jar and puts `kotlin-stdlib:2.4.10` in the published POM's compile scope.
    Empirically: Kotlin 2.1.10 FAILS, 2.2.21 FAILS, 2.3.21 PASSES — the consumer floor silently
    moved to **Kotlin 2.3+**. Nothing has shipped (only the human-dispatched `release.yml` publishes
    `io.iscc:iscc-lib-kotlin`), so it must be settled before the next Maven Central publish. Filed
    `normal` `[review]` with HUMAN REVIEW REQUESTED; rationale for approving the slice anyway is in
    `decisions.md` 2026-07-25.
- **Doc/spec drift (unresolved, small)**: `.claude/context/specs/kotlin-bindings.md` still shows
    `net.java.dev.jna:jna:5.16.0@aar` at lines 150 and 165 (human-owned file), and no spec or README
    states a consumer Kotlin floor.

## README

**Status**: met

- Polyglot README with CI + registry badges, per-language install/quick start for all 12 languages,
    architecture + MainTypes. Untouched this iteration.

## Per-Crate READMEs

**Status**: met — the last stale install snippet was fixed this iteration

- READMEs present for all 12 crates/packages; registry metadata references them.
- **RESOLVED (iter 129)**: `packages/kotlin/README.md:13` now reads
    `implementation("io.iscc:iscc-lib-kotlin:0.5.0")`, and the file was added to `TARGETS` in
    `scripts/version_sync.py` so it can no longer rot. Verified independently:
    `uv run python scripts/version_sync.py --check` exits 0 with **21** `OK:` lines, and
    `grep -rn '0\.3\.1' packages/ docs/ README.md` returns nothing.
- **Small correction for planning**: `TARGETS` now holds **21** entries, not 22 as the iter-129
    handoff/learnings state (21 `OK:` lines, 21 tuples in the list).

## Documentation

**Status**: met (one low-priority cosmetic gap)

- Docs site, **11** `docs/howto/*.md` language guides, tabbed examples, llms-full.txt (22 pages),
    benchmarks page with speedup factors all present. No docs content changed this iteration.
- **`docs.yml` action bumps are still statically verified only** — the workflow triggers on push to
    `main`, so `upload-pages-artifact@v5` + `deploy-pages@v5` get their first real exercise on the
    next develop→main merge (PR #44 "Release 0.6.0" is open). If the docs deploy breaks after that
    merge, slice 4 is the first suspect.
- **Gap (low, CID skips)**: language logos in `docs/index.md` / howto headers — cosmetic only.

## Benchmarks

**Status**: met

- Criterion benches for all 10 `gen_*_v0` (+2) — **12** `bench_*` functions in `benchmarks.rs`;
    `Bench (compile check)` job GREEN on criterion 0.7 with `black_box` from `std::hint`. **18**
    pytest-benchmark functions in `tests/test_benchmarks.py`, speedup factors published (1.3x-158x)
    in `docs/benchmarks.md`.
- Second harness `iai_benches.rs` (iai-callgrind 0.16, **11** `bench_*` → 16 cases) enforcing the
    > 10% Ir regression gate — GREEN against the unmodified baseline.

## CI/CD and Publishing

**Status**: partially met — **CI GREEN**; one CID-doable gap remains (dependency freshness, in
progress)

- **LATEST CI RUN — SUCCESS.** origin/develop tip `9273743` (the iteration-129 review commit): **41
    check-runs, 21 distinct names, 0 non-success, 0 still running** — verified via
    `gh api repos/iscc/iscc-lib/commits/9273743.../check-runs?per_page=100`. (41 rather than ~20
    because every develop commit triggers both a `push` run and a `pull_request` run from the open
    develop→main PR #44.) HEAD `b83b1ee` is a +1 log-only commit touching `iterations.jsonl` only,
    unpushed — no code sits outside CI coverage.
- ci.yml has **19 job entries → 20 jobs → 21 distinct check names** (`python-test` is a 3.10/3.14
    matrix, `python` is an `if: always()` aggregator gate). Untouched this iteration.
- **Dependency refresh progress**: ✅ slice 1 Rust `Cargo.lock` (124), ✅ slice 2 Python `uv.lock`
    (125), ✅ slice 3 Rust direct pins (126), ✅ slice 4 GitHub Actions in ci.yml + docs.yml (127), ✅
    slice 5 JVM manifests (128), ✅ slice 6 Go module + `packages/kotlin/README.md` under version
    sync (129). Also closed as "verified current, no edit": `crates/iscc-napi/package.json`,
    `packages/dotnet/*/*.csproj`, `.pre-commit-config.yaml`, and the already-current action refs.
    **Remaining under the `normal` `[human]` issue**: Ruby `Gemfile` + gemspec (+`Gemfile.lock`),
    `release.yml` action refs (human-timed), and the deferred majors/migrations — ruff 0.16, magnus
    0.8, jni 0.22, xunit 3.x, Test.Sdk 18.x, Gradle wrapper 9, JUnit 6.x.
- **Standing correction for planning**: `release.yml` contains **no `astral-sh/setup-uv` step** —
    the claim in issues.md that its setup-uv needs `@v9.0.0` is wrong. Its actual lag is
    `actions/checkout@v4`, `download-artifact@v4`, `upload-artifact@v4` (must move as a pair),
    `setup-java@v4`, `setup-node@v5`, `setup-dotnet@v4`, `setup-python@v5`, `cache@v4`. Nothing in
    release.yml is exercised by a CID push, so it stays human-timed.
- **cargo-deny gate enforcing** (unchanged): root `deny.toml`, `Audit (cargo-deny)` job GREEN. A
    fresh live advisory can flip this red on any push with no code change (prefer
    `cargo update -p <crate>` over a `deny.toml` ignore).
- **Gap (v0.6.0)**: no automated dependency updates — `.github/dependabot.yml` and `renovate.json`
    both absent (deliberate; refresh is a manual per-release pass).
- v0.5.0 published; release workflow with 8 per-registry toggles + version sync (**21** `TARGETS`
    entries) in place. Two release-workflow reliability issues remain open (npm OIDC migration,
    single-registry re-trigger bug) — both human-gated `normal`.

## Open Issues (issues.md lists 7 — 0 critical, 5 normal, 2 low)

CI is green — no open issue is CI-blocking. Any open issue keeps the project IN_PROGRESS. One issue
is **new this iteration** (the Rust core Unicode divergence, source tag `[review]`).

CID-doable now (`normal`, `[human]`, spec'd):

- Dependency review and refresh across the project (spec: `.claude/context/specs/ci-cd.md` →
    "Dependency Freshness"). Slices 1-6 DONE; remaining slices listed above.

Needs a human policy decision (`normal`, `[review]`, HUMAN REVIEW REQUESTED — 2 of them):

- **Kotlin binding silently raised the consumer Kotlin floor to 2.3** (spec:
    `specs/kotlin-bindings.md`, which documents no floor). Options: accept and document "requires
    Kotlin 2.3+" (review agent's recommendation, docs-only), or hold `kotlin("jvm")` at 2.1.x — note
    pinning `languageVersion` alone is insufficient, the transitive `kotlin-stdlib:2.4.10` triggers
    the same error. Release-blocking for `io.iscc:iscc-lib-kotlin`, harmless on develop.
- **Rust core diverges from `iscc-core` and Go on Unicode 16/17 characters** (spec:
    `specs/rust-core.md` → conformance). Options: (a) pin the core to the reference's Unicode
    version, (b) declare a Unicode version in the spec + add post-15 conformance vectors, (c)
    document the divergence as out-of-contract. Has an upstream (ISO 24138 / `iscc-core`) dimension
    and will flip direction when CPython ships Unicode 16.

Release-workflow reliability (`normal`, `[human]`, human-gated):

- Migrate npm publishing to OIDC Trusted Publishing (needs npm-side trusted-publisher config first)
- Fix broken single-registry re-trigger in `release.yml`

Low (human-directed, CID skips):

- Release core as v1.0.0 — **held** by Titusz (stay on 0.5.x; flip Semver to enforcing at the cut).
- Add programming language logos to docs site — cosmetic.

## Next Milestone

**CI is green — no CI fix needed.** Both `[review]` issues are support/conformance-policy calls
reserved for Titusz, so the loop should not decide either. Recommended order:

1. **Dependency-refresh slice 7 — Ruby manifests** (`crates/iscc-rb/Gemfile`, `iscc_lib.gemspec`,
    `Gemfile.lock`). This is the last CID-actionable, fully-locally-verifiable refresh slice, and
    Ruby is exercised end-to-end by the `Ruby (magnus build, test)` job on every develop push.
    Guards: `magnus` 0.8 stays its own migration step; `rb_sys` in `Gemfile.lock` must keep
    matching the `oxidize-rb/actions/cross-gem` Docker image tag (a mismatch causes rbconfig
    errors); and do **not** touch `required_ruby_version` — a consumer floor is a support-policy
    decision, exactly the class of change that produced the open Kotlin issue.
2. **ruff 0.16 adoption** (alternative to 1, or immediately after) — self-contained and entirely
    local: `ruff check --fix` clears ~60, hand-fix the rest (mostly `_lowlevel.pyi` stub-style
    PIE790/PYI048/RUF022), then drop the `ruff<0.16` pin from `pyproject.toml`. Retires a
    documented hold-back.
3. **Kotlin floor follow-up — only once Titusz picks an option.** If option 1 (accept and document)
    is chosen the work is docs-only: state "requires Kotlin 2.3+" in `packages/kotlin/README.md`,
    `docs/howto/kotlin.md`, and the root README Kotlin section, and refresh the stale
    `jna:5.16.0@aar` lines (150, 165) in `specs/kotlin-bindings.md`.
4. **Unicode divergence follow-up — only once Titusz picks an option.** Whichever way it goes, the
    durable part is a *gate*: post-Unicode-15 conformance vectors so the choice is enforced instead
    of rediscovered. Note the differential technique that found it (dump outputs across the whole
    code space, swap one dep version under a `replace`/patch, `diff`) generalizes to any
    Unicode/locale-table dependency and costs ~2 minutes.
5. **`magnus` 0.8 and `jni` 0.22 migrations** — each its own dedicated step with a source rewrite
    (`crates/iscc-rb/src/lib.rs`, `crates/iscc-jni/src/lib.rs`); update the corresponding `# held:`
    comment in `Cargo.toml` when a hold-back is lifted.

Deferred, not autonomous: `release.yml` action refs, the Gradle wrapper major, JUnit 6.x, xunit 3.x
/ Test.Sdk 18.x — none is exercised by a CID push in a way that proves the change, and the first is
best opened once, by a human, bundled with the single-registry re-trigger fix near the next release.

Do NOT cut v1.0.0, flip the `Semver` gate to enforcing, or raise the workspace MSRV to 1.86 (which
would unlock criterion 0.8) — all three are human policy calls tied to the v1.0.0 cut. Guards: any
source change that adds a branch/loop to a covered function must refresh `.crap-baseline.json` in
the same step (the CRAP regression gate is CI-only, not in `mise run check`); watch for the
enforcing `Audit (cargo-deny)` gate turning red on a fresh live advisory; before bumping any action
to `@vN`, confirm the floating major tag exists via
`gh api repos/<owner>/<repo>/git/matching-refs/tags/v<N>`; and re-run Gradle after `./gradlew clean`
before believing a Kotlin build failure (this bind mount flakes on incremental state).
