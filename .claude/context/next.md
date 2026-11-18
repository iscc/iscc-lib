# Next Work Package

## Step: ASCII-only upper-casing in Rust and Go base32 decoding (non-ASCII case-fold aliases)

## Goal

Resolve the `normal` issue "Base32 decoding accepts non-ASCII case-fold aliases": Rust and Go
upper-case with Unicode rules before decoding, so `ı`/`ſ` (and in Rust also `ß`/`ﬁ`) decode as
aliases of canonical codes that iscc-core rejects. Fixing it before the Go `EncodeSeq` port means
the port does not copy the alias site in `encode_seq`.

## Alternatives Considered

- **Chosen:** case-fold alias fix: small and reference-matching, and it removes the very line
    (`codec.rs:834`) a Go `EncodeSeq` port would copy.
- **Rejected:** Go `EncodeSeq`/`DecodeSeq`: it would add a third alias site or need this fix anyway.
    It comes next. Text-utils U+001C–U+001F parity is a separate iai-gated change and waits. The red
    CI is not a step: the annotations API on run 44336381374 shows a macOS arm64 runner-capacity
    cancel ("not acquired by Runner of type hosted"). Nothing in the tree causes it.

## Scope

- **Modify**: `crates/iscc-lib/src/codec.rs` (`decode_base32` L520, `encode_seq` L834),
    `crates/iscc-lib/src/lib.rs` (`iscc_normalize` prefix check, L229), `packages/go/codec.go`
    (`decodeBase32` L405 and its non-canonical error at L411, `checkPrefix` L454)
- **Tests**: `crates/iscc-lib/tests/test_strict_base32.rs`, `packages/go/codec_test.go`
- **Reference**: issues.md entry (probe table), `packages/go/codec.go` `isIsccScheme` (L547, the
    existing ASCII-only byte idiom), `reference/iscc-core/iscc_core/codec.py`

## Not In Scope

- Text-utils U+001C–U+001F trimming (separate `normal` issue).
- Go `EncodeSeq`/`DecodeSeq` or any binding-surface seq work.
- Changing other error wording (Rust "Malformed"/"Invalid scheme" echoes stay as they are).
- Editing `.iai-baseline.json` or `.crap-baseline.json` to hide a cost (see notes).
- Python/other binding tests: they all call the Rust core and inherit the fix.

## Implementation Notes

- **Rust:** swap `to_uppercase()` → `to_ascii_uppercase()` at all three sites. Make these same-line
    token swaps and keep any doc-comment edits line-count-neutral (e.g. "case-insensitive" → "ASCII
    case-insensitive" on the same line). The CI-only CRAP `--fail-regression` gate reds when lines
    below an edit point move. If lines do move, run `mise run coverage && mise run crap:baseline` in
    this step. Non-ASCII bytes then reach `data_encoding` and fail as invalid symbols with the
    existing `base32 decode error: …` message.
- **`encode_seq`** still has to upper-case *before* `iscc_clean`, so a bare lowercase
    `faaa-aaaaaaaa` keeps working. Only the casing rule changes.
- **Go `decodeBase32`:** replace `strings.ToUpper` with an ASCII-only upper-caser that maps `a`–`z`
    and copies every other byte unchanged. Write it as a small helper next to `isIsccScheme`, which
    uses the same byte idiom. `b32Encoding.DecodeString` then rejects the non-ASCII bytes ("illegal
    base32 data"), wrapped in `iscc: base32 decode error: %w`. Also drop the `%q` whole-input echo
    from the non-canonical error so it carries only the reason (Codex note carried from iter 200).
    No Go test asserts on the quoted input (`non-canonical` appears only in a comment at
    `codec_test.go:451`).
- **Go `checkPrefix`:** use the same helper. After the fix no test can tell whether the prefix sites
    changed, because `decodeBase32` rejects those inputs either way. The grep criterion below is
    their only gate.
- **Fixture traps:**
    - Put the non-ASCII char **after** the 2-char prefix. In the prefix it fails the prefix check with
        a different message, not `base32 decode error`.
    - Pair each reject with an accept of its canonical ASCII twin, one input per assertion. This
        proves the reject comes from the non-ASCII char.
    - `iscc_decode` rejects (Rust + Go): `ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGı` (twin `…CNGI`),
        `ISCC:GAAWDLQNG6ßDPF2` (twin `…NG6SSDPF2`), `ISCC:GAAT5ﬁNQWRC4HQV` (twin `…T5FINQWRC4HQV`),
        plus a U+017F case such as `ISCC:GAAWDLQNG6ſSDPF2` (twin `…NG6SSDPF2`). Assert the error
        contains `base32 decode error`. In Go only `ı` and `ſ` discriminate (`ß`/`ﬁ` already error),
        but pin all four.
    - **Rust `encode_seq` needs its own case:** `codec.rs:834` is observable even with `decode_base32`
        fixed. Use a UNIT: `KUAK…` is an ISCC-CODE and fails by MainType. For example
        `encode_seq(&["gaawdlqng6ſsdpf2"])` must Err with `base32 decode error`, while
        `encode_seq(&["gaawdlqng6ssdpf2"])` must be Ok. Also add a `ı` unit case.
- **Perf:** `to_ascii_uppercase` is cheaper than `to_uppercase`. `mise run bench:iai:check` must
    exit 0 with `.iai-baseline.json` untouched (iai is CI-only and not in `mise run check`; valgrind
    and the runner are installed).

## Verification

- `cargo test -p iscc-lib` passes, including new `test_strict_base32.rs` cases that reject the four
    `iscc_decode` inputs and the two non-ASCII `encode_seq` units with `base32 decode error`, and
    that accept their ASCII twins
- `CGO_ENABLED=0 go -C packages/go test -count=1 ./...` passes with the same four rejects and twins
    in `codec_test.go`; `go -C packages/go vet ./...` is clean and `gofmt -l packages/go` prints
    nothing
- `grep -rn "to_uppercase()" crates/iscc-lib/src` and
    `grep -n "strings.ToUpper" packages/go/codec.go` both print nothing
- `mise run bench:iai:check` exits 0 and `git diff --quiet -- .iai-baseline.json` exits 0
- `cargo clippy -p iscc-lib -- -D warnings` is clean
- `mise run check` exits 0

## Done When

Rust and Go both reject every non-ASCII case-fold alias in the issue (plus U+017F) with a base32
decode error, the ASCII twins still decode, and all six verification checks pass.
