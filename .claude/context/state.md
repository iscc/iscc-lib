<!-- assessed-at: 2dee9133fe23a5454cdb4407c8a6898768b15e5d -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — CI GREEN; dependency refresh has closed every per-ecosystem slice

v0.5.0 is released across all registries and all 12 language bindings meet their core criteria.
Iteration 130 landed **slice 7 of the dependency refresh** (Ruby `Gemfile` + `Gemfile.lock`:
`bundle update` for the dev deps, `rb_sys` tightened to an exact `0.9.123` and `minitest ~> 5.0`
both under `# held:` comments), which closes the last per-ecosystem slice. **CI is fully green on
the develop tip (41 check-runs, 21 distinct names, 0 non-success, 0 running.)** Two `[review]`
issues (Kotlin consumer floor, Rust-core Unicode divergence) remain parked on Titusz; ruff 0.16 is
the only fully-autonomous dependency work left.

## Rust Core Crate

**Status**: partially met — CI GREEN, but **two** unmet items: the held semver/v1.0.0 criterion and
the Unicode conformance divergence

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols, all
    vendored `iscc-core/data.json` vectors passing — `Rust (fmt, clippy, test)` job GREEN. Workspace
    version `0.5.0` (unchanged). 11 source modules in `crates/iscc-lib/src/`, **320** `#[test]`
    functions (re-counted). **No Rust source touched this iteration** — the only non-context change
    outside `crates/iscc-rb/` was a comment reword on the `magnus` `# held:` block in the root
    `Cargo.toml` (it pointed at "the Ruby dependency slice", which completed without touching
    magnus; now reads "in its own dedicated step"). `grep -c '# held' Cargo.toml` → **4**,
    unchanged.
- **Unmet: Unicode-version divergence from the reference (filed iter 129, open `[review]` issue).**
    Target criterion "All Tier 1 functions produce output matching their iscc-core counterparts" is
    violated outside the vendored vectors. `crates/iscc-lib/src/utils.rs:28-32` strips
    `GeneralCategory::Unassigned` via `unicode-general-category` **1.1.0 (Unicode 16)** (and
    `unicode-normalization` 0.1.25 = Unicode 17), while Go's stdlib tables are 15.0.0 and CPython
    3.13 `unicodedata` is 15.1.0 — so the Rust core *keeps* characters that Go and `iscc-core`
    *strip* (`Ɤ` U+A7CB is the canonical repro). Swept code space: **5,813** `text_clean` and
    **5,750** `text_collapse` disagreements, reproduced end-to-end (Meta-Code and Text-Code differ).
    All 10 non-Go bindings inherit the core's behaviour; `packages/go` matches the reference. No
    gate catches it — every vendored vector predates Unicode 16. Choosing a Unicode version is a
    conformance-policy call reserved for Titusz (it flips direction once CPython 3.14 ships Unicode
    16), so the loop must not pick unilaterally.
- **Rust pin hold-backs documented (iter 126, still standing)**: criterion 0.8 (needs rustc 1.86 vs
    declared `rust-version = "1.85"`), jni 0.22 (wholesale API rework), magnus 0.8 (drops
    `old-api`), uniffi 0.32 (needs Swift+Kotlin regeneration), plus a `# note:` on `pyo3` tying
    bumps to issue #41.
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
    `crates/iscc-py/src/lib.rs` = **12** (re-verified).
- **`uv.lock` refreshed (iter 125, slice 2)**: 40 packages incl. iscc-core `1.3.0`, ty 0.0.63,
    maturin 1.14.1, prek 0.4.11.
- **Documented hold-back, now the next actionable slice**: `pyproject.toml:27` pins `ruff<0.16` with
    an inline `# held:` comment (0.16 expands default lint rules → 104 new errors at iter 125, ~72
    of them in `_lowlevel.pyi`). Re-measure before starting — the lock has moved since.
- **aarch64 wheels wired (issue #49 DONE, iter 123)**; first real aarch64 wheel ships at v0.6.0.

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols with TypeScript declarations; `Node.js (napi build, test)` job GREEN
    (`actions/setup-node@v7`, node 20). `crates/iscc-napi/package.json` closed as "verified current"
    (its lone `@napi-rs/cli: ^3` devDependency already floats over the 3.x line).

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
    version from `go.mod`), `CGO_ENABLED=0` holds. Untouched this iteration.
- `go.mod`: `golang.org/x/text v0.40.0` (direct), `github.com/zeebo/blake3 v0.2.4`, indirect
    `github.com/klauspost/cpuid/v2 v2.4.0` and `golang.org/x/sys v0.47.0` — all latest published.
    The `go 1.26.1` **consumer floor** was deliberately untouched.
- The `x/text` bump was proven **output-neutral, not merely vector-green**:
    `TextClean`/`TextCollapse` are byte-identical across all 1,112,032 code points under 0.34.0 and
    0.40.0.
- `packages/go/iscc_id.go` provides `EncodeIsccID`/`DecodeIsccID` + `IsccIDv1Result`; `codec.go`
    adds `VSV1` and a MainType-ID-only Version=1 relaxation in `decodeHeader`. `IsccDecode` retains
    both "too short" and "too long" rejection branches.
- **Note**: Go currently matches `iscc-core` on post-Unicode-15 text only because Go's stdlib tables
    are at 15.0.0. That alignment is incidental and inverts when Go 1.27 lands (or when CPython 3.14
    becomes the reference runtime) — see the Rust Core Unicode item.

## Ruby Bindings

**Status**: met — dependency refresh (slice 7) landed this iteration

- 32 Tier 1 symbols via Magnus; `Ruby (magnus build, test)` job GREEN on the develop tip; version
    synced. `magnus` stays pinned at 0.7 with a documented hold-back (0.8 drops `old-api`, needing
    an `exception::runtime_error()` call-site refactor across 5 sites in `crates/iscc-rb/src/lib.rs`
    — explicitly **not** done in this slice; the `Cargo.toml` comment now says so).
- **`Gemfile.lock` refreshed by `bundle update`** (verified in the diff): rake `13.4.2`, standard
    `1.56.0`, rubocop `1.88.2`, rubocop-ast `1.50.0`, rubocop-minitest `0.40.0`, json `2.21.1`,
    parser `3.3.12.0`, parallel `1.28.0`, regexp_parser `2.12.0`, language_server-protocol
    `3.17.0.6`. Review confirmed each is the newest published release and all declare a Ruby floor ≤
    3.1. 111 tests, 0 failures; `bundle outdated --strict` clean; `standardrb` clean.
- **Two documented hold-backs added** — `grep -c 'held:' crates/iscc-rb/Gemfile` → **2**:
    - `rb_sys` tightened `~> 0.9` → exact **`0.9.123`**, because it bundles
        `rake-compiler-dock (= 1.10.0)` which must match `tag: 0.9.123` at `release.yml:853` (both
        verified in-tree this iteration). Registry metadata confirms `~> 0.9` would have floated to
        0.9.128 → dock 1.12.0. `rb_sys` now appears pinned in **three** places that must move
        together: `Gemfile`, `Gemfile.lock`, `release.yml` — documented in `crates/iscc-rb/CLAUDE.md`
        (updated this iteration).
    - `minitest ~> 5.0` held because minitest 6.x requires Ruby ≥ 3.2, above the gem's declared floor.
- `crates/iscc-rb/iscc-lib.gemspec` needed **no** change — it declares no dev dependencies, only the
    human-owned `required_ruby_version = ">= 3.1.0"` (verified untouched). "Gemfile + gemspec" is
    now fully closed.

## C# / .NET Bindings

**Status**: met

- 32 public symbols via P/Invoke over the C FFI; `C# / .NET (dotnet build, test)` job GREEN
    (`actions/setup-dotnet@v6`, SDK 8.0). Manifests closed as "verified current" — both projects
    target `net8.0` and the test project's package refs already float; only the xunit 3.x / Test.Sdk
    18.x majors remain, each its own step.

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
    `Kotlin (gradle build, test)` job GREEN. Untouched this iteration.
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

**Status**: met

- READMEs present for all 12 crates/packages; registry metadata references them. The last stale
    install snippet (`packages/kotlin/README.md`) was fixed iter 129 and is now under
    `scripts/version_sync.py`.
- **Standing correction**: `scripts/version_sync.py` `TARGETS` holds **21** entries — re-verified
    this iteration (`uv run python scripts/version_sync.py --check` → exit 0, **21** `OK:` lines).
    issues.md still says "22 targets now" (line 55); that number is wrong and should not be
    propagated.

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
- Second harness `iai_benches.rs` (iai-callgrind 0.16, **11** `bench_*` → 16 cases) enforcing the Ir
    regression gate (fails above 10%) — GREEN against the unmodified baseline.

## CI/CD and Publishing

**Status**: partially met — **CI GREEN**; one CID-doable gap remains (dependency freshness, one
slice left)

- **LATEST CI RUN — SUCCESS.** origin/develop tip `03a92f5` (the iteration-130 review commit): **41
    check-runs, 21 distinct names, 0 non-success, 0 still running** — verified via
    `gh api repos/iscc/iscc-lib/commits/03a92f5.../check-runs?per_page=100`. (41 rather than ~20
    because every develop commit triggers both a `push` run and a `pull_request` run from the open
    develop→main PR #44.) The 3 unpushed commits on HEAD (`004cdcc`, `9e00007`, `2dee913`) touch
    only `iterations.jsonl` and `metrics.jsonl` — no code sits outside CI coverage.
- ci.yml has **19 job entries → 20 jobs → 21 distinct check names** (`python-test` is a 3.10/3.14
    matrix, `python` is an `if: always()` aggregator gate). Untouched this iteration.
- **Dependency refresh — every per-ecosystem slice is now DONE**: ✅ 1 Rust `Cargo.lock` (124), ✅ 2
    Python `uv.lock` (125), ✅ 3 Rust direct pins (126), ✅ 4 GitHub Actions in ci.yml + docs.yml
    (127), ✅ 5 JVM manifests (128), ✅ 6 Go module + `packages/kotlin/README.md` under version sync
    (129), ✅ 7 Ruby `Gemfile` + `Gemfile.lock` (130). Also closed as "verified current, no edit":
    `crates/iscc-napi/package.json`, `packages/dotnet/*/*.csproj`, `.pre-commit-config.yaml`, and
    the already-current action refs. **Remaining under the `normal` `[human]` issue**: ruff 0.16
    adoption (the last fully-local, CID-doable item), `release.yml` action refs (human-timed), and
    the deferred majors/migrations — magnus 0.8, jni 0.22, uniffi 0.32, criterion 0.8, xunit 3.x,
    Test.Sdk 18.x, Gradle wrapper, JUnit 6.x.
- **Standing correction for planning**: `release.yml` contains **no `astral-sh/setup-uv` step** —
    the claim in issues.md that its setup-uv needs `@v9.0.0` is wrong. Its actual lag is
    `actions/checkout@v4`, `download-artifact@v4`, `upload-artifact@v4` (must move as a pair),
    `setup-java@v4`, `setup-node@v5`, `setup-dotnet@v4`, `setup-python@v5`, `cache@v4`. Nothing in
    release.yml is exercised by a CID push, so it stays human-timed.
- **cargo-deny gate enforcing** (unchanged): root `deny.toml`, `Audit (cargo-deny)` job GREEN. A
    fresh live advisory can flip this red on any push with no code change (prefer
    `cargo update -p <crate>` over a `deny.toml` ignore).
- **Gap (v0.6.0)**: no automated dependency updates — `.github/dependabot.yml` and `renovate.json`
    both confirmed absent (deliberate; refresh is a manual per-release pass).
- v0.5.0 published; release workflow with 8 per-registry toggles + version sync (**21** `TARGETS`
    entries) in place. Two release-workflow reliability issues remain open (npm OIDC migration,
    single-registry re-trigger bug) — both human-gated `normal`.

## Open Issues (issues.md lists 7 — 0 critical, 5 normal, 2 low)

CI is green — no open issue is CI-blocking. Any open issue keeps the project IN_PROGRESS. **No issue
was opened or closed this iteration**; the iteration-130 audit run appended a metrics snapshot only
and filed nothing.

CID-doable now (`normal`, `[human]`, spec'd):

- Dependency review and refresh across the project (spec: `.claude/context/specs/ci-cd.md` →
    "Dependency Freshness"). Slices 1-7 DONE; only ruff 0.16 remains autonomously verifiable.

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
reserved for Titusz, so the loop must not decide either. Recommended order:

1. **Adopt ruff 0.16 and retire the `ruff<0.16` hold-back** — the last self-contained, fully
    locally-verifiable dependency slice. Re-measure the baseline first (`uvx ruff@0.16.0 check .`
    reported 104 errors at iter 125, 72 in `_lowlevel.pyi`; the lock has moved since), run
    `uv run ruff check --fix` for the auto-fixable set, hand-fix the remainder (stub-style
    `PIE790`/`PYI048`/`RUF022`), drop the pin in `pyproject.toml:27`, then confirm `mise run check`
    and the pre-push Ruff hooks (`S` security, `C901` complexity) stay green. **Fix the code, never
    widen the lint config** — `per-file-ignores`/`# noqa` added to make 0.16 pass is gate weakening
    and will be rejected.
2. **Kotlin floor follow-up — only once Titusz picks an option.** If option 1 (accept and document)
    is chosen the work is docs-only: state "requires Kotlin 2.3+" in `packages/kotlin/README.md`,
    `docs/howto/kotlin.md`, and the root README Kotlin section, and refresh the stale
    `jna:5.16.0@aar` lines (150, 165) in `specs/kotlin-bindings.md`.
3. **Unicode divergence follow-up — only once Titusz picks an option.** Whichever way it goes, the
    durable part is a *gate*: post-Unicode-15 conformance vectors so the choice is enforced instead
    of rediscovered. The differential technique that found it (dump outputs across the whole code
    space, swap one dep version, `diff`) generalizes to any Unicode/locale-table dependency and
    costs ~2 minutes.
4. **`magnus` 0.8 and `jni` 0.22 migrations** — each its own dedicated step with a source rewrite
    (`crates/iscc-rb/src/lib.rs`: 5 `exception::runtime_error()` call sites →
    `Ruby::exception_runtime_error()`; `crates/iscc-jni/src/lib.rs`). Update the corresponding
    `# held:` comment in `Cargo.toml` when a hold-back is lifted.
5. **Housekeeping (cheap, optional)**: fix the "22 targets" figure in issues.md line 55 — the real
    `version_sync.py` `TARGETS` count is 21.

Deferred, not autonomous: `release.yml` action refs, the Gradle wrapper major, JUnit 6.x, xunit 3.x
/ Test.Sdk 18.x, uniffi 0.32, criterion 0.8 — none is exercised by a CID push in a way that proves
the change; the release.yml refs are best opened once, by a human, bundled with the single-registry
re-trigger fix near the next release. After ruff 0.16 the loop is essentially out of autonomous
dependency work and will be waiting on the two `[review]` decisions.

Do NOT cut v1.0.0, flip the `Semver` gate to enforcing, or raise the workspace MSRV to 1.86 (which
would unlock criterion 0.8) — all three are human policy calls tied to the v1.0.0 cut. Guards: any
source change that adds a branch/loop to a covered function must refresh `.crap-baseline.json` in
the same step (the CRAP regression gate is CI-only, not in `mise run check`); watch for the
enforcing `Audit (cargo-deny)` gate turning red on a fresh live advisory; any `rb_sys` move must
update `Gemfile`, `Gemfile.lock` **and** `tag:` in `release.yml` together; and re-run Gradle after
`./gradlew clean` before believing a Kotlin build failure (this bind mount flakes on incremental
state).
