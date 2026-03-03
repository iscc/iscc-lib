## 2026-03-03 — Review of: Vendor iscc-core v1.3.0 data.json and fix Go conformance loader

**Verdict:** PASS

**Summary:** Vendored iscc-core v1.3.0 conformance vectors (50 total, 4 new Meta-Code tests) to both
Rust and Go test data locations. Fixed Go conformance loader and all 9 per-function test files to
tolerate the new `_metadata` top-level key via a clean `parseConformanceData()` helper. Updated Rust
meta conformance assertion from 16 to 20. All tests pass across Rust (314) and Go (155).

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 314 tests (258+31+24+1), all green. Note: next.md predicted
    318+ but the 4 new vectors are iterated within the existing single test function, not separate
    test entries
- [x] `cargo clippy -p iscc-lib -- -D warnings` — clean
- [x] `cd packages/go && go test ./...` — passes (155 tests including 4 new meta vectors as
    subtests)
- [x] `cd packages/go && go vet ./...` — clean
- [x] `grep -c 'test_0017\|test_0018\|test_0019\|test_0020' crates/iscc-lib/tests/data.json` —
    returns 4 (new vectors present)
- [x] `grep '_metadata' crates/iscc-lib/tests/data.json` — found (confirms v1.3.0)
- [x] `mise run check` — all 14 pre-commit hooks pass
- [x] No quality gate circumvention — no lint suppressions, test skips, or threshold changes
- [x] Both data.json copies are byte-identical (verified with `diff`)

**Issues found:**

- (none)

**Codex review:** Clean — Codex confirmed the changes correctly handle the `_metadata` key and the
Rust test expectation update matches the new vector count. No actionable findings.

**Next:** The critical iscc-core v1.3.0 conformance issue is now fully resolved — all 50 vectors
pass in Rust and Go. Other binding tests (Python, Node.js, WASM, Java, C FFI) access data.json by
section name and will naturally pick up the new vectors without code changes. Next priority is the
Ruby bindings implementation (`normal` issue) — scaffold the `iscc-rb` Magnus crate per the spec at
`.claude/context/specs/ruby-bindings.md`. A good first step would be crate setup (Cargo.toml,
src/lib.rs skeleton, Gemfile, Rakefile) before implementing the 32 Tier 1 symbols.

**Notes:**

- The advance agent went slightly beyond next.md scope by fixing 9 Go per-function test files and
    the lib.rs assertion — both were necessary for tests to pass and the fixes are minimal/correct
- The `_metadata` key handling in Go uses a well-designed two-pass parsing approach
    (`parseConformanceData()`) that is reused across all test files (DRY)
- Other bindings (Python, Node.js, WASM, Java, C FFI) use the updated data.json from
    `crates/iscc-lib/tests/` and parse by section name — no changes needed
- `iscc_validate` codec validation tightening and `iscc_nph_compare` from v1.3.0 are not
    conformance-critical and were correctly deferred
