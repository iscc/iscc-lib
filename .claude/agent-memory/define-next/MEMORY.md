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
- **State assessments can go stale** — always verify claimed gaps by reading the actual files. E.g.,
    state claimed "speedup factors not yet published" but `docs/benchmarks.md` already had them
- New Tier 1 symbols: always implement in Rust core first, then propagate to bindings in separate
    steps. Core + tests in one step, bindings in subsequent steps
- **Handoff "IDLE" can be stale** — state assessment may create new normal-priority issues AFTER the
    handoff. Always check issues.md directly rather than trusting handoff "IDLE" signals
- **Generated files (tool output) don't count toward the 3-file modification limit** — they're
    created by running a tool, not hand-written
- **CI red always first** — even if handoff suggests feature work, a failing CI job takes priority.
    Green CI is a prerequisite for all other work
- **Root cause before fix** — when CI fails with a dependency error, check the actual coordinates
    (groupId, artifactId, version) against known-good references in the project before suggesting
    environment fixes
- **Target gaps vs low issues** — when state.md says "IDLE" but target.md has unmet verification
    criteria that aren't filed as `low` issues in issues.md, they're legitimate gaps to work on. The
    target is the source of truth for what needs to be done
- **Review agent can miscount issues** — always read issues.md directly to check priorities

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
- Key type constraints: no `usize` (use `u64`), no borrowed types (use owned `String`, `Vec<u8>`),
    no generics on exported functions (use concrete types like `Vec<Vec<i32>>`)
- Constants are getter functions (UniFFI can't export `const`)
- Streaming types use `Mutex<Option<Inner>>` pattern for interior mutability with `&self` methods
- **SPM module name MUST match generated code**: UniFFI generates `#if canImport(iscc_uniffiFFI)` /
    `import iscc_uniffiFFI`. The SPM FFI target must use `iscc_uniffiFFI` (not custom names)

## Swift XCFramework Implementation Plan

Multi-step effort, tracked as normal-priority issue. Progress:

1. ~~Build script + root Package.swift restructure~~ (DONE — iteration 2)
2. Release workflow integration (build-xcframework job, force-update tag) — **in progress**
3. Version sync for releaseTag in version_sync.py
4. Docs/howto update (swift.md SPM install docs)

Key constraints:

- Two Package.swift files: root (SPM consumers) and `packages/swift/Package.swift` (CI/dev)
- CI Swift job uses packages/swift/Package.swift — root changes don't break CI
- Build script only runs on macOS (lipo, xcodebuild, ditto are macOS tools)
- `cargo build -p iscc-uniffi` (not iscc-lib) produces `libiscc_uniffi.a`
- Ferrostar pattern: `useLocalFramework` variable toggle, force-update tag for checksum
- macOS sed uses `sed -E -i ''` (empty backup ext) — differs from GNU `sed -i`
- `build-xcframework` job is independent (no `needs`) — builds from source on macOS

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
- 1 normal issue: Swift XCFramework distribution
- 1 low issue: language logos in docs
- `docs/benchmarks.md` reproduction commands reference wrong path (`pytest benchmarks/python/`
    should be `tests/test_benchmarks.py`) — minor doc bug for future cleanup

## Version Sync Script Patterns

- `scripts/version_sync.py` uses `(file_path, get_fn, sync_fn)` triples in TARGETS list
- JSON targets (package.json, vcpkg.json): can reuse
    `_get_package_json_version`/`_sync_package_json`
- `version_sync.py --check` runs in CI (`version-check` job)

## CI/Release Patterns

- v0.3.1 released to all registries
- Release workflow has `workflow_dispatch` with per-registry checkboxes + `ffi` boolean
- `iscc-rb` requires `libclang-dev` — cannot remove `--exclude iscc-rb` from Rust CI job

## Docs Infrastructure

- `zensical.toml` has `nav` array for howto guides — must add entry when creating new guide
- `scripts/gen_llms_full.py` has `ORDERED_PAGES` list — must add entry for llms-full.txt generation
- All howto guides follow identical structure (see `docs/howto/dotnet.md` as template)
- Per-package CLAUDE.md files follow `packages/dotnet/CLAUDE.md` structure

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- Windows GHA runners default to `pwsh` — always add `shell: bash` for bash syntax
- When vendoring new data.json vectors, ALL binding crates with hardcoded vector count assertions
    must be updated (Rust core + WASM)
- **Gson groupId trap**: Maven groupId is `com.google.code.gson`, NOT `com.google.gson`. The Java
    import package `com.google.gson.*` differs from the Maven coordinate — easy to confuse
