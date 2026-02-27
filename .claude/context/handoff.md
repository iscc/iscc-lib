## 2026-02-27 — Implement pure Go GenDataCodeV0 and GenInstanceCodeV0

**Done:** Implemented `GenDataCodeV0` with `DataHasher` streaming type and `GenInstanceCodeV0` with
`InstanceHasher` streaming type as pure Go gen functions. All 7 conformance vectors pass (4 data + 3
instance). Also renamed WASM bridge streaming types to `WasmDataHasher`/`WasmInstanceHasher` to
resolve name collisions.

**Files changed:**

- `packages/go/code_data.go`: New file — `DataHasher` struct with `Push`/`Finalize` API,
    `DataCodeResult` type, `GenDataCodeV0` convenience function. Uses CDC + xxh32 + MinHash pipeline
- `packages/go/code_data_test.go`: New file — 4 conformance vector tests using `TestPureGo*` prefix
- `packages/go/code_instance.go`: New file — `InstanceHasher` struct with `Push`/`Finalize` API,
    `InstanceCodeResult` type, `GenInstanceCodeV0` convenience function. Uses BLAKE3 + multihash
- `packages/go/code_instance_test.go`: New file — 3 conformance vector tests verifying iscc,
    datahash, and filesize fields
- `packages/go/iscc.go`: Renamed `DataHasher` → `WasmDataHasher`, `InstanceHasher` →
    `WasmInstanceHasher` to resolve type name collision with new pure Go implementations

**Verification:**

- `go build ./...` — exits 0
- `TestPureGoGenDataCodeV0` — 4/4 vectors PASS
- `TestPureGoGenInstanceCodeV0` — 3/3 vectors PASS
- `go vet ./...` — exits 0
- `go test ./...` — all tests pass (pure Go + WASM bridge)
- Race detector clean on both new test suites
- All grep verification criteria return 1
- `mise run check` — all 14 hooks pass

**Next:** Implement remaining gen functions (`GenImageCodeV0`, `GenAudioCodeV0`, `GenVideoCodeV0`,
`GenMixedCodeV0`, `GenIsccCodeV0`) following the same pattern. Image and Audio codes need SimHash;
Mixed and ISCC codes need component decoding/composition.

**Notes:** The WASM bridge in `iscc.go` already defined `DataHasher` and `InstanceHasher` types for
its FFI streaming interface. Since both live in the same `iscc` package and Go doesn't support type
name overloading, the WASM bridge types were renamed to `WasmDataHasher` and `WasmInstanceHasher`.
This was the minimal change needed — the WASM bridge tests reference these types through
`rt.NewDataHasher()` which only required changing the return type signature, not the test code
itself. The existing WASM bridge tests all pass with the rename.
