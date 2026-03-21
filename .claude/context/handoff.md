## 2026-03-21 — Kotlin documentation — howto guide, package README/CLAUDE.md, root README integration

**Done:** Created all Kotlin documentation artifacts: howto guide (`docs/howto/kotlin.md`, 452
lines), package README (`packages/kotlin/README.md`, 88 lines), and package CLAUDE.md
(`packages/kotlin/CLAUDE.md`, 101 lines). Integrated Kotlin into root README (installation +
quickstart sections, polyglot language lists), docs nav (`zensical.toml`), and LLM page list
(`scripts/gen_llms_full.py`).

**Files changed:**

- `docs/howto/kotlin.md`: New howto guide with 13 sections (installation, all code gen functions,
    streaming, codec, text utils, encoding utils, algorithm primitives, constants, conformance,
    error handling). Follows Swift howto structure adapted for Kotlin syntax.
- `packages/kotlin/README.md`: New package README with install, usage example, API overview, build
    from source instructions.
- `packages/kotlin/CLAUDE.md`: New package CLAUDE.md with file layout, build commands, test
    patterns, binding generation, CI, and pitfalls.
- `README.md`: Added Kotlin installation section (Gradle DSL dependency block) and quickstart
    section (genMetaCodeV0 example) between Swift and WASM. Added "Kotlin" to polyglot feature list
    and description paragraph (needed for >= 4 mentions).
- `zensical.toml`: Added `{ "Kotlin" = "howto/kotlin.md" }` after Swift in nav.
- `scripts/gen_llms_full.py`: Added `"howto/kotlin.md"` after `"howto/swift.md"` in ORDERED_PAGES.

**Verification:** All criteria pass:

- [x] `docs/howto/kotlin.md` exists
- [x] `packages/kotlin/README.md` exists
- [x] `packages/kotlin/CLAUDE.md` exists
- [x] `grep -q 'kotlin' zensical.toml` — nav entry present
- [x] `grep -q 'kotlin' scripts/gen_llms_full.py` — ORDERED_PAGES entry present
- [x] `grep -q 'Kotlin' README.md` — root README has Kotlin sections
- [x] `grep -c 'Kotlin' README.md` returns 4 (>= 4 required)
- [x] `uv run python scripts/gen_llms_full.py && test -f site/llms-full.txt` — generates OK (note:
    output path is `site/llms-full.txt`, not `docs/llms-full.txt` as stated in next.md)
- [x] `mise run check` — all 15 pre-commit hooks pass

**Next:** Release workflow integration — add `maven-kotlin` input to `release.yml` for Kotlin
package publishing to Maven Central. This is the remaining sub-task from the Kotlin bindings issue.

**Notes:**

- Next.md stated "Adding Kotlin to the polyglot badge line (already listed in README line 26)" as
    not in scope, but Kotlin was NOT already listed there. Added it to meet the >= 4 mentions
    verification criterion.
- The streaming API uses `update()` method (not `push()` as next.md mentioned) — verified against
    the generated `iscc_uniffi.kt` source.
- The `textTrim` function takes `nbytes: ULong` (not `UInt`) in Kotlin — documented correctly.
- Next.md verification says `docs/llms-full.txt` but the script outputs to `site/llms-full.txt` —
    minor error in the verification spec.
