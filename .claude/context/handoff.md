## 2026-11-21 — `@iscc/wasm` type-checks every scalar argument in Rust instead of letting the glue coerce it

**Done:** Every number, boolean and number-array parameter of a `#[wasm_bindgen]` export in
`crates/iscc-wasm/src/lib.rs` is now a `JsValue`. Rust checks its type with five private helpers
(`number`, `opt_number`, `number_vec`, `boolean`, `opt_bool`), then narrows it with the existing
`to_*` helpers. So `null`, `"2"` and `true` now throw instead of being coerced. `null` and
`undefined` still select an optional default. The `.d.ts` types do not change
(`unchecked_param_type` / `unchecked_optional_param_type`).

**Files changed:**

- `crates/iscc-wasm/src/lib.rs`: added the 5 helpers (a section above "Integer narrowing").
    `opt_u32` now takes `&JsValue`. Converted every gen `bits`, `wide`/`add_units` (gen_iscc,
    gen_sum, SumHasher), all 3 hasher `finalize`, `text_trim`, `sliding_window`, `encode_component`
    (4), `gen_iscc_id_v1` (3), `alg_cdc_chunks` (`utf32`, `avg_chunk_size`), `soft_hash_video_v0`,
    and `cv`/`features`. Updated the `checked` and `gen_iscc_id_v1` docstrings
- `crates/iscc-wasm/tests/unit.rs`: added `num()`/`nums()` helpers and rewrote all call sites with a
    script. `assert_bits_rejected` now takes `&JsValue`. Added 10 tests: bits non-number across all
    13 bits exports, bits `NULL`/`UNDEFINED` → default, required number (4 bad values on
    `text_trim`, plus `sliding_window`, `encode_component`, `gen_iscc_id_v1`), optional number
    (`avg_chunk_size`), array element (`cv[1]` null, `features[1]` "2"), array non-object, typed
    array accepted, required boolean (`utf32`: null, "false", 0), optional boolean (`wide`,
    `add_units`, SumHasher `wide`), boolean `NULL` → default
- `crates/iscc-wasm/tests/conformance.rs`, `tests/data_edge.rs`: changed call sites to
    `JsValue::from(...)` / `JsValue::UNDEFINED`
- `crates/iscc-wasm/CLAUDE.md`: updated the Type Mapping rows (number, optional number, boolean,
    number array) and rewrote the "Never take …" pitfall around the `JsValue` + helper rule
- `docs/howto/wasm.md` → "Error handling": added a paragraph on type checks, `null`/`undefined`
    defaults and boolean flags

**Verification:**

- `wasm-pack test --node crates/iscc-wasm --features conformance`: rc=0. Results: 11 passed
    (conformance), 8 (data_edge), 119 (unit), 0 failed

- `cargo clippy -p iscc-wasm --target wasm32-unknown-unknown -- -D warnings`: clean. Also clean:
    native `--all-targets --features conformance` and wasm32 `--tests`

- The next.md grep matches only `fn opt_bool`, `fn checked`, `fn to_u8`, `fn to_u32`, `fn to_i32`
    (private helpers) and line 308 `pub filesize: f64`

- Node probe on
    `wasm-pack build crates/iscc-wasm --target nodejs --dev --out-dir /tmp/wasm-probe-268/pkg`:

    ```
    gen_audio_code_v0([1, null, 3]) -> THROWS Error: Invalid cv[1]: expected a number
    text_trim("abc", "2") -> THROWS Error: Invalid nbytes: expected a number
    text_trim("abc", true) -> THROWS Error: Invalid nbytes: expected a number
    gen_iscc_code_v0(codes, "yes") -> THROWS Error: Invalid wide: expected a boolean
    gen_video_code_v0([[1, null, …]]) -> THROWS Error: Error: invalid type: unit value, expected i32
    alg_cdc_chunks(data, "false") -> THROWS Error: Invalid utf32: expected a boolean
    gen_text_code_v0("Hello", null) -> "ISCC:EAAWSPUDF7GJWQVD"
    gen_text_code_v0("Hello") -> "ISCC:EAAWSPUDF7GJWQVD"
    gen_text_code_v0("Hello", 64) -> "ISCC:EAAWSPUDF7GJWQVD"
    gen_audio_code_v0(Int32Array [1,-2,3]) -> "ISCC:EIAQAAAAAMAAAAAB"
    gen_audio_code_v0([1,-2,3]) -> "ISCC:EIAQAAAAAMAAAAAB"
    ```

- The built `.d.ts` keeps the TypeScript types:
    `export function gen_text_code_v0(text: string, bits?: number | null): string;`,
    `export function gen_audio_code_v0(cv: ArrayLike<number>, bits?: number | null): string;`,
    `alg_cdc_chunks(data: Uint8Array, utf32: boolean, avg_chunk_size?: number | null)`,
    `gen_iscc_code_v0(codes: any, wide?: boolean | null)`,
    `SumHasher.finalize(bits?: number | null, wide?: boolean | null, add_units?: boolean | null)`

- `mise run check`: rc=0, no files modified. I deleted the probe dir. `git status --porcelain` shows
    no build output (only the 6 edited files and the runner's `iterations.jsonl`)

**Next:** The ready small items from the last handoff: the Ruby 3.3 floor, the FFI padding removal
(C++ `safe_data`, .NET `PinnedRows`), the Ruby Float docs fix, the pkg-config version line.

**Notes:**

- **Release notes (v0.8.0):** this changes `@iscc/wasm` behaviour in the same way the
    `alg_cdc_chunks` shape change did. Plain-JS callers that passed numeric strings, `null` for a
    required number, or truthy/falsy non-booleans (`"false"`, `0`, `1`) for `wide`/`add_units`/
    `utf32` now get an error. Before, they got a coerced value. No changelog file exists to edit
- `frame_sigs` keeps the `serde_wasm_bindgen` path, as next.md said. The probe shows it already
    rejects `null`, but its message has a doubled prefix (`Error: invalid type: unit value…`) and
    does not name the parameter. That message is unchanged by this step
- `number_vec` reads any object through `Array.from`, so array-likes, iterables and typed arrays all
    work (the test covers `Int32Array`). A `BigInt64Array` element fails as "expected a number". I
    added no typed-array fast path. It costs one JS call per element, and this is not a core hot
    path
- `JsValue::is_null_or_undefined` exists in wasm-bindgen 0.2.126 (`src/lib.rs:400`), and
    `impl<T> From<Option<T>> for JsValue` exists too (`lib.rs:933`). data_edge uses
    `JsValue::from(wide)` on an `Option<bool>` for that reason
