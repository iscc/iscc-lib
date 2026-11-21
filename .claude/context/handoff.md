## 2026-11-21 — Review of: Make Go's canonical meta JSON emit `&`, `<`, `>`, U+2028 and U+2029 raw, as RFC 8785 does

**Verdict:** PASS

**Summary:** `parseMetaJSON` in the Go package now writes the five characters raw, so
`GenMetaCodeV0` and `JsonToDataUrl` return what the Rust core returns for metadata such as
`{"title":"Tom & Jerry <3"}`. The diff stays in scope (one code file, its test file, the Go
`CLAUDE.md`), touches no `data-edge.json`, and the hard-coded expected values match the Rust core
when recomputed.

**Verification:**

- [x] `cd packages/go && CGO_ENABLED=0 go test ./... && go vet ./...` passes — both exit 0. The four
    tests (`TestParseMetaJSONStringEscaping`, `TestJsonToDataUrlStringEscaping`,
    `TestGenMetaCodeV0StringEscaping`, `TestJsonToDataUrlTomAndJerry`) pass by name, with subtests
    for `&`, `<`, `>`, U+2028, U+2029 and the combined input that holds the literal backslash case.
- [x] `gofmt -l packages/go` prints nothing
- [x] `grep -n "json.Marshal(" packages/go/code_meta.go` prints nothing — and no other non-test Go
    file calls it
- [x] `JsonToDataUrl` for `{"title":"Tom & Jerry <3"}` equals `iscc_lib.json_to_data_url` — asserted
    in `TestJsonToDataUrlTomAndJerry`; I recomputed the value from `iscc_lib` and it matches
- [x] `mise run check` exits 0
- [x] Done-when: no `data-edge.json` change in the diff

One probe beyond the list: a differential of Go (from a `git archive` copy) against `iscc_lib` on 37
inputs, comparing data-URL, `iscc` and `metahash`. All 7 committed test inputs and all 15 extra
string-escaping inputs match (escaped backslash before a real U+2028 escape, raw U+2028 in the
input, all 32 control characters, the characters in a key, U+2027/U+202A neighbours, astral
characters). Three inputs outside this step's scope differ — see below.

**Issues found:**

- Filed `low`, untriaged: "Go canonical `meta` JSON differs from the core in key order, `-0` and
    lone surrogates". Go sorts keys by UTF-8 bytes where JCS sorts by UTF-16 code units (only
    visible with an emoji key next to a key from U+E000–U+FFFF), writes `-0.0` as `-0`, and turns a
    lone surrogate escape into U+FFFD where the core rejects the input. All three are older than
    this step and need contrived input.
- The Go JCS issue is rewritten to what is left of it: the shared vector. It stays `normal`.
- Not filed: the three table tests overlap (the data-URL test decodes the payload that the first
    test already checks). Harmless.

**Codex review:** no actionable findings. Codex ran the Go suite and `go vet`, and reports that
1,457 escape-heavy inputs gave the same data-URLs, codes and metahashes as the Rust core.

**Next:** the single fan-out step that adds two sections to `data-edge.json` and to all 12
harnesses:

- `gen_meta_code_v0`: the `combined` input from `jcsStringCases` in `packages/go/code_meta_test.go`,
    with its Rust-derived `iscc`, `metahash` and `meta` values. Keep the three inputs of the new
    `low` issue out of this vector: Go fails them today.
- `gen_video_code_v0`: the ragged error vector (`[380-long, 381-long]`) with a `diverges` marker.

This closes both `normal` `[human]` issues. After that, four `normal` `[audit]` issues remain (C FFI
`(NULL, 0)`, integer wrap, published-package docs, pytest gate time).

**Notes:**

- CI on `origin/develop` is green at `2c87dfe` (the video fix of iteration 250).
- The published Go module 0.7.0 still has the escaping defect; the fix ships with the next release.
- `issues.md` is at its 300-line budget. I shortened five `low` entries to make room; a new entry
    needs an equal cut.
- No decisions.md entry: the choice between a post-pass over the encoder output and a hand-written
    string encoder was left open by next.md and is explained in the code comment. A fix for the key
    order would need a custom encoder anyway.
- No learnings.md entry: the file is at budget, and what this step taught is in the Go `CLAUDE.md`
    bullet and in the new issue.
