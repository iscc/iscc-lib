# Next Work Package

## Step: Go port rejects ISCC-UNITs above 256 bits (encode and decode)

## Goal

Bring the Go port to the 256-bit unit limit that the Rust core enforces since iteration 301
(decisions.md, "A unit is at most 256 bits"). This closes the Go part of the `normal` issue "Go
accepts `bits` above 256, no vector pins the rejection, and most API pages lack the range".

## Alternatives Considered

- **Chosen:** the Go port. Go is the only surface that still returns undecodable units above 256
    bits, and the shared vector rows that come next would fail on Go until this lands. A change to
    Go behaviour goes first and alone
- **Rejected:** the `data-edge.json` rows (`diverges`) together with this step. The rows have to
    pass on all 12 harnesses, which makes that a separate cross-surface step. The three `[audit]`
    `normal` issues (conformance-table duplication, Conan sha256, stale specs) are independent and
    come after the bits chain

## Scope

- **Modify**: `packages/go/codec.go` (`encodeLength` bound and wording, a checked helper
    `unitBitLength`, the callers in `IsccDecompose` and `IsccDecode`, the `EncodeComponent` doc
    comment), `packages/go/code_content_mixed.go`, `packages/go/code_iscc.go`, `docs/howto/go.md`
    (line 493 comment only)
- **Tests**: `packages/go/codec_test.go`, plus the matching `*_test.go` files for the gen functions,
    the hashers, mixed and `GenIsccCodeV0`
- **Reference**: `crates/iscc-lib/src/codec.rs` (`encode_length` near line 370, `unit_bit_length` at
    line 438, its test `test_unit_bit_length_above_256`), `crates/iscc-lib/src/lib.rs`
    (`units_reach_128_bits` near line 940, the `*_above_256` tests)

## Not In Scope

- Any `data-edge.json` row, the generated C header, or any other binding's harness (the next step)
- The API pages and comments on other surfaces that say only "multiple of 32" (`iscc.h`,
    `IsccLib.java`, `_lowlevel.pyi`, the other how-to and reference pages): a later docs step
- The 88-bit ISCC-ID behaviour (`low` issue) and the ISCC/ID branches of `decodeLength`: they stay
    unchanged
- Changing the signature or results of `decodeLength` itself; its unit tests stay as they are

## Implementation Notes

- `encodeLength`, Meta through Flake: accept `32 <= length <= 256 && length%32 == 0`. Error text
    follows Rust:
    `iscc: invalid length %d for MainType %d (must be a multiple of 32 from 32 to 256)`. ISCC and ID
    branches unchanged. Every gen function, both hashers' `Finalize` and `EncodeComponent` reach
    `encodeLength` through `encodeComponentInternal` (codec.go:510), so they get the bound without
    their own check. Confirm this for image and video. If one of them fails earlier with a different
    error, the test still only needs an error
- `unitBitLength(mt, ln, st) (uint32, error)` mirrors Rust `unit_bit_length`: it calls
    `decodeLength` and rejects a result above 256 for every MainType except `MTIscc` and `MTId`.
    Error text: `iscc: invalid length %d bits for MainType %d (an ISCC-UNIT is at most 256 bits)`
- Replace all four callers outside `decodeLength`'s own tests: `codec.go:641` (`IsccDecompose`),
    `codec.go:792` (`IsccDecode`), `code_content_mixed.go:76`, `code_iscc.go:88-89`. In `IsccDecode`
    the bound must apply before `make([]byte, nbytes)`
- Wide check (`code_iscc.go` step 8): the `&&` chain cannot carry an error. Port Rust
    `units_reach_128_bits` as a small loop that returns `(false, nil)` at the first unit below 128
    bits, *before* it checks the next unit. Evaluate it only when `wide && n == 2 && Data+Instance`
    holds, so a 64-bit Data unit next to a 288-bit Instance unit still succeeds with `wide=true`
    (iscc-core does the same)
- Hand-built over-long units for the tests: `encodeHeader(MTMeta, STNone, V0, 8)` (length index 8 =
    288 bits) + 36 body bytes, then `encodeBase32`. Expected results, measured on Rust and on
    iscc-core 1.5.0 (handoff of iteration 301): a 288-bit Meta unit fails `IsccDecode` and
    `IsccDecompose`; a 288-bit Content unit fails `GenMixedCodeV0` (with a valid 64-bit Content unit
    as the second input); a 288-bit Data unit + 128-bit Instance unit fails
    `GenIsccCodeV0(.., true)` and succeeds with `false`
- Encode tests: `bits=288` fails for meta, text, image, audio, video, data, instance,
    `DataHasher.Finalize`, `InstanceHasher.Finalize`, and `EncodeComponent(0, 0, 0, 288, 36 bytes)`;
    `bits=256` still succeeds on each of them. Assert the rejection on the raw call (learnings: one
    bad case per call). Put `Above256` in the test names
- Go-only step: it trips neither the CRAP gate nor the iai gate. Watch the 386 leg: keep the
    arithmetic in `uint32`

## Verification

- `cd packages/go && CGO_ENABLED=0 go test -count=1 ./... && GOARCH=386 CGO_ENABLED=0 go test -count=1 ./...`
    passes, and `go test -run Above256 -v ./...` lists passing tests for encode (gen functions,
    hashers, `EncodeComponent`), decode (`IsccDecode`, `IsccDecompose`), mixed and both wide cases
- `cd packages/go && go vet ./... && GOARCH=386 go vet ./...` clean
- `git grep -n "decodeLength(" -- packages/go ':!*_test.go'` prints lines in `codec.go` only (the
    definition and the call inside `unitBitLength`)
- `git grep -n ">= 32)" -- packages/go` prints nothing;
    `git grep -n "32 to 256" -- packages/go docs/howto/go.md` shows `encodeLength`, the
    `EncodeComponent` comment and the how-to line
- `mise run check` exits 0

## Done When

Every criterion above passes and the Go port rejects a unit above 256 bits on encode and decode with
the outcomes listed in Implementation Notes.
