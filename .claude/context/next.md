# Next Work Package

## Step: WASM `alg_cdc_chunks` returns `Uint8Array` chunks

## Goal

Fix the `normal` issue "WASM `alg_cdc_chunks` returns plain arrays, its doc comment says
`Uint8Array`": `@iscc/wasm`'s `alg_cdc_chunks` must return an array of `Uint8Array` chunks, as its
doc comment and published JSDoc promise and as every other `@iscc/wasm` byte output already does.

## Alternatives Considered

- **Chosen:** WASM `alg_cdc_chunks` return type — a user-facing binding defect (a caller following
    the JSDoc gets a `TypeError` on `.subarray()`), small, fully verifiable with `wasm-pack test`.
    The last four iterations were tooling, tests and refactors; this one fixes a published API
- **Rejected:** closing the Tier 1 doc gate's holes (fail-closed `lib.rs` parser + registering the
    four sub-threshold docs, handoff "Next" item 1) — internal guidance tooling with no user impact,
    and `docs/rust-api.md` alone may need its own step; it follows next, with the Java how-to fix

## Scope

- **Modify**: `crates/iscc-wasm/src/lib.rs` (`alg_cdc_chunks` only), `crates/iscc-wasm/Cargo.toml`
    and root `Cargo.toml` (add `js-sys` if used — `Cargo.lock` already carries js-sys 0.3.103
    transitively), `docs/howto/wasm.md` → "Additional primitives" (the `alg_cdc_chunks` bullet)
- **Tests**: `crates/iscc-wasm/tests/unit.rs` (the three `test_alg_cdc_chunks_*` tests)
- **Reference**: issues.md entry; `crates/iscc-wasm/CLAUDE.md` (type mapping table);
    `crates/iscc-napi/src/lib.rs` → `alg_cdc_chunks` (returns `Buffer[]`, the sibling shape)

## Not In Scope

- The napi `Buffer` reference leak (`low`) and any other `@iscc/lib` change
- Other WASM exports: the probe found `alg_cdc_chunks` is the only `serde_wasm_bindgen::to_value`
    return in `crates/iscc-wasm/src/lib.rs`; every other byte output is already `Vec<u8>` →
    `Uint8Array`. Do not migrate input-side `JsValue` parameters (`alg_simhash`, `gen_video_code_v0`
    …) — they are inputs, not this defect
- A `data-edge.json` vector: the defect is a JS return *type*, not a JSON-expressible value
    divergence (`edge-vectors.md` → Rules), so none is owed
- The Tier 1 doc gate holes and the Java how-to example (next steps)

## Implementation Notes

- Preferred shape: return `Result<Vec<js_sys::Uint8Array>, JsError>`, building each chunk with
    `js_sys::Uint8Array::from(chunk)` (copies into JS memory). wasm-bindgen 0.2.126 supports
    `Vec<T>` returns for JsValue-repr types (`src/convert/slices.rs:546`,
    `impl<T: ErasableGeneric<Repr = JsValue> + WasmDescribe> VectorIntoWasmAbi for T`), so the
    generated `.d.ts` becomes `Uint8Array[]` instead of `any` — check it in the built `pkg/`. Add
    `js-sys = "0.3"` to `[workspace.dependencies]` and `js-sys = { workspace = true }` to the crate
    (keep entries sorted the way the files already are). If another wasm-bindgen-native route is
    simpler (no new dep), it is acceptable as long as the d.ts and runtime type are `Uint8Array`
- Keep the empty-input contract: one chunk, and that chunk is an empty `Uint8Array`
- Update the Rust doc comment only if wording changes; it already says `Uint8Array`
- Tests: rewrite the three tests for the new return type, and assert the RUNTIME element type with a
    real `instanceof` check, not the static type — e.g. `JsValue::from(chunk.clone())`
    `.is_instance_of::<js_sys::Uint8Array>()` (JsCast's `is_instance_of` performs a JS
    `instanceof`). Also assert concatenated bytes equal the input (via `.to_vec()`) and the
    empty-input case. If js-sys is only a regular dep, tests can use it directly
- Docs: the `docs/howto/wasm.md` bullet currently says chunks are plain `number[]` with a
    `Uint8Array.from(chunk)` workaround — replace it with "returns an array of `Uint8Array` chunks"
    and drop the workaround. Grep `docs/`, `crates/iscc-wasm/README.md` and
    `crates/iscc-wasm/CLAUDE.md` for any other claim about the chunk element type
- Local WASM probe: `wasm-pack test --node crates/iscc-wasm --features conformance` (the CI line);
    `cargo test -p iscc-wasm` runs zero wasm tests (learnings). Binding-only change: no CRAP, iai or
    semver gate is affected

## Verification

- `wasm-pack test --node crates/iscc-wasm --features conformance` passes, including a test that
    asserts every chunk `is_instance_of::<Uint8Array>()` and the empty-input single empty chunk
- `cargo clippy -p iscc-wasm --all-targets -- -D warnings` (the CI host form) and
    `cargo clippy -p iscc-wasm --target wasm32-unknown-unknown -- -D warnings` are both clean
- `grep -n "to_value" crates/iscc-wasm/src/lib.rs` prints nothing (no serde-serialized return left)
- `docs/howto/wasm.md` contains no `number[]` / `Uint8Array.from(chunk)` text for `alg_cdc_chunks`,
    and `uv run zensical build` exits 0
- `mise run check` exits 0

## Done When

All verification criteria pass and `@iscc/wasm`'s `alg_cdc_chunks` hands JS callers `Uint8Array`
chunks, pinned by a runtime `instanceof` test.
