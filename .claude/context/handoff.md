## 2026-02-27 — Commit Go WASM binary for go get distribution

**Done:** Removed `packages/go/*.wasm` from `.gitignore` and committed the release WASM binary
(682KB, built with `--release` profile: LTO, single codegen unit, stripped). Go tests pass with the
release binary.

**Files changed:**

- `.gitignore`: Removed the `packages/go/*.wasm` ignore rule (lines 231-232)
- `packages/go/iscc_ffi.wasm`: Release WASM binary (682,992 bytes) now tracked in git

**Verification:**

- `grep -c 'packages/go/\*.wasm' .gitignore` → `0` (line removed)
- `file packages/go/iscc_ffi.wasm` → `WebAssembly (wasm) binary module version 0x1 (MVP)`
- Binary size: 682,992 bytes (< 1,000,000)
- Go tests: all pass (`go test -count=1 ./...` — 11.2s)
- `mise run check`: all 14 hooks pass

**Next:** The Go module is now distributable via `go get`. Remaining issues in issues.md: dict
auto-serialization for `gen_meta_code_v0` (#5), `encode_component` API promotion (#6), `iscc_decode`
implementation (#7), constants exposure (#8). These are the last gaps before full iscc-core API
parity.

**Notes:** The CI Go job continues to build a debug WASM binary for testing (correct behavior —
tests current code). The committed release binary serves `go get` consumers. When FFI exports
change, the release binary needs to be rebuilt and recommitted — this could be automated in a future
release workflow step.
