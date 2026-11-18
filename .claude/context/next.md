# Next Work Package

## Step: Go `decodeBase32` rejects spare-bit aliases; Rust and Go tests pin strict base32

## Goal

Resolve the `normal` issue "Go `decodeBase32` accepts spare-bit aliases that Rust rejects": make the
pure-Go decoder as strict as Rust (decisions.md 2026-11-18, "Base32 decoding stays strict"), and add
tests on both sides so a "match the reference" change can never relax it unnoticed. This is a Go
prerequisite for Go `EncodeSeq`/`DecodeSeq`.

## Alternatives Considered

- **Chosen:** strict Go base32 + Rust/Go strict tests. It is a one-function fix behind which every
    Go decode path already routes (`codec.go:397` is the only `b32Encoding.DecodeString` call), and
    it is independent of every other open issue.
- **Rejected:** bundling the Go `isccClean` port (same 3 Go files). That is a separate, hand-written
    design with a 4-function × 5-input-form test matrix. Together they would not fit in 6 criteria,
    and a bounce on the port would hold back this fix. Scope it next.
- **Rejected:** the napi seq binding. It is feature progress, but the target names the Go
    prerequisites first, and this step is much smaller.

## Scope

- **Create**: `crates/iscc-lib/tests/test_strict_base32.rs`
- **Modify**: `packages/go/codec.go` (`decodeBase32` only), `packages/go/codec_test.go`
- **Reference**: `crates/iscc-lib/src/codec.rs:519` (`decode_base32`, the strict model),
    `.claude/context/issues.md` → the Go `decodeBase32` entry (case table)

## Not In Scope

- The Go `isccClean` port (scheme case, whitespace, dashes, empty guard): that is the next step. In
    particular, do NOT add whitespace or newline trimming here.
- Any change to Rust source (`src/`). Rust is already strict; this step only adds tests. Keeping
    `src/` untouched also keeps the CI-only CRAP gate and the iai gate unaffected.
- Changing `decodeBase32("")`, which returns an empty slice with no error. That matches Rust; the
    empty-input guard belongs to `isccClean`.
- Asserting that a **trailing** newline is rejected by `IsccDecode`/`IsccDecompose`/`Gen*`. The
    reference strips it, and the `isccClean` step must flip that behaviour.
- Go seq functions, the docs, or any binding surface.

## Implementation Notes

- **Go fix (from the issue):** after `b32Encoding.DecodeString(upper)` succeeds, re-encode with
    `encodeBase32(decoded)` and return an error when the result differs from `upper`. The error must
    start `iscc: base32 decode error:` and say why (e.g. "non-canonical encoding"). Update the
    docstring to state the strictness.
- **Bonus effect, intended:** Go's decoder silently ignores `\r` and `\n` anywhere in the input
    (`go doc encoding/base32 Encoding.DecodeString`), so Go currently accepts `"GAAA\nAAAAAA"`. Rust
    rejects it. The re-encode compare closes this too. Pin it with a `decodeBase32` unit test for an
    *interior* `\n`.
- Case-insensitivity is preserved: compare against the upper-cased input, not the raw one.
- **Case set (from the issue; one bad input per call / per subtest):**
    - Reject `ISCC:GAAAAAAAAB` (spare bits), `ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGJ` (spare bits),
        `ISCC:GAAAAAAAAA======` (padding).
    - Accept `ISCC:GAAAAAAAAA` and `ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGI`, with the decoded fields
        matching each other where Go currently aliases them. Read the expected values off Rust
        `iscc_decode`. Do not hand-derive them.
- **Assert the reason, not just `is_err()`.** Rust messages contain `base32 decode error`
    (`trailing bits` / `invalid symbol`); Go messages must contain `base32 decode error`. A fixture
    that fails earlier (prefix check, header, length) would otherwise pass for the wrong reason.
    `GA`/`KU` are both in Go's `validPrefixes`.
- Rust: test via the public `iscc_lib::iscc_decode` and `iscc_lib::codec::decode_base32` (bare, no
    prefix). Go: test via `IsccDecode` and a direct `decodeBase32` table test.
- If an existing Go test goes red, its hand-typed fixture has non-zero spare bits. Fix the fixture
    (re-encode the intended bytes), never relax the check, and list each such fixture in the
    handoff.
- The SemVer angle is settled (decisions.md 2026-07-24, "Tightening decode input-validation is not a
    SemVer break").

## Verification

- `cargo test -p iscc-lib --test test_strict_base32` passes, and the file asserts the 3 rejects
    (each with a `base32 decode error` message check) plus the 2 canonical accepts
- `CGO_ENABLED=0 go -C packages/go test -count=1 ./...` passes and `go -C packages/go vet ./...` is
    clean
- `go -C packages/go test -count=1 -run 'Base32|Strict' ./...` runs Go tests that cover the 3 issue
    rejects through `IsccDecode`, an interior-`\n` reject through `decodeBase32`, and the 2
    canonical accepts
- `grep -l GAAAAAAAAB packages/go/codec_test.go crates/iscc-lib/tests/test_strict_base32.rs` lists
    both files
- `cargo test -p iscc-lib` and `mise run check` both exit 0

## Done When

Go rejects every input in the issue's reject set with a base32 error, accepts the canonical forms,
and both Rust and Go tests pin that behaviour, with all criteria above passing.
