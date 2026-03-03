# Next Work Package

## Step: Add 5 algorithm primitives to Ruby bridge

## Goal

Expose the 5 algorithm primitive symbols (`sliding_window`, `alg_simhash`, `alg_minhash_256`,
`alg_cdc_chunks`, `soft_hash_video_v0`) in the Ruby Magnus bridge, advancing from 25/32 to 30/32
Tier 1 symbols. These are lower-level functions with simple signatures — no result classes or Ruby
wrappers needed.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-rb/src/lib.rs` — add 5 bridge functions + register in `init()`
    - `crates/iscc-rb/test/test_smoke.rb` — add smoke tests for each function
- **Reference**:
    - `crates/iscc-lib/src/simhash.rs` — `sliding_window` and `alg_simhash` signatures
    - `crates/iscc-lib/src/minhash.rs` — `alg_minhash_256` signature
    - `crates/iscc-lib/src/cdc.rs` — `alg_cdc_chunks` signature
    - `crates/iscc-lib/src/lib.rs` — `soft_hash_video_v0` signature
    - `crates/iscc-py/src/lib.rs` — Python bridge patterns for these 5 functions

## Not In Scope

- `DataHasher` and `InstanceHasher` streaming types — those are the remaining 2/32 symbols and
    require Ruby class wrappers (not module functions), so they belong in a separate step
- Modifying `lib/iscc_lib.rb` — algorithm primitives are exposed directly on the `IsccLib` module
    (no `_` prefix, no keyword-arg wrapper), matching the pattern of `text_clean`, `encode_base64`
- Conformance tests against `data.json` — a separate step after all 32 symbols are complete
- `alg_simhash_from_iscc` — this is NOT one of the 32 Tier 1 symbols; do not add it
- Updating the docstring symbol count in lib.rs — do that when all 32 are done

## Implementation Notes

All 5 functions are exposed as direct module functions (no `_` prefix, no Ruby wrapper layer). They
follow the same pattern as `text_clean`, `encode_base64`, etc.

**Signature mapping (Rust → Ruby):**

1. **`sliding_window`**: `(seq: &str, width: usize) → IsccResult<Vec<String>>`

    - Ruby: `sliding_window(String, Integer) → Array<String>`
    - Simple — direct argument mapping, error maps to `RuntimeError`

2. **`alg_simhash`**: `(hash_digests: &[impl AsRef<[u8]>]) → IsccResult<Vec<u8>>`

    - Ruby: `alg_simhash(Array<String>) → String` (binary)
    - Accept `RArray`, iterate to collect `Vec<Vec<u8>>` from each element's bytes
    - Return binary `RString` via `RString::from_slice(&result)`

3. **`alg_minhash_256`**: `(features: &[u32]) → Vec<u8>`

    - Ruby: `alg_minhash_256(Array<Integer>) → String` (binary)
    - Infallible — no `Result`, no error mapping
    - Accept `Vec<u32>` (Magnus auto-converts), return `RString::from_slice(&result)`

4. **`alg_cdc_chunks`**: `(data: &[u8], utf32: bool, avg_chunk_size: u32) → Vec<&[u8]>`

    - Ruby: `alg_cdc_chunks(String, Boolean, Integer) → Array<String>` (binary strings)
    - Accept `RString` for binary data, use `unsafe { data.as_slice() }` (same pattern as
        `gen_data_code_v0`)
    - Copy each returned slice to owned `RString::from_slice(chunk)` in a Ruby array
    - Default `avg_chunk_size` = 1024 in Python; Ruby doesn't need default (positional args)

5. **`soft_hash_video_v0`**:
    `(frame_sigs: &[S: AsRef<[i32]> + Ord], bits: u32) → IsccResult<Vec<u8>>`

    - Ruby: `soft_hash_video_v0(Array<Array<Integer>>, Integer) → String` (binary)
    - Reuse the exact same `RArray` → `Vec<Vec<i32>>` conversion pattern from `gen_video_code_v0`
    - Return binary `RString` via `RString::from_slice(&result)`

**Registration in `init()`**: Add under a new `// Algorithm primitives` comment section, using
direct names (no `_` prefix):

```rust
module.define_module_function("sliding_window", function!(sliding_window, 2))?;
module.define_module_function("alg_simhash", function!(alg_simhash, 1))?;
module.define_module_function("alg_minhash_256", function!(alg_minhash_256, 1))?;
module.define_module_function("alg_cdc_chunks", function!(alg_cdc_chunks, 3))?;
module.define_module_function("soft_hash_video_v0", function!(soft_hash_video_v0, 2))?;
```

**Test patterns**: Each function gets 1-2 smoke tests verifying:

- Return type (String for binary, Array for sequences)
- Non-empty output for valid input
- Error raised for invalid input (where applicable: `sliding_window` with width < 2, `alg_simhash`
    with mismatched digest lengths)

## Verification

- `cargo check -p iscc-rb` compiles successfully
- `cargo clippy -p iscc-rb -- -D warnings` clean (0 warnings)
- `bundle exec rake compile` builds native extension in release profile (run from `crates/iscc-rb/`)
- `bundle exec rake test` passes with 0 failures (run from `crates/iscc-rb/`); expect ~42-47 runs
    (37 existing + 5-10 new)
- `IsccLib.sliding_window("Hello World", 3)` returns an Array of Strings
- `IsccLib.alg_simhash` returns a binary String
- `IsccLib.alg_minhash_256` returns a 32-byte binary String
- `IsccLib.alg_cdc_chunks` returns an Array of binary Strings
- `IsccLib.soft_hash_video_v0` returns a binary String

## Done When

All verification criteria pass: the 5 algorithm primitive functions are callable from Ruby with
correct return types, cargo clippy is clean, and all Minitest smoke tests pass.
