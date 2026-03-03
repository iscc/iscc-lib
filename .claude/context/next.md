# Next Work Package

## Step: Add Ruby conformance tests against data.json

## Goal

Add conformance tests that verify all 9 gen\_\*\_v0 functions in the Ruby binding produce output
matching the official `data.json` test vectors (50 vectors across 9 function sections). This proves
correctness of the entire Ruby binding surface before adding CI and release infrastructure.

## Scope

- **Create**: `crates/iscc-rb/test/test_conformance.rb`
- **Modify**: (none)
- **Reference**:
    - `tests/test_conformance.py` — Python conformance tests (pattern to mirror)
    - `crates/iscc-rb/test/test_smoke.rb` — existing Ruby test patterns
    - `crates/iscc-rb/test/test_helper.rb` — test setup
    - `crates/iscc-rb/lib/iscc_lib.rb` — Ruby API surface (keyword args, result classes)
    - `crates/iscc-lib/tests/data.json` — conformance vectors (50 total, 9 function sections)

## Not In Scope

- Adding a Ruby CI job to `ci.yml` — that's a separate infrastructure step
- Standard Ruby linting (`standard` gem) — separate step
- RubyGems publish step in `release.yml` — separate step
- `version_sync.py` gemspec update — separate step
- `docs/howto/ruby.md` or README updates — separate step
- Testing `gen_sum_code_v0` — not in data.json (only 9 of 10 gen functions have vectors)
- Streaming type conformance — the existing `test_iscc_lib.rb` already verifies DataHasher and
    InstanceHasher produce results matching one-shot calls

## Implementation Notes

**Pattern**: Mirror `tests/test_conformance.py` structure using Ruby/Minitest idioms.

**File structure**:

```ruby
require "test_helper"
require "json"

class TestConformance < Minitest::Test
  # Load data.json once, generate test methods dynamically
end
```

**Vector loading**: Load `data.json` from `../../iscc-lib/tests/data.json` relative to the test
directory. Use `File.expand_path` for robust path resolution. Parse with `JSON.parse`.

**Dynamic test generation**: Use `define_method("test_gen_xxx_v0_#{vector_name}")` in a class-level
loop over each function section's vectors. This gives individual test names in output (like pytest
parametrize).

**Input decoding helpers** (match Python patterns exactly):

1. `prepare_meta_arg(meta_val)` — handle `nil`, `String`, and `Hash` (dict) inputs. For Hash, use
    `JSON.generate(meta_val)` with sorted keys to produce JCS-compatible JSON string.
2. `decode_stream(stream_str)` — strip `"stream:"` prefix, hex-decode remainder with
    `[hex].pack("H*")`. Return empty binary string (`"".b`) for empty hex.

**Per-function input mapping** (from data.json `inputs` array to Ruby keyword args):

| Function             | inputs[0]                       | inputs[1]               | inputs[2]              | inputs[3]      | Ruby call                                                                  |
| -------------------- | ------------------------------- | ----------------------- | ---------------------- | -------------- | -------------------------------------------------------------------------- |
| gen_meta_code_v0     | name (String)                   | description (String/"") | meta (nil/String/Hash) | bits (Integer) | `gen_meta_code_v0(name, description: desc_or_nil, meta: meta, bits: bits)` |
| gen_text_code_v0     | text (String)                   | bits (Integer)          | —                      | —              | `gen_text_code_v0(text, bits: bits)`                                       |
| gen_image_code_v0    | pixels (Array<int>)             | bits (Integer)          | —                      | —              | `gen_image_code_v0(pixels.pack("C*"), bits: bits)`                         |
| gen_audio_code_v0    | cv (Array<int>)                 | bits (Integer)          | —                      | —              | `gen_audio_code_v0(cv, bits: bits)`                                        |
| gen_video_code_v0    | frame_sigs (Array\<Array<int>>) | bits (Integer)          | —                      | —              | `gen_video_code_v0(frame_sigs, bits: bits)`                                |
| gen_mixed_code_v0    | codes (Array<String>)           | bits (Integer)          | —                      | —              | `gen_mixed_code_v0(codes, bits: bits)`                                     |
| gen_data_code_v0     | stream (String)                 | bits (Integer)          | —                      | —              | `gen_data_code_v0(decode_stream(stream), bits: bits)`                      |
| gen_instance_code_v0 | stream (String)                 | bits (Integer)          | —                      | —              | `gen_instance_code_v0(decode_stream(stream), bits: bits)`                  |
| gen_iscc_code_v0     | codes (Array<String>)           | —                       | —                      | —              | `gen_iscc_code_v0(codes)`                                                  |

**Key edge cases**:

- `gen_meta_code_v0`: empty description string `""` in data.json → pass `nil` to Ruby (matching
    Python's `description or None` pattern)
- `gen_image_code_v0`: pixel array in JSON is `Array<Integer>` (0-255) → pack to binary string with
    `.pack("C*")` (unsigned bytes)
- `gen_iscc_code_v0`: no `bits` parameter, no `wide` parameter in vectors — call with just `codes`
    (default `wide: false`)
- Optional output fields (`description`, `meta` in meta results; `parts` in mixed results) — assert
    presence/absence matches expected outputs

**Output assertions** (per function):

- **meta**: `iscc`, `name`, `metahash` (always); `description`, `meta` (conditional on outputs)
- **text**: `iscc`, `characters`
- **image/audio/video**: `iscc` only
- **mixed**: `iscc`, `parts`
- **data**: `iscc` only
- **instance**: `iscc`, `datahash`, `filesize`
- **iscc**: `iscc` only

**Expected test count**: 50 vectors across 9 functions = 50 dynamically generated test methods, plus
the existing 61 tests = ~111 total tests.

## Verification

- `cd crates/iscc-rb && bundle exec rake test` passes with 0 failures, 0 errors
- Test output shows `test_gen_meta_code_v0_test_0001` style names (dynamically generated)
- Total test count ≥ 100 (existing 61 + ~50 conformance vectors)
- `grep -c "define_method" crates/iscc-rb/test/test_conformance.rb` returns 9 (one per function)
- `mise run check` passes (formatting/linting clean)

## Done When

All verification criteria pass — `bundle exec rake test` runs the conformance test file with all 50
data.json vectors passing alongside existing smoke/streaming tests.
