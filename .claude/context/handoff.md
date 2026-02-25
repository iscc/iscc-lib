## 2026-02-25 — Review of: Create Go how-to guide and add to navigation

**Verdict:** PASS

**Summary:** Created `docs/howto/go.md` with comprehensive Go how-to content covering all 10
sections specified in next.md. Added the Go nav entry to `zensical.toml`. All API signatures
verified against `packages/go/iscc.go` — the advance agent correctly used `uint32` for `bits`
(next.md incorrectly said `int32`) and omitted the `bits` parameter from `GenIsccCodeV0` (hardcoded
to `wide=false` in Go).

**Verification:**

- [x] `test -f docs/howto/go.md` exits 0 — file exists
- [x] `grep 'go get github.com/iscc/iscc-lib/packages/go' docs/howto/go.md` exits 0 — install
    command present
- [x] `grep 'NewRuntime' docs/howto/go.md` exits 0 — runtime setup documented
- [x] `grep 'GenMetaCodeV0' docs/howto/go.md` exits 0 — code generation documented
- [x] `grep 'DataHasher' docs/howto/go.md` exits 0 — streaming documented
- [x] `grep 'TextClean' docs/howto/go.md` exits 0 — text utilities documented
- [x] `grep 'ConformanceSelftest' docs/howto/go.md` exits 0 — conformance testing documented
- [x] `grep '"Go"' zensical.toml` exits 0 — nav entry present
- [x] `grep 'howto/go.md' zensical.toml` exits 0 — nav path correct
- [x] `uv run zensical build` exits 0 — site builds with `/howto/go/` in output
- [x] All 14 pre-commit hooks pass
- [x] No quality gate circumvention in diff

**Issues found:**

- (none)

**Next:** Create `docs/howto/java.md` — the last remaining how-to guide gap. Or address tracked
issues from issues.md (codec optimization, DataHasher buffer allocation, etc.).

**Notes:** The Go guide includes an "Algorithm primitives" section that Python and Node.js guides
lack — this matches what next.md requested and aligns with the Rust guide. The Go guide is ~388
lines, consistent with the ~280-350 line range of existing guides. The advance agent's memory
correctly recorded that `bits` is `uint32` in Go (not `int32` as next.md stated).
