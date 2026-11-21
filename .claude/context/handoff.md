## 2026-11-21 — Review of: Go `GenMetaCodeV0` enforces the 128,000-byte meta limit, pinned by a shared vector

**Verdict:** PASS_WITH_NOTES

**Summary:** The Go port now rejects an oversized meta payload exactly as the Rust core does: the
same two checks (raw string over 170,922 bytes, decoded payload over 128,000 bytes), in the same
order, with the core's wording. The shared vector `test_0004_meta_payload_over_limit` is in all 5
copies and the regenerated C header, and every harness that was run rejects it. One note: the vector
puts a 128 KB string literal into the generated C header, which MSVC cannot compile (new `normal`
issue). The step followed next.md here, which ruled out a generator change.

**Verification:**

- [x] `CGO_ENABLED=0 go test ./...` passes in `packages/go` (data-edge suite runs the new vector)
- [x] `GOARCH=386 go test ./...` passes; `go vet ./...` clean (also under `GOARCH=386`); `gofmt -l`
    empty
- [x] `grep -n 'MetaTrimMeta' packages/go/code_meta.go` — line 72/75 (post-decode check) and line
    143 (`metaPreDecodeLimit`, used by the pre-decode check at line 55)
- [x] `cargo test -p iscc-lib` passes (315 + 44 + 28 + … passed, 0 failed)
- [x] `uv run pytest tests/test_data_edge.py tests/test_vendored_fixtures.py tests/test_gen_data_edge_vectors.py`
    — 165 passed; the 5 copies have one md5; the generator re-run is a no-op
- [x] C FFI test program, built as the `c-ffi` CI job builds it: 148 passed, 0 failed, the new
    vector PASS. C++ (cmake Debug + ASAN, as the `cpp` job): 119 passed, 0 failed, the new vector
    PASS
- [x] `mise run check` exits 0; `cargo clippy --workspace --all-targets -- -D warnings` clean

Beyond next.md's list (3 probes):

- iscc-core 1.5.0 on Python 3.10 (compiled wheel) and 3.15 (pure Python) both raise `ValueError` on
    the vector (canonical length 128,001) and both return a code one byte below it
- The advance agent did not run the other Rust-backed harnesses. Three of them, run here on the new
    vector: .NET 155 passed, JNI (`mvn test`) 122 passed, Kotlin `gradlew cleanTest test` green.
    Node, WASM, Ruby and Swift are left to CI
- Sibling call sites: Go `JsonToDataUrl` also calls `parseMetaJSON`, but the core's
    `json_to_data_url` has no size check either, so no check is owed there

**Issues found:**

- Filed `normal` `[review]`: "Generated `data_edge_vectors.h` holds a 128 KB string literal that
    MSVC cannot compile". MSVC limits a string literal to 65,535 bytes (C2026/C1091). The C and C++
    test programs were portable before this step. Not measured (no MSVC here or in CI)
- Deleted the resolved issue "Go `GenMetaCodeV0` never enforces the 128,000-byte meta payload limit"
- Nit, not filed: the Go tests pin the pre-decode bound only for the JSON form. The check runs
    before the form is looked at, so the Data-URL form is covered by the same line

**Codex review:** One finding (P2), the same MSVC literal limit as above: emit an oversized value as
a byte array or assemble it at runtime, then regenerate the header. Accepted and filed as the issue
above. Codex confirmed that the Go checks match Rust.

**Next:** Two small candidates, define-next's choice:

1. The new MSVC issue: `scripts/gen_data_edge_vectors.py` writes a string over the limit as a
    `static const char` array (or the like), plus a generator test that fails on any string literal
    over 65,535 bytes. One script, the regenerated header, one test file. It is a leftover of this
    step and a later large vector would add to it
2. The `[audit]` issue "`@iscc/wasm` docs show a Node.js setup the published package cannot run"
    (docs only; probe a packed `wasm-pack build --target web` release build first)

**Notes:**

- `cmake` is not on PATH in the devcontainer: `uv run --with cmake cmake …` works
- `data-edge.json` grew from about 16 KB to 144 KB per copy (5 copies), and the header to 147 KB;
    all stay under the 256 KB added-file limit. A second vector of this size would not
- No Rust code changed: iai and CRAP baselines are untouched, no API change
- Unpushed commits besides the advance commit touch only context and agent-memory files (audit 280,
    update-state, define-next)
