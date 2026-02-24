## 2026-02-24 — Review of: Harden `alg_simhash` — return `IsccResult` on mismatched digest sizes

**Verdict:** PASS

**Summary:** Clean, well-scoped implementation. Split `alg_simhash` into a validated public API and
an unchecked `pub(crate)` inner variant. All 7 internal call sites use the inner variant to skip
redundant validation. All 4 binding crates properly propagate the `Result` through their respective
error conventions. Two new error-case tests added at both unit and integration levels.

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 206 tests (including new error-case tests)
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean
- [x] `cargo test --workspace` passes — all crates compile and tests pass
- [x] `pytest` passes — 147 Python tests
- [x] `npm test --prefix crates/iscc-napi` passes — 103 Node.js tests
- [x] `alg_simhash` with `vec![vec![1u8, 2], vec![1u8, 2, 3]]` returns `Err` — confirmed by both
    unit and integration tests
- [x] `mise run check` — all 14 pre-commit hooks pass

**Issues found:**

- (none)

**Next:** Continue with the remaining robustness issues from issues.md. The `sliding_window` panic
on `width < 2` is the next [normal] priority. Then the [low] issues (`alg_dct` power-of-two
enforcement, `alg_wtahash` short-input guard). Alternatively, the [normal] performance issues (codec
header parsing, DataHasher buffer allocation) could be tackled if the team prefers optimization over
hardening.

**Notes:** The inner/outer split pattern (`alg_simhash` / `alg_simhash_inner`) is established and
can be reused for other hardening work (e.g., `sliding_window`). The `alg_simhash` issue has been
resolved and removed from issues.md.
