## 2026-03-04 — Review of: Add avgChunkSize validation to Go AlgCdcChunks

**Verdict:** PASS

**Summary:** Clean implementation that adds `avgChunkSize < 2` validation to Go's `AlgCdcChunks`,
exactly mirroring the Rust core pattern. The public function now returns `([][]byte, error)`, an
unexported `algCdcChunksUnchecked` serves internal callers, and `DataHasher.Push` correctly uses the
unchecked variant. All verification criteria pass; code is minimal and correct.

**Verification:**

- [x] `cd packages/go && go test ./... -count=1` passes — 155 tests, all green
- [x] `cd packages/go && go vet ./...` clean — no output
- [x] `grep -q 'avgChunkSize < 2' packages/go/cdc.go` exits 0 — validation guard present
- [x] `grep -q 'algCdcChunksUnchecked' packages/go/code_data.go` exits 0 — internal caller uses
    unchecked variant
- [x] `grep -q '([][]byte, error)' docs/howto/go.md` exits 0 — doc updated with error return
- [x] `mise run check` passes — all 15 pre-commit hooks green

**Issues found:**

- (none)

**Codex review:** No correctness, security, or maintainability issues identified. Codex confirmed
the validation cleanly rejects `avgChunkSize < 2` without affecting existing valid call paths.

**Next:** The Go `AlgCdcChunks` validation issue is resolved. The only remaining `normal`-priority
issue is "Add release smoke tests for all binding publish pipelines." All other issues are `low`
priority. Consider working on the release smoke tests issue next, or creating a PR from develop →
main.

**Notes:** Advance agent claimed 156 Go tests but actual count is 155 (known pattern — advance agent
test counts are often slightly inaccurate). All tests pass regardless. The resolved issue was
`[human]` sourced with no `**Spec:**` field, so no spec update needed.
