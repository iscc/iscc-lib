# Next Work Package

## Step: Go `AlgMinhash256` returns an error for an empty feature list

## Goal

Port the decided empty-list error (decisions.md 2026-11-22, "`alg_minhash_256` rejects an empty
feature list") to the pure-Go module, the first part of the `normal` issue "Go `AlgMinhash256` still
returns `0xFF` for an empty list, and no shared vector pins the error". After this step Go agrees
with the Rust core and the 10 surfaces built on it, so the shared `data-edge.json` section can land
as a clean 12-surface fan-out with no `except` entry.

## Alternatives Considered

- **Chosen:** Go port alone — it is a change to the Go port's behaviour and public signature, and
    "core changes go first and alone": it must land and be verified before the fan-out that pins it.
- **Rejected:** the `alg_minhash_256` `data-edge.json` section now — Go would fail it (or need an
    `except` entry that the next step deletes again). The `bits` above 256 issue is the other
    `normal` gap; it is larger (core + 12 surfaces + a vector) and the issue ordering puts it after
    the minhash work is closed.

## Scope

- **Modify**: `packages/go/minhash.go`, `packages/go/code_content_text.go`,
    `packages/go/code_data.go`
- **Modify (tests/docs, outside the cap)**: `packages/go/minhash_test.go`, `docs/howto/go.md`,
    `.claude/context/specs/go-bindings.md` (row of `alg_minhash_256`)
- **Reference**: `packages/go/simhash.go` (`AlgSimhash` / `algSimhash`: the exported-checked plus
    unexported-unchecked pattern to mirror), `crates/iscc-lib/src/minhash.rs` or wherever the Rust
    `alg_minhash_256` and its unchecked helper live (`git grep -n "fn alg_minhash_256"`)

## Not In Scope

- The `alg_minhash_256` section in `data-edge.json` and any harness edit (`data_edge_test.go`
    included) — that is the next step, a 12-surface fan-out
- The .NET XML doc / C++ `///` nit from the last review — it rides with the data-edge fan-out
- Anything about `bits` above 256 (`GenDataCodeV0` accepting 512 stays as it is here)
- Changing `minhashFn` / `minhashCompress` arithmetic, or the output of any gen function

## Implementation Notes

- New exported signature: `func AlgMinhash256(features []uint32) ([]byte, error)`. On
    `len(features) == 0` return `nil, fmt.Errorf("features must not be empty")` — the bare message,
    the way `AlgSimhash` returns `hash_digests must not be empty` (Go errors carry no
    `invalid input:` prefix). Otherwise delegate to an unexported `algMinhash256(features) []byte`
    holding today's two-line body; doc comments of both say who guarantees non-empty input.
- Switch the two internal callers to `algMinhash256`: `softHashTextV0` (`SlidingWindow` always
    yields at least one window, so features are never empty) and the data hasher's `Finalize`
    (`code_data.go` already appends one feature for empty input). No gen output may move — the
    existing conformance tests (`data.json`) and data-edge tests prove it.
- `git grep -n "AlgMinhash256" packages/go` showed no other call site (no bench, no example); if
    `go vet` finds one, update it too.
- Tests in `minhash_test.go`: replace `TestMinhash256Empty` with a test asserting a non-nil error
    whose `Error()` equals `features must not be empty` and a nil result, for both `[]uint32{}` and
    a `nil` slice; adapt `TestMinhash256Single` / `TestMinhash256Deterministic` to the two returns
    (assert `err == nil`). Keep one test that the exported function equals `algMinhash256` on a
    non-empty input, so the delegation is pinned.
- Docs: `docs/howto/go.md` line ~340 becomes `AlgMinhash256(features []uint32) ([]byte, error)` and
    names the empty-list error (match how `docs/howto/python.md` or another how-to page words it).
    Spec row in `go-bindings.md` gets the new signature. `packages/go/README.md` and `CLAUDE.md`
    list only names — no edit needed.
- Behaviour-neutral call swaps in the two gen files are part of the same pattern; the step has 3
    behaviour/code files, under the 6-file cap. Go is outside the Rust CRAP/iai gates.

## Verification

- `cd packages/go && go test ./...` passes, and `GOARCH=386 go test ./...` passes
- `cd packages/go && go vet ./...` clean and `CGO_ENABLED=0 go build ./...` exits 0
- `grep -n "func AlgMinhash256(features \[\]uint32) (\[\]byte, error)" packages/go/minhash.go`
    prints one line
- `grep -rn "0xFF" packages/go/minhash_test.go` prints no line that tests `AlgMinhash256` of an
    empty list (the old `TestMinhash256Empty` body is gone)
- `grep -n "AlgMinhash256(features \[\]uint32) (\[\]byte, error)" docs/howto/go.md .claude/context/specs/go-bindings.md`
    prints a line from each file
- `mise run check` exits 0

## Done When

All verification criteria pass, with Go's `AlgMinhash256` rejecting an empty list with
`features must not be empty` and every gen-function conformance test still green.
