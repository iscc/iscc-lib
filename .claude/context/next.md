# Next Work Package

## Step: Create version sync tooling

## Goal

Create `scripts/version_sync.py` and add `mise run version:sync` / `mise run version:check` tasks so
non-Cargo manifests (`package.json`, `pom.xml`) stay in sync with the workspace version from root
`Cargo.toml`. This is the last unchecked item in the Version Sync verification section of
`specs/ci-cd.md` and the only remaining `[normal]` issue.

## Scope

- **Create**: `scripts/version_sync.py`
- **Modify**: `mise.toml` (add `version:sync` and `version:check` tasks)
- **Reference**:
    - `.claude/context/specs/ci-cd.md` — sync tooling spec, release protocol, manifest table
    - `.claude/context/issues.md` — `[normal] Create version sync tooling` issue description
    - `Cargo.toml` — workspace version source of truth (`[workspace.package] version = "0.0.1"`)
    - `crates/iscc-napi/package.json` — `"version": "0.0.1"` field to update
    - `crates/iscc-jni/java/pom.xml` — `<version>0.0.1-SNAPSHOT</version>` element to update
    - `scripts/gen_llms_full.py` — existing script for style/pattern reference

## Not In Scope

- Adding `version:check` to CI workflow — that's a future CI integration step
- Adding a pre-commit hook for version checking
- Updating the WASM `pkg/package.json` — that's generated at build time by the release workflow
    script, not a checked-in manifest
- Removing `-SNAPSHOT` suffix from pom.xml (the sync script writes the bare version; the `-SNAPSHOT`
    convention is a Maven development practice that can be addressed when Maven Central publishing
    is set up)
- Bumping the actual version number — all manifests are already at `0.0.1`

## Implementation Notes

**Script design** (`scripts/version_sync.py`):

- Pure Python, stdlib only (no third-party dependencies). Cross-platform (Windows/Linux/macOS)
- Use `pathlib.Path` for all file paths
- Start with a module docstring explaining the script's purpose
- Accept `--check` flag for validation mode (exit 0 if in sync, exit 1 with diff details if not)
- Default mode (no flag) performs the sync — reads version and updates files in place

**Reading workspace version from `Cargo.toml`**:

- Use regex: `r'^version\s*=\s*"(.+?)"'` with `re.MULTILINE` on root `Cargo.toml`
- Match the first occurrence (which is the `[workspace.package]` version line)
- This is the same pattern used in the WASM release CI script (per learnings)

**Updating `package.json`** (`crates/iscc-napi/package.json`):

- Use `json.load` / `json.dump` (stdlib) — read, update `"version"` key, write back
- Preserve 2-space indentation: `json.dump(data, f, indent=2)` + ensure trailing newline

**Updating `pom.xml`** (`crates/iscc-jni/java/pom.xml`):

- Use regex replacement on the raw XML string — do NOT use `xml.etree` (it rewrites the entire file,
    changes attribute order, loses comments)
- Target pattern:
    `r'(<groupId>io\.iscc</groupId>\s*<artifactId>iscc-lib</artifactId>\s*<version>).+?(</version>)'`
    with `re.DOTALL`
- Replace the version content with the workspace version (bare, no `-SNAPSHOT` suffix)
- This targets only the project's own `<version>`, not dependency versions

**mise tasks** (add to `mise.toml`):

```toml
[tasks."version:sync"]
description = "Sync non-Cargo manifest versions with workspace version"
run = "uv run scripts/version_sync.py"

[tasks."version:check"]
description = "Check that all manifest versions match workspace version"
run = "uv run scripts/version_sync.py --check"
```

**Output**: The script should print what it's doing — which file was updated to which version (sync
mode) or which file is out of sync (check mode). Keep output concise (1 line per manifest).

## Verification

- `uv run scripts/version_sync.py --check` exits 0 (all manifests already at 0.0.1)
- `uv run scripts/version_sync.py` runs without error and reports sync complete
- `mise run version:sync` executes the sync script successfully
- `mise run version:check` executes the check script and exits 0
- `grep -q 'version:sync' mise.toml` exits 0 (task registered)
- `grep -q 'version:check' mise.toml` exits 0 (task registered)
- `grep -q 'import re' scripts/version_sync.py` exits 0 (uses regex, no third-party deps)
- `mise run check` passes (all pre-commit hooks clean)

## Done When

All 8 verification criteria pass — the version sync script correctly reads the workspace version,
validates that `package.json` and `pom.xml` match, and both mise tasks are functional.
