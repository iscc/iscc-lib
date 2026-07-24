# Handoff

## 2026-07-24 — Review of: Reject trailing bytes in Go `IsccDecode`

**Verdict:** PASS

**Summary:** The advance agent added a `len(tail) > nbytes` rejection branch to Go `IsccDecode`
(packages/go/codec.go), closing the alias gap where `ISCC:MAIGHFECJMOPMIABAA` decoded identically to
canonical `ISCC:MAIGHFECJMOPMIAB`. Implemented as a distinct "too long" branch alongside the
existing "too short" branch (so `TestCodecIsccDecodeBodyTooShort` is untouched), plus two focused
tests. `DecodeIsccID` inherits the fix via delegation. Scope is exactly next.md's — 3 source lines +
2 tests, `IsccDecompose` untouched.

**Verification:**

- [x] `go -C packages/go test -count=1 ./...` passes — `ok` (full suite, all existing + 2 new)
- [x] `CGO_ENABLED=0 go -C packages/go test -count=1 ./...` passes — `ok` (pure-Go invariant holds)
- [x] `go -C packages/go vet ./...` clean — exit 0
- [x] `go -C packages/go build ./...` succeeds — `BUILD_OK`
- [x] `IsccDecode("ISCC:MAIGHFECJMOPMIABAA")` errors ("too long"), canonical still yields 8-byte
    digest — `TestCodecIsccDecodeRejectsTrailingBytes` PASS
- [x] `DecodeIsccID(knownIsccID+"AA")` errors, canonical still decodes realm 0 / hub 1 / ts
    1751831876325218 — `TestDecodeIsccIDRejectsTrailingBytes` PASS
- [x] `IsccDecompose` unaffected — its own body loop untouched; `TestCodecDecompose*` +
    `TestCodecDecomposeConformanceVectors` PASS
- [x] Conformance holds — `TestPureGoConformanceSelftest` + `TestCodecIsccDecodeConformanceVectors`
    (incl. composite ISCC-CODE) PASS
- [x] `mise run check` — all 15 pre-commit hooks Passed

**Issues found:**

- (none in this diff) — implementation is correct, minimal, and conformance-safe.
- Filed follow-up: **Rust core `iscc_decode` has the identical trailing-byte gap** (lib.rs:234 —
    `tail.len() < nbytes` guard then `tail[..nbytes]` truncation; docstring only documents "too
    short"). Codec-wide, inherited by all 11 delegating bindings. Added to issues.md as `normal`
    `[review]`. `iscc_decode` is Tier 1 but the fix is a signature-neutral robustness change
    (rejecting previously-accepted malformed input), not an API break.

**Codex review:** Clean. Confirmed the exact-length guard "correctly rejects trailing decoded bytes
while preserving valid canonical units, including composite ISCC codes and variable-length ISCC-IDv0
values", and that the Go suite + vet pass. No actionable findings.

**Next:** Pick up the newly filed **Rust core `iscc_decode` trailing-byte hardening** (`normal`,
`[review]`) — same two-branch fix pattern in `crates/iscc-lib/src/lib.rs`, add a
`tail.len() > nbytes` rejection branch, keep the "too short" message/test intact, then
`cargo test -p iscc-lib` + conformance. This closes the gap for the stability-committed core and all
bindings that delegate to it, and it also updates the `iscc_decode` docstring (currently only
mentions "too short"). Remaining CID-doable v0.6.0 targets afterward: #49 aarch64 Python wheels and
the project-wide dependency review/refresh. Release-reliability issues (npm OIDC migration,
single-registry re-trigger) stay human-gated. Do NOT cut v1.0.0 or flip the Semver gate — both held
by Titusz.

**Notes:**

- The Go binding reimplements ISCC natively (not an FFI wrapper), so the Go fix and the Rust core
    fix are independent — landing one does not change the other. Both need doing.
- `.claude/agent-memory/define-next/MEMORY.md` was mdformat-dirtied by a prior agent commit; the
    advance agent's `mise run check` auto-fixed it and folded it into the advance commit (formatting
    only, no content change) — clean tree now.
- `.claude/context/iterations.jsonl` remains runner-owned; left unstaged.
