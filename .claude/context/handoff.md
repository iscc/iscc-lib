## 2026-02-24 — Fix `gen_meta_code_v0` empty Data-URL payload routing

**Done:** Fixed two bugs in `gen_meta_code_v0` that caused incorrect routing when a Data-URL with an
empty base64 payload was provided. Change 1: removed the `if decoded.is_empty() { None }` branch in
`gen_meta_code_v0` so that `Some(vec![])` flows through to the meta bytes path (matching Python's
truthy `if meta:` behavior). Change 2: added an early return guard in `soft_hash_meta_v0_with_bytes`
for empty bytes, returning the name-only SimHash without interleaving (matching Python's
`if extra in {None, "", b""}:` guard).

**Files changed:**

- `crates/iscc-lib/src/lib.rs`: Removed empty-payload-to-None mapping in `gen_meta_code_v0` (line
    185), added `if extra.is_empty() { return name_simhash; }` guard in
    `soft_hash_meta_v0_with_bytes` (lines 102-104), added two new test functions at end of test
    module

**Verification:** `cargo test -p iscc-lib` passes — 198 unit + 28 algorithm + 22 text = 248 total, 0
failures. `cargo clippy -p iscc-lib -- -D warnings` clean. Both new tests pass:
`test_gen_meta_code_empty_data_url_enters_meta_branch` and
`test_soft_hash_meta_v0_with_bytes_empty_equals_name_only`. All 16 existing Meta-Code conformance
tests still pass. `mise run check` passes — all 14 pre-commit hooks green.

**Next:** The empty Data-URL issue in issues.md is now resolved and can be deleted. The next
highest-impact work should target one of the remaining normal-priority robustness issues:
`alg_simhash` panic on mismatched digest sizes or `sliding_window` panic on width < 2. Both are
straightforward defensive fixes similar in scope to this iteration.

**Notes:** No surprises. The `rustfmt` auto-formatter collapsed the multi-line match arm into a
single line, which is cleaner. The empty Data-URL edge case has no conformance test vector in
`data.json`, so the two new unit tests are the only coverage for this behavior. The fix ensures
parity with Python's `code_meta.py` lines 62 and 142.
