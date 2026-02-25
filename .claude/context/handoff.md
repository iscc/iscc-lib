## 2026-02-25 — Review of: Optimize DataHasher::update buffer allocation

**Verdict:** PASS

**Summary:** Replaced per-call heap allocations in `DataHasher::update` with a persistent
`buf: Vec<u8>` that is reused across calls. The `data.to_vec()`, `[tail, data].concat()`, and
`prev_chunk.to_vec()` patterns are eliminated — new data is appended via `extend_from_slice`, and
the tail is shifted to the front with `copy_within` + `truncate`. A Criterion streaming benchmark
was added. All 261 tests pass, clippy clean, 14 pre-commit hooks pass, benchmark runs at ~1.0 GiB/s.

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 261 tests (208 + 31 + 22), 0 failures
- [x] `cargo clippy -p iscc-lib -- -D warnings` clean
- [x] `grep -c 'to_vec\|\.concat()' crates/iscc-lib/src/streaming.rs` returns 0 — no per-call
    allocations remain
- [x] `cargo bench -p iscc-lib -- DataHasher` runs successfully — ~1.0 GiB/s throughput

**Issues found:**

- (none)

**Next:** The `[normal]` DataHasher issue is resolved. Consider the `[normal]` iscc-ffi video frame
allocation issue (changing `iscc_lib` video API to accept `&[&[i32]]`), or one of the `[low]` issues
(dct power-of-two validation, wtahash bounds check, iscc-py `__version__`, etc.).

**Notes:** The borrow checker pattern (extract `tail_len` as `usize` before `drop(chunks)`) is clean
and well-documented with inline comments. The `drop(chunks)` is explicit to make the borrow release
obvious — good practice for code that manipulates borrowed data and the owning buffer in sequence.
