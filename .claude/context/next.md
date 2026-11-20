# Next Work Package

## Step: Strict data-edge compares in the Swift and Python harnesses

## Goal

Close the `normal` `[human]` issue "`data-edge.json` harnesses accept a value that only resembles
the expected one": the Swift harness must compare strings by Unicode scalars and never let a JSON
Boolean pass as a number (or the reverse), and the Python harness must never let `0`/`1` equal
`False`/`True`, so the first non-ASCII or Boolean vector gets the full check without a harness edit.

## Alternatives Considered

- **Chosen:** strict compares in Swift + Python. Two test files, already promoted by Titusz, and the
    handoff's "Next".
- **Rejected:** the derived-facts parity check (the other `normal` issue). It needs a new pre-commit
    script plus a sweep over specs and `CLAUDE.md` files, so it is larger and goes next.

## Scope

- **Modify**: `packages/swift/Tests/IsccLibTests/DataEdgeTests.swift`, `tests/test_data_edge.py`
- **Reference**: `.claude/context/specs/edge-vectors.md` → "Rules"; `packages/swift/CLAUDE.md`
    (Linux Swift toolchain recipe); `scripts/gen_data_edge_vectors.py:215` (the C/C++ generator
    already fails closed on a bool where an integer is expected, so it needs no change)

## Not In Scope

- Any `data-edge.json` edit. No current section has a non-ASCII or Boolean output, and a fixture
    edit pulls in 4 vendored copies plus the generated C header.
- The other 9 surfaces' compares. They were probed and are strict already: Go `reflect.DeepEqual`,
    JS/.NET compare JSON text, Gson `JsonPrimitive.equals`, the Kotlin `isNumber` and `Long` check,
    serde `Value ==`, and Ruby `==`.
- The partial-key compare on .NET, Java, napi and WASM ("only the expected keys the result
    exposes"). It is a separate question, not part of this issue.
- `edge-vectors.md` or decisions.md edits.

## Implementation Notes

**Python** (`matches`, used by both `test_iscc_lib_outcome` and the iscc-core cross-check):

- Replace the bare `==` and the dict `value.get(k) == v` with a recursive equality helper.
- Rule: a `bool` never equals a non-`bool`, recursively through lists and dict values. Otherwise use
    `==`.
- Do not demand exact type identity. The reference side may return an `int` subclass, and that must
    still match.
- Dict fields keep the rule that every expected key must be present.
- The converters (lines 74–84) need no change.

**Swift** (`compareValue` plus the scalar input reads `bits` and the `wide` read in
`prepareIsccCode`):

- The constraint: the JSON kind of a value (string, number, Boolean) must be decided without relying
    on `as? Int` / `as? Bool`. `JSONSerialization` returns `NSNumber` for both kinds, and on Darwin
    `as? Int` accepts `false` while `as? Bool` accepts `0`/`1`. The real `iscc_decode` outputs hold
    `0`s, so a cast-order fix ("try `Bool` first") could pass on Linux and go red only in CI's
    `macos-15` job.
- Candidate mechanisms, to be probed rather than taken on trust:
    - Detect a Boolean `NSNumber` in a way that behaves the same on corelibs and Darwin Foundation.
    - Re-serialise the scalar with `JSONSerialization` (`.fragmentsAllowed`) and branch on the text
        (`true`/`false` versus digits).
    - Decode the expected values through `JSONDecoder` into a small `enum`.
- Pick one, use it for both the expected-value compare and the `bits`/`wide` input reads, so one
    surface has one way of reading a scalar, and document it in a doc comment.
- Strings compare by `unicodeScalars` (`Array(a.unicodeScalars) == Array(b.unicodeScalars)`), never
    by `String ==`.
- Add a Boolean branch to `compareValue`: an expected Boolean equals only an actual Swift `Bool` of
    the same value. Then a future Boolean output needs no harness edit.

**Swift negative tests:**

- Build each synthetic doc from a JSON string through `JSONSerialization`, never from Swift
    literals. A literal `false` in `[String: Any]` is a native `Bool`, not the `NSNumber` that real
    loads produce, so a literal-built test can pass against today's loose code.
- One bad case per doc:
    1. `gen_text_code_v0` with `["Hello\u0000World", 64]` and an expected `iscc` of
        `"ISCC:EAAS\u212aDNZNYGUUF5A"` as JSON text (U+212A KELVIN SIGN for `K`) must fail.
    2. `iscc_decode` on `ISCC:GAAAAAAAAA` (actual `[3,0,0,0,"bytes:00000000"]`, see the existing
        `testDataEdgeHonoursExcept`) with expected `[3,false,0,0,"bytes:00000000"]` must fail.
    3. A `gen_text_code_v0` vector whose `bits` input is `true` must fail with a `harness:` reason.
- Plus one positive unit test on `compareValue`: an expected JSON `true` (from parsed text) equals a
    Swift `true` and does not equal `1`.
- The Python tests go directly on `matches`, one bad case each:
    - `[3, False, 0, 0, "x"]` vs expected `[3, 0, 0, 0, "x"]` → False
    - expected `True` vs `1` → False
    - dict field `{"characters": 0}` vs `{"characters": False}` → False
    - a matching nested list → True

**Gates:** `tests/`-only and Swift-test-only, so CRAP and iai are untouched. Local Swift runs on
Linux only. The macOS proof comes from the committed tests in CI's `swift` job after the push, so
the mechanism must not depend on corelibs-only behaviour.

## Verification

- `uv run pytest tests/test_data_edge.py` passes, including the new `matches` cases.
- Each new Python negative case fails when `matches` is temporarily reverted to bare `==`; restore
    afterwards.
- `swift test` in `packages/swift` (Linux recipe in `packages/swift/CLAUDE.md`, after
    `cargo build -p iscc-uniffi`) passes, including the 3 new negative tests and the positive Bool
    test, with 0 failures.
- `grep -n 'as? Int\b\|as? Bool\|(actual as? String) ==' packages/swift/Tests/IsccLibTests/DataEdgeTests.swift`
    prints nothing.
- `mise run check` exits 0.

## Done When

The Swift and Python data-edge harnesses reject a look-alike string, a Boolean-for-number and a
number-for-Boolean in committed tests built from parsed JSON, and all verification checks pass.
