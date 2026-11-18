# Next Work Package

## Step: Bind `encode_seq` / `decode_seq` on the napi (Node.js) and WASM surfaces

## Goal

Continue the v0.8.0 ISCC-SEQ fan-out (#45): expose `encode_seq` / `decode_seq` from `@iscc/lib`
(napi) and `@iscc/wasm`, and run the `encode_seq` / `decode_seq` vectors in both conformance
harnesses. This moves the two JS surfaces from 33 to 35 Tier 1 symbols, so 4 of 11 surfaces are
done.

## Alternatives Considered

- **Chosen:** napi + WASM together. Both are thin Rust wrappers over the core with a fixed idiom
    already in each file, so this is 2 source files and finishes the JS family in one pass
- **Rejected:** C FFI seq. It unblocks .NET and C++, but it needs `iscc.h` + `NativeMethods.g.cs`
    regen and a C ownership design for the returned byte buffer and string array. Better as its own
    step next. The `uv.lock` high alerts are dev-only and can follow alongside the other small
    `normal` issues

## Scope

- **Modify**: `crates/iscc-napi/src/lib.rs`, `crates/iscc-wasm/src/lib.rs`
- **Regenerated (gitignored, never hand-edit)**: `crates/iscc-napi/index.d.ts`, `index.js`
- **Tests**: `crates/iscc-napi/__tests__/conformance.test.mjs` (seq sections), a new
    `crates/iscc-napi/__tests__/seq.test.mjs`, `crates/iscc-wasm/tests/conformance.rs` (seq
    sections), `crates/iscc-wasm/tests/unit.rs` (seq unit and negative tests)
- **Reference**: `crates/iscc-py/src/lib.rs` (`encode_seq` / `decode_seq`, ~L515–534),
    `tests/test_seq.py` (constants, divergence rows, negative cases), core
    `crates/iscc-lib/src/codec.rs` L818 / L857

## Not In Scope

- The Tier 1 33→35 doc/count sweep (READMEs, `docs/`, `notes/`, crate `CLAUDE.md`). It runs once
    after every surface binds seq, as with IDv1 (iter 190)
- The `@iscc/wasm` license/description `normal` issue. It is in the same crate, but it is verified
    through a `wasm-pack build` + `pkg/package.json` check, so it is its own step
- Any core (`iscc-lib`) change, including error wording. If a core behaviour looks wrong, report it
    in the handoff instead of patching it
- Other surfaces (FFI, JNI, Ruby, .NET, C++, Swift, Kotlin)

## Implementation Notes

Signatures use snake_case JS names on both surfaces, like every existing export:

- **napi:**
    `#[napi(js_name = "encode_seq")] fn encode_seq(units: Vec<String>) ->   napi::Result<Buffer>`
    and `decode_seq(data: Buffer) -> napi::Result<Vec<String>>`. Map errors with
    `napi::Error::from_reason(e.to_string())`, as `iscc_decode` (L69) does. Collect `Vec<&str>` refs
    as the Python binding does
- **wasm:** `encode_seq(units: JsValue) -> Result<Vec<u8>, JsError>`. Convert the input via
    `serde_wasm_bindgen::from_value::<Vec<String>>`, the same way `gen_iscc_code_v0` (L153) does.
    `decode_seq(data: &[u8]) -> Result<Vec<String>, JsError>`, shaped like `iscc_decompose`
- Doc comments: `decode_seq` returns canonical `ISCC:`-prefixed units. Decoding is strict, so
    `encode_seq` rejects `=` padding, non-zero spare bits and non-ASCII case-fold aliases (`ſ`, `ß`)
    that iscc-core accepts. Say this here; the Python docstring issue is the reason

Conformance vectors (4 per section, no error cases):

- Values are `bytes:<hex>`, not `stream:<hex>`. The `encode_seq` input is `inputs[0]` (a unit list),
    and the output is `bytes:<hex>`. The `decode_seq` input is `inputs[0]` (`bytes:<hex>`), and the
    output is a list of unit strings
- Derive the "tested" guard from the fixture section length, or keep the existing hard-coded count
    style in `conformance.rs`, but never let zero cases pass. Also assert the round trip
    `encode_seq(decode_seq(b)) == b`

Negative tests. The vectors carry no error cases, so these must come from tests, with **one bad
input per call/assertion**:

- `encode_seq([])`, `encode_seq([""])`, `encode_seq(["ISCC:GAAAAAAAAB"])` (spare bits), and
    `encode_seq(["gaawdlqng6ſsdpf2"])` (case-fold alias)
- A bare string `encode_seq("ISCC:GAAAAAAAAA")` must throw, not iterate the string
- `decode_seq` of empty bytes, and of `UNIT_256_HEX` with its last byte dropped (truncated)
- Positive spot checks: `encode_seq(["ISCC:GAAAAAAAAA"])` → hex `300000000000`, and order plus
    duplicates are preserved (`[UNIT_256, UNIT_32, UNIT_32]`)

Traps:

- The napi `.node` is a gitignored artifact and goes stale silently. Rebuild it before `npm test`,
    or the suite runs against the old 33-symbol binary
- napi `Buffer` vs JS `Uint8Array`: compare with `Buffer.from(...).equals(...)` or hex, not
    `strictEqual` on objects
- This is a binding-only step, so neither the iai Perf gate nor the CRAP gate is affected (no core
    edit)

## Verification

- `npm --prefix crates/iscc-napi run build:debug && npm --prefix crates/iscc-napi test` passes,
    including the seq conformance and the new `seq.test.mjs` cases
- `grep -q 'export declare function encode_seq' crates/iscc-napi/index.d.ts` and the same for
    `decode_seq` both succeed after the build
- `wasm-pack test --node crates/iscc-wasm --features conformance` passes, including the seq
    conformance and unit/negative tests
- `cargo clippy -p iscc-napi -p iscc-wasm --all-targets -- -D warnings` clean
- `cargo fmt -p iscc-napi -p iscc-wasm --check` clean
- `mise run check` exits 0

## Done When

Both JS surfaces export `encode_seq` / `decode_seq`, both conformance harnesses run all 8 seq
vectors, and the negative cases throw. All six verification criteria pass.
