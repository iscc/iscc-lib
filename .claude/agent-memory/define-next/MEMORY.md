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
- **Handoff "IDLE" can be stale** — always check issues.md directly
- **Generated files (tool output) don't count toward the 3-file modification limit**
- **CI red always first** — green CI is a prerequisite for all other work
- **Target gaps vs low issues** — target is source of truth for what needs to be done
- **Review agent can miscount issues** — always read issues.md directly
- **Batch related small changes** — version sync + docs update for same feature can combine into one
    step (1 code file + 1 doc file excluded from limit)
- **When blocked issues dominate** — look for target verification criteria gaps rather than
    accepting "idle". Docs completeness (tabbed examples, tables) is often missed
- **next.md is a sensitive file** — Write tool may be blocked; use `cat > file << 'EOF'` via Bash
- **HUMAN REVIEW REQUESTED issues**: When evidence is overwhelming (bytecode-verified), scope the
    fix — the review agent will verify. Don't block the CID loop on human confirmation for
    well-understood bugs.
- **IDLE is valid** — when all target sections are met and only low issues remain, signal IDLE.
    Don't invent work. The CID loop ran 7 iterations and correctly reached completion.
- **Prefer boolean-verifiable prerequisites over high-impact-but-risky infra fixes** when both are
    available and there's no in-progress feature. A single-file `pub`→`pub(crate)` change with grep
    \+ `cargo test` checks beats an infra fix whose real verification needs publishing/CI.
- **The npm `optionalDependencies` fix (#38) is high user-impact but risky to autonomously verify**:
    dropping `napi prepublish -t npm` may leave the napi-generated `index.js` loader referencing
    unpublished optional-dep packages. Needs investigation of napi v3's bundled-loader behavior — do
    not scope as a trivial one-line release.yml edit. Better as a dedicated/interactive step.

## Architecture Decisions

- Go bindings are pure Go (no WASM, no wazero, no binary artifacts)
- All binding conformance tests follow the same structure: load data.json, iterate per-function
    groups, decode inputs per signature, compare `.iscc` output
- `gen_iscc_code_v0` test vectors have no `wide` parameter — always pass `false`
- `"stream:<hex>"` prefix denotes hex-encoded byte data for Data/Instance-Code tests

## UniFFI Scaffolding (Swift/Kotlin foundation)

- UniFFI v0.31.0 is the latest stable version (checked 2026-03-21)
- Proc macro approach: `#[uniffi::export]`, `#[derive(uniffi::Record)]`, `#[derive(uniffi::Object)]`
- Key type constraints: no `usize` (use `u64`), no borrowed types, no generics on exported functions
- Constants are getter functions (UniFFI can't export `const`)
- **SPM module name MUST match generated code**: `iscc_uniffiFFI`

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

## CI/Release Patterns

- v0.3.1 released to all registries
- Release workflow has `workflow_dispatch` with 9 per-registry checkboxes
- `iscc-rb` requires `libclang-dev` — cannot remove `--exclude iscc-rb` from Rust CI job
- XCFramework cache key at release.yml:1269 — must hash ALL build inputs
- `swift package dump-package` validates manifest syntax without downloading binary targets — safe
    to use with PLACEHOLDER checksum on develop

## Docs Infrastructure

- `zensical.toml` has `nav` array for howto guides — must add entry when creating new guide
- `scripts/gen_llms_full.py` has `ORDERED_PAGES` list — must add entry for llms-full.txt generation
- All howto guides follow identical structure (see `docs/howto/dotnet.md` as template)
- Howto install sections use collapsible `??? tip "Build from source"` pattern

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- Windows GHA runners default to `pwsh` — always add `shell: bash` for bash syntax
- When vendoring new data.json vectors, ALL binding crates with hardcoded vector count assertions
    must be updated (Rust core + WASM)
- **Gson groupId trap**: Maven groupId is `com.google.code.gson`, NOT `com.google.gson`

## Language API Patterns (for doc examples)

- Ruby: `IsccLib.gen_text_code_v0("text")` — snake_case module methods
- C#: `IsccLib.GenTextCodeV0("text")` — PascalCase static methods
- C++: `iscc::gen_text_code_v0("text")` — namespace free functions, RAII
- Swift: `genTextCodeV0(text: "text", bits: 64)` — camelCase free functions, named params
- Kotlin: `genTextCodeV0(text = "text", bits = 64u)` — camelCase free functions, UInt params

## v1.0.0 Hardening Phase (post-v0.4.0, started ~iter 86)

- v0.4.0 shipped to all registries; CI green 16/16. Human added 7 `normal` issues toward v1.0.0 and
    raised target.md bar. No critical issues. Module-visibility narrowing started at iter 86.
- **Tier 1 surface = 32 crate-root re-exports.** Internal modules (`cdc`, `minhash`, `simhash`,
    `utils`, `conformance`) only leak their *paths*; narrowing them to `pub(crate) mod` is a pre-1.0
    breaking change that must land BEFORE the `cargo-semver-checks` gate locks the surface.
- **Integration tests in `crates/iscc-lib/tests/` deliberately assert module-path access** —
    `test_module_path_imports*` in test_algorithm_primitives.rs + test_text_utils.rs. They break
    when modules go `pub(crate)`; delete them (crate-root `test_flat_crate_root_imports` /
    `test_crate_root_imports*` already cover the same functions). Tests excluded from 3-file limit.
- **v1.0.0 backlog ordering** (per state.md Next Milestone): npm #38 fix → PyO3 0.23→0.29 → v1.0.0
    gates (semver-checks, iai-callgrind, module-visibility) → coverage/CRAP → streaming SumHasher
    (core first, then PyO3 + WASM wrappers) + Python GIL release. `pub use` from a `pub(crate) mod`
    is valid Rust — re-exported Tier 1 symbols stay public after narrowing.
- **iter 88: scoped SumHasher CORE struct first** (issue #37). Chose this over the CI-infra gates
    (semver-checks/iai-callgrind need valgrind/baseline/network — hard to verify in this Linux
    devcontainer) and over PyO3 0.23→0.29 (six-minor migration, too big for one step). The SumHasher
    core is pure Rust, fully `cargo test`-verifiable, builds on existing
    `DataHasher`/`InstanceHasher`. Design: `SumHasher` *composes* the two existing hashers (holds
    both as fields, `update` feeds the same slice to both); `finalize(bits, wide, add_units)`
    returns `SumCodeResult` by porting `gen_sum_code_v0`'s composition (calls
    `crate::gen_iscc_code_v0`). Then `gen_sum_code_v0` is refactored to read file → drive
    `SumHasher` (de-dups the inline loop).
- **SumHasher stays at `iscc_lib::streaming::SumHasher` — NOT promoted to crate-root Tier 1.**
    `streaming` is already `pub mod`, so bindings reach it without any core change. Tier 1 is
    explicitly "bound in all languages" (32 symbols), but issue #37 only adds SumHasher to Python +
    WASM — bumping "32→33 / 2→3 streaming types" would falsely imply all 12 bindings expose it and
    create cross-binding inconsistency. So treat SumHasher as a Python/WASM-specific streaming
    convenience; keep README/rust-core.md/target.md counts untouched. This **revises** the earlier
    "promote crate-root + count bump together with bindings" plan — the handoff (iter 88 review)
    recommended promotion, but the all-languages Tier 1 invariant overrides it.
- **iter 89: scoped Python SumHasher WRAPPER only** (Python half of #37). Chose Python over WASM
    (different test harness, would exceed 3-code-file limit) and over npm #38 (risky napi v3
    bundled-loader verification — see Gotcha note) / PyO3 0.29 (six-minor migration). Touch points:
    `crates/iscc-py/src/lib.rs` (PySumHasher mirrors PyDataHasher Option<inner> + gen_sum_code_v0
    dict construction with optional `units`), `__init__.py` (SumHasher wrapper class + `_SumHasher`
    import + `__all__`), `_lowlevel.pyi` (stub) = exactly 3 code files. `.pyi` counted as code.
    SumCodeResult Python class already exists (reuse). \_lowlevel hasher.update takes `&[u8]` only —
    stream handling lives in the Python wrapper (CLAUDE.md pitfall). Sets up #39 (GIL release
    references "the new SumHasher"). WASM SumHasher is the clean mirror follow-up.
- **gen_sum_code_v0 has a full existing test suite** in lib.rs (~:2168): equivalence, empty file,
    file-not-found, wide mode, bits 64/128, large data, units on/off. Any refactor must keep these
    green. streaming.rs has 15 existing `#[test]`s. Python streaming tests live in
    `tests/test_streaming.py` (project root, not in-crate) — follow `test_data_hasher_*` patterns.
