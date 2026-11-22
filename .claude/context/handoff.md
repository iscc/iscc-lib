## 2026-11-22 — Go parses `data.json` once and adopts the Rust runner's malformed-vector policy

**Done:** `conformance.go` now parses every `data.json` section once, into one `vectorEntry` type
whose outputs stay raw. `seqVector`, `seqSections` and `parseSeqConformanceData` are gone. Each gen
runner is split into an inputs parser (`metaInputs`, `textInputs`, `imageInputs`, `audioInputs`,
`videoInputs`, `mixedInputs`, `streamInputs`, `isccInputs`, plus `decodeSeqInputs`) and the call.
`expectedISCC` returns `(string, error)` and runs before the gen function. Bare hex without
`stream:` and a missing or null `iscc` output now return `errUnparsable`, as in Rust. Every
per-function test reads the embedded fixture through `conformanceSection(t, name)` and the shared
parsers.

**Files changed:**

- `packages/go/conformance.go`: one parse; shared input parsers; `decodeStream` uses `CutPrefix`;
    `expectedISCC` matches the `iscc` key exactly and rejects null or non-string values
- `packages/go/conformance_test.go`: `loadConformance` (`sync.OnceValues`), `loadConformanceData`,
    `conformanceSection`; added the `bare_hex` and `no_iscc` failure arms; new test
    `TestGenRunnerRejectsMalformedVectorsAsUnparsable` (`errors.Is(err, errUnparsable)` for
    bare_hex, no_iscc, null_iscc, and a seq-shaped `outputs` string)
- `code_{meta,data,instance,iscc}_test.go`, `code_content_{text,image,audio,video,mixed}_test.go`:
    shared parsers plus `expectedISCC`; the extra fields are read from a per-test struct over
    `vec.Outputs`. The inline meta dispatch and the `TrimPrefix` stream decoding are deleted
- `packages/go/codec_test.go`: both data.json tests use `conformanceSection` + `isccInputs` /
    `expectedISCC` (helper `decodeExpectedISCC`); the `t.Skipf` calls are gone
- `packages/go/seq_test.go`: the roundtrip test uses `decodeSeqInputs`
- `packages/go/CLAUDE.md`: updated "Conformance tests" and the "Conformance vector path" pitfall

**Verification:**

- `CGO_ENABLED=0 go test ./...` ok; `go vet ./...` clean; `GOARCH=386 go test ./...` ok; `gofmt -l`
    prints nothing
- `grep -n 'tests/data\.json"' packages/go/*.go` → nothing.
    `grep -n 'parseSeqConformanceData\|seqVector\|TrimPrefix(.*"stream:"'` → nothing.
    `grep -c 'func metaArg' conformance.go` → `1`
- `go test -run 'TestGenSectionFailureArms|TestGenSectionPositiveCase|TestGenRunnerRejects…' -v`:
    PositiveCase PASS. FailureArms printed 7 FAILED lines, one per case (5 × "could not parse", 1
    mismatch, 1 error), and PASS. The errUnparsable test PASS
- No check lost: I diffed the sorted `go test -v | grep -- '--- PASS'` output against a
    `git archive HEAD` copy in `/tmp`. Old 566, new 567; the only added line is the new test, so
    every per-vector subtest still runs
- Mutation probes in a throwaway copy, now deleted. (a) Putting `TrimPrefix` back in `decodeStream`
    made the new test fail: `bare_hex: got error <nil>, want errUnparsable`. (b) Mapping a nil
    `iscc` to `""` made it fail: `null_iscc: …`. A missing key (`{}`) is caught by the Unmarshal
    error on its own
- `mise run check` → all hooks Passed

**Next:** the Rust half of the same `[audit]` issue: the per-function tests in the `lib.rs`,
`codec.rs` and `streaming.rs` test modules should read through `conformance.rs`'s parsers. The
policy is already the same on both sides.

**Notes:**

- The per-function tests are slightly stricter than before. A missing `iscc` output is now
    `t.Fatalf`; the meta test used to skip the iscc check when the key was absent. The real fixture
    passes either way
- `expectedISCC` reads the outputs as `map[string]json.RawMessage` instead of a struct, because Go
    matches struct tags case-insensitively. That way an `"ISCC"` key is not accepted where Rust
    would reject it
- `parseConformanceData` now fails if any non-`_metadata` section is not an object of cases. Today
    every section is one. A future non-object top-level key would make `ConformanceSelftest` return
    an error rather than skipping that key
- Memory: MEMORY.md is at 141 lines, one over its soft budget of 140
