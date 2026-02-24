## 2026-02-24 — Review of: Create per-crate READMEs (batch 2: iscc-wasm, iscc-jni)

**Verdict:** PASS

**Summary:** Two registry-facing README.md files created for the remaining publishable binding
crates (npm WASM and Maven JNI). Both follow the exact 6-section template from batch 1 with correct
registry-specific content, accurate API surfaces verified against source code, and identical shared
sections across all 5 crate READMEs. Documentation-only change, no code modifications.

**Verification:**

- [x] `test -f crates/iscc-wasm/README.md` exits 0
- [x] `test -f crates/iscc-jni/README.md` exits 0
- [x] `grep -c '^## ' crates/iscc-wasm/README.md` returns 6
- [x] `grep -c '^## ' crates/iscc-jni/README.md` returns 6
- [x] `grep -q '@iscc/wasm' crates/iscc-wasm/README.md` exits 0
- [x] `grep -q 'io.iscc' crates/iscc-jni/README.md` exits 0
- [x] `cargo clippy --workspace --all-targets -- -D warnings` clean
- [x] `mise run check` passes all 14 pre-commit hooks
- [x] "What is ISCC" paragraph md5 identical across all 5 READMEs
- [x] WASM API surface (9 gen + utilities + streaming) matches `crates/iscc-wasm/src/lib.rs`
- [x] JNI API surface (9 gen + utilities + streaming) matches `IsccLib.java` with camelCase names

**Issues found:**

- (none)

**Next:** All per-crate READMEs for publishable crates are complete (iscc-lib, iscc-py, iscc-napi,
iscc-wasm, iscc-jni). The `iscc-ffi` crate has no registry README since it's not published
separately. Next work could focus on: (1) updating root README.md with Java/WASM sections, (2)
creating `docs/howto/java.md`, (3) addressing tracked performance issues (codec header optimization,
DataHasher allocation overhead), or (4) the low-priority input validation issues (DCT power-of-two,
wtahash minimum length).

**Notes:** The WASM README correctly uses ESM `import`/`await init()` (not CommonJS) since
wasm-bindgen requires async initialization. The JNI README properly documents the handle-based
streaming API with explicit `*Free` calls (opaque `long` handles) since Java lacks RAII. Both crates
have `publish = false` in Cargo.toml so no `readme` field wiring was needed.
