# Next Work Package

## Step: Implement gen_mixed_code_v0 with SimHash code mixing

## Goal

Implement `gen_mixed_code_v0` — the Mixed Content-Code generator — which combines multiple ISCC
Content-Codes into a single similarity-preserving code using SimHash. This is the 5th of 9 gen
functions and exercises existing codec decode + SimHash infrastructure on a compositional function
(no new internal modules needed).

## Scope

- **Create**: none
- **Modify**: `crates/iscc-lib/src/lib.rs` — implement `soft_hash_codes_v0` helper and
    `gen_mixed_code_v0`, replace stub test with conformance tests
- **Reference**:
    - `iscc/iscc-core` via deepwiki — `code_content_mixed.py` (`gen_mixed_code_v0`,
        `soft_hash_codes_v0`)
    - `crates/iscc-lib/src/codec.rs` — `decode_base32`, `decode_header`, `encode_component`,
        `MainType::Content`, `SubType::MIXED`
    - `crates/iscc-lib/src/simhash.rs` — `alg_simhash` (accepts `&[impl AsRef<[u8]>]`)
    - `crates/iscc-lib/tests/data.json` — conformance vectors under `gen_mixed_code_v0`

## Implementation Notes

### gen_mixed_code_v0 function

```
pub fn gen_mixed_code_v0(codes: &[&str], bits: u32) -> IsccResult<String>:
    1. For each code string:
       - Strip "ISCC:" prefix if present
       - decode_base32(code) → raw bytes
       - Store raw bytes in a Vec
    2. Call soft_hash_codes_v0(&decoded_bytes, bits) → digest
    3. encode_component(MainType::Content, SubType::MIXED, Version::V0, bits, &digest)
    4. Return Ok(format!("ISCC:{component}"))
    5. Return parts in output (for now, just the ISCC string — parts tracking deferred)
```

### soft_hash_codes_v0 — Core mixing algorithm

This is the internal helper that performs the actual code combination:

```
fn soft_hash_codes_v0(cc_digests: &[Vec<u8>], bits: u32) -> IsccResult<Vec<u8>>:
    1. Validate: at least 2 codes required (return InvalidInput otherwise)
    2. For each digest, decode_header to verify MainType::Content
       (reject non-Content codes with InvalidInput error)
    3. nbytes = bits / 8  (e.g., 64 bits → 8 bytes)
    4. Prepare byte sequences for SimHash:
       For each raw_bytes (full decoded ISCC unit):
         - Take raw_bytes[0] (first byte of header — 1 byte)
         - Take body bytes (after header) truncated to nbytes-1 bytes
         - Concatenate: [header_byte_0] + body[:nbytes-1] = nbytes total
    5. Call alg_simhash(&prepared_sequences) → Vec<u8> of length nbytes
    6. Return the SimHash result
```

### Key detail: header byte extraction

The raw bytes from `decode_base32` contain: `[header_bytes...][body_bytes...]`. The header is
variable-length (varnibble encoded). However, the algorithm takes ONLY `raw_bytes[0]` (the very
first byte of the entire encoded unit). This first byte encodes MainType + SubType and preserves
content type information in the mixed hash.

The body starts after the full header. Use `decode_header()` which returns
`(MainType, SubType, Version, bit_length, body)` — the `body` is what follows the header. However,
the prepared bytes need `raw_bytes[0]` (not derived from decoded fields), so keep both the raw bytes
and the decoded body.

Implementation approach:

```rust
let nbytes = (bits / 8) as usize;
let mut prepared: Vec<Vec<u8>> = Vec::with_capacity(cc_digests.len());
for raw in cc_digests {
    let (mtype, _stype, _ver, _blen, body) = codec::decode_header(raw)?;
    if mtype != codec::MainType::Content {
        return Err(InvalidInput("all codes must be Content-Codes"));
    }
    let mut entry = Vec::with_capacity(nbytes);
    entry.push(raw[0]); // first byte of header preserves type info
    let take = std::cmp::min(nbytes - 1, body.len());
    entry.extend_from_slice(&body[..take]);
    // Pad with zeros if body is shorter than nbytes-1
    while entry.len() < nbytes {
        entry.push(0);
    }
    prepared.push(entry);
}
let digest = simhash::alg_simhash(&prepared);
```

### alg_simhash compatibility

`alg_simhash` accepts `&[impl AsRef<[u8]>]` and infers output length from the first element. With
`Vec<Vec<u8>>` where each inner vec is `nbytes` long, it will produce an `nbytes`-length output.
This is correct — the final `encode_component` will use the first `bits/8` bytes of this digest.

### ISCC prefix handling

The conformance test vectors provide input codes WITHOUT the "ISCC:" prefix (e.g.,
"EUA6GIKXN42IQV3S"). But in real usage, codes may have the prefix. Strip it if present:

```rust
let clean = code.strip_prefix("ISCC:").unwrap_or(code);
```

### Conformance vectors (2 test cases)

| Name                    | Input codes (count) | Bits | Expected ISCC                      |
| ----------------------- | ------------------- | ---- | ---------------------------------- |
| test_0000_std_64        | 4 Content-Codes     | 64   | ISCC:EQASNZJ36ZT33AL7              |
| test_0001_128_truncated | 4 Content-Codes     | 128  | ISCC:EQBSBXXOMP6SZ2VX6DXG332JFUX76 |

Test 0000 uses 64-bit codes:
`["EUA6GIKXN42IQV3S", "EIAUKMOUIOYZCKA5", "EQA6JK5IEKO6E732", "EIAU2XRWOT4AKMTZ"]`

Test 0001 uses 128-bit codes:
`["EQCR2VTB6AUI2J6A5AOYMRA2BNPNTBQS2GGNFQ2DUU", "EAC7ULQD5WEKFMNQUZWWYK5NHTATG4OV62AMIUWLYI", "EACQRBYECQSWFDC5JYDLCCJNF72Q4IYOXV3POUHRNI", "EEC453X23MWGUEZQC3SG7UJMY65HQYFQDJMO4CAL5A"]`

### Test structure

Replace the existing `test_gen_mixed_code_v0_stub` with:

1. A conformance test that iterates all vectors from data.json `gen_mixed_code_v0` section
2. Parse inputs[0] as an array of code strings, inputs[1] as bits
3. Compare output ISCC against expected
4. Also verify output `parts` matches input codes
5. Count tested vectors (assert 2 total)

Also add a unit test verifying error on fewer than 2 codes.

## Verification

- `cargo test -p iscc-lib` passes (all existing 92 tests + new mixed code tests, minus removed stub)
- Both `gen_mixed_code_v0` conformance vectors produce matching ISCC codes
- Invalid input (fewer than 2 codes) returns `Err(InvalidInput(...))`
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo fmt -p iscc-lib --check` clean
- No `unsafe` code

## Done When

The advance agent is done when both `gen_mixed_code_v0` conformance vectors pass and all quality
gates (clippy, fmt, existing tests) remain green.
