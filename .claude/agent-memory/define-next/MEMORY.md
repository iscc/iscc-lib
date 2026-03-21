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
- **Handoff "IDLE" can be stale** — state assessment may create new normal-priority issues AFTER the
    handoff. Always check issues.md directly rather than trusting handoff "IDLE" signals
- **Generated files (tool output) don't count toward the 3-file modification limit** — they're
    created by running a tool, not hand-written
- **CI red always first** — even if handoff suggests feature work, a failing CI job takes priority.
    Green CI is a prerequisite for all other work

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

## Kotlin Bindings (in progress)

- **UniFFI Kotlin output is JVM-only** — uses JNA (`com.sun.jna.*`), NOT Kotlin/Native cinterop
- Generated file: 3217 lines, package `uniffi.iscc_uniffi`, loads `libiscc_uniffi.so` via JNA
- KMP with Kotlin/Native targets (iOS, macOS) would need separate cinterop approach — NOT supported
    by UniFFI's Kotlin generator. Start JVM-only, KMP is a future enhancement
- Use `kotlin("jvm")` plugin, NOT `kotlin("multiplatform")` for initial scaffold
- JNA dependency: `net.java.dev.jna:jna:5.16.0`
- Gradle 8.12.1 available via `mise exec gradle@8.12.1 -- gradle wrapper`
- JDK 17 is installed in devcontainer (openjdk-17-jdk-headless)
- Generated Kotlin uses `--no-format` flag since ktlint is not installed in devcontainer
- Kotlin step sequence: scaffold+compile -> conformance tests -> CI job -> docs -> version sync ->
    README -> publishing
- data.json will need to be copied to `packages/kotlin/` for conformance tests (future step)

## Swift Bindings (COMPLETE)

- All sub-tasks done: UniFFI crate, SPM package, CI job, conformance tests, howto guide, README,
    CLAUDE.md, version sync. Issue resolved
- SPM distribution via Git tags (no registry upload needed)

## Dev Environment Constraints

- **No Swift toolchain** in Linux devcontainer — `swift test` can only run on macOS (CI)
- `uniffi-bindgen` not pre-installed — use in-crate binary via
    `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen`

## Conformance Vector Loader Differences (critical for data.json updates)

- **data.json copies**: `crates/iscc-lib/tests/data.json` (primary),
    `packages/go/testdata/data.json`, `packages/dotnet/Iscc.Lib.Tests/testdata/data.json`, and
    `packages/swift/Tests/IsccLibTests/data.json` (all identical). Must be updated together.

## Project Status

- v0.3.1 released, 15/15 CI jobs green
- All 11 bindings complete (Rust, Python, Node.js, WASM, C FFI, Java, Go, Ruby, C#/.NET, C++, Swift)
- 1 normal-priority issue: Kotlin bindings (in progress — scaffold step)

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
