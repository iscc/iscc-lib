## 2026-11-20 — Review of: data-edge output-subset rule — spec, validator and C/C++ generator

**Verdict:** PASS

**Summary:** The output-subset rule is in `edge-vectors.md`, and the Python validator and the C/C++
header generator both enforce it, each reading the allowed keys per `gen_*` function from
`data.json`. The diff stays inside next.md's four files, the generated header is byte-identical, and
every new failure arm has its own exact-match negative test. The four lenient harnesses (.NET, Java,
napi, WASM) are untouched, as scoped, so the issue stays open in a smaller form.

**Verification:**

- [x] `uv run pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py` exits 0: 116
    passed, including missing `iscc`, unknown key, non-object and the generator unknown-key fail
- [x] An unknown output key on `gen_text_code_v0/test_0001_embedded_nul` in a temp copy is rejected
    by both: the tests cover it, and my own run of `gen.render` on a mutated fixture copy gave
    `SystemExit: … output keys ['foo'] not carried by data.json` (same for `parts` on text)
- [x] `uv run --script scripts/gen_data_edge_vectors.py`, then `git status --porcelain` on
    `crates/iscc-ffi/tests/data_edge_vectors.h`, prints nothing
- [x] `edge-vectors.md` states the rule and a matching Verified When line; the only gen result key
    in the file is the existing JSON example (line 60); `uv run scripts/check_spec_paths.py` exits 0
- [x] `mise run check` exits 0

Beyond next.md: the derived key map matches next.md's claim (text: `iscc`, `characters`; mixed:
`iscc`, `parts`; iscc: `iscc`), and the three row renderers pass their own section name. No unpushed
commit weakens a gate.

**Issues found:**

- Fixed here (docstring only): a 128-character line in the generator's module docstring and a ragged
    wrap in the test module docstring. The two test files and the pre-push gates on the three Python
    files are green after the edit, and the header is still unchanged
- Nit, not filed: the eight-line `data.json` key derivation exists twice (validator and generator).
    They are independent checks of one rule, so I left them
- Nit, not filed: `test_gen_iscc_and_characters_only_still_renders` rewrites a vector that already
    holds exactly those two keys. It proves only that the `characters` value is not rendered
- `gen_mixed_code_v0` and `gen_iscc_code_v0` hold only error vectors in `data-edge.json`, so the
    unknown-key rule runs on real data for `gen_text_code_v0` alone. The code path is shared

**Codex review:** No actionable findings. Codex ran the Python suite (681 passed), the lint checks
and the header regeneration (byte-identical).

**Next:** The .NET, Java, napi and WASM harness fan-out that the open `normal` issue describes. The
behaviour is fixed by the spec's "Output subset" rule and the decisions.md entry of 2026-11-20, so
it qualifies as a `**Fan-out:**` step with one commit per surface. Each harness gets a static set of
uncompared keys per gen function, taken from its result type, in place of the runtime intersection.
Each surface needs two negative tests, one fault per call: an expected key outside the static set
that the result lacks, and a result without `iscc`. The other two `normal` docs issues (Java how-to
`genSumCodeV0`, Ruby precompiled-gem wording) make a cheap docs step afterwards.

**Notes:**

- Recorded the decision ("data-edge output subset", 2026-11-20). decisions.md went over budget, so
    the oldest entry (xunit v3 VSTest mode) rotated into the archive
- The issue entry is rewritten to the remaining problem and now points at the "Output subset" rule
- The spec's new Verified When line is half met: the validator and generator part holds, the "every
    harness fixes its uncompared keys statically" part waits for the fan-out
- For the fan-out, read each surface's result type before writing its static set. Do not copy the
    set from another surface: a surface that returns a structured result for a function must compare
    every key there
- Observed while reading the other harnesses, not filed: Go's `edgeCompare` reads a missing key as
    `nil`, so it would pass a missing key only if the expected value were JSON `null`. No
    `data.json` output is `null`. Rust, Swift, Kotlin, Ruby and Python fail on a missing key
- No core or binding code changed: CRAP, iai and semver are unaffected
