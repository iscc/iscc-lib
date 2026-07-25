# Next Work Package

## Step: Document the Kotlin consumer floor (2.3+) in README, package README, and howto guide

## Goal

Document the decided Kotlin consumer floor (**Kotlin 2.3 or newer**) in the three consumer-facing
places the spec names, closing the `normal` `[human]` issue "Document the Kotlin consumer floor as
2.3+ (DECIDED)". Iteration 128 raised the published artifact's metadata version to `mv=[2,4,0]`, so
consumers on Kotlin 2.1/2.2 now fail to compile against `io.iscc:iscc-lib-kotlin` with no warning
anywhere in the docs — this is release-blocking for the next Maven Central publish.

## Scope

- **Create**: (none)
- **Modify**:
    - `packages/kotlin/README.md` — add a `## Requirements` section (mirror the existing
        `packages/swift/README.md` "Requirements" pattern, lines 47-51)
    - `docs/howto/kotlin.md` — add the floor statement to the `## Installation` section, next to the
        existing JNA runtime note
    - `README.md` — add a one-line floor note to the Kotlin **Installation** section (the one at ~line
        146 with the `implementation("io.iscc:iscc-lib-kotlin:0.5.0")` block), not the Quick Start
        section at ~line 262
- **Reference**:
    - `.claude/context/specs/kotlin-bindings.md` → "Supported consumer Kotlin version" (lines 142-161)
        — the authoritative wording and rationale; and the verification criterion at lines 279-281
    - `.claude/context/issues.md` → "Document the Kotlin consumer floor as 2.3+ (DECIDED)" — the
        empirical measurements (2.1.10 fails, 2.2.21 fails, 2.3.21 succeeds)
    - `packages/kotlin/build.gradle.kts` (read only) — confirms the declared compiler is
        `kotlin("jvm") version "2.4.10"`
    - `packages/swift/README.md` (read only) — the house style for a `## Requirements` section

## Not In Scope

- **Do not touch `packages/kotlin/build.gradle.kts`.** 2.4.10 is the decided compiler; holding an
    older floor was explicitly rejected (it would also require pinning the transitive
    `kotlin-stdlib`).
- Do not edit `.claude/context/specs/kotlin-bindings.md` — the policy section, the refreshed
    `jna:5.19.1` lines and the verification criterion were already written in the interactive
    session. Do not tick spec checkboxes either (this spec tracks none as `[x]`).
- Do not delete or rewrite the issue in `.claude/context/issues.md` — the review agent resolves
    issues after verifying the fix.
- Do not invent a JDK, Gradle, or Android API-level floor. `build.gradle.kts` declares no
    `jvmToolchain`/`jvmTarget`, so only the Kotlin 2.3 floor is evidence-backed and decided.
- Do not start the Unicode 16.0.0 freeze-rule work (issue "Declare and gate a Unicode data version")
    or the ruff 0.16 slices B/C/D — each is its own future step.
- Do not bump versions or run `mise run version:sync`; `0.5.0` strings in these files are managed by
    `scripts/version_sync.py` and must stay exactly as they are.

## Implementation Notes

- **Use the phrase `Kotlin 2.3 or newer` verbatim in all three files** so the statement is
    grep-checkable and consistent with the spec. Add a short causal explanation in each place, e.g.:

    > Requires **Kotlin 2.3 or newer**. The published artifact is compiled with
    > `kotlin("jvm") 2.4.10`, and Kotlin accepts roughly one minor version of forward metadata; older
    > compilers fail with `Module was compiled with an incompatible version of Kotlin`.

    Keep the root `README.md` version to one or two lines (it is an install cheat sheet), and give the
    fuller explanation in `packages/kotlin/README.md` and `docs/howto/kotlin.md`.

- `packages/kotlin/README.md`: follow `packages/swift/README.md`'s `## Requirements` bullet-list
    shape. Place the section right after `## Installation` (after the existing
    `java.library.path`/`jna.library.path` sentence) so a reader hits it before `## Usage`.

- `docs/howto/kotlin.md`: put the statement in `## Installation`, directly after the
    `build.gradle.kts` dependency block / JNA sentence and before the existing
    `!!! note "Not yet published to Maven Central"` admonition. An mkdocs admonition
    (`!!! note "Requires Kotlin 2.3 or newer"`) or plain bold prose are both fine — match the
    surrounding style and keep mdformat happy.

- Both `docs/howto/kotlin.md` and `packages/kotlin/README.md` are `scripts/version_sync.py` targets
    (they are matched by the `io\.iscc:iscc-lib(?:-kotlin)?:\d+\.\d+\.\d+` and JNA-dependency
    regexes). Do not alter those dependency lines; new prose lines are invisible to the regexes.

- Run `mise run format` before committing — the pre-push mdformat hook (`--wrap 100 --number`)
    rejects the whole push batch on non-conforming markdown.

## Verification

- `grep -c 'Kotlin 2.3 or newer' README.md packages/kotlin/README.md docs/howto/kotlin.md` reports
    at least `1` for each of the three files
- `grep -c '## Requirements' packages/kotlin/README.md` → `1`
- `grep -c 'kotlin("jvm") version "2.4.10"' packages/kotlin/build.gradle.kts` → `1` (the decided
    compiler is still declared; the build file was not edited)
- `grep -c 'io.iscc:iscc-lib-kotlin:0.5.0'` → `1` for each of `README.md`,
    `packages/kotlin/README.md`, `docs/howto/kotlin.md`, and `grep -c 'net.java.dev.jna:jna:5.19.1'`
    → `1` for each of `packages/kotlin/README.md`, `docs/howto/kotlin.md` (version-sync anchors
    intact)
- `mise run version:check` exits 0 (21 `OK:` lines)
- `uv run zensical build` exits 0 with "No issues found"
- `mise run check` — all hooks pass and nothing is rewritten (working tree afterwards holds only the
    three doc files plus runner-owned context files)
- `git diff --stat` shows changes confined to `README.md`, `packages/kotlin/README.md`,
    `docs/howto/kotlin.md` (plus `.claude/context/` files) — no source, build, or manifest file
    touched

## Done When

All three consumer-facing documents state the Kotlin 2.3+ floor in the spec's wording, the docs site
and version-sync checks pass, and no build or manifest file was modified.
