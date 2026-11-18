# Next Work Package

## Step: Go `EncodeSeq` / `DecodeSeq` (ISCC-SEQ on the pure-Go port)

## Goal

Port ISCC-SEQ (IEP-0020, #45) to `packages/go`, the second of 11 surfaces after Python, and run the
`encode_seq` / `decode_seq` vectors in Go's `ConformanceSelftest`. Follows the confirmed rule
(decisions.md 2026-11-18): ASCII upper-casing before cleaning, so Go rejects the same case-fold
aliases Rust does.

## Alternatives Considered

- **Chosen:** Go seq. It is the handoff's "Next", its prerequisites (`isccClean`, strict base32,
    `asciiUpper`) all landed in 199–201, and it keeps the v0.8.0 feature set moving.
- **Rejected:** text-utils U+001C–U+001F parity (`normal` issue). It is a separate Rust hot-path
    change gated by iai. It comes next, and it should not be mixed into a Go-only diff.

## Scope

- **Create**: `packages/go/seq_test.go`
- **Modify**: `packages/go/codec.go`, `packages/go/conformance.go`,
    `packages/go/conformance_test.go` (stale comment and runner tests only)
- **Reference**: `crates/iscc-lib/src/codec.rs` (`read_unit`, `encode_seq`, `decode_seq`, approx.
    L738–870), `crates/iscc-lib/tests/test_seq.rs` (the case list to port),
    `reference/iscc-core/tests/test_seq.py`, `tests/test_seq.py` (`STRICT_BASE32_DIVERGENCES`),
    `crates/iscc-lib/src/conformance.rs` (`run_encode_seq_tests` / `run_decode_seq_tests`),
    `.claude/context/specs/rust-core.md` → "encode_seq / decode_seq (ISCC-SEQ, IEP-0020)"

## Not In Scope

- Exporting `readUnit` or the seq MainType set. Tier 2 is Rust-only, so keep both unexported
- The 33 → 35 count and docs sweep, and API entries in `packages/go/README.md`, `docs/` and
    `CLAUDE.md`. These are deferred to one sweep after the fan-out, as with IDv1
- Text-utils U+001C–U+001F parity, the Python `encode_seq` docstring issue, and any other binding
- Changing what typed `decodeHeader` / `encodeHeader` / `IsccDecode` / `IsccDecompose` accept

## Implementation Notes

- **Public API:** `EncodeSeq(units []string) ([]byte, error)` and
    `DecodeSeq(data []byte) ([]string, error)`, with Go doc comments modelled on the Rust ones. A
    nil or empty list or slice is an error.
- **Raw-`uint32` header path (porting trap).** Go's `MainType`/`SubType`/`Version` are `uint8`, so
    passing SubType 4679 to `encodeHeader` truncates it silently. Typed `decodeHeader` also rejects
    SubType > 7 and Version 1 on non-ID types. Factor out unexported raw helpers on top of
    `encodeVarnibble` / `decodeVarnibbleFromBytes`, for example
    `encodeHeaderRaw(mt, st, vs, ln uint32) ([]byte, error)` and a raw decode that returns the four
    `uint32` fields. Keep the typed functions as thin wrappers with unchanged behaviour. The
    existing Go tests are the guard.
- **`readUnit(data []byte, offset int) (int, error)`** follows the Rust check order:
    1. Use a header window of at most 8 bytes. An offset at or past `len(data)` gives an empty window
        and an error, never a panic.
    2. Raw-decode the header, re-encode it, and require the window to start with those bytes.
        `decodeHeader` silently skips a non-zero padding nibble, so this prefix compare is the only
        thing that rejects one.
    3. MainType must be one of META, SEMANTIC, CONTENT, DATA or INSTANCE.
    4. The length field must be ≤ 7.
    5. The end is offset + header length + `4*(ln+1)`, and the end must not pass `len(data)`.
    6. Never reject a unit for its SubType or Version.
- **`EncodeSeq` per unit, in this order:**
    1. Reject the unit if `strings.TrimFunc(u, isPyStripChar)` is empty.
    2. Apply `asciiUpper`, then `isccClean`, then `decodeBase32`.
    3. Require `readUnit(raw, 0) == len(raw)`, otherwise report trailing data.
    4. Append the bytes in the given order. Never sort or dedupe. `faaa-aaaaaaaa` must give
        `28000000000000`, which only works because upper-casing happens first.
- **`DecodeSeq`** builds each unit as `"ISCC:" + encodeBase32(data[off:end])`. Never use
    `encodeComponentInternal`, which is typed and would reject unknown SubTypes.
- **`asciiUpper` fast path.** Return `s` unchanged, with no allocation, when it has no byte in
    `a`–`z`. `EncodeSeq` calls it once per unit, and the review measured 26 → 34 allocations per
    `IsccDecode` without it.
- **Selftest.** `vectorEntry.Outputs` is a map, so it cannot hold the seq outputs (a `bytes:<hex>`
    string and a list). Add a separate loader for the `encode_seq` / `decode_seq` sections and two
    runners, `runEncodeSeqTests` and `runDecodeSeqTests`, called from `ConformanceSelftest`. They
    take parsed section data so a test can pass in a mutated vector. Decode `bytes:` with
    `encoding/hex`. Fix the stale "9 gen functions" / "50" / "46 vectors" comments without writing
    new hard counts.
- **Tests (`seq_test.go`).** Port the `test_seq.rs` cases with one bad input per assertion. Read the
    vectors from the embedded data or `../../crates/iscc-lib/tests/data.json`. Port every
    `STRICT_BASE32_DIVERGENCES` row (padding, both spare-bit rows, `ſ` and `ß`): `EncodeSeq` rejects
    each one and accepts its canonical twin.
- **Gates:** this is a Go-only diff, so it trips neither the iai Perf gate nor the CRAP gate. Run
    `gofmt -w` before committing.

## Verification

- `CGO_ENABLED=0 go -C packages/go test -count=1 ./...` exits 0. `go -C packages/go vet ./...` is
    clean and `gofmt -l packages/go` prints nothing
- `grep -n 'func EncodeSeq(units \[\]string) (\[\]byte, error)' packages/go/codec.go` and
    `grep -n 'func DecodeSeq(data \[\]byte) (\[\]string, error)' packages/go/codec.go` each print
    one line. `grep -n 'func ReadUnit' packages/go/*.go` prints nothing
- `seq_test.go` covers each of these with its own assertion: every `STRICT_BASE32_DIVERGENCES` row
    is rejected and its twin accepted; a unit with SubType ≥ 256 and one with Version 1 round-trip
    through `DecodeSeq`→`EncodeSeq`; a non-zero header padding nibble is rejected; an out-of-range
    `readUnit` offset returns an error; `faaa-aaaaaaaa` → `28000000000000`
- A test asserts `testing.AllocsPerRun(…, func(){ asciiUpper("GAAWDLQNG6SSDPF2") }) == 0`
- A test feeds `runEncodeSeqTests` and `runDecodeSeqTests` one mutated vector each, separately, and
    each returns `false`. With the real data, `ConformanceSelftest()` still returns `true`
- `mise run check` exits 0

## Done When

The Go port exposes `EncodeSeq`/`DecodeSeq` with Rust-equivalent strictness, its selftest runs the
seq vectors, and all verification criteria pass.
