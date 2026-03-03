# Next Work Package

## Step: Add gen_text/image/audio_code_v0 to Ruby bridge

## Goal

Add the first batch of 3 gen functions (`gen_text_code_v0`, `gen_image_code_v0`,
`gen_audio_code_v0`) to the Ruby Magnus bridge, advancing from 16/32 to 19/32 Tier 1 symbols. These
three share the simplest parameter/return patterns among the gen functions and establish the Result
class template for the remaining 6.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-rb/src/lib.rs` — add 3 bridge functions + register in `init()`
    - `crates/iscc-rb/lib/iscc_lib.rb` — add 3 Result classes + 3 Ruby wrapper methods
    - `crates/iscc-rb/test/test_smoke.rb` — add smoke tests for the 3 new functions
- **Reference**:
    - `crates/iscc-py/src/lib.rs` lines 130-169 — PyO3 equivalents (pattern to follow)
    - `crates/iscc-lib/src/types.rs` — return type structs (`TextCodeResult`, `ImageCodeResult`,
        `AudioCodeResult`)
    - `crates/iscc-lib/src/lib.rs` — core function signatures

## Not In Scope

- The remaining 6 gen functions (`gen_video_code_v0`, `gen_mixed_code_v0`, `gen_data_code_v0`,
    `gen_instance_code_v0`, `gen_iscc_code_v0`, `gen_sum_code_v0`) — separate steps
- Algorithm primitives (`sliding_window`, `alg_simhash`, etc.) — later batch
- Streaming types (`DataHasher`, `InstanceHasher`) — later batch
- Conformance tests against `data.json` — separate step after all 32 symbols are exposed
- Standard Ruby linting configuration — separate step
- Ruby CI job or RubyGems release workflow — separate step

## Implementation Notes

### Rust bridge functions (lib.rs)

Follow the exact pattern of existing `gen_meta_code_v0`. Each function:

1. Takes positional parameters matching the core API
2. Calls the `iscc_lib::gen_*_v0` function, mapping errors via `to_magnus_err`
3. Builds an `RHash` with string keys matching the Python dict keys
4. Returns `Result<RHash, Error>`

**`gen_text_code_v0(text: String, bits: u32)`**:

- Core returns `TextCodeResult { iscc: String, characters: usize }`
- Hash keys: `"iscc"`, `"characters"`

**`gen_image_code_v0(pixels: RString, bits: u32)`**:

- Use `RString` for binary data (same pattern as `encode_base64`): `unsafe { pixels.as_slice() }`
- Core returns `ImageCodeResult { iscc: String }`
- Hash key: `"iscc"`

**`gen_audio_code_v0(cv: Vec<i32>, bits: u32)`**:

- Magnus can convert Ruby Array of integers to `Vec<i32>` automatically
- Core returns `AudioCodeResult { iscc: String }`
- Hash key: `"iscc"`

Register all three with `_` prefix in `init()`:

```rust
module.define_module_function("_gen_text_code_v0", function!(gen_text_code_v0, 2))?;
module.define_module_function("_gen_image_code_v0", function!(gen_image_code_v0, 2))?;
module.define_module_function("_gen_audio_code_v0", function!(gen_audio_code_v0, 2))?;
```

Update the module docstring symbol count: "16 of 32" → "19 of 32", and add these three to the symbol
list.

### Ruby wrapper (lib/iscc_lib.rb)

Add three Result subclasses and three wrapper methods:

```ruby
class TextCodeResult < Result; end
class ImageCodeResult < Result; end
class AudioCodeResult < Result; end

def self.gen_text_code_v0(text, bits: 64)
  TextCodeResult[_gen_text_code_v0(text, bits)]
end

def self.gen_image_code_v0(pixels, bits: 64)
  ImageCodeResult[_gen_image_code_v0(pixels, bits)]
end

def self.gen_audio_code_v0(cv, bits: 64)
  AudioCodeResult[_gen_audio_code_v0(cv, bits)]
end
```

### Tests (test_smoke.rb)

Add tests verifying:

- `gen_text_code_v0("Hello World")` returns a Hash with `"iscc"` and `"characters"` keys
- `gen_text_code_v0("Hello World").iscc` returns a non-empty string starting with "ISCC:"
- `gen_text_code_v0("Hello World").characters` returns an integer > 0
- `gen_image_code_v0` with a small pixel buffer (e.g., `"\x00" * 100`) returns Hash with `"iscc"`
- `gen_audio_code_v0` with a short integer array returns Hash with `"iscc"`
- All three result types respond to attribute access (`.iscc`)
- Result types are correct subclass (`TextCodeResult`, `ImageCodeResult`, `AudioCodeResult`)

## Verification

- `cargo check -p iscc-rb` compiles successfully
- `cargo clippy -p iscc-rb -- -D warnings` clean (0 warnings)
- `bundle exec rake compile` builds native extension (run from `crates/iscc-rb/`)
- `bundle exec rake test` passes all tests with 0 failures (run from `crates/iscc-rb/`)
- `bundle exec ruby -e "require 'iscc_lib'; r = IsccLib.gen_text_code_v0('Hello World'); puts r.iscc"`
    prints an ISCC string starting with "ISCC:"

## Done When

All five verification commands pass — the three new gen functions are callable from Ruby with
keyword arguments and typed Result objects.
