## 2026-11-21 — Go canonical `meta` JSON matches the core on UTF-16 key order, `-0` and lone surrogates

**Done:** `parseMetaJSON` in the Go port now writes RFC 8785 JSON itself, recursively: object keys
sorted by UTF-16 code units at every depth, float64 `-0` written as `0`, and scalars still on the
`SetEscapeHTML(false)` + `unescapeLineSeparators` path. `checkSurrogateEscapes` runs after
`json.Unmarshal` and rejects any lone `\uD800`–`\uDFFF` escape (pairwise walk, so `\\ud800` stays
literal text), with the error in the `iscc: invalid JSON in meta:` form. Two vectors
(`test_0002_utf16_key_order`, `test_0003_negative_zero`) were added to the `gen_meta_code_v0`
section of `data-edge.json`.

**Files changed:**

- `packages/go/code_meta.go`: `parseMetaJSON` rewritten; added `checkSurrogateEscapes`,
    `escapeUnit`, `writeCanonical`, `writeCanonicalObject`, `writeCanonicalArray`,
    `writeCanonicalScalar`, `sortedKeysUTF16`; doc comment updated; imports `slices`, `strconv`,
    `unicode/utf16`
- `packages/go/code_meta_test.go`: `jcsStructureCases` (8 payload cases: key order, `-0.0`, `-0`,
    nested in an array, nested objects, a surrogate pair value, `\\ud800` literal, numbers),
    `TestParseMetaJSONLoneSurrogate` (7 inputs), `TestGenMetaCodeV0LoneSurrogate` (both public entry
    points), `TestGenMetaCodeV0Structure`. Every expected value comes from the Rust core via
    `iscc_lib.json_to_data_url` / `gen_meta_code_v0("Hello", None, m, 64)`. Non-ASCII written as
    JSON escapes inside raw strings
- `crates/iscc-lib/tests/data-edge.json` and its 4 vendored copies: 2 vectors, each with a `note`
    naming the Go defect; key-order input written with escapes; `metahash` taken from iscc_lib
- `crates/iscc-ffi/tests/data_edge_vectors.h`: regenerated (count 1→3, total 24→26)
- `packages/go/CLAUDE.md`: "JSON canonicalization" bullet describes the writer, the UTF-16 order,
    `-0`, the surrogate check, and numbers otherwise left to Go's formatting

**Verification:**

- `cd packages/go && CGO_ENABLED=0 go test ./... && go vet ./...`: ok, vet clean
- Old `code_meta.go` (`git show HEAD:`) in a `/tmp` copy with the new tests and fixture: all 4 new
    tests FAIL, and `TestDataEdgeVectorsPass` fails on exactly `test_0002` (got
    `ISCC:AAAWKLHFXPTYLTYX`) and `test_0003` (got `ISCC:AAAWKLHFXMCEEUNP`). Copy removed
- `uv run pytest tests/test_data_edge.py tests/test_vendored_fixtures.py tests/test_gen_data_edge_vectors.py`:
    162 passed (iscc-core cross-check included, no `diverges`)
- `cargo test -p iscc-lib`: all suites ok (data-edge harness included)
- C harness (CI's gcc line): 147 passed, 0 failed. Other surfaces not run locally; they canonicalize
    in the core, which the Rust and C runs exercise
- `python3 -c "...assert len(s)==3"`: exits 0
- `uv run --script scripts/gen_data_edge_vectors.py` after the commit: `git status --porcelain`
    shows only the runner's `iterations.jsonl`
- `mise run check`: exit 0, no file modified

**Next:** the Go `GOARCH=386` CI step (issue "No CI job runs the Go port as a 32-bit build"); it
will also run this step's code on 386.

**Notes:**

- **Lone surrogate is a Go unit test only, not a vector.** serde_json (Rust harness, C generator)
    rejects `\ud800` while parsing the fixture, and the JSON-text-in-a-string form fails because
    iscc-core reads a string `meta` as a data URL (`AttributeError`, not in `REJECTIONS`). Review
    should rewrite the issue's "each is a vector" clause or record this
- Rust core probed on raw JSON text (`json_to_data_url`): `\ud800`, `\udc00`, `\ud800A`,
    `\ud800\ud800`, `\ud800` in a key all raise `invalid JSON`; `\\ud800` is accepted as literal
    text. The Go tests mirror those inputs
- Number output: Go already matched the core for `1.0`, `1e21`, `1e-7`, `9007199254740993`, `0.1`,
    `-1.5e300` (case `numbers`, bytes from the core). No ECMAScript formatter was written
- Go-only change: no CRAP, iai or semver impact; no Rust or binding code touched
