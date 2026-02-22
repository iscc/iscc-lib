# Next Work Package

## Step: Implement gen_iscc_code_v0 composite code assembly

## Goal

Implement `gen_iscc_code_v0` — the composite ISCC-CODE generator — which combines multiple ISCC unit
codes (Meta, Content, Data, Instance) into a single ISCC-CODE. This is the 6th of 9 gen functions.
It is purely compositional (no new hashing/algorithms), exercises codec decode/encode roundtripping,
and uses only existing infrastructure.

## Scope

- **Create**: none
- **Modify**:
    - `crates/iscc-lib/src/codec.rs` — add `encode_units` helper function
    - `crates/iscc-lib/src/lib.rs` — implement `gen_iscc_code_v0`, replace stub test with conformance
        tests
- **Reference**:
    - `crates/iscc-lib/src/codec.rs` — `decode_base32`, `encode_base32`, `decode_header`,
        `encode_header`, `MainType`, `SubType`, `Version`, `decode_length`
    - `crates/iscc-lib/tests/data.json` — 5 conformance vectors under `gen_iscc_code_v0`
    - `iscc/iscc-core` via deepwiki — `iscc_code.py` for reference algorithm

## Implementation Notes

### encode_units — new codec helper

Add `pub fn encode_units(main_types: &[MainType]) -> IsccResult<u32>` to `codec.rs`. Maps the
**optional** MainTypes (sorted, excluding mandatory Data + Instance) to an index 0–7.

The UNITS lookup table (from iscc-core):

| Index | Optional MainTypes present  |
| ----- | --------------------------- |
| 0     | (none — only Data+Instance) |
| 1     | Content                     |
| 2     | Semantic                    |
| 3     | Semantic, Content           |
| 4     | Meta                        |
| 5     | Meta, Content               |
| 6     | Meta, Semantic              |
| 7     | Meta, Semantic, Content     |

This is a bitfield pattern: bit 0 = Content present, bit 1 = Semantic present, bit 2 = Meta present.
So the implementation is:

```rust
pub fn encode_units(main_types: &[MainType]) -> IsccResult<u32> {
    let mut result = 0u32;
    for &mt in main_types {
        match mt {
            MainType::Content => result |= 1,
            MainType::Semantic => result |= 2,
            MainType::Meta => result |= 4,
            _ => return Err(IsccError::InvalidInput(
                format!("{mt:?} is not a valid optional unit type")
            )),
        }
    }
    Ok(result)
}
```

Add a corresponding `decode_units` if needed, but it's not required for this step.

Also add unit tests for `encode_units` covering all 8 combinations.

### gen_iscc_code_v0 algorithm

```
pub fn gen_iscc_code_v0(codes: &[&str], wide: bool) -> IsccResult<String>:
    1. Clean inputs: strip "ISCC:" prefix from each code
    2. Validate: at least 2 codes required
    3. Validate: every code must be >= 16 base32 characters (64-bit minimum)
    4. Decode each code: decode_base32(clean) → raw, then decode_header(raw) →
       (MainType, SubType, Version, length, tail)
    5. Sort decoded units by MainType (ascending: Meta=0, Semantic=1, Content=2, Data=3, Instance=4)
    6. Extract main_types tuple from sorted order
    7. Validate: last two must be (Data, Instance) — these are mandatory
    8. Determine wide composite:
       - wide param is true AND exactly 2 codes AND main_types == (Data, Instance)
         AND both units have decode_length >= 128 bits
    9. Determine SubType:
       - If wide composite → SubType::Wide (7)
       - Else collect SubTypes of all Semantic/Content units:
         - If any exist and all same → use that SubType
         - If multiple different → error
         - If none and exactly 2 codes → SubType::Sum (5)
         - If none and 3+ codes → SubType::IsccNone (6)
   10. Get optional MainTypes = main_types[..len-2] (everything except last two Data+Instance)
   11. encoded_length = encode_units(optional_main_types)
   12. Build digest body:
       - Wide: first 16 bytes of each unit's tail (body), concatenated
       - Standard: first 8 bytes of each unit's tail (body), concatenated
   13. header = encode_header(MainType::Iscc, st, Version::V0, encoded_length)
   14. code = encode_base32(header + digest)
   15. Return Ok(format!("ISCC:{code}"))
```

### Conformance vectors (5 test cases)

| Test                          | Input codes                                                             | Expected ISCC                                                  |
| ----------------------------- | ----------------------------------------------------------------------- | -------------------------------------------------------------- |
| test_0000_standard            | 4 codes: Meta + Content-Text + Data + Instance                          | `ISCC:KACYPXW445FTYNJ3CYSXHAFJMA2HUWULUNRFE3BLHRSCXYH2M5AEGQY` |
| test_0001_no_meta             | 3 codes: Content-Text + Data + Instance                                 | `ISCC:KAARMJLTQCUWAND2LKF2GYSSNQVTYZBL4D5GOQCDIM`              |
| test_0002_no_meta_content_256 | 2 codes: Data(256-bit) + Instance(256-bit) → Content present in header? | `ISCC:KUAFVC5DMJJGYKZ4MQV6B6THIBBUG`                           |
| test_0003_no_meta_content_128 | 2 codes: Content(64-bit) + Instance(256-bit)                            | `ISCC:KUAAQICFKJYKY4KUMQV6B6THIBBUG`                           |
| test_0004_ordering            | Same as test_0000 but scrambled order                                   | Same output as test_0000                                       |

Note: test_0004 verifies that sorting by MainType normalizes input order.

Input codes in the test vectors do NOT have "ISCC:" prefix. The test should parse `inputs[0]` as an
array of code strings. There is no `bits` or `wide` parameter in the test vectors — use `wide=false`
as default.

### Edge cases

- Fewer than 2 codes → `Err(InvalidInput)`
- Codes shorter than 16 base32 chars → `Err(InvalidInput)`
- Missing Data or Instance → `Err(InvalidInput)`
- Mixed SubTypes among Content/Semantic units → `Err(InvalidInput)`
- Duplicate MainTypes should still work (sort puts them adjacent)

### Test structure

Replace `test_gen_iscc_code_v0_stub` with:

1. A conformance test iterating all 5 vectors from data.json `gen_iscc_code_v0` section
2. Parse `inputs[0]` as array of code strings; no other input parameters (wide defaults to false)
3. Compare output against `outputs.iscc`
4. Assert 5 vectors tested
5. Add error case tests: fewer than 2 codes, missing Data/Instance

## Verification

- `cargo test -p iscc-lib` passes (all existing 93 tests + new iscc_code tests, minus removed stub)
- All 5 `gen_iscc_code_v0` conformance vectors produce matching ISCC codes
- Invalid inputs (fewer than 2 codes, missing mandatory units) return appropriate errors
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo fmt -p iscc-lib --check` clean
- No `unsafe` code

## Done When

The advance agent is done when all 5 `gen_iscc_code_v0` conformance vectors pass and all quality
gates (clippy, fmt, existing tests) remain green.
