## 2026-11-21 — Review of: `gen_meta_code_v0` and `gen_video_code_v0` sections in `data-edge.json`, on all 12 surfaces

**Verdict:** PASS_WITH_NOTES

**Summary:** Both sections are in `data-edge.json` and its 4 copies, and all 12 surfaces run the
three vectors green. The fan-out is honest: 12 per-surface commits plus one for the fixture and the
generator, no core or binding change, no `except` entry, no lint suppression. Both `normal`
`[human]` issues are closed. Codex found two gaps in the new harness code that would reject a valid
future meta vector; they are filed as one `normal` issue.

**Verification:**

- [x] `cargo test -p iscc-lib --test test_data_edge` — 7 passed (default, `--no-default-features`,
    `--no-default-features --features text-processing`, `--all-features`); pytest on the three files
    141 passed; `cargo llvm-cov` + `cargo crap … --fail-regression --fail-above` exit 0 (0
    regressed, 157 unchanged)
- [x] Node `npm test` after a fresh `napi build` 182 pass; `wasm-pack test --node` data_edge 8
    passed; `mvn clean test` 116 run, 0 failures (DataEdgeTest 8); Go `go test ./...`, `go vet`,
    `gofmt -l` clean; .NET 153 passed; Ruby 165 runs, 0 failures
- [x] Swift (Linux recipe) 30 tests, 0 failures; Kotlin `./gradlew test --rerun-tasks`, JUnit XML of
    `DataEdgeTest`: `tests="6" failures="0" errors="0"`
- [x] C test (gcc as in `ci.yml`) 104 passed, C++ CMake build with `SANITIZE_ADDRESS=ON` 112 passed;
    each prints PASS for the three new rows and the two new negatives
- [x] Generator rerun leaves `data_edge_vectors.h` unchanged; the count assertion exits 0
- [x] `mise run check` exits 0; `cargo clippy --workspace --all-targets -- -D warnings` clean

Probes beyond next.md (2 of 3):

- The meta vector's four outputs match `iscc_lib` and iscc-core 1.5.0. Escaping any single one of
    `&`, `<`, `>`, U+2028, U+2029 in the canonical bytes changes the `iscc` (measured through a
    data-URL `meta`), so the four surfaces that return only the ISCC string detect the defect too
- A mutation of only the expected `meta` data URL reds Python, Ruby and Kotlin with "field meta"
    (the handoff claimed it for Rust, Go and Swift)

**Issues found:**

- Filed `normal`: "Four data-edge harnesses and a generator test reject valid `gen_meta_code_v0`
    vectors". The napi, WASM, JNI and .NET uncompared sets lack `description`, and
    `test_meta_row_renders_object_as_json_text` requires every `meta` input to be an object. Today's
    fixture is green; a later vector would need a harness edit, against "A vector costs one edit"
- Fixed in this review (comment only): the C negative row's comment said "the same object written
    with escaped `&` and `<`"; the row holds a different object, as the C++ comment says
- Not filed: the Python ragged-frame error text ("frame_sigs[1] has length 381, expected 380")
    differs from the core's. Harnesses never match error text, and the binding's own check is out of
    scope by next.md

**Codex review:** Two P2 findings, both confirmed by reading the code and filed as the issue above.
Codex ran the suites and reported all 12 surfaces green on the committed fixture.

**Next:** The new `normal` `[review]` issue is the smallest open item: five test files, no library
code, and the four affected suites plus pytest prove it. After that the `normal` `[audit]` issues,
the C FFI `(NULL, 0)` behaviour change first and alone.

**Notes:**

- The Swift harness ran only on Linux here. The `swift` job on `macos-15` is the first Darwin run of
    the two sections; check it in the CI run of this push
- `call_meta` in `crates/iscc-lib/tests/test_data_edge.rs` scores CRAP 30.0, exactly the
    `--fail-above` limit. One more `?` or branch in it reds CI: split it instead
- A data-edge `meta` input is a JSON object, because iscc-core reads a string `meta` as a data URL.
    C and C++ receive it as `json.dumps(…, ensure_ascii=False)` text from the generator
- issues.md is at 295 lines and learnings.md at 200: a new entry needs an equal cut
