## 2026-11-21 — Make Go's canonical meta JSON emit `&`, `<`, `>`, U+2028 and U+2029 raw, as RFC 8785 does

**Done:** `parseMetaJSON` now serializes with a `json.Encoder` (`SetEscapeHTML(false)`, trailing
`\n` trimmed) and then runs one helper, `unescapeLineSeparators`, which rewrites `\u2028`/`\u2029`
to raw UTF-8. The helper steps through escape sequences two bytes at a time, so an escaped backslash
followed by literal `u2028` text stays as it is. `GenMetaCodeV0` and `JsonToDataUrl` both use
`parseMetaJSON`, so they are fixed too.

**Files changed:**

- `packages/go/code_meta.go`: `parseMetaJSON` rewritten plus its doc comment; added
    `unescapeLineSeparators`; added the `bytes` import. `json.Marshal(` no longer appears in the
    file.
- `packages/go/code_meta_test.go`: shared table `jcsStringCases` with 6 cases: amp, lt, gt,
    line_separator, paragraph_separator, combined (the next.md input, including `a\\u2028b` and
    `\b\f\u001f`). Three table tests each run every case as its own subtest: canonical bytes, then
    data-URL plus decoded payload, then `GenMetaCodeV0` iscc/metahash/meta. A fourth test,
    `TestJsonToDataUrlTomAndJerry`, checks an exact data-URL.
- `packages/go/CLAUDE.md`: rewrote the "JSON canonicalization" bullet. It now says strings follow
    RFC 8785, warns never to use `json.Marshal` here, and notes that numbers are still not
    canonicalized per JCS.

**Verification:**

- Expected values come from the Rust core: `uv run python` with `iscc_lib.json_to_data_url` and
    `iscc_lib.gen_meta_code_v0("Name", None, meta, 64)`, hard-coded in the test. Cross-check:
    iscc-core under `uvx --python 3.13` returns the same iscc `ISCC:AAAX74W77W3IKDKI`, metahash and
    data-URL for the combined case.
- `CGO_ENABLED=0 go test ./... && go vet ./...` in `packages/go`: ok, the 4 tests PASS by name
- `gofmt -l .`: empty. `grep -n "json.Marshal(" code_meta.go`: empty
- Mutation 1, `SetEscapeHTML(true)`: amp/lt/gt/combined fail in all 3 table tests, and TomAndJerry
    fails.
- Mutation 2, naive `bytes.ReplaceAll` instead of the helper: only `combined` fails, in all 3 table
    tests. This shows the literal-backslash case catches the trap.
- The source was restored after both mutations, and the suite passed again.
- `mise run check`: exit 0. The first run failed because mdformat reflowed the CLAUDE.md bullet; the
    rerun was clean.
- `git diff --stat -- crates/iscc-lib/tests/data-edge.json`: empty

**Next:** a single fan-out step that adds two `data-edge.json` sections to all 12 harnesses: the
`gen_meta_code_v0` JCS vector (the `combined` input above with its Rust values is ready to reuse)
and the `gen_video_code_v0` ragged-input error vector (`[380-long, 381-long]`, marked `diverges`).

**Notes:**

- Go-only change, so no Rust CRAP or iai impact.
- The doc comment now states that numbers go through Go's float64 formatting unchanged. Nothing in
    this step checks number serialization against JCS.
