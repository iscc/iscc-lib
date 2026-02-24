## 2026-02-24 — Go module scaffold with wazero runtime and memory helpers

**Done:** Created the `packages/go/` Go module with wazero-based WASM runtime initialization, memory
management helpers (alloc/dealloc, writeString/readString, freeString), error retrieval
(`lastError`), and two public functions (`ConformanceSelftest`, `TextClean`). All 5 tests pass
including `iscc_conformance_selftest()` proving the end-to-end Go→wazero→WASM→Rust bridge works.

**Files changed:**

- `packages/go/go.mod`: Go module definition with wazero v1.11.0 dependency
- `packages/go/go.sum`: Auto-generated dependency checksums
- `packages/go/iscc.go`: Runtime type with NewRuntime/Close, memory helpers (alloc, dealloc,
    writeString, readString, freeString, lastError), and two public methods (ConformanceSelftest,
    TextClean)
- `packages/go/iscc_test.go`: 5 tests — TestRuntimeInit, TestConformanceSelftest, TestAllocDealloc,
    TestWriteReadString, TestTextClean
- `mise.toml`: Added `[tools]` section with `go = "latest"` (installs Go 1.26.0)
- `.gitignore`: Added `packages/go/*.wasm` pattern for the debug WASM binary

**Verification:** All 5 Go tests pass (3.4s total, ~0.6s each due to WASM compilation). Verified
with both `go test -v ./...` and `CGO_ENABLED=0 go test ./...` (proves no cgo dependency). All 14
pre-commit hooks pass via `mise run check`.

**Next:** Add Go wrappers for the 9 `gen_*_v0` functions. The memory helpers and string marshaling
pattern established here (writeString → call → readString → freeString) can be reused directly for
all gen functions. The `lastError` helper already handles error retrieval from NULL returns.

**Notes:**

- The next.md test spec for `TestTextClean` expected `"Hello  World"` → `"Hello World"` (double
    space collapsed to single), but `text_clean` doesn't collapse spaces within a line — it does
    NFKC normalization, control character removal, newline normalization, and leading/trailing
    whitespace stripping. Adjusted the test to use NFKC ligature normalization
    (`"  Hel\uFB01 World  "` → `"Helfi World"`) which exercises both NFKC and whitespace stripping.
- wazero v1.11.0 was resolved (latest at time of `go mod tidy`), not v1.9.0 as specified in the
    initial `go.mod` — Go resolved to the latest available version.
- Each test takes ~0.6s because wazero compiles the ~11MB WASM module on every `NewRuntime()` call.
    A shared runtime per test suite would be faster but the per-test isolation is preferred for the
    scaffold. Future optimization: use `CompileModule` once and `InstantiateModule` per test.
- The WASM binary must be pre-built and copied to `packages/go/` before tests run. `TestMain` checks
    for the binary and skips gracefully with instructions if missing.
