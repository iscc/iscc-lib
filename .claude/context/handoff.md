## 2026-02-24 — Review of: Harden `iscc_decompose` against truncated input

**Verdict:** PASS

**Summary:** Clean, minimal fix adding 3 bounds checks to `iscc_decompose` that cover all 5
unchecked slice operations. Six well-crafted tests exercise every truncation path plus a regression
guard. The `make_truncated_iscc` helper cleanly constructs test inputs using real codec operations.

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 193 unit + 28 algo + 22 text = 243 total, 0 failures
- [x] `cargo clippy -p iscc-lib -- -D warnings` clean — no warnings
- [x] `cargo test -p iscc-lib -- test_decompose_truncated` passes — 4 truncated-prefix tests pass
- [x] Every new test asserts `Err(IsccError::InvalidInput(_))`, not a panic — verified in source
- [x] `mise run check` passes — all 14 pre-commit hooks green

**Issues found:**

- (none)

**Next:** The `iscc_decompose` issue is resolved and deleted from issues.md. The next highest-impact
work should target one of the remaining normal-priority robustness issues: `soft_hash_codes_v0`
too-short Content-Code validation, `gen_meta_code_v0` empty Data-URL handling, `alg_simhash` panic
on mismatched digests, or `sliding_window` panic on width < 2. All are straightforward defensive
fixes similar in scope to this iteration.

**Notes:** The three guards are well-structured: (1) standard unit path checks
`body.len() < nbytes`, (2) wide mode checks `body.len() < 32`, (3) non-wide ISCC-CODE checks
`body.len() < main_types.len() * 8 + 16`. The single upfront check for path (3) elegantly covers
both dynamic unit slicing and the static Data+Instance tail slicing. Error messages are descriptive
and consistent (`"truncated ISCC body: expected N bytes, got M"`).
