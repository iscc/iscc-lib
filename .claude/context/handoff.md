## 2026-02-25 — Review of: JNI IllegalStateException for finalized hashers

**Verdict:** PASS

**Summary:** Clean, well-scoped implementation. Added `throw_state_error` helper alongside existing
`throw_and_default`, changed exactly 4 call sites for finalized-hasher errors, updated 2 doc
comments, and added 2 Java tests that verify the correct exception type. All verification criteria
pass, all 51 Maven tests green, clippy clean, all 14 pre-commit hooks pass.

**Verification:**

- [x] `cargo build -p iscc-jni` succeeds
- [x] `mvn test` passes — 51 tests (49 existing + 2 new), 0 failures
- [x] `grep -c 'throw_state_error' lib.rs` outputs 5 (1 definition + 4 call sites — next.md said 4
    counting call sites only, grep also matches the definition line)
- [x] `grep -c 'IllegalStateException' lib.rs` outputs 4 (≥1 required)
- [x] `grep -c 'IllegalStateException' IsccLibTest.java` outputs 4 (≥2 required)
- [x] `cargo clippy -p iscc-jni -- -D warnings` clean
- [x] `mise run check` — all 14 pre-commit hooks pass
- [x] No quality gate circumvention (no `#[allow]`, `#[ignore]`, or threshold changes)

**Issues found:**

- (none)

**Next:** The `[low] iscc-jni: All exceptions mapped to IllegalArgumentException` issue is resolved
and deleted. Two `[low]` issues remain: TypeScript port evaluation and WASM CLAUDE.md staleness. The
Java-side Javadoc in `IsccLib.java` still says `@throws IllegalArgumentException` for hasher
update/finalize methods — this is a cosmetic mismatch that could be cleaned up in a follow-up. The
project is ready for `v0.0.1` release — consider creating a PR from develop to main.

**Notes:** The tests only exercise `update()` after `finalize()` (not `finalize()` after
`finalize()`), but both code paths use the identical `throw_state_error` pattern and the Rust logic
is straightforward. The grep count discrepancy (5 vs 4) is correctly explained by the function
definition line also matching the pattern.
