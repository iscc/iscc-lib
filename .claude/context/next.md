# Next Work Package

## Step: Add gen_instance/iscc/sum_code_v0 to Ruby bridge

## Goal

Add the final 3 gen functions (`gen_instance_code_v0`, `gen_iscc_code_v0`, `gen_sum_code_v0`) to the
Ruby Magnus bridge, completing all 10 gen functions and advancing from 22/32 to 25/32 Tier 1
symbols.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-rb/src/lib.rs` — add 3 Rust bridge functions + register in `init()`
    - `crates/iscc-rb/lib/iscc_lib.rb` — add 3 result classes + 3 wrapper methods with keyword args
    - `crates/iscc-rb/test/test_smoke.rb` — add smoke tests for the 3 new functions
- **Reference**:
    - `crates/iscc-lib/src/types.rs` — `InstanceCodeResult`, `IsccCodeResult`, `SumCodeResult` struct
        definitions
    - `crates/iscc-py/src/lib.rs` — Python bridge patterns for these 3 functions (lines 305-352)
    - `crates/iscc-lib/src/api.rs` — core function signatures

## Not In Scope

- Algorithm primitives (`sliding_window`, `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`) —
    those are a separate batch
- `soft_hash_video_v0` — separate batch with algorithm primitives
- Streaming types (`DataHasher`, `InstanceHasher`) — separate batch after algorithms
- Conformance tests against `data.json` — deferred until all 32 symbols are exposed
- Ruby CI job, RubyGems release step, version_sync — deferred until all symbols are done
- Refactoring the existing `to_magnus_err` helper or result class hierarchy

## Implementation Notes

### `gen_instance_code_v0`

- **Rust signature**:
    `gen_instance_code_v0(data: &[u8], bits: u32) -> IsccResult<InstanceCodeResult>`
- **Bridge**: accept `RString` for binary data (same pattern as `gen_data_code_v0`), use
    `unsafe { data.as_slice() }` with safety comment
- **Return fields**: `iscc` (String), `datahash` (String), `filesize` (u64)
- **Ruby wrapper**: `InstanceCodeResult < Result`, keyword arg `bits: 64`
- Note: `bits` parameter is accepted for API consistency but ignored internally (always 256-bit)

### `gen_iscc_code_v0`

- **Rust signature**: `gen_iscc_code_v0(codes: &[&str], wide: bool) -> IsccResult<IsccCodeResult>`
- **Bridge**: accept `Vec<String>` for codes (same pattern as `gen_mixed_code_v0`), convert to
    `Vec<&str>`. Second param is `bool` for `wide`
- **Return fields**: `iscc` (String) only
- **Ruby wrapper**: `IsccCodeResult < Result`, keyword arg `wide: false`
- Note: test vectors have no `wide` field — always pass `false` in tests

### `gen_sum_code_v0`

- **Rust signature**:
    `gen_sum_code_v0(path: &Path, bits: u32, wide: bool, add_units: bool) ->   IsccResult<SumCodeResult>`
- **Bridge**: accept `String` path, convert to `std::path::Path`. 4 parameters total
- **Return fields**: `iscc` (String), `datahash` (String), `filesize` (u64), `units`
    (Option\<Vec<String>>) — only include `units` key when `Some`
- **Ruby wrapper**: `SumCodeResult < Result`, keyword args `bits: 64, wide: false, add_units: false`
- **Register**: `function!(gen_sum_code_v0, 4)` (4 positional args from Ruby underscore-prefixed
    function)

### Module registration

- Register all 3 with `_` prefix: `_gen_instance_code_v0` (2 args), `_gen_iscc_code_v0` (2 args),
    `_gen_sum_code_v0` (4 args)
- Update the docstring count from "22 of 32" to "25 of 32"

### Smoke tests

- `gen_instance_code_v0`: pass binary data, verify `iscc` starts with `"ISCC:"`, verify `datahash`
    and `filesize` keys exist, verify `filesize` matches input length
- `gen_iscc_code_v0`: generate a data code + instance code first, pass both ISCCs, verify result
    `iscc` starts with `"ISCC:"`
- `gen_sum_code_v0`: create a temp file with known content, pass its path, verify `iscc`,
    `datahash`, `filesize` keys; test with `add_units: true` and verify `units` is an Array

## Verification

- `cargo check -p iscc-rb` compiles successfully
- `cargo clippy -p iscc-rb -- -D warnings` — clean (0 warnings)
- `bundle exec rake compile` builds native extension (run from `crates/iscc-rb/`)
- `bundle exec rake test` — all tests pass with 0 failures, 0 errors
- `gen_instance_code_v0` smoke test verifies `iscc`, `datahash`, `filesize` fields
- `gen_iscc_code_v0` smoke test verifies `iscc` field from composite codes
- `gen_sum_code_v0` smoke test verifies `iscc`, `datahash`, `filesize`, and optional `units` field
- `mise run check` — all pre-commit hooks pass

## Done When

All verification criteria pass, confirming that the Ruby bridge exposes all 10 gen functions with
correct parameter handling, result types, and smoke tests.
