# Spec: Shared Edge-Case Vectors — `data-edge.json`

One file of edge-case inputs with their expected outcome, run by the conformance harness of every
surface. It exists because `data.json`, the iscc-core reference vectors, holds only well-formed
inputs. Every disagreement found between two surfaces so far was on an input `data.json` does not
cover: base32 padding and spare bits, non-ASCII case-fold aliases, an embedded NUL, an empty list.
Each was found by chance and then pinned by a hand-written test on one or two surfaces.

The file is this project's extension of the reference vectors. It grows as cross-language
idiosyncrasies surface, and it is meant to be contributed to iscc/iscc-core later (see "Upstream").

## The File

- Canonical path: `crates/iscc-lib/tests/data-edge.json`. It is vendored next to every vendored copy
    of `data.json`, and `tests/test_vendored_fixtures.py` keeps the copies identical
- Test data only: no library code embeds or loads it, and `conformance_selftest` does not run it.
    Packaged artifacts (crate, wheel, npm package, gem, jar, NuGet package) exclude it. Surfaces
    that are distributed as source (the Go module, the Swift package) carry it as test data next to
    their `data.json` copy
- Same layout and value encodings as `data.json`: a top-level object keyed by Tier 1 function name
    (snake_case, as in iscc-core), each holding named vectors (`test_NNNN_<slug>`)

Each vector is an object with these keys:

- `inputs` (required): positional arguments, encoded as in `data.json`
- `outputs` or `error` (exactly one): `outputs` is the expected result, encoded as in `data.json`.
    `error` is a short English reason and means the call must be rejected. Harnesses assert the
    rejection only and never match the text against an error message
- `note` (required): one sentence naming the idiosyncrasy that surfaced the vector
- `diverges` (optional): present when iscc-core 1.5.0 does not produce the stated outcome. Its value
    names the decisions.md entry that sanctions the difference. A difference without such an entry
    is a bug or an escalation, never a `diverges` marker
- `except` (optional): per-surface deviations, keyed by surface id. `"error"` means this surface
    must reject the input. `"skip"` means its language cannot express the input (a C `char*` cannot
    hold an embedded NUL). The `note` names the decisions.md entry or the reason

Surface ids: `rust`, `python`, `nodejs`, `wasm`, `c`, `cpp`, `java`, `go`, `ruby`, `dotnet`,
`swift`, `kotlin`.

```json
{
  "iscc_decode": {
    "test_0001_base32_spare_bits": {
      "inputs": [
        "ISCC:GAAAAAAAAB"
      ],
      "error": "non-zero spare bits in the last base32 character",
      "note": "The Go port accepted this alias of ISCC:GAAAAAAAAA while Rust rejected it.",
      "diverges": "decisions.md 2026-11-18: Base32 decoding stays strict"
    }
  },
  "gen_text_code_v0": {
    "test_0001_embedded_nul": {
      "inputs": [
        "Hello\u0000World",
        64
      ],
      "outputs": {
        "iscc": "ISCC:EAASKDNZNYGUUF5A",
        "characters": 10
      },
      "note": "C-string marshalling cut the text at the NUL; .NET and C++ reject it by decision.",
      "except": {
        "dotnet": "error",
        "cpp": "error",
        "c": "skip"
      }
    }
  }
}
```

## Rules

- **Every surface runs every vector.** The Rust core and all 11 bindings run the whole file from
    their conformance harness. A harness fails on a function section it does not handle. It never
    skips or special-cases a vector in code: `except` in the file is the only per-vector deviation,
    so every deviation is visible in one place. A harness compares every expected output key, except
    the keys its result type lacks under "Output subset" below
- **Output subset.** In a `gen_*` section, `outputs` is an object that holds `iscc` and may hold
    only keys that the same function's vectors in `data.json` carry. The allowed keys are read from
    `data.json`, never kept as a hand-written list; a vector with any other key fails on every
    surface, and so does a `gen_*` section without a `data.json` counterpart. `iscc` is always
    compared. A surface whose result type for that function lacks some of the allowed keys leaves
    exactly those keys uncompared. That set is fixed once per harness from the result type and is
    never computed from the returned value: a key the result type has is always compared, so a
    returned result that lacks it fails the vector
- **A vector costs one edit.** Adding a vector to a section the harnesses already handle needs no
    change to any harness. C and C++ test programs may read a generated table instead of parsing
    JSON, provided a check fails when the table is stale. A generator of this kind existed for the
    removed Unicode boundary vectors (`scripts/gen_ffi_boundary_vectors.py`, added in commit
    `ea69169`); recover it from git history instead of writing one from scratch
- **Python checks the reference.** For every function iscc-core has, the Python suite runs each
    vector against iscc-core as well as iscc-lib. The two must agree when `diverges` is absent and
    must differ when it is present, so a divergence that upstream removes turns the test red. This
    replaces hand-kept lists such as `STRICT_BASE32_DIVERGENCES` in `tests/test_seq.py`
- **Only inputs every language can express in JSON belong here.** Wrong argument types, `null`,
    objects with conversion hooks and resource exhaustion stay in per-binding tests

## Collecting Vectors

When any role finds a JSON-expressible input on which two surfaces behave differently, or on which a
surface differs from iscc-core, the fix adds a vector to this file in the same step. A test on one
surface alone does not close such a finding. A deliberate difference also needs its decisions.md
entry before it can carry `diverges` or `except`.

## Upstream

The maintainer decides when the vectors are contributed to iscc/iscc-core and signals it explicitly.
Until then nobody opens an upstream pull request or issue for them, neither an agent nor an
interactive session. Vectors that carry `diverges` are contributed only if upstream adopts the
stricter behaviour; https://github.com/iscc/iscc-core/issues/140 tracks that for base32 strictness
and case folding. `error`, `note`, `diverges` and `except` are this project's additions, because
`data.json` has no rejection vectors. The upstream format is settled at contribution time.

## Seed Set

The first version of the file holds at least the cases already found:

- `iscc_decode`: `ISCC:GAAAAAAAAA======` (padding), `ISCC:GAAAAAAAAB` and
    `ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGJ` (spare bits) are rejected; `ISCC:GAAAAAAAAA` decodes
- `encode_seq`: `["gaawdlqng6ſsdpf2"]` (non-ASCII case-fold alias) is rejected and its ASCII twin
    `["gaawdlqng6ssdpf2"]` encodes; `[]` is rejected; a unit followed by `\u0000junk` is rejected
- `gen_text_code_v0`: `"Hello\u0000World"` (the example above)
- `gen_mixed_code_v0` and `gen_iscc_code_v0`: `[]` is rejected

## Verified When

- `crates/iscc-lib/tests/data-edge.json` exists with the seed set, and a test rejects a vector that
    lacks `inputs`, lacks `note`, or does not have exactly one of `outputs` / `error`
- `tests/test_vendored_fixtures.py` covers the vendored copies of `data-edge.json`
- The Rust core and all 11 bindings run every vector, honour `except`, and fail on a function
    section their harness does not handle
- No harness skips or special-cases a vector in code
- The validator in `tests/test_data_edge.py` and `scripts/gen_data_edge_vectors.py` read each
    `gen_*` function's allowed output keys from `data.json` and reject a vector whose `outputs` is
    not an object, lacks `iscc`, or holds a key outside that set; every harness fixes its uncompared
    output keys statically from its result type and never intersects with the returned keys
- The Python suite checks every vector against iscc-core 1.5.0 as described under "Rules", and
    `tests/test_seq.py` keeps no separate hand-written divergence list
- The file is absent from every packaged artifact, and no library code embeds or loads it
