## 2026-11-21 — Review of: Range-check every JS integer argument, reject negative JNI counts, document the Python/Ruby classes

**Verdict:** PASS

**Summary:** napi and WASM now take every numeric input as `f64` and narrow it through one
range-checked helper set, JNI rejects a negative `bits`, `nbytes` or `width` before the unsigned
cast, and the Python and Ruby docs name `OverflowError` and `RangeError`. All six criteria pass, the
diff stays inside the scope (3 code files), and no gate was weakened. The `normal` `[audit]` issue
"Out-of-range integer arguments wrap silently on JS" is resolved and deleted.

**Verification:**

- [x] `npm run build:debug && npm test` in `crates/iscc-napi` — 238 pass, 0 fail. The listed cases
    (`bits` `2**32+64`, `-1`, `64.5`; `text_trim(…, -1)`; `gen_audio_code_v0([2**40])`;
    `alg_minhash_256([-1])`; a video frame with `2**31`; `encode_component(256, …)`) each throw,
    asserted on the message
- [x] `wasm-pack test --node crates/iscc-wasm` — 11 + 8 + 108 pass. Node on a fresh `/tmp/wasm-int`
    build: `gen_text_code_v0("x", 2**32+64)` throws `Invalid bits: 4294967360`,
    `gen_audio_code_v0([2**40])` throws `Invalid cv[0]: 1099511627776`
- [x] The signature grep over both JS `lib.rs` files prints nothing
- [x] `mvn -f crates/iscc-jni/java/pom.xml clean test` — 110 + 8 pass after
    `cargo build -p iscc-jni`. The new tests assert the message `<param> must be non-negative`,
    which the old wrapped value could not produce
- [x] `uv run pytest` — 769 pass (`tests/test_int_range.py` pins `OverflowError`);
    `bundle exec rake test` — 168 runs, 0 failures (`RangeError` pinned). `docs/howto/python.md` and
    `docs/ruby-api.md` name the classes
- [x] `cargo clippy --workspace --all-targets -- -D warnings` and `mise run check` exit 0

**Issues found:**

- Filed `low`, untriaged: "`@iscc/wasm` converts non-number values to numbers instead of rejecting
    them". The wasm-bindgen glue converts `null`, strings and booleans before the range check
    (`gen_audio_code_v0([1, null, 3])` gives the code of `[1, 0, 3]`). Pre-existing; napi throws
- Not filed (nits): JNI `encode_component` keeps its inline `bit_length < 0` check instead of the
    new `non_negative` helper (same behaviour, next.md left it out of scope). The narrowing helpers
    exist twice, in napi and WASM, because the error types differ

**Codex review:** no actionable findings. It re-ran the Node.js, WASM and JNI suites and the Python
and Ruby tests, and confirmed range validation, typed-array compatibility, defaults and hasher state
from the generated WASM JavaScript.

**Next:** "Published-package docs contradict what ships" (`normal` `[audit]`), the last `normal`
issue. It lists about 11 files, so it needs the audit valve (cite the issue by title, at most 8
non-test files per step). Start with `README.md` and the how-tos, as the entry says, and check every
install or packaging claim against the published artifact, not the repository.

**Notes:**

- Review probes beyond next.md, all from Node on the built packages: WASM video frames reject `1.5`
    and `-2**31-1` through `serde_wasm_bindgen` (so leaving them alone was right), `Int32Array` and
    `Uint32Array` inputs give the same result as plain arrays, and the `.d.ts` shows
    `ArrayLike<number>` for `cv` and `features` and `number` everywhere else. napi rejects a string
    or a fractional frame element
- No decision entry from the review: the advance agent's entry of 2026-11-21 covers the step. The
    choice of `unchecked_param_type = "ArrayLike<number>"` is explained in
    `crates/iscc-wasm/CLAUDE.md`
- Nothing enforces the "numeric inputs are `f64`" rule on a future export; it lives in the two crate
    `CLAUDE.md` files. A source gate would have to read only `#[napi]` / `#[wasm_bindgen]` exports
    to avoid false positives on private helpers. Worth a step only if the rule is broken once
- decisions.md is at 398 of 400 lines: the next entry triggers the runner's rotation
- The core is untouched: no iai, CRAP or semver impact
