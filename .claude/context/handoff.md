## 2026-02-27 — Review of: Propagate 7 Tier 1 symbols to C FFI

**Verdict:** PASS

**Summary:** Clean, focused implementation adding 4 constant getter functions,
`iscc_json_to_data_url`, `iscc_encode_component`, `iscc_decode` (with `IsccDecodeResult` struct and
`iscc_free_decode_result`) to the C FFI binding crate. All 30/30 Tier 1 symbols are now accessible
via the C ABI. The advance agent correctly identified that the next.md specification had an error in
the length index for 64-bit codes (should be 1, not 0) and implemented accordingly.

**Verification:**

- [x] `cargo test -p iscc-ffi` passes — 77 unit tests (62 existing + 15 new), 0 failures
- [x] `cargo clippy -p iscc-ffi --all-targets -- -D warnings` clean — no warnings
- [x] `grep -c '#[unsafe(no_mangle)]' crates/iscc-ffi/src/lib.rs` shows 44 (≥ 43 required)
- [x] C test program compiles and passes — 49 passed, 0 failed (requires cbindgen header generation)
- [x] `mise run check` passes — all 14 pre-commit hooks pass

**Issues found:**

- (none)

**Codex review:** No correctness issues identified. The added C FFI exports follow the crate's
existing error/ownership patterns, and the new Rust/C tests cover the expected success and failure
cases.

**Next:** All 30 Tier 1 symbols are now propagated to Python, Node.js, WASM, and C FFI bindings.
Remaining binding propagation targets: Java JNI (23/30) and Go/wazero (23/30). The define-next agent
should continue propagating the 7 new symbols to Java JNI next.

**Notes:** Minor fix applied — removed duplicate `// ── Codec ──` section comment in
`crates/iscc-ffi/src/lib.rs`. The C test compilation requires `cbindgen` to generate `iscc.h` before
gcc can build (header is not committed — CI generates it dynamically). The advance agent's
observation about next.md's incorrect length index specification is correct and has been noted in
learnings.
