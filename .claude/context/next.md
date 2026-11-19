# Next Work Package

## Step: data-edge.json fan-out, group 2 — Swift and Kotlin harnesses, plus the Ruby key-compare fix

## Goal

Fan-out group 2 of the v0.8.0 "Shared edge-case vectors run everywhere" criterion: Swift and Kotlin
(UniFFI) get a `data-edge.json` harness and a registered vendored copy. The step also fixes the
`normal` issue "Ruby `data-edge.json` harness passes a gen result that lacks an expected key",
because it is the same compare rule on a sibling harness.

## Alternatives Considered

- **Chosen:** Swift + Kotlin + the Ruby fix. The behaviour is fixed by `edge-vectors.md` → Rules,
    both toolchains are available here (cached Swift at `/tmp/swifttc`, cached Gradle 9.6.1), and
    the change is test-only
- **Rejected:** group 3 (C, C++). It needs the generator recovered from `ea69169` and a staleness
    check, which is a different kind of work. It comes next

## Scope

**Fan-out:** `edge-vectors.md` → Rules (every vector runs, `except` is the only deviation, an
unknown section fails, the rejection is asserted on the raw call only, every expected output key is
compared), applied to Swift and Kotlin, with Ruby's compare brought to the same rule. Commit once
per surface (Swift, Kotlin, Ruby).

- **Create**: `packages/swift/Tests/IsccLibTests/data-edge.json` and
    `packages/kotlin/src/test/resources/data-edge.json` (byte copies of the canonical file),
    `packages/swift/Tests/IsccLibTests/DataEdgeTests.swift`,
    `packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/DataEdgeTest.kt`
- **Modify**: `packages/swift/Package.swift` (test target `resources` gets
    `.copy("data-edge.json")`), `tests/test_vendored_fixtures.py` (both copies under the
    `data-edge.json` key of the registry), `crates/iscc-rb/test/test_data_edge.rb`, the test-file
    lists in `packages/swift/CLAUDE.md` and `packages/kotlin/CLAUDE.md`
- **Reference**: `crates/iscc-lib/tests/test_data_edge.rs`, `tests/test_data_edge.py`,
    `packages/go/data_edge_test.go`, `crates/iscc-rb/test/test_data_edge.rb`,
    `.claude/context/specs/edge-vectors.md`, and the existing `ConformanceTests.swift` /
    `ConformanceTest.kt` for JSON loading, `bytes:<hex>` decoding and the error type

## Not In Scope

- Group 3 (C, C++) and any vendored copy beyond the two above
- Any exclusion mechanism keeping the Swift copy out of the SwiftPM checkout. The "absent from every
    published artifact" question is a `low` issue waiting on Titusz; vendor the copy as "The File"
    says
- Deleting per-binding tests that overlap a vector (`ConformanceTests.swift` seq error tests,
    `ConformanceTest.kt` `assertIsccError` cases)
- Any binding, UniFFI, Go-port or core behaviour change; any edit to `data-edge.json`; the Node
    `Object.hasOwn` nit

## Implementation Notes

- **Test set per surface:** mirror Ruby's five tests (vectors pass, unknown section fails, expected
    error that succeeds fails, output mismatch fails, `except` honoured) and add a sixth: a gen
    result missing an expected key fails. Ruby gets that sixth test too, next to
    `test_data_edge_fails_output_mismatch`. One bad case per negative test (learnings: a doc mixing
    two bad cases stays green when one arm breaks).
- **Harness shape:** a section-name → small-caller table for the five sections, a `checkDoc(doc)`
    returning failures, no hardcoded vector count. `ConformanceTests.swift` hardcodes counts
    (`XCTAssertEqual(vectors.count, 20)`); do not copy that.
- **Expectation:** `except["swift"]` / `except["kotlin"]` if present, else `error`/`outputs`. `skip`
    means no call. On an expected rejection, the raw call alone must throw the surface's error type:
    Swift `IsccUniError` (internal, reachable via `@testable import IsccLib`), Kotlin
    `IsccUniException.IsccException`. Any other error type is a failure, never a pass. Never match
    the message text.
- **Compare rule:** UniFFI returns full records, so each gen caller maps its record to a dictionary
    of all its fields (`TextCodeResult` → `iscc`, `characters`; `MixedCodeResult` → `iscc`, `parts`;
    `IsccCodeResult` → `iscc`). Every expected key must be present in that dictionary and equal; a
    missing key is a failure. `iscc_decode` → `[maintype, subtype, version, length, "bytes:<hex>"]`;
    `encode_seq` → `"bytes:<hex>"`. Ruby: replace `actual.keys & expected.keys` with a check over
    every expected key, failing on one the Hash lacks.
- **Swift portability:** CI runs Swift on macOS, locally it runs on Linux. Stay with the JSON
    casting idioms `ConformanceTests.swift` already uses (`JSONSerialization`, `as! [String: Any]`,
    `as! Int`); numbers compared across Foundations behave differently with `as? NSNumber` tricks.
- **Stop rule (from 217):** if a vector fails on a surface for a real behavioural reason, add no
    `except` and fix no binding. Leave that surface's harness out, finish the others, and name the
    vector in the handoff.
- Nothing touches `crates/iscc-lib/src` or `crates/iscc-lib/tests/*.rs`, so CRAP and iai are
    unaffected.

## Verification

Each surface run must show its data-edge tests ran; exit 0 alone does not count.

- `uv run pytest tests/test_vendored_fixtures.py tests/test_data_edge.py` passes, and
    `mise run check` exits 0
- Swift: with the cached toolchain from the `packages/swift/CLAUDE.md` recipe on `PATH`,
    `cargo build -p iscc-uniffi` then `swift test` (recipe flags, `--scratch-path /tmp/swiftbuild`)
    in `packages/swift` passes and lists each `DataEdgeTests` test as passed
- Kotlin: `cargo build -p iscc-uniffi` then `./gradlew cleanTest test` in `packages/kotlin` exits 0,
    and `build/test-results/test/TEST-uniffi.iscc_uniffi.DataEdgeTest.xml` exists with
    `failures="0" errors="0" skipped="0"` and at least six `testcase` elements
- Ruby: in `crates/iscc-rb`, `bundle exec rake compile && bundle exec rake test` passes with
    `TestDataEdge` running at least six tests, and `bundle exec standardrb` is clean

## Done When

Swift and Kotlin run every `data-edge.json` vector through registered vendored copies, all three
harnesses fail on a missing expected key, and every check above passes (a surface held back under
the stop rule is the one sanctioned exception, reported in the handoff).
