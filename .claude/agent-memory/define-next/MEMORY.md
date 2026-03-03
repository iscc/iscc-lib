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

## Feature Flags Design (Issue #16)

- **Dependency graph analysis** (iteration 13):
    - `serde_json` is used by `conformance.rs` for parsing data.json — cannot be fully gated without
        restructuring conformance. Stays as regular dep for now
    - `serde_json_canonicalizer` only used by `gen_meta_code_v0` and `json_to_data_url` — gated behind
        `meta-code`
    - `unicode-normalization` + `unicode-general-category` only used in `utils.rs` by `text_clean` and
        `text_collapse` — gated behind `text-processing`
    - `gen_meta_code_v0` uses text_clean/text_collapse, so `meta-code` implies `text-processing`
- **Cross-module dependencies**:
    - `text_remove_newlines` and `text_trim` have NO unicode deps — always available
    - `multi_hash_blake3` in utils.rs has no unicode deps — always available
    - `gen_text_code_v0` uses `text_collapse` → needs `text-processing`
    - `gen_meta_code_v0` uses `text_clean/trim/remove_newlines/collapse` + serde_json_canonicalizer →
        needs `meta-code` (which brings `text-processing`)
    - `conformance` module calls `gen_meta_code_v0` + `gen_text_code_v0` → gated at module level
        behind `meta-code` for now; adaptation to partial features is follow-up
- **Incremental plan**: Step 1 = define features + gate code. Step 2 = adapt conformance_selftest to
    skip disabled tests. Step 3 = CI workflow changes
- **lib.rs test gating pattern**: Gate individual test functions with `#[cfg(feature)]`, NOT the
    entire `mod tests` block, because it contains tests for both gated and ungated code

## Documentation Sweep Patterns

- `crates/iscc-wasm/pkg/README.md` must always be identical to `crates/iscc-wasm/README.md`
- When updating "9 gen functions" to "10", distinguish context: data.json has 9 function sections
    (no gen_sum_code_v0), so conformance/benchmark code correctly says "9"
- Two docs pages (architecture.md, development.md) share identical directory tree and crate summary
    table — edits must be synced between them

## CI/Release Patterns

- v0.0.4 released to all registries
- Release workflow has `workflow_dispatch` with per-registry checkboxes + `ffi` boolean

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- Java `byte` is signed — values 128-255 wrap, JNI handles correctly
- Windows GHA runners default to `pwsh` — always add `shell: bash` for bash syntax

## Project Status

- Iteration 13: Starting issue #16 (feature flags). All other work complete
- 1 open issue: #16 (feature flags, normal priority)
- v0.0.4 released to all registries
