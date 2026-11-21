# Next Work Package

## Step: `gen_meta_code_v0` and `gen_video_code_v0` sections in `data-edge.json`, on all 12 surfaces

## Goal

Closes both `normal` `[human]` issues: "No shared vector pins raw `&`, `<`, `>`, U+2028 and U+2029
in canonical `meta` JSON" and "Ragged video frame signatures are rejected by the core but no shared
vector pins it". Two sections are added to `data-edge.json` and handled by every harness.

## Alternatives Considered

- **Chosen:** both sections in one fan-out. The behaviour is settled on the core and Go (iterations
    250/251, decisions.md 2026-11-21), and a section cannot be added to fewer than 12 harnesses.
    Doing both in one session pays for the toolchain setup (Swift, Kotlin, JNI, Ruby rebuilds) once.
- **Rejected:** C FFI `(NULL, 0)` (`[audit]`). It changes FFI behaviour, which goes first and alone,
    and two `[human]` issues come first.

## Scope

**Fan-out:** the two data-edge sections (behaviour fixed by the core, Go and decisions.md
2026-11-21), applied to Rust, Python, Node.js, WASM, Java, Go, .NET, Ruby, Swift, Kotlin, C and C++.
It cannot be split by surface: once a section is in the JSON, every harness without a handler and
the generator fail closed. Commit per surface; put the fixture, its 4 copies and the generator in
the last commit.

- **Modify (fixture):** `crates/iscc-lib/tests/data-edge.json` and its 4 vendored copies (same paths
    as the `alg_simhash` step: `packages/go/testdata/`, `packages/dotnet/Iscc.Lib.Tests/testdata/`,
    `packages/kotlin/src/test/resources/`, `packages/swift/Tests/IsccLibTests/`)
- **Modify (harnesses):** `crates/iscc-lib/tests/test_data_edge.rs`, `tests/test_data_edge.py`,
    `crates/iscc-napi/__tests__/data_edge.test.mjs`, `crates/iscc-wasm/tests/data_edge.rs`,
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/DataEdgeTest.java`,
    `packages/go/data_edge_test.go`, `packages/dotnet/Iscc.Lib.Tests/DataEdgeTests.cs`,
    `crates/iscc-rb/test/test_data_edge.rb`,
    `packages/swift/Tests/IsccLibTests/DataEdgeTests.swift`,
    `packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/DataEdgeTest.kt`,
    `crates/iscc-ffi/tests/test_iscc.c`, `packages/cpp/tests/test_iscc.cpp`
- **Modify (generator):** `scripts/gen_data_edge_vectors.py`, the regenerated
    `crates/iscc-ffi/tests/data_edge_vectors.h`, `tests/test_gen_data_edge_vectors.py` if it pins
    the section list
- **Reference:** `.claude/context/specs/edge-vectors.md`, commit `3237e83` (`next.md` of the
    `alg_simhash` fan-out) and its 11 advance commits up to `066a149`, `packages/swift/CLAUDE.md`
    (Linux Swift recipe), each surface's `data.json` conformance test (its meta and video calls)

## Not In Scope

- The three Go inputs of the `low` issue (UTF-16 key order, `-0`, lone surrogates): Go fails them
- Any core, Go or binding behaviour fix, and any `except` entry. If a surface disagrees with a
    vector, stop and report it in the handoff. Do not patch around it inside the fan-out
- The Python binding's own frame check in `extract_frame_sigs`, and the C FFI inner-pointer checks
    (the `[audit]` issue)
- A `soft_hash_video_v0` section (the ragged vector on `gen_video_code_v0` covers the issue)

## Implementation Notes

- **`gen_meta_code_v0/test_0001_raw_jcs_characters`**: inputs `["Hello", "", <meta object>, 64]`.
    The meta object is the `combined` case of `jcsStringCases` in `packages/go/code_meta_test.go`
    written as a JSON **object**, not a string (iscc-core reads a string meta as a data URL and
    crashes on it). Its keys are `title` (`Tom & Jerry <3 > `, then U+2028, a space and U+2029) and
    `note` = `a`, a literal backslash, `u2028b `, then U+0008, U+000C, U+001F. Outputs (probed:
    iscc_lib and iscc-core 1.5.0 agree, so no `diverges`): `iscc` `ISCC:AAAWKLHFXO3IKDKI`, `name`
    `Hello`, `metahash` `1e207dd6353ce193db311db03f79168a99738635c0c36f66ab187f24e62c831d669c`, and
    `meta` = the data URL of the Go `combined` case (`data:application/json;base64,eyJub3RlIjoi…`).
    Copy the exact values from Python `iscc_lib.gen_meta_code_v0`, not from this file
- **`gen_video_code_v0`**, bits 64, with F1 = `list(range(380))`:
    - `test_0001_equal_length`: `[[F1, [i % 3 for i in range(380)]], 64]` → `ISCC:EMAVFD4RIMPXYQWS`
        (probed on both). It makes every harness decode nested int lists with non-zero values
    - `test_0002_ragged_frame_sigs`: the same, but the second frame is `[i % 3 for i in range(381)]`;
        `error`, `diverges` "decisions.md 2026-11-21: Video frame signatures of unequal length are
        rejected, where iscc-core truncates". iscc-core returns `ISCC:EMAVFD4RIMPXYQWS`
- **Fixture layout:** keep the existing entries byte-identical; the frame lists may be written one
    array per line. Copy the canonical file over the 4 copies, never edit them by hand
- **Meta input on each surface:** convert the JSON object to the string the binding takes, as its
    `data.json` conformance test does for `test_0014_meta_object_json`. A serializer that escapes
    `<`/`&` (Gson, System.Text.Json, Go `json.Marshal`) is fine: the core parses the string before
    it canonicalizes. Compare `meta` exactly; a result type without some keys (C, C++) leaves them
    uncompared, fixed statically per the "Output subset" rule
- **Python:** `outcome()` already passes dicts and nested lists through; add the result converters
    and the `SEED_SET` row only if the existing tables require them. The video vector must come out
    as "differs from iscc-core" because of `diverges`
- **Generator:** the meta string renders as a C literal with explicit length, ASCII only (escape
    every non-ASCII byte and control character); write the JSON with `json.dumps`. Video needs a
    struct with `const int32_t *const *frames`, `lens`, `count` matching `iscc_gen_video_code_v0`
    (`iscc.h:282`); C++ builds a `std::vector<std::vector<int32_t>>`. Regenerate with
    `uv run --script scripts/gen_data_edge_vectors.py`
- **Rust:** `gen_meta_code_v0` sits behind the `meta-code` feature; give its handler a compiled-out
    twin as `call_text` has. Keep each new function at cyclomatic complexity ≤ 5 (CI scores test
    helpers at 0% coverage). If a CRAP score moves, run
    `mise run coverage && mise run crap:baseline`
- **Toolchains:** rebuild napi, Ruby and JNI native artifacts first; reinstall Swift per its
    `CLAUDE.md` if `/tmp/swifttc` is gone; for Kotlin force the run and read the JUnit XML. .NET:
    `cargo build -p iscc-ffi`, then
    `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug`

## Verification

- `cargo test -p iscc-lib --test test_data_edge` and
    `uv run pytest tests/test_data_edge.py tests/test_gen_data_edge_vectors.py tests/test_vendored_fixtures.py`
    pass; `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info` then
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above` exit
    0
- Node (`npm test` in `crates/iscc-napi` after a fresh build),
    `wasm-pack test --node crates/iscc-wasm`, `mvn -f crates/iscc-jni/java/pom.xml clean test`,
    `CGO_ENABLED=0 go test ./...` in `packages/go`, the .NET command above and
    `bundle exec rake test` in `crates/iscc-rb` all pass
- Swift `swift test` (Linux recipe) and Kotlin `./gradlew test` pass; Kotlin's JUnit XML shows the
    data-edge test ran with 0 failures
- The C test and the C++ CMake/ctest run (both as in `ci.yml`) pass, each running rows of both
    sections
- `uv run --script scripts/gen_data_edge_vectors.py` then
    `git status --porcelain crates/iscc-ffi/tests/data_edge_vectors.h` prints nothing, and
    `python3 -c "import json;d=json.load(open('crates/iscc-lib/tests/data-edge.json'));assert len(d['gen_meta_code_v0'])==1 and len(d['gen_video_code_v0'])==2"`
    exits 0
- `mise run check` exits 0

## Done When

Both sections are in `data-edge.json` and its copies, all 12 surfaces run them green, and a fresh
regeneration leaves the generated header unchanged.
