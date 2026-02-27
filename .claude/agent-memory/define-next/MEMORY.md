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

- **Algorithm batching**: CDC (~300 lines Rust) + MinHash (~260 lines) + SimHash+SlidingWindow (~330
    lines) = ~890 lines → 3 Go files. DCT and WTA-Hash are separate (only for Image/Video-Code)
- **Port order (dependency graph)**: codec (done) → text utils (done) → algorithms (CDC, MinHash,
    SimHash done; DCT, WTA-Hash next) → gen\_\*\_v0 functions → streaming hashers → conformance
    selftest → WASM removal
- **Coexistence pattern**: new pure Go files live alongside WASM bridge (`iscc.go`) in same `iscc`
    package. WASM removal only after all pure Go modules pass conformance
- **Codec**: stdlib only (no external deps). ~570 lines Go + ~930 lines tests
- **Text utils**: `golang.org/x/text/unicode/norm` for NFKC/NFD. ~130 lines + ~190 lines tests
- **Test verification**: `go build ./...` (compiles with WASM code) + `go test -run TestXxx` +
    `go vet ./...`. Full suite at each step (both pure Go and WASM tests pass)
- **Shared types**: `DecodeResult` struct in `iscc.go` referenced by `codec.go` (same package). When
    WASM is removed, struct moves to appropriate file
- **Next algorithm step**: DCT (~150 lines Rust) + WTA-Hash (~390 lines) — separate batch because
    only needed for Image/Video-Code. Both are pure computation, no external deps

## CI/Release Patterns

- Release workflow has `workflow_dispatch` inputs: `crates-io`, `pypi`, `npm`, `maven` (booleans)
- All publish jobs have idempotency checks (version-existence pre-check, `skip` output)
- `build-jni` matrix: 5 platforms with `native-dir`/`lib-name` matching NativeLoader conventions
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
