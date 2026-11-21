## 2026-11-21 — Range-check every JS integer argument, reject negative JNI counts, document the Python/Ruby classes

**Done:** napi and WASM now take every numeric input as `f64` (`Option<f64>`, `Vec<f64>`, napi
`Vec<Vec<f64>>`) and narrow it through one range-checked helper set (`checked` with a minimum, plus
`to_u8`/`to_u32`/`to_i32`/`opt_u32`/`to_*_vec`, napi `to_i32_rows`). `gen_iscc_id_v1` uses the same
`checked`. JNI gained `non_negative`, used at all 12 `bits` sites and for `nbytes`/`width`. The
Python and Ruby docs now name `OverflowError` / `RangeError`, and decisions.md records why no
`data-edge.json` vector covers this.

**Files changed:**

- `crates/iscc-napi/src/lib.rs`: helpers at the top; every gen fn, `gen_sum_code_v0`, hasher
    `finalize`, `text_trim`, `sliding_window`, `alg_cdc_chunks`, `encode_component`,
    `alg_minhash_256`, audio `cv`, video/soft-hash frames. `text_trim` and `alg_minhash_256` now
    return `napi::Result`
- `crates/iscc-wasm/src/lib.rs`: same helpers (no rows helper: video stays on `serde_wasm_bindgen`);
    `SumHasher.finalize` included. `cv`/`features` carry
    `#[wasm_bindgen(unchecked_param_type = "ArrayLike<number>")]`
- `crates/iscc-jni/src/lib.rs`: `non_negative<T: Default>(env, jint, name) -> Result<u32, …>`
- Tests: napi `functions.test.mjs` (+3 describe blocks, 52 cases), WASM `tests/unit.rs` (+16 tests;
    `conformance.rs`/`data_edge.rs`/older unit calls moved to `f64` args), `IsccLibTest.java` (+2
    tests, message asserted), `tests/test_int_range.py` (new, 6), `test_array_args.rb` (+3)
- Docs: `docs/howto/python.md`, `docs/ruby-api.md` (line 82 + Error Handling),
    `docs/howto/{nodejs,wasm,java}.md` error sections, napi/WASM `CLAUDE.md` type tables + pitfall
- `.claude/context/decisions.md`: entry "Out-of-range integer arguments are per-binding tests"

**Verification:**

- [x] `npm run build:debug && npm test` (iscc-napi): 238/238 pass. Every listed case throws with a
    message naming the parameter (`Invalid bits: 4294967360`, `Invalid cv[1]: …`,
    `Invalid frame_sigs[1][1]: 2147483648`, `Invalid mtype: 256`, `Invalid bits: 64.5`)
- [x] `wasm-pack test --node crates/iscc-wasm`: 11 + 8 + 108 pass (re-run after `cargo fmt`). Node
    probe on `/tmp/wasm-int`: `gen_text_code_v0("x", 2**32+64)` → `Invalid bits: 4294967360`,
    `gen_audio_code_v0([2**40])` → `Invalid cv[0]: 1099511627776`; `Int32Array`/`Uint32Array` inputs
    still give the same result as plain arrays
- [x] The signature grep from next.md prints nothing
- [x] `mvn -f crates/iscc-jni/java/pom.xml clean test`: 110 + 8 pass (after
    `cargo build -p iscc-jni`)
- [x] `uv run pytest`: 769 pass; `bundle exec rake test`: 168 runs, 0 failures
- [x] `cargo clippy --workspace --all-targets -- -D warnings` exit 0; also
    `cargo clippy -p iscc-wasm --target wasm32-unknown-unknown --all-targets` clean;
    `mise run check` exit 0

**Next:** the published-docs issue ("Published-package docs contradict what ships"), as next.md
says.

**Notes:**

- **TypeScript surface of `@iscc/wasm`:** a plain `Vec<f64>` would have changed the `.d.ts` from
    `Int32Array`/`Uint32Array` to `Float64Array`. Existing TS callers passing a typed array would
    then fail to compile. `unchecked_param_type = "ArrayLike<number>"` makes the type wider instead
    (plain arrays and every typed array are accepted). Everything else is `number` before and after.
    The napi `.d.ts` is unchanged (`Array<number>`, `number`)
- **Behaviour choices:** the `bits` check runs before the hasher is consumed, so a rejected
    `finalize(-1)` leaves the hasher usable (tested on napi, WASM and JNI). A core-level rejection
    such as `bits=13` still consumes it, as before. `-0` maps to 0. `null`/`undefined` keep the
    default
- WASM video: the Node probe confirms that `serde_wasm_bindgen` already rejects `2**31`
    (`invalid value: integer 2147483648, expected i32`). Left as is
- JNI `genSumCodeV0("unused", -1, …)` checks `bits` before reading the path, so the test needs no
    file
- Helpers are duplicated between napi and WASM (different error types); the old `checked` was
    duplicated the same way
- No hot path in the core changed. The napi/WASM element conversion adds one pass over `cv` /
    `features`; not benchmarked (bindings are outside iai/CRAP)
