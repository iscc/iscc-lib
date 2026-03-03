## 2026-03-03 — Add gen_instance/iscc/sum_code_v0 to Ruby bridge

**Done:** Added the final 3 gen functions (`gen_instance_code_v0`, `gen_iscc_code_v0`,
`gen_sum_code_v0`) to the Ruby Magnus bridge, completing all 10 gen functions and advancing from
22/32 to 25/32 Tier 1 symbols.

**Files changed:**

- `crates/iscc-rb/src/lib.rs`: Added 3 Rust bridge functions (`gen_instance_code_v0` with RString
    binary input, `gen_iscc_code_v0` with Vec<String> + bool, `gen_sum_code_v0` with String path + 3
    params). Registered all 3 with `_` prefix in `init()`. Updated docstring count to 25/32.
- `crates/iscc-rb/lib/iscc_lib.rb`: Added 3 result classes (`InstanceCodeResult`, `IsccCodeResult`,
    `SumCodeResult`) and 3 wrapper methods with keyword args matching the API conventions (bits: 64,
    wide: false, add_units: false).
- `crates/iscc-rb/test/test_smoke.rb`: Added 6 tests: basic + attribute access for
    `gen_instance_code_v0` (verifies iscc, datahash, filesize), basic + attribute for
    `gen_iscc_code_v0` (generates data+instance codes then composes), basic + with_units for
    `gen_sum_code_v0` (uses Tempfile, verifies units array).

**Verification:** 37 runs, 95 assertions, 0 failures, 0 errors, 0 skips. `cargo check -p iscc-rb`
clean. `cargo clippy -p iscc-rb -- -D warnings` clean (0 warnings). `bundle exec rake compile`
builds native extension in release profile. `mise run check` — all 14 pre-commit hooks pass.

**Next:** Continue with remaining 7/32 Tier 1 symbols: 4 algorithm primitives (`sliding_window`,
`alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`), 1 utility (`alg_simhash_from_iscc`), and 2
streaming types (`DataHasher`, `InstanceHasher`).

**Notes:** `gen_sum_code_v0` signature was collapsed to single line by `cargo fmt` (multi-line
params → single line). No issues — all patterns follow existing conventions exactly. The `units`
field in `SumCodeResult` is conditionally included (only when `Some`), matching the Python binding
pattern.
