# Advance Agent Memory

Detail lives in topic files: [ci-gates.md](ci-gates.md), UniFFI/Swift/Kotlin (Gradle+JNA):
[uniffi-swift-kotlin.md](uniffi-swift-kotlin.md), [deps-refresh.md](deps-refresh.md),
[wasm-simd.md](wasm-simd.md), [go-idv1.md](go-idv1.md), [release-gate.md](release-gate.md), Archived
phases: [MEMORY-archive.md](MEMORY-archive.md).

**Size budget:** Keep under 140 lines. Move detail to topic files; archive stale entries.

- [issues.md ledger rule](feedback-issues-ledger.md) — never edit issues.md; ledger → handoff Notes

## Code Locations

- Rust core: `crates/iscc-lib/src/` — lib.rs (crate root, Tier 1 re-exports), codec.rs, cdc.rs,
    minhash.rs, simhash.rs, dct.rs, wtahash.rs, utils.rs, streaming.rs, conformance.rs
- Conformance vectors: `crates/iscc-lib/tests/data.json` (iscc-core 1.5.0: 50 `gen_*` + 4
    `encode_seq` + 4 `decode_seq`, outputs `bytes:<hex>`/list). Vendored copies
    (packages/{dotnet,go,kotlin,swift}) gated by `tests/test_vendored_fixtures.py` — any NEW tracked
    `data.json` copy → register in `VENDORED_COPIES` or the test reds
- Edge vectors (215): `crates/iscc-lib/tests/data-edge.json` (spec `specs/edge-vectors.md`), no
    `_metadata`; Rust `tests/test_data_edge.rs` (runtime read, excluded from package; CRAP-clean
    caller table, see ci-gates.md), Python `tests/test_data_edge.py` (schema + lib + iscc_core
    iff-`diverges` + SEED_SET; raw call apart from converter, rejections on raw call only).
    `iscc_decode` output = `[mt, st, vs, li, "bytes:<hex>"]`. Group 1 harnesses DONE (217): Go
    `data_edge_test.go`, Node `__tests__/data_edge.test.mjs`, WASM `tests/data_edge.rs`, JNI
    `DataEdgeTest.java`, .NET `DataEdgeTests.cs` (vendored), Ruby `test/test_data_edge.rb`. Group 2
    (218): Swift/Kotlin `DataEdge*`. Group 3 (219): C/C++ read generated
    `crates/iscc-ffi/tests/data_edge_vectors.h` (`scripts/gen_data_edge_vectors.py`, rows carry
    resolved c/cpp expectation; gate `tests/test_gen_data_edge_vectors.py`). `mise run check` =
    pre-commit only — run `prek run --hook-stage pre-push --files …` too. Sections (226): 6 incl.
    `alg_simhash` — first section with `bytes:<hex>` INPUTS: every harness has a per-section input
    preparer (Py `PREPARERS`, Node/Ruby 3rd tuple slot) applied OUTSIDE the raw call; generator
    `byte_list_arrays` (binary items, explicit lens). fixture =
    `json.dumps(indent=2,   ensure_ascii=False)`. Mutation-check by sed'ing an expected output in
    the copy a harness reads
- GOTCHA: `clippy --all-targets --no-default-features` fails on benches (ungated imports) —
    `--tests`
- C FFI docs (221): C type names `iscc_`-prefixed, gated by `tests/test_c_ffi_docs.py`
- Bindings: Python `crates/iscc-py/python/iscc_lib/__init__.py`; Node `crates/iscc-napi/src/lib.rs`;
    WASM `crates/iscc-wasm/src/lib.rs`; C FFI `crates/iscc-ffi/src/lib.rs`; JNI
    `crates/iscc-jni/src/lib.rs` + `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/`
- Ruby: `crates/iscc-rb/` — src/lib.rs (Magnus), lib/iscc_lib.rb (wrapper); Cargo lib name `iscc_rb`
- UniFFI: `crates/iscc-uniffi/` — proc-macro interface for Swift/Kotlin, `publish = false`. Full
    detail → uniffi-swift-kotlin.md
- Go pure: `packages/go/` — one `.go` per algorithm/code-type (codec, utils, cdc, minhash, simhash,
    dct, wtahash, xxh32, `code_*.go`, conformance.go); pure Go only. ISCC-IDv1, decode guards, gofmt
    caveat → go-idv1.md

## Build and Tooling

- `cargo build -p iscc-jni` before `mvn clean test -f crates/iscc-jni/java/pom.xml` (native prereq;
    plain `mvn test` reuses stale classes). jni 0.22 → deps-refresh.md
- CI `.github/workflows/ci.yml`: 20 jobs (version-check, rust, python-test, python, nodejs, wasm,
    c-ffi, dotnet, java, go, ruby, cpp, swift, kotlin, bench, perf, semver, coverage, audit,
    release-workflow). `bench`=`cargo bench --no-run`; `swift` macos-15 (macos-14 retired
    2026-11-02, iter 196); `kotlin` ubuntu JDK17
- Enforcing gates: `coverage` (CRAP baseline `.crap-baseline.json`), `audit` (cargo-deny), `perf`
    (iai vs `.iai-baseline.json`); `semver` informational until v1.0.0. CRAP gate is CI-ONLY —
    refresh `mise run crap:baseline` in the SAME step as any branch-adding change → ci-gates.md
- Audit: advisory w/ patched release → `cargo update -p <crate>`, never `deny.toml` `ignore`
- Dependency refresh (iters 124-172; ALL authorized majors done through uniffi 0.32): manifests +
    GHA action majors probed current 2026-07-28; ruff 0.16; prek ruff hooks carry `pyi` in
    `types_or`; NEVER blanket `--fix` (deletes 13 `# noqa: S603/S607`); rb_sys pinned `0.9.123`.
    Detail + prek staged-probe gotcha → deps-refresh.md
- Commit quirks (217): Ruby `standardrb --fix` hook takes minutes (long timeout); trust
    `prek run --files` over bare `uv run mdformat`; never edit files while `mise run check` runs.
    WSL `index.lock` race after prek's stash-restore: retry `git commit` in a sleep loop (226)
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

## API Design

- Video API `<S: AsRef<[i32]> + Ord>` generics — FFI passes `&[&[i32]]` (zero-copy), others
    `&[Vec<i32>]`
- Tier 1 `encode_component` wrapper in `lib.rs` takes `u8` enum fields + validates with
    `TryFrom<u8>`; delegates to `codec::encode_component`
- `iscc_decode` strips "ISCC:" + dashes, returns digest bytes `(u8,u8,u8,u8,Vec<u8>)`
- Python strip set `codec::is_py_strip_char` (`pub(crate)`) / Go `isPyStripChar`: codec + text
    trim/split (203). Base32 strict (199) + ASCII-only casing (201)
- Codec input cleaning `iscc_clean` (Rust 191-198; Go `isccClean` 200, 4 sites) + perf →
    codec-clean.md
- ISCC-IDv1 #43 COMPLETE (iter 190, no dedicated decoder; Tier-1 = 35 since 211) → go-idv1.md
- ISCC-SEQ #45: core (195) + all 11 surfaces bound (197-210, UniFFI last); docs swept 33→35 (211) →
    iscc-seq.md. iscc-ffi lib.rs ~2860 lines (split hint 2500)
- `alg_simhash` rejects `[]` (225, core + Go); generators keep 32 zeros via Rust `alg_simhash_inner`
    / Go unexported `algSimhash` — never call the public form from a generator
- `json_to_data_url` combines `parse_meta_json` + `build_meta_data_url`. JCS canonical, media type
    depends on `@context` key. 5 cross-binding constants → MEMORY-archive.md

## Documentation Files / Feature Flags / Spec path gate → MEMORY-archive.md

- Docs page = disk + `zensical.toml` nav + `ORDERED_PAGES` + `docs/llms.txt` (`check_docs_nav.py`)
- Spec path gate `scripts/check_spec_paths.py`: `*` one segment, `**` crosses `/`; `NOT_PATHS`
    backticked spans only (228). Fix phantoms in the spec, never by adding a file

## Ruby Bindings (Magnus) → MEMORY-archive.md (Magnus 0.8 API, never `Vec<T>` params)

## Other Bindings — pointers

- .NET `packages/dotnet/` (P/Invoke over `iscc_ffi`, `dotnet-version: 8.0`); C++ `packages/cpp/`
    (header-only C++17, CMake + ASAN; no system cmake but `uv run --with cmake` works — build into
    gitignored `build-uv/`). Detail → MEMORY-archive.md
- Embedded NUL (213): .NET `ToNativeUtf8` / C++ `detail::checked_c_str` throw; route every new
    string input through them (bare `.c_str()` / `GetBytes` truncates at NUL)
- C++ empty vectors (227): every vector input → `detail::safe_data` (one template, read-only
    sentinel); never `.data()` (`{}` → FFI "must not be NULL" hides the core error)
- .NET jagged arrays (220): `PinnedRows` in `IsccLib.cs` (`fixed` on `T[0]` is NULL → pad). Filter
    tests: `--filter FullyQualifiedName~X`
- npm metadata (214): `@iscc/wasm` package.json comes from `crates/iscc-wasm/Cargo.toml` via
    wasm-pack (0.13.1 resolves `license.workspace`); `@iscc/lib` package.json is committed as-is.
    taplo aligns trailing comments to the longest line in a block — put comments on their own line
