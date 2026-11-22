# Next Work Package

## Step: NONE

## Reason

Gap analysis of target.md against state.md (assessed at `0e0f5d14`) finds no unmet criterion the CID
loop may act on:

- **v0.8.0 release criteria — all met.** ISCC-SEQ (`encode_seq` / `decode_seq`, Tier 1 = 35) runs on
    the Rust core and all 11 surfaces; `data-edge.json` (11 sections) runs in all 12 harnesses with
    deviations only as `except` entries; `version:check` runs in CI; CI is green on the origin tip
    `32cd3f91` (run 45036383921, 24 check runs); no `critical` or `normal` issue is open
    (`grep -nE '^## .*(critical|normal)' .claude/context/issues.md` prints nothing).
- **Cutting the v0.8.0 release** — human-only (`/release` skill); target.md says CID never cuts it.
- **v1.0.0 criteria** (crate `>= 1.0.0`, strict SemVer gate enforcing, MSRV prerequisite) —
    human-gated and explicitly out of scope for v0.8.0 (target.md, postponed per state.md).
- **18 open issues, all `low`** (16 untriaged) — reserved for human-directed work; the loop must
    skip them. The human promotes any of them to `normal` (or parks it in `postponed.md`).
- **.NET 8 floor** (end of support 2026-11-10) — raising a supported-version floor is the human's
    call (target.md CI/CD criterion, decision of 2026-11-21).

## Alternatives Considered

- **Chosen:** NONE — every reachable criterion is met; the remaining work is a human release cut,
    human-gated v1.0.0 items, or `low` issues.
- **Rejected:** the `low` `gen_mixed_code_v0` `bits < 8` panic (a real Go panic, closable with one
    `data-edge.json` row) — `low` entries are human-directed only; it needs promotion to `normal`
    first.

## Not In Scope

- Any `low` issue in issues.md, however small
- Cutting or preparing the v0.8.0 release (tags, `/release`, version bumps)
- Any v1.0.0 work (version `1.0.0`, MSRV, enforcing the semver gate)

## Verification

- `grep -nE '^## .*(critical|normal)' .claude/context/issues.md` prints nothing
- `git status --porcelain` shows no change made by advance (advance no-ops on NONE)

## Done When

The advance agent no-ops and the review agent signals IDLE, since every remaining blocker is `low`
or human-gated.
