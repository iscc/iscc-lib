# Next Work Package

## Step: Implement pure Go GenDataCodeV0 and GenInstanceCodeV0

## Goal

Add `GenDataCodeV0` and `GenInstanceCodeV0` as pure Go gen functions with `DataHasher` and
`InstanceHasher` streaming types, passing all 7 conformance vectors (4 data + 3 instance). This
continues step 5 of the Go rewrite — all algorithm dependencies (CDC, MinHash, xxh32, BLAKE3) are
already available.

## Scope

- **Create**:
    - `packages/go/code_data.go` — `GenDataCodeV0`, `DataHasher` struct, `DataCodeResult` type
    - `packages/go/code_data_test.go` — conformance tests for 4 data vectors
    - `packages/go/code_instance.go` — `GenInstanceCodeV0`, `InstanceHasher` struct,
        `InstanceCodeResult` type
    - `packages/go/code_instance_test.go` — conformance tests for 3 instance vectors
- **Modify**: (none)
- **Reference**:
    - `reference/iscc-core/iscc_core/code_data.py` — Python reference for DataHasherV0
    - `reference/iscc-core/iscc_core/code_instance.py` — Python reference for InstanceHasherV0
    - `crates/iscc-lib/src/streaming.rs` — Rust implementation (authoritative)
    - `packages/go/code_meta.go` — existing Go gen function pattern (result types, error handling)
    - `packages/go/code_content_text.go` — existing Go gen function pattern
    - `packages/go/code_meta_test.go` — existing Go conformance test pattern
    - `packages/go/cdc.go` — `AlgCdcChunks` function signature
    - `packages/go/minhash.go` — `AlgMinhash256` function signature
    - `packages/go/xxh32.go` — `Xxh32` function (for Data-Code chunk hashing)

## Not In Scope

- `DataHasher`/`InstanceHasher` with `io.Reader` interface — streaming with io.Reader is a
    convenience wrapper that should come in a later step (gen functions accept `[]byte` like the
    existing Rust `gen_data_code_v0(&[u8])` pattern)
- Other gen functions (`GenImageCodeV0`, `GenVideoCodeV0`, `GenAudioCodeV0`, `GenMixedCodeV0`,
    `GenIsccCodeV0`)
- Removing the WASM bridge (`iscc.go`, `iscc_ffi.wasm`, wazero dependency)
- Refactoring existing pure Go modules
- `conformance_selftest` function

## Implementation Notes

### GenDataCodeV0 (`code_data.go`)

Port from `crates/iscc-lib/src/streaming.rs` `DataHasher` and Python `code_data.py`:

1. **`DataCodeResult`** struct with `Iscc string` field (matching `MetaCodeResult` pattern)
2. **`DataHasher`** struct with:
    - `chunkFeatures []uint32` — accumulated xxh32 hashes of CDC chunks
    - `tail []byte` — buffered incomplete chunk from previous Push
    - `Push(data []byte)` — append data to tail, run `AlgCdcChunks`, xxh32-hash all complete chunks,
        keep last chunk as new tail (mirrors Python's `prev_chunk` pattern)
    - `Finalize(bits uint32) (*DataCodeResult, error)` — flush tail (hash if non-empty, or hash empty
        bytes if no features), compute `AlgMinhash256` on features, `EncodeComponent` with
        `MTData`/`STNone`/`VSV0`
3. **`GenDataCodeV0(data []byte, bits uint32) (*DataCodeResult, error)`** — create DataHasher,
    single Push, Finalize
4. CDC call: `AlgCdcChunks(data, false, 1024)` — `utf32=false`, `avgChunkSize=1024` (from
    `core_opts.data_avg_chunk_size`)
5. Empty input edge case: if tail is empty AND no features, hash empty bytes `Xxh32([]byte{}, 0)` to
    ensure at least one feature

### GenInstanceCodeV0 (`code_instance.go`)

Port from `crates/iscc-lib/src/streaming.rs` `InstanceHasher` and Python `code_instance.py`:

1. **`InstanceCodeResult`** struct with `Iscc string`, `Datahash string`, `Filesize uint64`
2. **`InstanceHasher`** struct with:
    - `hasher *blake3.Hasher` — BLAKE3 streaming hasher from `github.com/zeebo/blake3`
    - `filesize uint64`
    - `Push(data []byte)` — update hasher, add to filesize
    - `Finalize(bits uint32) (*InstanceCodeResult, error)` — get digest, build multihash
        (`"1e20" + hex(digest)`), `EncodeComponent` with `MTInstance`/`STNone`/`VSV0`
3. **`GenInstanceCodeV0(data []byte, bits uint32) (*InstanceCodeResult, error)`** — create
    InstanceHasher, single Push, Finalize
4. BLAKE3 API: `blake3.New()` returns `*blake3.Hasher`, `.Write(data)` for update, `.Sum(nil)`
    returns `[]byte` digest (32 bytes)

### Constants needed (already available)

- `MTData = 3`, `MTInstance = 4` — from codec.go MainType constants
- `STNone = 0`, `VSV0 = 0` — from codec.go
- CDC avg chunk size: use literal `1024` (matching `core_opts.data_avg_chunk_size`)

### Test pattern

Follow the `TestPureGo*` naming convention from existing tests. Load `data.json`, iterate
`gen_data_code_v0` / `gen_instance_code_v0` sections, decode `"stream:<hex>"` inputs, compare output
fields. For instance vectors, also verify `datahash` and `filesize` fields.

### Conformance vectors

- Data: 4 vectors — `test_0000_two_bytes_64`, `test_0001_empty_64`, `test_0002_zero_128`,
    `test_0003_static_256`. Inputs: `[stream_hex, bits]`. Output: `{iscc}`
- Instance: 3 vectors — `test_0000_empty_64`, `test_0001_zero_128`, `test_0002_static_256`. Inputs:
    `[stream_hex, bits]`. Output: `{iscc, datahash, filesize}`

## Verification

- `cd packages/go && go build ./...` exits 0
- `cd packages/go && go test -run TestPureGoGenDataCodeV0 -count=1 -v` — 4/4 data vectors PASS
- `cd packages/go && go test -run TestPureGoGenInstanceCodeV0 -count=1 -v` — 3/3 instance vectors
    PASS
- `cd packages/go && go vet ./...` exits 0
- `cd packages/go && go test ./...` — all tests pass (pure Go + WASM bridge)
- `cd packages/go && go test -race -run "TestPureGo(GenDataCodeV0|GenInstanceCodeV0)" -count=1` —
    race detector clean
- `grep -c 'func GenDataCodeV0' packages/go/code_data.go` returns 1
- `grep -c 'func GenInstanceCodeV0' packages/go/code_instance.go` returns 1
- `grep -c 'type DataHasher struct' packages/go/code_data.go` returns 1
- `grep -c 'type InstanceHasher struct' packages/go/code_instance.go` returns 1

## Done When

All verification criteria pass — `GenDataCodeV0` and `GenInstanceCodeV0` produce correct ISCC codes
for all 7 conformance vectors, `DataHasher` and `InstanceHasher` structs are implemented with
Push/Finalize API, and the full Go test suite (pure Go + WASM bridge) passes including race
detection.
