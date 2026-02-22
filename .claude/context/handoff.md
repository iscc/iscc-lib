## 2026-02-22 — Implement gen_mixed_code_v0 with SimHash code mixing

**Done:** Implemented `gen_mixed_code_v0` and its internal helper `soft_hash_codes_v0`. The mixed
code generator accepts multiple ISCC Content-Code strings, decodes them, extracts header byte +
truncated body for each, runs SimHash over the prepared sequences, and encodes the result as a Mixed
Content-Code.

**Files changed:**

- `crates/iscc-lib/src/lib.rs`: Added `soft_hash_codes_v0` helper (validates Content MainType,
    prepares nbytes-length entries from header byte + body, calls `alg_simhash`) and replaced
    `gen_mixed_code_v0` stub with full implementation (strips "ISCC:" prefix, decodes base32, calls
    helper, encodes component). Replaced `test_gen_mixed_code_v0_stub` with conformance test
    iterating all 2 vectors from data.json plus an error test for fewer than 2 codes.

**Verification:** All 93 tests pass (same count as before — removed 1 stub, added 2 new tests). Both
conformance vectors (`test_0000_std_64` and `test_0001_128_truncated`) produce matching ISCC codes.
Clippy clean, fmt clean, no unsafe code.

**Next:** Implement `gen_data_code_v0` — this requires CDC (content-defined chunking) plus MinHash,
which are the most complex algorithms remaining. The minhash module is already available. CDC will
need to be ported from `bio-codes/iscc-sum`.

**Notes:** The `alg_simhash` function correctly infers output length from input element length, so
passing nbytes-length prepared entries produces an nbytes-length SimHash — no additional truncation
needed before `encode_component`. The `collect` with `?` pattern (`collect::<IsccResult<Vec<_>>>()`)
works cleanly for propagating decode errors from the map iterator.
