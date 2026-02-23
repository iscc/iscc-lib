# Next Work Package

## Step: Add structured result types for all 9 gen functions

## Goal

Replace the plain `String` returns from all 9 `gen_*_v0` functions with structured result types that
carry additional fields (metahash, name, characters, datahash, filesize, parts). This is the
prerequisite for Python dict returns and iscc-core drop-in compatibility.

## Scope

- **Create**: `crates/iscc-lib/src/types.rs` — result struct definitions
- **Modify**: `crates/iscc-lib/src/lib.rs` — update gen functions to return structs, update tests
- **Reference**: `reference/iscc-core/iscc_core/code_meta.py` (lines 94-105, result dict),
    `reference/iscc-core/iscc_core/code_instance.py` (lines 47-53, datahash/filesize),
    `reference/iscc-core/iscc_core/code_content_text.py` (line 75, characters),
    `reference/iscc-core/iscc_core/code_content_mixed.py` (line 61, parts),
    `.claude/context/specs/python-bindings.md` (complete field listing)

## Implementation Notes

### Result Structs (in `types.rs`)

Define one struct per gen function. All structs derive `Debug, Clone, PartialEq, Eq`. Use
`#[non_exhaustive]` so fields can be added without breaking semver:

```rust
pub struct MetaCodeResult {
    pub iscc: String,
    pub name: String,
    pub description: Option<String>,  // None when description was empty/absent
    pub meta: Option<String>,         // Data-URL when meta was provided
    pub metahash: String,             // hex-encoded "1e20..." BLAKE3 multihash
}

pub struct TextCodeResult {
    pub iscc: String,
    pub characters: usize,            // char count after text_collapse
}

pub struct ImageCodeResult {
    pub iscc: String,
}

pub struct AudioCodeResult {
    pub iscc: String,
}

pub struct VideoCodeResult {
    pub iscc: String,
}

pub struct MixedCodeResult {
    pub iscc: String,
    pub parts: Vec<String>,           // input Content-Code strings (passed through)
}

pub struct DataCodeResult {
    pub iscc: String,
}

pub struct InstanceCodeResult {
    pub iscc: String,
    pub datahash: String,             // hex-encoded "1e20..." BLAKE3 multihash of data
    pub filesize: u64,                // byte length of input data
}

pub struct IsccCodeResult {
    pub iscc: String,
}
```

### Function Changes (in `lib.rs`)

Most functions already compute the extra values but discard them with `_` prefix. Changes:

1. **`gen_meta_code_v0`**: Already computes `_metahash`. Remove underscore, build `MetaCodeResult`
    with `iscc`, `name`, `description` (Some if non-empty after normalization), `meta` (Some if
    meta was provided — pass through the original meta string for Data-URL, or construct Data-URL
    for JSON meta), `metahash`.

    - For the `meta` field in the result: if input `meta` was a Data-URL string, pass it through. If
        it was a JSON string, build a Data-URL `data:application/json;base64,<base64>` from the
        canonical JSON bytes. Match iscc-core `code_meta.py` lines 69-76.
    - Use `data_encoding::BASE64_NOPAD` for the Data-URL base64 encoding (standard base64 no
        padding).

2. **`gen_text_code_v0`**: Already computes `_characters`. Remove underscore, return
    `TextCodeResult`.

3. **`gen_image_code_v0`**: Trivial — wrap `iscc` in `ImageCodeResult`.

4. **`gen_audio_code_v0`**: Trivial — wrap `iscc` in `AudioCodeResult`.

5. **`gen_video_code_v0`**: Trivial — wrap `iscc` in `VideoCodeResult`.

6. **`gen_mixed_code_v0`**: Return `MixedCodeResult` with `parts` from the input codes. Match
    iscc-core behavior: parts = list of input codes as-is (with "ISCC:" prefix if present).

7. **`gen_data_code_v0`**: Trivial — wrap `iscc` in `DataCodeResult`.

8. **`gen_instance_code_v0`**: Add `datahash` (BLAKE3 multihash of input data using
    `utils::multi_hash_blake3`) and `filesize` (data.len() as u64). Return `InstanceCodeResult`.

9. **`gen_iscc_code_v0`**: Trivial — wrap `iscc` in `IsccCodeResult`.

### Test Updates

All tests that currently compare `result` to a string must change to `result.iscc`. Example:

- `assert_eq!(result, "ISCC:...")` → `assert_eq!(result.iscc, "ISCC:...")`

Conformance tests for meta/text/instance should also verify the new fields match the expected values
from `data.json` (they already do this for some fields but by recomputing — now verify from the
struct directly). Specifically:

- Meta conformance: verify `result.metahash`, `result.name`
- Text conformance: verify `result.characters`
- Instance conformance: verify `result.datahash` and `result.filesize` (add `datahash`/`filesize`
    checks if present in `data.json` test vectors)
- Mixed conformance: verify `result.parts`

### Meta Data-URL construction

For the `meta` field when JSON is provided, iscc-core builds a Data-URL from the canonical JSON.
Check whether `data_encoding::BASE64` (with padding) or `BASE64_NOPAD` matches iscc-core. The Python
`python-datauri` library uses standard base64 with padding by default. Use `data_encoding::BASE64`
(with padding).

The media type logic from iscc-core: if the parsed JSON object has an `"@context"` key, use
`application/ld+json`; otherwise use `application/json`. Then build:
`data:<media_type>;base64,<base64_payload>`.

## Verification

- `cargo test -p iscc-lib` passes (all 143+ tests, updated to use `.iscc` field access)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo fmt -p iscc-lib --check` clean
- `gen_meta_code_v0` returns `MetaCodeResult` with correct `metahash`, `name`, optional
    `description` and `meta` fields matching iscc-core conformance vectors
- `gen_text_code_v0` returns `TextCodeResult` with correct `characters` count
- `gen_instance_code_v0` returns `InstanceCodeResult` with correct `datahash` and `filesize`
- `gen_mixed_code_v0` returns `MixedCodeResult` with correct `parts`
- All 9 `gen_*_v0` functions return structured result types (no `String` returns)

## Done When

The advance agent is done when all 9 `gen_*_v0` functions return structured result types with all
fields populated correctly, all existing tests pass (updated to use `.iscc` access), and conformance
tests verify additional fields (metahash, name, characters, parts, datahash, filesize) match
iscc-core output.
