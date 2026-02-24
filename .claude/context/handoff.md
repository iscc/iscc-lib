## 2026-02-24 — Optimize sliding_window with zero-copy string slices

**Done:** Added `sliding_window_strs` function to `simhash.rs` that returns `Vec<&str>` slices
borrowing from the input string instead of allocating `Vec<String>`. Updated all 3 internal call
sites in `lib.rs` (`meta_name_simhash`, `soft_hash_meta_v0`, `soft_hash_text_v0`) to use the new
zero-copy variant. The public `sliding_window` function is unchanged for API compatibility.

**Files changed:**

- `crates/iscc-lib/src/simhash.rs`: Added `pub(crate) fn sliding_window_strs` with
    `char_indices()`-based byte offset slicing, empty string edge case handling, and 7 new tests
    including a comprehensive equivalence test against `sliding_window`
- `crates/iscc-lib/src/lib.rs`: Changed 3 call sites from `simhash::sliding_window` to
    `simhash::sliding_window_strs` (drop-in replacement since `&str` and `String` both have
    `.as_bytes()`)

**Verification:** All 205 unit tests pass (`cargo test -p iscc-lib`), all 28 integration tests pass,
clippy clean (`-p iscc-lib` and `--workspace --all-targets`), all 14 pre-commit hooks pass.
Benchmark shows **33% improvement** for `gen_text_code_v0/1000_chars` (from ~150µs to ~100µs,
`change: [-35.7% -33.4% -31.1%]`). All 9 conformance vector sections pass unchanged.

**Next:** The `sliding_window` allocates O(n) Strings issue in `issues.md` can now be resolved. The
next highest-value items are the remaining performance issues (`DataHasher::update` copies, `codec`
`Vec<bool>`) or the correctness issues (`alg_simhash` panics on mismatched digests, `sliding_window`
panics on `width < 2`).

**Notes:** The `sliding_window_strs` function uses `char_indices()` to collect byte offsets, then
slices into the original `&str` — no per-n-gram `String` allocation. The empty string edge case
requires a special early return because `char_indices` produces an empty vec, making index access
impossible. The approach exactly matches the implementation sketch from `next.md`. The 33%
improvement is consistent with eliminating O(n) heap allocations in the hot path.
