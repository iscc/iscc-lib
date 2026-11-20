## 2026-11-20 — Review of: WASM `alg_cdc_chunks` returns `Uint8Array` chunks

**Verdict:** PASS

**Summary:** `@iscc/wasm`'s `alg_cdc_chunks` now returns `Vec<js_sys::Uint8Array>`, so JS callers
get `Uint8Array[]` at runtime and in the generated `.d.ts`, as the doc comment always promised. The
diff is small and stays in scope: one function, the `js-sys` dependency (already in the lockfile),
three rewritten tests, the how-to bullet and the crate's type-mapping row. The issue is resolved and
deleted.

**Verification:**

- [x] `wasm-pack test --node crates/iscc-wasm --features conformance` passes, including a test that
    asserts every chunk `is_instance_of::<Uint8Array>()` and the empty-input single empty chunk —
    exit 0: unit 94 passed, conformance 11, data_edge 5
- [x] `cargo clippy -p iscc-wasm --all-targets -- -D warnings` and
    `cargo clippy -p iscc-wasm --target wasm32-unknown-unknown -- -D warnings` are both clean — both
    exit 0; workspace clippy also exit 0
- [x] `grep -n "to_value" crates/iscc-wasm/src/lib.rs` prints nothing — exit 1, no output
- [x] `docs/howto/wasm.md` contains no `number[]` / `Uint8Array.from(chunk)` text for
    `alg_cdc_chunks`, and `uv run zensical build` exits 0 — grep empty, "No issues found"
- [x] `mise run check` exits 0 — clean on the first run, no files modified

One probe beyond next.md: a Node run of the built package (`wasm-pack build --target nodejs` into
`/tmp`). 4 KiB at avg 64 gives 65 chunks, all `instanceof Uint8Array`, each with its own buffer,
concatenating to the input; `.subarray()` works; empty input gives one empty `Uint8Array`; `utf32`
cuts stay 4-byte aligned; `avg_chunk_size` 0 still throws the core's error. The `.d.ts` line reads
`alg_cdc_chunks(...): Uint8Array[]`.

**Issues found:**

- Not filed (nit): the `is_instance_of` assert in `tests/unit.rs` cannot fail while the return type
    is `Vec<js_sys::Uint8Array>`, because the test calls the Rust function and never crosses the JS
    export glue. The static return type is what pins the shape: any other return type fails to
    compile against the tests. The JS-side proof is the Node probe above, which is not a committed
    test
- Fixed in this review: `crates/iscc-wasm/CLAUDE.md` → "Common Pitfalls" still told agents to use
    `serde_wasm_bindgen` for all Rust-to-JS conversions, which is how the defect got in. It now says
    `from_value` for nested inputs and forbids `to_value` for byte outputs

**Codex review:** No actionable findings. Codex ran the 110 WASM tests, clippy, a release build and
its own JavaScript probes, and confirmed `Uint8Array` chunks, independent byte copies, unchanged
error handling and the matching TypeScript declaration.

**Next:** Two `normal` issues are left, both docs and tooling:

1. "Derived facts in guidance and docs are written by hand and drift": make
    `scripts/check_tier1_docs.py`'s `lib.rs` parser fail closed (an unrecognised column-0 `pub`
    form fails the gate, pinned by a fire test) and register the four sub-threshold docs as `FULL`
    or `EXEMPT`. `docs/rust-api.md` is incomplete and still claims `IsccResult<String>` for all gen
    functions, so it may need its own step
2. "Java how-to `genSumCodeV0` example does not compile": a three-line fix in `docs/howto/java.md`
    that fits into either step above

**Notes:**

- The runtime shape of a published `@iscc/wasm` export changed (`number[][]` → `Uint8Array[]`).
    decisions.md records why it ships as a bug fix without a transition. Titusz: worth one line in
    the v0.8.0 release notes
- No core code changed, so the CRAP, iai and semver gates are not affected. `js-sys` 0.3.103 was
    already resolved; `Cargo.lock` gained only the dependency edge
- Not touched: the napi `Buffer` reference leak (`low`)
