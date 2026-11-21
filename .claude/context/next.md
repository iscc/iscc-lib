# Next Work Package

## Step: Kotlin plugins resolve from Maven Central; the .NET 10 leg skips setup-dotnet's LTS pre-pass

## Goal

Get CI green again and keep two CI legs from failing because of outside state the repository does
not pin. The Kotlin job on `332373c` went red when the Gradle plugin portal could not serve
`kotlin-gradle-plugins-bom:2.4.10`. The issue "The .NET 10 CI leg depends on which .NET is the
latest LTS" (`normal` [review]) breaks the 10.0 leg once .NET 12 ships.

## Alternatives Considered

- **Chosen:** CI robustness for Kotlin + .NET (2 files): CI red comes first, and both legs share the
    same defect (the result depends on whatever the outside source serves today)
- **Rejected:** `alg_cdc_chunks` overflow (Rust + Go + vector): a correctness fix, but a step that
    lands on red CI cannot be attributed. It is next

## Scope

- **Modify**: `packages/kotlin/settings.gradle.kts`, `.github/workflows/ci.yml` (`dotnet-test` job:
    the `setup-dotnet` input and the matrix comment)
- **Reference**: `packages/kotlin/build.gradle.kts`, `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs`
    (`Runtime_MatchesExpectedMajorVersion`), failed run
    https://github.com/iscc/iscc-lib/actions/runs/44908416407

## Not In Scope

- Bumping Kotlin, Gradle, JNA or any other pin. The resolution path is the fix, not the versions
- The three `setup-dotnet` steps in `release.yml` (lines ~955/1019/1065): they install only `8.0`
    into the default dir with no roll-forward or runtime-major assertion, so the latest LTS cannot
    change what they produce. They do not have this defect; leave them alone
- Raising the `net8.0` floor (human call) or changing the matrix legs or roll-forward values
- Adding a `dependencyResolutionManagement` block or moving the `repositories {}` block out of
    `build.gradle.kts`

## Implementation Notes

**Kotlin.** No `pluginManagement` block exists, so Gradle resolves `kotlin("jvm")` only through
`plugins.gradle.org`. The portal answers with a 303 to `repo.maven.apache.org`, and on the failed
run the BOM lookup failed inside that hop (log: "Could not parse POM
…plugins.gradle.org/…/kotlin-gradle-plugin-2.4.10.pom > Could not find
…kotlin-gradle-plugins-bom:2.4.10", searched only on the portal). Gradle resolves a POM's parent/BOM
from the repository where the child was found. The plugin marker
(`org/jetbrains/kotlin/jvm/org.jetbrains.kotlin.jvm.gradle.plugin/2.4.10/…pom`), the plugin and the
BOM all return 200 from `repo1.maven.org` (probed 2026-11-21). Add the block at the top of
`settings.gradle.kts`, with `mavenCentral()` **first** and `gradlePluginPortal()` second as the
fallback, and a short comment explaining why. `release.yml`'s Kotlin jobs build from the same
directory, so they pick up the block automatically.

**.NET.** `actions/setup-dotnet@v6` has `skip-lts-prepass` (action.yml at `v6`: "When true, skips
the LTS runtime pre-pass and installs only the requested dotnet-version. Default: false").
`src/installer.ts` at `v6`, lines ~810-840: the pre-pass runs
`install-dotnet --runtime dotnet --channel LTS --skip-non-versioned-files`, and the real install
that follows also passes `--skip-non-versioned-files`. With the pre-pass skipped, the `dotnet` muxer
therefore comes from the first SDK listed (8.0). It picks the highest `host/fxr` present, so on the
10.0 leg SDK 10 should still drive the build. This is unverified locally and CI is the check: the
10.0 leg's `Runtime_MatchesExpectedMajorVersion` must pass. Set `skip-lts-prepass: true` under
`with:` on the `dotnet-test` step. Rewrite the matrix comment so it says what actually holds: a
private install dir plus a skipped pre-pass, so a leg holds only the runtimes it lists. Do not name
a .NET 12 date.

These are CI and Gradle config files only. No Rust changes, so the CRAP and iai gates are untouched.

## Verification

- `grep -n 'pluginManagement' packages/kotlin/settings.gradle.kts` matches, and inside that block
    `mavenCentral()` comes before `gradlePluginPortal()`
- `cargo build -p iscc-uniffi && (cd packages/kotlin && ./gradlew test --refresh-dependencies)`
    exits 0. Force the run: no "UP-TO-DATE"-only test task
- `cd packages/kotlin && ./gradlew buildEnvironment --refresh-dependencies --info` shows
    `kotlin-gradle-plugins-bom` downloaded from `repo.maven.apache.org`/`repo1.maven.org`, not from
    `plugins.gradle.org` (quote the line in the handoff)
- `grep -n 'skip-lts-prepass: true' .github/workflows/ci.yml` matches inside the `dotnet-test` job
- `uv run python -c "import yaml,sys; yaml.safe_load(open('.github/workflows/ci.yml'))"` exits 0,
    and `mise run check` exits 0

## Done When

All verification criteria pass, and the handoff tells review to confirm on the pushed run that the
Kotlin job and both `dotnet-test` legs plus the `dotnet` aggregator are green, and that the 10.0
leg's setup-dotnet log shows no `--channel LTS` install.
