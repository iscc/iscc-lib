## 2026-02-25 — Add 9 gen\_\*\_v0 Go wrappers with conformance tests

**Done:** Implemented all 9 `gen_*_v0` Go wrappers on `*Runtime` (GenMetaCodeV0, GenTextCodeV0,
GenImageCodeV0, GenAudioCodeV0, GenVideoCodeV0, GenMixedCodeV0, GenDataCodeV0, GenInstanceCodeV0,
GenIsccCodeV0) with 4 memory helper functions (writeBytes, writeI32Slice, writeStringArray,
writeI32ArrayOfArrays) and a shared callStringResult helper. Added conformance tests covering all 46
vectors from data.json across all 9 functions.

**Files changed:**

- `packages/go/iscc.go`: Added `encoding/binary` import, 4 memory helpers (writeBytes,
    writeI32Slice, writeStringArray, writeI32ArrayOfArrays), callStringResult helper, and 9
    gen\_\*\_v0 public methods. Updated package docstring.
- `packages/go/iscc_test.go`: Added 9 conformance test functions (TestGenMetaCodeV0 through
    TestGenIsccCodeV0) with JSON vector parsing helpers. Each function iterates all vectors for its
    function type using Go subtests.

**Verification:** All tests pass:

- `cd packages/go && CGO_ENABLED=0 go test -v -count=1 ./...` — 14 tests pass (5 existing + 9 new
    conformance test functions covering 46 total vectors)
- `cd packages/go && go vet ./...` — clean
- `mise run check` — all 14 pre-commit hooks pass

**Next:** Add Go CI job in `.github/workflows/ci.yml` to run `go test` and `go vet` in CI. The Go
module scaffold and all 9 gen functions are complete — CI integration is the natural next step to
protect against regressions.

**Notes:**

- Empty slice handling required careful treatment: `iscc_alloc(0)` returns a dangling pointer with
    alignment 1, which is fine for `*const u8` (bytes) but causes `slice::from_raw_parts` to panic
    for `*const i32` (requires alignment 4). Fixed `writeI32Slice` to allocate minimum 4 bytes for
    empty slices, ensuring proper i32 alignment. `writeBytes` uses `iscc_alloc(0)` directly since u8
    has alignment 1.
- Meta test vectors with dict values (e.g., `{"some": "object"}`) are JSON-serialized in the Go test
    before passing to the FFI, matching how the Rust conformance tests handle them.
- The `writeI32Slice` helper returns 3 values (ptr, allocSize, count) instead of 2, because
    allocSize may differ from count\*4 when allocating a minimum 4 bytes for empty slices. This
    ensures correct deallocation.
