# Next Work Package

## Step: `@iscc/wasm` type-checks every scalar argument in Rust instead of letting the glue coerce it

## Goal

Resolve the issue "`@iscc/wasm` converts non-number values to numbers instead of rejecting them"
with the code branch: every numeric (and boolean) parameter of `crates/iscc-wasm/src/lib.rs` arrives
as a `JsValue` and is type-checked in Rust, so `null`, `"2"` and `true` throw like they do in
`@iscc/lib` instead of silently producing a different valid code.

## Alternatives Considered

- **Chosen:** code fix. Plain JavaScript callers get a wrong result without an error today (an audio
    vector with `null` hashes as `0`). That breaks the conformance-parity promise, and a docs
    sentence cannot fix it. wasm-bindgen 0.2.126 has `unchecked_optional_param_type` (read in
    `wasm-bindgen-macro-support-0.2.126/src/parser.rs`), so the `.d.ts` keeps `bits?: number | null`
    and TypeScript callers see no change.
- **Rejected:** docs-only branch (state the coercion in `docs/howto/wasm.md`). It is cheaper but
    keeps a silent wrong-result path that napi does not have.
- **Rejected for now:** Ruby 3.3 floor and FFI padding removal. Both are ready and small, but they
    rank below a correctness item.
- **Booleans included:** the glue maps `wide`/`add_units`/`utf32` by truthiness (`"false"` → `true`,
    seen in the `pkg/` glue). That is the same "scalar coerced in glue" pattern on the same surface,
    so the pattern rule puts it in this step.

## Scope

- **Modify**: `crates/iscc-wasm/src/lib.rs`, `crates/iscc-wasm/CLAUDE.md` (Type Mapping table),
    `docs/howto/wasm.md` (→ "Error handling")
- **Tests (outside budget)**: `crates/iscc-wasm/tests/unit.rs`, `tests/conformance.rs`,
    `tests/data_edge.rs`, with call sites adapted to the new signatures
- **Reference**: `~/.cargo/registry/src/*/wasm-bindgen-macro-support-0.2.126/src/parser.rs`
    (attributes, lines 120-130 and 1615-1705); `serde-wasm-bindgen-0.6.5/src/de.rs:225-301` (its
    `as_f64()` strictness is the model)

## Not In Scope

- `frame_sigs`, `codes`, `units`, `hash_digests` (the `serde_wasm_bindgen` path). It already rejects
    non-numbers through `as_f64()`. Leave it unchanged unless the Node probe shows otherwise.
- Byte inputs (`&[u8]`/`Uint8Array`) and string inputs. Do not change them.
- Changing range rules, error wording of the existing `Invalid <name>: <value>` messages, or napi.
- A `data-edge.json` vector (`edge-vectors.md` → Rules: wrong argument types are per-binding).

## Implementation Notes

- **Parameters to convert.** Do every `f64`, `Option<f64>`, `bool`, `Option<bool>` parameter and
    both `Vec<f64>` parameters (`cv`, `features`). That covers every gen `bits`, the three hasher
    `finalize` methods, `text_trim`, `sliding_window`, `encode_component` (4), `gen_iscc_id_v1` (3),
    `alg_cdc_chunks` (`utf32`, `avg_chunk_size`), `gen_sum_code_v0`, `gen_iscc_code_v0` and
    `soft_hash_video_v0`. Grep for them, and leave no `f64`/`bool` parameter on any
    `#[wasm_bindgen]` export.
- **Signatures.** A required scalar becomes `JsValue` with
    `#[wasm_bindgen(unchecked_param_type =   "number")]` (or `"boolean"`). An optional one becomes
    `JsValue` with `unchecked_optional_param_type = "number | null"`. Optional parameters must stay
    trailing (the macro errors otherwise). Arrays get `unchecked_param_type = "ArrayLike<number>"`.
- **Helpers** (short, documented, next to `checked`): a `number(&JsValue, name) -> Result<f64>` that
    uses `as_f64()` and errors as `Invalid {name}: expected a number`. An optional variant treats
    `undefined` **and** `null` as absent, which is today's `isLikeNone` behaviour, so a `null`
    `bits` still means the default. Add matching boolean helpers (`as_bool()`). The existing
    `to_u8`/`to_u32`/`to_i32`/`opt_u32` then take the checked `f64`. Do not duplicate them.
- **Arrays.** Reject a non-object. Read the rest via `js_sys::Array::from(&value)` and check each
    element with `as_f64()`, naming it `cv[i]`. The existing `to_i32_vec`/`to_u32_vec` index-naming
    style is the model. A typed-array fast path (`Float64Array::new(&v).to_vec()` when
    `is_instance_of` a typed array) is optional, so keep it only if it stays short.
- `wasm_bindgen_test` calls the Rust export directly. With `JsValue` parameters there is no glue
    conversion left, so `JsValue::NULL`, `JsValue::from_str("2")` and `JsValue::TRUE` in `unit.rs`
    exercise the real check. Add one rejection test per parameter class: required number, optional
    number, number array element, required boolean, optional boolean. Add a positive test that
    `JsValue::NULL`/`UNDEFINED` for `bits` gives the default. A small test helper like `num(64.0)`
    keeps the call-site rewrite readable.
- `docs/howto/wasm.md` → "Error handling": state that a non-number (incl. `null` for a required
    number, numeric strings, booleans) throws, that `null`/`undefined` select an optional default,
    and that boolean flags must be booleans. Update the CLAUDE.md table rows for `f64`,
    `Option<f64>`, `Option<bool>` and `Vec<f64>`.
- The behaviour change belongs in the v0.8.0 release notes, like the `alg_cdc_chunks` shape change.
    Say so in the handoff. No changelog file exists to edit.
- Not a hot path of the core, so the CRAP and iai gates do not apply. `iscc-wasm` sits outside both.

## Verification

- `wasm-pack test --node crates/iscc-wasm --features conformance` passes, including the new
    rejection and default tests
- `cargo clippy -p iscc-wasm --target wasm32-unknown-unknown -- -D warnings` clean
- `grep -nE '[( ][a-z_]+: (Option<)?(f64|bool|Vec<f64>)\b' crates/iscc-wasm/src/lib.rs` prints only
    lines of private helpers (`fn checked`, `to_*`, `opt_*`, …) and the `filesize` result field
    (today most matches are export parameters)
- Node probe on a `wasm-pack build --target nodejs` build outside the tracked tree, with output
    quoted in the handoff: `gen_audio_code_v0([1, null, 3])`, `text_trim("abc", "2")`,
    `text_trim("abc", true)`, `gen_iscc_code_v0(codes, "yes")` and
    `gen_video_code_v0([[1, null, …]])` each throw; `gen_text_code_v0("Hello", null)` and
    `gen_text_code_v0("Hello")` return the 64-bit default
- The built `.d.ts` still declares `bits?: number | null` on `gen_text_code_v0` and
    `cv: ArrayLike<number>` on `gen_audio_code_v0` (quote both lines)
- `mise run check` exits 0

## Done When

All six verification criteria pass and `git status --porcelain` shows no build output under the
tracked tree.
