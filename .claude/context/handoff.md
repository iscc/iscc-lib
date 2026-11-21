## 2026-11-21 — Go `TextTrim` stops panicking on a negative `nbytes`; Go hashers reject use after `Finalize`

**Done:** `TextTrim` returns `""` for any `nbytes <= 0` (before, `text[:-1]` panicked). `DataHasher`
and `InstanceHasher` carry a `finalized` flag. `Push` now returns `error`, and after `Finalize` both
`Push` and `Finalize` return `iscc: <Type> already finalized` without touching state. Every `Push`
call in the package checks the error (`GenDataCodeV0`, `GenInstanceCodeV0`, `GenSumCodeV0`).

**Files changed:**

- `packages/go/utils.go`: `TextTrim` non-positive guard + doc sentence
- `packages/go/code_data.go`: `finalized` field, `Push(...) error`, guard in `Push`/`Finalize`,
    type/method docs state the single-use rule, `GenDataCodeV0` wraps the `Push` error
- `packages/go/code_instance.go`: same for `InstanceHasher` / `GenInstanceCodeV0`
- `packages/go/code_sum.go`: both `Push` errors checked and wrapped (`iscc: push … code: %w`)
- `docs/howto/go.md`: both streaming examples check `Push`'s error; the single-use sentence now says
    `Push`/`Finalize` return an "already finalized" error and that a failed `Finalize` consumes
- `.claude/context/specs/go-bindings.md`: Streaming Types block shows `Push(data []byte) error` and
    `Finalize(bits uint32)`, plus a two-line single-use paragraph
- Tests: `utils_test.go` (3: `("abc",-1)`, `("",-1)`, `("abc",0)`), `code_data_test.go` and
    `code_instance_test.go` (4 each: Push after Finalize, second Finalize → error + nil result,
    failed `Finalize(63)` then `Finalize(64)` → error, `Push(a);Push(b)` == one-shot gen)

**Verification:**

- `CGO_ENABLED=0 go test -count=1 ./...` → `ok` (conformance + data-edge included)
- `go vet ./...` clean; `gofmt -l .` printed nothing
- `go test -run 'TextTrim' -v` → 11 PASS incl. the 3 new negative/zero cases, no panic
- `go test -run 'Hasher' -v` → 8 PASS (4 per hasher, both after-Finalize cases per hasher)
- `grep -n 'already finalized'` matches in `code_data.go`, `code_instance.go`, `docs/howto/go.md`
- `mise run check` → exit 0

**Next:** The Ruby floor 3.1 → 3.3 (decided, release blocker), or the `.NET` LTS pre-pass issue
(`skip-lts-prepass: true`, CI-only proof).

**Notes:**

- **Rule chosen for a failed `Finalize`:** the flag is set at entry, so ANY first `Finalize`
    consumes the hasher, even on invalid `bits`; the retry returns "already finalized". next.md's
    first sentence said "sets the flag only on success" but then offered this alternative ("pick one
    rule"). I picked it because it matches every sibling surface: Rust `finalize(self)` consumes,
    and the Python binding `take()`s `inner` before encoding (`crates/iscc-py/src/lib.rs` ~648). It
    also avoids the double-append trap in `DataHasher.Finalize` with no extra code. Pinned by
    `Test{Data,Instance}HasherFailedFinalizeIsFinal`.
- API: `Push` gaining an `error` return is source-compatible for statement calls but breaks any
    caller that stores the method value as `func([]byte)`. Go has no Tier-1 crate API here; this is
    the `packages/go` surface only, as next.md scoped.
- Out of scope, left alone: `docs/tutorials/getting-started.md:496` still calls
    `hasher.Push(buf[:n])` as a statement (compiles, ignores the error). The spec block's next line
    "Streaming functions also accept `io.Reader`" is false (no `io.Reader` API in `packages/go`);
    belongs to the spec/`CLAUDE.md` audit issue.
- No Rust touched; no CRAP/iai impact.
