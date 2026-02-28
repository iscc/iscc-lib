# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Scope Calibration Principles

- CI job additions are small, single-file changes that provide high value. Pattern: copy existing
    job structure, swap language-specific setup action and build/test commands
- Critical issues always take priority regardless of feature trajectory
- Multiple small issues in the same crate are a natural batch (e.g., 3 fixes touching 2 files)
- README files are "create" operations — less risky than code changes. Doc files excluded from
    3-file limit
- When CI is red, formatting/lint fixes are always the first priority regardless of handoff "Next"
- Prefer concrete deliverables over research tasks when both are available
- Generic API optimization: prefer generic bounds (`AsRef<T> + Ord`) over concrete type changes to
    avoid cascading modifications across binding crates

## Architecture Decisions

- Java conformance tests use `data.json` via relative path from Maven's working directory
- Maven Surefire `-Djava.library.path` points to `target/debug/` for finding native cdylib
- Go bindings are being rewritten as pure Go (no WASM, no wazero, no binary artifacts)
- All binding conformance tests follow the same structure: load data.json, iterate per-function
    groups, decode inputs per signature, compare `.iscc` output
- `gen_iscc_code_v0` test vectors have no `wide` parameter — always pass `false`
- `"stream:<hex>"` prefix denotes hex-encoded byte data for Data/Instance-Code tests

## Go Pure Rewrite — Scoping Strategy

- **Port order (dependency graph)**: codec (done) → text utils (done) → algorithms (all 7 done) →
    gen functions (6/9 done, video+mixed next) → conformance selftest → WASM removal
- **Coexistence pattern**: new pure Go files live alongside WASM bridge (`iscc.go`) in same `iscc`
    package. WASM removal only after all pure Go modules pass conformance
- **Test verification**: `go build ./...` (compiles with WASM code) + `go test -run TestXxx` +
    `go vet ./...`. Full suite at each step (both pure Go and WASM tests pass)
- **Gen function batching**: two gen functions per step works well (Image+Audio done, Video+Mixed
    next, then IsccCode alone as final). Each gen function is ~60-140 lines Go
- **Step 5a**: COMPLETE — GenMetaCodeV0 + GenTextCodeV0 (21/21 vectors pass)
- **Step 5b**: COMPLETE — GenDataCodeV0, GenInstanceCodeV0, DataHasher, InstanceHasher (7/7 vectors)
- **Step 5c**: COMPLETE — GenImageCodeV0 + GenAudioCodeV0 (8/8 vectors)
- **Step 5d**: COMPLETE — GenVideoCodeV0 + GenMixedCodeV0 (5/5 vectors)
- **Step 5e (current)**: GenIsccCodeV0 — composite ISCC-CODE assembly. Uses `encodeUnits`,
    `decodeBase32`, `decodeHeader`, `decodeLength`, `encodeHeader`, `encodeBase32` (all in
    codec.go). Sort decoded entries by MainType, validate Data+Instance mandatory, determine SubType
    from Content/Semantic codes. 5 conformance vectors, no `wide` or `bits` in vectors
- **Step 6 (current)**: ConformanceSelftest — pure Go function, embed data.json via
    `//go:embed   testdata/data.json`. Copy data.json to `packages/go/testdata/` since `//go:embed`
    only works with files inside module directory. Mirrors Rust's `include_str!` pattern.
- **After step 6**: WASM bridge cleanup (remove iscc.go, iscc_ffi.wasm, wazero dep, restore 256KB
    threshold)
- **Go function naming**: pure Go gen functions are package-level (e.g., `GenMetaCodeV0`) and
    coexist with WASM bridge methods (`(rt *Runtime) GenMetaCodeV0`) — no naming conflict in Go
- **Conformance vector format**: data.json inputs are positional arrays. For video:
    `[frame_sigs_array, bits]` (each frame = 380 int32s). For mixed: `[codes_array, bits]` (codes
    are ISCC strings without prefix)
- **Result types**: pure Go gen functions return rich structs (`*VideoCodeResult`,
    `*MixedCodeResult` etc.) unlike the WASM bridge which returns only `(string, error)`
- **Deduplication in SoftHashVideoV0**: Rust uses `BTreeSet<&S>`. Go: string-keyed map from
    serialized `[]int32` sigs. Order doesn't matter for column-wise sum
- **softHashCodesV0 is unexported**: matches Rust's `pub(crate)`. SoftHashVideoV0 IS exported
    (matches Rust's `pub`)
- **arraySplit helper**: defined in code_content_audio.go, reusable by any file in `iscc` package

## CI/Release Patterns

- Release workflow has `workflow_dispatch` inputs: `crates-io`, `pypi`, `npm`, `maven` (booleans)
- All publish jobs have idempotency checks (version-existence pre-check, `skip` output)
- Go CI job currently builds WASM binary before running tests — must simplify after rewrite

## Recurring Patterns

- When new symbols are added to bindings but docs predate them, batch howto guide + README update
    into one step (2 files). Verification is grep-based
- Interactive sessions can break CI (e.g., Python ruff format) — always check state.md CI section
- After PR merge, always switch back to develop: `git checkout develop`

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- ISCC Foundation URL is `https://iscc.io`
- Java `byte` is signed — values 128-255 wrap, JNI handles correctly
