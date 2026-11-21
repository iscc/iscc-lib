## 2026-11-21 — Review of: Repair the `@iscc/wasm` type checks — optional `gen_meta_code_v0` params and exception-safe number arrays

**Verdict:** PASS_WITH_NOTES

**Summary:** Both defects of iteration 268 are repaired and the `critical` issue is closed.
`gen_meta_code_v0` has its optional `description?` and `meta?` back, and a throwing array-like now
gives an ordinary error and leaves the instance usable. The price is about 2× marshalling time on
number arrays, which I accepted and recorded in decisions.md. The diff stays inside the three scoped
files.

**Verification:**

- [x] `wasm-pack test --node crates/iscc-wasm --features conformance` passes: 11 + 8 + 126, 0 failed
    (the 7 new throwing-array-like tests included)
- [x] `cargo clippy -p iscc-wasm --target wasm32-unknown-unknown -- -D warnings` is clean
- [x] Comment-stripped `.d.ts` of a release build of HEAD equals that of `3ef83ce`: `diff` exits 0
    (114 lines each, both built with `--features conformance`). The unstripped diff is one JSDoc
    sentence. HEAD declares
    `gen_meta_code_v0(name: string, description?: string | null, meta?: string | null, bits?: number | null)`
- [x] Node, release `--target nodejs` build: 10,000 throwing-`length` calls give 10,000 errors
    (`Invalid cv: boom`), and `text_trim("abc", 2)` then returns `"ab"`
- [x] `grep -n 'Array::from' crates/iscc-wasm/src/lib.rs` prints nothing
- [x] `mise run check` exits 0; `git status --porcelain` showed only `iterations.jsonl`

Probes beyond next.md (3):

- Hostile exception shapes, 6,000 calls each, instance usable afterwards: a thrown object with a
    throwing `message` getter, `throw null`, a proxy with a throwing `get` trap, a revoked proxy, a
    sparse array, a string `length`. All give a named error
- The `serde_wasm_bindgen::from_value` paths with a throwing element: the instance dies. See below
- Timing, base `3ef83ce` against HEAD: the handoff's ~2× is correct (figures in decisions.md)

**Issues found:**

- Filed `low`, untriaged: "`@iscc/wasm` list inputs read through serde let a throwing element kill
    the instance". `gen_video_code_v0`, `soft_hash_video_v0`, `gen_mixed_code_v0`,
    `gen_iscc_code_v0` and `encode_seq` fail with `memory access out of bounds` after 5,000 to 8,200
    calls with a throwing element getter. The build of `3ef83ce` behaves the same, so it is older
    than iteration 268
- Not filed (nit): `alg_cdc_chunks` changed `Uint8Array::from` to `new_from_slice` only because
    next.md's grep matched the substring. The two are equivalent and the CDC tests pass
- Fixed here: `crates/iscc-wasm/CLAUDE.md` → Common Pitfalls now states the two rules this step
    established (an `Option<T>` before a `JsValue` override needs its own TypeScript override; read
    JS values only through `catch` imports). The advance handoff reported both gaps
- Trimmed: the `low` byte-input issue no longer says `gen_audio_code_v0({})` reads as an empty list.
    It now throws `Invalid cv.length: expected a number`

**Codex review:** no actionable findings. Codex ran the 145 WASM tests and its own release-build
checks of the optional parameters, the array reads and the CDC chunk output.

**Next:** no `critical` issue is left. Pick from the `normal` entries. Two carry a decision by
Titusz and need no further input: "The Ruby floor is past end of life and current Rubies get no
precompiled gem" (`[human]`) and "C++ `safe_data` and .NET `PinnedRows` padding guard a NULL
rejection the FFI no longer has". I would take the Ruby floor first. Gate idea for a later step: a
committed, comment-stripped `.d.ts` snapshot that the `wasm` CI job compares against the build. It
would have caught the lost `?` of iteration 268 without a reviewer.

**Notes:**

- This push carries iterations 268 and 269 together. It is the first CI run of the WASM type checks,
    so update-state should read the `wasm` job result
- Decision recorded (2026-11-21, "`@iscc/wasm` reads number arrays element by element through
    `Reflect.get`"): the 2× marshalling cost is accepted. Measured on Node 22, `gen_audio_code_v0`:
    3,000 elements 0.79 → 1.65 ms, 1M elements 243 → 525 ms. Do not scope a faster read unless a
    user reports it. Titusz can overrule by editing the entry
- Narrowing confirmed: a `Set` or a generator is rejected for `cv` and `features` (0.7.0 read them
    as empty). Arrays, typed arrays and array-likes give the same code as before
- Review trap, now in learnings.md: two `git archive` trees that share one `CARGO_TARGET_DIR`
    package the first tree's wasm twice. My first `.d.ts` diff compared base with base. `cmp` the
    two `.wasm` files before trusting such a diff
- decisions.md was over budget after the new entry. I rotated it with `cid.archive_overflow` (398
    lines now)
