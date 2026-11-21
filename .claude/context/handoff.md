## 2026-11-21 — `gen_meta_code_v0` and `gen_video_code_v0` sections in `data-edge.json`, on all 12 surfaces

**Done:** Added `gen_meta_code_v0/test_0001_raw_jcs_characters` (the Go `combined` object, raw `&`,
`<`, `>`, U+2028, U+2029, the literal text `\u2028`, `\b\f\u001f`) and `gen_video_code_v0`
`test_0001_equal_length` / `test_0002_ragged_frame_sigs` (`error` + `diverges` decisions.md
2026-11-21). Every harness and the C/C++ generator handle both sections. One commit per surface
(12), fixture + 4 copies + generator + header in the last commit.

**Files changed:**

- `crates/iscc-lib/tests/data-edge.json` + 4 vendored copies (byte-identical, `cmp` clean): two
    sections appended; existing entries untouched (diff is 47 insertions only). U+2028/U+2029 are
    written as `\u2028`/`\u2029` JSON escapes, frames one array per line
- `crates/iscc-lib/tests/test_data_edge.rs`: `call_meta` (+ compiled-out twin for no `meta-code`),
    `meta_arg` (gated), `call_video`, `frames_arg`/`frame_item`/`i32_item`, `meta_json`;
    `.crap-baseline.json` refreshed
- `tests/test_data_edge.py`: `dict` converters for both sections (no preparer/SEED_SET row needed)
- napi / WASM / JNI / .NET harnesses: meta + video callers; `UNCOMPARED` gains
    `gen_meta_code_v0: name, meta, metahash` (these bindings return a bare ISCC string)
- Go / Ruby / Swift / Kotlin harnesses: meta + video callers comparing every result key
- `crates/iscc-ffi/tests/test_iscc.c`, `packages/cpp/tests/test_iscc.cpp`: check functions,
    `RUN_EDGE_SECTION` lines, one negative row each for meta (mismatch) and video (ragged expected
    ok)
- `scripts/gen_data_edge_vectors.py`: `iscc_edge_frames` (`const int32_t *const *frames`,
    `const uintptr_t *lens`, `count`), meta/video row structs, `int32_array` (20 per line, `{0}`
    sentinel for an empty frame), `frame_arrays`, `require_int32*`, `meta_text`
    (`json.dumps(ensure_ascii=False)`, then octal-escaped). Header regenerated
- `tests/test_gen_data_edge_vectors.py`: 6 tests (frame/sentinel rendering, wrapping, video rows
    incl. a ragged one, meta JSON text, int32 range and non-object meta fail closed)

**Verification:**

- `cargo test -p iscc-lib --test test_data_edge`: 7 passed (default, `--no-default-features`,
    `--features text-processing --no-default-features`, `--all-features`); clippy `--tests` clean
    with and without default features
- `cargo llvm-cov … && cargo crap --baseline .crap-baseline.json --fail-regression --fail-above`:
    exit 0, 0 regressed after the baseline refresh. `call_meta` scores 30.0 (cc 5, at the threshold,
    passes); `meta_arg` 20.0
- pytest `test_data_edge.py test_gen_data_edge_vectors.py test_vendored_fixtures.py`: 141 passed
    (the video ragged vector is "differs from iscc-core" via `diverges`; meta agrees with iscc-core)
- Node `npm test` after `napi build`: 182 pass / 0 fail. `wasm-pack test --node` data_edge: 8 passed
- `mvn clean test` (after `cargo build -p iscc-jni`): exit 0, DataEdgeTest XML 8 tests 0 failures
- `CGO_ENABLED=0 go test ./...` + `go vet` + `gofmt -l`: clean
- .NET: 153 passed. Ruby `bundle exec rake test`: 165 runs, 0 failures
- Swift (Linux recipe, `/tmp/swifttc`): 30 tests, 0 failures. Kotlin `./gradlew test --rerun-tasks`:
    DataEdgeTest XML `tests="6" failures="0" errors="0"`
- C (`gcc -Wall -Wextra -std=c99`, as ci.yml): 104 passed, rows of both sections PASS. C++ CMake
    with `SANITIZE_ADDRESS=ON` (cmake via `uvx cmake`, none in the container): 112 passed
- Mutation check on every surface (`/tmp` script: meta iscc + meta data URL, video iscc, ragged
    `error` → `outputs`): all three vectors red on each of the 12 surfaces, then restored. Meta-only
    mutation reds `field meta` on Rust, Go, Swift
- Regeneration is a byte no-op (`cmp` against the tracked render); the python3 count assertion exits
    0; `mise run check` exits 0; `prek --hook-stage pre-push` on the generator, its test and the
    Rust harness exits 0

**Next:** Both `normal` `[human]` issues ("No shared vector pins raw `&`, `<`, `>`, U+2028 and
U+2029 in canonical `meta` JSON", "Ragged video frame signatures are rejected by the core but no
shared vector pins it") can close. Then the remaining `normal` `[audit]` issues, the C FFI
`(NULL, 0)` behaviour change first.

**Notes:**

- The C/C++ `meta` field carries the JSON text from `json.dumps(meta, ensure_ascii=False)`, so the C
    test passes raw U+2028/U+2029 bytes into the core; the other harnesses pass whatever their
    serializer writes (Gson/System.Text.Json raw text, `JSON.stringify`, `JSON.generate`). Every
    surface agrees, which confirms the core parses before it canonicalizes
- `call_meta` in the Rust harness sits at cc 5 = CRAP 30.0, at the `--fail-above` limit. A sixth `?`
    there would red CI
- C/C++ test: an empty description is passed as `NULL` / `std::nullopt`, matching each surface's
    `data.json` conformance test
- No surface disagreed with a vector; no `except` entries, no core or binding changes
