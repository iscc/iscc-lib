# Next Work Package

## Step: Go `isccClean` helper routed through all four Go codec input sites

## Goal

Resolve issues.md "Go codec input cleaning diverges from iscc-core `iscc_clean`": port Rust
`codec::iscc_clean` to one Go helper and route every Go string-input site through it, so dashed,
whitespace-padded and lowercase-scheme ISCCs work in Go as in the reference. This is the
prerequisite target.md names before Go `EncodeSeq`/`DecodeSeq`. It also repairs the trailing-newline
rejection that iter 199's strict `decodeBase32` introduced.

## Alternatives Considered

- **Chosen:** Go `isccClean`. It comes next in target.md's suggested order and in the handoff, and a
    reference-valid input (`"ISCC:GAAAAAAAAA\n"`) is currently a Go regression.
- **Rejected:** "fix CI red" is not a step. The red tip `5abda9f` comes from the Swift job being
    `cancelled` with "not acquired by Runner of type hosted … capacity constraints" on macOS arm64.
    `macos-15` is the live label (not a retirement, cf. 196), and no file change fixes it. The next
    batch push re-runs CI.
- **Rejected:** Rust+Go non-ASCII case-fold alias fix, and text-utils U+001C–U+001F parity. Both are
    on the iai-gated Rust path and touch different files. They come after this.

## Scope

- **Create**: none (tests may go in `packages/go/codec_test.go` or a new
    `packages/go/clean_test.go`)
- **Modify**: `packages/go/codec.go` (helper + `IsccDecompose` L539 + `isccNormalize` L649-650),
    `packages/go/code_iscc.go` (L27), `packages/go/code_content_mixed.go` (L23)
- **Reference**: `crates/iscc-lib/src/codec.rs:571-634` (`is_py_strip_char`, `iscc_clean`, the model
    to port branch for branch), `reference/iscc-core/iscc_core/codec.py:762-785`, Go tests at
    `codec_test.go:402-470` (strict base32 tests that must stay green)

## Not In Scope

- The `strings.ToUpper` calls in `decodeBase32` (L404) and `checkPrefix` (L453), which belong to the
    open case-fold alias issue. Also the `%q` whole-input echo in the `decodeBase32` error (Codex
    nit), which that step fixes when it rewrites `decodeBase32`. Do not touch either here.
- Go `EncodeSeq`/`DecodeSeq` (next step; it will reuse this helper).
- Multiformat (multibase-prefixed) decoding, which stays a documented non-goal.
- Any Rust change, and Go text utilities (`TextTrim` etc.).
- `codec.go:673` (`TrimPrefix(composed.Iscc, "ISCC:")`) strips an *output* prefix. Leave it.

## Implementation Notes

- Port `codec.rs:594-634` branch for branch into `isccClean(s string) (string, error)`. These are
    the places where idiomatic Go gets it wrong:
    - **Strip set**: `unicode.IsSpace(r) || (r >= 0x1C && r <= 0x1F)` via `strings.TrimFunc`.
        `unicode.IsSpace` follows White_Space, as Rust `char::is_whitespace` does (`go doc` confirms),
        but it misses the four information separators. Apply it to the whole string and again to the
        scheme and code parts after splitting at the colon.
    - **Scheme compare must be ASCII-only.** Do NOT use `strings.EqualFold` (folds `ſ`→`s`) or
        `strings.ToLower` (maps `İ`). Compare byte-wise with ASCII lower-casing, e.g. `len==4` + a
        per-byte `|0x20` check against `"iscc"`, or a small ASCII helper. Probed on iscc_core 1.5.0:
        `iſcc:GAAAAAAAAA` and `İSCC:GAAAAAAAAA` both raise `Invalid scheme`.
    - **More than one colon** → error ("Malformed ISCC string").
    - **No-colon branch**: strip dashes unless the first byte is `f`/`b`/`v`/`z`/`u`.
    - **Empty cleaned result** → error before decode (`""`, `"-"`, `"iscc:"`, `"   "`). The reference
        raises IndexError, and `decodeBase32("")` would otherwise return an empty slice.
    - Keep error messages reason-only (do not echo large inputs), using the existing `iscc: ` prefix.
- Routing: `IsccDecompose` and `isccNormalize` call `isccClean` in place of the
    `TrimPrefix`/`ReplaceAll`. `checkPrefix` must then run on the *cleaned* string (as Rust
    `lib.rs:223-229` does). In `GenIsccCodeV0`/`GenMixedCodeV0`, clean each code and propagate the
    error, with an index in `GenMixedCodeV0`'s existing `decode code %d` style.
- Reference facts (probed live, `iscc_core` 1.5.0 / CPython 3.13). Each of these decodes to
    `(3,0,0)`, the same as `ISCC:GAAAAAAAAA`: `"ISCC:GAAAAAAAAA\n"`, `"\x1cISCC:GAAAAAAAAA\x1f"`,
    `"iscc:GAAAAAAAAA"`, `" ISCC : GAAAAAAAAA "`. `iscc_decompose("ISCC:GAAAAAAAAA\n")` gives
    `['GAAAAAAAAA']`. `"a:b:c"` raises Malformed. The interior-newline unit reject in
    `TestCodecBase32StrictRejects` must stay (it calls `decodeBase32` directly).
- Tests: use one table-driven family, named so a single `-run` pattern selects it (e.g.
    `TestIsccClean*`). Run each of 4 entry points (`IsccDecompose`, `IsccDecode`, `GenIsccCodeV0`,
    `GenMixedCodeV0`) over these forms: dashed (insert `-` every 4 chars of a real canonical code
    from `data.json`, e.g. a 4-unit ISCC-CODE and the `gen_mixed_code_v0` inputs), surrounding
    whitespace including U+001C–U+001F, lowercase `iscc:` scheme, and trailing `\n`. Each form is
    checked metamorphically: same result as the canonical input. Add rejects for the empty-cleaned
    forms, `a:b:c`, and a non-ASCII scheme (`iſcc:`). Put one bad input per subtest. Also add direct
    `isccClean` unit cases, including a multibase-led bare string keeping its dashes.
- Go-only step: it trips neither the CRAP nor the iai gate. No fixture or doc changes are needed
    beyond the doc comments on the touched functions (update the "optional ISCC: prefix" wording).

## Verification

- `CGO_ENABLED=0 go -C packages/go test -count=1 ./...` passes (includes the strict-base32 suites)
- `go -C packages/go test -count=1 -run 'IsccClean' -v ./...` runs the new family. It covers the 4
    entry points × {dashed, whitespace incl. U+001C–U+001F, lowercase scheme, trailing `\n`} plus
    the reject cases (empty-cleaned, `a:b:c`, `iſcc:`), each passing
- `go -C packages/go vet ./...` clean and `gofmt -l packages/go` prints nothing
- `grep -nE 'TrimPrefix\((isccCode|iscc|code|c), "ISCC:"\)|EqualFold|ToLower' packages/go/codec.go packages/go/code_iscc.go packages/go/code_content_mixed.go`
    prints nothing (old ad-hoc input stripping gone, no Unicode-folding scheme compare)
- `mise run check` exits 0

## Done When

All five verification checks pass with the Go suite green and the strict-base32 tests unchanged.
