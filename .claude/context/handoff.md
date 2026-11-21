## 2026-11-21 — Review of: Kotlin plugins resolve from Maven Central; the .NET 10 leg skips setup-dotnet's LTS pre-pass

**Verdict:** PASS

**Summary:** CI is green again. `packages/kotlin/settings.gradle.kts` resolves Gradle plugins from
Maven Central first, with the plugin portal as the fallback, and the `dotnet-test` job sets
`skip-lts-prepass: true`, so each .NET leg holds only the SDKs it lists. The diff touches the two
files next.md named and nothing else. Both changes are confirmed on the pushed run, including the
one claim that could not be checked locally.

**Verification:**

- [x] `grep -n 'pluginManagement' packages/kotlin/settings.gradle.kts` matches, `mavenCentral()`
    before `gradlePluginPortal()` — lines 5, 7, 8
- [x] `cargo build -p iscc-uniffi && ./gradlew test --refresh-dependencies` exits 0, test task
    forced — run with `--rerun-tasks`: BUILD SUCCESSFUL, "5 actionable tasks: 5 executed"
- [x] `./gradlew buildEnvironment --refresh-dependencies --info` shows the BOM from Maven Central —
    `Cached resource https://repo.maven.apache.org/maven2/org/jetbrains/kotlin/kotlin-gradle-plugins-bom/2.4.10/kotlin-gradle-plugins-bom-2.4.10.pom is up-to-date`;
    0 lines mention `plugins.gradle.org`
- [x] `grep -n 'skip-lts-prepass: true' .github/workflows/ci.yml` matches inside `dotnet-test` —
    line 183; `yaml.safe_load` reads it as `True` under the setup-dotnet step's `with:`
- [x] `yaml.safe_load` on `ci.yml` exits 0 and `mise run check` exits 0 — all hooks Passed
- [x] Done When, on CI run 44909648953 (commit `28d4fb8`, whole run green): Kotlin job, both
    `dotnet-test` legs and the `dotnet` aggregator succeeded. Both setup-dotnet logs print "Skipping
    LTS runtime pre-pass (skip-lts-prepass: true)." and hold no `--channel LTS` call. The 8.0 leg
    installed SDK 8.0.428 only, the 10.0 leg SDK 8.0.428 and 10.0.404. Each leg: 154 passed, 0
    failed, so `Runtime_MatchesExpectedMajorVersion` passed on runtime 10. The Kotlin job log has 0
    lines mentioning `plugins.gradle.org`

**Issues found:**

- The issue "The .NET 10 CI leg depends on which .NET is the latest LTS" is resolved and deleted
- Minor, fixed in this commit: a Gradle run leaves an untracked `packages/kotlin/.kotlin/` session
    directory (advance noted it). Added `.kotlin/` to `packages/kotlin/.gitignore`
- No new issues filed

**Codex review:** No actionable findings. It reports 25 Kotlin tests and 154 .NET tests passing on
runtimes 8 and 10 in its own run.

**Next:** `alg_cdc_chunks` accepts an `avg_chunk_size` whose parameter maths overflows (`normal`
[audit]): Rust + Go validation or u64 maths, plus a `data-edge.json` vector at the boundary. It is a
core change on a benchmarked path, so the step needs `mise run bench:iai:check`, the CRAP baseline
and the full feature matrix. A new vector in an existing `data-edge.json` section reaches every
harness; a new section is a 12-surface fan-out.

**Notes:**

- With the pre-pass skipped, the `dotnet` host on the 10.0 leg comes from SDK 8 (the first one
    installed, later installs keep non-versioned files). It still runs SDK 10 and runtime 10, as the
    green leg shows. If a later .NET major needs a newer host, list the newest SDK first
- `release.yml` was not touched and needs nothing: its Kotlin jobs build from `packages/kotlin` and
    pick up the `pluginManagement` block; its `setup-dotnet` steps install 8.0 only, with no
    roll-forward
- The advance commits were pushed before this review commit, because only a CI run could prove the
    .NET claim
