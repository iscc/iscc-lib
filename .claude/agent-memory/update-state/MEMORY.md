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

## Codebase Landmarks

- `crates/` — 6 crates: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni
- `packages/go/` — pure Go module (no WASM bridge, no binary artifacts)
- `.github/workflows/ci.yml` — jobs: version-check, Rust, python-test (matrix 3.10+3.14), python
    (gate), Node.js, WASM, C FFI, Java, Go, Bench
- `docs/howto/` — 6 files: rust.md, python.md, nodejs.md, wasm.md, go.md, java.md (all complete)
- `scripts/version_sync.py` — syncs workspace version across Cargo.toml, package.json, pom.xml
- `packages/go/codec.go` — codec enums, varnibble, header, base32/64, JsonToDataUrl,
    EncodeComponent, IsccDecompose, IsccDecode, **5 constants** (MetaTrimName, MetaTrimDescription,
    MetaTrimMeta, IoReadSize, TextNgramSize)
- `docs/c-ffi-api.md` — C FFI API reference (694 lines); 45 extern "C" symbols
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` — Java class (subpath:
    `iscc_lib/`); now has META_TRIM_META as `public static final int`

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

## Current State (assessed-at: d27f5f6)

- **Target**: 32 Tier 1 symbols — Rust core NOW AT 32/32 ✅; all 6 bindings at 31/32
- **Iteration 5 completed**: `gen_sum_code_v0` + `SumCodeResult` implemented in Rust core
- **Issues**: #15 (gen_sum_code_v0 propagation to 6 bindings, in-progress), #16 (feature flags)
- **gen_sum_code_v0**: Rust ✅ Python ❌ Node.js ❌ WASM ❌ C FFI ❌ Java ❌ Go ❌
- **SumCodeResult**: Rust ✅ (types.rs, exported via `pub use types::*`); all bindings ❌
- **Rust core tests**: 310 (256 unit + 31 streaming + 22 utils + 1 doctest)
- **gen_sum_code_v0 impl details**: line 967 lib.rs; `pub fn`; uses DataHasher + InstanceHasher
    single-pass; calls gen_iscc_code_v0 internally; `SumCodeResult.units` field deferred
- **v0.0.3 released**: tags `v0.0.3` and `packages/go/v0.0.3`; all registries
- **CI latest**: Run 22550764467 — all 11 job records SUCCESS (all 5 recent runs green)
- **Next priority**: Propagate gen_sum_code_v0 + SumCodeResult to all 6 bindings (start Python)

## Go Package Tier 1 Coverage (31/32)

All 31 symbols present: 9 gen functions, ConformanceSelftest, DataHasher, InstanceHasher, 4 text
utilities, SlidingWindow, AlgMinhash256, AlgCdcChunks, AlgSimhash, SoftHashVideoV0, EncodeBase64,
EncodeComponent, IsccDecode, IsccDecompose, JsonToDataUrl, **5 constants** (MetaTrimName,
MetaTrimDescription, MetaTrimMeta, IoReadSize, TextNgramSize). Missing: GenSumCodeV0 +
SumCodeResult.

**Note**: MetaTrimMeta declared in codec.go but has no explicit test assertion in Go test files.
Other constants also have no explicit value tests. This is acceptable since CI passes 147 tests.

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
- **gen_sum_code_v0 WASM**: path-based I/O doesn't exist in browser WASM context — needs design
    decision (accept Uint8Array? throw? only Node.js WASM? skip?)
- **META_TRIM_META validation**: pre-decode fast check on Data-URL string length AND post-decode
    payload check; both needed in gen_meta_code_v0
- **Java META_TRIM_META**: added as compile-time `public static final int` (no JNI function needed)
