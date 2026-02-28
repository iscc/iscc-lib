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
- **Go exported funcs**: `grep -h "^func " packages/go/*.go | grep -v "_test.go" | sort`

## Codebase Landmarks

- `crates/` — 6 crates: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni
- `packages/go/` — pure Go module (no WASM bridge); WASM bridge fully removed in commit 0cbff37
- `.github/workflows/ci.yml` — 7 jobs: Rust, Python, Node.js, WASM, C FFI, Java, Go
- `docs/howto/` — 6 files: rust.md, python.md, nodejs.md, wasm.md, go.md, java.md (all complete)
- `scripts/version_sync.py` — syncs workspace version across Cargo.toml, package.json, pom.xml

## Recurring Patterns

- **Incremental review**: compare `assessed-at` hash vs HEAD `--stat` first, then re-verify only
    affected sections. Always carry forward sections where no relevant files changed
- **CI has 7 jobs**: Rust, Python, Node.js, WASM, C FFI, Java, Go. All 7 must pass
- `gh run list` does NOT need `--repo` when running from within the workspace; but `--json` fields
    are needed to avoid GraphQL deprecation error
- **Verify claims independently**: review agents can make incorrect claims (e.g., claimed 30/30 Go
    symbols after WASM bridge removal but `JsonToDataUrl` was never ported). Always grep for each
    missing symbol rather than trusting handoff verdict counts

## Current State (assessed-at: 0cbff37)

- **All bindings except Go**: 30/30 Tier 1 symbols (Python, Node.js, WASM, C FFI, Java, Rust)
- **Go bindings**: 29/30 Tier 1 symbols — missing `JsonToDataUrl`
    - WASM bridge FULLY REMOVED: `iscc.go`, `iscc_ffi.wasm`, `iscc_test.go` gone
    - `wazero v1.11.0` removed from `go.mod`; `--maxkb=256` restored
    - `JsonToDataUrl` was only in the WASM bridge — never ported to pure Go
    - 142 test functions across 18 test files (pure Go only)
    - go.mod: zeebo/blake3, golang.org/x/text, klauspost/cpuid (indirect)
- **CI latest**: Run 22509596765 — all 7 jobs SUCCESS (develop branch, 2026-02-28)
- **Publishing**: 0.0.2 not published to PyPI, npm, or Maven Central

## Go Package Tier 1 Coverage (29/30)

Missing: `JsonToDataUrl` (Rust: `json_to_data_url` in `crates/iscc-lib/src/lib.rs:260`) Present: all
9 gen functions, ConformanceSelftest, DataHasher, InstanceHasher, 4 text utilities, SlidingWindow,
AlgMinhash256, AlgCdcChunks, AlgSimhash, SoftHashVideoV0, EncodeBase64, EncodeComponent, IsccDecode,
IsccDecompose, 4 constants (MetaTrimName, MetaTrimDescription, IoReadSize, TextNgramSize)

## Gotchas

- Go target requires pure Go (no WASM, no wazero, no binary artifacts)
- `JsonToDataUrl` was only implemented in deleted `iscc.go` WASM bridge — not yet in pure Go
- WASM constant name gotcha: `#[wasm_bindgen(js_name = "META_TRIM_NAME")]` exports uppercase
- `state.md` section order must include Go Bindings and Per-Crate READMEs sections
- Python ruff format check can fail in CI even if local `mise run check` passes
- `dct.go`: `algDct` is unexported. `AlgWtahash` is exported. Go has no const arrays so
    `wtaVideoIdPermutations` is `var`
- **JCS gotcha**: Go `json.Marshal` passes current vectors. If future vectors have floats, a proper
    RFC 8785 JCS library may be needed
- **DataHasher/InstanceHasher API**: Go uses `Push([]byte)` + `Finalize(bits)` pattern
- **GenIsccCodeV0 key details**: `encode_units` produces a bitfield; `wide` param always false in
    test vectors; SubType from content code's SubType (or NONE if absent); 5 conformance vectors
- **Go README lists JsonToDataUrl** in API table — misleading since not implemented; README needs
    update after implementation
