## 2026-02-27 — Propagate 7 Tier 1 symbols to Go binding

**Done:** Added 4 constants (`MetaTrimName`, `MetaTrimDescription`, `IoReadSize`, `TextNgramSize`),
`DecodeResult` struct, and 3 new methods (`JsonToDataUrl`, `EncodeComponent`, `IsccDecode`) to the
Go/wazero binding. Also rebuilt the WASM binary to include the 3 new FFI exports that were missing.

**Files changed:**

- `packages/go/iscc.go`: Added 4 package-level constants, `DecodeResult` struct, and 3 new `Runtime`
    methods (`JsonToDataUrl`, `EncodeComponent`, `IsccDecode`) using existing bridge patterns
- `packages/go/iscc_test.go`: Added 7 new tests (`TestConstants`, `TestJsonToDataUrl`,
    `TestJsonToDataUrlLdJson`, `TestEncodeComponent`, `TestIsccDecode`, `TestIsccDecodeInvalid`,
    `TestEncodeDecodeRoundtrip`)
- `packages/go/iscc_ffi.wasm`: Rebuilt from
    `cargo build -p iscc-ffi --target wasm32-wasip1 --release` to include the 3 new FFI exports

**Verification:**

- `cd packages/go && mise exec -- go test -v -run TestConstants ./...` — PASS
- `cd packages/go && mise exec -- go test -v -run TestJsonToDataUrl ./...` — PASS (both variants)
- `cd packages/go && mise exec -- go test -v -run TestEncodeComponent ./...` — PASS
- `cd packages/go && mise exec -- go test -v -run TestIsccDecode ./...` — PASS (both valid and
    invalid)
- `cd packages/go && mise exec -- go test -v -run TestEncodeDecodeRoundtrip ./...` — PASS
- `cd packages/go && mise exec -- go test -count=1 ./...` — PASS (all existing + new tests)
- `cd packages/go && mise exec -- go vet ./...` — clean
- `grep -c` for constants returns 4
- `grep -c 'func (rt \*Runtime)'` returns 48 (≥ 26)
- `mise run check` — all 14 pre-commit hooks pass

**Next:** All 30/30 Tier 1 symbols are now propagated to all language bindings (Python, Node.js,
WASM, C FFI, Java JNI, Go/wazero). The project can move on to other priorities from the target —
possibly documentation updates for the Go binding, or addressing remaining issues.

**Notes:** The WASM binary (`packages/go/iscc_ffi.wasm`) was stale — it didn't include the 3 new FFI
functions (`iscc_json_to_data_url`, `iscc_encode_component`, `iscc_decode`,
`iscc_free_decode_result`). It had to be rebuilt despite next.md stating it was already up-to-date.
The `IsccDecode` method uses the WASM sret calling convention (16-byte struct allocation, pass as
first hidden param) and `iscc_free_decode_result` takes the sret pointer directly (struct-by-pointer
in WASM ABI), consistent with the existing `freeByteBuffer` pattern.
