# Next Work Package

## Step: Mint `gen_iscc_id_v1` on WASM + settle JS-number input validation (napi & wasm)

## Goal

Add the experimental `gen_iscc_id_v1` minting export to `crates/iscc-wasm` (32→33 Tier 1 symbols)
and settle the JS-number input-validation approach for the whole #43 JS-number fan-out, applying it
to both the wasm and napi surfaces. This advances #43 ("ISCC-IDv1 unsupported outside Go") and
resolves the `normal` issue "JS-number IDv1 minting silently coerces invalid inputs instead of
throwing".

## Alternatives Considered

- **Chosen:** wasm minting + JS-number validation on both wasm & napi — the handoff explicitly
    routes wasm next *because* the validation approach must be settled here before it is re-decided
    per surface; napi already carries the open coercion issue, so fixing both while the design is
    fresh is coherent and within the 3-file budget.
- **Rejected:** the ffi minting slice — ffi is not a JS-number surface (no coercion hazard) and
    needs an `iscc.h` regen + freshness gate; it can follow once the JS-number question is closed.

## Scope

- **Modify**: `crates/iscc-wasm/src/lib.rs` (add `gen_iscc_id_v1` with validation),
    `crates/iscc-napi/src/lib.rs` (retrofit the same validation)
- **Tests (outside budget)**: extend `crates/iscc-wasm/tests/unit.rs` (or a new tests file) and
    `crates/iscc-napi/__tests__/iscc_id_v1.test.mjs` with the low/non-integral/non-finite cases
- **Reference**: `crates/iscc-napi/src/lib.rs:313` (current napi export),
    `crates/iscc-wasm/src/lib.rs` `iscc_decode`/`IsccDecodeResult` (`version: u8`), core
    `crates/iscc-lib/src/lib.rs` `gen_iscc_id_v1` (high-end validation: ts≥2^52, hub≥2^12,
    realm∉{0,1}), issues.md JS-number entry

## Not In Scope

- The remaining minting surfaces (ffi, jni, rb, uniffi→Swift/Kotlin, dotnet, cpp) — separate steps.
- The Tier-1 32→33 doc/count sweep (wasm CLAUDE.md/README, `docs/`, etc.) — its own #43 slice.
- No version-enum widening: wasm & napi `iscc_decode` already return `version: u8`, so
    `iscc_decode(gen_iscc_id_v1(...))` already round-trips.
- Changing the core `gen_iscc_id_v1` signature or its high-end validation.

## Implementation Notes

- **Take all three params as `f64`** on both surfaces (`timestamp`, `hub_id`, `realm`). napi/wasm
    coerce JS numbers into `u16`/`u8` *before* our code runs (ToUint32/truncation), so integer param
    types make the low/non-integral end unvalidatable — hence `f64` for all three. This changes the
    napi signature from `(f64, u16, u8)`; `index.d.ts`/`index.js` are napi-build- generated — do not
    hand-edit them.
- Validate each param before narrowing: reject non-finite (`!x.is_finite()`), non-integral
    (`x.fract() != 0.0`), and out-of-range with a thrown error. Ranges: `0.0 ≤ timestamp < 2^52`,
    `0.0 ≤ hub_id < 4096.0`, `realm ∈ {0.0, 1.0}` (i.e. `0.0 ≤ realm < 2.0` with integrality). A
    small local helper `fn checked(value: f64, max_exclusive: f64, name: &str) -> Result<..>` in
    each file is fine (duplicating a ~4-line fn across two surfaces is acceptable).
- After validation, narrow with `as u64/u16/u8` and delegate to
    `iscc_lib::gen_iscc_id_v1(ts, hub, realm)`; map its error too (core re-checks the high end).
- wasm returns `String` (bare, mirroring napi — `.map(|r| r.iscc)`); errors via `JsError::new`.
- Golden (both surfaces): `gen_iscc_id_v1(1751831876325218, 1, 0) == "ISCC:MAIGHFECJMOPMIAB"`.
- wasm tests are Rust `#[wasm_bindgen_test]` — pass `-1.0`, `f64::NAN`, `0.9` and assert
    `.is_err()`; napi `.mjs` — pass `-1`, `NaN`, `0.9`, `2**32` and assert `throws(...)`. Keep the
    existing napi high-end cases (ts=2^52, hub=4096, realm=2) — they still throw under the new
    validation.

## Verification

- `wasm-pack test --node crates/iscc-wasm --features conformance` passes, incl. new IDv1 golden,
    validation-throw, and `iscc_decode` round-trip (maintype 6 / version 1 / subtype 0) cases.
- `cd crates/iscc-napi && npm run build:debug && npm run test` passes, incl. new
    low/non-integral/non-finite throw cases plus the retained high-end cases.
- `cargo clippy -p iscc-wasm -p iscc-napi --all-targets -- -D warnings` clean.
- `cargo fmt -p iscc-wasm -p iscc-napi --check` clean.
- `iscc_wasm::gen_iscc_id_v1` is exported (wasm now at 33 Tier 1 symbols); on both surfaces a
    non-finite / non-integral / negative / out-of-range input throws instead of minting a wrong ID.
- `mise run check` — all prek pre-commit hooks pass.

## Done When

All verification criteria pass: wasm mints `gen_iscc_id_v1` (33 symbols) and both JS-number surfaces
reject non-finite, non-integral, and out-of-range inputs with a thrown error covered by tests.
