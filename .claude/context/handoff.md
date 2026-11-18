## 2026-11-18 — Go `isccClean` helper routed through all four Go codec input sites

**Done:** Ported Rust `codec::iscc_clean` to Go `isccClean` in `codec.go`, with two helpers.
`isPyStripChar` is `unicode.IsSpace` plus U+001C–U+001F. `isIsccScheme` is an ASCII-only per-byte
`|0x20` compare against `"iscc"`. The port handles a second colon (malformed), keeps dashes on a
bare string led by a multibase prefix, and errors on an empty cleaned result. All four Go
string-input sites now call it: `IsccDecompose`, `isccNormalize` (where `checkPrefix` runs on the
cleaned string), `GenIsccCodeV0` and `GenMixedCodeV0` (both report `clean code %d: …`). Errors give
only the reason and never echo the input.

**Files changed:**

- `packages/go/codec.go`: adds `isPyStripChar`, `isIsccScheme`, `isccClean` and the `unicode`
    import. Routes `IsccDecompose` and `isccNormalize` through the helper and updates the doc
    comments on `IsccDecompose` and `IsccDecode`.
- `packages/go/code_iscc.go`: step 1 calls `isccClean` and returns an error with the code's index.
    Drops the unused `strings` import and updates the doc comment.
- `packages/go/code_content_mixed.go`: same change.
- `packages/go/clean_test.go` (new): 12 `TestIsccClean*` functions.
    - Direct `isccClean` cases: 15 accepts (5 multibase-led strings keep their dashes; a scheme input
        always loses them) and 12 rejects.
    - Metamorphic forms × 4 entry points: canonical, bare, dashed, dashed bare, whitespace incl. NBSP,
        U+001C–U+001F, lowercase, mixed case, trailing `\n`, trailing `\r\n`.
    - The reference facts from next.md: 4 inputs decode to `(3,0,0)`, and decomposing
        `"ISCC:GAAAAAAAAA\n"` gives `[GAAAAAAAAA]`.
    - 6 reject inputs × 4 entry points, one bad input per subtest: `""`, `"-"`, `"   "`, `"iscc:"`,
        `a:b:c`, `iſcc:`. The gen functions must also name the bad code's index.

**Verification:**

- `CGO_ENABLED=0 go -C packages/go test -count=1 ./...`: ok. The strict-base32 suites
    (`codec_test.go`) are unchanged and green.
- `go -C packages/go test -count=1 -run 'IsccClean' -v ./...`: 12 top-level tests and 127 PASS lines
    in total, 0 FAIL.
- `go vet ./...` is clean and `gofmt -l packages/go` prints nothing.
- The grep from next.md (`TrimPrefix(...)|EqualFold|ToLower` over the 3 files) prints nothing (exit
    1). My first doc comment mentioned `strings.EqualFold`/`strings.ToLower`, so I reworded it.
- Mutation check on a temporary copy, restored afterwards:
    - Replacing `isIsccScheme` with `strings.EqualFold` fails the 5 `long_s_scheme` subtests.
    - Dropping the U+001C–U+001F clause fails the `info_separators` subtests in every family.
- `mise run check`: exit 0.

**Next:** Go `EncodeSeq`/`DecodeSeq`. They can reuse `isccClean` directly. Alternatively, do the
Rust+Go non-ASCII case-fold alias issue first (`strings.ToUpper` in `decodeBase32`/`checkPrefix`).
That fix also replaces the `%q` whole-input echo in `decodeBase32`'s non-canonical error, which I
left alone because it was out of scope.

**Notes:**

- Rust's `iscc_clean` echoes the whole input in its "Malformed" error and the scheme in its "Invalid
    scheme" error. As next.md asked, the Go errors give only the reason
    (`iscc: malformed ISCC   string: more than one colon`, `iscc: invalid scheme: expected "iscc"`,
    `iscc: empty ISCC   string`). The two languages' messages therefore differ in text but not in
    behaviour.
- `GenIsccCodeV0` cleans the codes before its "at least 2 codes" check, so an empty code in a
    1-element list reports the clean error first. Each check still rejects its own bad inputs.
- `GenMixedCodeV0` still copies `Parts` from the raw inputs. This matches Rust, which keeps the
    caller's strings.
- The vectors I first picked for the tests were wrong:
    - `IsccDecompose` of the `gen_iscc_code_v0` test_0000 code returns 64-bit units, not the longer
        input units.
    - The mixed vector input `EUA6…` fails `checkPrefix` (`EU` is not on the allow-list), so
        `IsccDecode` rejects it. I switched to `EIAU…`.
- Go-only change: Rust is untouched and so are the CRAP and iai baselines. No bench was needed.
