## 2026-02-28 — Update ci-cd.md spec to match actual CI state

**Done:** Updated `.claude/context/specs/ci-cd.md` with three targeted edits: (1) replaced stale
WASM-based Go job description with pure Go (CGO_ENABLED=0), (2) added Version and Bench rows to the
CI jobs table, (3) updated Go verification entry and added Version/Bench verification entries.

**Files changed:**

- `.claude/context/specs/ci-cd.md`: Go row updated from WASM FFI to pure Go; added Version and Bench
    job rows to CI table; added three verification checklist entries (Go updated, Version added,
    Bench added); removed all `wasm32-wasip1` references

**Verification:** All four verification criteria pass:

- `grep -q 'CGO_ENABLED=0'` — PASS (Go row updated)
- `grep -q 'Version'` — PASS (version-check job row present)
- `grep -q 'Bench'` — PASS (bench job row present)
- `grep -c 'wasm32-wasip1'` outputs 0 — PASS (no stale WASM references)
- `mise run check` — all 14 hooks pass (mdformat auto-fixed table alignment on first run)

**Next:** The project is in maintenance mode with remaining work being human-dependent tasks
(publishing config, PR #10 merge, tab order decision). The CID loop provides diminishing returns at
this point. Consider closing the loop or setting the target to a human-gated milestone.

**Notes:** This was a documentation-only change — no code modified. The mdformat hook adjusted table
column alignment on first run (expected behavior for markdown table reformatting). The CI jobs table
now lists all 9 jobs matching the actual `ci.yml` workflow. The standard action set description on
line 30 still doesn't list `actions/setup-go@v5` or `actions/setup-java@v4`, but updating it was not
in scope for this step.
