# Next Work Package

## Step: Go canonical `meta` JSON matches the core on UTF-16 key order, `-0` and lone surrogates

## Goal

Fix issue "Go canonical `meta` JSON differs from the core in key order, `-0` and lone surrogates":
`parseMetaJSON` (shared by `GenMetaCodeV0` and `JsonToDataURL`) must return the Rust core's result
for all three inputs, and the two inputs the fixture can hold become `data-edge.json` vectors.

## Alternatives Considered

- **Chosen:** Go canonical `meta` — the only open correctness defect where a pure-Go user gets a
    different Meta-Code than every other surface for the same input; state.md ranks it first.
- **Rejected:** Go `GOARCH=386` CI step (handoff "Next") — one `ci.yml` line that guards code that
    is already correct; it adds a test leg and fixes no output. It is the next step after this one
    (and will then also run the code from this step on 386).

## Scope

- **Modify**: `packages/go/code_meta.go` (`parseMetaJSON` and its helpers; update its doc comment)
- **Modify** (fixture, not counted): `crates/iscc-lib/tests/data-edge.json` + its 4 vendored copies
    (`packages/{go/testdata,dotnet/Iscc.Lib.Tests/testdata,kotlin/src/test/resources,swift/Tests/IsccLibTests}/data-edge.json`,
    must stay byte-identical — `tests/test_vendored_fixtures.py`)
- **Regenerate**: `crates/iscc-ffi/tests/data_edge_vectors.h` via `scripts/gen_data_edge_vectors.py`
- **Modify** (docs): `packages/go/CLAUDE.md` → "JSON canonicalization" bullet (it still says numbers
    are not canonicalized)
- **Tests**: `packages/go/code_meta_test.go` (`parseMetaJSON` table at ~line 165)
- **Reference**: `crates/iscc-lib/src/lib.rs:158` (`parse_meta_json`, uses
    `serde_json_canonicalizer`), `packages/go/data_edge_test.go` (`edgeMetaArg`: object → raw JSON
    text), `.claude/context/specs/edge-vectors.md`

## Not In Scope

- Invalid raw UTF-8 bytes in the Go `meta` string (Rust `&str` cannot carry them) — no change
- Rewriting Go number formatting beyond `-0`: `1.0`, `1e21`, `1e-7`, `9007199254740993` already
    match (measured in the issue). Do not hand-roll an ES6 number formatter
- Any harness code change on any surface; any `except` or `diverges` entry
- The Go 386 CI step, `jsonHasContext`, other Go files

## Implementation Notes

Probed today, `iscc-core 1.5.0` (py3.13) vs `iscc_lib 0.7.0`,
`gen_meta_code_v0("Hello", "", m, 64)`:

| `meta`                          | both return                                                                           |
| ------------------------------- | ------------------------------------------------------------------------------------- |
| `{"\ue000":1,"\ud83d\ude00":2}` | `ISCC:AAAWKLHFXP2I3X3H`, meta `data:application/json;base64,eyLwn5iAIjoyLCLugIAiOjF9` |
| `{"a":-0.0}`                    | `ISCC:AAAWKLHFXMHM7WHO`, meta `data:application/json;base64,eyJhIjowfQ==`             |
| dict with value `"\ud800"`      | both raise (`UnicodeEncodeError`, a `ValueError`)                                     |

So the two object vectors need no `diverges`. Derive `metahash` with iscc_lib, not by hand. Name
them `test_0002_utf16_key_order` and `test_0003_negative_zero` in the `gen_meta_code_v0` section,
each with a `note` naming the Go defect. Write the key-order input with escapes so the file stays
ASCII; `-0.0` is plain JSON.

**Lone surrogate — Go unit test only, no vector.** It cannot sit in the fixture: serde_json (Rust
harness, the C generator) rejects `\ud800` while parsing the file. The JSON-text-in-a-string trick
also fails, because iscc-core reads a string `meta` as a Data-URL and raises `AttributeError`, which
`tests/test_data_edge.py` `REJECTIONS` does not count. Say this in the handoff so review can rewrite
the issue's "each is a vector" clause or record it.

Go approach (hypothesis — implement the intent):

- `encoding/json` decodes `\ud800` to U+FFFD silently, so detect lone surrogate escapes on the raw
    input before or while decoding: a `\uD800`–`\uDBFF` escape not directly followed by a
    `\uDC00`–`\uDFFF` escape, or a low surrogate on its own, is an error. Mind `\\u...` (an escaped
    backslash, then literal text) — the existing `unescapeLineSeparators` shows the pairwise walk.
- Write objects yourself, recursively: sort keys by UTF-16 code units (`utf16.Encode([]rune(k))`,
    lexicographic), keep strings on the current RFC 8785 path (encoder with `SetEscapeHTML(false)` +
    `unescapeLineSeparators`), and emit a float64 `-0` as `0`. Nested objects inside arrays too.
- Keep the function's error messages in the `iscc: invalid JSON in meta:` form.

Go-only plus a fixture edit: no CRAP, iai or semver impact. Every other harness already runs the
meta section and canonicalizes in the core, so the two vectors must pass unchanged on 12 surfaces;
if one does not, that is a finding for the handoff, not a code skip.

## Verification

- `cd packages/go && CGO_ENABLED=0 go test ./... && go vet ./...` passes (incl. the data-edge
    harness on the two new vectors and unit tests for all three inputs)
- `uv run pytest tests/test_data_edge.py tests/test_vendored_fixtures.py tests/test_gen_data_edge_vectors.py`
    passes (iscc-core cross-check agrees on both vectors without `diverges`)
- `cargo test -p iscc-lib` passes (Rust data-edge harness)
- `uv run --script scripts/gen_data_edge_vectors.py` leaves
    `git status --porcelain crates/iscc-ffi/tests/data_edge_vectors.h` empty after the commit
- `python3 -c "import json;s=json.load(open('crates/iscc-lib/tests/data-edge.json'))['gen_meta_code_v0'];assert len(s)==3"`
    exits 0
- `mise run check` exits 0

## Done When

Go returns the core's canonical bytes for all three inputs, the two object inputs are shared vectors
green on Go, Rust and Python, and all checks above pass.
