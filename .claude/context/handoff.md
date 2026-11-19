## 2026-11-19 — Review of: data-edge.json fan-out, group 2 — Swift and Kotlin harnesses, plus the Ruby key-compare fix

**Verdict:** PASS_WITH_NOTES

**Summary:** Swift and Kotlin now run every `data-edge.json` vector through byte-identical,
registered vendored copies. Ruby's compare fails on a missing expected key. Each of the three
surfaces has the sixth negative test, landed one commit per surface (`cc53186`, `616f5ae`,
`a75832b`). The harnesses follow `edge-vectors.md` → Rules and the reviewed Rust/Python templates.
Every check passed in my own runs. The notes are harness-strictness edge cases that no current
vector or binding can reach.

**Verification:**

- [x] `uv run pytest tests/test_vendored_fixtures.py tests/test_data_edge.py`: 86 passed before my
    validator row, 87 after. `mise run check` exit 0, tree clean
- [x] Swift (`/tmp/swifttc` 6.1.2, `--scratch-path /tmp/swiftbuild`, after
    `cargo build -p iscc-uniffi`): exit 0. All 6 `DataEdgeTests` listed as passed by name; 25 tests,
    0 failures. **Linux only**: the macOS `swift` CI job has not run it yet
- [x] Kotlin `./gradlew cleanTest test`: exit 0. `TEST-uniffi.iscc_uniffi.DataEdgeTest.xml` has
    tests=6, skipped=0, failures=0, errors=0 and 6 `testcase` elements
- [x] Ruby `rake compile && rake test TESTOPTS=-v`: 160 runs, 0 failures. All 6 `TestDataEdge#…`
    tests ran by name. `bundle exec standardrb` exit 0
- [x] Probes: (1) `swift` / `kotlin` are the ids in `SURFACE_IDS` and the spec, so `except` is live;
    (2) the missing-key negative's `iscc` equals the canonical vector's `ISCC:EAASKDNZNYGUUF5A`, so
    the missing key is its only bad case; (3) Codex's Swift claim `"K" == "\u{212A}"` prints `true`
- [x] Scope: Package.swift is the only non-test, non-doc file touched. No suppressions, skips or
    gate edits anywhere in `@{upstream}..HEAD`. No core, binding or `data-edge.json` change

**Issues found:**

- Filed `low` (untriaged): "Swift `data-edge.json` compare is looser than byte equality". Swift
    `String ==` uses canonical equivalence, and `as? Int` accepts JSON booleans. Unreachable today,
    since the Rust harness pins the same bytes strictly
- Fixed (step 9): `outputs: {}` passed vacuously on every harness, the Rust and Python templates
    included. `validate_vector` in `tests/test_data_edge.py` now rejects it, pinned by the
    `empty_outputs` row in `test_validate_vector_rejects_malformed`
- Fixed (step 9): both package `CLAUDE.md` subsections had a hand-written "6 test methods" count,
    against the open `normal` [human] derived-facts issue. Replaced with count-free wording
- Nit: Kotlin `convert*` use unchecked `as` casts outside the `try`. A wrong record type crashes the
    test rather than reporting a failure, so it still fails closed
- Resolved and deleted: "Ruby `data-edge.json` harness passes a gen result that lacks an expected
    key"

**Codex review:** one run over the whole range (`--base 32c7813`). (P2) Swift string compare is
canonical equivalence. Confirmed and filed `low`. (P3) Swift reads a JSON bool as an Int. Filed in
the same entry. (P3) The Ruby rewrite dropped the `keys.empty?` guard, so `outputs: {}` passes. The
Rust and Python templates behave the same way, so I fixed it once in the fixture validator, not per
harness.

**Next:** Fan-out group 3: C and C++ through a generated table plus a staleness check. Recover
`scripts/gen_ffi_boundary_vectors.py` from `ea69169`. Cheap `normal` alternatives: the Python
`encode_seq` docstring issue, and the Dependabot alerts on `soupsieve`/`setuptools`/`json`.

**Notes:**

- update-state: check that the macOS `swift` CI job is green on the pushed sha. Swift
    `Bundle.module` resource lookup and `JSONSerialization` bridging have only run on Linux
- Go still finds a missing key only because `gotFields[key]` reads as nil (advance note). The new
    validator rule covers the empty-object case for every surface
- issues.md is at 300 lines, exactly on budget. The next filer should trim before adding
