## 2026-02-23 — Review of: Implement iscc_decompose and decode_units codec functions

**Verdict:** PASS

**Summary:** Clean implementation of `decode_units` (bitfield decoding, inverse of `encode_units`)
and `iscc_decompose` (Tier 1 function decomposing composite ISCC-CODEs into constituent ISCC-UNITs).
Both functions faithfully mirror the Python reference. 17 well-structured tests cover all 8
`decode_units` mappings, roundtrip with `encode_units`, single-unit decomposition (with/without
prefix), and all 5 `gen_iscc_code_v0` conformance vectors with structural validation. All 164 Rust
tests pass, clippy clean workspace-wide, all pre-commit hooks pass.

**Issues found:**

- (none)

**Next:** 20 of 22 Tier 1 symbols implemented. Remaining 2: `DataHasher`/`InstanceHasher` (streaming
types implementing `new() → update(&[u8]) → finalize()`) and `conformance_selftest`. The streaming
types are the next logical step — they enable the streaming API pattern central to the library's
design. `state.md` should be updated to reflect 20/22 Tier 1 symbols.

**Notes:** `iscc_decompose` intentionally skips `normalize_multiformat` (handles alternative
multibase encodings). Standard ISCC strings are always base32, so this is fine for now. Can be added
later if needed. The function returns ISCC-UNIT strings WITHOUT "ISCC:" prefix, matching Python
`encode_component` behavior.
