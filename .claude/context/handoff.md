## 2026-11-21 — Data-edge harnesses accept every valid `gen_meta_code_v0` vector shape

**Done:** The napi, WASM, JNI and .NET `gen_meta_code_v0` uncompared sets now hold `name`,
`description`, `meta`, `metahash`, and their doc comments list the same keys. The generator test
`test_meta_row_renders_meta_text` renders a dict `meta` as JSON text and a str `meta` as is, and
asserts at least one dict row exists. Two throwaway meta vectors (description output; data-URL
string `meta`) ran green on all 12 surfaces with no other harness edit, then were reverted.

**Files changed:**

- `crates/iscc-napi/__tests__/data_edge.test.mjs`: `UNCOMPARED.gen_meta_code_v0` + comment
- `crates/iscc-wasm/tests/data_edge.rs`: `UNCOMPARED` meta entry (rustfmt split it over lines) + doc
    comment
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/DataEdgeTest.java`: `UNCOMPARED` + Javadoc
- `packages/dotnet/Iscc.Lib.Tests/DataEdgeTests.cs`: `Uncompared` + summary
- `tests/test_gen_data_edge_vectors.py`: `test_meta_row_renders_object_as_json_text` →
    `test_meta_row_renders_meta_text` (dict/str branches, `object_rows >= 1`).
    `test_non_object_meta_fails_closed` unchanged

**Verification:**

Throwaway vectors (outputs from `uv run --no-project --with iscc-core==1.5.0 --python 3.13`, only
`data.json` keys), in all 5 fixture copies plus a regenerated header:

- `test_0002_description_object_meta`: `["Hello", "A short description", {"title": "x"}, 64]` → iscc
    `ISCC:AAAWKLHFXO3B7GJD`, outputs `iscc`/`name`/`description`/`meta`/`metahash`
- `test_0003_data_url_meta`: `["Hello", "", "data:application/json;base64,eyJuIjox…", 64]` → iscc
    `ISCC:AAAWKLHFXP6JMUP3`, outputs `iscc`/`name`/`meta`/`metahash`

Per surface with both vectors present (all green):

- Rust `cargo test -p iscc-lib --test test_data_edge`: 7 passed
- Python `pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py`: 137 passed (131 on
    the committed fixture, so the 2 vectors add 6 parametrized rows)
- Go `go test -count=1 -run DataEdge ./...`: 5 tests PASS
- Node (fresh `napi build --platform`) `npm test`: 182 pass, 0 fail
- WASM `wasm-pack test --node crates/iscc-wasm --test data_edge`: 8 passed
- JNI `cargo build -p iscc-jni` + `mvn clean test`: 116 run, DataEdgeTest `tests="8" failures="0"`
- .NET `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug`: 153
    passed
- Ruby `rake compile` + `rake test`: 165 runs, 0 failures, 0 errors
- Swift (Linux toolchain `/tmp/swifttc`): 30 tests, 0 failures
- Kotlin `./gradlew test --rerun-tasks`: `TEST-uniffi.iscc_uniffi.DataEdgeTest.xml` `tests="6"`, 0
    failures/errors
- C (gcc as in ci.yml): 106 passed, 0 failed; prints PASS for `test_0002…` and `test_0003…`
- C++ (`uvx cmake`, `SANITIZE_ADDRESS=ON`): 114 passed, 0 failed; PASS for both rows

Fix is real: with only the napi set put back to `['name', 'meta', 'metahash']`, `npm test` fails 1
("test_0002_description_object_meta: missing field description in {"iscc":…}"); set restored after.

Revert: `git checkout --` on the 5 fixture copies and the header only. After a generator rerun,
`git status --porcelain -- '*data-edge.json' crates/iscc-ffi/tests/data_edge_vectors.h` prints
nothing.

On the committed fixture: pytest 131 passed; napi 182 pass; WASM data_edge 8 passed;
`mvn clean test` exit 0 (DataEdgeTest 8, 0 failures); .NET 153 passed. `mise run check` exits 0
(first run failed only because rustfmt reformatted the WASM tuple).
`prek run --hook-stage pre-push --files <5 files>` exits 0 (clippy, cargo test, ty, pytest).

**Next:** The pytest gate-time `[audit]` issue, as next.md planned. After it, the C FFI `(NULL, 0)`
core change, alone.

**Notes:**

- No full-result surface (Rust, Python, Go, Ruby, Swift, Kotlin, C, C++) needed a change: each
    already compares `description` and accepts a string `meta`. `call_meta` was not touched
- The WASM data_edge run came before rustfmt reformatted the tuple. The change is layout only, and
    the pre-push hook ran clippy and cargo test on it afterwards
- `dotnet test packages/dotnet` (as next.md writes it) fails with MSB1003, since there is no
    solution file at that level. CI runs `packages/dotnet/Iscc.Lib.Tests/` with `LD_LIBRARY_PATH`,
    and so did I
- The throwaway fixture was written with plain `json.dumps(indent=2)`, which differs in layout from
    the committed fixture (compact arrays, escaped U+2028 and U+2029). It was reverted, so this has
    no effect
