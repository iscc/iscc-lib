# Next Work Package

## Step: Add `gen_iscc_id_v1` minting to the Rust core

## Goal

Implement the one new Tier 1 symbol `gen_iscc_id_v1` (issue #43, GitHub) plus its `IsccIdResult`
type in the pure-Rust core. This is the foundation for the 11-surface fan-out and the last
CID-doable v0.6.0 release blocker; Part 1 (generic decode accepts V1) already landed.

## Alternatives Considered

- **Chosen:** core `gen_iscc_id_v1` minting — every binding surface must wrap a core symbol that
    does not exist yet, so the core is the strict prerequisite for the whole #43 fan-out.
- **Rejected:** the `iscc_clean` codec-input-cleaning divergence (`normal` [review]) — real, but
    independent of the named v0.6.0 release blocker (#43), which wins per gap priority.
- **Rejected:** jump straight to a binding surface (e.g. Python) — impossible before the core symbol
    exists; also the Python differential test needs a rebuilt wheel that this core-only step omits.

## Scope

- **Modify**: `crates/iscc-lib/src/types.rs` (add `IsccIdResult { pub iscc: String }`,
    `#[non_exhaustive]`, matching the other result structs), `crates/iscc-lib/src/lib.rs` (add
    `pub fn gen_iscc_id_v1` + `#[test]`s).
- **Reference**: `.claude/context/specs/rust-core.md` → "ISCC-IDv1 Operations (Experimental)" (lines
    335-540); `reference/iscc-core/iscc_core/iscc_id.py` `gen_iscc_id_v1` (lines 59-149);
    `crates/iscc-lib/src/codec.rs` `encode_component` (line 490). `.crap-baseline.json` (generated;
    re-baseline, not counted against the file budget).

## Not In Scope

- The 11 binding surfaces, the Go `DecodeIsccID`/`IsccIDv1Result` deletion + `EncodeIsccID` rename,
    and the Python differential test against `iscc_core` — each a separate follow-up step.
- The Tier-1 32→33 doc/count sweep across `docs/`, `notes/`, CLAUDE.md/README (separate step).
- Any `decode_iscc_id_v1` — deliberately does not exist (Titusz, 2026-07-28); generic `iscc_decode`
    covers decoding.
- Adding realm variants to `SubType`, or any time/clock dependency — the core is clock-free.
- Touching the `iscc_clean` divergence issue.

## Implementation Notes

- Signature:
    `pub fn gen_iscc_id_v1(timestamp: u64, hub_id: u16, realm: u8) -> IsccResult<IsccIdResult>`.
    Mark experimental in the doc comment. Add to crate root — a `pub fn` in `lib.rs` is already at
    the root; `IsccIdResult` re-exports via the existing `pub use types::*`.
- Validation order is normative, first failure wins, exact core text: `timestamp >= 2^52` →
    `IsccError::InvalidInput("Timestamp overflow")`; `hub_id >= (1 << 12)` → `"HUB-ID overflow"`;
    `realm` not in `(0, 1)` → `"Realm-ID must be 0 (test) or 1 (operational)"`.
- Encode exactly like the reference: `body = (timestamp << 12) | hub_id as u64`,
    `digest = body.to_be_bytes()`, then
    `encode_component(MainType::Id, SubType::try_from(realm)?, Version::V1, 64, &digest)` and
    prepend `"ISCC:"`. `SubType::try_from(1)` yielding `Image` is cosmetic — only the nibble is
    written.
- cargo tests (Python oracle differential is out of scope — it needs the wheel):
    - Golden: `gen_iscc_id_v1(1751831876325218, 1, 0).unwrap().iscc == "ISCC:MAIGHFECJMOPMIAB"`.
    - Round-trip via existing `iscc_decode` across realm {0,1} × hub_id {0,4095} × timestamp {0,
        `2^52 - 1`}: decode returns `(6, realm, 1, 0, 8 bytes)`, and `body>>12 == timestamp`,
        `body & 0xFFF == hub_id`.
    - Validation ordering: assert the correct message fires first for each overflow case.
- Adding a fn shifts line numbers in `lib.rs`, so the CI-only CRAP `--fail-regression` gate will
    likely report moved/new functions. Re-run `mise run coverage && mise run crap:baseline` and
    commit the refreshed `.crap-baseline.json` in this same step. No iai baseline change (no new
    bench). Semver stays informational (additive symbol, no break).

## Verification

- `cargo test -p iscc-lib` passes (existing suite + new `gen_iscc_id_v1` golden, round-trip, and
    validation-ordering tests).
- `iscc_lib::gen_iscc_id_v1` and `iscc_lib::IsccIdResult` resolve from the crate root (exercised by
    the new tests compiling).
- `cargo clippy -p iscc-lib -- -D warnings` clean and `cargo fmt -p iscc-lib --check` clean.
- `mise run coverage` then
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json   --fail-regression --fail-above`
    exits 0 (re-baseline committed if it flagged moves).

## Done When

The core mints a correct ISCC-IDv1 via `gen_iscc_id_v1` with passing golden/round-trip/validation
tests and all four verification checks green.
