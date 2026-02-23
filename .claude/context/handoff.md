## 2026-02-23 — Review of: Promote alg_cdc_chunks to Tier 1 public API

**Verdict:** PASS

**Summary:** Clean mechanical promotion of `alg_cdc_chunks` from `pub(crate)` to `pub`, completing
all 4 algorithm primitives in the Tier 1 API. 6 new integration tests added (basic chunking, empty
input, reassembly, UTF-32 alignment, different avg sizes, module path import) plus 1 existing test
updated to include the new import. All 188 tests pass (143 unit + 23 algorithm primitives + 22 text
utils), clippy clean, no gate circumvention.

**Issues found:**

- (none)

**Next:** The remaining Tier 1 symbols to promote are: `encode_base64` (not implemented),
`iscc_decompose` (not implemented), `DataHasher`/`InstanceHasher` streaming types (not implemented),
`soft_hash_video_v0` (exists as private fn), and `conformance_selftest` (not implemented). The codec
utilities (`encode_base64`, `iscc_decompose`) are likely the next straightforward targets since they
involve implementing new public functions in the codec module rather than complex streaming types.

**Notes:** Promoted symbol count is now 17 of 22 Tier 1 targets. The remaining 5 symbols require new
implementation rather than simple visibility promotion. `state.md` should be updated to reflect the
new count. The promotion pattern is fully proven (4 successful applications: utils, simhash,
minhash, cdc) and requires no further iteration.
