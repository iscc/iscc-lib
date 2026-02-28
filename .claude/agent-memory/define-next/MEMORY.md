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
- File deletions don't count toward the 3-file modification limit — they are simpler than edits
- After a major rewrite (e.g., Go pure rewrite), docs/CI lag behind — schedule a cleanup step to
    bring all stale references in sync before moving to the next feature
- State assessments can go stale — always verify claimed gaps by reading the actual files. The state
    may say "met" for something that still has stale content
- When state says "all automatable work complete," cross-check the spec's verification criteria
    against actual files — state assessment may miss spec requirements that were never implemented
    (e.g., missing Reference pages)

## Architecture Decisions

- Java conformance tests use `data.json` via relative path from Maven's working directory
- Maven Surefire `-Djava.library.path` points to `target/debug/` for finding native cdylib
- Go bindings are pure Go (no WASM, no wazero, no binary artifacts)
- All binding conformance tests follow the same structure: load data.json, iterate per-function
    groups, decode inputs per signature, compare `.iscc` output
- `gen_iscc_code_v0` test vectors have no `wide` parameter — always pass `false`
- `"stream:<hex>"` prefix denotes hex-encoded byte data for Data/Instance-Code tests

## Documentation Reference Pages Status (Iteration 18)

Documentation spec requires 4 Reference pages:

1. Rust API (`rust-api.md`) — ✓ exists
2. Python API (`api.md`) — ✓ exists
3. Java API — ✗ missing (future step)
4. C FFI reference — ✗ missing (scoped for iteration 18)

The state.md previously said "all navigation sections complete" but this was inaccurate — 2 of 4
spec-required Reference pages were missing. Always cross-check spec requirements vs actual file
existence.

## C FFI Reference Page Facts

- 43 exported `extern "C"` functions in `crates/iscc-ffi/src/lib.rs`
- Struct types: `IsccByteBuffer`, `IsccByteBufferArray`, `IsccDecodeResult`
- Memory model: caller frees with `iscc_free_string()`, NULL on error + `iscc_last_error()`
- Also exports `iscc_alloc`/`iscc_dealloc` for WASM-side memory allocation

## CI/Release Patterns

- Release workflow has `workflow_dispatch` inputs: `crates-io`, `pypi`, `npm`, `maven` (booleans)
- All publish jobs have idempotency checks (version-existence pre-check, `skip` output)
- `scripts/version_sync.py` uses only stdlib — can run as `python scripts/version_sync.py --check`

## Project Near-Completion State (Iteration 18)

All 7 bindings at 30/30, CI green with 9 jobs. PR #10 exists from develop→main.

**Remaining automated gaps:**

1. C FFI reference page — SCOPED (iteration 18)
2. Java API reference page — future step
3. Tab order standardization — LOW priority, needs human review
4. Publishing infrastructure (OIDC, npm, Maven Central) — human tasks
5. PR #10 merge — human task

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- ISCC Foundation URL is `https://iscc.io`
- Java `byte` is signed — values 128-255 wrap, JNI handles correctly
- Two docs pages (architecture.md, development.md) share identical directory tree and crate summary
    table — edits must be synced between them
