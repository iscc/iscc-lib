## 2026-02-27 — Review of: Implement pure Go GenIsccCodeV0

**Verdict:** PASS

**Summary:** GenIsccCodeV0 implemented as pure Go, completing all 9 gen functions for the Go
rewrite. The code faithfully mirrors the Rust `gen_iscc_code_v0` step by step — all 5 conformance
vectors pass, all quality gates clear, and the diff is tightly scoped to the 2 new files specified
in next.md.

**Verification:**

- [x] `cd packages/go && go build ./...` exits 0
- [x] `cd packages/go && go test -run TestPureGoGenIsccCodeV0 -count=1 -v` — 5/5 vectors PASS
- [x] `cd packages/go && go vet ./...` exits 0
- [x] `cd packages/go && go test ./...` — all tests pass (pure Go + WASM bridge)
- [x] `grep -c 'func GenIsccCodeV0' packages/go/code_iscc.go` returns 1
- [x] `mise run check` — all 14 hooks pass

**Issues found:**

- (none)

**Codex review:** Codex examined the commit in detail, comparing GenIsccCodeV0 against the codec
helpers and Rust reference. It flagged potential validation gaps (body length not checked against
digest requirements, no unique MainType validation). These are non-issues — the Go implementation
faithfully mirrors the Rust reference, which also does not perform these validations. No actionable
findings.

**Next:** All 9 gen functions are complete (Meta, Text, Image, Audio, Video, Mixed, Data, Instance,
IsccCode) — 46 conformance vectors pass across all functions. The next step should be implementing
`conformance_selftest` — a function that validates all 46 conformance vectors from data.json in a
single call, ensuring the complete pure Go implementation matches the reference. After that: WASM
bridge cleanup (removing `iscc.go`, `iscc_ffi.wasm`, wazero dependency, restoring 256KB large-file
threshold).

**Notes:** The `decodedEntry` struct stores `vs Version` which is assigned but never read — this
mirrors Rust's tuple pattern where Version is decoded but unused in the ISCC-CODE assembly. Not a
bug, just slightly wasteful. The `wide` mode logic is fully implemented but only exercised with
`wide=false` (no test vectors use `wide=true`). Implementation dependency order is well maintained —
all codec helpers were already available. The Go rewrite is now at ~2,900 lines of pure Go algorithm
code + tests, with only `conformance_selftest` and cleanup remaining before the WASM bridge can be
removed.
