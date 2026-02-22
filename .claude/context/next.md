# Next Work Package

## Step: Implement gen_audio_code_v0 with multi-stage SimHash

## Goal

Implement `gen_audio_code_v0` — the Audio Content-Code generator — which applies a multi-stage
SimHash algorithm to Chromaprint feature vectors. This is the fourth gen function (4/9) and
exercises the existing `simhash` module with a different input type (4-byte digests instead of
32-byte BLAKE3 hashes).

## Scope

- **Create**: none
- **Modify**: `crates/iscc-lib/src/lib.rs` — implement `soft_hash_audio_v0` helper and
    `gen_audio_code_v0`, fix signature from `&[u32]` to `&[i32]`, add conformance tests
- **Reference**:
    - `iscc/iscc-core` via deepwiki — `code_content_audio.py` (`gen_audio_code_v0`,
        `soft_hash_audio_v0`)
    - `crates/iscc-lib/src/simhash.rs` — reuse `alg_simhash` (already generic over byte length)
    - `crates/iscc-lib/src/codec.rs` — `SubType::Audio` (value 2), `encode_component`
    - `crates/iscc-lib/tests/data.json` — conformance vectors under `gen_audio_code_v0`

## Implementation Notes

### CRITICAL: Signature must change from `&[u32]` to `&[i32]`

The current stub uses `&[u32]` but conformance test vectors include negative values (e.g.,
`[-1, 0, 1]`). Chromaprint produces signed 32-bit integers. The Python reference uses
`int.to_bytes(4, 'big', signed=True)`. Change the parameter type to `&[i32]` and update the stub
test accordingly.

### soft_hash_audio_v0 — Multi-stage SimHash algorithm

The algorithm builds a hash digest in stages, concatenating 4-byte SimHash chunks:

**Stage 1 — Convert inputs to 4-byte digests:**

```
For each i32 in cv: convert to 4-byte big-endian bytes → Vec<[u8; 4]>
If cv is empty: return vec![0u8; 32]
```

**Stage 2 — Overall SimHash (4 bytes):**

```
parts = vec![alg_simhash(&all_digests)]   // 4-byte SimHash of all digests
```

**Stage 3 — Quarter-based SimHashes (4 × 4 = 16 bytes):**

```
Divide digests into 4 equal-length quarters (integer division, remainder to last)
For each quarter:
    if empty → push [0u8; 4]
    else → push alg_simhash(&quarter)
```

**Stage 4 — Sorted-third-based SimHashes (3 × 4 = 12 bytes):**

```
Sort the ORIGINAL i32 values by numeric value
Convert sorted values to big-endian 4-byte digests
Divide into 3 equal-length thirds (integer division, remainder to last)
For each third:
    if empty → push [0u8; 4]
    else → push alg_simhash(&third)
```

**Concatenate all parts:** stages 2+3+4 = 4 + 16 + 12 = 32 bytes max. The function returns all 32
bytes. The `bits` parameter in `gen_audio_code_v0` controls how many bytes are used via
`encode_component` (which truncates the digest to `bits / 8` bytes).

### Partitioning helper

Python uses `numpy.array_split` which distributes remainder across first chunks. Implement a small
helper `array_split(slice, n) -> Vec<&[T]>` that divides a slice into `n` parts where:

- Each part has `len / n` elements
- The first `len % n` parts get one extra element
- If `n > len`, excess parts are empty slices

This can be a local function in `lib.rs` or a closure inside `soft_hash_audio_v0`.

### Key details

- `alg_simhash` already accepts `&[impl AsRef<[u8]>]` so it works with 4-byte digests. The output
    length matches input digest length (4 bytes in, 4 bytes out).
- **Empty input edge case:** `alg_simhash(&[])` returns `vec![0u8; 32]` (hardcoded), but for audio
    the empty case is handled before calling simhash (return 32 zero bytes directly). Individual
    empty quarters/thirds push `[0u8; 4]` directly instead of calling `alg_simhash`.
- **Sorting:** Sort the original `i32` values, not the byte representations. Then convert sorted
    values to big-endian bytes for SimHash.
- Big-endian encoding of signed i32: Rust `i32::to_be_bytes()` produces the same bytes as Python's
    `int.to_bytes(4, 'big', signed=True)`.

### gen_audio_code_v0 function

```
pub fn gen_audio_code_v0(cv: &[i32], bits: u32) -> IsccResult<String>:
    hash_digest = soft_hash_audio_v0(cv)
    component = encode_component(
        MainType::Content, SubType::Audio, Version::V0, bits, &hash_digest
    )
    Ok(format!("ISCC:{component}"))
```

### Conformance vectors (5 test cases)

| Name                   | Input CV            | Bits | Expected ISCC                                                |
| ---------------------- | ------------------- | ---- | ------------------------------------------------------------ |
| test_0000_empty_64     | []                  | 64   | ISCC:EIAQAAAAAAAAAAAA                                        |
| test_0001_one_128      | [1]                 | 128  | ISCC:EIBQAAAAAEAAAAABAAAAAAAAAAAAA                           |
| test_0002_two_256      | [1, 2]              | 256  | ISCC:EIDQAAAAAMAAAAABAAAAAAQAAAAAAAAAAAAAAAAAAEAAAAACAAAAAAA |
| test_0003_test_neg_256 | [-1, 0, 1]          | 256  | ISCC:EIDQAAAAAH777777AAAAAAAAAAAACAAAAAAP777774AAAAAAAAAAAAI |
| test_0004_cv_256       | 112-element real CV | 256  | ISCC:EIDWUJFCEZZOJYVDHJHIRB3KQSQCM2REUITDUTVAQNRGJIRENCCCULY |

The test vectors use JSON arrays of integers directly (no "stream:" prefix). Parse as
`serde_json::Value` and extract `as_i64() as i32` for each element.

### Learning to correct

The learnings file says `gen_audio_code_v0` takes `&[u32]`. It should take `&[i32]` because
Chromaprint features are signed integers and conformance vectors include negative values.

## Verification

- `cargo test -p iscc-lib` passes (all existing 85 tests + new audio code tests)
- All 5 `gen_audio_code_v0` conformance vectors produce matching ISCC codes
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo fmt -p iscc-lib --check` clean
- No `unsafe` code
- The `gen_audio_code_v0` stub test is replaced/updated for the new `&[i32]` signature

## Done When

The advance agent is done when all 5 `gen_audio_code_v0` conformance vectors pass and all quality
gates (clippy, fmt, existing tests) remain green.
