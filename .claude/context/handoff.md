## 2026-02-25 — Optimize DataHasher::update buffer allocation

**Done:** Replaced per-call heap allocations in `DataHasher::update` with a persistent internal
buffer (`buf: Vec<u8>`) that is reused across calls. The `data.to_vec()`, `[tail, data].concat()`,
and `prev_chunk.to_vec()` patterns are eliminated — new data is appended via `extend_from_slice`,
and the tail is shifted to the front with `copy_within` + `truncate`, reusing existing buffer
capacity. Added a `DataHasher` streaming benchmark to `benchmarks.rs`.

**Files changed:**

- `crates/iscc-lib/src/streaming.rs`: Replaced `tail: Vec<u8>` field with `buf: Vec<u8>` in
    `DataHasher`. Rewrote `update()` to use `extend_from_slice` + `copy_within` pattern. Updated
    `finalize()` to reference `self.buf` instead of `self.tail`.
- `crates/iscc-lib/benches/benchmarks.rs`: Added `bench_data_hasher_streaming` function (1 MB data,
    64 KiB chunks, measures throughput). Registered in `criterion_group!` macro.

**Verification:** All 265 tests pass (including all DataHasher conformance, multi-chunk, and
byte-at-a-time tests). Clippy clean. Zero `to_vec()`/`.concat()` allocations in `streaming.rs`.
Benchmark runs at ~1.1 GiB/s throughput. All 14 pre-commit hooks pass.

**Next:** The `[normal]` DataHasher issue is resolved. Consider the `[normal]` iscc-ffi video frame
allocation issue, or one of the `[low]` issues (dct power-of-two validation, wtahash bounds check,
iscc-py `__version__`, etc.).

**Notes:** The borrow checker constraint was handled exactly as described in next.md — extracting
`tail_len` as a `usize` from `prev_chunk` before dropping the `chunks` Vec, then using `copy_within`
to relocate the tail. The `drop(chunks)` is explicit to make the borrow release clear. Test count is
265 (vs 261 mentioned in next.md) — 4 additional tests were added in prior iterations.
