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
- Repetitive doc additions across language guides: all 6 howto files follow identical structure
    (heading, 1-line description, fenced code block). Safe to batch all in one step

## Signature Change Propagation

- When a Rust core function signature changes, ALL Rust-based binding crates must be updated in the
    SAME step to keep CI green
- WASM binding has its OWN inline `gen_sum_code_v0` (no filesystem in WASM)
- Go binding is pure Go — completely independent of Rust core signatures

## Architecture Decisions

- Go bindings are pure Go (no WASM, no wazero, no binary artifacts)
- All binding conformance tests follow the same structure: load data.json, iterate per-function
    groups, decode inputs per signature, compare `.iscc` output
- `gen_iscc_code_v0` test vectors have no `wide` parameter — always pass `false`
- `"stream:<hex>"` prefix denotes hex-encoded byte data for Data/Instance-Code tests

## Conformance Vector Loader Differences (critical for data.json updates)

- **Rust core** (`conformance.rs`): Uses `serde_json::Value`, accesses sections by name
    (`data["gen_meta_code_v0"]`). Ignores unknown top-level keys. Auto-discovers new vectors.
- **Python** (`test_conformance.py`): Accesses by name (`data[function_name]`). Safe.
- **Node.js** (`conformance.test.mjs`): Accesses `data.gen_meta_code_v0`. Safe.
- **WASM** (`conformance.rs`): Same as Rust core (uses `serde_json::Value`). Safe.
- **Java** (`IsccLibTest.java`): Uses `data.getAsJsonObject("gen_meta_code_v0")`. Safe.
- **Go** (`conformance.go`): Uses `map[string]map[string]vectorEntry` — parses ALL top-level keys.
    **BREAKS** on non-vector entries like `_metadata`. Must use `json.RawMessage` intermediate step.
- **C FFI**: No data.json loader (uses Rust core conformance_selftest).
- **data.json copies**: `crates/iscc-lib/tests/data.json` (primary) and
    `packages/go/testdata/data.json` (identical copy). Both must be updated together.

## Feature Flags Design (Issue #16) — RESOLVED

- Issue #16 fully resolved across iterations 13-15 (definitions, selftest, CI matrix)
- `default = ["meta-code"]`, `text-processing` (unicode deps), `meta-code` (implies text-processing)

## Documentation Sweep Patterns

- `crates/iscc-wasm/pkg/README.md` must always be identical to `crates/iscc-wasm/README.md`
- When updating "9 gen functions" to "10", distinguish context: data.json has 9 function sections
    (no gen_sum_code_v0), so conformance/benchmark code correctly says "9"
- Two docs pages (architecture.md, development.md) share identical directory tree and crate summary
    table — edits must be synced between them

## CI/Release Patterns

- v0.1.0 released to all registries
- Release workflow has `workflow_dispatch` with per-registry checkboxes + `ffi` boolean

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- Java `byte` is signed — values 128-255 wrap, JNI handles correctly
- Windows GHA runners default to `pwsh` — always add `shell: bash` for bash syntax
- Go `json.Marshal` for float64: uses 'f' format for values >= 1e-6 and < 1e21, otherwise 'e'
    format. 1e20 < 1e21 → outputs "100000000000000000000". May or may not match JCS exactly for edge
    cases. Risk area for test_0017/test_0018.

## Project Status

- Issue #16 fully resolved (iterations 13-15)
- v0.1.0 released to all registries
- 3 open issues: iscc-core v1.3.0 conformance (critical), Ruby bindings (normal), README logos (low)
- Current iteration: vendoring v1.3.0 data.json + Go loader fix
