# Next Work Package

## Step: Update ci-cd.md spec to match actual CI state

## Goal

Fix three stale entries in the CI/CD spec so it accurately reflects the current CI workflow (9 jobs,
pure Go, version-check and bench jobs). This is the last automatable gap before the project enters
human-dependent maintenance mode.

## Scope

- **Create**: (none)
- **Modify**: `.claude/context/specs/ci-cd.md`
- **Reference**: `.github/workflows/ci.yml` (actual CI workflow, source of truth)

## Not In Scope

- Modifying the actual CI workflow (ci.yml) — it is correct; only the spec is stale
- Adding Maven Central or Go release jobs to the spec — those are future human tasks
- Updating the verification checklist checkmarks — only fix the text descriptions
- Any code changes — this is a documentation-only step
- Resolving the tab order issue (needs human decision)

## Implementation Notes

Three targeted edits in the spec file:

**Edit 1 — CI jobs table, Go row (line 26):** Change the Go row from referencing "cargo build
--target wasm32-wasip1 for FFI binary" to "CGO_ENABLED=0 go test, go vet" (pure Go, no Rust
toolchain).

**Edit 2 — CI jobs table, add missing rows:** Add two new rows after Go for the Version and Bench
jobs:

- Version job: runs python version_sync.py --check for manifest consistency
- Bench job: runs cargo bench --no-run for compile-only benchmark verification

**Edit 3 — Verification checklist, Go entry (line 260):** Change "Go job builds WASM FFI binary,
runs go test and go vet" to reflect pure Go (CGO_ENABLED=0 go test and go vet, no Rust toolchain).
Also add two new verification entries for the version-check and bench jobs.

## Verification

- `grep -q 'CGO_ENABLED=0' .claude/context/specs/ci-cd.md` exits 0 (Go row updated)
- `grep -q 'Version' .claude/context/specs/ci-cd.md` finds the version-check job row
- `grep -q 'Bench' .claude/context/specs/ci-cd.md` finds the bench job row
- `grep -c 'wasm32-wasip1' .claude/context/specs/ci-cd.md` outputs 0 (no stale WASM ref)

## Done When

All four verification commands pass, confirming the spec accurately reflects the 9-job CI workflow
with pure Go and version-check/bench jobs.
