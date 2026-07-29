# Advance Agent Memory

Detail lives in topic files: [ci-gates.md](ci-gates.md),
[uniffi-swift-kotlin.md](uniffi-swift-kotlin.md), [deps-refresh.md](deps-refresh.md),
[wasm-simd.md](wasm-simd.md), [go-idv1.md](go-idv1.md), [release-gate.md](release-gate.md),
[unicode-freeze.md](unicode-freeze.md). Archived phases: [MEMORY-archive.md](MEMORY-archive.md).

**Size budget:** Keep under 140 lines. Move detail to topic files; archive stale entries.

- [issues.md ledger rule](feedback-issues-ledger.md) — never edit issues.md; ledger → handoff Notes

## Code Locations

- Rust core: `crates/iscc-lib/src/` — lib.rs (crate root, Tier 1 re-exports), codec.rs, cdc.rs,
    minhash.rs, simhash.rs, dct.rs, wtahash.rs, utils.rs, streaming.rs, conformance.rs
- Conformance vectors: `crates/iscc-lib/tests/data.json` (50 total, v1.3.0). Vendored copies
    (packages/{dotnet,go,kotlin,swift}) gated by `tests/test_vendored_fixtures.py` — any NEW tracked
    `data.json`/`unicode_boundary.json` copy → register in `VENDORED_COPIES` or the test reds
- Unicode 16.0.0 freeze rule (iter 148): `text_clean`/`text_collapse` MAP 16.0-unassigned code
    points to `UNASSIGNED_SENTINEL` (`U+FFFF`) pre-normalization; unchanged category-`C` filter
    removes it. NOT delete-filter, NOT category override. Case freeze (156): `Final_Sigma` via
    `to_lowercase_unicode16` (vendored `utils/unicode16_case.rs`) — never bare
    `str::to_lowercase()`. Design, table regen, boundary fixture → unicode-freeze.md
- Bindings: Python `crates/iscc-py/python/iscc_lib/__init__.py`; Node `crates/iscc-napi/src/lib.rs`;
    WASM `crates/iscc-wasm/src/lib.rs`; C FFI `crates/iscc-ffi/src/lib.rs`; JNI
    `crates/iscc-jni/src/lib.rs` + `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/`
- Ruby: `crates/iscc-rb/` — src/lib.rs (Magnus bridge), lib/iscc_lib.rb (wrapper + Result classes).
    Cargo lib name `iscc_rb` (not `iscc_lib` — matches package name for rb_sys)
- UniFFI: `crates/iscc-uniffi/` — proc-macro interface for Swift/Kotlin, `publish = false`. Full
    detail → uniffi-swift-kotlin.md
- Go pure: `packages/go/` — one `.go` per algorithm/code-type (codec, utils, cdc, minhash, simhash,
    dct, wtahash, xxh32, `code_*.go`, conformance.go); pure Go only. ISCC-IDv1, decode guards, gofmt
    caveat → go-idv1.md

## Build and Tooling

- `cargo build -p iscc-jni` before `mvn clean test -f crates/iscc-jni/java/pom.xml` (native prereq;
    plain `mvn test` reuses stale classes). jni 0.22 → deps-refresh.md
- CI `.github/workflows/ci.yml`: 21 jobs (version-check, rust, python-test, python, nodejs, wasm,
    c-ffi, dotnet, java, go, ruby, cpp, swift, kotlin, bench, perf, semver, coverage, audit,
    release-workflow, unicode-sweep). `bench`=`cargo bench --no-run`; `swift` macos-14; `kotlin`
    ubuntu JDK17; `unicode-sweep`=CPython 3.14 (bare `uv run scripts/unicode_sweep.py` fails closed
    without `--rebuilt` → unicode-freeze.md)
- Enforcing gates: `coverage` (CRAP baseline `.crap-baseline.json`), `audit` (cargo-deny), `perf`
    (iai vs `.iai-baseline.json`); `semver` informational until v1.0.0. CRAP gate is CI-ONLY —
    refresh `mise run crap:baseline` in the SAME step as any branch-adding change → ci-gates.md
- Audit: advisory w/ patched release → `cargo update -p <crate>`, never `deny.toml` `ignore`
- Dependency refresh (iters 124-172; ALL authorized majors done through uniffi 0.32): manifests +
    GHA action majors probed current 2026-07-28; ruff 0.16; prek ruff hooks carry `pyi` in
    `types_or`; NEVER blanket `--fix` (deletes 13 `# noqa: S603/S607`); rb_sys pinned `0.9.123`.
    Detail + prek staged-probe gotcha → deps-refresh.md
- GOTCHAs: ci.yml `cancel-in-progress: true` per ref — a second develop push cancels the previous
    sha's in-flight CI run; system `python3` lacks PyYAML (use `uv run python`); piping cargo-crap
    or cargo-deny to a pager makes `$?` the pager's exit — redirect to a file instead
- Ruby CI job: libclang-dev required; ruby/setup-ruby@v1 `working-directory` is a `with:` param.
    `rust` job matrix: `--no-default-features` / `--all-features` / `--features text-processing`
- Adding/renaming a ci.yml job requires updating the job table in `.claude/context/specs/ci-cd.md`
    same commit — parity-gated by `scripts/check_ci_job_table.py` (prek `check-ci-job-table` +
    pytest anchor; backticked first-column keys, count floor 10)
- `version-check` job: `scripts/version_sync.py --check` (16 targets; exits 1 on mismatch). Go CI
    job has zero Rust deps
- Python dev builds: `uv run maturin develop -m crates/iscc-py/Cargo.toml` (always via uv run).
    Single `cp310-abi3` wheel. GOTCHA: `uv sync` uninstalls the editable ext — re-run maturin after
- PyO3 pin = single source: root `Cargo.toml` (`pyo3` "0.29", `abi3-py310`); ONLY `crates/iscc-py`
    consumes it. KEEP `#[pymodule(name="_lowlevel", gil_used=true)]` (lib.rs:697) — 0.28+ defaults
    `gil_used` false, unsafe for raw `PyList_GetItem` ptrs. GIL release (#39/#41): 12 `py.detach`
    sites; video detach opens after frame-sig extraction. Recipe → MEMORY-archive.md
- release.yml: static gate `scripts/check_release_workflow.py` (prek hook, opt-in
    `--check-action-inputs`) + shape (9 inputs, 97 `uses:` refs) — keep green → release-gate.md
- WASM SIMD (#42): `blake3/wasm32_simd` + RUSTFLAGS simd128 + wasm-opt `--enable-simd` →
    wasm-simd.md

## Benchmarks

- Two benches in `crates/iscc-lib/benches/`, both `harness = false`: `benchmarks.rs` (criterion 0.8,
    wall-clock; use `std::hint::black_box` — `criterion::black_box` fails clippy) + `iai_benches.rs`
    (iai-callgrind 0.16.1). iai needs valgrind + `iai-callgrind-runner` (NOT preinstalled;
    `mise run bench:iai:check`) → ci-gates.md

## Streaming

- `DataHasher`: persistent `buf` reused across `update()`; CDC → BLAKE3 chunk hash → MinHash; ~1.1
    GiB/s. `InstanceHasher`: BLAKE3 → ISCC multihash (64-byte digest truncated)
- `SumHasher` (#37): inner `DataHasher` + `InstanceHasher`; `finalize(bits, wide, add_units)`
    composes via `gen_iscc_code_v0`. Path `iscc_lib::streaming::SumHasher` (NOT crate-root). All 7
    bindings implement `gen_sum_code_v0` → MEMORY-archive.md

## API Design

- Video API `<S: AsRef<[i32]> + Ord>` generics — FFI passes `&[&[i32]]` (zero-copy), others
    `&[Vec<i32>]`
- Tier 1 `encode_component` wrapper in `lib.rs` takes `u8` enum fields + validates with
    `TryFrom<u8>`; delegates to `codec::encode_component`
- `iscc_decode` strips "ISCC:" prefix + dashes, returns exact digest bytes as
    `(u8,u8,u8,u8,Vec<u8>)`
- Rust codec input-cleaning (iter 191): private `codec::iscc_clean(&str)->IsccResult<String>` ports
    `iscc_core.codec.iscc_clean` — trims ws, case-insensitive `iscc:` scheme, strips dashes UNLESS
    first char is multibase prefix (`f/b/v/z/u`). Routed through 4 sites: `iscc_decompose`,
    `iscc_normalize`, `gen_mixed_code_v0`, `gen_iscc_code_v0` (`cleaned` is now `Vec<String>`). Go
    half of the same issue is NOT done (packages/go isccNormalize etc.). `iscc_clean` is
    `pub(crate)` → its dedicated unit tests live in codec.rs, not integration tests
- ISCC-IDv1 #43 COMPLETE (iter 190): Part 1 codec `Version` V1 (174) + Part 2 minting on all 11
    surfaces (178-188) + Tier-1 32→33 doc/count sweep + per-surface API/howto entries (190). Tier-1
    count is now 33 in ALL shipped-artifact docs (canonical breakdown `specs/rust-core.md:693`); no
    dedicated IDv1 decoder anywhere — decode via generic `iscc_decode` + bit-math `ts=n>>12`,
    `hub=n&0xFFF`, `realm=subtype` on 8-byte BE body. Signatures + go detail → go-idv1.md
- `json_to_data_url` combines `parse_meta_json` + `build_meta_data_url`. JCS canonical, media type
    depends on `@context` key. 5 cross-binding constants → MEMORY-archive.md

## Documentation Files

- Tabbed syntax: `=== "Language"` 4-space indent, blank line before code block. Landing-page tab
    order (11): Python, Rust, Ruby, Node.js, WASM, Go, Java, C#, C++, Swift, Kotlin; zensical.toml
    nav order differs (nav starts Rust, Python)
- Howto guides `docs/howto/{lang}.md`; API refs
    `docs/{rust-api,api,c-ffi-api,java-api,ruby-api}.md`; per-package READMEs + CLAUDE.md under
    `packages/{dotnet,cpp,swift,kotlin}/`; `docs/unicode.md` = user-facing Unicode 16.0.0
    freeze-rule page (nav Explanation + `ORDERED_PAGES` + llms.txt)
- `scripts/gen_llms_full.py`: generates `site/llms-full.txt` + per-page `.md`; run after the
    `zensical build` step in docs CI
- Adding a docs page = update `zensical.toml` nav + `ORDERED_PAGES` + `docs/llms.txt` together —
    enforced by `scripts/check_docs_nav.py` (prek `check-docs-nav` + pytest anchor; disk set is the
    reference, `includes/` allowlisted; nav parse is comment-aware `strip_toml_comments` — naive
    `#.*$` breaks on `"C# / .NET"`; prek `files:` misses `git rm`)

## Feature Flags

- `crates/iscc-lib/Cargo.toml`: `default = ["meta-code"]`; `text-processing` gates `text_clean`,
    `text_collapse`, `gen_text_code_v0`, `sliding_window_strs`; `meta-code` (implies text-processing
    \+ JCS) gates META_TRIM consts, meta helpers, `gen_meta_code_v0`, `json_to_data_url`,
    `run_meta_tests`
- `conformance` module always available; `conformance_selftest()` skips disabled types via `#[cfg]`;
    `serde_json` non-optional. When gating `pub(crate)` fns, gate their tests too (dead-code lint
    fires in library builds; `tests/test_text_utils.rs` needs per-function gating)

## Ruby Bindings (Magnus) — full details → MEMORY-archive.md

- Magnus 0.8 (iter 167; Ruby 3.0-3.4, MSRV 1.65); `function!` no `&Ruby` param — use `Ruby::get()`.
    0.8 dropped `old-api`: use `ruby.exception_runtime_error()` + `ruby.str_from_slice()`, never
    `magnus::exception::*` / `RString::from_slice`. `ExtensionTask.new("iscc-rb")` = Cargo pkg name;
    streaming `RefCell<Option<inner>>` one-shot finalize; `_` prefix for methods NOT class names

## Other Bindings — pointers

- .NET `packages/dotnet/` (P/Invoke over `iscc_ffi`, 32/32 Tier 1, `dotnet-version: 8.0`); C++
    `packages/cpp/` (header-only C++17, CMake + ASAN; no system cmake but `uv run --with cmake`
    works — build into gitignored `build-uv/`). Detail → MEMORY-archive.md
- UniFFI scaffolding, Swift package, Kotlin/JVM (Gradle + JNA) → uniffi-swift-kotlin.md
