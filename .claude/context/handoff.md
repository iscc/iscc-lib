## 2026-11-20 — Strict data-edge compares in the Swift and Python harnesses

**Done:** The Swift harness now reads every Boolean or integer scalar through one classifier,
`DataEdge.scalar`. It re-serialises the value inside an array with `JSONSerialization` and branches
on the JSON text (`true`/`false` or digits). Strings compare by `unicodeScalars`, and a Boolean
never equals a number, in either direction. The `bits` and `wide` input reads go through the same
classifier (`intArg`/`boolArg`). In Python, `matches` uses a recursive `same` helper in which a
`bool` never equals a non-`bool`; dict fields still require every expected key to be present.

**Files changed:**

- `packages/swift/Tests/IsccLibTests/DataEdgeTests.swift`: adds the `Scalar` enum, `scalar`,
    `intArg`, `boolArg` and `describe`; `bits` and `prepareIsccCode` read through them;
    `compareValue` compares by Unicode scalars and by `scalar`; adds a `parsedDoc` helper that
    builds synthetic docs from JSON text. Adds 5 tests: look-alike string (U+212A), `false` for `0`
    in `iscc_decode`, `bits: true`, `wide: 1` (the advisor's suggestion, so the `wide` read is
    covered), and a positive `compareValue` Bool test, which also checks that parsed `1` ≠ Swift
    `true`
- `tests/test_data_edge.py`: adds `same`; `matches` uses it, with `k in value` in place of `.get`.
    Adds 3 parametrized negatives (`false_for_zero`, `one_for_true`, `zero_for_false`) and one
    positive nested-list test

**Verification:**

- Probe on corelibs (Swift 6.1.2, Linux): a parsed `false` gives a conditional cast to `Int` of
    `Optional(0)`; a parsed `0` gives a cast to `Bool` of `Optional(false)`. Re-serialisation gives
    `[false]` vs `[0]` for parsed values and for native Swift values alike. An invalid object (a
    struct) fails `isValidJSONObject` and returns nil, with no crash
- Tests-first: the 5 new Swift tests ran against the **old** harness and all 5 failed (8 assertion
    failures; for example `bits: true` read as 1 and `wide: 1` read as `true` and produced an ISCC).
    After the change: `swift test` (Linux recipe) ran 30 tests with 0 failures (19 conformance and
    11 data-edge)
- `grep -n 'as? Int\b\|as? Bool\|(actual as? String) ==' …/DataEdgeTests.swift` prints nothing (exit
    1\)
- `uv run pytest tests/test_data_edge.py`: 88 passed. With `matches` reverted by hand to bare `==` /
    `.get`, all 3 new negatives failed and the positive passed; the file was restored and gives 88
    passed again
- `mise run check` exits 0; `prek run --hook-stage pre-push --files <both files>` exits 0
- **Not verified:** Darwin Foundation. The mechanism relies only on documented `JSONSerialization`
    writing (CFBoolean → `true`/`false`) and on the `isValidJSONObject` guard, but the macOS proof
    comes from CI's `swift` job (`macos-15`) after the push

**Next:** The derived-facts parity check, the other open `normal` issue.

**Notes:**

- `packages/swift/CLAUDE.md` line "typed casts, no `Any` equality" (Edge-case vectors section) is
    now slightly stale. The compare now goes through `scalar` and Unicode scalars. It is out of
    scope here and needs a one-line doc touch-up later
- `arg<T>` is now used only for `String` and `[String]`. Those casts are strict: an `NSNumber` never
    casts to `String`
- An integral JSON float (`64.0`) may re-serialise as `64` and read as an integer on some
    Foundations. No vector uses floats; Python would reject it differently. Edge case, not addressed
- No core, binding or bench code changed, so CRAP, iai and semver were not run
