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
    `grep -n "encode_component\|META_TRIM\|IO_READ\|TEXT_NGRAM\|iscc_decode\|json_to_data_url\|JsonToDataUrl" crates/iscc-py/src/lib.rs crates/iscc-napi/src/lib.rs crates/iscc-wasm/src/lib.rs crates/iscc-ffi/src/lib.rs crates/iscc-jni/src/lib.rs packages/go/*.go`
- **Go test count**: `grep -r "^func Test" packages/go/ --include="*_test.go" | wc -l`
- **Go gen functions**: `grep "^func Gen" packages/go/code_*.go`
- **Go exported funcs**: `grep -h "^func " packages/go/*.go | grep -v "_test.go" | sort`

## Codebase Landmarks

- `crates/` — 6 crates: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni
- `packages/go/` — pure Go module (no WASM bridge, no binary artifacts)
- `.github/workflows/ci.yml` — 7 jobs: Rust, Python, Node.js, WASM, C FFI, Java, Go
- `docs/howto/` — 6 files: rust.md, python.md, nodejs.md, wasm.md, go.md, java.md (all complete)
- `scripts/version_sync.py` — syncs workspace version across Cargo.toml, package.json, pom.xml
- `packages/go/codec.go` — codec enums, varnibble, header, base32/64, JsonToDataUrl,
    EncodeComponent, IsccDecompose, IsccDecode
- `packages/go/code_meta.go` — `parseMetaJSON`, `jsonHasContext`, `buildMetaDataURL` helpers

## Recurring Patterns

- **Incremental review**: compare `assessed-at` hash vs HEAD `--stat` first, then re-verify only
    affected sections. Always carry forward sections where no relevant files changed
- **CI has 7 jobs**: Rust, Python, Node.js, WASM, C FFI, Java, Go. All 7 must pass
- `gh run list` does NOT need `--repo` when running from within the workspace; but `--json` fields
    are needed to avoid GraphQL deprecation error
- **Verify claims independently**: review agents can make incorrect claims. Always grep for each
    missing symbol rather than trusting handoff verdict counts

## Current State (assessed-at: 37100bf)

- **All 7 bindings**: 30/30 Tier 1 symbols complete (Rust, Python, Node.js, WASM, C FFI, Java, Go)
- **Go CI**: Clean — checkout → setup-go → `go test` → `go vet` (no WASM vestiges since 37100bf)
- **Go docs** (`docs/howto/go.md`): Pure Go — no Runtime/wazero, package-level function examples
- **Go README** (`packages/go/README.md`): Pure Go — no wazero, Push/Finalize streaming API
- **CI latest**: Run 22511013392 — all 7 jobs SUCCESS (develop branch, 2026-02-28)
- **Publishing**: 0.0.2 not published to PyPI, npm, or Maven Central
- **Remaining gaps**: Benchmark CI integration; Maven Central publishing; npm/PyPI 0.0.2 release
- **Minor cosmetic**: 5 Go test files have vestigial "do NOT require the WASM binary" comments

## Go Package Tier 1 Coverage (30/30 — COMPLETE)

All symbols present: 9 gen functions, ConformanceSelftest, DataHasher, InstanceHasher, 4 text
utilities, SlidingWindow, AlgMinhash256, AlgCdcChunks, AlgSimhash, SoftHashVideoV0, EncodeBase64,
EncodeComponent, IsccDecode, IsccDecompose, JsonToDataUrl, 4 constants (MetaTrimName,
MetaTrimDescription, IoReadSize, TextNgramSize).

## Gotchas

- Go target requires pure Go (no WASM, no wazero, no binary artifacts)
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
- Java Maven Central: requires GPG key, Sonatype OSSRH account, pom.xml signing plugin — not yet
    configured; end-to-end release untested
