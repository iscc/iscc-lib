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

## gen_sum_code_v0 — Status: all 7 bindings DONE, howto examples pending

Issue #15 fully resolved. All 7 bindings complete. README/docs sweep complete (iteration 14).
Remaining: 6 howto guides need `### Sum-Code` subsections with code examples.

**WASM is the outlier:** Takes `Uint8Array`/`&[u8]` (no filesystem in browser) while all other
bindings take file paths. The WASM howto already uses `TextEncoder` for data encoding.

**Howto structure (all 6 files identical):** 9 subsections under `## Code generation` heading
(Meta-Code through ISCC-CODE). New `### Sum-Code` goes after `### ISCC-CODE`.

**SumCodeResult already in result tables:** rust.md and python.md already list `SumCodeResult` in
their "Structured results" tables — no need to update those.

## Documentation Sweep Patterns

- `crates/iscc-wasm/pkg/README.md` must always be identical to `crates/iscc-wasm/README.md` — both
    are published to npm
- When updating "9 gen functions" to "10", distinguish context: data.json has 9 function sections
    (no gen_sum_code_v0), so conformance/benchmark code correctly says "9"
- Two docs pages (architecture.md, development.md) share identical directory tree and crate summary
    table — edits must be synced between them

## CI/Release Patterns

- v0.0.3 released to all registries. Next release after doc completion.
- Release workflow has `workflow_dispatch` with per-registry checkboxes

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- Java `byte` is signed — values 128-255 wrap, JNI handles correctly

## Post-Howto Roadmap

After howto completion, only issue #16 remains (feature flags for minimal builds, low priority). The
project may be approaching target completion — verify all target.md criteria are met.
