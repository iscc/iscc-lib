## 2026-11-21 — Review of: `@iscc/wasm` type-checks every scalar argument in Rust instead of letting the glue coerce it

**Verdict:** NEEDS_WORK

**Summary:** The coercion is fixed. Every number, boolean and number-array parameter arrives as a
`JsValue` and is type-checked in Rust, and all six next.md criteria pass on a release build. The
step also introduced two defects that the criteria did not look for: the TypeScript signature of
`gen_meta_code_v0` lost its optional parameters, and a JavaScript exception inside `Array.from` can
make the WASM instance unusable. Nothing is pushed. The advance commit and this review commit stay
local until the repair.

**Verification:**

- [x] `wasm-pack test --node crates/iscc-wasm --features conformance` passes: 11 (conformance), 8
    (data_edge), 119 (unit), 0 failed
- [x] `cargo clippy -p iscc-wasm --target wasm32-unknown-unknown -- -D warnings` is clean
- [x] The grep prints only `opt_bool`, `checked`, `to_u8`, `to_u32`, `to_i32` and the `filesize`
    result field
- [x] Node probe on a release `wasm-pack build --target nodejs` in `/tmp`: `[1, null, 3]`, `"2"`,
    `true`, `wide = "yes"` and the nested `null` each throw; `gen_text_code_v0("Hello", null)` and
    `gen_text_code_v0("Hello")` return `ISCC:EAAWSPUDF7GJWQVD`. The advance probe used `--dev`,
    whose glue differs, so I repeated it on a release build
- [x] The `.d.ts` declares `gen_text_code_v0(text: string, bits?: number | null)` and
    `gen_audio_code_v0(cv: ArrayLike<number>, bits?: number | null)`
- [x] `mise run check` exits 0. `git status --porcelain` shows no build output

**Issues found:**

- **TypeScript regression (blocks the step).** The built `.d.ts` now declares
    `gen_meta_code_v0(name: string, description: string | null | undefined, meta: string | null | undefined, bits?: number | null)`.
    The build of the previous commit has `description?: string | null, meta?: string | null`. So
    `gen_meta_code_v0("ISCC Test Document!")`, the README call, no longer type-checks. It is the
    only export declaration that changed (full diff of both `.d.ts` files). The handoff claim "the
    `.d.ts` types do not change" was false
- **Instance can be killed (blocks the step).** `number_vec` calls `js_sys::Array::from`, which has
    no `catch`. Measured:
    `for (…) try { gen_audio_code_v0({ get length() { throw new Error("boom") } }) } catch {}` fails
    with `RuntimeError: memory access out of bounds` after 4,095 calls, and then
    `text_trim("abc", 2)` fails too. The previous commit survives 200,000 calls
- Both are in issues.md as one `critical` entry
- Filed `low`, untriaged: `@iscc/wasm` converts byte inputs and traps on a non-string
    (`gen_data_code_v0("abc")` returns the code of three zero bytes). Pre-existing
- Fixed in this commit: `docs/howto/wasm.md` said "Arguments are type-checked, not converted", which
    is false for bytes and strings. It now says "Number and boolean arguments"
- Nit: the new docs paragraph ends with a colon that leads into an unrelated code example
- Added to the audit entry on stale specs: the bridge sketch in `specs/wasm-bindings.md`

**Codex review:** Both blocking defects come from the Codex run. I reproduced each one against a
build of the previous commit before accepting it. Codex suggested a catch-enabled conversion for the
array and explicit optional declarations for `description` and `meta`.

**Next:** Repair the step, WASM surface only. The `critical` issues.md entry has the "Resolved
when".

- `gen_meta_code_v0`: restore `description?` and `meta?` in the `.d.ts`. Untested idea: put
    `unchecked_optional_param_type = "string | null"` on the two `Option<String>` parameters, or
    read them as `JsValue` with a string check. Check the macro source first
- `number_vec`: no JavaScript exception may cross a Rust frame. Use a catch-enabled read
    (`js_sys::try_iter`, `Reflect::get`), or accept only real arrays and typed arrays, which cannot
    throw in `Array.from`. The second choice narrows what the function accepts, so say which one you
    picked and why
- Verify with the full `.d.ts` diff against a build of `3ef83ce` (the commit before the advance
    commit), not with quoted lines, and with the throwing-getter loop from Node
- Gate idea for a later step: no check compares the built `.d.ts` with an expected set of
    declarations. The CI `wasm` job could build once and diff the export lines against a committed
    list

**Notes:**

- Unpushed: `3ef83ce` (define-next), `8560f6a` (advance) and this review commit
- Measured cost of the element-wise array read, release build, `gen_audio_code_v0`: 3,000 elements
    0.6 ms → 1.1 ms, 1,000,000 elements 213 ms → 350 ms (plain array), 196 ms → 381 ms
    (`Int32Array`). I consider that acceptable. It gets a decisions.md entry when the step passes,
    because the repair may change the read
- Release notes (v0.8.0), from the advance handoff: plain-JS callers that passed numeric strings,
    `null` for a required number, or non-booleans for `wide`/`add_units`/`utf32` now get an error
- Other inputs I probed that behave as intended: a missing required argument throws,
    `new Number(64)` and a `BigInt64Array` element throw, `Uint32Array` and `Int32Array` work
- learnings.md: the string-input divergence bullet moved to `learnings-archive.md` to make room (the
    three decisions of 2026-11-18 cover it)
