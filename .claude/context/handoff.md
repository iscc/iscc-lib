# Handoff

## 2026-07-24 — Reject trailing bytes in Go `IsccDecode`

**Done:** Hardened the pure-Go codec so `IsccDecode` rejects an ISCC string whose base32-decoded
body is longer than the header-declared digest length, closing the alias gap where
`ISCC:MAIGHFECJMOPMIABAA` decoded identically to canonical `ISCC:MAIGHFECJMOPMIAB`. Implemented as a
distinct "too long" branch alongside the existing "too short" branch, so
`TestCodecIsccDecodeBodyTooShort` and its error message are untouched. `DecodeIsccID` inherits the
fix through its delegation to `IsccDecode`.

**Files changed:**

- `packages/go/codec.go`: added `len(tail) > nbytes` guard in `IsccDecode` returning
    `"iscc: decoded body too long: expected %d digest bytes, got %d trailing bytes"` (3 lines).
- `packages/go/codec_test.go`: added `TestCodecIsccDecodeRejectsTrailingBytes` — asserts
    `IsccDecode("ISCC:MAIGHFECJMOPMIABAA")` errors with "too long" and the canonical form still
    yields an 8-byte digest.
- `packages/go/iscc_id_test.go`: added `TestDecodeIsccIDRejectsTrailingBytes` — asserts
    `DecodeIsccID(knownIsccID + "AA")` errors and the canonical form still decodes to realm 0 / hub
    1 / ts 1751831876325218.

**Verification:**

- `go -C packages/go build ./...` — succeeds.
- `go -C packages/go vet ./...` — clean (exit 0, "VET_OK").
- `go -C packages/go test -count=1 ./...` — ok (full suite).
- `CGO_ENABLED=0 go -C packages/go test -count=1 ./...` — ok (pure-Go invariant holds; includes both
    new tests, `ConformanceSelftest`, `TestCodecIsccDecodeConformanceVectors`, and all
    `TestCodecDecompose*` tests — verified passing in verbose targeted run).
- `IsccDecompose` unaffected as required: its own body loop (`len(body) < nbytes` /
    `rawCode = body[nbytes:]`) was not touched; all decompose tests incl. conformance vectors pass.
- gofmt: `codec.go` and `iscc_id_test.go` fully clean; `codec_test.go` has exactly the one
    pre-existing go1.26 drift hunk (`TestCodecVarnibbleBitLengths` alignment, untouched lines) — my
    new test lines produce no gofmt diff (verified via `gofmt -d | grep RejectsTrailing` → empty).
    CI pins go 1.23 with no gofmt gate.
- `mise run check` — all 15 pre-commit hooks Passed (second run; first run had mdformat auto-fix an
    unrelated pre-existing file, see Notes).

**Next:** The identical trailing-byte gap exists in the Rust Tier 1 `iscc_decode`
(`crates/iscc-lib/src/lib.rs:234` — checks only `tail.len() < nbytes`, then silently truncates via
`tail[..nbytes]`), and therefore in every binding that delegates to it (py, napi, wasm, ffi, jni,
rb, uniffi). Suggest filing/scoping a follow-up issue for the Rust core (the fix is the same
two-line exact-length guard; conformance-safe for the same byte-alignment reason). Otherwise the
v0.6.0 `normal` backlog stands: #49 aarch64 Python wheels, dependency review/refresh, npm OIDC
migration, broken single-registry re-trigger in `release.yml`.

**Notes:**

- Out-of-scope finding (not fixed, per next.md): Rust `iscc_decode` accepts trailing bytes exactly
    like the Go version did — evidence at lib.rs:225-247. This is a Tier 1 symbol; tightening it is
    a behavior change (stricter input validation, same signature), analogous to this Go fix.
- `mise run check` first run failed only because mdformat reformatted
    `.claude/agent-memory/define-next/MEMORY.md` (left mdformat-dirty by a prior agent's commit).
    The auto-fix is included in this commit to keep the tree clean; no content change, formatting
    only.
- `.claude/context/iterations.jsonl` is modified in the working tree (runner-managed); left unstaged
    per protocol.
