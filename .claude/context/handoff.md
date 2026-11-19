## 2026-11-19 — Review of: data-edge.json fan-out, group 1 finish — WASM, Java, .NET, Ruby harnesses

**Verdict:** PASS_WITH_NOTES

**Summary:** All six group-1 harnesses now run every `data-edge.json` vector: Go and Node (carried
over, `a84b665`/`be0c4d3`) and WASM, Java, .NET and Ruby (one commit per surface). Each has a
section → caller table, asserts the rejection on the raw call only with the surface's error type,
honours its `except` id, fails on an unknown section, and hardcodes no vector count. One weakness:
the Ruby harness compares only the keys a result happens to have. It is filed `normal`, does not
fail any criterion, and did not hold anything back.

**Verification:**

- [x] `uv run pytest tests/test_vendored_fixtures.py tests/test_data_edge.py`: 84 passed.
    `mise run check` exit 0
- [x] Go: `TestDataEdge*` 5 PASS by name; full `CGO_ENABLED=0 go test -count=1 ./...` ok; `go vet`
    clean; the `grep -l data-edge` over non-test `.go` files prints nothing
- [x] Node: `npm run build:debug && npm test` exit 0. `ok 12 - data-edge.json` with 5 named
    subtests; 179 pass, 0 fail
- [x] WASM: `wasm-pack test --node crates/iscc-wasm --features conformance`: `tests/data_edge.rs` 5
    passed by name (conformance 11, unit 94)
- [x] Java: `cargo build -p iscc-jni && mvn clean test`: surefire `DataEdgeTest` 5 run, 0 failures
    (5 named testcases); `IsccLibTest` 103
- [x] .NET: `dotnet test … -e LD_LIBRARY_PATH=$PWD/target/debug`: 5 `DataEdgeTests.*` Passed; 130
    total, 0 failed
- [x] Ruby: `rake compile && rake test`: `TestDataEdge` 5 tests by name; 159 runs, 0 failures.
    `standardrb` exit 0
- [x] Extra checks (3 of 3): `cargo clippy --workspace --all-targets -D warnings` clean and
    `cargo test -p iscc-lib` green (`test_data_edge` 7 passed after the `a84b665` note reword); the
    CRAP gate cannot see `crates/iscc-wasm/**` (`.cargo-crap.toml` excludes it); and
    published-artifact exclusion (see the second issue below)
- [x] Gate integrity: the unpushed diff adds no suppressions, skips, exclusions or hook changes

**Issues found:**

- Filed `normal`: **Ruby `data-edge.json` harness passes a gen result that lacks an expected key.**
    `actual.keys & expected.keys`, where the Rust/Python templates and Go fail on a missing key.
    Introduced this step
- Filed `low`, untriaged: **Go and Swift ship `data-edge.json` despite "absent from every published
    artifact".** The Go module zip includes `packages/go/testdata/`, and SwiftPM clones the whole
    repo. The spec also demands a copy next to every `data.json` copy. Which line wins is Titusz's
    call, not a defect of this step
- Nit (handoff only): the Node harness uses `SECTIONS[section]`, which resolves inherited names such
    as `constructor`. Only a fixture section named after an `Object.prototype` member could exploit
    it; `Object.hasOwn` would close it
- Process note: the "keys the result exposes" wording first appeared in a `next.md` hunk committed
    inside advance's Go commit `a84b665`, replacing "compare each listed key". That wording is where
    the Ruby weakness came from

**Codex review:** one run over `14adaa9..HEAD`, which covers all six harnesses. It found two things,
both test-only: the Ruby key-intersection compare (P2, valid, filed) and the Node inherited section
lookup (P3, valid but contrived, nit above).

**Next:** Fan-out group 2: Swift and Kotlin (UniFFI) harnesses, each with a vendored copy registered
in `VENDORED_COPIES` in the same step. Use this compare rule: a surface that returns a full result
fails on any missing expected key, and only a bare-string or one-field result is wrapped to
`{iscc}`. Do not copy Ruby's intersection. The Ruby fix (a few lines plus one negative test) can
ride along as its own per-surface commit. Then group 3 (C, C++ through a generated table plus a
staleness check; recover the generator from `ea69169`). Cheap `normal` fallback: the Python
`encode_seq` docstring issue.

**Notes:**

- Swift group 2: do not treat `edge-vectors.md` → "absent from every published artifact" as settled
    for source-distributed surfaces. The filed issue waits on Titusz. Vendor the copy as the spec's
    "The File" section says, and do not invent an exclusion mechanism
- .NET meets the gen_text NUL vector through `except.dotnet = error` (the wrapper raises
    `IsccException`). `GenMixedCodeV0([])` and `GenIsccCodeV0([])` raise `IsccException` with the
    FFI's NULL message, so they pass; the pin-loop issue stays open
- Ruby's error type is `RuntimeError` (`to_magnus_err` → `exception_runtime_error`). A
    `TypeError`/`ArgumentError` from the raw call is reported as a failure, never as a pass
