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
- Small utility functions that compose existing helpers are ideal single-step scopes (e.g.,
    `JsonToDataUrl` = 3 existing unexported helpers composed into 1 public function)

## Architecture Decisions

- Java conformance tests use `data.json` via relative path from Maven's working directory
- Maven Surefire `-Djava.library.path` points to `target/debug/` for finding native cdylib
- Go bindings are pure Go (no WASM, no wazero, no binary artifacts)
- All binding conformance tests follow the same structure: load data.json, iterate per-function
    groups, decode inputs per signature, compare `.iscc` output
- `gen_iscc_code_v0` test vectors have no `wide` parameter — always pass `false`
- `"stream:<hex>"` prefix denotes hex-encoded byte data for Data/Instance-Code tests

## Go Package — Key Facts

- **30/30 after JsonToDataUrl**: Was 29/30 — WASM bridge had it but pure Go rewrite missed it
- Unexported helpers in `code_meta.go`: `parseMetaJSON`, `jsonHasContext`, `buildMetaDataURL` —
    these compose directly into the public `JsonToDataUrl` function
- `EncodeBase64` and `EncodeComponent` live in `codec.go` — encoding utilities group here
- `DecodeResult` struct and 4 algorithm constants (`MetaTrimName`, etc.) live in `codec.go`
- Go `encoding/json.Marshal` produces JCS-compatible output for string-only JSON values
- Go function naming: `PascalCase` for exported symbols (e.g., `JsonToDataUrl`)
- `TestPureGo*` test prefix is historical vestige — cosmetic cleanup only, no urgency
- Module deps: `github.com/zeebo/blake3`, `golang.org/x/text` (+ cpuid indirect)

## CI/Release Patterns

- Release workflow has `workflow_dispatch` inputs: `crates-io`, `pypi`, `npm`, `maven` (booleans)
- All publish jobs have idempotency checks (version-existence pre-check, `skip` output)

## Post-Go-Completion Planning

After Go reaches 30/30 ("met"):

- PR from develop → main — major milestone worth merging
- Remaining gaps: benchmarks (CI integration), publishing (OIDC, npm, Maven Central)
- Go CI job may have leftover WASM build steps that can be simplified

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
- Handoff review agent incorrectly claimed 30/30 after WASM removal — state assessment caught that
    `JsonToDataUrl` was only in the deleted WASM bridge. Always verify state.md over handoff claims
