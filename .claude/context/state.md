<!-- assessed-at: b4a31aa -->

# Project State

## Status: IN_PROGRESS

## Phase: CI fix required — Python ruff format check failing after interactive CPython optimisation

All core bindings are functionally complete and an interactive session added CPython C API
optimisations to the Python video functions. However, the `ruff format --check` step now fails in CI
on every push to `develop`, blocking pytest from running. Fixing this is the immediate priority.

## Rust Core Crate

**Status**: met

- 23 Tier 1 public symbols at crate root: 9 `gen_*_v0` functions, 4 text utilities (`text_clean`,
    `text_remove_newlines`, `text_trim`, `text_collapse`), 4 algorithm primitives (`sliding_window`,
    `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`), `soft_hash_video_v0`, `encode_base64`,
    `iscc_decompose`, `DataHasher`, `InstanceHasher`, `conformance_selftest`
- Tier 2 codec module (`codec.rs`) with `MainType`/`SubType`/`Version` enums and all encode/decode
    helpers — correctly Rust-only, not bound to foreign languages
- 216 `#[test]` functions in `src/`; 53 additional tests in `tests/`; 269 total
- `decode_header` and `decode_varnibble_from_bytes` use direct bitwise extraction from `&[u8]` — no
    `Vec<bool>` allocation in any production decode path; `bytes_to_bits` and `bits_to_u32` are
    `#[cfg(test)]`-gated
- `DataHasher::update` uses persistent `buf: Vec<u8>` reused across calls — no `to_vec()` or
    `.concat()` allocations on any update path; tail shift done via `copy_within` + `truncate`
- `bench_data_hasher_streaming` Criterion benchmark added: 1 MB input, 64 KiB chunks, ~1.0 GiB/s
- `soft_hash_video_v0` and `gen_video_code_v0` now generic: `S: AsRef<[i32]> + Ord` — accepts both
    `&[Vec<i32>]` (owned) and `&[&[i32]]` (borrowed); backward-compatible with all binding crates
- `alg_dct`: validation strictly enforces `n.is_power_of_two()` (lengths like 6, 10, 12 rejected); 4
    unit tests covering non-power-of-two cases
- `alg_wtahash`: return type `IsccResult<Vec<u8>>`; validates `vec.len() >= 380` and bit
    constraints; 4 unit tests; `soft_hash_video_v0` propagates error directly
- All conformance vectors from `data.json` pass for every `gen_*_v0` function (CI-verified at HEAD)
- Pure Rust: zero binding dependencies (no PyO3, napi, wasm-bindgen in `iscc-lib`)
- `cargo clippy --workspace --all-targets -- -D warnings` clean (CI-verified at HEAD — Rust job
    passes)
- **Open issues**: none

## Python Bindings

**Status**: partially met (functional; ruff format CI gate broken)

- 23/23 Tier 1 symbols exposed via PyO3 in `crates/iscc-py/src/lib.rs`
- All `gen_*_v0` functions return `PyDict` (translated to typed `IsccResult` subclasses in Python)
- `DataHasher` and `InstanceHasher` as `#[pyclass]` with `Option<inner>` finalize-once pattern
- `gen_data_code_v0` and `gen_instance_code_v0` accept `bytes | bytearray | memoryview | BinaryIO`
    in the Python layer; file-like stream inputs use 64 KiB chunked reads
- `DataHasher.update` and `InstanceHasher.update` accept `bytes | bytearray | memoryview | BinaryIO`
    with same chunked-read logic
- `sliding_window` returns `PyResult<Vec<String>>` and raises `ValueError` on `width < 2`
- `__version__ = version("iscc-lib")` via `importlib.metadata` — present in `__init__.py`
- Module docstring in `crates/iscc-py/src/lib.rs` corrected to `iscc_lib._lowlevel`
- `gen_video_code_v0` and `soft_hash_video_v0` now use direct CPython C API (`PyList_GetItem`,
    `PyLong_AsLong`) for fast extraction from any nested Python sequence (commit `5461a65`)
- Two Python-specific flat-buffer variants added: `gen_video_code_v0_flat` and
    `soft_hash_video_v0_flat` (accept pre-flattened native-endian i32 byte buffers; for
    numpy/array.array callers); stubs added to `_lowlevel.pyi`
- Type signatures for `gen_video_code_v0` / `soft_hash_video_v0` updated from `list[list[int]]` to
    `Sequence[Sequence[int]]` in both `__init__.py` and `_lowlevel.pyi`
- 117 test functions across 5 files; 159 total pytest tests
- `ruff check` passes in CI; **`ruff format --check` FAILS** in CI (both runs 22401304896 and
    22401336439\) — pytest is skipped as a result
- abi3-py310 wheel configuration in place; `ty` type checking configured
- OIDC trusted publishing not yet configured (registry-side setup required)
- **Missing**: Python CI gate broken; must fix ruff format issue before pytest can be confirmed
    green

## Node.js Bindings

**Status**: met

- 23/23 Tier 1 symbols exported via napi-rs in `crates/iscc-napi/src/lib.rs`
- `DataHasher` and `InstanceHasher` implemented as `#[napi(js_name)]` structs
- `sliding_window` returns `napi::Result<Vec<String>>` and throws on `width < 2`
- 103 tests (CI-verified at HEAD); all conformance vectors pass
- Version sync resolved: `package.json` version `0.0.1` matches workspace version; `version:sync`
    script now handles future updates automatically
- npm packaging fixed: `"files"` allowlist ensures correct tarball contents
- **Open issues**: none

## WASM Bindings

**Status**: met

- 23/23 Tier 1 symbols exported via wasm-bindgen in `crates/iscc-wasm/src/lib.rs`
- `DataHasher` and `InstanceHasher` added as `#[wasm_bindgen]` structs
- `sliding_window` propagates `IsccError` as `JsError` on `width < 2`
- 54 tests: 9 in `conformance.rs` + 45 in `unit.rs`; all pass (CI-verified at HEAD)
- `conformance_selftest` gated behind `#[cfg(feature = "conformance")]`
- Browser and Node.js build targets supported
- `crates/iscc-wasm/CLAUDE.md` updated to reflect all 23 Tier 1 symbols + 2 streaming types bound
    (stale "not yet bound" text removed in iteration 28)
- **Open issues**: none

## C FFI

**Status**: met

- 25 exported `extern "C"` functions: 23 Tier 1 symbols + `iscc_alloc` / `iscc_dealloc`
- All streaming hasher types: `FfiDataHasher` and `FfiInstanceHasher` with complete lifecycle
- Finalize-once semantics via `Option<Inner>`; `iscc_sliding_window` propagates error via
    thread-local last-error
- 62 `#[test]` Rust unit tests; C test program covers full lifecycle (tests 14–17)
- cbindgen generates valid C headers; C test program compiles and runs (CI-verified at HEAD)
- Video frame allocation eliminated: `iscc_gen_video_code_v0` uses zero-copy borrows
- **Open issues**: none

## Java Bindings

**Status**: partially met (JNI bridge + Java wrapper + NativeLoader + Maven config + conformance
tests + CI job + how-to guide complete; platform native bundling inside JAR and Maven Central
publishing absent)

- `crates/iscc-jni/` crate: `lib.rs` with all 23 Tier 1 symbols as 29 `extern "system"` JNI
    functions; `throw_and_default` at 68 call sites + `throw_state_error` at 4 call sites (finalized
    hasher errors), zero `unwrap()` calls
- Negative `jint` validation in 3 guards; local reference frame safety in 5 array loops
- `IsccLib.java` (331 lines): 29 `native` declarations, `NativeLoader.load()` static initializer
- `NativeLoader.java` (169 lines): OS/arch detection, JAR extraction to temp, `System.loadLibrary`
    fallback; extraction path inactive (no native binaries bundled yet)
- `IsccLibTest.java`: 9 `@TestFactory` conformance methods (46 vectors) + 3 `@Test` negative-value
    methods + 2 `@Test` `IllegalStateException` hasher-state methods = **51 total tests**; all
    passing
- Java CI job (`Java (JNI build, mvn test)`) passing (CI-verified at HEAD)
- `docs/howto/java.md` (319 lines): complete; navigation entry in `zensical.toml` ✅
- Version: `pom.xml` at `0.0.1` (synced from workspace via `version:sync`)
- Missing: platform-specific native library bundling inside JAR (`META-INF/native/`)
- Missing: Maven Central publishing configuration
- Note: `IsccLib.java` Javadoc still says `@throws IllegalArgumentException` for hasher
    update/finalize methods — cosmetic mismatch, not blocking
- **Open issues**: none

## Go Bindings

**Status**: partially met (23/23 Tier 1 symbols + 35 test functions + Go CI job passing + README
done + howto/go.md done; io.Reader streaming interface absent)

- `packages/go/iscc.go` (1,165 lines): `Runtime` struct + 23 Tier 1 exported symbols
- `DataHasher` / `InstanceHasher` structs with `New*/Update/Finalize/Close` lifecycle
- `packages/go/iscc_test.go` (1,069 lines): 36 function declarations, 35 actual tests covering 46
    conformance vectors, 8 streaming hasher tests, error paths
- `TestMain` skips gracefully if `iscc_ffi.wasm` is not present
- `CGO_ENABLED=0 go test ./...` passes (CI-verified at HEAD)
- `docs/howto/go.md` (388 lines): complete; navigation entry in `zensical.toml` ✅
- Missing: `io.Reader` interface for `Update` methods (target.md verified-when criteria do not
    explicitly require it)

## README

**Status**: met

- Rewritten as public-facing polyglot developer README (238 lines)
- CI badge, DeepWiki badge, Crate, PyPI, npm, and Go Reference version badges
- Experimental notice, tagline, Key Features, ISCC Architecture diagram, MainTypes table
- "What is the ISCC" and "What is iscc-lib" sections; all 6 language bindings mentioned
- Installation and Quick Start for Rust, Python, Node.js, Java, Go, WASM
- Implementors Guide with all 9 `gen_*_v0` entry points listed
- Documentation link, Contributing, Apache-2.0 license, Maintainers
- Maven Central badge not added (Java not yet published to Maven Central; not blocking)

## Per-Crate READMEs

**Status**: met

- `crates/iscc-lib/README.md`, `crates/iscc-py/README.md`, `crates/iscc-napi/README.md` — done
- `crates/iscc-wasm/README.md`, `crates/iscc-jni/README.md`, `packages/go/README.md` — done
- `crates/iscc-ffi/README.md` — created in iteration 29 (123 lines); all 7 per-crate READMEs
    complete

## Documentation

**Status**: met

- 13 pages deployed to lib.iscc.codes: all navigation sections complete (Tutorials, How-to Guides,
    Explanation, Reference, Benchmarks, Development)
- All pages have `icon: lucide/...` and `description:` YAML front matter
- Site builds and deploys via GitHub Pages; latest Docs run: **PASSING**
    ([Run 22395922643](https://github.com/iscc/iscc-lib/actions/runs/22395922643))
- ISCC branding, copy-page split-button, `gen_llms_full.py`, Open Graph meta tags in place
- `docs/CNAME` contains `lib.iscc.codes`; `docs/includes/abbreviations.md` (19 abbreviations)
- `docs/index.md` Quick Start section now has all 6 language tabs: Rust, Python, Node.js, Java, Go,
    WASM (expanded in iteration 30); Available Bindings table includes all 7 entries (Java and Go
    rows added)
- Target requirement "All code examples use tabbed multi-language format" now met for the landing
    page

## Benchmarks

**Status**: partially met

- Criterion benchmarks exist for all 9 `gen_*_v0` functions + `bench_data_hasher_streaming`
- pytest-benchmark comparison files: `benchmarks/python/bench_iscc_lib.py` and
    `benchmarks/python/bench_iscc_core.py` (101 lines each) plus `conftest.py`
- Speedup factors documented in `docs/benchmarks.md`
- Missing: CI does not run benchmarks automatically; no published benchmark results in CI artifacts

## CI/CD and Publishing

**Status**: partially met — **CI FAILING**

- 3 workflows: `ci.yml`, `docs.yml`, `release.yml`
- `ci.yml` covers 7 binding targets: Rust (fmt, clippy, test), Python (ruff, pytest), Node.js (napi
    build, test), WASM (wasm-pack test --features conformance), C FFI (cbindgen, gcc, test), Java
    (JNI build, mvn test), Go (go test, go vet)
- `ci.yml` triggers on push to `main` and `develop` branches and PRs to `main`
- **Latest CI run on develop: FAILING** —
    [Run 22401336439](https://github.com/iscc/iscc-lib/actions/runs/22401336439) — **Python (ruff,
    pytest) FAILING** (ruff format check step fails; pytest skipped); 6 other jobs (Rust, Node.js,
    WASM, C FFI, Java, Go) passing
- Previous completed run on develop: FAILING —
    [Run 22401304896](https://github.com/iscc/iscc-lib/actions/runs/22401304896) — same failure
- Earlier passing run on develop:
    [Run 22396424642](https://github.com/iscc/iscc-lib/actions/runs/22396424642) — all 7 jobs
    success (before interactive session commits)
- Latest Docs run: **PASSING** —
    [Run 22395922643](https://github.com/iscc/iscc-lib/actions/runs/22395922643)
- `release.yml` `workflow_dispatch` with `inputs:` block (three boolean checkboxes) and `if:`
    conditions on all 8 jobs
- **Idempotency checks** on all 4 publish jobs (crates.io, PyPI, npm lib/wasm)
- `scripts/version_sync.py` created (120 lines, stdlib only); reads workspace version from root
    `Cargo.toml`, updates `package.json` and `pom.xml`; `--check` mode exits 1 on mismatch;
    `mise run version:sync` and `mise run version:check` tasks registered in `mise.toml`
- **PR #1 merged**: develop → main; `v0.0.1` release is ready once CI is green again
- Missing: OIDC trusted publishing for crates.io and PyPI not yet configured in registry settings
    (workflow code is correct; registry-side setup is outside CI scope)
- Missing: npm publishing pipeline not fully wired
- Missing: Java platform native bundling in CI matrix
- Missing: Maven Central publishing configuration

## Next Milestone

**CI is FAILING — fix Python ruff format check first.**

The interactive session commit `5461a65` introduced CPython C API optimisations to
`crates/iscc-py/src/lib.rs` and updated `_lowlevel.pyi` / `__init__.py`. The `ruff format --check`
step now fails in CI (confirmed on two consecutive runs: 22401304896 and 22401336439). The handoff
claims `mise run check` passes locally, but the global `uv run ruff format --check` used in CI
fails.

Priority order:

1. **[critical] Fix Python ruff format check** — run `uv run ruff format` on the Python files
    (`crates/iscc-py/python/iscc_lib/`) to apply canonical formatting, then verify
    `ruff format  --check` passes. Commit the fix so CI goes green.
2. **[low] Evaluate unofficial TypeScript port** — check whether `branciard/iscc-core-ts` passes
    `data.json` conformance vectors; document result or mention in docs if conformant
3. **Consider tagging v0.0.1** — main branch is stable; tagging and triggering the release workflow
    is the logical next step once CI is green
