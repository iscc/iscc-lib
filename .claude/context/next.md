# Next Work Package

## Step: Implement CDC module and gen_data_code_v0

## Goal

Implement `gen_data_code_v0` — the Data-Code generator that uses content-defined chunking (CDC) plus
MinHash to create a similarity-preserving fingerprint of binary data. This completes 7 of 9 gen
functions and is the most complex remaining stub.

## Scope

- **Create**: `crates/iscc-lib/src/cdc.rs` — CDC module with gear table, parameter calculation,
    offset finding, and chunk splitting
- **Modify**: `crates/iscc-lib/src/lib.rs` — register `cdc` module, implement `gen_data_code_v0`
    replacing the stub
- **Reference**:
    - `iscc/iscc-core` via deepwiki — `alg_cdc_offset`, `alg_cdc_params`, `alg_cdc_chunks`,
        `DataHasherV0`, gear table (in `iscc_core/cdc.py` and `iscc_core/code_data.py`)
    - `bio-codes/iscc-sum` via deepwiki — Rust CDC implementation in `src/cdc.rs`
    - `crates/iscc-lib/src/minhash.rs` — existing `alg_minhash_256` to reuse
    - `crates/iscc-lib/tests/data.json` — 4 conformance vectors for `gen_data_code_v0`

## Implementation Notes

### CDC Module (`src/cdc.rs`)

Port the CDC algorithm from `iscc-core` Python reference. Mark all items `pub(crate)` except tests.

1. **`CDC_GEAR` constant** — `[u32; 256]` array. Copy exactly from iscc-core's `cdc_gear` option
    (values start: 1553318008, 574654857, 759734804, ... end: 854125182). This is a fixed lookup
    table for the gear rolling hash.

2. **`DATA_AVG_CHUNK_SIZE: u32 = 1024`** — default from iscc-core `core_opts.data_avg_chunk_size`.

3. **`alg_cdc_params(avg_size: u32) -> (usize, usize, usize, u32, u32)`** — calculate CDC parameters
    from average chunk size:

    - `min_size = avg_size / 4`
    - `max_size = avg_size * 8`
    - `offset = min_size + ceil_div(min_size, 2)` where `ceil_div(x,y) = (x+y-1)/y`
    - `center_size = avg_size - offset`
    - `bits = (avg_size as f64).log2().round() as u32`
    - `mask_s = (1u32 << (bits + 1)) - 1`
    - `mask_l = (1u32 << (bits - 1)) - 1`
    - Returns `(min_size, max_size, center_size, mask_s, mask_l)`

4. **`alg_cdc_offset(buffer: &[u8], mi: usize, ma: usize, cs: usize, mask_s: u32, mask_l: u32) -> usize`**
    — find the CDC cut point within a buffer:

    ```
    pattern = 0u32
    i = min(mi, size)
    barrier = min(cs, size)
    while i < barrier:
        pattern = (pattern >> 1) + CDC_GEAR[buffer[i]]  // wrapping_add
        if pattern & mask_s == 0: return i + 1
        i += 1
    barrier = min(ma, size)
    while i < barrier:
        pattern = (pattern >> 1) + CDC_GEAR[buffer[i]]  // wrapping_add
        if pattern & mask_l == 0: return i + 1
        i += 1
    return i
    ```

    Use `wrapping_add` for the gear hash addition (u32 overflow). Use `u32` for `pattern`.

5. **`alg_cdc_chunks(data: &[u8], utf32: bool, avg_chunk_size: u32) -> Vec<&[u8]>`** — split data
    into content-defined chunks:

    - If data is empty, return `vec![&data[0..0]]` (one empty slice)
    - Call `alg_cdc_params(avg_chunk_size)` for parameters
    - Loop: find cut_point via `alg_cdc_offset` on remaining buffer, collect
        `&data[start..start+cut_point]`, advance by cut_point
    - If `utf32`, align cut points to 4-byte boundaries by subtracting `cut_point % 4`
    - Continue until all data consumed

### gen_data_code_v0 (`src/lib.rs`)

Non-streaming implementation (takes all data at once):

```
pub fn gen_data_code_v0(data: &[u8], bits: u32) -> IsccResult<String>:
    1. chunks = cdc::alg_cdc_chunks(data, false, cdc::DATA_AVG_CHUNK_SIZE)
    2. features: Vec<u32> = chunks.iter()
           .map(|chunk| xxhash_rust::xxh32::xxh32(chunk, 0))
           .collect()
    3. If features is empty, push xxh32(b"", 0) as fallback
       (Note: alg_cdc_chunks always returns at least one chunk, so this is defensive)
    4. digest = minhash::alg_minhash_256(&features)
    5. component = codec::encode_component(MainType::Data, SubType::None, Version::V0, bits, &digest)
    6. Return Ok(format!("ISCC:{component}"))
```

### Edge Cases

- **Empty data** (`stream:` in tests): CDC yields one empty chunk → xxh32 of empty bytes → one
    feature → minhash → encode. Conformance vector `test_0001_empty_64` expects
    `ISCC:GAASL4F2WZY7KBXB`.
- **Small data** (< min_size = 256 bytes): CDC returns the entire buffer as one chunk since
    `alg_cdc_offset` will return `min(mi, size)` = `size` immediately.
- **Large data** (test_0003): 8192 bytes with 256-bit output — exercises multiple CDC chunks.

### Conformance Vectors (4 tests)

| Test      | Input            | Bits | Expected                                           |
| --------- | ---------------- | ---- | -------------------------------------------------- |
| test_0000 | `ff00` (2 bytes) | 64   | `ISCC:GAAXL2XYM5BQIAZ3`                            |
| test_0001 | empty            | 64   | `ISCC:GAASL4F2WZY7KBXB`                            |
| test_0002 | `00` (1 byte)    | 128  | `ISCC:GABXOD4P2IS6YHS2XOK6IBVPVXPPG`               |
| test_0003 | 8192 bytes       | 256  | `ISCC:GAD2FL7K437RJZ...` (full value in data.json) |

### Test Structure

In `src/cdc.rs`:

- Unit tests for `alg_cdc_params(1024)` verifying exact values of min/max/center/masks
- Unit tests for `alg_cdc_offset` with known buffer
- Unit test for `alg_cdc_chunks` verifying chunk count/sizes for known data

In `src/lib.rs`:

- Replace `test_gen_data_code_v0_stub` with conformance test iterating all 4 vectors
- Parse `"stream:"` prefix + hex decode (same pattern as gen_instance_code_v0 tests)
- Assert 4 vectors tested

## Verification

- `cargo test -p iscc-lib` passes with all 4 `gen_data_code_v0` conformance vectors (no skips)
- Total test count increases from 107 (stub test replaced by conformance + CDC unit tests)
- `cargo clippy -p iscc-lib -- -D warnings` is clean
- `cargo fmt -p iscc-lib --check` is clean
- No `unsafe` code
- CDC module has its own unit tests for `alg_cdc_params`, `alg_cdc_offset`, `alg_cdc_chunks`

## Done When

All 4 `gen_data_code_v0` conformance vectors pass, CDC module has unit tests, and all quality gates
(clippy, fmt) remain clean.
