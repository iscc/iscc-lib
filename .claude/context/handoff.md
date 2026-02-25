## 2026-02-25 — Add 6 Go wrappers (text utils + encode + string-array functions)

**Done:** Implemented 6 new Go wrappers on the `Runtime` type: `TextRemoveNewlines`, `TextCollapse`,
`TextTrim`, `EncodeBase64` (Group A: string-returning, following the `TextClean` pattern), plus
`SlidingWindow` and `IsccDecompose` (Group B: string-array-returning, using new `readStringArray`,
`freeStringArray`, and `callStringArrayResult` helpers). All build on the established WASM bridge
patterns.

**Files changed:**

- `packages/go/iscc.go`: Added 6 public methods (TextRemoveNewlines, TextCollapse, TextTrim,
    EncodeBase64, SlidingWindow, IsccDecompose) and 3 private helper methods (readStringArray,
    freeStringArray, callStringArrayResult) for reading null-terminated C string pointer arrays from
    WASM32 memory
- `packages/go/iscc_test.go`: Added 8 test functions (TestTextRemoveNewlines, TestTextCollapse,
    TestTextTrim, TestEncodeBase64, TestSlidingWindow, TestSlidingWindowError, TestIsccDecompose)
    plus the existing 14 tests continue to pass

**Verification:**

- `CGO_ENABLED=0 go test -v -count=1 ./...` in `packages/go/` passes — 22 tests (14 existing + 8
    new), all PASS
- `go vet ./...` in `packages/go/` is clean (no output)
- `grep -c 'func (rt \*Runtime)' packages/go/iscc.go` outputs 32 (23 existing + 9 new — the 6 public
    wrappers plus 3 private helpers). Note: next.md expected 17 but undercounted the existing
    private helper methods on Runtime
- `grep 'TextRemoveNewlines\|TextTrim\|TextCollapse\|EncodeBase64\|SlidingWindow\|IsccDecompose' packages/go/iscc.go | wc -l`
    outputs 12 (each function has definition + docstring mention)
- `mise run check` passes — all 14 pre-commit hooks clean

**Next:** The Go Tier 1 gap is now 6 functions (down from 12). The remaining 6 require new
infrastructure: 4 byte-buffer-returning functions (`AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`,
`SoftHashVideoV0`) needing `IsccByteBuffer` read/free helpers, and 2 streaming hashers
(`DataHasher`, `InstanceHasher`) needing opaque pointer lifecycle management.

**Notes:** The `readStringArray` helper reads u32 pointers from WASM memory 4 bytes at a time until
hitting a zero (null terminator), matching the WASM32 memory layout documented in next.md. The
`IsccDecompose` test uses the `test_0000_standard` conformance vector from `gen_iscc_code_v0` to
verify round-trip decomposition (4 units: Meta, Content-Text, Data, Instance). The verification
criterion for method count (expected 17) was based on an undercount of existing methods — the actual
pre-change count was 23 (12 public + 11 private helpers), now 32 with 9 additions.
