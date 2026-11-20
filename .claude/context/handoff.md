## 2026-11-20 — data-edge output-subset rule — spec, validator and C/C++ generator

**Done:** I wrote the output-subset rule into `edge-vectors.md`: a new "Output subset" rule, an
amended "Every surface runs every vector" bullet, and a Verified When line. The Python validator and
the C/C++ header generator now enforce it, and both read each `gen_*` function's allowed output keys
from `data.json`. Neither keeps a hand-written table. The regenerated header is byte-identical.

**Files changed:**

- `.claude/context/specs/edge-vectors.md`: the rule and the Verified When line. The rule refers to
    `data.json` and lists no gen result keys. The only `characters` in the file is the existing JSON
    example on line 60.
- `scripts/gen_data_edge_vectors.py`: adds `DATA_JSON_PATH`, `gen_output_keys()` (cached; the union
    of output keys over each `data.json` gen section) and `allowed_output_keys(section)`. That
    function raises `SystemExit` when a section has no `data.json` counterpart, and the lookup runs
    before the error-vector early return. `gen_iscc(vector, where, section)` also raises
    `SystemExit` on any output key outside the allowed set. The three row renderers pass their
    section name explicitly. The module docstring lists the new fail-closed cases. `HEADER_PREAMBLE`
    is untouched.
- `tests/test_data_edge.py`: `validate_vector(section, name, vector)` gains a section argument.
    `GEN_OUTPUT_KEYS` is derived from `data.json`. The new helper `validate_gen_outputs` reports a
    missing counterpart (on error vectors too), outputs that are not an object, a missing `iscc`,
    and unknown keys. Added 8 tests:
    - positive: `characters` passes for `gen_text_code_v0`; an error vector passes; the derived table
        is not empty
    - negative: unknown `foo`, `parts` on text, missing `iscc`, non-object, no counterpart
    - Each negative test changes one thing in a copy of the real `test_0001_embedded_nul` vector and
        asserts that the problems list has exactly one entry.
    - The existing `GOOD` cases now pass the section `iscc_decode`.
- `tests/test_gen_data_edge_vectors.py`: 4 tests:
    - an unknown `foo` key fails closed (`SystemExit` matching `foo`)
    - a vector holding only `iscc` + `characters` renders identically
    - the allowed keys equal the `data.json` gen sections
    - `gen_foo_v0` fails closed even on an error vector

**Verification:**

- `uv run pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py -q`: 116 passed
- Mutation check: I replaced the unknown-key computation with `unknown = []` in both the generator
    and the validator. Exactly 3 tests went red: `test_validate_gen_outputs_rejects_unknown_key`,
    `..._rejects_key_of_other_function` and `test_unknown_gen_output_key_fails_closed`. After I
    restored both files, all 116 passed.
- `uv run --script scripts/gen_data_edge_vectors.py`, then
    `git status --porcelain crates/iscc-ffi/tests/data_edge_vectors.h`, printed nothing
- `uv run scripts/check_spec_paths.py`: OK, exit 0
- `mise run check`: exit 0
- `prek run --hook-stage pre-push --files <3 py files>`: exit 0. The first run caught a `ty` error
    on `sorted(set(outputs) - allowed)` (object elements). Fixed with `str(key)`, the same pattern
    as the existing `unknown` check.
- I confirmed that the C++ `TextCodeResult`, `MixedCodeResult` and `IsccCodeResult` hold only
    `std::string iscc` (`iscc.hpp:177-214`), so C and C++ leave every non-`iscc` key uncompared.

**Next:** The .NET, Java, napi and WASM `Compare`/`compare` fan-out, with a negative test per
surface. Each harness gets a static set of uncompared keys per gen section, taken from its result
type, replacing the runtime intersection. The new Verified When line ("every harness fixes its
uncompared output keys statically…") stays unmet until that fan-out lands.

**Notes:**

- Proposed decisions.md entry for review to record:
    - **data-edge output subset:** allowed keys come from `data.json`, and the keys left uncompared
        are fixed per harness from the result type (2026-11-20)
    - In a `gen_*` section, `outputs` must hold `iscc` and may hold only keys that the function's
        `data.json` vectors carry. Any other key fails on every surface. A harness never intersects
        the expected keys with the returned ones.
    - Rejected: a per-surface `except` value carrying reduced outputs, and a top-level metadata key.
        Either would be a 12-surface change, because the generator accepts only `error`/`skip` and
        every harness fails on an unknown section, and it would break "a vector costs one edit".
- "Gen section" means the `gen_` name prefix, not membership in the derived map, so a gen section
    missing from `data.json` fails closed instead of passing unchecked.
- No core or binding code changed, so CRAP, iai and semver are unaffected.
