# Next Work Package

## Step: NONE

## Goal

No CID-workable step exists. Every v0.8.0 release criterion is met, CI is green on `develop`
(`4d04bf9`), and every open issue is `low`, which the loop must not pick up.

## Alternatives Considered

- **Chosen:** NONE. The gap analysis of target.md against state.md found no unmet criterion inside
    v0.8.0 scope. The only open items are `low` issues and v1.0.0 work the human has to start.
- **Rejected:** realigning the release tag trigger in `ci-cd.md` (issue "`ci-cd.md` describes a tag
    trigger the release workflow does not have"). It is the only `low` entry the loop could finish
    on its own, but it is untriaged `low`. The 2026-11-19 decision keeps such entries off the loop
    until Titusz promotes them to `normal`.

## Reason

Probes run while scoping this step (2026-11-21):

- **CI:** the last 3 `develop` runs are `success`; HEAD is ahead only by `.claude` log and state
    commits.
- **Advisories:** osv-scanner over all tracked lockfiles finds only the 3 unmaintained crates that
    `deny.toml` ignores (expected). The 6 open Dependabot alerts are all for the default branch
    (`main`), and none applies to `develop`: `uv.lock` has setuptools 84.2.1 (fixed in 84.1.0) and
    no soupsieve at all, and `crates/iscc-rb/Gemfile.lock` has json 2.21.2 (fixed in 2.21.2). They
    close once `develop` is merged to `main`.

What blocks further work. Each item needs Titusz:

1. **Cut v0.8.0.** The release criteria are met. Merge `develop` → `main` and run `/release 0.8.0`.
    CID never cuts a release. The merge also closes the stale Dependabot alerts. Release notes
    should mention that `@iscc/wasm` `alg_cdc_chunks` now returns `Uint8Array[]`.
2. **Triage the 7 `untriaged` `low` issues**: promote each to `normal` or park it. The `ci-cd.md`
    tag-trigger realignment and the Sum-Code `KAA` comments are small docs/spec fixes the loop can
    do right after promotion.
3. **End-of-life version floors** (Python 3.10, .NET 8, Ruby 3.1): raise each one or keep it on
    purpose. Only the human may raise a floor.
4. **v1.0.0 and MSRV**: Titusz said "not yet" (2026-11-20). There is no milestone after v0.8.0.
5. **Human-only `low` issues**: the upstream Unicode drift (iscc-core#137), yanking the broken Ruby
    source gems, the napi `Buffer` leak, the JNI guard test, Ruby Float truncation, and the gate
    remainders from iteration 146.

Side note for Titusz: the CI/CD section of target.md says "all 12 sync targets", but
`scripts/version_sync.py` `TARGETS` now holds 21. The criterion is met either way. Only the number
in the human-owned target is out of date.

## Not In Scope

- Any `low` issue, the release itself, and any change to a version floor.

## Verification

- No code change: the advance agent no-ops on NONE.

## Done When

Nothing to do. The advance agent no-ops, and review signals IDLE because every blocker is `low` or
human-gated.
