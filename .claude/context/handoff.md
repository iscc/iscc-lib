## 2026-11-18 — Go `decodeBase32` rejects spare-bit aliases; Rust and Go tests pin strict base32

**Done:** Go `decodeBase32` now re-encodes the decoded bytes and rejects the input when the result
differs from the upper-cased input. The error is
`iscc: base32 decode error: non-canonical encoding "<input>"`. This rejects spare-bit aliases and
interior `\r`/`\n` (which `encoding/base32` skips). Padding was already rejected. New Rust and Go
tests pin the issue's 3 rejects and 2 canonical accepts. Each test asserts the `base32 decode error`
reason and feeds one bad input per call/subtest.

**Files changed:**

- `packages/go/codec.go`: `decodeBase32` only. Adds the re-encode compare and a docstring that
    states the strictness.
- `packages/go/codec_test.go`: 3 tests. `TestCodecBase32StrictRejects` calls `decodeBase32` on spare
    bits, padding, interior `\n` and interior `\r`. `TestCodecIsccDecodeStrictBase32Rejects` runs
    the 3 issue rejects through `IsccDecode`. `TestCodecIsccDecodeStrictBase32Accepts` checks the 2
    canonical forms, with fields read off Rust `iscc_decode`.
- `crates/iscc-lib/tests/test_strict_base32.rs` (new): 7 tests, using public `iscc_decode` and
    `codec::decode_base32`. They cover the 3 rejects with a `base32 decode error` check, and the
    specific reasons (`trailing bits` ×2, `invalid symbol` for padding). Also a bare reject of spare
    bits, padding and interior `\n`, the 2 canonical accepts with full fields and digest, and a
    lowercase accept.

**Verification:**

- `cargo test -p iscc-lib --test test_strict_base32`: 7 passed
- `cargo test -p iscc-lib`: all green (297 lib + every integration binary).
    `cargo clippy -p iscc-lib --all-targets -- -D warnings` is clean
- `CGO_ENABLED=0 go -C packages/go test -count=1 ./...` passes. `go -C packages/go vet ./...` is
    clean
- `go test -count=1 -v -run 'Base32|Strict'`: all 3 new tests and their subtests PASS
- Mutation check: I disabled the compare (`if false && …`). The Go spare-bits, interior `\n`/`\r`
    and both `IsccDecode` spare-bit subtests then FAIL. Padding still passes because Go's decoder
    already rejected it, which is expected. I then restored the file.
- `grep -l GAAAAAAAAB …` lists both files
- `mise run check` exits 0 (with `LANG=C.UTF-8`)
- Expected values come from a throwaway Rust probe, since deleted. `ISCC:GAAAAAAAAA` gives
    `(3,0,0,0,[0;4])`. `…CNGI` gives
    `(5,5,0,0,[173,172,61,235,54,120,118,21,58,231,216,5,246,120,154,100])`. Rust's messages:
    "non-zero trailing bits at 9/28" and "invalid symbol at 10". Bare `"GAAA\nAAAAAA"` gives
    "invalid length at 10".

**Next:** The Go `isccClean` port (scheme case, the Python strip set including U+001C–U+001F,
dashes, empty guard). After it, Go `EncodeSeq`/`DecodeSeq`.

**Notes:**

- No existing Go fixture went red. No hand-typed fixture had non-zero spare bits, so no fixture
    changed.
- Rust `src/` is untouched, so the CRAP and iai gates are unaffected. The Go change is not on a
    Rust-benchmarked path.
- The Go error quotes the raw input (`%q`). Rust's message gives a position instead. Both contain
    `base32 decode error`, which is the asserted contract.
- **Out-of-scope divergence for review (probed):** case folding differs for characters whose
    uppercase expands to several characters. Rust `to_uppercase` (and Python `str.upper()`) map
    `ß`→`SS` and `ﬁ`→`FI`, while Go `strings.ToUpper` leaves them unchanged. So Rust
    `decode_base32("ﬁAA")` gives `Ok([42, 0])`, the same as `"FIAA"`, and Go rejects it ("illegal
    base32 data"). Rust thus accepts a non-canonical spelling, which arguably contradicts the strict
    decision. Not fixed here (`src/` was out of scope). Single-char mappings (`ſ`/`ı` → `SI`) agree.
- Side effect, as next.md anticipated: Go `decodeBase32("GAAAAAAAAA\n")` now errors, and so does
    `IsccDecode("ISCC:GAAAAAAAAA\n")` (both probed: "non-canonical encoding"). Before this change
    Go's decoder skipped the `\n`. The reference strips it, so the `isccClean` step must strip it
    too.
