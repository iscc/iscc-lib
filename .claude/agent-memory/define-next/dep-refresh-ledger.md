---
name: dep-refresh-ledger
description: Per-slice ledger and hard-won gotchas for the v0.6.0 "Dependency review and refresh" issue — what each slice covered, what is held back and why, and what remains
metadata:
  type: project
---

# Dependency-refresh slice ledger (v0.6.0)

Fact: the `normal` `[human]` issue "Dependency review and refresh across the project" is worked
**one ecosystem per CID iteration** (~12 manifests). It cites no `[audit]` tag → no 8-file escape
valve; lockfiles are generated and cost 0 source-file budget.

**Why:** manifests drift between releases because there is no Dependabot/Renovate (deliberate). A
whole-repo refresh in one step would blow the 3-file cap and make a conformance regression
un-bisectable.

**How to apply:** pick the next unstarted slice below; hold a dep back with an inline documented
comment (`# held:` / `// held:` / XML `held:`) beside the pin, never by disabling a rule or gate.

## Golden rules learned the hard way

- **Never move a consumer floor inside a refresh slice.** MSRV, the `go` directive,
    `required_ruby_version`, `java-version`, `net8.0`, and a *published* binding's compiler version
    are support-policy decisions reserved for Titusz. Iter 128 broke this accidentally: KGP
    2.1.10→2.4.10 stamped `mv=[2,4,0]` into the published Kotlin jar and raised the consumer floor
    to Kotlin 2.3 (open `[review]` issue, HUMAN REVIEW REQUESTED). Cheap detectors:
    `javap -v -p <class> | grep mv=` on a built jar, plus a throwaway consumer project.
- **A deprecation IS a hard error in any dep bump** — `mise run lint` is
    `clippy --all-targets -D warnings` (iter 126, `criterion::black_box`).
- **A floating `@vN` GitHub Action tag is a publisher convention, NOT a guarantee.** Confirm with
    `gh api repos/<o>/<r>/git/matching-refs/tags/v<N>`; `releases/latest` is not proof. `setup-uv`
    publishes no floating major past v7 → exact tag `@v9.0.0`. Getting this wrong reddened CI
    mid-127.
- Every Rust slice is verified with the 4-gate set: `test` / `lint` / `audit` / `bench:iai:check`.

## Slices done

1. **124 — Rust `Cargo.lock`** (`cargo update`, ~100 transitive crates, all pins held).
2. **125 — Python `uv.lock`** (`uv lock --upgrade`, 40 pkgs incl. iscc-core 1.3.0, ty 0.0.63).
    Hold-back: `ruff<0.16` in `pyproject.toml`.
3. **126 — Rust direct pins.** criterion 0.5→0.7 (bench import moved to `std::hint::black_box`). The
    4 surviving `# held:` comments in root `Cargo.toml` are the authoritative record: criterion 0.8
    (needs rustc 1.86 > declared 1.85 — never raise MSRV for a dev-dep), magnus 0.8 (`old-api` off
    by default → `exception::runtime_error()` deprecated at 5 sites in
    `crates/iscc-rb/src/ lib.rs`), jni 0.22 (wholesale `JNIEnv`→`Env`/`EnvUnowned` rework per
    upstream `docs/0.22-MIGRATION.md`), uniffi 0.32 (needs Swift+Kotlin regen; no Swift toolchain
    locally). pyo3 0.29 is already latest.
4. **127 — GitHub Actions in `ci.yml` + `docs.yml`.** Residue: `.pre-commit-config.yaml` needs NO
    bump (both pinned repos already latest → the feared mdformat reformat wave is moot).
5. **128 — JVM manifests** (`crates/iscc-jni/java/pom.xml`, `packages/kotlin/build.gradle.kts`).
    Devcontainer HAS JDK 17 + Maven 3.8.7 (no `gradle` binary, but `./gradlew` works, `~/.gradle`
    warm ~516 MB; `~/.m2` empty → first `mvn` run downloads). Landed junit-jupiter 5.14.4, gson
    2.14.0, compiler 3.15.0, surefire 3.5.6, source 3.4.0, javadoc 3.12.0, gpg 3.2.8, KGP 2.4.10,
    JNA 5.19.1. HELD: `central-publishing-maven-plugin` 0.7.0 (its `deploy` goal runs only in a
    real Central publish — unverifiable locally and in CI). JUnit 6.1.2 deferred (platform
    artifacts renumbered 1.x→6.x). **JNA version is duplicated in 3 doc files** (`README.md`,
    `packages/kotlin/README.md`, `docs/howto/kotlin.md`) + junit/gson in
    `crates/iscc-jni/CLAUDE.md` → sync in the same step; `.claude/context/specs/kotlin-bindings.md`
    is human-owned → leave.
6. **129 — Go module + `version_sync.py` TARGETS fix.** Proven output-neutral across all 1.1M code
    points; surfaced the pre-existing Rust-core Unicode divergence (`[review]`, human-gated).
7. **130 — Ruby manifests** (`crates/iscc-rb/Gemfile` + `Gemfile.lock`; `rb_sys` pinned exactly).

## Slice 6 facts (Go)

- **Go 1.26.1 IS in the devcontainer.** Run from repo root with `-C` — never `cd`:
    `CGO_ENABLED=0 go test -C packages/go -count=1 ./...`, `go vet -C packages/go ./...`,
    `go mod tidy -C packages/go -diff` (exits 0/1 — a clean tidiness criterion).
- **`go list -m -u all` OVER-reports**: it lists test-only deps of deps (x/tools, x/mod,
    zeebo/assert) that never enter the build list, so "no updates available" is NOT a reachable
    verification criterion — grep explicit versions instead.
- Actual gaps at scoping: `golang.org/x/text` 0.34.0→0.40.0 (its own `go 1.25.0` < our 1.26.1, so no
    `toolchain` line should appear), `klauspost/cpuid/v2` 2.0.12→2.4.0 (indirect).
    `zeebo/blake3 v0.2.4` is already latest.
- **Conformance risk worth naming in next.md**: x/text ships Unicode tables and
    `packages/go/utils.go` uses `unicode/norm` NFKC inside `TextClean`, which feeds every
    `Gen*CodeV0`. The vendored `packages/go/testdata/data.json` vectors are the guard.

## Slice 7 facts (Ruby)

- **The devcontainer runs the Ruby gates end to end** (verified iter 130): ruby 3.1.2 (= CI's
    `ruby-version: '3.1'`), bundler 2.6.9, `libclang-14`, warm `crates/iscc-rb/vendor/bundle` (51
    MB, gitignored via `BUNDLE_PATH` in `crates/iscc-rb/.bundle/config`), prebuilt `iscc_rb.so`.
    `standardrb` exits 0 and `rake test` gives 111 runs / 299 assertions in \<1 s. No `-C` flag
    exists → use a subshell `(cd crates/iscc-rb && bundle …)`.
- The gemspec is **`iscc-lib.gemspec`** (hyphen), not `iscc_lib.gemspec` as older notes said.
- `bundle outdated --strict` is the good boolean freshness criterion (only versions the Gemfile
    constraints actually allow); plain `bundle outdated` over-reports held majors.
- **`rb_sys` must be pinned EXACTLY, not `~> 0.9`**: 0.9.123 bundles `rake-compiler-dock = 1.10.0`
    whose `cross_rubies` map matches the `oxidize-rb/actions/cross-gem` `tag: 0.9.123` in
    `release.yml`; 0.9.124+ ships a Ruby 4.0 build tool that breaks `RbSys::ExtensionTask` (see
    commit `1e4a30e`). Change the Gemfile pin and the workflow `tag:` together or not at all.
- **`minitest` 6.x requires Ruby >= 3.2** — blocked by the gem's `required_ruby_version >= 3.1.0` (a
    consumer floor, human-only). Document as held; never "solve" it by raising the floor.

## Empty / near-empty slices (verified iter 129 — do not churn)

- `crates/iscc-napi/package.json`: `@napi-rs/cli: ^3` already covers latest 3.7.4.
- `packages/dotnet/*/*.csproj`: test refs float on `17.*` / `2.*` wildcards. Only majors remain
    (xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x).

## Major bump A — xunit v3 + `Microsoft.NET.Test.Sdk` 18.x (scoped iter 164)

Only `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` + its 3 `.cs` test files + the package
`CLAUDE.md`. Verified while scoping (read-only, nuget.org API):

- **dotnet SDK 8.0.423 IS in the devcontainer** and nuget.org is reachable, so the whole slice is
    locally verifiable: `cargo build -p iscc-ffi` then the CI command
    `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug`.
- Latest stable at scoping: `xunit.v3` 3.2.2, `xunit.runner.visualstudio` 3.1.5,
    `Microsoft.NET.Test.Sdk` 18.8.1. **All three declare a `net8.0` group in their nuspec**, and
    MTSdk 18's own targets error only *below* net8.0 → **no consumer-floor move** (`net8.0` and CI
    `dotnet-version: '8.0'` stay). Check this the same way for the next .NET major.
- v3 renames the package (`xunit` → `xunit.v3`) and makes test projects **stand-alone executables**
    (`<OutputType>Exe</OutputType>`); `using Xunit;` and every `Assert` member used here survive.
- **Real hazard:** `ConformanceTests` `MemberData` yields `JsonElement` in `object[]`, which v3's
    data serializer cannot round-trip — the fallback is to yield the vector key `string` and look up
    the JSON in the test body. A **passed-count floor** in the criteria is what catches a silent
    collapse from one case per vector to one case per theory method.
- No `Directory.Build.props`, `global.json` or `TreatWarningsAsErrors` anywhere in the repo.

## Slice 8 — ruff 0.16 adoption, split into 3 sub-slices (scoped iter 131)

`uvx ruff@0.16.0 check .` = **104 errors over 5 non-test files** (0.16 widened the *default* rule
set — the repo has no `lint.select`, so PIE/PYI/RUF/I/EXE/PLW arrived for free). Over the 3-file
budget → sliced by *decision content*, not by file count:

- **A (iter 131) — config-free, mechanical.** `_lowlevel.pyi` 72 (PIE790+PYI048: every stub body is
    docstring + a lone `...`; delete the 36 `...` lines → both rules clear, `ruff format` stays a
    no-op, `ty` passes — prototyped) + `tests/test_new_symbols.py` 6 RUF059 (`_`-prefix 4 unpacked
    tuples; fix is "unsafe" only because it renames).
- **B (scoped iter 134) — the `# noqa` / security-gate cluster.** Resolution measured with
    `--config` overrides: `[tool.ruff.lint] extend-select = ["S", "C901"]` (**`extend-select`, not
    `select` — `select` replaces the E4/E7/E9/F default**) + delete exactly two dead directives
    (`tools/metrics.py` `# noqa: S603` on the static-list `git rev-parse` call — 0.16 refined S603
    to skip literal argv, verified unused at 0.15.22 *and* 0.16.0; `tools/cid.py`
    `# noqa: PLC0415`). 27 → 12 findings, 0 RUF100, 0.15.22 stays "All checks passed". **Real value:
    `S`/`C901` were pre-push-hook-only — CI's `uv run ruff check` never ran them.** `PLC0415` must
    NOT be selected (adds 5 `tests/` findings). **TRAP: a blanket `ruff@0.16 check --fix .` deletes
    the load-bearing `# noqa: S603/S607`** and reddens the pre-push `--select S` gate. Never plain
    `--fix`.
- **C (scoped iter 135) — isort config only.** Re-probed live: the two fidelity settings hold
    (`[tool.ruff] src = [".", "crates/iscc-py/python"]` else `iscc_lib` is third-party;
    `lint.isort.combine-as-imports = true` else the `__init__.py` fix explodes the single
    `from iscc_lib._lowlevel import (X as X, …)` re-export block into ~60 one-member imports — a
    110-line diff instead of one blank line). **Setting `src` moves the finding set, it does not
    only shrink it**: 3 test files go clean and `benchmarks/python/bench_iscc_lib.py` becomes dirty
    → 7 fixes over 6 files (3 non-test = exactly at budget), so the three one-liners could NOT ride
    along. Pinned 0.15.22 + `--extend-select I,RUF022,RUF100` reproduces the identical 7 findings,
    so the selection is enforceable before the pin drops, and `ruff format --check` stays clean
    after the fix (probed on a scratch copy).
- **D (scoped iter 136) — the three one-liners** = exactly 3 non-test files, so the `# held:`
    comment in `pyproject.toml` (a 4th file) could NOT ride along and stays stale for one more
    iteration. Probed live: `zip(ranges, ranges[1:])` → `pairwise` is exactly equivalent and the
    generator re-runs in \<1 s producing a byte-identical `unicode16.rs`; `run()` in
    `test_install.py` has 20 callers testing `.returncode`, so `check=False` (not `True`) is the
    behaviour-preserving fix; `tools/cid.py` is invoked only as `uv run tools/cid.py …` from 10
    `mise.toml` tasks, so the exec bit is purely additive and no hook
    (`check-executables-have-shebangs` is absent) reacts.
- **E (scoped iter 137) — drop `ruff<0.16` + `uv lock --upgrade-package ruff`** + retire the
    `# held:` comment. Probed live: linter side is a pure no-op (0.16 `check .`, hook-mode per-file,
    `--select S`, `--select C901` all exit 0; 13 `# noqa: S60x` intact). **The only real change is
    the formatter's file discovery: ruff 0.16 formats Python code blocks inside `.md`**, so the bare
    `uv run ruff format --check` used by `mise run lint` + `ci.yml` widens 25 → ~153 files. All 129
    tracked `.md` are already 0.16-clean → free coverage, no exclude config needed. `ruff check`
    does NOT lint Markdown. Gitignored `reference/` (32 dirty) and `cauldron/` (1 dirty) stay
    excluded from a bare `.` run — never pass them explicitly.

Reproduce baselines with `uvx ruff@0.16.0 …` — it needs no lockfile change and the cache is warm.

## Major bump B — Gradle wrapper 8.12.1 → 9.6.1 (scoped iter 165)

Measured while scoping, all read-only:

- Latest stable Gradle from `curl -s https://services.gradle.org/versions/current` = **9.6.1** (the
    `/versions/all` list needs `not snapshot/nightly/rcFor/milestoneFor` filtering).
- **A plugin's Gradle compatibility window is readable from its jar** — much better evidence than a
    docs table: fetch the highest `-gradleNN` variant
    (`repo1.maven.org/maven2/org/jetbrains/kotlin/kotlin-gradle-plugin/<v>/…-gradle88.jar`), unzip,
    `strings org/jetbrains/kotlin/gradle/internal/diagnostics/GradleCompatibilityCheck.class`. KGP
    2.4.10 holds exactly two versions: `minSupportedGradleVersion` 7.6.3 and
    `nextMinimumSupportedGradleVersion` 8.14.4 — **no upper bound**, so the older memory claim "KGP
    2.4.x supports 7.6.3–9.5.0" was a docs-table ceiling, not an enforced one, and 8.12.1 < 8.14.4
    is why the tree already emits KGP's "Deprecated Gradle Version" warning.
- Gradle 9 raises the **daemon** JVM floor to 17 (local 17.0.19, CI temurin 17 → no floor move) and
    **removes the auto-injected `junit-platform-launcher`**, which makes the existing explicit
    `testRuntimeOnly(...:1.14.4)` load-bearing for a new reason. Configuration cache is NOT on by
    default in 9.x (the 9.x upgrade guide's deprecations are all conditioned on it being enabled).
- Wrapper = 4 tracked tool-generated files (`gradle-wrapper.{properties,jar}`, `gradlew`,
    `gradlew.bat`); jar 43 KB < the 256 KB `check-added-large-files` cap. Gradle writes
    `gradlew.bat` CRLF but the repo stores it **LF** (`.gitattributes` `* text=auto eol=lf` +
    `mixed-line-ending --fix=lf`) — `mise run format` before `git add`.
- Only two places name the wrapper version: the launcher comment in
    `packages/kotlin/build.gradle.kts` and `packages/kotlin/CLAUDE.md` ("Gradle 8.x" prerequisite).
    No workflow, README, howto or mise pin does (`mise.toml` has no gradle/java entry at all).
- Kotlin suite size for count assertions: `build/test-results/test/TEST-…ConformanceTest.xml`
    `tests="9"` + `…UnicodeBoundaryTest.xml` `tests="13"`.

## Major bump C — JUnit 5.14.4 → 6.1.2, both JVM manifests (scoped iter 166)

Measured while scoping, all read-only (`repo1.maven.org` metadata + the 6.0.0 release notes at
`docs.junit.org/6.0.0/release-notes/`, dumped with a regex tag-stripper):

- **JUnit 6 gives Platform, Jupiter and Vintage one shared version number**, so
    `junit-platform-launcher` jumps 1.14.4 → **6.1.2** alongside `junit-jupiter` 5.14.4 → **6.1.2**.
    Filter `maven-metadata.xml` by `^[0-9]+(\.[0-9]+)*$` — the `<latest>` field includes milestones.
- **Read the release notes' "removed support for" line instead of bumping the build plugin**: JUnit
    6 drops Maven Surefire/Failsafe < 3.0.0 only, so surefire 3.5.6 (already latest) stays put.
    Baselines are Java 17 + Kotlin 2.2 → no floor move against JDK 17 / KGP 2.4.10.
- `.module` JSON on Maven Central carries `"org.gradle.jvm.version"` — the cheapest proof of a JVM
    floor. `junit-jupiter-api` 6.1.2 = 17, and its pom pulls no `kotlin-stdlib`.
- Removed in 6.0: `junit-platform-runner`, `junit-platform-jfr`, ConsoleLauncher shorthands — none
    used here. `@Test`/`@TestFactory`/`DynamicTest`/`@BeforeAll`/`Assertions.*` all survive.
- Doc sync surface is exactly two files (`crates/iscc-jni/CLAUDE.md` lines 30/80/86/87,
    `packages/kotlin/CLAUDE.md` line 62); `specs/java-bindings.md` also says "JUnit 5" but is
    human-owned — leave it.
- Count assertions: Kotlin `tests="9"` + `tests="13"`; Maven's total is **dynamic**
    (`@TestFactory`), so the honest baseline is a `mvn test` run on the unmodified tree *before* the
    edit.

## Major bump D — magnus 0.7 → 0.8, Ruby binding (scoped iter 167)

Measured while scoping, all read-only (`cargo search`, then
`curl -sL https://static.crates.io/crates/magnus/magnus-0.8.2.crate | tar xz` — the crate ships its
own `CHANGELOG.md`, which is the cheapest breaking-change list there is):

- **A `#[cfg_attr(not(feature = "old-api"), deprecated(note = …))]` bump is a grep-able migration,
    not a rewrite**: the symbols still exist, and each carries its replacement in the note. magnus
    0.8.2 has 340 such attributes; only two families reach `crates/iscc-rb/src/lib.rs` —
    `magnus::exception::runtime_error()` → `Ruby::exception_runtime_error()` and
    `RString::from_slice` → `Ruby::str_from_slice` (5 sites each). `RString::as_slice` is NOT
    deprecated. Confirm each replacement really lives in an `impl Ruby` block before writing it.
- No floor moves: magnus 0.8 MSRV 1.65, "Ruby 3.0-3.4 fully supported" (README), and its
    `rb-sys >= 0.9.113` requirement is already met by `Cargo.lock`'s 0.9.128 — so the exact-pinned
    `rb_sys` **gem** 0.9.123 (cross-gem Docker tag lockstep) does not enter the step.
- Doc surface is one line: `crates/iscc-rb/CLAUDE.md:7` ("Magnus 0.7.1"). No README/howto/spec names
    the version.
- `crates/iscc-rb/test/test_conformance.rb` has **zero** `def test_` (cases are defined dynamically
    from the vectors), so the suite total is not greppable — the honest baseline is a `rake test`
    run on the unmodified tree, same pattern as Maven's `@TestFactory` total at 166.

## Remaining after major bump D

`release.yml` GHA refs (97 `uses:`; `upload-artifact@v4` ↔ `download-artifact@v4` move as a pair;
nothing in it is exercised by a CID push → human-timed), `uniffi` 0.32 (Swift+Kotlin regen — the "no
Swift toolchain" veto died at 161, so re-scope it on evidence), and **`jni` 0.22**: rust-version
1.85.0 = exactly our MSRV, 41 `JNIEnv` sites in a 1,065-line file, and a 778-line upstream
`docs/0.22-MIGRATION.md` inside the `.crate` tarball. Budget a scoping pass for that doc: the step
has to choose an `ErrorPolicy` and a `with_env` closure shape for ~40 `extern "system"` natives,
which is design work, not a rename sweep.

## Handy version-lookup commands

- crates.io: `cargo search <crate> --limit 1`; changelog via
    `curl -sL https://static.crates.io/crates/<c>/<c>-<ver>.crate | tar xz`
- Maven **stable**: `repo1.maven.org/maven2/<path>/maven-metadata.xml` filtered by
    `^[0-9]+(\.[0-9]+)*$` (the `<latest>` field includes betas/milestones)
- npm: `curl -s https://registry.npmjs.org/<pkg>` → `.dist-tags.latest`
- Go: `curl -s https://proxy.golang.org/<module>/@v/list | sort -V | tail`, and
    `https://proxy.golang.org/<module>/@v/<ver>.mod` for its `go` directive
- GitHub Actions: `gh api repos/<o>/<r>/git/matching-refs/tags/v<N>`

Network access works from the devcontainer.
