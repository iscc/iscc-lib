## 2026-03-03 — Add 5 algorithm primitives to Ruby bridge

**Done:** Added 5 algorithm primitive functions (`sliding_window`, `alg_simhash`, `alg_minhash_256`,
`alg_cdc_chunks`, `soft_hash_video_v0`) to the Ruby Magnus bridge, advancing from 25/32 to 30/32
Tier 1 symbols. All functions are exposed as direct module functions (no `_` prefix, no Ruby wrapper
layer), matching the pattern of `text_clean`, `encode_base64`, etc.

**Files changed:**

- `crates/iscc-rb/src/lib.rs`: Added 5 bridge functions with docstrings, registered in `init()`
    under `// Algorithm primitives` section. Updated module docstring symbol count to 30/32.
- `crates/iscc-rb/test/test_smoke.rb`: Added 9 smoke tests covering all 5 functions — return types,
    binary encoding, output sizes, error handling for invalid inputs, and chunk reassembly.

**Verification:**

- `cargo check -p iscc-rb` — compiles successfully
- `cargo clippy -p iscc-rb -- -D warnings` — clean (0 warnings)
- `bundle exec rake compile` — builds native extension in release profile
- `bundle exec rake test` — 46 runs, 121 assertions, 0 failures, 0 errors, 0 skips
- `mise run check` — all 14 pre-commit hooks pass
- `sliding_window("Hello World", 3)` returns Array of 9 Strings
- `alg_simhash` returns binary String matching input digest length
- `alg_minhash_256` returns 32-byte binary String
- `alg_cdc_chunks` returns Array of binary Strings that reassemble to original
- `soft_hash_video_v0` returns binary String of correct size (bits/8)

**Next:** The remaining 2/32 Tier 1 symbols are `DataHasher` and `InstanceHasher` streaming types.
These require Ruby class wrappers (not module functions) — a different pattern from all prior
symbols. This should be a separate step.

**Notes:**

- All 5 functions follow established patterns exactly: `RString` + `unsafe { as_slice() }` for
    binary input, `RString::from_slice` for binary output, `RArray` + `TryConvert` for nested
    arrays, `to_magnus_err` for error mapping.
- `alg_minhash_256` is the only infallible function (no `Result` return) — returns `RString`
    directly.
- `alg_cdc_chunks` holds the unsafe slice only for the Rust call, then copies each chunk slice to
    owned `RString::from_slice(chunk)` — safe pattern.
- Test for `alg_cdc_chunks` includes a reassembly check (chunks joined == original data).
