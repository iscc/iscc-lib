# Next Work Package

## Step: data-edge output-subset rule — spec, validator and C/C++ generator

## Goal

Settle the rule that the four lenient data-edge harnesses wait on (issue "Four `data-edge.json`
harnesses ignore expected output keys their result lacks"): which expected output keys a surface
with a reduced gen result may leave uncompared, declared statically instead of intersected at
runtime. Land it in `edge-vectors.md`, the central validator, and the C/C++ generator, which has the
same silent drop (`gen_iscc` reads `outputs["iscc"]` and ignores every other key).

## Alternatives Considered

- **Chosen:** the rule plus its two Python landing sites. It is the design decision the issue names
    as its route, and the .NET/Java/napi/WASM fan-out cannot start without it. A spec-settling step
    must land alone, before the fan-out.
- **Rejected:** Java how-to `genSumCodeV0` fix plus the Ruby "precompiled up to 3.3" docs line. Both
    are cheap docs fixes but unblock nothing. They make a good next docs step.
- **Rejected (design):** a per-surface `except` value carrying reduced outputs, or a top-level
    metadata key. The generator accepts only `error`/`skip` and every harness fails on an unknown
    section, so either is a 12-surface change, and an `except` per subset surface breaks "a vector
    costs one edit".

## Scope

- **Modify**: `.claude/context/specs/edge-vectors.md`, `scripts/gen_data_edge_vectors.py`,
    `tests/test_data_edge.py` (validator), `tests/test_gen_data_edge_vectors.py`
- **Regenerated (must not change)**: `crates/iscc-ffi/tests/data_edge_vectors.h`
- **Reference**: `crates/iscc-lib/tests/data.json` (the gen sections' `outputs` keys),
    `crates/iscc-lib/tests/data-edge.json`, `packages/cpp/include/iscc/iscc.hpp` (result structs,
    around line 177: `TextCodeResult` and `MixedCodeResult` hold only `iscc`)

## Not In Scope

- The .NET, Java, napi and WASM harnesses (`Compare`/`compare` in the four files the issue names).
    They are the next step, one fan-out with a negative test per surface. Do not touch them here
- Changing any binding to return a structured result (an API/DX decision for Titusz)
- Adding vectors or new sections to `data-edge.json`; adding a new `except` value form
- The Java how-to, the Ruby gem docs, and the Tier 1 doc-gate holes (other `normal` issues)

## Implementation Notes

**The rule** (write it into `edge-vectors.md`: amend the "Every surface runs every vector" bullet,
add an "Output subset" rule under Rules, and add a Verified When line):

1. For a `gen_*` section, `outputs` is an object that must hold `iscc` and may hold only keys that
    the same function's vectors in `data.json` carry (`gen_text_code_v0`: `iscc`, `characters`;
    `gen_mixed_code_v0`: `iscc`, `parts`; `gen_iscc_code_v0`: `iscc`). Read these from `data.json`
    when you verify, but **do not write the key list into the spec**: the open derived-fact issue
    forbids hand-written lists there. State the rule by reference to `data.json`.
2. `iscc` is always compared. A surface whose result type for that function lacks some of the
    allowed keys leaves exactly those uncompared. That set is fixed statically from the result
    type, once per harness, and never computed from the returned value. Intersecting with the
    returned keys is the defect being fixed.
3. An expected key that the function's `data.json` vectors do not carry fails the vector on every
    surface. A key the surface's result type has is always compared.

**Validator** (`tests/test_data_edge.py`): `validate_vector` currently has no section argument. Make
the gen-section check able to see the section, derive the allowed keys per gen section from
`data.json` (read it once; fail closed if a gen section of `data-edge.json` has no `data.json`
counterpart), and report: outputs not an object, `iscc` missing, a key outside the allowed set. Add
one-bad-case-per-call negative tests, each feeding exactly one fault: missing `iscc`, an unknown key
such as `"foo"`, and a non-object. Also add a positive test that `characters` passes for
`gen_text_code_v0`.

**Generator** (`scripts/gen_data_edge_vectors.py`): C and C++ results expose only `iscc`, so every
other allowed key is uncompared there. `gen_iscc` must fail closed (`SystemExit`, like the existing
checks) on a key outside the allowed set. Derive the allowed set from `data.json` in the generator
too (it is a PEP 723 script; stdlib `json` suffices), rather than keeping a hand-written table. Add
`*_fails_closed(tmp_path)` tests next to the existing ones (an unknown key in a `gen_text_code_v0`
vector fails; a vector holding only `iscc` and `characters` still renders). The rendered header must
be byte-identical: `test_tracked_header_matches_fresh_render` already proves that.

**Decision record:** advance does not write `decisions.md`. Put a proposed entry in the handoff
Notes, "data-edge output subset: allowed keys come from `data.json`, uncompared keys are fixed per
harness from the result type", with the rejected routes above, for review to record.

No core or binding code changes: CRAP, iai and semver are unaffected.

## Verification

- `uv run pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py` exits 0, including the
    new negative tests (missing `iscc`, unknown key, non-object; generator unknown-key fail)
- With an unknown output key added to `gen_text_code_v0/test_0001_embedded_nul` in a temp copy of
    the fixture, both the validator and the generator reject it (covered by the tests above)
- `uv run --script scripts/gen_data_edge_vectors.py` followed by
    `git status --porcelain   crates/iscc-ffi/tests/data_edge_vectors.h` prints nothing
- `edge-vectors.md` states the output-subset rule and has a matching Verified When line, and
    contains no literal list of the gen result keys; `uv run scripts/check_spec_paths.py` exits 0
- `mise run check` exits 0

## Done When

The rule is in `edge-vectors.md`, and the validator and the C/C++ generator both enforce it with
negative tests, while the generated header stays unchanged.
