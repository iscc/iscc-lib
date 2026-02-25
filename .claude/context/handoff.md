## 2026-02-25 — Add io.Reader support to Go streaming hashers

**Done:** Added `UpdateFrom(ctx context.Context, r io.Reader) error` methods to both `DataHasher`
and `InstanceHasher` in the Go package. Both methods use a 64 KiB internal buffer to read from any
`io.Reader` and delegate to the existing `Update` method, matching the Python binding's chunked-read
pattern.

**Files changed:**

- `packages/go/iscc.go`: Added `DataHasher.UpdateFrom` and `InstanceHasher.UpdateFrom` methods
- `packages/go/iscc_test.go`: Added `TestDataHasherUpdateFrom`, `TestInstanceHasherUpdateFrom`, and
    `TestDataHasherUpdateFromMultiChunk` tests plus a `smallBufReader` helper type

**Verification:** All 39 Go tests pass (36 existing + 3 new).
`grep -c 'func.*UpdateFrom' packages/go/iscc.go` returns 2. `grep 'io.Reader' packages/go/iscc.go`
finds both method signatures. `cargo clippy --workspace --all-targets -- -D warnings` is clean.
`mise run check` passes all 14 hooks.

**Next:** The CID loop is in maintenance mode on `develop`. All core functionality, bindings, docs,
and CI/release workflows are complete. The only CID-actionable item is the low-priority TypeScript
port evaluation (issues.md). Remaining work is human-gated: merge PR #3, configure publishing
credentials, and trigger a release.

**Notes:** The implementation exactly follows the pattern from next.md. The `smallBufReader` test
helper limits reads to 7 bytes per call, forcing many iterations through the `UpdateFrom` loop to
exercise the multi-chunk code path. All three new tests verify results against the one-shot
`GenDataCodeV0`/`GenInstanceCodeV0` functions using conformance vector data.
