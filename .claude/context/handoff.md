# Handoff

## 2026-07-28 — Review of: Add `gen_iscc_id_v1` minting to the Rust core

**Verdict:** PASS_WITH_NOTES

**Summary:** Tier 1
`gen_iscc_id_v1(timestamp: u64, hub_id: u16, realm: u8) -> IsccResult<IsccIdResult>` and the
`IsccIdResult { iscc }` struct land in the pure-Rust core (#43 Part 2), matching the `iscc-core`
reference exactly. Clock-free, `#[non_exhaustive]`, additive (no API break). Code is correct and
oracle-verified; all four verification criteria are green. The only blemish is a stale note carried
in the handoff/advance-memory (see Notes).

**Verification:**

- [x] `cargo test -p iscc-lib` — 302 passed, 0 failed; new golden/round-trip/validation-ordering
    tests green.
- [x] `iscc_lib::gen_iscc_id_v1` / `IsccIdResult` resolve from crate root — tests compile and pass.
- [x] `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean; `cargo fmt -p iscc-lib --check`
    clean. (The `proc-macro-error2` future-incompat note is a pre-existing transitive-dep warning,
    not from this crate.)
- [x] `mise run coverage` + `cargo crap … --fail-regression --fail-above` — exit 0:
    `0 regressed, 0 new, 0 moved, 108 unchanged`; baseline entry for `gen_iscc_id_v1` committed.
- [x] (extra) Oracle parity vs pip-installed `iscc-core` 1.3.0: golden `ISCC:MAIGHFECJMOPMIAB`,
    realm1 `ISCC:MEIGHFECJMOPMIAB`, zero `ISCC:MAIAAAAAAAAAAAAA`, max `ISCC:MEIP777777777777` — Rust
    matches byte-for-byte.
- [x] (extra) `cargo test -p iscc-lib --no-default-features` — passes (see Notes: the "fails to
    compile" claim is stale).
- [x] `mise run check` — all prek hooks Passed; no context files reformatted.

**Issues found:** (none material). Scope clean: only 2 non-test/non-doc source files touched
(`lib.rs`, `types.rs`) + the generated `.crap-baseline.json`. All Not-In-Scope items respected — no
binding surfaces, no doc sweep, no `decode_iscc_id_v1`, no `SubType` realm variants.

**Codex review:** Clean — "implementation matches the documented IDv1 algorithm, validation order,
and result shape. Full crate tests, clippy, formatting, doc tests, and the no-default-features build
pass." No actionable findings.

**Next:** Issue #43 fan-out continues. Suggested order: (1) 11-surface binding fan-out wrapping
`gen_iscc_id_v1` (a `**Fan-out:**`-eligible mechanical change); (2) Go `EncodeIsccID` rename +
`DecodeIsccID`/`IsccIDv1Result` deletion; (3) Python differential test vs `iscc_core` (needs a
rebuilt wheel — but the reference itself is oracle-checkable without one, see learnings); (4) the
Tier-1 32→33 doc/count sweep across `docs/`, `notes/`, CLAUDE.md/README + the three non-Markdown
sources named in issue #43.

**Notes:**

- **Stale claim to ignore:** the handoff and advance memory state
    `cargo test -p iscc-lib --no-default-features` "fails to COMPILE" (a phantom from iter 174). It
    does NOT — that test (`test_iscc_decode_rejects_uncomposable_sequence`, lib.rs:2167) is already
    gated `#[cfg(feature = "meta-code")]` (line 2165), and all feature combos pass green
    (233+28+10+2+0). No work owed; do not scope a fix.
- Validation order matches the reference, first failure wins: `timestamp >= 2^52` → "Timestamp
    overflow"; `hub_id >= 2^12` → "HUB-ID overflow"; `realm ∉ {0,1}` → "Realm-ID must be 0 (test) or
    1 (operational)". `IsccError` Display prefix is `invalid input: `.
- Tier 1 is now **33 symbols**. Doc/count text still reads 32 by design (separate sweep step, #43).
- Additive symbol — `semver` stays informational (expected pre-1.0). No new bench, no iai change.
