# Next Work Package

## Step: Add streaming `SumHasher` to the WASM bindings

## Goal

Expose a `SumHasher` streaming class in `@iscc/wasm` over the shared core
`iscc_lib::streaming::SumHasher`, so WASM consumers can produce an ISCC-SUM code from a chunked
stream in a single pass instead of running two hashers and crossing the JS→WASM boundary twice per
chunk. This closes the **only remaining half of issue #37** ("Add streaming `SumHasher` to WASM
bindings"); the core struct (iteration 88) and the Python wrapper (iteration 89) already exist.

## Scope

- **Modify**: `crates/iscc-wasm/src/lib.rs` — add a `#[wasm_bindgen]` `SumHasher` struct in the
    "Streaming hashers" section, mirroring the existing `DataHasher`/`InstanceHasher`
    `Option<inner>` finalize-once pattern. `finalize` returns the existing `WasmSumCodeResult`. (1
    code file.)
- **Modify** (tests, excluded from file limit): `crates/iscc-wasm/tests/unit.rs` — add
    `#[wasm_bindgen_test]` tests for the new class.
- **Modify** (docs, excluded from file limit): `docs/howto/wasm.md` — add a `### SumHasher`
    subsection to the existing `## Streaming` section. `crates/iscc-wasm/CLAUDE.md` — bump the "2
    streaming types" enumeration to "3 streaming types" (`DataHasher`, `InstanceHasher`,
    `SumHasher`).
- **Reference**: `crates/iscc-lib/src/streaming.rs` (core `SumHasher` API, lines 150–210),
    `crates/iscc-wasm/src/lib.rs` (existing `gen_sum_code_v0` + `WasmSumCodeResult` at 163–224 and
    `DataHasher`/`InstanceHasher` at 421–523), `crates/iscc-wasm/tests/unit.rs` (existing
    `gen_sum_code_v0` and `DataHasher`/`InstanceHasher` test patterns), `crates/iscc-py/src/lib.rs`
    (`PySumHasher`, the analogous wrapper from iteration 89).

## Not In Scope

- **Do NOT promote `SumHasher` to a crate-root Tier 1 re-export** or bump the documented "32 Tier 1
    symbols" / "2 streaming types → 3" counts in `README.md`, `.claude/context/specs/rust-core.md`,
    `target.md`, or the core crate's tier docs. `SumHasher` is a Python/WASM streaming convenience
    reachable via `iscc_lib::streaming::SumHasher` (the `streaming` module is already `pub mod`); it
    is intentionally not bound in all languages. Only the WASM crate's own `CLAUDE.md` enumeration
    is updated.
- Do NOT touch the core `iscc-lib` crate, the Python bindings, or any other binding crate.
- Do NOT introduce a new result struct — reuse the existing `WasmSumCodeResult`.
- Do NOT take on the other backlog items (npm `optionalDependencies` #38, PyO3 0.23→0.29, GIL
    release #39, semver-checks / iai-callgrind / coverage gates) — those are separate steps.

## Implementation Notes

- Add the `SumHasher` struct after `InstanceHasher` (after line 523 in
    `crates/iscc-wasm/src/lib.rs`). Hold `inner: Option<iscc_lib::streaming::SumHasher>` — use the
    **full path** `iscc_lib::streaming::SumHasher` (there is no crate-root re-export, unlike
    `iscc_lib::DataHasher`).
- Provide `#[wasm_bindgen(constructor)] new()`, a `Default` impl that delegates to `new()` (matches
    the existing two hashers and avoids the clippy `new_without_default` lint), and
    `update(&mut self, data: &[u8]) -> Result<(), JsError>` that errors with
    `"SumHasher already finalized"` when `inner` is `None`.
- `finalize(&mut self, bits: Option<u32>, wide: Option<bool>, add_units: Option<bool>) ->   Result<WasmSumCodeResult, JsError>`:
    `self.inner.take()` (erroring if `None`), then call the core
    `hasher.finalize(bits.unwrap_or(64), wide.unwrap_or(false), add_units.unwrap_or(false))`. Map
    the core `SumCodeResult` into `WasmSumCodeResult`, converting `filesize: u64` to `f64` (cast
    `as f64`, exactly as the existing `gen_sum_code_v0` does at lib.rs:221). `units` maps
    `Option<Vec<String>>` directly.
- Map all errors with `.map_err(|e| JsError::new(&e.to_string()))`. Never panic across the WASM
    boundary (per the crate's CLAUDE.md).
- This is a translation layer only — no algorithm logic. The single-pass composition already lives
    in the core `SumHasher`.
- Tests: mirror the existing `gen_sum_code_v0` and `DataHasher` tests. Add at least 8
    `#[wasm_bindgen_test]` functions covering: (1) streamed `SumHasher` output equals
    `iscc_wasm::gen_sum_code_v0(data, ...)` for the same bytes; (2) multi-`update()` split equals a
    single `update()`; (3) empty input; (4) result shape (non-empty `iscc`, non-empty `datahash`,
    correct `filesize`); (5) `add_units=Some(true)` yields two unit strings; (6) `add_units`
    default/`None` yields `units == None`; (7) wide mode at 128-bit differs from narrow; (8)
    finalize-once: a second `finalize()` errors and `update()` after `finalize()` errors. Use real
    byte data, not mocks.
- Docs: in `docs/howto/wasm.md`, add a `### SumHasher` subsection after `### InstanceHasher` (around
    line 302) showing `import { SumHasher } from "@iscc/wasm"`, chunked `update()`, and
    `const result = hasher.finalize();` reading `result.iscc` / `result.units`. Keep the same
    `javascript` fenced-block style as the neighbouring examples.

## Verification

- `wasm-pack test --node crates/iscc-wasm` passes (all existing unit + conformance tests plus ≥8 new
    `SumHasher` tests).
- `cargo clippy -p iscc-wasm -- -D warnings` clean.
- `cargo fmt -p iscc-wasm --check` clean.
- `grep -q "pub struct SumHasher" crates/iscc-wasm/src/lib.rs` exits 0 (class is defined).
- `grep -q "SumHasher" docs/howto/wasm.md` exits 0 (howto documents the new class).
- The documented "32 Tier 1 symbols" count is unchanged in `README.md`,
    `.claude/context/specs/rust-core.md`, and `target.md` (SumHasher is not promoted to Tier 1).

## Done When

`wasm-pack test --node crates/iscc-wasm` is green with the new `SumHasher` tests, clippy and fmt are
clean, and the WASM howto documents the `SumHasher` class — fully closing issue #37 across all
bindings.
