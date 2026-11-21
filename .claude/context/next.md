# Next Work Package

## Step: Make Go's canonical meta JSON emit `&`, `<`, `>`, U+2028 and U+2029 raw, as RFC 8785 does

## Goal

Fix issue "Go Meta-Code differs from every other surface when metadata contains `&`, `<`, `>` or
U+2028/9": `parseMetaJSON` must produce the same canonical bytes as the Rust core, so
`GenMetaCodeV0` (iscc, metahash) and `JsonToDataUrl` match every other surface. The shared
`data-edge.json` vector comes in the next step's fan-out, together with the ragged-video vector.

## Alternatives Considered

- **Chosen:** Go JCS escaping fix — a published conformance defect on one surface, and the behaviour
    must land before the fan-out step adds its shared vector (a new section fails closed on all 12
    harnesses, so both pending sections go out in one fan-out).
- **Rejected:** the fan-out of the ragged-video vector alone now — it would need a second 12-surface
    fan-out for the JCS vector one iteration later.
- **Rejected:** pytest gate time (`--benchmark-disable`) — small and independent; it does not
    unblock the fan-out.

## Scope

- **Modify**: `packages/go/code_meta.go` (`parseMetaJSON` and its doc comment)
- **Modify (docs)**: `packages/go/CLAUDE.md` (the "JSON canonicalization" bullet, about line 172)
- **Tests**: `packages/go/code_meta_test.go` and/or `packages/go/codec_test.go`
- **Reference**: `packages/go/codec.go` (`JsonToDataUrl`, line 465, shares `parseMetaJSON`); RFC
    8785 section 3.2.2.2 (string serialization)

## Not In Scope

- Adding a `gen_meta_code_v0` or `json_to_data_url` section to `data-edge.json` — the next step adds
    it to all 12 harnesses at once; adding it here breaks the other 11 harnesses.
- Full JCS number serialization (floats, large integers) or invalid-UTF-8 handling in Go — no issue
    names them; do not widen the canonicalizer beyond string escaping.
- Removing or changing Python's or any binding's JSON handling.
- Touching the Rust core.

## Implementation Notes

- `json.Marshal` escapes `&` `<` `>` as `\u0026` `\u003c` `\u003e` and U+2028/U+2029 as
    `\u2028`/`\u2029`. `json.Encoder` with `SetEscapeHTML(false)` drops the first three escapes
    only, still escapes U+2028/9, and appends a trailing `\n` that must be trimmed.
- U+2028/9 then need a post-pass that rewrites the 6-byte escapes `\u2028` / `\u2029` to the raw
    UTF-8 bytes. Trap: a naive `bytes.ReplaceAll` corrupts a string that contains a literal
    backslash followed by `u2028` text (marshalled as `\\u2028`). Only replace an escape preceded by
    an even number of backslashes — or write a small string encoder instead; choose the simpler one
    that is correct, one helper, no nested functions.
- Everything else Go already emits like JCS (Go ≥ 1.22: `\b`, `\f`, `\n`, `\r`, `\t`, lowercase
    `\u00xx` for other control chars, `/` raw, sorted keys, compact). Keep that unchanged.
- Expected values must come from the Rust core, not from Go: compute them with
    `uv run python -c "import iscc_lib; …"` (`iscc_lib.json_to_data_url`,
    `iscc_lib.gen_meta_code_v0`) and hard-code them in the Go test. Cross-check one with iscc-core
    if handy (name the interpreter).
- Suggested test input (one JSON object, Go raw string with JSON escapes):
    `{"title":"Tom & Jerry <3 > \u2028 \u2029","note":"a\\u2028b \b\f\u001f"}` — covers the five
    characters, the literal-backslash trap and the control-char forms. Also test each of the five
    characters in its own input, so one broken mapping fails one named case (one bad case per call).
- Assert the exact canonical bytes (decode the data-URL payload) plus `iscc` and `metahash` of
    `GenMetaCodeV0(name, nil, &meta, 64)` equal the Rust values.
- Update the `parseMetaJSON` doc comment and the CLAUDE.md bullet to say what it now guarantees
    (string escaping per RFC 8785) and what it still does not (number serialization).
- Go-only step: the Rust CRAP and iai gates are not affected.

## Verification

- `cd packages/go && CGO_ENABLED=0 go test ./... && go vet ./...` passes, including tests that
    assert Rust-derived canonical bytes, data-URL, `iscc` and `metahash` for inputs with `&`, `<`,
    `>`, U+2028 and U+2029, and for the literal `\\u2028` input
- `gofmt -l packages/go` prints nothing
- `grep -n "json.Marshal(" packages/go/code_meta.go` prints nothing
- `JsonToDataUrl` for `{"title":"Tom & Jerry <3"}` returns exactly what `iscc_lib.json_to_data_url`
    returns for the same string (asserted in a Go test)
- `mise run check` exits 0

## Done When

All verification criteria pass with Go's canonical meta bytes matching the Rust core for the five
characters and no `data-edge.json` change in the diff.
