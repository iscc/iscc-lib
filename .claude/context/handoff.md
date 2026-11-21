## 2026-11-21 — Kotlin plugins resolve from Maven Central; the .NET 10 leg skips setup-dotnet's LTS pre-pass

**Done:** Added a `pluginManagement { repositories { mavenCentral(); gradlePluginPortal() } }` block
with a comment to the top of `packages/kotlin/settings.gradle.kts`, so `kotlin("jvm")` and its BOM
come from Maven Central and the portal is only the fallback. Set `skip-lts-prepass: true` on the
`dotnet-test` `setup-dotnet` step and rewrote the matrix comment to say that the private install dir
plus the skipped pre-pass leave a leg with only the runtimes it lists.

**Files changed:**

- `packages/kotlin/settings.gradle.kts`: `pluginManagement` block, Maven Central before the portal
- `.github/workflows/ci.yml`: `dotnet-test` gets `skip-lts-prepass: true` under `with:`, plus the
    rewritten matrix comment (no date named)

**Verification:**

- `grep -n 'pluginManagement'` matches line 5; `mavenCentral()` (l.7) comes before
    `gradlePluginPortal()` (l.8)
- `cargo build -p iscc-uniffi && ./gradlew test --refresh-dependencies --rerun-tasks`: BUILD
    SUCCESSFUL, "5 actionable tasks: 5 executed" (`:test` ran, not UP-TO-DATE). JUnit XML:
    ConformanceTest 19 tests, DataEdgeTest 6 tests, 0 failures/errors/skipped
- `./gradlew buildEnvironment --refresh-dependencies --info` exit 0. The BOM line:
    `Cached resource https://repo.maven.apache.org/maven2/org/jetbrains/kotlin/kotlin-gradle-plugins-bom/2.4.10/kotlin-gradle-plugins-bom-2.4.10.pom is up-to-date`
    (Gradle checked the remote again because of `--refresh-dependencies`). The plugin marker POM
    also resolves from `repo.maven.apache.org`. `grep -c plugins.gradle.org` on the log gives **0**
- `grep -n 'skip-lts-prepass: true' .github/workflows/ci.yml` → line 183, inside `dotnet-test`
- `yaml.safe_load` on ci.yml OK; `mise run check` exit 0 (all hooks Passed)
- Confirmed the input exists: `gh api …/actions/setup-dotnet/contents/action.yml?ref=v6` lists
    `skip-lts-prepass` ("skips the LTS runtime pre-pass … Default: false")

**Next:** Review must confirm on the pushed run that the Kotlin job, both `dotnet-test` legs and the
`dotnet` aggregator pass, and that the 10.0 leg's setup-dotnet log shows no `--channel LTS` install.
The muxer claim (SDK 10 drives the 10.0 leg without the pre-pass) is **unverified locally**. Only
`Runtime_MatchesExpectedMajorVersion` on that leg in CI can settle it. After that: the
`alg_cdc_chunks` overflow (Rust + Go + vector).

**Notes:**

- The Gradle run creates `packages/kotlin/.kotlin/` (Kotlin 2.x session dir), which is not
    gitignored. I deleted it and did not commit it. It is out of scope here, but `.gitignore` could
    cover it.
- I left `release.yml` alone as next.md said. Its Kotlin jobs build from the same directory, so they
    pick up the `pluginManagement` block automatically.
- No Rust changed, so the CRAP and iai gates are untouched.
