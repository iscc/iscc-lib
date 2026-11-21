## 2026-11-21 — Repair the `@iscc/wasm` type checks — optional `gen_meta_code_v0` params and exception-safe number arrays

**Done:** `gen_meta_code_v0`'s `description` and `meta` have their `?` back in the `.d.ts`. The fix
is `#[wasm_bindgen(unchecked_optional_param_type = "string | null")]` on the two `Option<String>`
parameters. The macro accepted it, so the `JsValue` fallback was not needed. `number_vec` now reads
`length` and each index through `Reflect::get` / `Reflect::get_u32`, both `catch` imports. A caught
exception becomes `Invalid <name>: <message>`. The message is itself read via `Reflect::get`, so a
throwing `message` getter cannot escape either. `Array::from` is gone.

**Files changed:**

- `crates/iscc-wasm/src/lib.rs`: `number_vec` rewritten as an array-like read. Added a private
    `thrown` helper, `string | null` optional overrides on `description`/`meta`, and
    `alg_cdc_chunks` maps with `Uint8Array::new_from_slice` instead of `Uint8Array::from`. The
    `From<&[u8]>` impl is an `#[inline]` call to `new_from_slice` (js-sys lib.rs:14116), so
    behaviour is the same. I made that change only because the criterion's grep matched the
    substring `Array::from` inside `Uint8Array::from`
- `crates/iscc-wasm/tests/unit.rs`: a `js()` eval helper and 7 tests. They cover an array-like
    accepted, a throwing `length` ×5,000 followed by `text_trim("abc", 2) == "ab"`, a throwing
    element getter, a thrown non-Error (`throw 42` → `exception thrown`), a `Set` rejected, a
    `length: -1` rejected, and `length: 4294967295` failing at index 1
- `docs/howto/wasm.md`: "Error handling" has a number-array paragraph (accepted inputs, the iterable
    rejection, caught exceptions). The dangling colon is now a sentence that introduces the example

**Verification:**

- `wasm-pack test --node crates/iscc-wasm --features conformance`: 11 + 8 + 126 passed (unit was
    119), 0 failed. I re-ran it after the `new_from_slice` edit: same counts
- `cargo clippy -p iscc-wasm --target wasm32-unknown-unknown -- -D warnings`: clean (re-run after
    the last edit)
- `.d.ts`: release `--target nodejs` builds of `git archive 3ef83ce` and of the working tree, with
    JSDoc lines stripped (`grep -Ev '^[[:space:]]*(/\*\*|\*|\*/)'`). `diff` printed nothing and
    exited 0 (112 lines each). The new build declares
    `gen_meta_code_v0(name: string, description?: string | null, meta?: string | null, bits?: number | null)`.
    This build came before the `new_from_slice` edit, which touches only the inside of a function
    that returns `Uint8Array[]`
- Node, release build: the throwing-`length` loop ran 10,000/10,000 errors (`"Invalid cv: boom"`),
    and so did the throwing-element loop for `alg_minhash_256` (`"Invalid features: elem"`). After
    both, `text_trim('abc', 2)` returned `"ab"`. `gen_meta_code_v0('ISCC Test Document!')` returned
    `ISCC:AAAT4EBWK27737D2`. `new Set([1,2])` and a generator both threw
    `Invalid cv.length: expected a number`. The array-like gave the same code as `[1,-2,3]`
- `grep -n 'Array::from' crates/iscc-wasm/src/lib.rs`: no output (exit 1)
- `mise run check`: exit 0. `git status --porcelain` shows only the 3 scoped files and
    `iterations.jsonl`. All `/tmp` build dirs, probes and scripts are removed

**Narrowing:** an iterable without `length` (`Set`, a generator) is now rejected. Iteration 268
accepted it via `Array.from`, and 0.7.0 read it as empty. Arrays, typed arrays and array-likes
behave as in 0.7.0, except that elements are now type-checked. This is stated in the `number_vec`
doc comment and in the docs page.

**Performance (flag):** `gen_audio_code_v0`, release builds, all measured on this machine in one
session (ms per call):

| input              | `3ef83ce` (glue copy) | HEAD `8560f6a` (`Array.from`) | working tree (`Reflect.get`) |
| ------------------ | --------------------- | ----------------------------- | ---------------------------- |
| plain array, 3,000 | 0.62                  | 0.69                          | 1.43                         |
| plain array, 1M    | 242                   | 218                           | 512                          |
| Int32Array, 3,000  | 0.65                  | 0.68                          | 1.39                         |
| Int32Array, 1M     | 247                   | 216                           | 463                          |

Each `Reflect.get` goes through the glue's `handleError` try/apply wrapper, and that costs about 2×
the `Array.from` read. The reviewer's 350 ms for HEAD was from another run. On this machine HEAD
measured 218 ms. A Chromaprint vector of a few thousand elements costs about 0.8 ms more per call.
This is binding marshalling, not a core hot path, but it is more than the ~10% threshold, so it
needs a decision.

**Next:** decide whether ~2× on number arrays is acceptable. If not, a cheaper catch-safe read could
use a crate-local `catch` import of `Array.prototype.slice` in bounded chunks (one crossing per
chunk) and read the fresh chunk by index. That index read uses `Array::get` without `catch`. A hole
in a sliced array falls through to `Array.prototype`, so it is not strictly exception-proof. I have
not measured it.

**Notes:**

- I did not probe the `serde_wasm_bindgen::from_value` paths (`frame_sigs`, `codes`, `units`,
    `hash_digests`) with throwing elements. They are out of scope and still open
- `crates/iscc-wasm/CLAUDE.md` is outside this step's scope. Two parts of it are now stale. The type
    table row `Option<String>` → `string | undefined` does not mention that an `Option` before a
    `JsValue` override needs its own `unchecked_optional_param_type`. The pitfall "Never take … a
    `Vec` of numbers" does not mention the `catch`-only rule for JS reads
- Unpushed: `3ef83ce`, `8560f6a`, the review commit, and this commit
