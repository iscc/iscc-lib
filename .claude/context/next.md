# Next Work Package

## Step: Go `TextTrim` stops panicking on a negative `nbytes`; Go hashers reject use after `Finalize`

## Goal

Fix the `normal` [audit] issue "Go `TextTrim` panics on a negative `nbytes` and Go hashers accept
`Push` after `Finalize`". Right now the library panics, or silently returns a wrong code, on input
that every sibling surface rejects.

## Alternatives Considered

- **Chosen:** the Go correctness pair. A panic and a silently wrong code are the most serious
    defects among the 17 open `normal` issues. The fix stays inside `packages/go` (4 source files)
    and is fully verifiable with local `go test`.
- **Rejected:** the Ruby floor 3.1 → 3.3 (handoff option 1). It is decided and blocks the release,
    but it touches `ci.yml` and `release.yml` and needs research into which Ruby series the
    cross-gem image can build. The last iteration was CI-only, so a user-facing correctness fix goes
    first. Also rejected: `alg_cdc_chunks` overflow. That step changes the Rust core, and its vector
    needs a new `data-edge.json` section, which is an unsplittable 12-surface fan-out.

## Scope

- **Modify**: `packages/go/utils.go` (`TextTrim`), `packages/go/code_data.go` (`DataHasher`),
    `packages/go/code_instance.go` (`InstanceHasher`), `packages/go/code_sum.go` (check the `Push`
    errors), `docs/howto/go.md` (streaming section + single-use sentence),
    `.claude/context/specs/go-bindings.md` ("Streaming Types" block only)
- **Tests**: `packages/go/utils_test.go`, `packages/go/code_data_test.go`,
    `packages/go/code_instance_test.go`
- **Reference**: `crates/iscc-py/src/lib.rs` (~640-740: the "already finalized" errors),
    `crates/iscc-lib/src/utils.rs` (`text_trim`)

## Not In Scope

- No `data-edge.json` vector. A negative `nbytes` is an out-of-range integer, which the decision of
    2026-11-21 keeps as a per-binding test. Hasher misuse cannot be written as JSON.
- Do not change `TextTrim`'s signature to `uint` or `(string, error)`. Callers passing an `int`
    variable would stop compiling.
- No `io.Writer` implementation on the hashers, and no exported sentinel error (the package has
    none; it uses `fmt.Errorf`).
- The other stale claims in `go-bindings.md` and `packages/go/CLAUDE.md` (IDv1 "only on develop",
    test list, tag) belong to the separate spec/`CLAUDE.md` audit issue. Edit only the Streaming
    Types block.
- `docs/architecture.md`, other bindings, and the Rust core.

## Implementation Notes

- **`TextTrim`:** clamp: `if nbytes <= 0 { return "" }` before the length check (`text[:0]` would
    strip to `""` anyway). Say so in the doc comment ("a non-positive nbytes returns the empty
    string"). Go has no unsigned convention for lengths, and its only other exported `int` parameter
    (`SlidingWindow` `width`) already rejects bad values. With this change no exported Go `int`
    parameter can panic.
- **Hashers:** add a `finalized bool` field to both types. Change `Push(data []byte)` to
    `Push(data []byte) error`. Adding a return value keeps every statement call compiling
    (`hasher.Push(buf[:n])` in the docs and in callers' code). Once `finalized` is set, `Push` and
    `Finalize` return `fmt.Errorf("iscc: DataHasher already finalized")` (or `InstanceHasher`)
    without touching any state. `Finalize` sets the flag only on success. If `EncodeComponent` fails
    (bad `bits`), the call errors and the flag stays unset; follow that rule for both types and
    state it in the doc comment. **Trap:** `DataHasher.Finalize` appends the tail feature to
    `h.chunkFeatures` before encoding, so a retry would append it twice. Build the feature list
    without mutating state, or set `finalized` before encoding and treat any error as final. Pick
    one rule and test it.
- `GenDataCodeV0`, `GenInstanceCodeV0` (fresh hasher) and `GenSumCodeV0` (`code_sum.go:38-39`)
    should check the `Push` error and return it (wrap it as `iscc: …: %w`, the way `code_sum.go`
    wraps its other errors). That keeps one rule on the surface: no ignored `Push` errors in the
    package.
- Docs: `docs/howto/go.md:284`. Replace "do not call…" with: after `Finalize`, `Push` and `Finalize`
    return an "already finalized" error. Update the two streaming examples to check `Push`'s error
    (`if err := hasher.Push(buf[:n]); err != nil { … }`). In the spec block, write
    `Push(data []byte) error` and `Finalize(bits uint32)`, which is the real Go shape (the spec
    still shows `Update` and `int`).
- Tests (one bad case per test function):
    - `TextTrim("abc", -1)`, `TextTrim("", -1)` and `TextTrim("abc", 0)` each return `""`.
    - For each hasher: `Push` after `Finalize` returns an error.
    - For each hasher: a second `Finalize` returns an error and a nil result.
    - An invalid `bits` on the first `Finalize`, then a valid one: assert the rule chosen above (a
        retry gives `GenDataCodeV0`'s code, or an error).
    - The `Push(a); Push(b); Finalize` == `GenDataCodeV0(a+b)` invariant still holds.
- This step touches no Rust and no hot paths, so the CRAP and iai gates are untouched.

## Verification

- `cd packages/go && CGO_ENABLED=0 go test -count=1 ./...` passes, conformance and data-edge
    included
- `cd packages/go && go vet ./...` clean, and `gofmt -l .` prints nothing
- `go test -run 'TextTrim' -v ./...` shows the negative/zero cases passing, with no panic
- `go test -run 'Hasher' -v ./...` shows, for each hasher, both after-`Finalize` cases returning an
    error
- `grep -n 'already finalized' packages/go/code_data.go packages/go/code_instance.go docs/howto/go.md`
    matches in all three files
- `mise run check` exits 0

## Done When

All six verification criteria pass, and `TextTrim(x, -1)` plus both hashers' use after `Finalize`
are pinned by Go tests.
