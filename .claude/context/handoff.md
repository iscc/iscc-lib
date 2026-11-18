## 2026-11-18 — Bind `encode_seq` / `decode_seq` on the napi (Node.js) and WASM surfaces

**Done:** `@iscc/lib` (napi) and `@iscc/wasm` export `encode_seq` / `decode_seq` as thin wrappers
over the core. Both conformance harnesses run all 8 seq vectors plus a round trip. The negative
cases (one bad input per assertion) throw on both surfaces. Both JS surfaces now have 35 Tier 1
symbols.

**Files changed:**

- `crates/iscc-napi/src/lib.rs`: `encode_seq(units: Vec<String>) -> napi::Result<Buffer>`,
    `decode_seq(data: Buffer) -> napi::Result<Vec<String>>`, both `js_name` snake_case. The doc
    comment covers strict decoding (padding, spare bits, `ſ`/`ß` rejected although iscc-core accepts
    them)
- `crates/iscc-wasm/src/lib.rs`: `encode_seq(units: JsValue) -> Result<Vec<u8>, JsError>` (via
    `serde_wasm_bindgen::from_value::<Vec<String>>`),
    `decode_seq(&[u8]) -> Result<Vec<String>,   JsError>`, with the same doc notes
- `crates/iscc-napi/__tests__/conformance.test.mjs`: `encode_seq` / `decode_seq` suites driven by
    the fixture, a `decodeBytes` helper for `bytes:<hex>`, a non-empty guard per section, and a
    round trip per decode vector
- `crates/iscc-napi/__tests__/seq.test.mjs` (new): spot check `["ISCC:GAAAAAAAAA"]` →
    `300000000000`, returns a `Buffer`, order and duplicates kept, wire-order decode; throws on
    `[]`, `[""]`, spare bits, the `ſ` alias, a bare string, empty bytes, and truncated
    `UNIT_256_HEX`
- `crates/iscc-wasm/tests/conformance.rs`: `test_encode_seq_conformance` /
    `test_decode_seq_conformance` (the decode test also asserts the round trip), using the
    hard-coded `tested == 4` guard style
- `crates/iscc-wasm/tests/unit.rs`: 10 seq tests, the napi set minus `returns a Buffer` (the bare
    string goes in as `JsValue::from_str`)

**Verification:**

- `npm --prefix crates/iscc-napi run build:debug && npm --prefix crates/iscc-napi test`: 174 tests,
    174 passed, 0 failed (re-run after the final test edit). The TAP output lists all 4 `encode_seq`
    vectors (`test_0001`–`test_0004`), all 4 `decode_seq` vectors and 4 round trips, each with `ok`
- `index.d.ts` after the build has
    `export declare function decode_seq(data: Buffer):   Array<string>` (L85) and
    `export declare function encode_seq(units: Array<string>): Buffer` (L111)
- `wasm-pack test --node crates/iscc-wasm --features conformance`: exit 0. conformance 11 passed
    (incl. both seq tests), unit 94 passed (all 10 seq tests listed `ok`; re-run after the final
    edit)
- `cargo clippy -p iscc-napi -p iscc-wasm --all-targets -- -D warnings`: clean
- `cargo fmt -p iscc-napi -p iscc-wasm --check`: exit 0
- `mise run check`: exit 0
- No core edit, so the iai and CRAP gates are not affected and were not run

**Next:** C FFI seq (`iscc.h` + `NativeMethods.g.cs` regen in the SAME step, plus a C ownership
design for the returned byte buffer and string array). That unblocks .NET and C++. After that come
JNI, Ruby, then UniFFI (Swift/Kotlin), and finally the 33→35 doc/count sweep.

**Notes:**

- A bare string is rejected at argument conversion and never reaches the core. The tests pin this
    because an iterated string would also throw, but in the core. napi throws
    `code: 'ArrayExpected'` ("Failed to get Array length"), and the test asserts that code. WASM's
    `serde_wasm_bindgen::from_value` throws `TypeError: Reflect.get called on non-object`. The test
    asserts the message lacks the core's `invalid input` marker, with a control call on `[""]` that
    shows core errors carry it
- I did not touch `crates/iscc-napi/CLAUDE.md`, the READMEs or the docs. Their Tier 1 counts and the
    "3 codec functions" wording are now out of date for the JS surfaces. That is deliberate: the
    sweep is out of scope (next.md)
- The fixture `outputs` for `encode_seq` is a bare `bytes:<hex>` string, not an object. The
    `decode_seq` outputs is a plain list
