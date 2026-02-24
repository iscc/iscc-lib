## 2026-02-24 — Review of: Validate Content-Code length in `soft_hash_codes_v0`

**Verdict:** PASS

**Summary:** Clean, minimal fix adding length validation to `soft_hash_codes_v0` that matches the
Python reference behavior exactly (`code_content_mixed.py:88-90`). Three well-structured tests cover
rejection, exact-length acceptance, and longer-code acceptance. The `make_content_code_raw` helper
constructs test data via a full encode/decode roundtrip, ensuring realistic inputs.

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 196 unit + 28 algo + 22 text = 246 total, 0 failures
- [x] `cargo clippy -p iscc-lib -- -D warnings` clean — no warnings
- [x] `cargo test -p iscc-lib -- test_soft_hash_codes` passes — 3 new validation tests pass
- [x] The rejection test asserts `Err(IsccError::InvalidInput(_))` containing "too short" — verified
    in source (line 1342)
- [x] `mise run check` passes — all 14 pre-commit hooks green

**Issues found:**

- (none)

**Next:** The `soft_hash_codes_v0` too-short issue is resolved and deleted from issues.md. The next
highest-impact work should target one of the remaining normal-priority robustness issues:
`gen_meta_code_v0` empty Data-URL handling, `alg_simhash` panic on mismatched digests, or
`sliding_window` panic on width < 2. All are straightforward defensive fixes similar in scope to
this iteration.

**Notes:** The zero-padding logic (lines 601-604 in lib.rs) is now dead code for valid inputs since
validation rejects short codes before they reach it. The `next.md` explicitly scoped this as "not in
scope" to remove, which is fine — it's a cosmetic cleanup. The `decode_length` call correctly passes
`stype` which the Rust version requires (Python infers it differently but the result is identical
for Content-Codes).
