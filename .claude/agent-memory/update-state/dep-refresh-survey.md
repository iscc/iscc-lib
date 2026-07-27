---
name: dep-refresh-survey
description: Per-ecosystem dependency-pin inventory (GHA refs, tooling pins, per-binding manifests) backing the sliced "Dependency review and refresh" issue
metadata:
  type: project
---

# Dependency-refresh survey (11 slices closed; last updated iteration 166)

Backing detail for the sliced `normal` `[human]` issue "Dependency review and refresh across the
project" (spec: `.claude/context/specs/ci-cd.md` → Dependency Freshness). MEMORY.md keeps only a
pointer here.

**Why:** the project has no Dependabot/Renovate, so manifests drift between releases and the refresh
is a manual per-release pass, sliced one ecosystem at a time to keep each step reviewable.

**How to apply:** when assessing the CI/CD section, re-verify the lines relevant to the slice that
just landed instead of re-surveying everything; refresh this file when a slice lands.

## GitHub Actions

- **ci.yml + docs.yml — CURRENT since slice 4 (iter 127)**: `actions/checkout@v7`,
    `setup-python@v7`, `setup-node@v7`, `setup-java@v5`, `setup-go@v7`, `setup-dotnet@v6`,
    `upload-artifact@v7`, `github/codeql-action/upload-sarif@v4`, `upload-pages-artifact@v5` +
    `deploy-pages@v5` (paired), `astral-sh/setup-uv@v9.0.0`.
- `astral-sh/setup-uv` uses an **exact tag** — upstream publishes no floating major past v7, so
    `@v9` does not resolve. Confirm floating tags with
    `gh api repos/<owner>/<repo>/git/matching-refs/tags/v<N>`; `releases/latest` is not proof.
- Deliberately untouched and already current: `dtolnay/rust-toolchain@stable`,
    `Swatinem/rust-cache@v2`, `taiki-e/install-action@v2`, `ruby/setup-ruby@v1`,
    `obi1kenobi/cargo-semver-checks-action@v2`.
- **release.yml — CURRENT since slice 9 (iter 140)**, 97 `uses:` total: `checkout@v7` (23),
    `download-artifact@v8` (20) ↔ `upload-artifact@v7` (11) — these two MUST move as a pair,
    `setup-java@v5` (6), `setup-node@v7` (5), `setup-dotnet@v6` (3), `action-gh-release@v3` (2),
    `setup-python@v7` (2), `cache@v6` (1). **It contains NO `astral-sh/setup-uv` step and needs
    none** — it invokes no `uv`/`uvx`; issues.md and the iteration-127 handoff claimed otherwise and
    both were wrong (corrected in issues.md at iter 140).
- `rubygems/configure-rubygems-credentials` was pinned `@v2.1.0` at iter 162 (release.yml:897, with
    a `# exact tag:` comment). The repo now has **zero `@main` refs** across ci/docs/release.
- **Verifying a bump statically** (release.yml is `workflow_dispatch`-only, so this is all there
    is): resolve `git/ref/tags/<vN>`; fetch each new major's `action.yml` at the tag and confirm
    every `with:` key is still a declared `inputs` key and every `steps.<id>.outputs.<x>` the
    workflow reads is still an `outputs` key; read every *intervening* major's release notes for
    **default** changes (an input surviving is not its default surviving). Accepted risk →
    `decisions.md` 2026-07-25.
- `docs.yml` only triggers on push to `main`, so its bumps are statically verified until the next
    develop→main merge.

## Tooling pins

- `.pre-commit-config.yaml`: exactly 2 pinned repos — `pre-commit/pre-commit-hooks` `rev: v6.0.0`
    and `executablebooks/mdformat` `rev: 1.0.0`, both current. Everything else is `repo: local`.
    Risk if a future mdformat rev lands: it can reformat every Markdown file in the repo.
- `mise.toml`: **no `[tools]` section** — out of scope for the refresh.
- Only 2 `package.json` files: `crates/iscc-napi/` (hand-written, single `@napi-rs/cli: ^3` dep) and
    `crates/iscc-wasm/pkg/` (generated).

## Slice history

1. Rust `Cargo.lock` via `cargo update` (iter 124, ~100 crates).

2. Python `uv.lock` via `uv lock --upgrade` (iter 125; iscc-core 1.3.0 = zero vector drift).

3. Rust direct pins (iter 126; criterion 0.5→0.7 + `std::hint::black_box`; 4 `# held:` comments).

4. GHA refs in ci.yml + docs.yml (iter 127; 9 refs, setup-uv exact tag).

5. JVM manifests (iter 128; 11 pins — see below; raised the Kotlin consumer floor → `[review]`
    issue).

6. Go module (iter 129; `x/text` 0.34.0→0.40.0, `cpuid/v2` 2.0.12→2.4.0, new indirect `x/sys`
    0.47.0; `go 1.26.1` floor + `zeebo/blake3` untouched). Also wired `packages/kotlin/README.md`
    into version_sync TARGETS. **Technique worth reusing**: proved output-neutrality by dumping
    `TextClean`/`TextCollapse` for all 1,112,032 code points under both versions via a throwaway
    module with `replace` — turns "no vector regressed" into "no input can regress" (~2 min). It is
    what surfaced the pre-existing Unicode 16/17 divergence (see MEMORY.md).

7. Ruby manifests (iter 130; `Gemfile.lock` via `bundle update`, rb_sys tightened to exact
    `0.9.123`).

8. ruff 0.16 adoption, sub-slices A–E (iters 131, 134, 135, 136, 137) → `lint-tooling.md`.

9. GHA refs in release.yml (iter 140; the 9 refs listed above).

**Slice 10 — dotnet test framework major (iter 164):** `xunit` → `xunit.v3` at `3.*`,
`xunit.runner.visualstudio` at `3.*`, `Microsoft.NET.Test.Sdk` at `18.*`, plus the
`<OutputType>Exe</OutputType>` that v3 requires (its test projects are stand-alone executables).
Zero test-source edits; v2 and v3 both report exactly 104 results — that pre-vs-post equality, not
the coverage floor, is what proves row-level enumeration survived. VSTest runner mode kept on
purpose: `dotnet test -e` (VSTest-only) carries `LD_LIBRARY_PATH` for the P/Invoke lib →
`decisions.md` 2026-07-27.

Closed as **verified current, no edit needed** (iter 129): napi `package.json` (`@napi-rs/cli: ^3`
covers 3.7.4) — editing it is churn.

**Slice 11 — Kotlin Gradle wrapper major (iter 165):** 8.12.1 → **9.6.1**. Only `packages/kotlin/`
moved: the four `wrapper`-task outputs (`gradle-wrapper.jar` + `.properties` + `gradlew` +
`gradlew.bat`), a comment block in `build.gradle.kts`, one `CLAUDE.md` prerequisite line. The
`.properties` gains `retries=0` / `retryBackOffMs=500` — tool-generated defaults, not hand-added.
**Gradle 9 no longer auto-injects a JUnit platform launcher**, so the existing
`testRuntimeOnly junit-platform-launcher` is now load-bearing rather than merely version-aligning.
Test totals unmoved (9 + 13 cases). The CI `kotlin` job downloads 9.6.1 on a fresh checkout — the
green run on the review sha is the proof.

**All 9 original slices plus the dotnet and Gradle-wrapper majors are CLOSED.** Remaining under the
parent issue, authorized for CID one per step: **JUnit 6.x** (still 5.14.4 in BOTH
`packages/kotlin/build.gradle.kts` and `crates/iscc-jni/java/pom.xml`; the platform artifacts
renumber 1.14.x → 6.x, so the launcher pin stops being a 1.x number — two build systems,
splittable), then `jni` 0.22 and `magnus` 0.8 (source rewrites, riskiest).

## Per-binding manifests

- `crates/iscc-jni/java/pom.xml` — **refreshed iter 128**: junit-jupiter 5.14.4, gson 2.14.0,
    maven-compiler-plugin 3.15.0, maven-surefire-plugin 3.5.6; release-only profile maven-source
    3.4.0, maven-javadoc 3.12.0, maven-gpg 3.2.8. `central-publishing-maven-plugin` **held at
    0.7.0** (inline `held:` XML comment) — its `deploy` goal only runs in a real Central publish, so
    nothing local or in CI can validate a bump. `mvn -Prelease package` exercises source+javadoc but
    **not** gpg or central-publishing.
- `packages/kotlin/build.gradle.kts` — **refreshed iter 128**: `kotlin("jvm") 2.4.10`, JNA 5.19.1,
    junit-jupiter 5.14.4 + required `testRuntimeOnly junit-platform-launcher:1.14.4` (mandatory
    under the Gradle 9.6.1 wrapper from iter 165), gson 2.14.0; JUnit 6.x `held:`. **Consumer-floor
    trap**: the KGP bump stamps jar metadata `mv=[2,4,0]` and publishes `kotlin-stdlib:2.4.10` →
    Kotlin < 2.3 consumers fail to compile (2.1.10 and 2.2.21 fail, 2.3.21 passes).
    `java-version: '17'`, the jvmToolchain, `required_ruby_version`, the `go` directive and `net8.0`
    are all support-policy decisions, not dependency pins — never move them in a refresh step.
    Detect this class with `javap -v -p <class> | grep mv=` plus a throwaway consumer project.
- `crates/iscc-rb/Gemfile` — pessimistic constraints only (minitest `~> 5.0`, rake `~> 13.0`,
    rake-compiler `~> 1.2`, rb_sys `~> 0.9`, standard `~> 1.0`, rubocop-minitest `~> 0.36`); gemspec
    declares `required_ruby_version >= 3.1.0`. rb_sys must match the `oxidize-rb/actions/cross-gem`
    Docker image tag.
- `packages/go/go.mod` — **refreshed iter 129**: `go 1.26.1` (consumer floor, untouched),
    `github.com/zeebo/blake3 v0.2.4`, `golang.org/x/text v0.40.0`, indirect
    `github.com/klauspost/cpuid/v2 v2.4.0` + `golang.org/x/sys v0.47.0`. All at latest published; no
    hold-back. Dep `go` directives (x/text 1.25.0, x/sys 1.25.0, cpuid 1.24.0) all sit below 1.26.1.
- `packages/dotnet/*/*.csproj` — `net8.0` (support policy, never move in a refresh); test refs float
    on `3.*` / `18.*` since iter 164. **The only ecosystem in the repo with no lockfile** (Cargo,
    uv, Gemfile, Gradle all pin), so CI can resolve an unreviewed 3.x/18.x;
    `RestorePackagesWithLockFile` is the fix if a float ever reds CI. Not filed as an issue — the
    wildcard style is long-standing.

## Documented hold-backs (do not "fix" these)

- Root `Cargo.toml`, 4 `# held:` comments: criterion 0.8 (needs rustc 1.86 vs `rust-version 1.85`),
    jni 0.22 (wholesale iscc-jni API rework), magnus 0.8 (drops `old-api`, deprecates
    `exception::runtime_error()` — 5 call sites), uniffi 0.32 (needs Swift/Kotlin regen). Plus a
    `# note:` on pyo3 tying bumps to issue #41.
- `pyproject.toml`: **zero** `# held:` comments — the `ruff<0.16` pin died at iter 137 when slice 8
    landed.
