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

## Codebase Landmarks

- `crates/` — 6 crates: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni
- `packages/go/` — pure Go module (codec.go, utils.go, cdc.go, minhash.go, simhash.go, dct.go,
    wtahash.go + tests); WASM bridge (iscc.go) still present during transition
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

- **All 6 bindings**: 30/30 Tier 1 symbols (Python, Node.js, WASM, C FFI, Java, Go/wazero)
- **Go pure rewrite**: ~4/5 complete — all 7 algorithm modules done:
    - codec.go (570L, 47T), utils.go (130L, 21T), cdc.go (129L, 15T), minhash.go (205L, 8T),
        simhash.go (86L, 14T), dct.go (52L, 10T), wtahash.go (92L, 9T)
    - 124 total pure Go test functions (excl. 46 WASM bridge tests in iscc_test.go)
- **Remaining in step 5**: 9 `gen_*_v0` functions, DataHasher/InstanceHasher, conformance_selftest,
    then cleanup (remove iscc.go, iscc_ffi.wasm, wazero dep, restore 256KB large-file threshold)
- **zeebo/blake3 not yet added**: Needed for gen_data/instance_code_v0. Not in go.mod yet
- **check-added-large-files**: threshold is 1024KB (must restore to 256KB after Go rewrite cleanup)
- **assessed-at**: bdbc92f (2026-02-27)

## Gotchas

- Go target requires pure Go (no WASM, no wazero, no binary artifacts). Key deps:
    `github.com/zeebo/blake3`, `golang.org/x/text/unicode/norm`
- WASM constant name gotcha: `#[wasm_bindgen(js_name = "META_TRIM_NAME")]` exports uppercase, NOT
    the Rust function name
- `state.md` section order must include both Go Bindings and Per-Crate READMEs sections
- Python ruff format check can fail in CI even if local `mise run check` passes (CI uses global
    `uv run ruff format --check`, pre-commit may only check staged files)
- `dct.go`: `algDct` is unexported (matches Rust `pub(crate)`). `AlgWtahash` is exported. Go has no
    const arrays so `wtaVideoIdPermutations` is `var`
