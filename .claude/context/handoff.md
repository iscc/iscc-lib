## 2026-11-22 — Go `AlgMinhash256` returns an error for an empty feature list

**Done:** Go `AlgMinhash256` is now `func AlgMinhash256(features []uint32) ([]byte, error)`. It
returns `nil, "features must not be empty"` for an empty or nil list and otherwise delegates to the
unexported `algMinhash256` (the old body). This mirrors `AlgSimhash`/`algSimhash`. Both internal
callers (`softHashTextV0`, `DataHasher.Finalize`) call the unchecked helper, so no gen output moves.

**Files changed:**

- `packages/go/minhash.go`: checked exported wrapper plus the unchecked `algMinhash256`; imports
    `fmt`
- `packages/go/code_content_text.go`: `softHashTextV0` calls `algMinhash256`
- `packages/go/code_data.go`: `DataHasher.Finalize` calls `algMinhash256`
- `packages/go/minhash_test.go`: `TestMinhash256Empty` now asserts the error text and a nil result
    for `[]uint32{}` and `nil`. `Single`/`Deterministic` check `err == nil`. Added
    `TestMinhash256DelegatesToUnchecked`, which pins the exported result to `algMinhash256`
- `docs/howto/go.md`: new signature plus "returns an error on an empty list"
- `.claude/context/specs/go-bindings.md`: `alg_minhash_256` row has the new signature

**Verification:**

- `go vet ./...`, `GOARCH=386 go vet ./...` and `CGO_ENABLED=0 go build ./...` are clean.
    `gofmt -l .` prints nothing
- `go test -count=1 ./...` gives ok, and `GOARCH=386 CGO_ENABLED=0 go test -count=1 ./...` gives ok.
    The `-v` run shows the 4 `TestMinhash256*` tests, `TestPureGoConformanceSelftest` and
    `TestDataEdgeVectorsPass` all PASS
- The signature grep on `minhash.go` prints line 204. The docs/spec grep prints
    `docs/howto/go.md:340` and `go-bindings.md:58`
- The `0xFF` grep on `minhash_test.go` hits only `TestMinhashCompressAllOnes` (lines 61/71/72) and
    the feature value `0xFFFFFFFF` in the delegation test. None of them tests an empty
    `AlgMinhash256`
- `mise run check` exits 0 (no `Failed` in the log)

**Next:** the `alg_minhash_256` section in `data-edge.json` as a 12-surface fan-out (one error row,
one non-empty row), with no `except` entry. It can carry the .NET XML doc / C++ `///` nit from the
last review. After that comes the `bits` above 256 issue.

**Notes:**

- `AlgMinhash256` has no other call site in `packages/go` (no bench or example), and `go vet` found
    none
- `packages/go/README.md` and `CLAUDE.md` list only the name, so they need no edit
- This changes Go's public signature. Go is a pure port with its own versioning, and the decision of
    2026-11-22 covers it, so it is not a Rust-core API break
