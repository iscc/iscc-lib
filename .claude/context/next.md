# Next Work Package

## Step: Data-edge harnesses accept every valid `gen_meta_code_v0` vector shape

## Goal

Fix the `normal` `[review]` issue "Four data-edge harnesses and a generator test reject valid
`gen_meta_code_v0` vectors", so that a later meta vector with a `description` output or a data-URL
`meta` input costs one fixture edit, as `edge-vectors.md` requires. Test code only.

## Alternatives Considered

- **Chosen:** the harness-gap issue. It is the smallest open `normal` item (five test files, no
    library code), it closes the data-edge section the last step opened, and the four affected
    suites plus pytest prove it.
- **Rejected:** the pytest gate-time `[audit]` issue (1-2 files). It is just as small, but it is
    tooling hygiene, while this one is a defect in a v0.8.0 target criterion ("a vector costs one
    edit"). It goes next, with the C FFI `(NULL, 0)` core change after it, as its own step.

## Scope

- **Modify** (all test files, outside the 6-file budget):
    - `crates/iscc-napi/__tests__/data_edge.test.mjs`: `UNCOMPARED.gen_meta_code_v0`
    - `crates/iscc-wasm/tests/data_edge.rs`: `UNCOMPARED` and the doc comment above it
    - `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/DataEdgeTest.java`: `UNCOMPARED`
    - `packages/dotnet/Iscc.Lib.Tests/DataEdgeTests.cs`: `Uncompared`
    - `tests/test_gen_data_edge_vectors.py`: `test_meta_row_renders_object_as_json_text`
- **Reference**: `.claude/context/specs/edge-vectors.md` ("Output subset", "A vector costs one
    edit"), `scripts/gen_data_edge_vectors.py` (`meta_text`, `row_gen_meta_code_v0`),
    `reference/iscc-core/iscc_core/data.json` (meta output keys: `iscc`, `name`, `description`,
    `meta`, `metahash`)

## Not In Scope

- Committing the throwaway vectors. They prove the fix and are then removed from all 5 copies and
    the regenerated header goes back to its committed state. A permanent description or data-URL
    vector would be a fixture change and needs its own reason.
- The untested `parts` entry on the mixed harnesses and the missing non-object branch in the WASM
    `compare` (review noted both and did not file them).
- Splitting `call_meta` in `crates/iscc-lib/tests/test_data_edge.rs` (CRAP 30.0). Do not touch it.
    Any extra `?` or branch there reds the CI CRAP gate.
- Any change to bindings, the core, the generator script, or `data-edge.json` as committed.

## Implementation Notes

- The four sets become `name`, `description`, `meta`, `metahash`, with `description` placed next to
    `name`. These surfaces return a bare ISCC string, so every `data.json` meta output key other
    than `iscc` is uncompared (decision of 2026-11-20 on the output subset). Update every comment
    that lists the dropped keys, e.g. the WASM doc comment "meta drops `name`, `meta` and
    `metahash`".
- Generator test: for each meta row, a dict `meta` renders as
    `gen.c_str(json.dumps(meta, ensure_ascii=False).encode("utf-8"))`, a str `meta` as
    `gen.c_str(meta.encode("utf-8"))`. Both must appear in the header text. Also assert that at
    least one row has a dict `meta`, so the object path cannot go unchecked once a string row
    exists. Rename the test and its docstring to fit (e.g. `test_meta_row_renders_meta_text`). Keep
    `test_non_object_meta_fails_closed` as it is.
- Throwaway proof, done in the working tree and then reverted:
    1. Add two vectors to `crates/iscc-lib/tests/data-edge.json`, `gen_meta_code_v0` section: (a) a
        non-empty description with an object `meta`, its `outputs` carrying `description`; (b) a
        `data:application/json;base64,…` string `meta`. Take the expected outputs from iscc-core
        1.5.0 (`uv run --with iscc-core==1.5.0 --python 3.13 …`), keeping only keys from `data.json`.
    2. Copy the file to the 4 vendored copies, run `uv run --script scripts/gen_data_edge_vectors.py`.
    3. Run all 12 surfaces (commands as in the last review's handoff: napi needs a fresh `napi build`,
        WASM `wasm-pack test --node`, `mvn clean test`, Kotlin `--rerun-tasks`). Record the pass
        counts per surface in the handoff.
    4. Revert all 5 copies and the header (`git checkout -- <paths>`, on those paths only, never a
        stash or reset), and confirm the generator rerun leaves the header unchanged.
- If a full-result surface (Rust, Python, Go, Ruby, Swift, Kotlin, C, C++) fails on (a) or (b), that
    is a real harness gap of the same issue. Fix it in that surface's data-edge test file, which is
    still test code, and name it in the handoff. Don't change `call_meta`'s branch count (see Not In
    Scope). If that seems unavoidable, stop and report.

## Verification

- In each of the four Modify files for napi, WASM, JNI and .NET, the `gen_meta_code_v0` uncompared
    set contains `"description"` (read off the set literal)
- `uv run pytest tests/test_gen_data_edge_vectors.py tests/test_data_edge.py` passes; the meta-row
    test has a branch for a str `meta` and asserts that at least one dict row exists
- napi `npm test`, `wasm-pack test --node crates/iscc-wasm --test data_edge`,
    `mvn -f crates/iscc-jni/java/pom.xml clean test` and `dotnet test packages/dotnet` pass on the
    committed fixture
- The handoff reports all 12 surfaces green with both throwaway vectors present (per-surface counts)
- `git status --porcelain -- '*data-edge.json' crates/iscc-ffi/tests/data_edge_vectors.h` is empty
    at the end
- `mise run check` exits 0

## Done When

The four uncompared sets include `description`, the generator test accepts a string `meta` and still
requires an object row, and both throwaway vectors ran green on all 12 surfaces without a harness
edit before they were reverted.
