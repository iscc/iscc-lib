# Update-State Agent Memory

Codepaths, patterns, and key findings accumulated across CID iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Exploration Shortcuts

- **Java files**: `find crates/iscc-jni -type f | sort`
- **Per-crate READMEs**: `ls crates/*/README.md packages/go/README.md 2>&1`
- **CI jobs in a run**: `gh run view <id> --json jobs --jq '.jobs[] | {name, conclusion}'`
- **Latest CI runs**:
    `gh run list --branch "$(git branch --show-current)" --limit 3 --json status,conclusion,url,databaseId`
- **Incremental diff**: `git diff <assessed-at-hash>..HEAD --stat`
- **Go files**: `ls packages/go/*.go` — check pure Go source files
- **Go in CI**: `grep -n "go\|Go\|golang" .github/workflows/ci.yml`
- **Binding symbol check**:
    `grep -n "encode_component\|META_TRIM\|IO_READ\|TEXT_NGRAM\|iscc_decode\|json_to_data_url" crates/iscc-py/src/lib.rs crates/iscc-napi/src/lib.rs crates/iscc-wasm/src/lib.rs crates/iscc-ffi/src/lib.rs crates/iscc-jni/src/lib.rs packages/go/*.go`
- **Go test count**: `grep -c "^func Test" packages/go/*_test.go`
- **Go gen functions**: `grep "^func Gen" packages/go/code_*.go`

## Codebase Landmarks

- `crates/` — 6 crates: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni
- `packages/go/` — pure Go module (codec.go, utils.go, cdc.go, minhash.go, simhash.go, dct.go,
    wtahash.go, xxh32.go, code_meta.go, code_content_text.go, code_data.go, code_instance.go +
    tests); WASM bridge (iscc.go + iscc_ffi.wasm) still present during transition
- `.github/workflows/ci.yml` — 7 jobs: Rust, Python, Node.js, WASM, C FFI, Java, Go
- `docs/howto/` — 6 files: rust.md, python.md, nodejs.md, wasm.md, go.md, java.md (all complete)
- `scripts/version_sync.py` — syncs workspace version across Cargo.toml, package.json, pom.xml

## Recurring Patterns

- **Incremental review**: compare `assessed-at` hash vs HEAD `--stat` first, then re-verify only
    affected sections. Always carry forward sections where no relevant files changed
- **CI has 7 jobs**: Rust, Python, Node.js, WASM, C FFI, Java, Go. All 7 must pass
- `gh run list` does NOT need `--repo` when running from within the workspace; but `--json` fields
    are needed to avoid GraphQL deprecation error

## Current State

- **All 6 bindings**: 30/30 Tier 1 symbols (Python, Node.js, WASM, C FFI, Java)
- **Go pure rewrite**: step 5 in progress — 6/9 gen functions done:
    - codec.go (570L, 47T), utils.go (130L, 21T), cdc.go (129L, 15T), minhash.go (205L, 8T),
        simhash.go (86L, 14T), dct.go (52L, 10T), wtahash.go (92L, 9T)
    - xxh32.go (81L, 8T): pure Go xxHash32
    - code_meta.go (281L, 1T/16 vectors): GenMetaCodeV0 + MetaCodeResult
    - code_content_text.go (41L, 1T/5 vectors): GenTextCodeV0 + TextCodeResult
    - code_data.go (90L, 1T/4 vectors): GenDataCodeV0 + DataHasher streaming
    - code_instance.go (67L, 1T/3 vectors): GenInstanceCodeV0 + InstanceHasher streaming
    - code_content_image.go (134L, 1T/3 vectors): GenImageCodeV0 + ImageCodeResult (DCT-based)
    - code_content_audio.go (112L, 1T/5 vectors): GenAudioCodeV0 + AudioCodeResult + arraySplit[T]
    - 184 total Go test functions (138 pure Go + 46 WASM bridge tests)
- **`github.com/zeebo/blake3 v0.2.4`** in go.mod — needed for Meta/Data/Instance code
- **`arraySplit[T any]`** generic helper in code_content_audio.go — reusable for MixedCode
- **Remaining in step 5**: GenVideoCodeV0 (DCT+WTA-Hash+SimHash), GenMixedCodeV0
    (decompose+SimHash), GenIsccCodeV0, conformance_selftest, cleanup (remove iscc.go,
    iscc_ffi.wasm, wazero dep, restore 256KB threshold)
- **check-added-large-files**: threshold is 1024KB (must restore to 256KB after cleanup)
- **assessed-at**: 79973c2 (2026-02-27)

## Gotchas

- Go target requires pure Go (no WASM, no wazero, no binary artifacts). Key deps:
    `github.com/zeebo/blake3`, `golang.org/x/text/unicode/norm`
- WASM constant name gotcha: `#[wasm_bindgen(js_name = "META_TRIM_NAME")]` exports uppercase, NOT
    the Rust function name
- `state.md` section order must include Go Bindings and Per-Crate READMEs sections
- Python ruff format check can fail in CI even if local `mise run check` passes
- `dct.go`: `algDct` is unexported (matches Rust `pub(crate)`). `AlgWtahash` is exported. Go has no
    const arrays so `wtaVideoIdPermutations` is `var`
- **JCS gotcha**: Go `json.Marshal` passes current vectors (string-only). If future vectors have
    floats, a proper RFC 8785 JCS library may be needed
- **DataHasher/InstanceHasher API**: Go uses `Push([]byte)` + `Finalize(bits)` pattern (not
    `io.Writer`), matching the Rust streaming interface
