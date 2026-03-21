## 2026-03-21 — Fix Kotlin CI (gradlew permissions) and add Kotlin version sync

**Done:** Fixed `packages/kotlin/gradlew` git permissions from `100644` to `100755` via
`git update-index --chmod=+x`, and added `packages/kotlin/gradle.properties` as a version sync
target in `scripts/version_sync.py` (new `_get_gradle_properties_version` /
`_sync_gradle_properties` functions + TARGETS entry + docstring update).

**Files changed:**

- `packages/kotlin/gradlew`: git permission metadata changed from `100644` to `100755` (no content
    change)
- `scripts/version_sync.py`: added `_get_gradle_properties_version` and `_sync_gradle_properties`
    functions, added TARGETS entry for `packages/kotlin/gradle.properties` (after Swift, before
    .NET), updated module docstring to list the new target

**Verification:**

- `git ls-files -s packages/kotlin/gradlew` shows `100755` — PASS
- `uv run scripts/version_sync.py --check` exits 0 with
    `OK: packages/kotlin/gradle.properties = 0.3.1` — PASS
- `grep 'gradle.properties' scripts/version_sync.py` finds 6 matches (docstring + 2 functions + 3
    TARGETS lines) — PASS
- `mise run check` — all 15 hooks pass
- `mise run lint` — formatting, clippy, ruff all clean

**Next:** Continue Kotlin integration. Remaining sub-tasks: documentation (howto guide, README,
CLAUDE.md), release workflow (`maven-kotlin` input in `release.yml`).

**Notes:** No tests needed for this step — `version_sync.py` is a standalone script validated by
`--check` mode (which is itself tested in CI via the `version-check` job). The gradlew fix is
metadata-only — git will record the permission change in the next commit.
