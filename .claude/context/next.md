# Next Work Package

## Step: NONE

## Goal

No CID-workable step remains. Every v0.8.0 criterion in target.md is met, CI is green on
`origin/develop` (`844f8c4`, run conclusion `success`), and no `critical` or `normal` issue is open.

## Alternatives Considered

- **Chosen:** NONE. Every remaining gap is human-gated, and every open issue is `low`.
- **Rejected:** picking up a `low` `[review]` doc issue, for example the `KAA` Sum-Code comments or
    the pkg-config `Version: 0.2.0`. `low` issues are reserved for human-directed work, and the 13
    untriaged ones wait on Titusz to triage them.

## Reason

Each remaining item is blocked on the human:

- **Cut v0.8.0:** a human release through the `/release` skill. CID never cuts a release. The
    release notes should mention that the `@iscc/wasm` `alg_cdc_chunks` return type is now
    `Uint8Array[]`.
- **v1.0.0 criteria** (crate `>= 1.0.0`, enforcing `cargo-semver-checks`, MSRV verification): the
    `low` `[human]` issues "Release core as v1.0.0" and "The declared MSRV is asserted but never
    verified". Titusz has set no milestone after v0.8.0.
- **Upstream Unicode drift:** `low` `[human]`, parked on iscc/iscc-core#137.
- **EOL runtime floors** (Python 3.10, .NET 8, Ruby 3.1): `low`. Only the human raises a floor.
- **Published Ruby source gems up to 0.7.0:** yanking them is a registry action for Titusz.
- **`target.md` sync-target count (12 vs 21):** only the human edits `target.md`.
- **13 untriaged `low` review findings:** each one waits for Titusz to promote it to `normal` or
    park it.

## Not In Scope

- Any `low` issue, until the human promotes it to `normal`
- Version bumps, tags or release-workflow dispatches

## Verification

- `grep -E '^## .*(critical|normal)' .claude/context/issues.md` prints nothing, which means no open
    issue is `critical` or `normal`

## Done When

The advance agent no-ops on this NONE step.
