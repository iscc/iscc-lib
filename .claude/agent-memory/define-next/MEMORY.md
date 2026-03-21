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
- **State assessments can go stale** — always verify claimed gaps by reading the actual files
- New Tier 1 symbols: always implement in Rust core first, then propagate to bindings in separate
    steps. Core + tests in one step, bindings in subsequent steps
- **Handoff "IDLE" can be stale** — always check issues.md directly
- **Generated files (tool output) don't count toward the 3-file modification limit**
- **CI red always first** — green CI is a prerequisite for all other work
- **Root cause before fix** — check actual coordinates against known-good references
- **Target gaps vs low issues** — target is source of truth for what needs to be done
- **Review agent can miscount issues** — always read issues.md directly
- **Batch related small changes** — version sync + docs update for same feature can combine into one
    step (1 code file + 1 doc file excluded from limit)

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

## UniFFI Scaffolding (Swift/Kotlin foundation)

- UniFFI v0.31.0 is the latest stable version (checked 2026-03-21)
- Proc macro approach: `#[uniffi::export]`, `#[derive(uniffi::Record)]`, `#[derive(uniffi::Object)]`
    — no UDL files or `build.rs` needed
- Key type constraints: no `usize` (use `u64`), no borrowed types, no generics on exported functions
- Constants are getter functions (UniFFI can't export `const`)
- Streaming types use `Mutex<Option<Inner>>` pattern
- **SPM module name MUST match generated code**: `iscc_uniffiFFI`

## Swift XCFramework Implementation Plan

Multi-step effort, tracked as normal-priority issue. Progress:

1. ~~Build script + root Package.swift restructure~~ (DONE — iteration 2)
2. ~~Release workflow integration (build-xcframework job)~~ (DONE — iteration 3)
3. Version sync + docs update — **current step** (iteration 4)

Key constraints:

- Two Package.swift files: root (SPM consumers) and `packages/swift/Package.swift` (CI/dev)
- CI Swift job uses packages/swift/Package.swift — root changes don't break CI
- Build script only runs on macOS (lipo, xcodebuild, ditto are macOS tools)
- `cargo build -p iscc-uniffi` (not iscc-lib) produces `libiscc_uniffi.a`
- Ferrostar pattern: `useLocalFramework` variable toggle, force-update tag for checksum
- macOS sed uses `sed -E -i ''` (empty backup ext) — differs from GNU `sed -i`
- `build-xcframework` job is independent (no `needs`) — builds from source on macOS
- GITHUB_REF_NAME bug flagged `HUMAN REVIEW REQUESTED` — CID must not fix without human input

## Dev Environment Constraints

- **No Swift toolchain** in Linux devcontainer — `swift test` can only run on macOS (CI)
- **No shellcheck** in Linux devcontainer — can't lint shell scripts locally
- `uniffi-bindgen` not pre-installed — use in-crate binary via
    `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen`

## Conformance Vector Loader Differences (critical for data.json updates)

- **data.json copies**: `crates/iscc-lib/tests/data.json` (primary),
    `packages/go/testdata/data.json`, `packages/dotnet/Iscc.Lib.Tests/testdata/data.json`,
    `packages/swift/Tests/IsccLibTests/data.json`, and
    `packages/kotlin/src/test/resources/data.json` (all identical). Must be updated together.

## Project Status

- v0.3.1 released, 16/16 CI jobs green
- All 12 bindings complete (Rust, Python, Node.js, WASM, C FFI, Java, Go, Ruby, C#/.NET, C++, Swift,
    Kotlin)
- pytest-benchmark done (18 functions), speedup factors already in `docs/benchmarks.md`
- 2 normal issues: Swift XCFramework vend (version sync + docs remain), Swift GITHUB_REF_NAME bug
    (HUMAN REVIEW)
- 1 low issue: language logos in docs

## Version Sync Script Patterns

- `scripts/version_sync.py` uses `(file_path, get_fn, sync_fn)` triples in TARGETS list
- JSON targets (package.json, vcpkg.json): can reuse
    `_get_package_json_version`/`_sync_package_json`
- `version_sync.py --check` runs in CI (`version-check` job)
- Currently 15 targets; adding Package.swift releaseTag will make 16

## CI/Release Patterns

- v0.3.1 released to all registries
- Release workflow has `workflow_dispatch` with 9 per-registry checkboxes + `ffi` boolean
- `iscc-rb` requires `libclang-dev` — cannot remove `--exclude iscc-rb` from Rust CI job

## Docs Infrastructure

- `zensical.toml` has `nav` array for howto guides — must add entry when creating new guide
- `scripts/gen_llms_full.py` has `ORDERED_PAGES` list — must add entry for llms-full.txt generation
- All howto guides follow identical structure (see `docs/howto/dotnet.md` as template)
- Per-package CLAUDE.md files follow `packages/dotnet/CLAUDE.md` structure
- Howto install sections use collapsible `??? tip "Build from source"` pattern

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- Windows GHA runners default to `pwsh` — always add `shell: bash` for bash syntax
- When vendoring new data.json vectors, ALL binding crates with hardcoded vector count assertions
    must be updated (Rust core + WASM)
- **Gson groupId trap**: Maven groupId is `com.google.code.gson`, NOT `com.google.gson`
