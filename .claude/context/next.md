# Next Work Package

## Step: Fix Kotlin CI (gradlew permissions) and add Kotlin version sync

## Goal

Fix the failing Kotlin CI job by correcting the `gradlew` file permissions in git, and add
`packages/kotlin/gradle.properties` to the version sync script — restoring CI green (16/16) and
ensuring Kotlin version stays coordinated with the workspace.

## Scope

- **Create**: (none)
- **Modify**:
    - `packages/kotlin/gradlew` — git permission fix via `git update-index --chmod=+x` (not a content
        change, just git metadata)
    - `scripts/version_sync.py` — add `gradle.properties` get/sync functions and a new TARGETS entry;
        update the docstring to list the new target
- **Reference**:
    - `packages/kotlin/gradle.properties` — current format: `version=0.3.1` (key=value, no quotes)
    - `scripts/version_sync.py` — existing target patterns (especially `_get_ruby_version` /
        `_sync_ruby_version` for simple `key = "value"` pattern, and `_get_pyproject_version` for
        key=value without quotes)

## Not In Scope

- Kotlin howto guide (`docs/howto/kotlin.md`) — separate documentation step
- Kotlin README or CLAUDE.md — separate documentation step
- Root README Kotlin sections — separate documentation step
- Release workflow (`release.yml`) maven-kotlin input — separate step
- Fixing the two Swift packaging issues — unrelated normal-priority issues

## Implementation Notes

The `gradle.properties` format is `version=0.3.1` (no quotes, no spaces around `=`). The get/sync
functions should:

```python
def _get_gradle_properties_version(text):
    """Extract version from Gradle properties file."""
    m = re.search(r"^version=(\d+\.\d+\.\d+)", text, re.MULTILINE)
    return m.group(1) if m else ""


def _sync_gradle_properties(text, version):
    """Update version in Gradle properties file."""
    return re.sub(
        r"^(version=)\d+\.\d+\.\d+",
        rf"\g<1>{version}",
        text,
        count=1,
        flags=re.MULTILINE,
    )
```

Add to TARGETS list (after the Swift entry, before the .NET entry):

```python
(
    (
        "packages/kotlin/gradle.properties",
        _get_gradle_properties_version,
        _sync_gradle_properties,
    ),
)
```

For the gradlew fix: run `git update-index --chmod=+x packages/kotlin/gradlew` before committing.
This changes the git tree entry from `100644` to `100755` without modifying file content.

Update the module docstring (lines 7-18) to include
`- packages/kotlin/gradle.properties — Gradle project version` in the synced targets list.

## Verification

- `git ls-files -s packages/kotlin/gradlew` shows `100755` (not `100644`)
- `uv run scripts/version_sync.py --check` exits 0 (all targets including new Kotlin one match)
- `grep 'gradle.properties' scripts/version_sync.py` finds the TARGETS entry
- `uv run scripts/version_sync.py` includes `OK: packages/kotlin/gradle.properties = 0.3.1` in
    output
- `mise run lint` passes (formatting + clippy + ruff clean)

## Done When

All verification criteria pass — gradlew is executable in git, Kotlin gradle.properties is a version
sync target, and all quality gates are green.
