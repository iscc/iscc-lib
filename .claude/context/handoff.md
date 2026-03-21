## 2026-03-21 — Review of: Kotlin documentation — howto guide, package README/CLAUDE.md, root README integration

**Verdict:** PASS

**Summary:** All Kotlin documentation artifacts created correctly — 451-line howto guide, 89-line
package README, 101-line package CLAUDE.md. Root README integration (install + quickstart sections),
zensical.toml nav entry, and gen_llms_full.py entry all verified. API names, types, and signatures
in the howto match the generated `iscc_uniffi.kt` bindings. Minor fix applied: added
`jna.library.path` to README library path notes (found by Codex review).

**Verification:**

- [x] `test -f docs/howto/kotlin.md` — howto guide exists (451 lines)
- [x] `test -f packages/kotlin/README.md` — package README exists (89 lines)
- [x] `test -f packages/kotlin/CLAUDE.md` — package CLAUDE.md exists (101 lines)
- [x] `grep -q 'kotlin' zensical.toml` — nav entry present
- [x] `grep -q 'kotlin' scripts/gen_llms_full.py` — ORDERED_PAGES entry present
- [x] `grep -q 'Kotlin' README.md` — root README has Kotlin sections
- [x] `grep -c 'Kotlin' README.md` returns 4 (>= 4 required)
- [x] `uv run python scripts/gen_llms_full.py && test -f site/llms-full.txt` — generates OK (output
    path is `site/llms-full.txt`, not `docs/llms-full.txt` as next.md stated)
- [x] `mise run check` — all 15 pre-commit hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean

**Issues found:**

- (none) — all scope items completed correctly

**Codex review:** Two findings, both advisory:

1. **P1** (Maven Central snippet before publishing): README shows aspirational install snippet for
    an unpublished package. Assessed as consistent with project pattern (Swift has the same). The
    detailed howto has a "Not yet published" admonition. No action needed.
2. **P2** (Missing `jna.library.path` in READMEs): Valid — both READMEs mentioned only
    `java.library.path` but JNA needs `jna.library.path` too. Fixed in this review commit.

**Next:** Add `maven-kotlin` input to `release.yml` for Kotlin package publishing to Maven Central.
This is the last remaining sub-task of the Kotlin bindings issue. After that, the Kotlin issue can
be fully resolved (docs + CI + tests + version sync + release workflow all complete).

**Notes:**

- next.md's "Not In Scope" stated Kotlin was "already listed in README line 26" — this was incorrect
    (Kotlin had 0 mentions). Advance agent correctly added it to the polyglot feature list to meet
    the >= 4 mentions criterion.
- next.md verification said `docs/llms-full.txt` but script outputs to `site/llms-full.txt`.
- Streaming API uses `update()` (not `push()` as next.md mentioned) — advance agent used the correct
    name after checking the generated bindings.
