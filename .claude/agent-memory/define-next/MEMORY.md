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
- File deletions don't count toward the 3-file modification limit — they are simpler than edits

## Architecture Decisions

- Java conformance tests use `data.json` via relative path from Maven's working directory
- Maven Surefire `-Djava.library.path` points to `target/debug/` for finding native cdylib
- Go bindings are being rewritten as pure Go (no WASM, no wazero, no binary artifacts)
- All binding conformance tests follow the same structure: load data.json, iterate per-function
    groups, decode inputs per signature, compare `.iscc` output
- `gen_iscc_code_v0` test vectors have no `wide` parameter — always pass `false`
- `"stream:<hex>"` prefix denotes hex-encoded byte data for Data/Instance-Code tests

## Go Pure Rewrite — Status and Key Facts

- **All 30/30 Tier 1 symbols implemented as pure Go** — 9 gen functions + all utilities + streaming
    hashers + ConformanceSelftest. All pass conformance vectors
- **WASM bridge cleanup is the final step**: remove `iscc.go`, `iscc_ffi.wasm`, `iscc_test.go`,
    wazero dependency, restore `--maxkb=256`
- **Critical relocation needed**: `DecodeResult` struct and 4 algorithm constants (`MetaTrimName`,
    `MetaTrimDescription`, `IoReadSize`, `TextNgramSize`) are defined in `iscc.go` but used by pure
    Go files — must be moved to `codec.go` before deletion
- **Go function naming**: pure Go gen functions are package-level (e.g., `GenMetaCodeV0`) and
    coexist with WASM bridge methods (`(rt *Runtime) GenMetaCodeV0`) — no naming conflict
- **Result types**: pure Go gen functions return rich structs (`*VideoCodeResult`,
    `*MixedCodeResult` etc.) unlike the WASM bridge which returns only `(string, error)`
- **Go `DataHasher`/`InstanceHasher` Finalize is single-use** (mutates internal state). Do not call
    Finalize twice

## CI/Release Patterns

- Release workflow has `workflow_dispatch` inputs: `crates-io`, `pypi`, `npm`, `maven` (booleans)
- All publish jobs have idempotency checks (version-existence pre-check, `skip` output)
- Go CI job currently builds WASM binary before running tests — will simplify after WASM removal

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

## Post-Go-Rewrite Planning

- After WASM cleanup: Go bindings section becomes "met" in state.md
- Critical issue in issues.md can be closed after WASM removal is verified
- Remaining gaps: benchmarks (CI integration), publishing (OIDC, npm, Maven Central)
- Consider: PR from develop → main once Go rewrite is complete and CI is green
