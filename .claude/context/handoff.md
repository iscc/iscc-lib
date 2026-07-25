# Handoff

## 2026-07-25 — Document the Kotlin consumer floor (2.3+) in README, package README, and howto guide

**Done:** Documented the decided Kotlin consumer floor (**Kotlin 2.3 or newer**, verbatim in all
three files) in the three consumer-facing places the spec names: a `## Requirements` section in
`packages/kotlin/README.md` (mirroring the Swift README pattern), a
`!!! note "Requires Kotlin 2.3 or newer"` admonition in `docs/howto/kotlin.md`'s Installation
section, and a two-line floor note in the root README's Kotlin Installation section. Each carries
the causal explanation from next.md (compiled with `kotlin("jvm") 2.4.10`; Kotlin accepts ~one minor
of forward metadata; older compilers fail with
`Module was compiled with an incompatible version of Kotlin`).

**Files changed:**

- `README.md`: Kotlin Installation section (~line 155) — prepended "Requires **Kotlin 2.3 or newer**
    (the artifact is compiled with Kotlin 2.4.10; older compilers reject its metadata)." to the
    existing native-library paragraph. Quick Start section untouched.
- `packages/kotlin/README.md`: added `## Requirements` section between `## Installation` and
    `## Usage` with the floor bullet (Swift README house style).
- `docs/howto/kotlin.md`: added the floor admonition in `## Installation`, after the JNA sentence
    and before the existing "Not yet published to Maven Central" note.

**Verification:** All next.md criteria reproduced in this session:

- `grep -c 'Kotlin 2.3 or newer'` → 1 for each of `README.md`, `packages/kotlin/README.md`,
    `docs/howto/kotlin.md`
- `grep -c '## Requirements' packages/kotlin/README.md` → 1
- `grep -c 'kotlin("jvm") version "2.4.10"' packages/kotlin/build.gradle.kts` → 1 (build file not
    edited)
- Version-sync anchors intact: `io.iscc:iscc-lib-kotlin:0.5.0` → 1 in each of the three files;
    `net.java.dev.jna:jna:5.19.1` → 1 in each of `packages/kotlin/README.md`, `docs/howto/kotlin.md`
- `mise run version:check` exit 0 with 21 `OK:` lines
- `uv run zensical build` exit 0, "No issues found" (10.6s)
- `mise run format` applied (mdformat re-wrapped the howto admonition body once), then
    `mise run check` — all 15 hooks Passed, nothing rewritten
- `git diff --stat` — exactly `README.md`, `docs/howto/kotlin.md`, `packages/kotlin/README.md` plus
    runner-owned `.claude/context/iterations.jsonl`; no source, build, or manifest file touched

No tests were owed: docs-only change, no code paths touched, so no cargo/pytest run was required
beyond what `mise run check` executes (its hook set for this diff is format/lint hygiene).

**Next:** Per the previous review's plan, ruff 0.16 slice B (the isort cluster: `I001` 8 + `RUF022`
1 — decide `[tool.ruff.lint.isort]` config, then `--fix`), followed by slice C (promote `S`/`C901`
into `[tool.ruff.lint] select`, clearing the 15 `RUF100`s). The other open decided item is the
Unicode 16.0.0 freeze-rule documentation/gating step.

**Notes:**

- The issue "Document the Kotlin consumer floor as 2.3+ (DECIDED)" in issues.md is now addressed —
    left in place for the review agent to resolve, per next.md.
- The spec (`kotlin-bindings.md`) and `build.gradle.kts` were read-only references and are
    untouched, as required.
- Wording nuance: the root README says "Kotlin 2.4.10" (prose, cheat-sheet brevity) while the two
    fuller docs quote `kotlin("jvm") 2.4.10` — all three contain the grep-checkable verbatim phrase
    "Kotlin 2.3 or newer".
- Agent memory (`deps-refresh.md`): recorded the spec's maintenance rule that any future Kotlin
    compiler bump moving the floor must update all three docs + spec in the same step.
