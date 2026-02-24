## 2026-02-24 — Fix `alg_cdc_chunks` infinite loop when `utf32=true`

**Done:** Fixed the infinite loop in `alg_cdc_chunks` when `utf32=true` and the remaining buffer is
smaller than 4 bytes. The bug was at `cdc.rs:130-132` where `cut_point -= cut_point % 4` reduced
`cut_point` to 0 for small remainders, preventing `pos` from advancing. The fix adds a guard: when
UTF-32 alignment reduces `cut_point` to 0, it is set to `min(4, remaining.len())` to guarantee
forward progress.

**Files changed:**

- `crates/iscc-lib/src/cdc.rs`: Added 3-line guard after UTF-32 alignment subtraction + 5 new tests

**Verification:**

- `cargo test -p iscc-lib` passes all 187 tests (182 existing + 5 new utf32 tests)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo test -p iscc-lib -- test_alg_cdc_chunks_utf32` passes (5 new + 1 existing utf32 test)
- 3-byte input test (`test_alg_cdc_chunks_utf32_small_buffer`) proves the infinite loop is fixed
- `mise run check` passes all pre-commit hooks

**Next:** The critical issue is resolved. Remaining issues from issues.md are all normal/low
priority. The `iscc_decompose` panics on malformed input (normal priority) is the highest-impact
remaining robustness issue.

**Notes:** The Python reference (`cdc.py:47`) has the identical bug — `cut_point -= cut_point % 4`
can produce 0 in Python too, but Python's generator-based approach might mask it differently (the
`while buffer:` loop would exit if `buffer` becomes empty). This upstream issue was already noted in
next.md as out of scope.
