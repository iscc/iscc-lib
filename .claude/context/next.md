# Next Work Package

## Step: Reject trailing bytes in Go `IsccDecode`

## Goal

Harden the pure-Go codec so `IsccDecode` rejects an ISCC string whose base32-decoded body is longer
than the header-declared digest length, closing the alias gap where `ISCC:MAIGHFECJMOPMIABAA`
decodes identically to the canonical `ISCC:MAIGHFECJMOPMIAB` (issue: "Go `IsccDecode` silently
accepts trailing bytes" `normal` `[review]`). `DecodeIsccID` inherits the fix.

## Scope

- **Modify**: `packages/go/codec.go` — the length guard inside `IsccDecode`.
- **Modify**: `packages/go/codec_test.go` — add a trailing-byte rejection test (test file).
- **Modify**: `packages/go/iscc_id_test.go` — add a `DecodeIsccID` trailing-byte rejection test
    (test file).
- **Reference**: `packages/go/iscc_id.go` (`DecodeIsccID` delegates to `IsccDecode`), issues.md
    entry, `.claude/context/handoff.md` (Codex P2 finding).

## Not In Scope

- Do NOT touch `IsccDecompose` — it has its own body loop (`len(body) < nbytes` then
    `rawCode = body[nbytes:]`) that legitimately consumes trailing units in a composite sequence.
    Verify it stays unaffected; do not "harden" it.
- Do NOT change `decodeHeader`, `decodeBase32`, or the public function signatures — the fix is a
    single length-comparison change; behavior gets stricter but the API is unchanged.
- Do NOT touch the other-language codecs (Rust/WASM/etc.) — this issue is scoped to the Go binding
    only. If you suspect the same gap elsewhere, note it, don't fix it here.
- No CLAUDE.md / README doc change is required (the lenient behavior was never documented and the
    signature is unchanged).

## Implementation Notes

- Current guard in `IsccDecode` (packages/go/codec.go, ~line 594):

    ```go
    if len(tail) < nbytes {
        return nil, fmt.Errorf("iscc: decoded body too short: expected %d digest bytes, got %d", nbytes, len(tail))
    }
    ```

- Change it to reject a too-long body as well. Prefer keeping the existing "too short" branch intact
    and adding a distinct "too long" branch, so the existing `TestCodecIsccDecodeBodyTooShort`
    (which asserts the error contains `"too short"`) keeps passing:

    ```go
    if len(tail) < nbytes {
        return nil, fmt.Errorf("iscc: decoded body too short: expected %d digest bytes, got %d", nbytes, len(tail))
    }
    if len(tail) > nbytes {
        return nil, fmt.Errorf("iscc: decoded body too long: expected %d digest bytes, got %d trailing bytes", nbytes, len(tail)-nbytes)
    }
    ```

    (An equivalent single `len(tail) != nbytes` exact check is fine too, but then you MUST update
    `TestCodecIsccDecodeBodyTooShort`'s "too short" assertion — the two-branch form avoids that.)

- Why this is conformance-safe: canonical ISCC base32 round-trips exactly. A body of N whole bytes
    encodes to `ceil(8N/5)` chars and decodes back to exactly N bytes; standard/ID headers are
    byte-aligned 2-byte headers, so `tail == digest` for every canonical code. Extra base32 chars
    are the only way to make `len(tail) > nbytes`. Verified empirically: `MAIGHFECJMOPMIAB` → 10
    bytes (2 header + 8 body); `MAIGHFECJMOPMIABAA` → 11 bytes (tail 9 > 8). The full test suite
    (incl. `ConformanceSelftest` + `TestCodecIsccDecodeConformanceVectors`) guards against any
    vendored vector that relied on padding.

- Add `TestCodecIsccDecodeRejectsTrailingBytes` in codec_test.go: assert
    `IsccDecode("ISCC:MAIGHFECJMOPMIABAA")` returns a non-nil error, and (control) that
    `IsccDecode("ISCC:MAIGHFECJMOPMIAB")` still succeeds with an 8-byte digest.

- Add `TestDecodeIsccIDRejectsTrailingBytes` in iscc_id_test.go: assert
    `DecodeIsccID("ISCC:MAIGHFECJMOPMIABAA")` returns a non-nil error (fix propagates through the
    delegation), and (control) the canonical form still decodes to realm 0 / hub 1 / ts
    1751831876325218\.

- Keep new test code gofmt-clean (CI pins go 1.23; only your new lines matter — pre-existing gofmt
    drift on untouched files under newer local toolchains is a known non-issue).

## Verification

- `go -C packages/go test -count=1 ./...` passes (all existing + the 2 new tests).
- `CGO_ENABLED=0 go -C packages/go test -count=1 ./...` passes (pure-Go invariant holds).
- `go -C packages/go vet ./...` is clean (exit 0).
- New assertion: `IsccDecode("ISCC:MAIGHFECJMOPMIABAA")` returns a non-nil error, while
    `IsccDecode("ISCC:MAIGHFECJMOPMIAB")` still returns an 8-byte digest.
- New assertion: `DecodeIsccID("ISCC:MAIGHFECJMOPMIABAA")` returns a non-nil error, while
    `DecodeIsccID("ISCC:MAIGHFECJMOPMIAB")` returns realm 0 / hub 1 / ts 1751831876325218.
- `go -C packages/go build ./...` succeeds (no compile breakage).

## Done When

`IsccDecode` (and thus `DecodeIsccID`) rejects trailing-byte bodies, both new tests plus the full Go
suite pass under `CGO_ENABLED=0`, and `go vet` is clean.
