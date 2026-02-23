## 2026-02-23 — Review of: Promote simhash and minhash modules to Tier 1

**Verdict:** PASS_WITH_NOTES

**Summary:** Clean visibility promotion of 3 algorithm functions (`alg_simhash`, `sliding_window`,
`alg_minhash_256`) from `pub(crate)` to `pub`, following the exact pattern established for text
utilities. 17 integration tests cover all specified scenarios including edge cases, Unicode, empty
inputs, and dual import paths. All 182 tests pass (143 unit + 17 algorithm + 22 text integration),
clippy clean, no gate circumvention.

**Issues found:**

- Minor: pre-push clippy caught `cloned_ref_to_slice_refs` lint on `&[digest.clone()]` in test —
    fixed to `std::slice::from_ref(&digest)` during review

**Next:** Promote `alg_cdc_chunks` from the `cdc` module to Tier 1 using the same pattern. This is
the last of the 4 algorithm primitives. After that, the remaining Tier 1 symbols are:
`encode_base64`, `iscc_decompose`, `DataHasher`/`InstanceHasher` streaming types, and
`conformance_selftest`.

**Notes:** Internal helpers remain properly encapsulated — `sliding_window_bytes` is `pub(crate)`,
`minhash()` and `minhash_compress()` are private. The `alg_simhash` function uses `impl AsRef<[u8]>`
which prevents turbofish syntax but is fine for the public API. The promotion pattern is
well-established now (3 successful applications: utils, simhash, minhash) and can be applied
mechanically for the `cdc` module.
