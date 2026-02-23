# Next Work Package

## Step: Implement iscc_decompose and decode_units codec functions

## Goal

Add `iscc_decompose` (Tier 1) and its helper `decode_units` to the codec module. This is the 20th of
22 Tier 1 symbols — a pure codec function that splits a composite ISCC-CODE or concatenated ISCC
sequence into individual ISCC-UNIT strings. It depends entirely on existing codec primitives.

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-lib/src/codec.rs` (add `decode_units` + `iscc_decompose` + tests),
    `crates/iscc-lib/src/lib.rs` (add `pub use codec::iscc_decompose;` re-export)
- **Reference**: `reference/iscc-core/iscc_core/codec.py` (lines 193-200: `decode_units`, lines
    376-421: `iscc_decompose`), `reference/iscc-core/iscc_core/constants.py` (lines 216-228: `UNITS`
    table), `crates/iscc-lib/src/codec.rs` (existing `encode_units`, `decode_header`,
    `decode_length`, `encode_component`, `decode_base32`)

## Implementation Notes

1. **`decode_units` helper** — inverse of `encode_units`. Takes a `u32` unit_id (0-7), returns a
    sorted `Vec<MainType>` of optional units present. The mapping is a 3-bit bitfield:

    ```
    0 → []
    1 → [Content]
    2 → [Semantic]
    3 → [Semantic, Content]
    4 → [Meta]
    5 → [Meta, Content]
    6 → [Meta, Semantic]
    7 → [Meta, Semantic, Content]
    ```

    The Python reference uses a `UNITS` lookup table and sorts the result. In Rust, decode the 3 bits
    directly: bit0=Content, bit1=Semantic, bit2=Meta. Append in `MainType` discriminant order
    (Meta=0, Semantic=1, Content=2) so the result is automatically sorted. Signature:
    `pub fn decode_units(unit_id: u32) -> IsccResult<Vec<MainType>>`. Error if `unit_id > 7`.

2. **`iscc_decompose` function** — port from Python `iscc_decompose` (codec.py lines 376-421).
    Signature: `pub fn iscc_decompose(iscc_code: &str) -> IsccResult<Vec<String>>`.

    Algorithm:

    - Strip "ISCC:" prefix if present (use the same `iscc_clean` pattern from `decode_base32` — it
        already strips "ISCC:" prefix)
    - Call `decode_base32(iscc_code)` to get raw bytes
    - Loop while `raw_code` is not empty:
        - `decode_header(raw_code)` → `(mt, st, vs, ln, body)`
        - **If `mt != MainType::Iscc`** (standard unit): decode `ln_bits = decode_length(mt, ln, st)`,
            extract body `[..ln_bits/8]`, `encode_component(mt, st, vs, ln_bits, &body[..ln_bits/8])`,
            push to result, advance `raw_code = body[ln_bits/8..]`, continue
        - **If `mt == MainType::Iscc`** (composite ISCC-CODE):
            - `main_types = decode_units(ln)`
            - **WIDE subtype** (`st == SubType::Wide`): extract Data-Code 128-bit from `body[..16]`,
                Instance-Code 128-bit from `body[16..32]`, push both, break
            - **Standard**: for each `(idx, mtype)` in `main_types`, set `stype = SubType::None` if
                `mtype == MainType::Meta` else `st`, encode 64-bit component from `body[idx*8..]`. Then
                Data-Code 64-bit from `body[body.len()-16..body.len()-8]`, Instance-Code 64-bit from
                `body[body.len()-8..]`. Push all, break

    **Skip `normalize_multiformat`** for now — it handles alternative multibase encodings (base16,
    base58, base64url). Standard ISCC strings are always base32. Can be added later if needed. The
    `decode_base32` function already strips "ISCC:" prefix.

3. **Re-export**: Add `pub use codec::iscc_decompose;` in `lib.rs` alongside existing re-exports.
    `decode_units` stays `pub` in codec module (Tier 2, like other codec functions) but does NOT
    get a flat re-export.

4. **Tests**: Add to `codec.rs` `#[cfg(test)] mod tests`:

    - `decode_units` tests: verify all 8 mappings (0→empty, 1→[Content], ... 7→\[Meta, Semantic,
        Content\]). Verify error for `unit_id > 7`. Verify roundtrip with `encode_units`.
    - `iscc_decompose` roundtrip tests using `gen_iscc_code_v0` conformance vectors:
        - test_0000_standard: compose
            `["AAAYPXW445FTYNJ3", "EAARMJLTQCUWAND2", "GABVVC5DMJJGYKZ4ZBYVNYABFFYXG", "IADWIK7A7JTUAQ2D..."]`
            → get composite ISCC → decompose → verify output contains the original 64-bit units for
            Meta, Semantic, Data, Instance (Content-Code is 256-bit input truncated to 64-bit in the
            composite, so the decomposed Content-Code will be 64-bit)
        - test_0001_no_meta: compose without Meta → decompose → verify 4 units (Semantic, Content,
            Data, Instance — no Meta)
        - Single unit: `iscc_decompose("AAAYPXW445FTYNJ3")` → `["AAAYPXW445FTYNJ3"]` (pass-through)
        - Also verify by generating a known composite with `gen_iscc_code_v0`, decomposing it, and
            checking the number of result units matches expectations

    **NOTE on verification**: There are no explicit `iscc_decompose` test vectors in `data.json`.
    Tests must compose ISCCs first, then decompose and verify structural properties (correct number
    of units, each unit decodes to the expected MainType, Data+Instance always present as last
    two).

5. **Edge cases**:

    - The `decode_length` Rust signature is
        `decode_length(mtype: MainType, length: u32, stype: SubType)` — note it takes stype as 3rd
        param (unlike Python which doesn't)
    - `encode_component` takes `bit_length: u32` and `digest: &[u8]` — it will only consume the
        needed bytes from the digest slice
    - The body slice from `decode_header` already excludes the header bytes, so `body[idx*8..]` works
        directly

## Verification

- `cargo test -p iscc-lib` passes (all 197+ existing tests plus new decompose/decode_units tests)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `decode_units` correctly maps all 8 unit_ids (0-7) and roundtrips with `encode_units`
- `iscc_decompose` on a composite ISCC from `gen_iscc_code_v0` returns the correct number of units
- `iscc_decompose` on a single ISCC-UNIT returns a 1-element vec with the same code
- Each decomposed unit decodes to the expected `MainType` via `decode_header`
- `iscc_decompose` is importable as `iscc_lib::iscc_decompose` (flat re-export)

## Done When

All verification criteria pass — `iscc_decompose` and `decode_units` are implemented with tests
proving correctness against the gen_iscc_code_v0 conformance vectors, and the function is publicly
accessible as Tier 1 API.
