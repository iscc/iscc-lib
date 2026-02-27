# Next Work Package

## Step: Implement pure Go text utilities (utils.go)

## Goal

Port the 4 text utility functions (`TextClean`, `TextCollapse`, `TextTrim`, `TextRemoveNewlines`) to
pure Go as step 2 of the Go rewrite dependency chain (codec → **text utils** → algorithms → gen
functions). These functions are prerequisites for the gen functions that perform text normalization.

## Scope

- **Create**: `packages/go/utils.go`, `packages/go/utils_test.go`
- **Modify**: `packages/go/go.mod` (add `golang.org/x/text` dependency; `go.sum` updates
    automatically)
- **Reference**: `crates/iscc-lib/src/utils.rs` (Rust implementation — primary reference),
    `reference/iscc-core/iscc_core/code_meta.py` (Python `text_clean`, `text_trim`,
    `text_remove_newlines`), `reference/iscc-core/iscc_core/code_content_text.py` (Python
    `text_collapse`), `packages/go/codec.go` (existing pure Go module — follow same patterns),
    `packages/go/codec_test.go` (test structure to follow)

## Not In Scope

- Removing the WASM bridge (`iscc.go`, `iscc_ffi.wasm`, wazero dep) — happens only after all pure Go
    modules are complete
- Modifying the existing `iscc_test.go` tests — they test the WASM bridge and remain as a regression
    suite
- Implementing `multi_hash_blake3` or any hashing functions — those belong in a later algorithms
    step
- Any CI workflow changes — the Go CI job already runs `go test ./...` which handles dependencies
    automatically

## Implementation Notes

### Functions to implement (all in package `iscc`, file `utils.go`)

**1. `TextClean(text string) string`** — display text cleaning:

1. NFKC normalize: `norm.NFKC.String(text)` (from `golang.org/x/text/unicode/norm`)
2. Remove control characters except newlines. Use Go's `unicode` stdlib package:
    - C-category check: `unicode.Is(unicode.C, c)` covers Cc, Cf, Co, Cs (matches Rust's
        `is_c_category` minus `Unassigned`; in practice NFKC text won't contain unassigned
        codepoints)
    - Newline set (preserve these): `\n` (U+000A), `\v` (U+000B), `\f` (U+000C), `\r` (U+000D),
        U+0085 (NEL), U+2028 (LS), U+2029 (PS)
    - Handle `\r\n` as single newline (normalize all newlines to `\n`)
3. Collapse consecutive empty/whitespace-only lines to at most one
4. `strings.TrimSpace()` the result

Port directly from `text_clean` in `crates/iscc-lib/src/utils.rs` lines 62-101.

**2. `TextRemoveNewlines(text string) string`** — single-line conversion:

- `strings.Join(strings.Fields(text), " ")` — Go's `strings.Fields` splits on any whitespace and
    filters empty strings, exactly matching Python's `" ".join(text.split())` and Rust's
    `split_whitespace().join(" ")`

This is a one-liner.

**3. `TextTrim(text string, nbytes int) string`** — UTF-8 byte limit trimming:

1. If `len(text) <= nbytes`, return `strings.TrimSpace(text)`
2. Take `text[:nbytes]` bytes — but this may split a multibyte rune
3. Find last valid rune boundary: iterate backwards from `nbytes` using `utf8.RuneStart(text[i])` to
    find where the last complete rune starts, then verify it's complete with
    `utf8.DecodeRuneInString`
4. Alternative simpler approach: convert to `[]byte`, take `[:nbytes]`, scan for valid UTF-8 with
    `utf8.Valid()`, if not valid use `utf8.ValidString()` up to nbytes. Simplest: iterate runes and
    accumulate byte length, stop when adding next rune would exceed nbytes
5. `strings.TrimSpace(result)`

Port from `text_trim` in `utils.rs` lines 116-126. Key behavior: multi-byte characters that would be
split are dropped entirely (matching Python's `decode('utf-8', 'ignore')`).

**4. `TextCollapse(text string) string`** — similarity hashing normalization:

1. NFD normalize: `norm.NFD.String(text)`
2. Lowercase: `strings.ToLower(result)`
3. Filter: keep chars that are NOT whitespace (`unicode.IsSpace`) AND NOT in C/M/P categories. Use
    Go's super-category range tables:
    - `unicode.Is(unicode.C, c)` — all Control categories
    - `unicode.Is(unicode.M, c)` — all Mark categories (includes diacritics/accents)
    - `unicode.Is(unicode.P, c)` — all Punctuation categories
4. NFKC normalize the filtered result: `norm.NFKC.String(filtered)`

Port from `text_collapse` in `utils.rs` lines 133-145.

### Go Unicode category mapping

The Rust code uses `unicode_general_category` crate. Go's `unicode` stdlib provides equivalent
super-category range tables (no external dependency needed for category checks):

| Rust Category             | Go Equivalent                                                                          |
| ------------------------- | -------------------------------------------------------------------------------------- |
| `is_c_category` (C)       | `unicode.Is(unicode.C, c)`                                                             |
| `is_cmp_category` (C+M+P) | `unicode.Is(unicode.C, c) \|\| unicode.Is(unicode.M, c) \|\| unicode.Is(unicode.P, c)` |
| whitespace                | `unicode.IsSpace(c)`                                                                   |

### Dependency management

Run `cd packages/go && go get golang.org/x/text@latest` to add the dependency. Only
`golang.org/x/text/unicode/norm` is imported — the rest of the module is pulled transitively.

### Test structure

Follow the `codec_test.go` pattern: `package iscc`, direct function calls (no WASM Runtime), no
external dependencies. Include tests ported from the Rust `utils.rs` test section (lines 160-263):

- `TextClean`: NFKC normalization (U+FB01 ligature → "fi"), control char removal (tab removed),
    newline preservation, empty line collapsing (`"a\n\n\nb"` → `"a\n\nb"`), CRLF handling
    (`"a\r\nb"` → `"a\nb"`), whitespace stripping, empty input
- `TextRemoveNewlines`: basic newline replacement, multi-space collapsing
- `TextTrim`: no-truncation, exact fit, truncation, Unicode boundary safety (é at 1 byte → ""),
    whitespace stripping
- `TextCollapse`: basic lowercasing + space removal, accent stripping (café → cafe), punctuation
    removal, empty input

Also include the test cases from `iscc_test.go` (testing pure Go functions directly):

- `TextClean("  Hel\uFB01 World  ")` → `"Helfi World"`
- `TextRemoveNewlines("hello\nworld\r\nfoo")` → `"hello world foo"`
- `TextCollapse("Hello, World!")` → `"helloworld"`
- `TextTrim("Hello, World! This is a long string.", 10)` → ≤10 bytes, non-empty

## Verification

- `cd packages/go && go build ./...` exits 0 (compiles alongside existing WASM code)
- `cd packages/go && go test -run TestUtils -count=1 -v` — all utils tests pass
- `cd packages/go && go vet ./...` exits 0
- `cd packages/go && go test ./...` — both new utils tests and existing codec+WASM tests all pass
- `grep -c 'golang.org/x/text' packages/go/go.mod` returns at least 1
- `grep -c 'func Text' packages/go/utils.go` returns 4 (all 4 public functions present)
- `mise run check` — all pre-commit/pre-push hooks pass

## Done When

All 4 text utility functions are implemented as pure Go in `utils.go`, all verification criteria
pass including the full test suite (`go test ./...`), and the `golang.org/x/text` dependency is
properly recorded in `go.mod`.
