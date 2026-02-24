# Next Work Package

## Step: Optimize sliding_window with zero-copy string slices

## Goal

Add a zero-allocation internal sliding window function that yields `&str` slices into the source
string, eliminating O(n) `String` heap allocations in `gen_text_code_v0` and `gen_meta_code_v0` —
two of the most frequently called ISCC functions. This addresses the tracked issue "`sliding_window`
allocates O(n) Strings for n-gram generation."

## Scope

- **Create**: none
- **Modify**: `crates/iscc-lib/src/simhash.rs`, `crates/iscc-lib/src/lib.rs`
- **Reference**: `crates/iscc-lib/src/simhash.rs` (current `sliding_window` and
    `sliding_window_bytes` implementations), `crates/iscc-lib/src/lib.rs` (call sites at lines 62,
    81, 270), `crates/iscc-lib/benches/benchmarks.rs` (criterion benchmarks)

## Not In Scope

- Changing the public `sliding_window` function signature or return type — it stays `Vec<String>`
    for API compatibility
- Changing `sliding_window_bytes` — it already returns `Vec<&[u8]>` (zero-copy slices), no
    optimization needed
- Converting `sliding_window` or `alg_simhash` to return `Result` — that's a separate correctness
    issue requiring cross-crate API changes
- Optimizing `alg_simhash`, `DataHasher::update`, or codec `Vec<bool>` — separate tracked issues
- Updating any binding crates — this is a pure internal optimization

## Implementation Notes

**Current pattern** (3 call sites in `lib.rs`):

```rust
let ngrams = simhash::sliding_window(text, width);  // Vec<String> — O(n) allocations
let hashes: Vec<_> = ngrams.iter()
    .map(|ng| hash_fn(ng.as_bytes()))
    .collect();
```

**Optimized approach**: Add a `pub(crate)` function in `simhash.rs` that returns `&str` slices
borrowing from the input string:

```rust
pub(crate) fn sliding_window_strs<'a>(seq: &'a str, width: usize) -> Vec<&'a str> {
    assert!(width >= 2, "Sliding window width must be 2 or bigger.");
    let char_indices: Vec<usize> = seq.char_indices().map(|(i, _)| i).collect();
    let len = char_indices.len();
    let range = cmp::max(len.saturating_sub(width).saturating_add(1), 1);
    (0..range)
        .map(|i| {
            let start = char_indices[i];
            let end = if i + width >= len {
                seq.len()
            } else {
                char_indices[i + width]
            };
            &seq[start..end]
        })
        .collect()
}
```

This uses `char_indices()` to get byte offset positions, then returns `&str` slices into the
original string. No `String` allocation per n-gram — each slice is just a pointer + length into the
source `&str`.

**Call site update pattern** (3 sites in `lib.rs`):

```rust
// Before:
let ngrams = simhash::sliding_window(text, 13);
let features: Vec<u32> = ngrams.iter()
    .map(|ng| xxh32(ng.as_bytes(), 0))
    .collect();

// After:
let ngrams = simhash::sliding_window_strs(text, 13);
let features: Vec<u32> = ngrams.iter()
    .map(|ng| xxh32(ng.as_bytes(), 0))
    .collect();
```

The callers are identical except for the function name — `&str` and `String` both have
`.as_bytes()`.

**Edge cases** to handle (matching current `sliding_window` behavior):

- Empty string → single empty slice `&[""]`
- Input shorter than width → single slice of the full input
- Width of exactly input length → single slice of the full input

**Tests**: Add tests for `sliding_window_strs` in the `mod tests` block of `simhash.rs` that mirror
the existing `sliding_window` tests but verify the slices match the corresponding `String` outputs.

## Verification

- `cargo test -p iscc-lib` passes (all 198 existing tests + new `sliding_window_strs` tests)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo clippy --workspace --all-targets -- -D warnings` clean
- All 9 conformance vector sections still pass (no behavioral change from optimization)
- `cargo bench -p iscc-lib -- gen_text_code_v0` runs without error (timing improvement expected but
    not a pass/fail criterion)

## Done When

All verification criteria pass — the internal sliding window optimization is in place, all existing
tests and conformance vectors pass unchanged, and the public API is unmodified.
