# Next Work Package

## Step: Add gen_video/mixed/data_code_v0 to Ruby bridge

## Goal

Expose `gen_video_code_v0`, `gen_mixed_code_v0`, and `gen_data_code_v0` in the Ruby Magnus bridge,
advancing from 19/32 to 22/32 Tier 1 symbols. These three functions have more complex parameter
types than the previous batch (nested arrays, string arrays, binary data).

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-rb/src/lib.rs` — add 3 bridge functions + register in `init()`
    - `crates/iscc-rb/lib/iscc_lib.rb` — add `VideoCodeResult`, `MixedCodeResult`, `DataCodeResult`
        classes + keyword-arg wrappers
    - `crates/iscc-rb/test/test_smoke.rb` — add smoke tests for the 3 new functions
- **Reference**:
    - `crates/iscc-py/src/lib.rs` — Python bridge patterns for the same 3 functions
    - `crates/iscc-lib/src/types.rs` — result struct fields
    - `crates/iscc-lib/src/lib.rs` — Rust core API signatures

## Not In Scope

- `gen_instance_code_v0`, `gen_iscc_code_v0`, `gen_sum_code_v0` — next batch after this one
- Algorithm primitives (`sliding_window`, `alg_simhash`, etc.) — future step
- Streaming types (`DataHasher`, `InstanceHasher`) — future step
- Conformance tests against `data.json` — separate step after all 32 symbols are exposed
- Ruby CI job, RubyGems release workflow, or documentation — later steps
- Refactoring existing bridge functions or result class hierarchy

## Implementation Notes

### `gen_video_code_v0` — nested array conversion (trickiest)

The Rust core signature is `gen_video_code_v0<S: AsRef<[i32]> + Ord>(frame_sigs: &[S], bits: u32)`.
From Ruby, this comes as `Array<Array<Integer>>`.

Magnus does NOT auto-convert nested arrays of integers. The bridge function must:

1. Accept `RArray` (the outer array)
2. Iterate over elements, converting each to `Vec<i32>` (Magnus can convert inner arrays)
3. Build `Vec<Vec<i32>>` and pass as `&[Vec<i32>]` to the core function

Pattern (similar to Python's `extract_frame_sigs`):

```rust
fn gen_video_code_v0(frame_sigs: RArray, bits: u32) -> Result<RHash, Error> {
    let frames: Vec<Vec<i32>> = frame_sigs
        .each()
        .map(|frame| {
            let arr: Vec<i32> = frame?.try_convert()?;
            Ok(arr)
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let r = iscc_lib::gen_video_code_v0(&frames, bits).map_err(to_magnus_err)?;
    // ...build hash...
}
```

Return type: `VideoCodeResult { iscc: String }` — simple hash with one key.

### `gen_mixed_code_v0` — string array

Signature: `gen_mixed_code_v0(codes: &[&str], bits: u32)`. From Ruby: `Array<String>`.

Magnus auto-converts `Vec<String>`. Then convert to `Vec<&str>` for the core call:

```rust
fn gen_mixed_code_v0(codes: Vec<String>, bits: u32) -> Result<RHash, Error> {
    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
    let r = iscc_lib::gen_mixed_code_v0(&refs, bits).map_err(to_magnus_err)?;
    // ...build hash with iscc + parts (Array of Strings)...
}
```

Return type: `MixedCodeResult { iscc: String, parts: Vec<String> }` — hash with `iscc` key and
`parts` key (Ruby Array of Strings).

### `gen_data_code_v0` — binary data

Signature: `gen_data_code_v0(data: &[u8], bits: u32)`. From Ruby: binary `String`.

Use the same `RString` + `unsafe { data.as_slice() }` pattern as `gen_image_code_v0`:

```rust
fn gen_data_code_v0(data: RString, bits: u32) -> Result<RHash, Error> {
    let bytes = unsafe { data.as_slice() };
    let r = iscc_lib::gen_data_code_v0(bytes, bits).map_err(to_magnus_err)?;
    // ...build hash...
}
```

Return type: `DataCodeResult { iscc: String }` — simple hash with one key.

### Registration pattern

All 3 functions use `_` prefix in the module registration (for Ruby wrapper layer):

```rust
module.define_module_function("_gen_video_code_v0", function!(gen_video_code_v0, 2))?;
module.define_module_function("_gen_mixed_code_v0", function!(gen_mixed_code_v0, 2))?;
module.define_module_function("_gen_data_code_v0", function!(gen_data_code_v0, 2))?;
```

### Ruby wrapper pattern

Follow the existing pattern exactly — `Result < Hash` subclass + `self.` class method with keyword
`bits:` argument:

```ruby
class VideoCodeResult < Result; end
class MixedCodeResult < Result; end
class DataCodeResult < Result; end

def self.gen_video_code_v0(frame_sigs, bits: 64)
  VideoCodeResult[_gen_video_code_v0(frame_sigs, bits)]
end
# ...etc
```

### Update module docstring

Update the symbol count in `src/lib.rs` docstring from "19 of 32" to "22 of 32" and add the 3 new
function names to the list.

### Smoke tests

Add tests for each function following the existing pattern:

- `gen_video_code_v0`: pass a small nested array like `[[1,2,3,4],[5,6,7,8]]`, verify
    `VideoCodeResult` type and `iscc` starts with `"ISCC:"`
- `gen_mixed_code_v0`: pass array of ISCC unit strings (e.g., from `gen_meta_code_v0` and
    `gen_text_code_v0`), verify `MixedCodeResult` type, `iscc` starts with `"ISCC:"`, and `parts` is
    an Array
- `gen_data_code_v0`: pass binary string `("Hello World" * 100).b`, verify `DataCodeResult` type and
    `iscc` starts with `"ISCC:"`
- Test attribute access (`.iscc`, `.parts`) on each result type

## Verification

- `cargo check -p iscc-rb` compiles successfully
- `cargo clippy -p iscc-rb -- -D warnings` clean (0 warnings)
- `bundle exec rake compile` builds native extension (run from `crates/iscc-rb/`)
- `bundle exec rake test` passes with 0 failures, 0 errors (run from `crates/iscc-rb/`)
- `bundle exec ruby -e "require 'iscc_lib'; r = IsccLib.gen_video_code_v0([[1,2,3],[4,5,6]]); puts r.iscc"`
    prints an ISCC string starting with `ISCC:`
- `bundle exec ruby -e "require 'iscc_lib'; r = IsccLib.gen_data_code_v0(('x' * 1000).b); puts r.iscc"`
    prints an ISCC string starting with `ISCC:`

## Done When

All 6 verification criteria pass and the Ruby bridge exposes 22 of 32 Tier 1 symbols.
