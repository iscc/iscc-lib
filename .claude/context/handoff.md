# Handoff

## 2026-07-28 — Reject truncated varnibble header fields in `decode_header`

**Done:** Replaced the three `as u8` narrowing casts in `decode_header` (codec.rs) with
range-checked `u8::try_from` conversions, so a multi-nibble varnibble value that overflows a `u8` is
rejected before the enum `TryFrom` runs — closing the critical bug where a malformed header wrapped
(version `257`→`1`, MainType `262`→`6`=`Id`) and silently canonicalized to a valid ISCC.

**Files changed:**

- `crates/iscc-lib/src/codec.rs`: `decode_header` now converts `mtype_val`/`stype_val`/`version_val`
    via `u8::try_from(..).map_err(|_| IsccError::InvalidInput(..))?` before `MainType::try_from`
    etc. Added `#[test] test_decode_header_rejects_truncated_varnibble_fields` asserting both
    `iscc_decode("MDFZAAAAAAAAAAAAAA")` and `iscc_decompose("MDFZAAAAAAAAAAAAAA")` return `Err`.

**Verification:**

- `cargo test -p iscc-lib` — **299 passed** (298 prior + 1 new), 0 failed. New test green; all
    existing codec/conformance/boundary tests unchanged.
- New test confirms `iscc_decode("MDFZAAAAAAAAAAAAAA")` and `iscc_decompose(..)` both `Err`.
- Valid path preserved: existing `test_decode_header_idv1_version1` /
    `test_decompose_idv1_accepts_version1` still green, so `iscc_decode("ISCC:MAIGHFECJMOPMIAB")` →
    `(6, 0, 1, 0, <8 bytes>)` and decompose → `["MAIGHFECJMOPMIAB"]`.
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` — clean (the `proc-macro-error2`
    future-incompat note is a transitive dep, not our code; clippy finished OK).
- `cargo fmt -p iscc-lib --check` — clean.

**Next:** Prerequisite is now green — the Part 2 `gen_iscc_id_v1` minting + `IsccIdResult` step
(issue #43) can proceed. Also note the whole NEEDS_WORK batch from 174 (incl. out-of-loop commit
`2c4e487`) is still unpushed and CI-unverified; the next cycle pushes the full batch.

**Notes:**

- Pure core codec fix, no signature change, no API break. No binding rebuild needed.
- Applied the same range-check to `stype_val` too (not just mtype/version) for consistency — a
    wrapped SubType would have been an equivalent latent defect; message style matches existing
    `InvalidInput` format.
- Did not run `mise run check` (full prek suite); relied on the targeted clippy/fmt/test gates the
    work package named. Batch is unpushed so pre-push mdformat is not yet exercised.
