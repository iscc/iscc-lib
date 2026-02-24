## 2026-02-24 — Review of: Add Rust how-to guide

**Verdict:** PASS

**Summary:** Clean, well-structured Rust how-to guide (351 lines) mirroring the Python guide's
section layout. All 9 gen functions documented with correct signatures and result types. Streaming,
text utilities, algorithm primitives, conformance testing, and error handling sections are complete.
One minor inaccuracy fixed (alg_cdc_chunks return type). All quality gates pass.

**Verification:**

- [x] `uv run zensical build` exits 0 — site builds with `/howto/rust/` page
- [x] `docs/howto/rust.md` exists and has 250+ lines — 351 lines (target: ~280–350)
- [x] `zensical.toml` nav contains `"Rust" = "howto/rust.md"` in How-to Guides — line 18
- [x] `grep -c 'gen_.*_code_v0' docs/howto/rust.md` returns 9+ — 29 matches
- [x] `grep -c 'DataHasher\|InstanceHasher' docs/howto/rust.md` returns 2+ — 8 matches

**Issues found:**

- Minor: `alg_cdc_chunks` return type documented as `Vec<Vec<u8>>` but actual is `Vec<&[u8]>` —
    fixed in this review commit

**Next:** The documentation section is now closer to "met" — both Rust and Python how-to guides are
in place. The remaining documentation gap is nav icons for top-level sections. Beyond documentation,
the highest-value work items from issues.md are the correctness issues: `alg_simhash` panics on
mismatched digest sizes [normal], `sliding_window` panics on width < 2 [normal]. The API reference
page (`docs/rust-api.md`) is now inconsistent with actual result struct types — updating it should
be considered.

**Notes:** The advance agent correctly used actual result struct types (`MetaCodeResult`,
`TextCodeResult`, etc.) from `types.rs` rather than the outdated `IsccResult<String>` pattern. The
Rust how-to has an extra "Algorithm primitives" section not present in the Python guide, as
specified by next.md. All function signatures verified against actual source code.
