## 2026-02-24 — Create per-crate READMEs (batch 2: iscc-wasm, iscc-jni)

**Done:** Created registry-facing README.md files for the two remaining publishable binding crates.
`crates/iscc-wasm/README.md` targets npm as `@iscc/wasm` with ESM/async init quick start.
`crates/iscc-jni/README.md` targets Maven Central as `io.iscc:iscc-lib` with Java camelCase API
examples and native library path note.

**Files changed:**

- `crates/iscc-wasm/README.md`: Created (77 lines). 6 H2 sections, npm badge, ESM import pattern
    with `await init()`, notes on browser + Node.js support, string return pattern documented.
- `crates/iscc-jni/README.md`: Created (80 lines). 6 H2 sections, Maven dependency XML, Java quick
    start with `IsccLib.genMetaCodeV0()`, native library path caveat, streaming hasher handle-based
    API listed.

**Verification:**

- [x] `test -f crates/iscc-wasm/README.md` exits 0
- [x] `test -f crates/iscc-jni/README.md` exits 0
- [x] `grep -c '^## ' crates/iscc-wasm/README.md` returns 6
- [x] `grep -c '^## ' crates/iscc-jni/README.md` returns 6
- [x] `grep -q '@iscc/wasm' crates/iscc-wasm/README.md` exits 0
- [x] `grep -q 'io.iscc' crates/iscc-jni/README.md` exits 0
- [x] `cargo clippy --workspace --all-targets -- -D warnings` clean
- [x] `mise run check` passes all 14 pre-commit hooks

**Next:** All per-crate READMEs for publishable crates are complete (iscc-lib, iscc-py, iscc-napi,
iscc-wasm, iscc-jni). The remaining crate `iscc-ffi` is not published to any registry and was
explicitly excluded from scope. Next work could focus on updating the root README.md with Java/WASM
sections, creating `docs/howto/java.md`, or addressing tracked issues (codec header optimization,
DataHasher allocation overhead, DCT input validation).

**Notes:** Both READMEs follow the exact 6-section template from batch 1. The WASM quick start uses
ESM `import`/`await init()` pattern (not CommonJS `require()`) since wasm-bindgen requires async
initialization. The JNI README documents the handle-based streaming API (opaque `long` pointers with
explicit `*Free` calls) since Java lacks RAII. No Cargo.toml `readme` field changes needed since
both crates have `publish = false` (they publish via npm and Maven respectively).
