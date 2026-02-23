## 2026-02-23 — Implement iscc_decompose and decode_units codec functions

**Done:** Implemented `decode_units` (inverse of `encode_units`, maps unit_id 0-7 to sorted
`Vec<MainType>` via 3-bit bitfield decoding) and `iscc_decompose` (Tier 1 function that splits a
composite ISCC-CODE or concatenated ISCC sequence into individual ISCC-UNIT base32 strings). Added
`pub use codec::iscc_decompose` re-export in `lib.rs` for flat crate-root access.

**Files changed:**

- `crates/iscc-lib/src/codec.rs`: Added `decode_units` (pub, Tier 2) and `iscc_decompose` (pub, Tier
    1\) functions with 17 new tests covering all 8 unit_id mappings, roundtrip with `encode_units`,
    single-unit pass-through, prefix stripping, and conformance-based decomposition of all 5
    `gen_iscc_code_v0` test vectors
- `crates/iscc-lib/src/lib.rs`: Added `pub use codec::iscc_decompose;` re-export

**Verification:** All 164 Rust tests pass (147 existing + 17 new). Clippy clean. All pre-commit
hooks pass. Tests verify: all 8 `decode_units` mappings (0→[] through 7→[Meta,Semantic,Content]),
error for unit_id > 7, roundtrip with `encode_units`, single-unit decompose with and without "ISCC:"
prefix, MainType verification of decomposed units, 4-unit standard composite (Meta + Content + Data
\+ Instance), 3-unit no-meta composite, 2-unit sum-only composite, and all 5 `gen_iscc_code_v0`
conformance vectors decomposed with structural validation (correct count, Data + Instance always
last two, no ISCC MainType in output).

**Next:** 20 of 22 Tier 1 symbols implemented. Remaining 2: `DataHasher`/`InstanceHasher` (streaming
types that implement the `new() → update(&[u8]) → finalize()` pattern) and `conformance_selftest`.
The streaming types are the next logical step since they enable the streaming API pattern that is
central to the library's design.

**Notes:** `iscc_decompose` returns ISCC-UNIT strings WITHOUT "ISCC:" prefix, matching the Python
reference implementation's behavior (`encode_component` returns bare base32). This differs from the
`gen_*_v0` functions which all return with "ISCC:" prefix. The function accepts input both with and
without prefix. `decode_units` uses direct bitfield decoding (bit0=Content, bit1=Semantic,
bit2=Meta) rather than a lookup table — equivalent to Python's `UNITS` table but more idiomatic in
Rust.
