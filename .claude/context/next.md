# Next Work Package

## Step: Add version consistency check to CI

## Goal

Add the `version:check` script to the CI workflow so manifest version drift (between Cargo.toml,
package.json, and pom.xml) is caught automatically on every push. The ci-cd spec explicitly labels
this task as "(run in CI)" but it's currently missing from `ci.yml`.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/ci.yml` — add a version consistency check step
- **Reference**: `scripts/version_sync.py` (stdlib-only Python script, no `uv` needed),
    `.claude/context/specs/ci-cd.md` (version sync section)

## Not In Scope

- Adding a dedicated `mise run bench` task or benchmark CI job improvements
- Updating the stale Go job description in `specs/ci-cd.md` (spec maintenance is a separate concern)
- Cosmetic Go test cleanup (vestigial WASM comments, `TestPureGo*` naming)
- Tab order standardization (blocked on human decision)
- Publishing infrastructure (registry-side configuration)

## Implementation Notes

The `scripts/version_sync.py --check` script reads the workspace version from root `Cargo.toml` and
validates that `crates/iscc-napi/package.json` and `crates/iscc-jni/java/pom.xml` match. It uses
only Python stdlib modules (json, re, sys, pathlib) — no `uv` or pip dependencies needed.

**Recommended approach:** Add a lightweight standalone CI job named `version-check` (or similar)
that runs early and fast. It needs only `actions/checkout@v4` and `actions/setup-python@v5` — no
Rust toolchain, no uv, no caching. A single step: `python scripts/version_sync.py --check`.

Alternatively, it could be added as an early step in the existing Python job (which already has
Python), but a standalone job is cleaner since version consistency is a cross-cutting concern, not
Python-specific.

The job should run on the same triggers as other CI jobs (push to main/develop, PR to main) and is
expected to complete in under 10 seconds.

## Verification

- `python scripts/version_sync.py --check` exits 0 locally (versions are currently in sync)
- `grep -q 'version_sync' .github/workflows/ci.yml` exits 0 (script is referenced in CI)
- `mise run check` passes (all pre-commit/pre-push hooks pass)
- CI on develop passes after push (all 9 jobs green including the new version check)

## Done When

All verification criteria pass and the CI workflow includes a version consistency check job/step
that runs `scripts/version_sync.py --check`.
