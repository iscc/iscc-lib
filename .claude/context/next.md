# Next Work Package

## Step: Reject truncated varnibble header fields in `decode_header`

## Goal

Fix the open `critical` issue "`decode_header` truncates varnibble fields, canonicalizing malformed
input": a multi-nibble header value wraps under `as u8` (version `257`→`1`, MainType `262`→`6`=`Id`)
so a malformed string silently canonicalizes to a valid ISCC. This is the prerequisite the 174
review named before the unpushed IDv1-decode batch can be pushed green.

## Alternatives Considered

- **Chosen:** the `critical` `decode_header` fix — a `critical` issue outranks everything, and the
    handoff makes it the explicit gate for pushing the whole NEEDS_WORK batch.
- **Rejected:** Part 2 `gen_iscc_id_v1` minting (issue #43) — the handoff states its prerequisite is
    this fix landing green; scoping it now would ship on top of an unpushed, CI-unverified, buggy
    batch.

## Scope

- **Modify**: `crates/iscc-lib/src/codec.rs` (the three narrowing casts at ~321-323 + one new
    `#[cfg(test)]` case)
- **Reference**: injected issues.md (`decode_header` critical entry) and handoff.md "Next"; the
    `Version`/`validate_version` code at codec.rs:96-135 is already in this session

## Not In Scope

- `gen_iscc_id_v1` minting, the Go rename/deletion, or any Tier-1-count doc edits (issue #43, later
    step).
- The `iscc_clean` codec-input-cleaning divergence (separate open `normal` issue).
- Touching `encode_header` or `validate_version` — the wrap happens only on the decode narrowing.
- Rebuilding or re-verifying bindings; this is a pure core codec fix with no signature change.

## Implementation Notes

- Replace `mtype_val as u8` / `stype_val as u8` / `version_val as u8` (codec.rs ~321-323) with a
    range-checked conversion so a `u32` that does not fit `u8` is rejected *before* the enum
    `TryFrom` runs. `u8::try_from(262)` fails, so it catches the wrap; `MainType::try_from` then
    rejects in-`u8`-range-but-invalid values as today.
- `IsccError` has no `From<TryFromIntError>`, so map the error explicitly, e.g.
    `u8::try_from(mtype_val).map_err(|_| IsccError::InvalidInput(format!("invalid MainType: {mtype_val}")))?`
    then feed into `MainType::try_from`. Keep the existing `InvalidInput` message style.
- Add one `#[test]` in the existing `mod tests` (near the version-1 rejection tests at ~1128)
    asserting both `iscc_decode("MDFZAAAAAAAAAAAAAA")` and `iscc_decompose("MDFZAAAAAAAAAAAAAA")`
    return `Err` — on HEAD they wrongly return `(6,0,1,0,…)` and `["MAIAAAAAAAAAAAAA"]`.
- Do not weaken any existing test; all 298 current cases plus the new one must pass.

## Verification

- `cargo test -p iscc-lib` passes (298 existing + 1 new case) with the new rejection test green.
- `iscc_decode("MDFZAAAAAAAAAAAAAA")` and `iscc_decompose("MDFZAAAAAAAAAAAAAA")` both return `Err`
    (asserted by the new test), while `iscc_decode("ISCC:MAIGHFECJMOPMIAB")` still returns
    `(6, 0, 1, 0, <8 bytes>)`.
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean.
- `cargo fmt -p iscc-lib --check` clean.

## Done When

`decode_header` rejects any wrapped multi-nibble header before narrowing, the new rejection test and
all existing codec/conformance tests pass, and clippy + fmt are clean.
