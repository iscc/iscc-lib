## 2026-11-19 — data-edge.json fan-out, group 2 — Swift and Kotlin harnesses, plus the Ruby key-compare fix

**Done:** Swift (`DataEdgeTests.swift`) and Kotlin (`DataEdgeTest.kt`) now run every
`data-edge.json` vector through registered vendored copies. Each uses a section → caller table, a
`checkDoc` that returns failures, the surface's `except` id, the raw-call-only rejection (Swift
`IsccUniError`, Kotlin `IsccUniException.IsccException`; any other error type is a failure), and no
hardcoded vector count. Ruby's compare now fails on any missing expected key. All three surfaces
gained the sixth test, `…fails_missing_output_key`. One commit per surface: Ruby `cc53186`, Swift
`616f5ae`, Kotlin `a75832b`.

**Files changed:**

- `crates/iscc-rb/test/test_data_edge.rb`: the `actual.keys & expected.keys` intersection is
    replaced by a check over every expected key (`actual.key?`), giving "missing field <k>". Adds
    `test_data_edge_fails_missing_output_key`
- `packages/swift/Tests/IsccLibTests/DataEdgeTests.swift` (new): 6 tests. Typed-cast comparison
    (`String`/`Int`/`[Any]` element-wise, fail-closed on any other type). Record fields are boxed as
    `Int(...)`. No `as!`/`fatalError` in the harness path
- `packages/swift/Tests/IsccLibTests/data-edge.json` (new, byte copy);
    `packages/swift/Package.swift`: `.copy("data-edge.json")`
- `packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/DataEdgeTest.kt` (new): 6 tests. Strict input
    readers. Comparison is against plain Kotlin values (`Long`, `String`, `List`, `Map`); numbers
    are compared by their JSON text, so Gson's double coercion plays no part
- `packages/kotlin/src/test/resources/data-edge.json` (new, byte copy)
- `tests/test_vendored_fixtures.py`: both copies registered under the `data-edge.json` key
- `packages/swift/CLAUDE.md`, `packages/kotlin/CLAUDE.md`: file trees plus an "Edge-case vectors"
    subsection

**Verification:**

- Ruby: the sixth test went red against the old intersection compare (`Expected: 1 Actual: 0`), then
    green after the fix. `rake compile && rake test`: 160 runs, 0 failures. `TestDataEdge` ran 6
    named tests (`-v`). Bare `bundle exec standardrb` in `crates/iscc-rb`: exit 0
- Swift (`/tmp/swifttc` 6.1.2, `--scratch-path /tmp/swiftbuild`, after
    `cargo build -p iscc-uniffi`): exit 0. `DataEdgeTests` executed 6 tests, all 6 listed as passed
    by name. 25 total, 0 failures. No compiler warnings from `DataEdgeTests.swift`. **Linux only**:
    the macOS CI `swift` job has not run this code yet
- Kotlin: `./gradlew cleanTest test` exit 0. `TEST-uniffi.iscc_uniffi.DataEdgeTest.xml` shows
    tests=6, skipped=0, failures=0, errors=0, with 6 `testcase` elements
- Mutation probe on both surfaces: the vendored `"characters": 10` was changed to 11. Swift and
    Kotlin each went red on `field characters: expected 11, got 10`. Both copies were restored with
    `cp` (`cmp` identical); then the full Swift suite (exit 0, 6 `DataEdgeTests` passed, 25 total)
    and the Kotlin suite (XML above) were re-run green on the restored copies
- `uv run pytest tests/test_vendored_fixtures.py tests/test_data_edge.py`: 86 passed.
    `mise run check`: exit 0

**Next:** Fan-out group 3: C and C++ through a generated table plus a staleness check. Recover
`scripts/gen_ffi_boundary_vectors.py` from `ea69169`. Cheap `normal` fallback: the Python
`encode_seq` docstring issue.

**Notes:**

- No vector needed an `except` on Swift or Kotlin. UniFFI keeps the embedded NUL, so the core
    returns the expected `gen_text` output and rejects the seq NUL unit
- The missing-key negative uses `gen_text_code_v0` with outputs `{iscc: <correct>, datahash: "x"}`
    and no `characters`, so it is one bad case. The assertion checks for "missing field datahash"
- Swift, Kotlin and Ruby check every key now. Go still compares `gotFields[key]` without an explicit
    presence check; that only works because a missing key reads as nil. It is out of scope and
    unchanged
- The Swift copy ships with the SwiftPM checkout. That is the open `low` issue; no exclusion
    mechanism was added, as next.md specified
