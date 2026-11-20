## 2026-11-20 — Review of: Strict data-edge compares in the Swift and Python harnesses

**Verdict:** PASS

**Summary:** The Swift harness now reads every Boolean or integer through one classifier
(`DataEdge.scalar`, which re-serialises the value and branches on the JSON text) and compares
strings by Unicode scalars. The Python `matches` uses a recursive `same` helper in which a `bool`
never equals a non-`bool`. The diff touches the two test files next.md named and nothing else. The
`normal` `[human]` issue "`data-edge.json` harnesses accept a value that only resembles the expected
one" is deleted.

**Verification:**

- [x] `uv run pytest tests/test_data_edge.py` passes, including the new `matches` cases — 88 passed
- [x] Each new Python negative fails with `matches` reverted to bare `==` — run on a throwaway copy:
    3 failed (`false_for_zero`, `one_for_true`, `zero_for_false`), 85 passed
- [x] `swift test` in `packages/swift` (Linux recipe, Swift 6.1.2) passes with 0 failures — 30 tests
    (19 conformance, 11 data-edge), including the 4 negatives and the positive Bool test
- [x] `grep -n 'as? Int\b\|as? Bool\|(actual as? String) ==' …/DataEdgeTests.swift` prints nothing
- [x] `mise run check` exits 0
- Extra probe: the 5 new Swift tests spliced onto the previous harness (throwaway copy) fail, all 5
    of them, with 8 assertion failures. The handoff's tests-first claim holds
- **macOS is not proven locally.** The proof is the `swift` job (`macos-15`) of the CI run on this
    push. The reviewer watches that job after pushing; if it is red, a follow-up review commit
    restores the issue. update-state: confirm the job result at the source

**Issues found:**

- (none filed)
- Nit, handoff only: an integral JSON float (`64.0`) may re-serialise as `64` and read as an integer
    in the Swift harness on some Foundations. No vector uses floats, and the vector validator is the
    place to reject one if it ever matters
- Nit: in Python `same`, a nested dict needs exactly the expected keys while the top level allows
    extra result keys. That matches the behaviour before this step

**Codex review:** No actionable findings. Codex ran both suites itself (88 Python, 30 Swift on
Linux) and also could not test macOS.

**Next:** The derived-facts parity check, the one open `normal` issue ("Derived facts in guidance
and docs are written by hand and drift"). It has two parts: a pre-commit check that compares the
Tier 1 symbol list in every doc or README that enumerates it with the public API at the `iscc-lib`
crate root, and a sweep that removes unchecked counts, file lists and line numbers from the specs
and the per-crate `CLAUDE.md` files. They can be separate steps; the check comes first, because it
decides which lists stay.

**Notes:**

- Reviewer fix (step 9): `packages/swift/CLAUDE.md` → "Edge-case vectors" said "typed casts, no
    `Any` equality". It now describes the scalar and Unicode-scalar compare. Doc-only
- No spec edit: the issue's `**Spec:**` field named the rule the harnesses broke (`edge-vectors.md`
    → "A vector costs one edit"), and the rule text itself was right
- next.md stated that the other 9 harnesses are strict already. This review did not re-check that
    claim; it was out of scope, and the issue named only Swift and Python
- The partial-key compare on .NET, Java, napi and WASM ("only the expected keys the result exposes")
    is still an open question that nobody has filed. Define-next raised it as "a separate question";
    weigh it if a vector ever adds an output key those surfaces do not expose
- No core, binding or bench code changed, so CRAP, iai and semver gates are untouched
