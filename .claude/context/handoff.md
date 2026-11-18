## 2026-11-18 — Review of: Bind `encode_seq` / `decode_seq` on the napi (Node.js) and WASM surfaces

**Verdict:** PASS_WITH_NOTES

**Summary:** `@iscc/lib` (napi) and `@iscc/wasm` now export `encode_seq` / `decode_seq` as thin core
wrappers, following each crate's existing idiom. Both conformance harnesses run all 8 seq vectors
plus round trips. The negative tests use one bad input per assertion, and both bare-string tests
check that the string is rejected at argument conversion, not by the core. Scope is clean: 2 source
files plus tests, no core edit. This brings ISCC-SEQ to 4 of 11 surfaces.

**Verification:**

- [x] `npm --prefix crates/iscc-napi run build:debug && npm … test`: 174/174 pass. Both
    `seq.test.mjs` suites are collected (the test script globs `__tests__/*.test.mjs`)
- [x] `index.d.ts` has `export declare function encode_seq` and `… decode_seq`
- [x] `wasm-pack test --node crates/iscc-wasm --features conformance`: conformance 11 passed (incl.
    both seq tests), unit 94 passed (all 10 seq tests `ok`)
- [x] `cargo clippy -p iscc-napi -p iscc-wasm --all-targets -- -D warnings` clean (workspace clippy
    also clean)
- [x] `cargo fmt -p iscc-napi -p iscc-wasm --check` clean
- [x] `mise run check` exit 0
- Extra probes: the spare-bits and `ſ` negatives fail for the intended reasons (non-zero trailing
    bits; a base32 decode error). The napi and serde-wasm-bindgen source confirms both Codex P2
    findings (below)

**Issues found:**

- Fixed in review: the WASM `encode_seq` doc claimed it "throws on a non-array argument". It does
    not: `serde_wasm_bindgen` accepts any JS iterable, so `encode_seq(new Set([unit]))` succeeds.
    The doc now says "Throws on a bare string, …", and no behaviour changed
- The JS surfaces differ on non-array iterables. napi rejects a `Set` (`ArrayExpected`); WASM
    accepts it. This is the same for every existing WASM list parameter (`gen_iscc_code_v0`,
    `gen_mixed_code_v0`, `alg_simhash`), so it is a crate-wide convention, not a seq defect. Not
    filed
- Filed `normal`: every napi `Buffer` parameter keeps a reference to a rejected non-Buffer object
    for the life of the Node environment (an upstream napi-rs 3.11 bug that predates this step and
    affects the whole crate)
- WASM `encode_seq` types its parameter as `units: any` in `.d.ts`, like the existing `codes: any`.
    Cosmetic, not filed

**Codex review:** Run on the main commit `fba5378`. (1) P2, napi `Buffer` leaks a reference on
`ArrayBuffer` input: confirmed in napi 3.11.0 source; crate-wide and pre-existing → filed. (2) P2,
WASM accepts a `Set`: confirmed; the false doc sentence is fixed, and the behaviour follows the
crate convention. (3) P3, `any` TS type: crate-wide, noted. (4) P3, the bare-string tests accepted
any error: already fixed by `ea1527e`.

**Next:** C FFI seq, as advance proposed. Regenerate and commit `iscc.h` AND `NativeMethods.g.cs` in
the SAME step (csbindgen rewrites the `.g.cs` on every build, and pre-push clippy fails otherwise),
and design C ownership for the returned byte buffer and string array (reuse the existing
`IsccByteBuffer` / string-array free functions if they fit). That unblocks .NET and C++. The other
`normal` issues compete: the uv.lock Dependabot alerts, the Ruby `json` alert, the Ruby locale fix,
the Python `encode_seq` docstring, the `@iscc/wasm` license, `docs/c-ffi-api.md` type names, and the
napi `Buffer` leak.

**Notes:**

- The Tier 1 33→35 doc/count sweep is still deliberately deferred until every surface binds seq.
    `crates/iscc-napi/CLAUDE.md` and `crates/iscc-wasm/CLAUDE.md` still say 33 symbols and "3 codec
    functions"
- When the C FFI step adds a `char**` return, follow the existing `iscc_decompose` FFI pattern and
    test the free function under ASan or valgrind if the step touches ownership
