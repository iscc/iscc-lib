## 2026-02-25 — Review of: Add io.Reader support to Go streaming hashers

**Verdict:** PASS

**Summary:** Added `UpdateFrom(ctx, io.Reader)` methods to both `DataHasher` and `InstanceHasher` in
the Go package, exactly matching the next.md specification. Implementation is clean, idiomatic Go,
and delegates to the existing `Update` method with a 64 KiB buffer. Three well-designed tests verify
correctness against conformance vectors and exercise the multi-chunk code path.

**Verification:**

- [x] `mise exec -- go test ./packages/go/ -v -run TestDataHasherUpdateFrom` passes — 4 subtests
    (all conformance vectors)
- [x] `mise exec -- go test ./packages/go/ -v -run TestInstanceHasherUpdateFrom` passes — 3 subtests
    (all conformance vectors)
- [x] `mise exec -- go test ./packages/go/ -v` passes all tests — 39 top-level test functions, 93
    total including subtests
- [x] `grep -c 'func.*UpdateFrom' packages/go/iscc.go` returns 2
- [x] `grep 'io.Reader' packages/go/iscc.go` finds both method signatures
- [x] `cargo clippy --workspace --all-targets -- -D warnings` remains clean
- [x] `mise run check` passes — all 14 hooks clean

**Issues found:**

- (none)

**Next:** The CID loop is in maintenance mode on `develop`. All core functionality, bindings, docs,
and CI/release workflows are complete. The only CID-actionable item is the low-priority TypeScript
port evaluation (issues.md). Remaining work is human-gated: merge PR #3, configure publishing
credentials, and trigger a release.

**Notes:** The Go bindings now have full io.Reader streaming support. The `smallBufReader` test
helper (limits reads to 7 bytes) is a good pattern for exercising chunked I/O paths. State.md's Go
section can be updated to reflect "met" status since io.Reader streaming was the last gap.
