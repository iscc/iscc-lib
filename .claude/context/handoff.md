## 2026-03-03 — Review of: Add gen_instance/iscc/sum_code_v0 to Ruby bridge

**Verdict:** PASS

**Summary:** Added the final 3 gen functions (`gen_instance_code_v0`, `gen_iscc_code_v0`,
`gen_sum_code_v0`) to the Ruby Magnus bridge, completing all 10 gen functions and advancing from
22/32 to 25/32 Tier 1 symbols. Implementation is clean, follows established patterns exactly, and
all tests pass with 37 runs, 95 assertions, 0 failures.

**Verification:**

- [x] `cargo check -p iscc-rb` compiles successfully
- [x] `cargo clippy -p iscc-rb -- -D warnings` — clean (0 warnings)
- [x] `bundle exec rake compile` — builds native extension in release profile
- [x] `bundle exec rake test` — 37 runs, 95 assertions, 0 failures, 0 errors, 0 skips
- [x] `gen_instance_code_v0` smoke test verifies `iscc`, `datahash`, `filesize` fields
- [x] `gen_iscc_code_v0` smoke test verifies `iscc` field from composite codes
- [x] `gen_sum_code_v0` smoke test verifies `iscc`, `datahash`, `filesize`, and optional `units`
    field
- [x] `mise run check` — all 14 pre-commit hooks pass

**Issues found:**

- (none)

**Codex review:** Flagged a potential portability issue: `test_gen_sum_code_v0_*` tests call
`gen_sum_code_v0(file.path)` while the `Tempfile` handle is still open (flushed but not closed). On
Windows with strict file-locking this could fail. Advisory only — Ruby CI currently runs on Linux
and this matches common Ruby testing patterns. Consider `file.close(false)` before the API call if
Windows CI is added later.

**Next:** Continue with remaining 7/32 Tier 1 symbols: 4 algorithm primitives (`sliding_window`,
`alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`), 1 utility (`alg_simhash_from_iscc`), and 2
streaming types (`DataHasher`, `InstanceHasher`). The algorithm primitives and utility can likely be
batched together (5 symbols), leaving the 2 streaming types for a final batch.

**Notes:**

- All 10 gen functions now complete in Ruby. Pattern is fully established — each gen function has:
    Rust bridge function, `_` prefixed registration, Ruby result class, keyword-arg wrapper, and 2
    smoke tests (basic + attribute access).
- The remaining 7 symbols have different patterns: algorithm primitives are lower-level functions
    with simpler signatures; streaming types (`DataHasher`, `InstanceHasher`) need Ruby class
    wrappers (not just module functions).
- `gen_sum_code_v0` correctly handles the `units` field as conditional (only included when `Some`),
    matching the Python binding pattern.
