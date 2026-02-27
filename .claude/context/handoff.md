## 2026-02-27 — Propagate 7 Tier 1 symbols to C FFI

**Done:** Added 4 constant getter functions (`iscc_meta_trim_name`, `iscc_meta_trim_description`,
`iscc_io_read_size`, `iscc_text_ngram_size`), `iscc_json_to_data_url`, `iscc_encode_component`,
`iscc_decode` (with `IsccDecodeResult` struct and `iscc_free_decode_result` free function) to the C
FFI binding crate. Total `#[unsafe(no_mangle)]` symbols: 44 (was 35). All 30/30 Tier 1 symbols are
now accessible via the C ABI.

**Files changed:**

- `crates/iscc-ffi/src/lib.rs`: Added 4 constant getter `extern "C"` functions,
    `iscc_json_to_data_url` (string-in/string-out), `iscc_encode_component` (digest+header→ISCC unit
    string), `IsccDecodeResult` repr(C) struct with `iscc_decode` and `iscc_free_decode_result`.
    Added 15 new Rust unit tests covering all 7 symbols (constants, json_to_data_url,
    encode_component, iscc_decode, roundtrip, error cases, null safety).
- `crates/iscc-ffi/tests/test_iscc.c`: Added tests 18-23 covering 4 constant getters,
    json_to_data_url, encode_component, iscc_decode (success + error), and encode→decode roundtrip
    with digest verification. Total: 49 C assertions (was 30).

**Verification:**

- `cargo test -p iscc-ffi` passes — 77 unit tests (62 existing + 15 new), 0 failures
- `cargo clippy -p iscc-ffi --all-targets -- -D warnings` clean — no warnings
- `grep -c '#\[unsafe(no_mangle)\]' crates/iscc-ffi/src/lib.rs` shows 44 (exceeds minimum of 43)
- C test program compiles and passes: 49 passed, 0 failed
- `mise run check` passes — all 14 pre-commit hooks pass

**Next:** All 30 Tier 1 symbols are now propagated to Python, Node.js, WASM, and C FFI bindings.
Remaining binding propagation targets: Java JNI (23/30) and Go/wazero (23/30). The define-next agent
should continue propagating the 7 new symbols to Java JNI next.

**Notes:** The next.md incorrectly stated `length==0 (64-bit = length index 0)` for the
`iscc_decode` verification test. The actual length index for 64-bit is 1, because `decode_length`
uses `(length_index + 1) * 32` for standard MainTypes (so index 0 = 32-bit, index 1 = 64-bit). The
`ptr_to_str` helper in this crate takes an additional `param_name` argument compared to the next.md
pseudocode, which was accounted for in the implementation. The generated `iscc.h` header is not
committed — CI generates it dynamically via `cbindgen`.
