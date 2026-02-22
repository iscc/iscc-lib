# Next Work Package

## Step: Implement gen_image_code_v0 with DCT-based perceptual hash

## Goal

Implement the Image-Code generator, which uses a 2D Discrete Cosine Transform (DCT) and median-based
comparison to produce a perceptual hash from grayscale pixel data. This is one of the two remaining
stub functions (8/9 gen functions complete after this step).

## Scope

- **Create**: `crates/iscc-lib/src/dct.rs` — DCT module with `alg_dct` function
- **Modify**: `crates/iscc-lib/src/lib.rs` — add `pub(crate) mod dct`, implement
    `soft_hash_image_v0`, replace `gen_image_code_v0` stub, update stub test to conformance tests
- **Reference**: `iscc/iscc-core` `iscc_core/dct.py` and `iscc_core/code_content_image.py` (via
    deepwiki), conformance vectors in `tests/data.json` under `gen_image_code_v0`

## Implementation Notes

### DCT Module (`dct.rs`)

Implement `pub(crate) fn alg_dct(data: &[i64]) -> Vec<i64>` — standard Type-II DCT:

```
fn alg_dct(data: &[i64]) -> Vec<i64>:
    n = data.len()
    if n == 0: return error
    result = vec![0.0f64; n]
    for k in 0..n:
        sum = 0.0f64
        for i in 0..n:
            sum += data[i] as f64 * (PI * k as f64 / (2.0 * n as f64) * (2.0 * i as f64 + 1.0)).cos()
        if k == 0:
            result[k] = sum * (1.0 / (n as f64).sqrt())
        else:
            result[k] = sum * (2.0 / n as f64).sqrt()
    return result.iter().map(|x| *x as i64).collect()  // truncate toward zero
```

Key details:

- Input/output are `i64` integers (Python reference uses `int()` which truncates toward zero)
- Uses `std::f64::consts::PI`, `f64::cos()`, `f64::sqrt()`
- Empty input → error
- Unit tests: `alg_dct(&[0;64]) == [0;64]`, `alg_dct(&[1;64]) == [64, 0, ..., 0]`,
    `alg_dct(&(0..64).collect())[0] == 2016`

### soft_hash_image_v0 (in lib.rs)

```
fn soft_hash_image_v0(pixels: &[u8], bits: u32) -> IsccResult<Vec<u8>>:
    assert pixels.len() == 1024
    assert bits <= 256

    // Step 1: Row-wise DCT (32 rows of 32 pixels)
    let rows: Vec<Vec<i64>> = pixels.chunks(32)
        .map(|row| alg_dct(&row.iter().map(|&p| p as i64).collect::<Vec<_>>()))
        .collect()

    // Step 2: Transpose
    let transposed = transpose_matrix(&rows)  // 32x32

    // Step 3: Column-wise DCT
    let dct_cols: Vec<Vec<i64>> = transposed.iter()
        .map(|col| alg_dct(col))
        .collect()

    // Step 4: Transpose back → dct_matrix
    let dct_matrix = transpose_matrix(&dct_cols)

    // Step 5: Extract 8x8 slices at positions (0,0), (1,0), (0,1), (1,1)
    // Position (col, row) means start at (col*8, row*8) in the matrix
    let positions = [(0,0), (1,0), (0,1), (1,1)]
    let mut bitstring = Vec::<bool>::new()

    for (col, row) in positions:
        let flat = flatten_8x8(&dct_matrix, col, row)  // 64 values
        let median = compute_median(&flat)  // f64 median
        for val in flat:
            bitstring.push(val as f64 > median)
        if bitstring.len() >= bits as usize:
            break

    // Step 6: Convert first `bits` bools to bytes
    bits_to_bytes(&bitstring[..bits as usize])
```

**flatten helper**: `flatten_8x8(matrix, col, row)` extracts `matrix[row*8+r][col*8+c]` for r in
0..8, c in 0..8, yielding 64 values in row-major order.

**median**: For 64 values (even count), Python `statistics.median` returns average of the two middle
values: `(sorted[31] + sorted[32]) / 2.0`. Compare with strict `>` (not `>=`).

### gen_image_code_v0

Replace the stub:

```rust
pub fn gen_image_code_v0(pixels: &[u8], bits: u32) -> IsccResult<String> {
    let hash_digest = soft_hash_image_v0(pixels, bits)?;
    let component = codec::encode_component(
        codec::MainType::Content,
        codec::SubType::Image,
        codec::Version::V0,
        bits,
        &hash_digest,
    )?;
    Ok(format!("ISCC:{component}"))
}
```

### Conformance Test Vectors (3 tests)

| Test      | Pixels                          | Bits | Expected ISCC                                                  |
| --------- | ------------------------------- | ---- | -------------------------------------------------------------- |
| test_0000 | 1024 × 0 (all black)            | 64   | `ISCC:EEAQAAAAAAAAAAAA`                                        |
| test_0001 | 1024 × 255 (all white)          | 128  | `ISCC:EEBYAAAAAAAAAAAAAAAAAAAAAAAAA`                           |
| test_0003 | 1024 varied pixels (real image) | 256  | `ISCC:EED4GQZQTY6J5DTHQ2DWCPDZHQOM6QZQTY6J5DTFZ2DWCPDZHQOMXDI` |

The test data format: `inputs` is `[pixel_array, bits]` where pixel_array is a JSON array of
integers (0-255). Parse input values as `u8`.

### Edge Cases

- All-zero pixels: DCT of uniform values → only DC component nonzero, all-zero 8×8 slice hash
- All-255 pixels: Same as all-zero — uniform input → same behavior, but DC component is 255×scale
- The median comparison uses `>` (strictly greater), so exactly-median values map to `0` bit

## Verification

- `cargo test -p iscc-lib` passes all existing 117 tests plus new image conformance + DCT unit tests
- All 3 `gen_image_code_v0` conformance vectors produce matching ISCC output
- `alg_dct` unit tests pass: empty→error, `[0;64]→[0;64]`, `[1;64]→[64,0,..]`, `(0..64)[0] == 2016`
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo fmt -p iscc-lib --check` clean
- No `unsafe` code

## Done When

All 3 image conformance vectors pass and the total test count increases beyond 117, with clippy and
fmt clean.
