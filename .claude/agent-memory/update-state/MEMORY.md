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
- **New symbol existence check**:
    `grep -r "gen_sum_code\|META_TRIM_META\|SumCodeResult" crates/ packages/ 2>/dev/null | grep -v "target\|\.lock\|\.md"`
- **Tier 1 pub fns in Rust core**:
    `grep -r "pub fn gen_\|pub const META\|pub const IO\|pub const TEXT" crates/iscc-lib/src/`
- **Go test count**: `grep -r "^func Test" packages/go/ --include="*_test.go" | wc -l`
- **Go gen functions**: `grep "^func Gen" packages/go/code_*.go`
- **Doc nav check**: `grep -A 15 "Reference" zensical.toml`
- **llms.txt page count**: `grep -c "^\-" docs/llms.txt`
- **C FFI extern count**: `grep -c "#\[unsafe(no_mangle)\]" crates/iscc-ffi/src/lib.rs`
- **Howto Sum-Code check**:
    `grep -n "### Sum-Code\|gen_sum_code_v0\|GenSumCodeV0\|genSumCodeV0" docs/howto/*.md`
- **Benchmark functions**:
    `grep -n "^fn bench_\|criterion_group" crates/iscc-lib/benches/benchmarks.rs`

## Codebase Landmarks

- `crates/` — 6 crates: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni
- `packages/go/` — pure Go module (no WASM bridge, no binary artifacts)
- `.github/workflows/ci.yml` — jobs: version-check, Rust, python-test (matrix 3.10+3.14), python
    (gate), Node.js, WASM, C FFI, Java, Go, Bench
- `docs/howto/` — 7 files: rust.md, python.md, nodejs.md, wasm.md, go.md, java.md, **c-cpp.md** ✅
    (resolved issue #22); `crates/iscc-ffi/examples/` has `iscc_sum.c` + `CMakeLists.txt` ✅
- `scripts/version_sync.py` — syncs workspace version across Cargo.toml, package.json, pom.xml
- `packages/go/codec.go` — codec enums, varnibble, header, base32/64, JsonToDataUrl,
    EncodeComponent, IsccDecompose, IsccDecode, **5 constants** (MetaTrimName, MetaTrimDescription,
    MetaTrimMeta, IoReadSize, TextNgramSize)
- `docs/c-ffi-api.md` — C FFI API reference (fully updated with iscc_gen_sum_code_v0)
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` — Java class (subpath:
    `iscc_lib/`); has META_TRIM_META as `public static final int`
- `crates/iscc-ffi/src/lib.rs` line 3 — module docstring says "10 `gen_*_v0` functions"
- `crates/iscc-lib/benches/benchmarks.rs` — 277 lines; docstring says "all 10 gen\_\*\_v0"; has
    `bench_sum_code` (64KB+1MB using NamedTempFile); `criterion_group!` lists 12 benches

## Recurring Patterns

- **Incremental review**: compare `assessed-at` hash vs HEAD `--stat` first, then re-verify only
    affected sections. Always carry forward sections where no relevant files changed
- **CI has matrix jobs**: python-test runs as Python 3.10 + Python 3.14 (separate records); gate job
    `python` checks both pass. Count distinct job definitions, not run records.
- `gh run list` does NOT need `--repo` when running from within the workspace; but `--json` fields
    are needed to avoid GraphQL deprecation error
- **Verify claims independently**: review agents can make incorrect claims. Always grep for each
    missing symbol rather than trusting handoff verdict counts
- **Target may change**: always re-read target.md diff when doing incremental review; symbol counts
    and spec requirements can increase

## Current State (assessed-at: ba1a030)

- **IN_PROGRESS**: C FFI DX — 1 of 6 criteria remain unmet (pre-built FFI tarballs); 3 open issues
- **Issues open**: #16 (feature flags, low priority), #21 (units in gen_sum_code_v0, normal), #25
    (FFI release tarballs, normal)
- **Issue #22 RESOLVED**: `docs/howto/c-cpp.md` (433 lines) created; linked in navigation via
    `"C / C++" = "howto/c-cpp.md"` entry in zensical.toml; all 13 spec sections present
- **docs/howto/c-cpp.md sections**: Building from source, CMake `find_library` + pkg-config,
    ISCC-SUM quick start, Streaming (dual-hasher), Composing ISCC-SUM manually, Error handling,
    Memory management (full table), Static vs dynamic linking, Cross-compilation, C++ RAII wrapper,
    Conformance verification
- **release.yml**: NO build-ffi/publish-ffi jobs; NO ffi workflow_dispatch input (issue #25)
- **CI latest**: Run 22589681430 — all 11 CI jobs SUCCESS ✅
- **Documentation section**: now fully met (17 pages, all howto guides complete)

## Go Package Tier 1 Coverage (32/32 — COMPLETE)

All 32 symbols: 10 gen functions (including GenSumCodeV0), ConformanceSelftest, DataHasher,
InstanceHasher, 4 text utilities, SlidingWindow, AlgMinhash256, AlgCdcChunks, AlgSimhash,
SoftHashVideoV0, EncodeBase64, EncodeComponent, IsccDecode, IsccDecompose, JsonToDataUrl, **5
constants** (MetaTrimName, MetaTrimDescription, MetaTrimMeta, IoReadSize, TextNgramSize).

## Gotchas

- Go target requires pure Go (no WASM, no wazero, no binary artifacts)
- WASM constant name gotcha: `#[wasm_bindgen(js_name = "META_TRIM_NAME")]` exports uppercase
- `state.md` section order must include Go Bindings, README, Per-Crate READMEs sections
- Python ruff format check can fail in CI even if local `mise run check` passes
- `dct.go`: `algDct` is unexported. `AlgWtahash` is exported. Go has no const arrays so
    `wtaVideoIdPermutations` is `var`
- **JCS gotcha**: Go `json.Marshal` passes current vectors. If future vectors have floats, a proper
    RFC 8785 JCS library may be needed
- **DataHasher/InstanceHasher API**: Go uses `Push([]byte)` + `Finalize(bits)` pattern
- **GenIsccCodeV0 key details**: `encode_units` produces a bitfield; `wide` param always false in
    test vectors; SubType from content code's SubType (or NONE if absent); 5 conformance vectors
- **gen_sum_code_v0 WASM**: path-based I/O doesn't exist in browser WASM context — accepts
    Uint8Array/&[u8] instead; WASM and Go both solve this differently from Rust/Python/Node.js/C FFI
- **META_TRIM_META validation**: pre-decode fast check on Data-URL string length AND post-decode
    payload check; both needed in gen_meta_code_v0
- **Java META_TRIM_META**: added as compile-time `public static final int` (no JNI function needed)
- **C FFI IsccSumCodeResult**: struct-return pattern (not output-pointer); matches IsccDecodeResult
    pattern precisely; partial allocation failure handled (free iscc before returning null)
- **9 vs 10 distinction**: data.json has 9 conformance sections (no gen_sum_code_v0 vectors);
    iscc-lib has 10 gen functions. Test/conformance docstrings correctly say "9"; user-facing docs
    and benchmarks file say "10"
- **gen_sum_code_v0 benchmark**: uses NamedTempFile (tempfile crate); temp files created outside
    criterion closure so file setup is not measured; cleanup via Drop
