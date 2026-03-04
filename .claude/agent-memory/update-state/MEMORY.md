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

- `crates/` — 7 crates: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni, **iscc-rb**
    (32/32 symbols — COMPLETE)
- `.claude/context/specs/` — per-binding spec files (ruby, go, java, nodejs, wasm, cpp, dotnet,
    swift, kotlin, rust-core, c-ffi-dx, documentation, ci-cd)
- `packages/go/` — pure Go module (no WASM bridge, no binary artifacts)
- `.github/workflows/ci.yml` — jobs: version-check, Rust, python-test (matrix 3.10+3.14), python
    (gate), Node.js, WASM, C FFI, Java, Go, Bench, **Ruby** (12 total)
- `docs/howto/` — **8 files**: rust.md, python.md, nodejs.md, wasm.md, go.md, java.md, c-cpp.md,
    **ruby.md** (422 lines) ✅; `crates/iscc-ffi/examples/` has `iscc_sum.c` + `CMakeLists.txt` ✅
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

## Current State (assessed-at: 2011e23)

- **IN_PROGRESS**: all 12 CI jobs green (run 22665835771); **0 normal-priority gaps remain**
- **Iter 4 (CID round 4) changes** (6404294, 8ada155, 2011e23 since ad7400d):
    - `release.yml`: 6 smoke test jobs added — `test-wheels`, `test-napi`, `test-wasm`, `test-jni`,
        `test-ffi`, `test-gem`; each gates its publish job ✅
    - `--features conformance` added to `build-wasm` so `conformance_selftest` is exported in WASM
    - Release smoke tests issue resolved and deleted from issues.md ✅
- **Only low-priority gaps remain**: C#, C++, Swift, Kotlin bindings; language logos — CID skips
- **CI (run 22665835771)**: ALL SUCCESS — 12 jobs
- **release.yml**: 6 checkboxes + 6 smoke test jobs: test-wheels/napi/wasm/jni/ffi/gem ✅
- **Magnus version**: 0.7.1 (not 0.8) — devcontainer Ruby is 3.1.2; Magnus 0.8 requires Ruby 3.2+
- **Test counts (Rust)**: 316 (default features)
- **docs/**: 8 howto files; `docs/ruby-api.md` ✅; `docs/c-ffi-api.md` ✅
- **packages/**: only `go/`; `dotnet/`, `cpp/`, `swift/`, `kotlin/` dirs do NOT exist yet
- **Recommended next action**: PR from `develop` → `main` for stable release (human-directed)

## iscc-core v1.3.0 Conformance (FULLY RESOLVED — all bindings)

- 4 new test vectors vendored: test_0017–test_0020 in both `crates/iscc-lib/tests/data.json` and
    `packages/go/testdata/data.json` (50 total vectors)
- `data.json` has top-level `_metadata` object — Go uses `parseConformanceData()` to skip it; Rust
    `serde_json` silently ignores unknown fields
- Rust lib.rs assertion: 20; WASM conformance.rs line 66: 20 ✅; Go all 9 test files updated ✅

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
- **DataHasher/InstanceHasher API (Go)**: `Push([]byte)` + `Finalize(bits)` pattern
- **DataHasher/InstanceHasher API (Ruby)**: `RefCell<Option<inner>>` for interior mutability (Magnus
    `&self`); Ruby wrapper reopens native class, adds `update(data)` (chaining) +
    `finalize(bits: 64)`
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
- **WASM count assertions**: `crates/iscc-wasm/tests/conformance.rs` has per-function
    `assert_eq!(tested, N, ...)` guards. When data.json gains new vectors, BOTH lib.rs AND the WASM
    conformance test must be updated. Review agents may miss this (check grep result:
    `grep -n "assert_eq.*tested" crates/iscc-wasm/tests/conformance.rs`)
- **Ruby JSON sort_keys no-op**: `JSON.generate(hash, sort_keys: true)` silently ignores `sort_keys`
    in Ruby's stdlib json gem. Use `JSON.generate(hash.sort.to_h)` instead for deterministic key
    ordering (required for JCS-compatible meta serialization)
- **alg_cdc_chunks API**: public fn returns `IsccResult<Vec<&[u8]>>` (validates
    `avg_chunk_size < 2`); internal callers use `alg_cdc_chunks_unchecked` (no validation, always
    valid constant). All 6 binding crates (Python, NAPI, WASM, FFI, JNI, Ruby) handle the Result. Go
    `AlgCdcChunks` also validates `avgChunkSize < 2` (returns `error`; internal caller uses
    `algCdcChunksUnchecked`) — gap now closed.
