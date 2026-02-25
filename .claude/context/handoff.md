## 2026-02-25 — Review of: Add 6 Go wrappers (text utils + encode + string-array functions)

**Verdict:** PASS

**Summary:** All 6 Go wrappers implemented cleanly following established patterns. Group A
(TextRemoveNewlines, TextCollapse, TextTrim, EncodeBase64) mirrors the TextClean pattern exactly.
Group B (SlidingWindow, IsccDecompose) introduces well-structured readStringArray/freeStringArray/
callStringArrayResult helpers for null-terminated pointer arrays. All 22 Go tests pass (14 existing
\+ 8 new), all 14 pre-commit hooks pass, go vet clean.

**Verification:**

- [x] `CGO_ENABLED=0 go test -v -count=1 ./...` in `packages/go/` passes — 22 tests (14 existing + 8
    new), all PASS
- [x] `go vet ./...` in `packages/go/` is clean (no output)
- [x] `grep -c 'func (rt \*Runtime)' packages/go/iscc.go` outputs 32 — next.md expected 17 but
    undercounted existing methods (pre-change was 23, not 11). The 9 new methods (6 public + 3
    private helpers) are correct
- [x] `grep 'TextRemoveNewlines\|TextTrim\|TextCollapse\|EncodeBase64\|SlidingWindow\|IsccDecompose' packages/go/iscc.go | wc -l`
    outputs 12 (≥ 6)
- [x] `mise run check` passes — all 14 pre-commit hooks clean

**Issues found:**

- (none)

**Next:** The Go Tier 1 gap is now 6 functions (down from 12). The remaining 6 split into two
groups: (1) 4 byte-buffer-returning functions (AlgSimhash, AlgMinhash256, AlgCdcChunks,
SoftHashVideoV0) needing new IsccByteBuffer read/free helpers, and (2) 2 streaming hashers
(DataHasher, InstanceHasher) needing opaque pointer lifecycle management. The byte-buffer group is
the simpler next step since it follows the same call→read→free pattern established by string and
string-array helpers.

**Notes:** The verification criterion for method count (expected 17) was based on an undercount of
existing private helper methods in next.md. Future next.md specifications should count existing
methods more carefully (use `grep -c 'func (rt \*Runtime)' packages/go/iscc.go` to get the current
count before setting the expected total).
