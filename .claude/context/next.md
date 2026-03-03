# Next Work Package

## Step: Add codec, encoding, and diagnostic functions to Ruby bridge

## Goal

Expose the 6 non-gen, non-algorithm utility symbols — `encode_base64`, `iscc_decompose`,
`encode_component`, `iscc_decode`, `json_to_data_url`, and `conformance_selftest` — in the Ruby
Magnus bridge with smoke tests. This brings the Ruby binding from 10/32 to 16/32 Tier 1 symbols,
covering all codec/encoding operations and the diagnostic function.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-rb/src/lib.rs` — add 6 bridge functions, register in `init()`
    - `crates/iscc-rb/test/test_smoke.rb` — add smoke tests for each new function
- **Reference**:
    - `crates/iscc-py/src/lib.rs` — Python bridge patterns for `encode_base64`, `iscc_decompose`,
        `encode_component`, `iscc_decode`, `json_to_data_url`, `conformance_selftest`
    - `crates/iscc-lib/src/lib.rs` — core API signatures (lines 190–283)
    - `crates/iscc-lib/src/codec.rs` — `encode_base64` (line 442), `iscc_decompose` (line 484)
    - `crates/iscc-lib/src/conformance.rs` — `conformance_selftest` (line 29)

## Not In Scope

- Algorithm primitive functions (`sliding_window`, `alg_simhash`, `alg_minhash_256`,
    `alg_cdc_chunks`) — these have more complex array-of-array argument types and belong in a
    subsequent step
- `soft_hash_video_v0` — needs nested array handling
- Gen functions (`gen_text_code_v0` through `gen_sum_code_v0`) — each needs a Ruby Result class
- Streaming types (`DataHasher`, `InstanceHasher`) — need Ruby class wrappers
- Conformance tests against `data.json` — premature until all 32 symbols are exposed
- Changes to `lib/iscc_lib.rb` Ruby wrapper — these 6 functions are direct utilities that don't need
    Result class wrappers or keyword argument adapters
- CI job or release workflow changes

## Implementation Notes

### Bridge functions to add (6 total)

All functions are exposed directly on the `IsccLib` module (no `_` prefix — the underscore prefix
convention is for gen functions that need Ruby wrapper indirection).

1. **`encode_base64(data)`** — `String → String`. Accept a Ruby String (which holds binary data),
    pass `.as_bytes()` to `iscc_lib::encode_base64`. Returns a URL-safe base64 string.

2. **`iscc_decompose(iscc_code)`** — `String → Array<String>`. Pass to `iscc_lib::iscc_decompose`.
    Returns a Ruby Array of base32-encoded ISCC unit strings. Map `IsccError` to `RuntimeError`.

3. **`encode_component(mtype, stype, version, bit_length, digest)`** —
    `(u32, u32, u32, u32, String) → String`. Accept 4 integers + binary String. Cast the integers
    to `u8`/`u32` as needed. Pass `digest.as_bytes()` to `iscc_lib::encode_component`. Returns a
    base32 ISCC unit string.

4. **`iscc_decode(iscc)`** — `String → Array`. Returns a 5-element Ruby Array:
    `[maintype, subtype, version, length_index, digest_bytes]`. Use `ruby.ary_new_from_values()` or
    build with `RArray`. The digest is a binary Ruby String. The Python bridge returns
    `(u8, u8, u8, u8, bytes)`.

5. **`json_to_data_url(json)`** — `String → String`. Pass to `iscc_lib::json_to_data_url`. Returns a
    `data:` URL string. This function is gated behind `meta-code` feature, which is enabled by
    default in the `iscc-rb` dependency on `iscc-lib`.

6. **`conformance_selftest()`** — `() → bool`. Calls `iscc_lib::conformance_selftest()`. No error
    mapping needed.

### Binary data in Magnus

Ruby `String` holds arbitrary binary data. In Magnus:

- **Accepting bytes**: Take `String` parameter and call `.as_bytes()` to get `&[u8]`
- **Returning bytes**: For `iscc_decode`'s digest, create a Ruby String from `Vec<u8>`. Use Magnus's
    `RString::from_slice` or equivalent to return binary data. Set encoding to `ASCII-8BIT` (Ruby's
    binary encoding) if needed.

### Registration pattern

Follow the existing pattern in `init()`:

```rust
// Codec and encoding functions
module.define_module_function("encode_base64", function!(encode_base64, 1))?;
module.define_module_function("iscc_decompose", function!(iscc_decompose, 1))?;
module.define_module_function("encode_component", function!(encode_component, 5))?;
module.define_module_function("iscc_decode", function!(iscc_decode, 1))?;
module.define_module_function("json_to_data_url", function!(json_to_data_url, 1))?;
module.define_module_function("conformance_selftest", function!(conformance_selftest, 0))?;
```

### Test patterns

Add tests in `test_smoke.rb` following the existing style. Key assertions:

- `encode_base64("Hello".b)` returns a non-empty String
- `iscc_decompose("ISCC:...")` returns an Array of Strings
- `encode_component(0, 0, 0, 64, digest_bytes)` returns a String starting with expected prefix
- `iscc_decode(some_iscc_code)` returns a 5-element Array with integers and a binary String
- `json_to_data_url('{"key":"value"}')` returns a String starting with
    `"data:application/json;base64,"`
- `conformance_selftest` returns `true`
- Round-trip: `encode_component` → `iscc_decode` produces matching components

### Docstring update

Update the module docstring at the top of `lib.rs` to reflect 16 symbols (was 10).

## Verification

- `cargo check -p iscc-rb` compiles successfully
- `cargo clippy -p iscc-rb -- -D warnings` is clean
- `bundle exec rake compile` succeeds (in `crates/iscc-rb/`)
- `bundle exec rake test` passes with ≥16 runs (was 10)
- `bundle exec ruby -e "require 'iscc_lib'; puts IsccLib.conformance_selftest"` prints `true`
- `bundle exec ruby -e "require 'iscc_lib'; puts IsccLib.iscc_decompose('ISCC:AAAWKLHFXM75OAMK').length"`
    prints a positive integer

## Done When

All 6 new bridge functions are callable from Ruby, smoke tests pass (≥16 test runs), clippy is
clean, and `conformance_selftest` returns `true` from Ruby.
