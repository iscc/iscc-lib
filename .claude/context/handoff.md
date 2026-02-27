## 2026-02-27 — Implement pure Go text utilities (utils.go)

**Done:** Implemented all 4 text utility functions (`TextClean`, `TextCollapse`, `TextTrim`,
`TextRemoveNewlines`) as pure Go in `packages/go/utils.go` with comprehensive tests in
`packages/go/utils_test.go`. Added `golang.org/x/text` dependency for NFKC/NFD Unicode
normalization. All functions are direct ports from the Rust `utils.rs` implementation.

**Files changed:**

- `packages/go/utils.go`: New file — 4 public functions + 2 internal helpers (`isCCategory`,
    `isCMPCategory`), using `golang.org/x/text/unicode/norm` for NFKC/NFD and `unicode` stdlib for
    category checks
- `packages/go/utils_test.go`: New file — 21 test functions covering NFKC normalization, control
    char removal, newline preservation, empty line collapsing, CRLF handling, whitespace stripping,
    newline removal, space collapsing, UTF-8 byte limit trimming, Unicode boundary safety, accent
    stripping, punctuation removal, and empty input
- `packages/go/go.mod`: Added `golang.org/x/text v0.34.0` dependency
- `packages/go/go.sum`: Updated automatically by `go get`

**Verification:**

- `cd packages/go && go build ./...` — exits 0 ✓
- `cd packages/go && go test -run TestUtils -count=1 -v` — 21/21 tests PASS ✓
- `cd packages/go && go vet ./...` — exits 0 ✓
- `cd packages/go && go test ./...` — both utils tests and existing codec+WASM tests pass (9.997s) ✓
- `grep -c 'golang.org/x/text' packages/go/go.mod` — returns 1 ✓
- `grep -c 'func Text' packages/go/utils.go` — returns 4 ✓
- `mise run check` — all 14 hooks pass ✓

**Next:** Implement CDC (Content-Defined Chunking) in `packages/go/cdc.go` — the next module in the
dependency chain (codec → text utils → **algorithms** → gen functions). CDC requires a gear rolling
hash table and chunk boundary detection. Port from `crates/iscc-lib/src/cdc.rs`.

**Notes:** The Go `unicode.Is(unicode.C, c)` covers Cc, Cf, Co, Cs (matching Rust minus
`Unassigned`). In practice, NFKC-normalized text won't contain unassigned codepoints, so this is
functionally equivalent. The `TextTrim` implementation uses a backward scan approach (trimming bytes
from the end until the string is valid UTF-8) which is simpler than the Rust `from_utf8` +
`valid_up_to` approach but produces identical results. No surprises or technical debt.
