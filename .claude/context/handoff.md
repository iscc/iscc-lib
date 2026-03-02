## 2026-03-02 — Review of: Add gen_sum_code_v0 to all documentation and READMEs

**Verdict:** PASS_WITH_NOTES

**Summary:** Documentation sweep successfully added `gen_sum_code_v0` to all 9 READMEs, 14 docs
pages, and code comments across 42 files. All changes are documentation-only — no code behavior was
modified. The advance agent's blanket "9→10" replacement introduced factual errors in 9 files where
the code/tests/benchmarks genuinely only operate on 9 functions (conformance tests, benchmarks,
iscc-core-ts). Fixed in this review commit.

**Verification:**

- [x] `grep -rl "gen_sum_code_v0|GenSumCodeV0|genSumCodeV0|iscc_gen_sum_code_v0"` returns hits in
    all expected files (12 files: 8 READMEs + 4 docs pages)
- [x] `grep -r "9 .gen_*_v0|the 9 gen|all 9 gen|All 9.*gen" crates/ docs/ README.md` returns zero
    hits — all user-facing "9 gen" references updated to "10" (exit code 1, no matches)
- [x] `cargo clippy -p iscc-ffi -- -D warnings` passes cleanly
- [x] `diff crates/iscc-wasm/README.md crates/iscc-wasm/pkg/README.md` returns no output (in sync)
- [x] `mise run check` passes (14/14 hooks pass)

**Issues found:**

- **Fixed (9 files):** Advance agent blindly changed "9" to "10" in conformance test docstrings,
    benchmark docstrings, and the type stub, but these files only test/benchmark 9 gen functions (no
    gen_sum_code_v0 in data.json, no gen_sum_code_v0 benchmarks). Fixed by reverting to "9" where
    the code genuinely operates on 9 functions: `conformance.rs` (2 places), WASM `conformance.rs`,
    WASM `unit.rs`, NAPI `conformance.test.mjs`, `test_conformance.py`, `bench_iscc_core.py`,
    `bench_iscc_lib.py`, `_lowlevel.pyi`, `benchmarks.rs`
- **Fixed (ecosystem.md):** iscc-core-ts section changed from "9" to "10" but the function table
    only lists 9 functions (no gen_sum_code_v0). Changed to "9 of the 10"

**Codex review:** Found the conformance.rs docstring inaccuracy (data.json has only 9 function
sections, not 10). This was the key finding that led to discovering the same issue in 8 additional
files. Codex was specifically useful here.

**Next:** The documentation sweep is complete. Remaining work: add gen_sum_code_v0 usage examples to
the howto guides (rust.md, python.md, nodejs.md, wasm.md, java.md, go.md each have a Code Generation
section with per-function examples but no Sum-Code subsection). After that, address issue #16
(feature flags for minimal builds, low priority).

**Notes:** The distinction between "10 gen\_*\_v0 functions in iscc-lib" (correct — the library has
10\) vs "9 gen functions tested by conformance/benchmarks" (correct — data.json has 9 sections, no
gen_sum_code_v0 vectors) is important for future documentation work. The advance agent should
distinguish context-sensitive counts rather than doing blanket find-and-replace. CID context files
(state.md, target.md, specs/*.md) also reference "9 gen" but those are managed separately and were
not in scope.
