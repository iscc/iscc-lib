## 2026-02-25 — Add 4 byte-buffer Go wrappers (AlgSimhash, AlgMinhash256, AlgCdcChunks, SoftHashVideoV0)

**Done:** Implemented all 4 byte-buffer-returning Go wrappers plus 7 new private helpers for the
WASM sret ABI pattern. Empirically verified that wasm32-wasip1 uses sret (struct return pointer as
hidden first param) for both `IsccByteBuffer` and `IsccByteBufferArray` returns, and pass-by-pointer
(single i32) for struct parameters in free functions.

**Files changed:**

- `packages/go/iscc.go`: Added 7 private helpers (`readByteBuffer`, `freeByteBuffer`,
    `callByteBufferResult`, `readByteBufferArray`, `freeByteBufferArray`, `writeU32Slice`,
    `writeByteArrayOfArrays`) and 4 public wrappers (`AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`,
    `SoftHashVideoV0`). Total Runtime methods: 43 (was 32).
- `packages/go/iscc_test.go`: Added 6 test functions (`TestAlgSimhash`, `TestAlgMinhash256`,
    `TestAlgCdcChunks`, `TestAlgCdcChunksEmpty`, `TestSoftHashVideoV0`, `TestSoftHashVideoV0Error`).
    Total tests: 28 (was 22).

**Verification:** All 28 Go tests pass (`CGO_ENABLED=0 go test -v -count=1 ./...`), `go vet` clean,
all 14 pre-commit hooks pass. `grep` counts: 8 public method references (≥4), 43 Runtime methods
(≥38).

**Next:** The Go binding is now at 21/23 Tier 1 symbols. The remaining 2 are the streaming hashers
(`DataHasher`/`InstanceHasher`) which need opaque pointer lifecycle management
(alloc/update/finalize pattern with a WASM-side handle).

**Notes:** Key ABI discovery: `iscc_free_byte_buffer` and `iscc_free_byte_buffer_array` take their
struct parameters as a single i32 (pointer to struct in WASM memory), not as flattened fields. This
means the sret pointer from the function call can be reused directly as the free function's struct
pointer — the struct is already laid out at that address. This pattern is clean and avoids extra
allocation for the free call. The `AlgCdcChunks` wrapper reads individual `IsccByteBuffer` entries
from the array at `buffersPtr + i*8` (8 bytes per struct on wasm32).
