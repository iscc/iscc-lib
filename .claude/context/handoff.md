## 2026-02-24 — Fix napi packaging, version skew, and unnecessary clone

**Done:** Resolved all three open normal-priority napi issues: added `"files"` allowlist to
`package.json` so `npm publish` includes entrypoints, replaced
`.iter().map(|c| Buffer::from(c.to_vec()))` with `.into_iter().map(Buffer::from)` in
`alg_cdc_chunks` to eliminate per-chunk Vec allocation, and regenerated `index.js`/`index.d.ts` with
the correct `0.0.1` version.

**Files changed:**

- `crates/iscc-napi/package.json`: Added `"files"` allowlist (`index.js`, `index.d.ts`, `*.node`,
    `README.md`) between `"types"` and `"napi"` fields
- `crates/iscc-napi/src/lib.rs`: Changed `alg_cdc_chunks` wrapper from
    `.iter().map(|c| Buffer::from(c.to_vec()))` to `.into_iter().map(Buffer::from)` — napi-rs
    `Buffer` implements `From<&[u8]>` directly, so no intermediate `Vec<u8>` allocation needed

**Verification:**

- [x] `cargo clippy -p iscc-napi -- -D warnings` clean
- [x] `npm test` passes in `crates/iscc-napi/` — all 103 tests (23 suites), 0 failures
- [x] `grep '"files"' crates/iscc-napi/package.json` returns a match
- [x] `grep 'c\.to_vec()' crates/iscc-napi/src/lib.rs` returns 0 matches
- [x] `grep -c 'expected 0.0.1' crates/iscc-napi/index.js` returns 26
- [x] `mise run check` passes all 14 pre-commit hooks

**Next:** All three napi normal-priority issues resolved. Remaining normal-priority candidates: wasm
silent null return (`alg_cdc_chunks`), FFI video frame copy, codec header parsing optimization,
DataHasher allocation overhead. Low priority: alg_dct power-of-two, alg_wtahash bounds check, JNI
exception types, Python __version__, Python module docstring, WASM conformance_selftest size, WASM
stale CLAUDE.md, TypeScript port evaluation.

**Notes:** `Buffer::from(&[u8])` works in napi-rs v3 — the fallback path with `to_vec()` mentioned
in next.md was not needed. The test count increased from 66 (noted in issues) to 103 since the
original issue was filed (streaming hasher tests and additional function tests were added). The
regenerated `index.js`/`index.d.ts` remain gitignored per convention — CI regenerates them during
build.
