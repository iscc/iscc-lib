## 2026-11-21 — Review of: Go canonical `meta` JSON matches the core on UTF-16 key order, `-0` and lone surrogates

**Verdict:** PASS

**Summary:** `parseMetaJSON` in the Go port now writes RFC 8785 JSON itself: keys sorted by UTF-16
code units at every depth, `-0` written as `0`, and a lone surrogate escape rejected. Two shared
vectors (`test_0002_utf16_key_order`, `test_0003_negative_zero`) pin the first two on all surfaces.
The diff stays in scope (one Go source file, tests, fixture copies, the regenerated C header, the Go
`CLAUDE.md`), and a differential against the Rust core found no difference.

**Verification:**

- [x] `cd packages/go && CGO_ENABLED=0 go test ./... && go vet ./...` — ok, vet and `gofmt -l`
    clean; the four new tests and `TestDataEdgeVectorsPass` pass by name (`-v`)
- [x] `uv run pytest tests/test_data_edge.py tests/test_vendored_fixtures.py tests/test_gen_data_edge_vectors.py`
    — 162 passed, iscc-core agrees on both vectors without `diverges`
- [x] `cargo test -p iscc-lib` — all suites ok (data-edge harness: 7 passed)
- [x] `uv run --script scripts/gen_data_edge_vectors.py` — `git status --porcelain` for the header
    is empty
- [x] `python3 -c "...assert len(s)==3"` — exit 0
- [x] `mise run check` — exit 0, no file modified

Probes beyond next.md:

- Differential of Go `JsonToDataUrl` against `iscc_lib.json_to_data_url` over 4,205 JSON texts (22
    surrogate forms in values and keys, 59 number literals, 25 keys in both orders, duplicate keys,
    invalid JSON, 4,000 random documents with random 64-bit doubles and integers up to 2^70): 0
    differences. The same probe on the previous `code_meta.go` reports differences in the key,
    number and random rows, so it can see the defect
- `GOARCH=386 CGO_ENABLED=0 go test ./...` passes
- The napi (`node --test __tests__/data_edge.test.mjs`) and JNI (`mvn -o test`) harnesses pass on
    the two new vectors. Advance ran Rust, Python, C and Go. Codex reports .NET, Kotlin and Swift
    green

**Issues found:**

- (none). The issue "Go canonical `meta` JSON differs from the core in key order, `-0` and lone
    surrogates" is resolved and deleted
- The lone surrogate has no vector, and none is owed: serde_json fails on the fixture file itself,
    so the input is not one "every language can express in JSON" (`edge-vectors.md` → Rules). The Go
    unit tests cover it (7 inputs, both public entry points, message asserted). Recorded in
    learnings.md
- Style notes, not filed: `writeCanonicalScalar` builds one encoder per scalar, and
    `checkSurrogateEscapes` scans the input a second time. Both are fine for `meta` sizes

**Codex review:** no actionable findings. It ran the Go suite (amd64 and 386), the Rust tests, the
fixture checks, and the C, .NET, Kotlin and Swift edge-vector tests; all passed.

**Next:** the Go `GOARCH=386` CI step (issue "No CI job runs the Go port as a 32-bit build", decided
2026-11-21). It is one step in the `go` job of `ci.yml`, and it also runs this step's canonicalizer
on 386. The suite passes on 386 locally today.

**Notes:**

- Go's `encoding/json` float64 output matched the core's ECMAScript number form on every probed
    number, random doubles included. No hand-written number formatter is needed; do not scope one
    without a failing input
- The two vectors run on Ruby, WASM, C++ and UniFFI for the first time in CI after this push. Those
    harnesses canonicalize in the core, so a failure there would be a harness JSON round-trip
    defect, not a core defect
- issues.md: 234 lines. learnings.md is at its 200-line budget: a new bullet needs an equal cut.
    decisions.md is at 398 of 400; no entry was added this iteration
