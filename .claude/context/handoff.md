## 2026-02-24 — Validate Content-Code length in `soft_hash_codes_v0`

**Done:** Added length validation to `soft_hash_codes_v0` so that Content-Codes with fewer bits than
the requested `bits` parameter are rejected with `IsccError::InvalidInput`, matching the Python
reference behavior (`code_content_mixed.py:88-90`). Previously the function silently zero-padded
short codes.

**Files changed:**

- `crates/iscc-lib/src/lib.rs`: Changed `_stype` → `stype` and `_blen` → `blen` in the
    `decode_header` destructure, added `decode_length` call and bit-length check returning
    `IsccError::InvalidInput` if too short. Added helper `make_content_code_raw` and 3 new tests.

**Verification:** `cargo test -p iscc-lib` passes — 196 unit tests (193 existing + 3 new), 0
failures. `cargo clippy -p iscc-lib -- -D warnings` clean. `mise run check` passes — all 14
pre-commit hooks green. The 3 new tests: `test_soft_hash_codes_v0_rejects_short_code` (32-bit code
rejected for 64-bit request), `test_soft_hash_codes_v0_accepts_exact_length` (64-bit codes pass for
64-bit request), `test_soft_hash_codes_v0_accepts_longer_codes` (128-bit codes pass for 64-bit
request).

**Next:** The `soft_hash_codes_v0` too-short issue in `issues.md` is resolved. Next highest-impact
work: `gen_meta_code_v0` empty Data-URL handling, `alg_simhash` panic on mismatched digests, or
`sliding_window` panic on width < 2.

**Notes:** The zero-padding logic (lines 596-598) is now dead code for valid inputs since validation
rejects short codes before they reach it. Removing it is a cosmetic cleanup that can wait. The
`SubType::None` variant is used for text content codes (value 0 maps to both "no subtype" and "text"
in the ISCC spec).
