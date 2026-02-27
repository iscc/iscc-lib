## 2026-02-27 — Review of: Add algorithm constants and Tier 1 encode_component wrapper

**Verdict:** PASS

**Summary:** Clean, well-scoped implementation adding 4 algorithm constants and a Tier 1
`encode_component` wrapper to the Rust core. The diff touches only `crates/iscc-lib/src/lib.rs` as
specified. All 280 tests pass, clippy is clean, and all 14 pre-commit hooks pass. Magic numbers in
`gen_meta_code_v0` and `soft_hash_text_v0` were replaced with the new constants.

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 280 tests (227 + 31 + 22 + 0 doc-tests), 0 failures
- [x] `cargo clippy -p iscc-lib -- -D warnings` clean
- [x] `grep -c 'pub const META_TRIM_NAME' crates/iscc-lib/src/lib.rs` returns 1
- [x] `grep -c 'pub const IO_READ_SIZE' crates/iscc-lib/src/lib.rs` returns 1
- [x] `grep -c 'pub fn encode_component' crates/iscc-lib/src/lib.rs` returns 1

**Issues found:**

- (none)

**Codex review:** Codex (gpt-5.2) ran tests and investigated two areas: (1) `IO_READ_SIZE` is only
referenced in its definition and test — confirmed correct since it's an advisory constant for SDK
buffer sizing, not used by core algorithms. (2) Edge case analysis of `bit_length < 8` in
`encode_component` — the `encode_length` function in `codec.rs` validates minimum bit lengths per
MainType, so sub-byte requests are caught downstream. No actionable issues.

**Next:** Implement `iscc_decode` (issue #7) — the inverse of `encode_component` that decodes an
ISCC unit string back into header components and raw digest. This is the next of 7 missing Tier 1
symbols. After that, `json_to_data_url` (issue #5). Then propagate all new symbols to the 6 binding
crates.

**Notes:** Issues #6 and #8 are partially addressed — the Rust core symbols exist but binding
propagation remains. These issues should stay open until bindings are updated. The `IO_READ_SIZE`
spec vs reference discrepancy (4 MB vs 2 MB) is documented in learnings and handoff notes — no
conformance impact since it's advisory.
