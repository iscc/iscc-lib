# Next Work Package

## Step: Optimize DataHasher::update buffer allocation

## Goal

Eliminate per-call heap allocations in `DataHasher::update` by replacing the `data.to_vec()` /
`[tail, data].concat()` pattern with a persistent internal buffer that is reused across calls. This
addresses the `[normal]` issue "DataHasher::update copies input data on every call" and improves
streaming throughput for large files processed with many small `update()` calls.

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-lib/src/streaming.rs` (DataHasher struct + update method),
    `crates/iscc-lib/benches/benchmarks.rs` (add DataHasher streaming benchmark)
- **Reference**: `crates/iscc-lib/src/cdc.rs` (CDC returns `Vec<&[u8]>` — borrowed slices into
    input), `.claude/context/issues.md` (issue description)

## Not In Scope

- Changing the `alg_cdc_chunks` API or CDC internals — it already returns borrowed slices
- Optimizing `InstanceHasher` — BLAKE3 Hasher already handles streaming efficiently
- Adding benchmark CI jobs — benchmarks remain local-only per current state
- Changing any binding crate code — this is an internal `iscc-lib` optimization with no API change
- Optimizing `gen_data_code_v0` one-shot function — only the streaming `DataHasher` is in scope
- Addressing the `[normal]` FFI video frame allocation issue — separate step with broader API impact

## Implementation Notes

**Current problem** (lines 88–108 in `streaming.rs`):

1. `data.to_vec()` allocates a new Vec even when tail is empty (line 90)
2. `[self.tail.as_slice(), data].concat()` allocates when tail exists (line 92)
3. `prev_chunk.unwrap_or(&b""[..]).to_vec()` allocates a new Vec for the tail every call (line 108)

**Approach — persistent buffer with tail retention:**

Replace the `tail: Vec<u8>` field with a single `buf: Vec<u8>` that persists across calls:

```rust
pub struct DataHasher {
    chunk_features: Vec<u32>,
    buf: Vec<u8>,  // persistent buffer: starts with retained tail, extended with new data
}
```

In `update()`:

1. `self.buf.extend_from_slice(data)` — append new data to buffer (tail already at front)
2. Run CDC: `let chunks = cdc::alg_cdc_chunks(&self.buf, false, cdc::DATA_AVG_CHUNK_SIZE);`
3. Hash all chunks except the last (same `prev_chunk` pattern as current code)
4. Retain only the tail in the buffer:
    - Calculate the byte offset where the last chunk (tail) starts
    - `self.buf.copy_within(tail_start.., 0); self.buf.truncate(tail_len);`
    - This reuses the buffer's existing capacity without allocating

**Borrow checker constraint:** CDC chunks borrow from `self.buf`, so you cannot mutate `self.buf`
while chunks exist. Extract `tail_len` (the length of the last chunk) as a `usize` before dropping
the chunks Vec. Then after the borrow is released, use `let tail_start = self.buf.len() - tail_len;`
to relocate the tail to the front.

Pattern:

```rust
pub fn update(&mut self, data: &[u8]) {
    self.buf.extend_from_slice(data);
    let chunks = cdc::alg_cdc_chunks(&self.buf, false, cdc::DATA_AVG_CHUNK_SIZE);

    let mut prev_chunk: Option<&[u8]> = None;
    for chunk in &chunks {
        if let Some(pc) = prev_chunk {
            self.chunk_features.push(xxhash_rust::xxh32::xxh32(pc, 0));
        }
        prev_chunk = Some(chunk);
    }

    // Extract tail length before dropping borrows
    let tail_len = prev_chunk.map_or(0, |c| c.len());
    drop(chunks);  // release borrow on self.buf

    // Shift tail to front, reusing buffer capacity
    let tail_start = self.buf.len() - tail_len;
    self.buf.copy_within(tail_start.., 0);
    self.buf.truncate(tail_len);
}
```

**`finalize()` adjustment:** Replace `self.tail.is_empty()` with `self.buf.is_empty()` and hash
`&self.buf` instead of `&self.tail`.

**Empty input edge case:** When `data` is empty and `buf` contains only the tail from a previous
call, CDC returns the tail as a single chunk. The `prev_chunk` pattern correctly carries it forward.
No special handling needed — existing tests cover this (byte-at-a-time test).

**Benchmark addition** (in `benchmarks.rs`): Add a `bench_data_hasher_streaming` function:

- Import `DataHasher` from `iscc_lib`
- Create a 1 MB deterministic buffer
- Benchmark feeding it in 64 KiB chunks (16 `update()` calls) then `finalize(64)`
- Set throughput to 1 MB
- Register in the `criterion_group!` macro

## Verification

- `cargo test -p iscc-lib` passes (261 existing tests, including all DataHasher conformance and
    multi-chunk tests)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `grep -c 'to_vec\|\.concat()' crates/iscc-lib/src/streaming.rs` returns 0 (no per-call allocations
    remain in `DataHasher::update` or tail retention)
- `cargo bench -p iscc-lib -- DataHasher` runs without error (new streaming benchmark executes)

## Done When

All verification criteria pass: existing tests confirm correctness, clippy is clean, no `to_vec()`
or `.concat()` allocations remain in the DataHasher code path, and the new streaming benchmark runs
successfully.
