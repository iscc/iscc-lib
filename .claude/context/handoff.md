## 2026-02-25 — Review of: Add Go streaming hashers (DataHasher + InstanceHasher)

**Verdict:** PASS

**Summary:** The advance agent implemented `DataHasher` and `InstanceHasher` streaming types in the
Go binding, completing the 23/23 Tier 1 symbol parity. The implementation is clean, idiomatic, and
follows established patterns (writeBytes, callStringResult, lastError). All 8 new tests verify
streaming equivalence against one-shot `Gen*CodeV0` functions using real conformance vectors.

**Verification:**

- [x] `CGO_ENABLED=0 mise exec -- go test -v -count=1 ./...` — all 35 tests PASS (27 existing + 8
    new streaming hasher tests)
- [x] `mise exec -- go vet ./...` — clean (no output)
- [x] `grep -c 'func (rt \*Runtime)' packages/go/iscc.go` — outputs 45 (≥ 45)
- [x] `grep -c 'type.*Hasher struct' packages/go/iscc.go` — outputs 2
- [x] `grep -c 'func Test' packages/go/iscc_test.go` — outputs 36 (≥ 35; 36 includes TestMain)
- [x] `mise run check` — all 14 pre-commit hooks pass

**Issues found:**

- (none)

**Next:** Go binding is now at full 23/23 Tier 1 parity. Recommended next steps (in priority order):
(1) Root README Go section — add Go installation/quick-start example, Go badge, fix "What is
iscc-lib" body text to include Java, add Maven Central badge. (2) Documentation how-to guides —
`docs/howto/go.md` and `docs/howto/java.md`. (3) Address tracked issues from issues.md (codec
optimization, DataHasher buffer allocation, etc.).

**Notes:** The `break` after the first vector in one-shot streaming tests is intentional — one
vector suffices to prove streaming equivalence since full conformance is already tested by the
`TestGenDataCodeV0`/`TestGenInstanceCodeV0` suites. The multi-chunk tests correctly find the first
vector with `len(data) >= 2` for splitting. Close methods zero the pointer before checking error
return, which is the correct pattern to prevent double-free even on error.
