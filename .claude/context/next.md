# Next Work Package

## Step: Implement ISCC codec module with type enums and encoding primitives

## Goal

Create the codec module containing ISCC type enums, varnibble encoding, header encoding/decoding,
base32 encoding/decoding, and `encode_component` — the foundational primitive that all 9 `gen_*_v0`
functions depend on to produce ISCC-encoded output strings.

## Scope

- **Create**: `crates/iscc-lib/src/codec.rs`
- **Modify**: `crates/iscc-lib/src/lib.rs` (add `mod codec;` declaration)
- **Modify**: `crates/iscc-lib/Cargo.toml` and root `Cargo.toml` (add `data-encoding` dependency via
    workspace.dependencies)
- **Reference**:
    - `iscc/iscc-core` codec module via deepwiki (encode_component, encode_header, encode_varnibble,
        decode_varnibble, encode_length, decode_length, encode_base32, decode_base32, decode_header)
    - `iscc/iscc-core` type enums via deepwiki (MT, ST, ST_CC, ST_ISCC, Version)
    - `notes/04-api-compatibility-safety.md` (tiered API — codec is internal, use `pub(crate)`)

## Implementation Notes

### Type Enums (define at top of codec.rs)

Define as `#[repr(u8)]` enums with integer values matching iscc-core:

```
MainType: META=0, SEMANTIC=1, CONTENT=2, DATA=3, INSTANCE=4, ISCC=5, ID=6, FLAKE=7
SubType:  NONE=0 (general), TEXT=0, IMAGE=1, AUDIO=2, VIDEO=3, MIXED=4 (for ST_CC)
          SUM=0, WIDE=1 (for ST_ISCC)
Version:  V0=0
```

Consider using a single `SubType` enum that covers all cases (values 0-5) rather than multiple enums
like Python does. The numeric values are what matter for header encoding.

### Varnibble Encoding (port from iscc-core)

Variable-length 4-bit chunk encoding scheme:

- `0xxx` (1 nibble, 3 data bits) → range 0-7
- `10xxxxxx` (2 nibbles, 6 data bits) → range 8-71
- `110xxxxxxxxx` (3 nibbles, 9 data bits) → range 72-583
- `1110xxxxxxxxxxxx` (4 nibbles, 12 data bits) → range 584-4679

Implement as bit manipulation on `u64` or use a small `BitVec`-like approach. Since values are small
(max 4 nibbles = 16 bits), a simple `u64` accumulator with bit count tracking works well. No
external bitarray dependency needed — use plain bit shifts.

### Header Encoding

`encode_header(mtype, stype, version, length) -> Vec<u8>`:

- Concatenate varnibble-encoded values of all 4 fields into a bit stream
- Pad to byte boundary with zero bits on the right
- Result is 2 bytes minimum (typical case for small enum values), up to 8 bytes max

`decode_header(data: &[u8]) -> (MainType, SubType, Version, u32, &[u8])`:

- Parse 4 varnibble values from byte stream
- Strip 4-bit zero padding if total nibble count is odd
- Return decoded fields plus remaining tail bytes

### Length Encoding

`encode_length(mtype, bit_length) -> u32`:

- For META/SEMANTIC/CONTENT/DATA/INSTANCE/FLAKE: `(bit_length / 32) - 1`
- For ISCC: pass-through (0-7, represents unit composition flags)
- For ID: `(bit_length - 64) / 8`

`decode_length(mtype, length, subtype) -> u32`:

- Inverse of encode_length, returns bit count

### Base32 Encoding

Use the `data-encoding` crate (mature, no-std compatible, fast). It provides RFC4648 base32.

- `encode_base32(data: &[u8]) -> String`: BASE32 encode, strip `=` padding, return uppercase
- `decode_base32(code: &str) -> Vec<u8>`: re-pad with `=` to multiple of 8 chars, decode with case
    folding

Do NOT implement base32hex yet — it's not needed for the core encode_component path. Add it later
when needed.

### encode_component (the key function)

`encode_component(mtype, stype, version, bit_length, digest: &[u8]) -> String`:

1. Call `encode_length(mtype, bit_length)` to get the encoded length value
2. Compute `nbytes = bit_length / 8`
3. Call `encode_header(mtype, stype, version, encoded_length)` to get header bytes
4. Concatenate header bytes + digest truncated to `nbytes`
5. Call `encode_base32(...)` on the concatenated bytes
6. Return the base32 string (without "ISCC:" prefix — callers add that)

### Module Visibility

All functions and types in codec.rs should be `pub(crate)` — this is an internal module, not part of
the Tier 1 API surface. Only `gen_*_v0` functions in lib.rs are public.

### Error Handling

Add an `IsccError::Codec(String)` variant for codec-specific errors (invalid MainType, length out of
range, etc.). Or reuse `IsccError::InvalidInput` which already exists.

## Verification

- `cargo test -p iscc-lib` passes — all existing stub tests still pass, plus new codec tests
- `encode_varnibble` / `decode_varnibble` roundtrip correctly for values 0, 7, 8, 71, 72, 583, 584,
    4679
- `encode_header(0, 0, 0, 1)` produces the expected 2 bytes (META, NONE, V0, length=1)
- `encode_base32` / `decode_base32` roundtrip correctly for arbitrary byte data
- `encode_component(MT::META, 0, 0, 64, &digest)` produces a valid ISCC unit string that matches
    iscc-core output for a known test case (e.g., from data.json example:
    `gen_meta_code_v0("Hello World")` → `"ISCC:AAAWKLHFPV6OPKDG"` — verify the base32 body portion
    matches after stripping "ISCC:" prefix)
- `decode_header` correctly round-trips with `encode_header` for all MainType values
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo fmt -p iscc-lib --check` clean

## Done When

The advance agent is done when `cargo test -p iscc-lib` passes with comprehensive codec unit tests
covering varnibble, header, base32, length encoding, and encode_component, and both clippy and fmt
checks are clean.
