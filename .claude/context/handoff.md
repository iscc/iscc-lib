## 2026-11-21 — Review of: Go `TextTrim` stops panicking on a negative `nbytes`; Go hashers reject use after `Finalize`

**Verdict:** PASS

**Summary:** `TextTrim` returns `""` for `nbytes <= 0` instead of panicking. `DataHasher` and
`InstanceHasher` carry a `finalized` flag: `Push` now returns `error`, and after the first
`Finalize` both methods return `iscc: <Type> already finalized` without touching state. The diff
stays inside the scope (4 Go source files, `docs/howto/go.md`, the Streaming Types block of the Go
spec), touches no gate, and every `Push` call in the package checks the error. The `[audit]` issue
is resolved and deleted.

**Verification:**

- [x] `CGO_ENABLED=0 go test -count=1 ./...` in `packages/go` passes — `ok`, conformance and
    data-edge included
- [x] `go vet ./...` clean and `gofmt -l .` prints nothing
- [x] `go test -run 'TextTrim' -v ./...` — 11 PASS, including `("abc",-1)`, `("",-1)` and
    `("abc",0)`; no panic
- [x] `go test -run 'Hasher' -v ./...` — 8 PASS; per hasher: `Push` after `Finalize` → error, second
    `Finalize` → error and nil result, failed `Finalize(63)` then `Finalize(64)` → error,
    `Push(a); Push(b)` equals the one-shot function
- [x] `grep -n 'already finalized'` matches in `code_data.go`, `code_instance.go` and
    `docs/howto/go.md`
- [x] `mise run check` exits 0

Extra checks: `TextTrim` and `SlidingWindow` are the only exported Go functions with an `int`
parameter (grep over the non-test sources), and both now handle bad values. The handoff's claim that
the Python binding consumes on a failed `finalize` holds (`self.inner.take()` runs before the core
call in `crates/iscc-py/src/lib.rs`).

**Issues found:**

- Not filed (style): the Go tab in `docs/tutorials/getting-started.md:496` still calls
    `hasher.Push(buf[:n])` as a statement. It compiles, the error cannot occur there, and the
    example already drops every other error (`f, _ :=`, `result, _ :=`).
- Added to the existing spec-drift `[audit]` issue instead of a new entry: `specs/go-bindings.md`
    says three times that the Go API takes `io.Reader`; `packages/go` has no such API.

**Codex review:** no actionable findings. It confirmed the guards, the updated callers and the
`TextTrim` boundary, and ran `go test` and `go vet` green.

**Next:** 16 `normal` issues block v0.8.0. Candidates, in the order I would take them:

1. **Ruby floor 3.1 → 3.3** (`[human]`, decided, has a `**Spec:**` field). Touches the gemspec,
    `ci.yml`, `release.yml`, the `Gemfile` minitest hold and four doc files, so it is over 6 files:
    scope it as a fan-out of one decided behaviour, or split CI/gemspec from docs. Research first
    which Ruby series the cross-gem image builds (web lookup is allowed).
2. **`alg_cdc_chunks` overflow** (`[audit]`): a Tier 1 function panics in debug builds. Needs a new
    `data-edge.json` vector; check first whether an existing section can hold it, because a new
    section is a 12-surface fan-out.
3. A docs batch of small `normal` entries that need no research: the `KAA` Sum-Code prefix, the
    pkg-config `Version: 0.2.0`, and the Ruby `to_int` doc sentence.

**Notes:**

- New decisions.md entry (2026-11-21, Go hashers): the first `Finalize` consumes even when it fails,
    and `Push` returns `error`. The `Push` change breaks a stored method value or a caller's own
    `interface{ Push([]byte) }`; accepted in 0.x. The release notes for v0.8.0 should name it.
- The decisions.md append went over 400 lines, so the runner's `archive_overflow` rotated the oldest
    entries into `decisions-archive.md` (their titles stay binding).
- No Rust changed: CRAP and iai gates are unaffected.
- learnings.md is at its 200-line budget; nothing was added this round.
