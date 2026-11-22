# Next Work Package

## Step: Go parses `data.json` once and adopts the Rust runner's malformed-vector policy

## Goal

This is the Go half of the `normal` `[audit]` issue "Rust and Go per-function tests re-parse
`data.json` beside the conformance runners". `conformance.go` parses the fixture once with one
vector type. The `code_*_test.go` files read inputs through the runner's parsers over the embedded
copy. The Go runner then gives the same verdict as the Rust runner on a bare-hex stream and on a
missing `iscc` output.

## Alternatives Considered

- **Chosen:** the Go half. Rust already rejects both malformed cases and pins them
    (`conformance.rs`, `test_gen_section_failure_arms_fail`: `["00ff", 64]` and `outputs: {}`). Go
    accepts bare hex (`TrimPrefix`), so the two selftests give different verdicts on one fixture.
    That is the only behavioural part of the issue, and it touches one Go non-test file
- **Rejected:** the Rust half, which the handoff suggested. Its policy is already settled and
    pinned, so it is a pure test refactor (`lib.rs`, `codec.rs`, `streaming.rs` test modules). It is
    the next step, not this one. `alg_minhash_256([])` and WASM `finalize()` wait on Titusz

## Scope

- **Modify**: `packages/go/conformance.go` (the only non-test file)
- **Modify (tests)**: `packages/go/conformance_test.go`, `seq_test.go`, `codec_test.go`,
    `code_meta_test.go`, `code_data_test.go`, `code_instance_test.go`, `code_iscc_test.go`,
    `code_content_{text,image,audio,video,mixed}_test.go`
- **Modify (docs)**: `packages/go/CLAUDE.md` → "Conformance tests" and the "Conformance vector path"
    pitfall (both still say the per-function tests read the monorepo path)
- **Reference**: `crates/iscc-lib/src/conformance.rs` (policy and its pinned failure arms)

## Not In Scope

- Any Rust file: the Rust half is the next step
- `data_edge_test.go` and `testdata/data-edge.json`. That harness reads its own file on purpose
    (CLAUDE.md: "never embedded")
- `ConformanceSelftest`'s signature, and any change to its verdict on the real fixture
- Editing issues.md: review rewrites the entry to the remaining Rust half

## Implementation Notes

- **One parse.** Use `vectorEntry{Inputs []json.RawMessage; Outputs json.RawMessage}` for every
    section, and drop `seqVector`, `seqSections` and `parseSeqConformanceData`.
    `parseConformanceData` returns all sections except `_metadata`. Today it skips non-`gen_` keys
    because the seq outputs are not objects; with raw `Outputs` that reason is gone. Keep "a missing
    section is reported by its runner" (`TestSeqSectionsMissingFail`, `TestGenSectionMissingFails`).
    `ConformanceSelftest` calls the parser once
- **Policy (match Rust):** `decodeStream` uses `strings.CutPrefix(s, "stream:")` and returns an
    error without the prefix, so the caller maps it to `errUnparsable`, as `decodeBytesValue` does
    for `bytes:`. `"stream:"` with empty hex stays valid (empty input). `expectedISCC` becomes
    `(string, error)`: it unmarshals `Outputs` into `struct{ Iscc *string }` and returns
    `errUnparsable` when that is nil or the outputs do not decode. A gen runner calls it **before**
    the gen function, as Rust's `expected_iscc(tc)?` runs before the call
- **Shared input parsers.** Split each `run*Case` into an inputs parser (for example `streamInputs`
    already exists, plus `metaInputs`, `imageInputs` and so on, returning typed args or
    `errUnparsable`) and the call. Each per-function test then: loads the embedded fixture once (one
    test helper over `parseConformanceData([]byte(conformanceData))`), iterates its section, gets
    inputs from the shared parser, calls the gen function, and checks only the extra output fields
    (`metahash`, `name`, `description`, `meta`, `datahash`, `filesize`, `parts`, …). It reads them
    by unmarshalling `vec.Outputs` into a per-test struct. Delete the inline meta dispatch that
    copies `metaArg` in `code_meta_test.go` (around lines 49-63) and the `TrimPrefix` stream
    decoding in the data and instance tests
- No `os.ReadFile` of `../../crates/iscc-lib/tests/data.json` remains, and no `t.Skipf` for a
    missing fixture: the embedded copy always exists. `codec_test.go` has two such sites
    (`TestCodecDecomposeConformanceVectors` and the one near line 1175);
    `code_content_image_test.go` has two
- **Pin the policy**, one bad case per call (learnings). In `TestGenSectionFailureArms`, add
    `"bare_hex": {["00ff", 64], {"iscc": "ISCC:X"}}` and `"no_iscc": {["stream:", 64], {}}`. Also
    add a test that calls `runDataCase` on each of the two and asserts
    `errors.Is(err, errUnparsable)`, so it proves the parse arm and not a mismatch.
    `TestGenSectionPositiveCase` must still pass
- Keep every existing per-function check. Do not lower any count or drop any field comparison

## Verification

- `cd packages/go && CGO_ENABLED=0 go test ./... && go vet ./...` exits 0, and
    `GOARCH=386 go test ./...` exits 0
- `grep -n 'tests/data\.json"' packages/go/*.go` prints nothing (12 sites today; no test reads the
    monorepo fixture)
- `grep -n 'parseSeqConformanceData\|seqVector\|TrimPrefix(.*"stream:"' packages/go/*.go` prints
    nothing, and `grep -c 'func metaArg' packages/go/conformance.go` prints `1`
- `go test -run 'TestGenSectionFailureArms|TestGenSectionPositiveCase' -v ./...` in `packages/go`
    shows the `bare_hex` and `no_iscc` cases failing the runner, and the `errUnparsable` test passes
- `mise run check` exits 0

## Done When

All verification criteria pass. The Go package then parses `data.json` once, its tests read inputs
only through the runner's parsers, and the Go runner rejects bare hex and a missing `iscc` the way
the Rust runner does.
