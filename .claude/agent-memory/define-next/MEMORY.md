# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Scope Calibration Principles

- Critical issues always take priority regardless of feature trajectory
- Multiple small issues in the same crate are a natural batch (e.g., 3 fixes touching 2 files)
- Doc files are excluded from the 3-file modification limit — can batch all 6 howto guides in one
    step since they follow identical patterns
- When CI is red, formatting/lint fixes are always the first priority regardless of handoff "Next"
- Prefer concrete deliverables over research tasks when both are available
- State assessments can go stale — always verify claimed gaps by reading the actual files
- New Tier 1 symbols: always implement in Rust core first, then propagate to bindings in separate
    steps. Core + tests in one step, bindings in subsequent steps
- When previous next.md already contains correct scoping, verify line references are still accurate
    and refresh rather than rewrite from scratch — avoid unnecessary churn
- Repetitive doc additions across language guides: all 6 howto files follow identical structure
    (heading, 1-line description, fenced code block). Safe to batch all in one step

## Architecture Decisions

- Go bindings are pure Go (no WASM, no wazero, no binary artifacts)
- All binding conformance tests follow the same structure: load data.json, iterate per-function
    groups, decode inputs per signature, compare `.iscc` output
- `gen_iscc_code_v0` test vectors have no `wide` parameter — always pass `false`
- `"stream:<hex>"` prefix denotes hex-encoded byte data for Data/Instance-Code tests

## Benchmark Patterns

- `benchmarks.rs` uses `criterion_group!` macro to register all bench functions
- Data/Instance/ISCC-Code benchmarks use `BenchmarkId` + `Throughput::Bytes` for throughput metrics
- `deterministic_bytes(size)` helper generates reproducible test data
- `gen_sum_code_v0` requires `&Path` (temp file needed) — unlike `gen_data_code_v0` which takes
    `&[u8]` directly. Temp file must be created OUTSIDE the bench closure

## Documentation Sweep Patterns

- `crates/iscc-wasm/pkg/README.md` must always be identical to `crates/iscc-wasm/README.md` — both
    are published to npm
- When updating "9 gen functions" to "10", distinguish context: data.json has 9 function sections
    (no gen_sum_code_v0), so conformance/benchmark code correctly says "9"
- Two docs pages (architecture.md, development.md) share identical directory tree and crate summary
    table — edits must be synced between them

## CI/Release Patterns

- v0.0.4 released to all registries. Next release after remaining C FFI DX gaps closed.
- Release workflow has `workflow_dispatch` with per-registry checkboxes

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- Java `byte` is signed — values 128-255 wrap, JNI handles correctly

## C FFI DX Phase

- 4 new C FFI DX criteria added in commit 9721877 (issues #22-#25)
- Priority order from state.md: #24 (header) ✅ → #23 (examples) → #22 (howto) → #25 (release)
- Issue #24 DONE: header committed at `crates/iscc-ffi/include/iscc.h`, CI freshness check active
- `.gitignore` ignores `crates/iscc-ffi/tests/iscc.h` but NOT `crates/iscc-ffi/include/iscc.h`
- `cbindgen.toml` already has correct config (language=C, guard=ISCC_H, sys_includes)
- cmake is NOT available in devcontainer — use gcc for compilation verification (matches CI pattern)
- CI C FFI compilation pattern:
    `gcc -o test_iscc <src> -I crates/iscc-ffi/include -L target/debug   -liscc_ffi -lpthread -ldl -lm`
    then `LD_LIBRARY_PATH=target/debug ./test_iscc`
- `crates/iscc-ffi/examples/` directory does not exist yet — advance agent must create it
- C example files are "create only" steps — no existing files need modification

## Project Status

- Iteration 19: C FFI DX continued. 3 criteria unmet (#23, #22, #25), 5 open issues
- All original target sections met. Current work is C FFI DX remaining items
- v0.0.4 released to all registries
