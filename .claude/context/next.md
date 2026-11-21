# Next Work Package

## Step: Repair the `@iscc/wasm` type checks — optional `gen_meta_code_v0` params and exception-safe number arrays

## Goal

Resolve the `critical` issue "`@iscc/wasm` type checks broke a TypeScript signature and let a JS
exception kill the instance" so the unpushed iteration-268 step can pass review and reach CI. This
reframes the number-array read rather than reverting the type checks.

## Alternatives Considered

- **Chosen:** repair both defects on the WASM surface. A `critical` issue comes before everything
    else, and the type-check work is done apart from these two defects.
- **Rejected:** revert `8560f6a` and go back to the docs-only branch of the coercion issue. That
    throws away verified work, and both defects have small, well-understood fixes.

## Scope

- **Modify**: `crates/iscc-wasm/src/lib.rs`, `docs/howto/wasm.md`
- **Modify (tests)**: `crates/iscc-wasm/tests/unit.rs`
- **Reference**: `~/.cargo/registry/src/*/js-sys-0.3.103/src/lib.rs` (`Reflect::get` :7627,
    `Reflect::get_u32` :7659, both `catch`; `try_iter` :5531), the critical entry in issues.md

## Not In Scope

- Byte and string parameter conversion (`&[u8]`, `&str`): this is the `low`, untriaged issue. Do not
    convert `name`, `text` or byte inputs to `JsValue`
- The `serde_wasm_bindgen::from_value` paths (`frame_sigs`, `codes`, `units`, `hash_digests`). Its
    `Array` fast path reads elements through getters that have no `catch`. This is older than
    iteration 268 and not part of the critical issue. Do not rewrite it. If a probe shows a throwing
    element kills the instance, report it in the handoff
- The CI `.d.ts` declaration-diff gate (a review idea for a later step)
- Any other binding, and the decisions.md entry about the element-wise read (review writes it)

## Implementation Notes

**`.d.ts` optional parameters.** In wasm-bindgen 0.2.126, `cli-support/src/js/binding.rs`
`typescript_signature` walks the parameters in reverse. A parameter with a type override gets `?`
whenever it is marked optional, and it ends `omittable` for every parameter before it. That is why a
plain `Option<String>` placed before `bits` lost its `?`. Source:
https://raw.githubusercontent.com/wasm-bindgen/wasm-bindgen/0.2.126/crates/cli-support/src/js/binding.rs.
The fix: add `#[wasm_bindgen(unchecked_optional_param_type = "string | null")]` to `description` and
`meta`, and keep them as `Option<String>`. The glue still converts them as before. If the macro
rejects the attribute on an `Option<String>`, the fallback is a `JsValue` read with `as_string()`
and `is_null_or_undefined()` that names the parameter on rejection. Say in the handoff which one you
used. Only `gen_meta_code_v0` has an `Option` before a `JsValue` (grep `Option<String>`).

**`number_vec` without unwinding.** No JavaScript exception may cross a Rust frame, so every JS
access must go through an import marked `catch`. `Array::from`, the `Array` index getter, and
`try_iter`'s `IteratorNext::done()`/`value()` all lack `catch`, so do not use them. Read the input
as an array-like:

1. Keep the `is_object()` rejection.
2. `Reflect::get(value, &"length".into())` → `number(…)` → `to_u32(…, "<name>.length")`.
3. Read each element with `Reflect::get_u32(value, i)`, then `number(&v, "<name>[i]")`.
4. Map a caught exception (`Err(JsValue)`) to `JsError` with the parameter name and the message
    (`js_sys::Error` `message()` via `dyn_ref`, or a fixed text).

This accepts the same inputs as 0.7.0, whose glue copied array-likes into typed arrays: arrays,
typed arrays, and objects with `length`. A missing index reads `undefined` and fails at the first
gap, so a huge fake `length` costs nothing. Iterables without `length` (`Set`, generators) are
rejected with an error. Iteration 268 accepted them through `Array.from`, and 0.7.0 read them as
empty. State this narrowing in the doc comment and in the handoff. Measure the release-build time of
`gen_audio_code_v0` for 1,000,000 elements against the handoff figures (350 ms plain array).

**Test** (`tests/unit.rs`, `#[wasm_bindgen_test]` calls Rust directly, so this runs the real code
path): build a throwing array-like with
`js_sys::eval("({ get length() { throw new Error('boom') } })")` (or
`js_sys::Function::new_no_args`). Call `gen_audio_code_v0` 5,000 times and assert every call returns
`Err`. Then assert `text_trim("abc", …)` still returns `Ok`. Add one case for an element getter that
throws (`length: 1` plus `get 0()`), and one for an iterable without `length` being rejected. One
bad input per call.

**Docs.** In `docs/howto/wasm.md` → "Error handling", say that number-array parameters accept
arrays, typed arrays and array-likes. Also fix the review nit: the paragraph ends with a colon that
leads into an unrelated code example.

**Verify the declarations** (scratch dirs in `/tmp`, removed afterwards):
`git archive 3ef83ce | tar -x -C /tmp/wasm-base`, then build both trees with
`wasm-pack build --release --target nodejs`. Strip JSDoc lines from both `.d.ts` files (lines
starting with `/**`, `*` or `*/` after leading whitespace) and diff what is left.

## Verification

- `wasm-pack test --node crates/iscc-wasm --features conformance` passes, including the new
    throwing-array-like tests
- `cargo clippy -p iscc-wasm --target wasm32-unknown-unknown -- -D warnings` is clean
- The comment-stripped `.d.ts` of a release build of the working tree is identical to that of
    `3ef83ce` (the `diff` exits 0, output quoted in the handoff)
- Node, on a release `--target nodejs` build: the throwing-getter loop
    (`gen_audio_code_v0({ get   length() { throw … } })` inside `try`, 10,000 times) completes, and
    a later `text_trim("abc", 2)` returns `"ab"`
- `grep -n 'Array::from' crates/iscc-wasm/src/lib.rs` prints nothing
- `mise run check` exits 0, and `git status --porcelain` shows no build output or `/tmp` leftovers

## Done When

All six verification criteria pass, and the handoff names the optional-parameter mechanism used and
the narrowing for iterables without `length`.
