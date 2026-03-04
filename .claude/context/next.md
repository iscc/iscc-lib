# Next Work Package

## Step: CID IDLE — No automatable work remains

## Goal

Signal that the CID loop has no actionable work. All remaining gaps between state.md and target.md
are blocked on `low`-priority issues (which the CID loop skips by policy) or require human
decisions.

## Status: IDLE

The CID loop should stop iterating until new `normal`+ priority work is filed in issues.md or
target.md is updated.

## Remaining gaps (all require human action)

1. **C# / .NET bindings** — `low` priority issue, not started
2. **C++ header-only wrapper** — `low` priority issue, not started
3. **Swift bindings via UniFFI** — `low` priority issue, not started
4. **Kotlin Multiplatform bindings** — `low` priority issue, not started
5. **`alg_cdc_chunks` avg_chunk_size=0 hang** — `low` priority, awaiting human review of upstream
6. **Language logos in README/docs** — `low` priority, cosmetic
7. **Merge develop → main and cut release** — human decision required
8. **RubyGems account setup** — manual human action

## How to resume

- To start a low-priority binding: elevate its priority to `normal` in issues.md, then re-run
    `mise run cid:step`
- To trigger a release: run `mise run pr:main`, review and merge, then trigger `release.yml`
- To add new work: file a `normal`+ priority issue in issues.md or update target.md

## Scope

- **Create**: (none)
- **Modify**: (none)
- **Reference**: (none)

## Not In Scope

- Picking up any `low` priority issue (CID policy prohibits this)
- Starting bindings for languages without human approval to elevate priority
- Making release decisions autonomously

## Implementation Notes

No implementation needed. This is an idle signal.

## Verification

- All issues in issues.md are `low` priority (no `critical` or `normal` issues exist)
- State.md confirms all non-low-priority work is complete

## Done When

The advance agent reads this file and confirms there is nothing to implement. The CID loop stops.
