## 2026-03-03 — Add codec, encoding, and diagnostic functions to Ruby bridge

**Done:** Added 6 bridge functions (`encode_base64`, `iscc_decompose`, `encode_component`,
`iscc_decode`, `json_to_data_url`, `conformance_selftest`) to the Ruby Magnus bridge, bringing the
binding from 10/32 to 16/32 Tier 1 symbols. All functions registered in `init()` and covered by
smoke tests.

**Files changed:**

- `crates/iscc-rb/src/lib.rs`: Added 6 bridge functions with docstrings, updated module docstring to
    reflect 16 symbols, added `RArray`, `RString` imports for binary data handling, registered all 6
    functions in `init()`
- `crates/iscc-rb/test/test_smoke.rb`: Added 9 new test methods covering all 6 functions including
    error handling and encode/decode round-trip

**Verification:**

- `cargo check -p iscc-rb` — compiles successfully
- `cargo clippy -p iscc-rb -- -D warnings` — clean, 0 warnings
- `bundle exec rake compile` — builds native extension in release profile
- `bundle exec rake test` — 19 runs, 47 assertions, 0 failures, 0 errors, 0 skips
- `conformance_selftest` from Ruby returns `true`
- `iscc_decompose("ISCC:AAAWKLHFXM75OAMK")` returns array with length 1
- `mise run check` — all 14 pre-commit hooks pass

**Next:** Continue adding Tier 1 symbols to the Ruby bridge. Suggested next batch: remaining gen
functions (`gen_text_code_v0`, `gen_image_code_v0`, `gen_audio_code_v0`, `gen_video_code_v0`,
`gen_mixed_code_v0`, `gen_data_code_v0`, `gen_instance_code_v0`, `gen_iscc_code_v0`,
`gen_sum_code_v0`) — each needs a Ruby Result class added to `lib/iscc_lib.rb`.

**Notes:**

- Binary data handling: Magnus `String` type validates UTF-8. For `encode_base64` and
    `encode_component` which accept arbitrary binary data, switched to `RString` parameter type and
    `unsafe { data.as_slice() }` to access raw bytes. The unsafe block is sound because we copy
    bytes immediately (or pass directly to a function that copies) before any Ruby API calls.
- `iscc_decode` returns an `RArray` with 5 elements. The digest bytes are returned as an `RString`
    created via `RString::from_slice(&digest)`.
- These 6 functions are registered without `_` prefix (direct utilities, no Ruby wrapper needed) as
    specified in next.md.
