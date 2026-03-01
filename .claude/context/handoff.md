## 2026-03-01 — Add gen_sum_code_v0 to WASM bindings

**Done:** Added `WasmSumCodeResult` struct and `gen_sum_code_v0` function to the WASM binding crate.
The function accepts `&[u8]` (maps to `Uint8Array` in JS), feeds both `DataHasher` and
`InstanceHasher` from the same byte slice, then composes the ISCC-CODE via `gen_iscc_code_v0`. Uses
`f64` for filesize (maps to JS `number`, avoids BigInt friction). Added 6 tests covering
equivalence, result shape, empty input, default params, wide mode, and filesize.

**Files changed:**

- `crates/iscc-wasm/src/lib.rs`: Added `WasmSumCodeResult` struct with `getter_with_clone` and
    `gen_sum_code_v0` function after `gen_iscc_code_v0`. Used `f64` for filesize per spec (not `u64`
    as the define-next scaffold had).
- `crates/iscc-wasm/tests/unit.rs`: Added 6 `wasm_bindgen_test` tests for `gen_sum_code_v0`
    (equivalence, result shape, empty input, default params, wide mode, filesize).

**Verification:**

- `cargo build -p iscc-wasm --target wasm32-unknown-unknown` — compiles clean
- `cargo clippy -p iscc-wasm -- -D warnings` — clean
- `wasm-pack test --node crates/iscc-wasm` — 75 tests pass (9 conformance + 66 unit, of which 6 are
    new gen_sum_code_v0 tests)
- `gen_sum_code_v0` exported and returns `WasmSumCodeResult` with `iscc`, `datahash`, `filesize`
    fields
- `mise run check` — all 14 hooks pass
- `cargo test -p iscc-lib` — 310 tests pass (no regressions)

**Next:** Propagate `gen_sum_code_v0` to C FFI bindings (`crates/iscc-ffi/`). After that, Java and
Go remain. Go will need a pure reimplementation (not a Rust wrapper).

**Notes:** The `define-next` agent scaffolded the implementation with `u64` for `filesize`, which
maps to `BigInt` in JS. Changed to `f64` per the spec in next.md — `f64` maps to JS `number` and
handles files up to 2^53 bytes (~9 PB), far beyond any browser WASM use case. Issue #15 progress:
Rust core ✅, Python ✅, Node.js ✅, WASM ✅. Three bindings remain: C FFI, Java, Go.
