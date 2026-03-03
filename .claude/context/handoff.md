## 2026-03-03 — Review of: Add gen_video/mixed/data_code_v0 to Ruby bridge

**Verdict:** PASS

**Summary:** Added 3 gen functions (`gen_video_code_v0`, `gen_mixed_code_v0`, `gen_data_code_v0`) to
the Ruby Magnus bridge, advancing from 19/32 to 22/32 Tier 1 symbols. Implementation follows
established patterns exactly with correct type conversions for each function's unique parameter
types (nested arrays, string arrays, binary data). Clean code, all tests pass.

**Verification:**

- [x] `cargo check -p iscc-rb` compiles successfully
- [x] `cargo clippy -p iscc-rb -- -D warnings` — clean (0 warnings)
- [x] `bundle exec rake compile` — builds native extension in release profile
- [x] `bundle exec rake test` — 31 runs, 75 assertions, 0 failures, 0 errors, 0 skips
- [x] `gen_video_code_v0` one-liner — prints `ISCC:EMAVFD4RIMPXYSWS`
- [x] `gen_data_code_v0` one-liner — prints `ISCC:GAAY5EN4BF2LH3FW`
- [x] `mise run check` — all 14 pre-commit hooks pass

**Issues found:**

- (none)

**Codex review:** Flagged potential panic in `iscc_lib::soft_hash_video_v0` when frame signatures
have inconsistent lengths (later frame longer than first causes index-out-of-bounds). This is a
**Rust core** issue affecting all bindings equally, not specific to the Ruby bridge. The Ruby bridge
correctly delegates to the core function. Advisory — consider adding length validation in the core
`soft_hash_video_v0` as a separate improvement.

**Next:** Continue adding gen functions to Ruby bridge. Next batch: `gen_instance_code_v0` (binary
data → hash with `iscc`, `datahash`, `filesize`), `gen_iscc_code_v0` (string arrays + wide flag →
hash with `iscc` + `units`), and `gen_sum_code_v0` (file path string + `add_units` bool → hash with
`iscc`, `datahash`, `filesize`, optional `units`). These complete all 10 gen functions (25/32
symbols).

**Notes:**

- Ruby bridge now at 22/32 symbols. Remaining 10: 3 gen functions (`gen_instance_code_v0`,
    `gen_iscc_code_v0`, `gen_sum_code_v0`), 4 algorithm primitives (`sliding_window`, `alg_simhash`,
    `alg_minhash_256`, `alg_cdc_chunks`), 1 utility (`alg_simhash_from_iscc`), 2 streaming types
    (`DataHasher`, `InstanceHasher`).
- `gen_instance_code_v0` accepts `bits` but ignores it (always 256-bit) — binding should still
    accept the parameter for API consistency.
- `gen_iscc_code_v0` has a `wide` bool parameter and complex return with `units` field.
- `gen_sum_code_v0` takes a file path string — check how other bindings handle this (file path vs
    binary data).
