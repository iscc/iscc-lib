## 2026-02-25 — Add Go streaming hashers (DataHasher + InstanceHasher)

**Done:** Implemented `DataHasher` and `InstanceHasher` structs in the Go binding with full
`New/Update/Finalize/Close` lifecycle, wrapping the WASM opaque pointer FFI
(`iscc_data_hasher_new/update/finalize/free` and `iscc_instance_hasher_*`). Added 2 factory methods
on `Runtime` and 3 methods on each hasher type. The Go binding now exposes 23/23 Tier 1 symbols.

**Files changed:**

- `packages/go/iscc.go`: Added `DataHasher` and `InstanceHasher` structs, `NewDataHasher` and
    `NewInstanceHasher` factory methods on Runtime, and `Update`/`Finalize`/`Close` methods on each
    hasher. Close zeroes the pointer to prevent double-free.
- `packages/go/iscc_test.go`: Added 8 new streaming hasher tests — one-shot, multi-chunk, empty
    data, and double-finalize for each of DataHasher and InstanceHasher. Streaming results are
    verified against the corresponding `Gen*CodeV0` one-shot functions.

**Verification:**

- `CGO_ENABLED=0 mise exec -- go test -v -count=1 ./...` passes — all 35 tests (27 existing + 8 new)
    PASS
- `mise exec -- go vet ./...` clean (no output)
- `grep -c 'func (rt \*Runtime)' packages/go/iscc.go` outputs 45 (≥ 45)
- `grep -c 'type.*Hasher struct' packages/go/iscc.go` outputs 2
- `grep -c 'func Test' packages/go/iscc_test.go` outputs 36 (≥ 35; 36 because TestMain counts)
- `mise run check` passes — all 14 pre-commit hooks clean

**Next:** Go binding is now at full 23/23 Tier 1 parity. Next steps could be: (1) ergonomic
`io.Reader`/`io.Writer` convenience wrappers for the streaming hashers, (2) Go package README
documentation, (3) address tracked issues (codec optimization, DataHasher buffer allocation), or (4)
work on other binding improvements from the issues list.

**Notes:** The test count is 36 (not 35) because `grep 'func Test'` also matches `TestMain` which is
a setup function, not a real test — but 36 ≥ 35 satisfies the criterion. The `break` after the first
vector in one-shot tests is intentional — we only need one vector to prove streaming equivalence,
and the conformance vectors are already fully tested by `TestGenDataCodeV0` /
`TestGenInstanceCodeV0`. The multi-chunk tests find the first vector with len ≥ 2 to split.
