# Next Work Package

## Step: Range-check every JS integer argument, reject negative JNI counts, document the Python/Ruby classes

## Goal

Fixes the `normal` `[audit]` issue "Out-of-range integer arguments wrap silently on JS, raise
undocumented classes elsewhere". Today napi and WASM silently turn an out-of-range JS number into a
different valid integer, so the caller gets a wrong code back with no error. After this step every
binding rejects such input, and the docs name the error class each binding actually raises.

## Alternatives Considered

- **Chosen:** integer range — silent wrong output is a correctness bug. It touches 3 non-test code
    files (fits the 6-file default, no audit valve needed) and the handoff points here
- **Rejected:** "Published-package docs contradict what ships" — docs only, ~11 files. It is the
    last blocker and comes next; this step's doc edits (python.md, ruby-api.md) do not overlap it

## Scope

- **Modify**: `crates/iscc-napi/src/lib.rs`, `crates/iscc-wasm/src/lib.rs`,
    `crates/iscc-jni/src/lib.rs`; docs: `docs/howto/python.md` (Error handling), `docs/ruby-api.md`
    (error statement near line 82). Also any napi/WASM/Java doc page or crate `CLAUDE.md` that
    describes the IDv1-only `checked()` rule or integer error behaviour (grep for `checked` and
    `ValueError`/`RuntimeError`)
- **Tests**: `crates/iscc-napi/__tests__/functions.test.mjs`, `crates/iscc-wasm/tests/unit.rs`,
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`, a Python test under
    `tests/`, `crates/iscc-rb/test/test_array_args.rb`
- **Reference**: `checked()` in napi (lib.rs ~330) and WASM (~378); decision 2026-07-29 "JS-number
    ISCC-IDv1 surfaces take all params as `f64` and validate before narrowing"; `narrow_to_i32` in
    `crates/iscc-rb/src/lib.rs`; JNI `encode_component` range checks (~755)

## Not In Scope

- No `data-edge.json` vector. A value like `2**32+64` for `bits` cannot be passed through the typed
    signatures of the Rust, C, C++, Go, .NET, Swift and Kotlin harnesses. To them it is a wrong
    argument type, which `edge-vectors.md` → Rules leaves to per-binding tests. Append a short
    decisions.md entry that records this reading
- Python and Ruby keep their exceptions (`OverflowError`, `RangeError`). Each is the class the host
    language raises itself when an integer conversion fails. Fix the docs, not the bindings
- JNI `alg_minhash_256` keeps reading `int[]` elements as unsigned (`v as u32`, the commented Java
    idiom). Its `avg_chunk_size`, `encode_component` and `gen_iscc_id_v1` checks already exist
- WASM `gen_video_code_v0` / `soft_hash_video_v0` deserialize through `serde_wasm_bindgen`, which
    already rejects non-safe and out-of-range integers. Leave them alone unless a probe shows
    wrapping
- Go, C FFI, C++, .NET, Swift and Kotlin take typed unsigned/signed integers and are unaffected. Do
    not touch the core

## Implementation Notes

- **Why it wraps:** napi converts `u8`/`u32` with `napi_get_value_uint32` and `i32` with
    `napi_get_value_int32` (napi-3.11.0 `js_values/number.rs:63-69`). Both apply ToUint32/ToInt32,
    so `256` becomes `u8` 0 and `2**40` becomes `i32` 0. The wasm-bindgen glue does `>>> 0` for
    `u32` and copies `Vec<i32>`/`Vec<u32>` into typed arrays, which wrap the same way
- **Fix on both JS surfaces (finish the pattern):** every numeric parameter of an exported fn or
    method becomes `f64` / `Option<f64>` / `Vec<f64>` / `Vec<Vec<f64>>` and is narrowed through one
    range-checked helper. Generalise the existing `checked` to take a minimum, or add thin
    `to_u8`/`to_u32`/`to_i32` wrappers over it, and keep `gen_iscc_id_v1` on the same helper.
    Bounds: `u8` \[0, 256), `u32` \[0, 2^32), `i32` \[-2^31, 2^31); reject non-finite and fractional
    values. `-0.0` maps to 0. `undefined` stays `None`. Sites: `bits` on every gen fn and on the
    streaming `finalize`/`finalize_code`, `gen_sum_code_v0` `bits`, `text_trim` `nbytes`,
    `sliding_window` `width`, `alg_cdc_chunks` `avg_chunk_size`, `encode_component`
    (`mtype`/`stype`/`version`/`bit_length`), and the elements of `gen_audio_code_v0` `cv`,
    `alg_minhash_256` `features` and napi's `gen_video_code_v0`/`soft_hash_video_v0` frames. Name
    the parameter in the error message, and an element by its index
- Constants and result fields (`META_TRIM_*`, `IsccDecodeResult`) are outputs; leave them. The napi
    `.d.ts` is generated, not committed: `f64` and `u32` both map to `number`
- `#[wasm_bindgen_test]` bypasses the JS glue (learnings 234). Prove the WASM fix from Node as well:
    `wasm-pack build crates/iscc-wasm --target nodejs --out-dir /tmp/wasm-int` and call it
- **JNI:** one helper that rejects a negative `jint` with `IllegalArgumentException` (via
    `throw_and_default`) before the unsigned cast. Use it at every `bits as u32` site (all gen fns,
    `gen_sum_code_v0` and any hasher finalize), `text_trim` `nbytes` and `sliding_window` `width`
- Tests: per surface, one negative case and one `2**32 + 64`-style case (where the host type can
    hold it) per parameter kind. Assert on the raw call (learnings: one bad case per call)
- Binding-only change: CRAP and iai gates are not affected

## Verification

- `cd crates/iscc-napi && npm run build:debug && npm test` passes. New tests show that `bits`
    `2**32+64` and `-1`, `text_trim(…, -1)`, `gen_audio_code_v0([2**40])`, `alg_minhash_256([-1])`,
    a video frame holding `2**31`, `encode_component(256, …)` and `bits` `64.5` each throw
- `wasm-pack test --node crates/iscc-wasm` passes with the same cases in `tests/unit.rs`, and a Node
    call on the built `/tmp/wasm-int` package shows `gen_text_code_v0("x", 2**32+64)` and
    `gen_audio_code_v0([2**40])` throwing
- `grep -nE '(\(|^\s+)[a-z_]+: (Option<)?(u8|u32|i32|Vec<(Vec<)?(i32|u32)>)[,)>]' crates/iscc-napi/src/lib.rs crates/iscc-wasm/src/lib.rs`
    prints nothing
- `mvn -f crates/iscc-jni/java/pom.xml clean test` passes, with tests showing that negative `bits`,
    `nbytes` and `width` each raise `IllegalArgumentException`
- `uv run pytest` passes with a test pinning `OverflowError` for `bits=-1` and `bits=2**32+64`, and
    `bundle exec rake test` (in `crates/iscc-rb`) passes with a test pinning `RangeError`. The two
    doc pages name these classes
- `cargo clippy --workspace --all-targets -- -D warnings` and `mise run check` exit 0

## Done When

All six verification criteria pass. Every out-of-range integer throws on napi, WASM and JNI, and the
Python and Ruby docs name the exception class each binding actually raises.
