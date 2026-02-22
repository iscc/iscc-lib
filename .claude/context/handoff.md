## 2026-02-22 — Implement gen_video_code_v0 with WTA-Hash

**Done:** Implemented `gen_video_code_v0` — the last remaining stub — completing all 9 gen
functions. Created the `wtahash` module with the 256-entry permutation table and `alg_wtahash`
algorithm. Implemented `soft_hash_video_v0` with frame deduplication (BTreeSet) and column-wise i64
summing, then WTA-Hash. Changed the function signature from `&[Vec<u8>]` to `&[Vec<i32>]` as frame
signature values can exceed 255.

**Files changed:**

- `crates/iscc-lib/src/wtahash.rs`: Created — WTA-Hash module with `WTA_VIDEO_ID_PERMUTATIONS` (256
    pairs) and `alg_wtahash` function, plus 5 unit tests
- `crates/iscc-lib/src/lib.rs`: Added `mod wtahash`, implemented `soft_hash_video_v0` and
    `gen_video_code_v0`, replaced stub test with conformance tests and empty-input error test

**Verification:** 134 tests pass (was 128 before — gained 5 wtahash unit tests + 2 video tests, lost
1 stub test). All 3 gen_video_code_v0 conformance vectors produce exact matching ISCC strings.
`cargo clippy -p iscc-lib -- -D warnings` clean. `cargo fmt -p iscc-lib --check` clean. No unsafe
code.

**Next:** All 9 gen functions are now implemented and passing conformance. Next steps could be:

- Python bindings via PyO3/maturin (the binding crate structure)
- Streaming API (`new() -> update() -> finalize()`) for Data-Code and Instance-Code
- `meta` parameter support in `gen_meta_code_v0` (JSON/Data-URL objects, currently returns
    NotImplemented)
- Performance benchmarks comparing against iscc-core

**Notes:** The WTA-Hash bit logic is straightforward: `vec[i] >= vec[j]` → bit 0, `vec[i] < vec[j]`
→ bit 1, packed MSB-first. Python uses `set()` for dedup (unordered), Rust uses `BTreeSet` (ordered)
— this is correct because column-wise summing is commutative so iteration order doesn't affect the
result. The signature change from `&[Vec<u8>]` to `&[Vec<i32>]` was required because the range test
vector uses values 0-379.
