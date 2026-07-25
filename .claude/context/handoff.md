# Handoff

## 2026-07-25 — Review of: Dependency refresh slice 5 — JVM binding manifests (pom.xml + build.gradle.kts)

**Verdict:** PASS_WITH_NOTES

**Summary:** All 11 in-scope pins were bumped exactly as specified, both hold-backs carry `held:`
comments, docs are synced, and every gate is green (mvn 69/69, gradle 9/9, `mise run check` 15/15,
clippy clean, `version:check` OK). One consequence next.md did not anticipate: the `kotlin("jvm")`
2.1.10 → 2.4.10 bump stamps Kotlin metadata `mv=[2,4,0]` into the published jar, which **verifiably
breaks downstream consumers compiling with Kotlin < 2.3**. Nothing ships from `develop`, so this is
filed as a `normal` `[review]` issue to settle before the next Maven Central publish rather than a
blocker on an otherwise clean slice.

**Verification:**

- [x] `cargo build -p iscc-jni && mvn test -f crates/iscc-jni/java/pom.xml` — exit 0, **Tests run:
    69, Failures: 0, Errors: 0, Skipped: 0**, BUILD SUCCESS
- [x] `mvn -Prelease package -DskipTests` — exit 0; source-plugin 3.4.0 and javadoc-plugin 3.12.0
    both executed, `iscc-lib-0.5.0-sources.jar` + `-javadoc.jar` present. Caveat: `maven-gpg-plugin`
    is **not** resolved by this command (verify-phase binding; absent from `~/.m2`), so 3.2.8 was
    confirmed the other way — `repo1.maven.org` `.pom` HTTP 200 (all 9 target versions checked)
- [x] `cargo build -p iscc-uniffi` then `./gradlew test` from `packages/kotlin` — exit 0,
    `tests="9" failures="0" errors="0"`. First two attempts died on bind-mount incremental-state
    races (`Unable to delete file …/build/kotlin/…`, `NoSuchFileException …/build/reports/…`); green
    after `./gradlew clean` — infrastructure flake, not the change
- [x] pom bumped-pin grep → **7**
- [x] `<version>0.7.0</version>` → **1**, `held:` in pom.xml → **1** (comment is valid XML; mvn
    parsed it)
- [x] gradle pin grep prints the 3 expected lines; `held:` in build.gradle.kts → **1**
- [x] `jna:5.16.0` in the 3 listed docs → exit 1 (none); `jna:5.19.1` → **1** each
- [x] `5.11.4|2.11.0` in `crates/iscc-jni/CLAUDE.md` → exit 1 (none)
- [x] `git status --porcelain packages/kotlin/src packages/kotlin/gradle` → empty (generated
    bindings + wrapper untouched)
- [x] `mise run version:check` → exit 0
- [x] `mise run check` → exit 0, all 15 hooks pass, **nothing rewritten** (define-next formatted
    next.md correctly this cycle — no reformat needed in the review commit)
- [x] Extra: `cargo clippy --workspace --all-targets -- -D warnings` clean; local commands are
    byte-identical to the `java:` and `kotlin:` CI jobs, so CI should reproduce

**Issues found:**

- **Consumer Kotlin floor silently raised from ~2.0/2.1 to 2.3** (filed: "Kotlin binding silently
    raised the consumer Kotlin floor to 2.3" `normal` `[review]`, carries `**Spec:**` + HUMAN REVIEW
    REQUESTED). The published jar's metadata is now `mv=[2,4,0]` and the published POM declares
    `kotlin-stdlib:2.4.10` (compile). Verified against throwaway consumer projects resolving the
    artifact from `mavenLocal`: Kotlin **2.1.10 FAILS**, **2.2.21 FAILS**
    (`Module was compiled with   an incompatible version of Kotlin … metadata is 2.4.0`), **2.3.21
    PASSES**. Pinning `languageVersion` alone would not fix it — the transitive stdlib raises the
    same error. Rationale for approving anyway is in `decisions.md` 2026-07-25.
- **Approved deviation:** the advance agent added
    `testRuntimeOnly("org.junit.platform:junit-platform-launcher:1.14.4")` — a dependency line
    next.md had only anticipated for JUnit 6. It is genuinely required (Gradle 8.12.1 injects a
    launcher predating platform 1.12 → "OutputDirectoryCreator not available"), is documented in
    place, and does **not** leak into the published artifact — verified via
    `./gradlew generatePomFileForMavenPublication`: the published POM lists only `kotlin-stdlib` and
    `jna`. Not a scope violation.
- No gate circumvention anywhere in the unpushed range (`@{upstream}..HEAD` scanned; no
    suppressions, skips, threshold changes, or hook weakening). Scope: 2 non-test/non-doc files,
    within the 3-file cap.
- Minor doc drift, deliberately untouched: `.claude/context/specs/kotlin-bindings.md` still shows
    `jna:5.16.0@aar` — next.md correctly declared that human-owned file out of scope. Worth folding
    into whichever step settles the Kotlin support policy.

**Codex review:** Codex independently reached the *same* P1 finding as this review — that the KGP
2.4.10 bump raises the published metadata version and breaks Kotlin 2.1.x consumers, "despite no
documented support-floor change", recommending either keeping the publishing compiler on a
compatible line or documenting it as an intentional break. Both opinions converging made this worth
verifying empirically rather than reasoning about, which is how the precise floor (fails < 2.3,
passes ≥ 2.3) was established. No other findings; Codex raised nothing about the Maven side.

**Next:** Settle the Kotlin consumer floor — it is the only loose end from this slice and is
release-blocking for `io.iscc:iscc-lib-kotlin`. The recommended shape is a small docs-only step:
state "requires Kotlin 2.3+" in `packages/kotlin/README.md`, `docs/howto/kotlin.md`, and the root
README Kotlin section, refresh the stale `jna:5.16.0@aar` line in `specs/kotlin-bindings.md`, and
record the policy — but the *choice* between documenting the raised floor and holding the plugin at
2.1.x needs Titusz (the issue carries HUMAN REVIEW REQUESTED). If the loop should keep moving
without that decision, dependency-refresh slice 6 (rb `Gemfile`/gemspec, go `go.mod`, napi
`package.json`, or dotnet `.csproj` — one ecosystem per step) is the obvious continuation, and each
of those is CI-exercised on develop pushes exactly like this slice.

**Notes:**

- **Generalized lesson now in learnings.md:** a toolchain/compiler bump inside a *published* binding
    is a support-policy change, not a version pin. The cheap detectors are
    `javap -v -p <class> | grep mv=` on the built jar and a throwaway consumer project — both used
    here. Expect the same class of question for the deferred Gradle wrapper 8.12.1 major and JUnit
    6.x, and for the rb/napi/dotnet slices where a language-runtime floor may move.
- `mvn -Prelease package` does not exercise `maven-gpg-plugin` or `central-publishing-maven-plugin`
    at all; treat "release-profile plugins resolve" claims for those two as unverified by local
    builds. The `central-publishing` hold-back at 0.7.0 is well-reasoned for exactly this reason.
- Gradle on this workspace's bind mount flakes on incremental state — two consecutive failures here
    were both post-test bookkeeping (the test XML showed 9/0/0 the whole time). Always re-run after
    `./gradlew clean` before reporting a Kotlin build failure.
- learnings.md was at 212 lines after this review's additions; the per-algorithm internals bullets
    (all 10 `gen_*_v0` functions are conformance-complete) were archived to `learnings-archive.md`
    to bring it back under budget.
- Only one CID loop is running (checked `ps`); the working tree is clean apart from the runner-owned
    `iterations.jsonl`.
