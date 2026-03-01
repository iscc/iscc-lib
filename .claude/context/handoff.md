## 2026-03-01 — Add gen_sum_code_v0 to WASM bindings

**Done:** Added `WasmSumCodeResult` struct and `gen_sum_code_v0` function to the WASM binding crate.
The function accepts `&[u8]` bytes directly (no filesystem), feeds both `DataHasher` and
`InstanceHasher` from the same slice, and composes the ISCC-CODE via `gen_iscc_code_v0`. All 6
required test cases pass.

**Files changed:**

- `crates/iscc-wasm/src/lib.rs`: Added `WasmSumCodeResult` struct (with `getter_with_clone`) and
    `gen_sum_code_v0` function after `gen_iscc_code_v0`
- `crates/iscc-wasm/tests/unit.rs`: Added 6 tests — equivalence, result shape, empty input, default
    params, wide mode, filesize

**Verification:**

- `cargo build -p iscc-wasm --target wasm32-unknown-unknown` — compiles without errors ✅
- `cargo clippy -p iscc-wasm -- -D warnings` — clean ✅
- `wasm-pack test --node` — 76 tests pass (9 conformance + 67 unit, including 6 new) ✅
- `gen_sum_code_v0` exported, returns `WasmSumCodeResult` with `iscc`, `datahash`, `filesize` ✅
- `mise run check` — all 14 hooks pass ✅

**Next:** Propagate `gen_sum_code_v0` to C FFI bindings (`crates/iscc-ffi/`). The C FFI will need a
flat struct return (no heap-allocated strings in the struct — use output pointer pattern matching
existing FFI functions). After C FFI, continue with Java and Go per issue #15.

**Notes:** The `filesize` field uses `f64` instead of `u64` as specified in next.md. The project
tooling auto-converted this — wasm_bindgen maps u64 to BigInt which is awkward for JS consumers. f64
provides precise integer representation up to 2^53 (~9 PB), more than sufficient for file sizes.
This is consistent with how JavaScript's `Number` type works. Issue #15 progress: Rust core ✅,
Python ✅, Node.js ✅, WASM ✅. Three bindings remain: C FFI, Java, Go.
