## 2026-11-21 — Review of: Data-edge harnesses accept every valid `gen_meta_code_v0` vector shape

**Verdict:** PASS

**Summary:** The napi, WASM, JNI and .NET harnesses now leave `name`, `description`, `meta` and
`metahash` uncompared for `gen_meta_code_v0`, which is every `data.json` meta output key except
`iscc`. The generator test accepts a string `meta` and still requires one object row. The diff is
five test files and nothing else. I reproduced the throwaway proof with three vectors of my own, and
the issue is closed.

**Verification:**

- [x] The four uncompared sets contain `"description"` — read off the four literals. They equal the
    `data.json` meta output keys minus `iscc`
- [x] `uv run pytest tests/test_gen_data_edge_vectors.py tests/test_data_edge.py` — 131 passed. The
    meta-row test has a `str` branch and asserts `object_rows >= 1`
- [x] napi, WASM, JNI and .NET pass on the committed fixture — napi 182 pass / 0 fail (fresh
    `napi build`), WASM `data_edge` 8 passed, `mvn clean test` 116 run (DataEdgeTest 8, 0 failures),
    .NET 153 passed. `dotnet test packages/dotnet` as next.md writes it exits with MSB1003 (no
    project file at that level, also before this step). I ran the `ci.yml` form,
    `dotnet test packages/dotnet/Iscc.Lib.Tests/`
- [x] The handoff reports all 12 surfaces green with both throwaway vectors — it does, with counts.
    Reproduced with three vectors of my own from iscc-core 1.5.0 (description + object `meta`;
    data-URL `meta`; description + data-URL `meta`): Rust 7 passed, Go 5 PASS, napi 182/0, WASM 8,
    JNI DataEdgeTest 8/0, .NET 153, C 107 passed (104 + 3 rows), C++ 115 (112 + 3), Ruby 0 failures,
    Kotlin DataEdgeTest 6/0, Swift 30/0, pytest 137 passed. The only failures were 3
    `test_vector_schema` rows, because my probe vectors had no `note` (my omission, the validator
    did its job)
- [x] `git status --porcelain -- '*data-edge.json' crates/iscc-ffi/tests/data_edge_vectors.h` is
    empty — after my own revert and a generator rerun as well
- [x] `mise run check` exits 0. `cargo clippy --workspace --all-targets -- -D warnings` is clean

**Issues found:**

- Not filed (nit): in `test_meta_row_renders_meta_text` a data-URL input is also the expected `meta`
    output in the header, so `in text` cannot tell the input from the output for such a row. The C
    and C++ runs would catch a wrongly rendered input, because the `iscc` would differ
- Not filed (process): next.md's `dotnet test packages/dotnet` criterion cannot run. Copy the test
    command from `ci.yml`
- No gate weakening in the unpushed commits. No core, binding or generator change

**Codex review:** No findings. Codex ran the Python, Node.js, WASM, JNI and .NET tests and its own
temporary vectors for a description output and a string `meta`. All passed.

**Next:** The `normal` `[audit]` issue "Pytest gate spends ~25 s of ~44 s on benchmark loops and
doc-tree copies" (1-2 files), as planned. After it, the C FFI `(NULL, 0)` core change as its own
step.

**Notes:**

- Deleted from issues.md: "Four data-edge harnesses and a generator test reject valid
    `gen_meta_code_v0` vectors". Four `normal` issues remain, all `[audit]`: the C FFI `(NULL, 0)`
    split, out-of-range integer arguments, published-package docs, and the pytest gate time
- A throwaway data-edge vector needs a non-empty `note`, or `test_vector_schema` fails on it
- With a `meta` input present, iscc-core ignores the description for the similarity hash: my
    data-URL vectors with and without a description have the same `iscc` and `metahash`. A
    description vector that should move the `iscc` needs an empty `meta`
- No learnings.md or decisions.md change: the rule behind this fix was recorded in iteration 252
