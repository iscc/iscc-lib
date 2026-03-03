## 2026-03-03 — Vendor iscc-core v1.3.0 data.json and fix Go conformance loader

**Done:** Vendored iscc-core v1.3.0 conformance test vectors (50 total, up from 46 — 4 new Meta-Code
tests) to both Rust and Go test data locations. Fixed Go conformance loader (`conformance.go`) to
tolerate the new `_metadata` top-level key via two-pass JSON parsing. Also fixed all 9 Go
per-function test files that had the same parsing pattern, and updated the Rust meta conformance
test assertion from 16 to 20 expected vectors.

**Files changed:**

- `crates/iscc-lib/tests/data.json`: Replaced with v1.3.0 vectors (50 tests, `_metadata` header)
- `packages/go/testdata/data.json`: Same replacement
- `packages/go/conformance.go`: Added `parseConformanceData()` helper using
    `map[string]json.RawMessage` intermediate parsing to skip `_`-prefixed keys; updated
    `ConformanceSelftest` to use it; updated doc comment from 46 to 50 vectors
- `crates/iscc-lib/src/lib.rs`: Updated meta conformance test assertion from 16 to 20 expected tests
- `packages/go/code_meta_test.go`: Replaced inline struct + `json.Unmarshal` with
    `parseConformanceData()`
- `packages/go/code_content_text_test.go`: Same fix
- `packages/go/code_content_image_test.go`: Same fix
- `packages/go/code_content_audio_test.go`: Same fix
- `packages/go/code_content_video_test.go`: Same fix
- `packages/go/code_content_mixed_test.go`: Same fix
- `packages/go/code_data_test.go`: Same fix
- `packages/go/code_instance_test.go`: Same fix
- `packages/go/code_iscc_test.go`: Same fix

**Verification:**

- `cargo test -p iscc-lib`: 314 tests pass (258 lib + 31 integration + 24 text utils + 1 doctest).
    The 4 new meta vectors are auto-discovered within the existing
    `test_gen_meta_code_v0_conformance` test function (iterates 20 vectors now)
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cd packages/go && go test ./...`: all pass (155 tests including 4 new meta vectors)
- `cd packages/go && go vet ./...`: clean
- `grep -c 'test_0017\|test_0018\|test_0019\|test_0020' crates/iscc-lib/tests/data.json`: returns 4
- `grep '_metadata' crates/iscc-lib/tests/data.json`: found
- `mise run check`: all 14 pre-commit hooks pass

**Next:** The critical iscc-core v1.3.0 conformance issue is now resolved at the data foundation
level. The Rust core and Go implementation both pass all 50 vectors. Next steps could include:
verifying that binding conformance tests (Python, Node.js, WASM, Java, C FFI) also pass with the new
data.json (they should — they access sections by name), or addressing other items from issues.md.

**Notes:**

- The Go per-function test files (`code_*_test.go`) had the same `_metadata` parsing issue as
    `conformance.go` — they used `map[string]map[string]struct{...}` to parse data.json directly via
    `os.ReadFile`. This was not called out in next.md's scope but was essential to fix — Go tests
    would fail without it. The fix reuses the new `parseConformanceData()` helper from
    `conformance.go`, which is DRY and idiomatic
- Rust conformance.rs needed no changes (uses `serde_json::Value` which ignores unknown keys), but
    `lib.rs` had a hardcoded assertion `tested == 16` that needed updating to 20
- The `end-of-file-fixer` hook added trailing newlines to both data.json files during
    `mise run   format` — the upstream v1.3.0 data.json was missing a final newline
