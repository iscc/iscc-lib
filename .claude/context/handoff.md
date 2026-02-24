## 2026-02-24 — Fix JSON metadata canonicalization (RFC 8785 / JCS)

**Done:** Replaced `serde_json::to_vec` with `serde_json_canonicalizer::to_writer` in
`parse_meta_json()` for RFC 8785 (JCS) compliant JSON canonicalization. The previous implementation
used `serde_json` sorted-key serialization, which diverges from iscc-core's `jcs.canonicalize()` on
numeric values (e.g., JCS serializes `1.0` as `1`, `1e20` as `100000000000000000000`). Added two
failing-then-passing tests with expected values generated from iscc-core. Filed iscc/iscc-core#131
requesting upstream add float-valued JSON metadata test vectors.

**Files changed:**

- `Cargo.toml`: Added `serde_json_canonicalizer = "0.3.2"` to workspace dependencies
- `crates/iscc-lib/Cargo.toml`: Added `serde_json_canonicalizer.workspace = true`
- `crates/iscc-lib/src/lib.rs`: Replaced `serde_json::to_vec` with
    `serde_json_canonicalizer::to_writer` in `parse_meta_json()`; added two JCS conformance tests
    (`test_gen_meta_code_v0_jcs_float_canonicalization`,
    `test_gen_meta_code_v0_jcs_large_float_canonicalization`)
- `crates/iscc-lib/CLAUDE.md`: Updated pitfall note to reflect JCS compliance

**Verification:**

- `cargo test -p iscc-lib` passes all 182 tests (including 2 new JCS tests)
- All 28 integration tests pass, all 22 text utils tests pass
- Existing conformance vectors unaffected (string-only JSON produces identical output under JCS)

**Next:** The remaining documentation gaps are: (1) abbreviations file
(`docs/includes/abbreviations.md`) with ISCC-specific terms and `pymdownx.snippets` auto-append
configuration, (2) CNAME file (`docs/CNAME` with `lib.iscc.codes`), (3) OIDC trusted publishing
configuration in `release.yml`. The abbreviations and CNAME items are small and could be combined
into a single step.
