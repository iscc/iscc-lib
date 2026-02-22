## 2026-02-22 — Implement gen_iscc_code_v0 composite code assembly

**Done:** Implemented `gen_iscc_code_v0` — the composite ISCC-CODE generator that combines multiple
ISCC unit codes (Meta, Content, Data, Instance) into a single ISCC-CODE. Added `encode_units` helper
to codec.rs and derived `Ord`/`PartialOrd` on `MainType` for sorting. All 5 conformance vectors
pass.

**Files changed:**

- `crates/iscc-lib/src/codec.rs`: Added `Ord, PartialOrd` derives on `MainType`; added
    `encode_units` function (bitfield encoding of optional unit types) with 11 unit tests
- `crates/iscc-lib/src/lib.rs`: Replaced `gen_iscc_code_v0` stub with full implementation (input
    cleaning, validation, decode/sort/encode pipeline, wide mode support); replaced stub test with 5
    conformance vector tests + 3 error case tests

**Verification:** All 107 tests pass (was 93 + removed 1 stub + added 15 new = 107). `cargo clippy`
clean, `cargo fmt --check` clean, no `unsafe` code.

**Next:** 6/9 gen functions implemented. 3 stubs remain: `gen_image_code_v0`, `gen_video_code_v0`,
`gen_data_code_v0`. The most impactful next step is `gen_data_code_v0` (CDC chunking + MinHash)
since it's a prerequisite for real-world ISCC-CODE generation. Prior Rust CDC implementation exists
in `bio-codes/iscc-sum` to adapt from. Alternatively, `gen_image_code_v0` or `gen_video_code_v0`
could be tackled if their reference algorithms are researched first.

**Notes:**

- `MainType` now derives `Ord`/`PartialOrd` (needed for sorting units by type). The discriminant
    order (Meta=0..Instance=4) matches the required sort order, so the derive is correct.
- The `wide` parameter isn't exercised by conformance vectors (all use `wide=false`), but the code
    path is implemented per the Python reference algorithm.
- The test description in next.md for test_0002 and test_0003 was slightly misleading about input
    types — trusted the actual conformance vector data instead.
