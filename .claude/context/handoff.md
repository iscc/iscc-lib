## 2026-02-24 — Review of: Fix `gen_meta_code_v0` empty Data-URL payload routing

**Verdict:** PASS

**Summary:** Clean, minimal two-part fix that corrects empty Data-URL payload routing in
`gen_meta_code_v0`. Change 1 stops discarding empty decoded payloads as `None`, Change 2 adds an
empty-bytes early return in `soft_hash_meta_v0_with_bytes`. Both changes precisely match the Python
reference (`code_meta.py` lines 62 and 142). Two well-structured unit tests with docstrings verify
the new behavior.

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 198 unit + 28 algo + 22 text = 248 total, 0 failures
- [x] `cargo clippy -p iscc-lib -- -D warnings` clean — no warnings
- [x] `cargo test -p iscc-lib -- test_gen_meta_code_empty_data_url` passes (new test)
- [x] `cargo test -p iscc-lib -- test_soft_hash_meta_v0_with_bytes_empty` passes (new test)
- [x] `mise run check` passes — all 14 pre-commit hooks green
- [x] No quality gate circumvention — no lint suppressions, no test skipping, no threshold changes

**Issues found:**

- (none)

**Next:** The empty Data-URL issue is resolved and deleted from issues.md. The next highest-impact
work should target one of the remaining normal-priority robustness issues: `alg_simhash` panic on
mismatched digest sizes or `sliding_window` panic on width < 2. Both are straightforward defensive
fixes similar in scope to this iteration. Alternatively, the performance issues (`sliding_window`
allocation, codec `bytes_to_bits`, `DataHasher::update` copies) could be tackled.

**Notes:** The empty Data-URL edge case has no conformance test vector in `data.json`, so the two
new unit tests are the only coverage. The `rustfmt` auto-formatter collapsed the multi-line match
arm into a single line, which is appropriate. All 16 existing Meta-Code conformance tests remain
passing.
