# Next Work Package

## Step: Optimize codec header decoding with direct bitwise operations

## Goal

Replace the `Vec<bool>` intermediate allocation in `decode_header` and `decode_varnibble` with
direct bitwise extraction from the byte slice, eliminating the per-call heap allocation that occurs
on every codec operation (decompose, mixed-code hashing, conformance checks).

## Scope

- **Create**: none
- **Modify**: `crates/iscc-lib/src/codec.rs` — rewrite `decode_header` and `decode_varnibble` to
    operate on `&[u8]` + bit position instead of `&[bool]`; remove `bytes_to_bits` and `bits_to_u32`
    if they become unused (they may still be needed by tests or `encode_varnibble` — check before
    removing)
- **Reference**: `crates/iscc-lib/src/codec.rs` (current implementation),
    `crates/iscc-lib/benches/benchmarks.rs` (benchmark setup)

## Not In Scope

- Changing `encode_varnibble` or `encode_header` — encoding still uses `Vec<bool>` and is less
    performance-sensitive (called once per code generation, not per codec parse)
- Changing `bits_to_bytes` — used by encode path, not the decode path being optimized
- Modifying any file outside `codec.rs` — the public signature of `decode_header` must not change
- Adding new benchmarks — the existing `gen_mixed_code_v0` and full conformance suite already
    exercise `decode_header` heavily
- Optimizing `encode_header` in the same step — that's a separate future optimization

## Implementation Notes

**Core approach:** Replace the `decode_varnibble(bits: &[bool])` function with a
`decode_varnibble_from_bytes(data: &[u8], bit_pos: usize)` that reads bits directly from the byte
slice using bitwise operations.

**Bit extraction helper:**

```rust
/// Read bit at position `bit_pos` from byte slice (MSB-first ordering).
fn get_bit(data: &[u8], bit_pos: usize) -> bool {
    let byte_idx = bit_pos / 8;
    let bit_idx = 7 - (bit_pos % 8);
    (data[byte_idx] >> bit_idx) & 1 == 1
}

/// Extract `count` bits starting at `bit_pos` as a u32 (MSB-first).
fn extract_bits(data: &[u8], bit_pos: usize, count: usize) -> u32 {
    let mut value = 0u32;
    for i in 0..count {
        value = (value << 1) | u32::from(get_bit(data, bit_pos + i));
    }
    value
}
```

**Rewritten `decode_varnibble_from_bytes`:** Same prefix-detection logic as current
`decode_varnibble` but using `get_bit(data, bit_pos)` instead of `bits[0]`, and
`extract_bits(data, bit_pos, n)` instead of `bits_to_u32(&bits[..n])`. Return type remains
`IsccResult<(u32, usize)>` (value, bits consumed). Check available bits via
`data.len() * 8 - bit_pos`.

**Rewritten `decode_header`:** Instead of `let bits = bytes_to_bits(data)`, maintain a `bit_pos`
counter and call `decode_varnibble_from_bytes(data, bit_pos)` four times. For the tail extraction,
compute `tail_byte_start = (bit_pos + 7) / 8` (round up to next byte boundary after padding check)
and use `data[tail_byte_start..].to_vec()`.

**Tail/padding logic:** The 4-bit zero-padding check at lines 262-267 becomes: if
`bit_pos % 8 != 0`, verify the next 4 bits are zero using `extract_bits(data, bit_pos, 4) == 0`,
then advance `bit_pos` by 4.

**Dead code cleanup:** After rewriting, check if `bytes_to_bits` and `bits_to_u32` are still
referenced. `bytes_to_bits` is used in one test (`test_bits_to_u32` at line 984 calls
`bytes_to_bits`). If the only remaining caller is in `#[cfg(test)]`, either gate them with
`#[cfg(test)]` or keep them — don't remove test helpers that validate bit-level correctness. The
`decode_varnibble` function on `&[bool]` is used in `test_varnibble_roundtrip` (line 560) — the
roundtrip test can call the new function through `decode_header` or be updated to use
`decode_varnibble_from_bytes` directly.

**Test updates:** Update tests that directly call `decode_varnibble` on `&[bool]` to instead call
the new byte-based function. Add at least 2 new unit tests:

1. `test_extract_bits_basic` — verify `extract_bits` extracts correct values from known byte
    patterns
2. `test_decode_varnibble_from_bytes_boundary_values` — verify decoding at non-zero bit offsets

## Verification

- `cargo test -p iscc-lib` passes (all 259 existing tests, including 71 codec tests)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `grep 'fn bytes_to_bits' crates/iscc-lib/src/codec.rs` outputs nothing OR the function is gated
    with `#[cfg(test)]` (no longer in production code path)
- `grep 'Vec<bool>' crates/iscc-lib/src/codec.rs` — only in `encode_varnibble` and `bits_to_bytes`
    (encode path), NOT in `decode_header` or any decode function
- `cargo bench -p iscc-lib -- gen_mixed_code_v0` runs without error (benchmark still works)

## Done When

All verification criteria pass — `decode_header` operates directly on `&[u8]` with zero intermediate
`Vec<bool>` allocation, all 259+ tests pass, and clippy is clean.
