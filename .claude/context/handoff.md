## 2026-11-22 — Review of: Go parses `data.json` once and adopts the Rust runner's malformed-vector policy

**Verdict:** PASS

**Summary:** `packages/go/conformance.go` now parses every section of the embedded fixture once,
into one `vectorEntry` type. Each gen runner is an inputs parser plus the call, and the per-function
tests read their inputs through those parsers. A bare-hex stream and a missing or null `iscc` output
are parse failures, as in the Rust runner. The diff touches one non-test file and the package's
`CLAUDE.md`; no check was lost.

**Verification:**

- [x] `CGO_ENABLED=0 go test ./... && go vet ./...` exits 0, and `GOARCH=386 go test ./...` exits 0
    — all three ok; `GOARCH=386 go vet` and `gofmt -l` are clean too
- [x] `grep -n 'tests/data\.json"' packages/go/*.go` prints nothing
- [x] `grep -n 'parseSeqConformanceData\|seqVector\|TrimPrefix(.*"stream:"' packages/go/*.go` prints
    nothing, and `grep -c 'func metaArg' packages/go/conformance.go` prints `1`
- [x] `go test -run 'TestGenSectionFailureArms|TestGenSectionPositiveCase' -v ./...` — the positive
    case passes; the failure arms print seven `FAILED` lines (five parse failures, among them
    `bare_hex` and `no_iscc`, one mismatch, one error);
    `TestGenRunnerRejectsMalformedVectorsAsUnparsable` passes
- [x] `mise run check` exits 0
- [x] Probe, "keep every existing check": the sorted `--- PASS` list of `go test -v` against a
    `git archive` copy of the define-next commit is 566 → 567 lines, and the only difference is the
    new test
- [x] Probe, the same malformed vectors through both runners: see **Issues found**

**Issues found:**

- The two runners agree on the two cases this step named, but not on six other malformed shapes. I
    ran one table through both (Rust through a throwaway test in a `/tmp` copy): a short inputs list
    makes the Rust runner panic, although `conformance_selftest` documents "Does not panic". A float
    `bits`, a `null` description and a fractional or negative pixel pass in Go and fail in Rust. A
    number or array as `meta` and a non-string entry in a code list pass in Rust and fail in Go.
    None of this is reachable with the real fixture. The audit entry in issues.md is rewritten to
    this remaining problem (table included) and stays `normal`
- Note, not filed separately: `parseConformanceData` now returns an error for a top-level key that
    is not an object of cases (next.md asked for it); the Rust runner ignores unknown keys. It is
    one line in the same issue entry

**Codex review:** no actionable findings. Codex ran the Go tests and vet on amd64 and 386, a
race-enabled shuffled run and the vendored-fixture check, all green.

**Next:** the Rust half of the rewritten `[audit]` issue "Rust tests re-parse `data.json`, and the
two runners still disagree on malformed vectors". Two parts, which can be one step or two:

1. The test modules of `crates/iscc-lib/src/lib.rs`, `codec.rs` and `streaming.rs` read inputs
    through `conformance.rs`'s case parsers instead of decoding the fixture by hand (test-only
    refactor; `conformance.rs` is the one non-test file).
2. One rule for a value of the wrong JSON type or range, applied to both runners and pinned per row
    of the table in the issue. "Parse failure, never a panic, a skip or a truncation" is the
    simplest rule. It touches `conformance.rs` (indexing, `string_entries`, the `meta` fallback)
    and `conformance.go` (`bits`, description, pixel and `cv` conversions, `metaArg`).

Part 1 is easier to review if the parsers are split out first, as the Go step did. Part 2 then
changes only those parsers. `alg_minhash_256([])` and WASM `finalize()` still wait on Titusz.

**Notes:**

- Rust facts for part 2, measured: `inputs[1]` on a `Vec<Value>` panics on a short list (use
    `.get()`); `as_u64()` rejects `64.0`; `string_entries` drops non-string entries with
    `filter_map`; a `meta` that is neither null, string nor object becomes `None`
- Go facts for part 2, measured: `json.Unmarshal` of `null` into a string is a no-op;
    `uint32(float64)` and `byte(float64)` truncate, and their result for a negative or out-of-range
    value is implementation-defined
- The Rust runner is a benchmark-free path, but `conformance.rs` and the test modules are scored by
    the CRAP gate: a new branch in a covered function needs the baseline moved in the same step
    (learnings → "CRAP gate")
- learnings.md is at its 200-line budget: one bullet went to `learnings-archive.md` to make room
