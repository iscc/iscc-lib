# Advance Agent Memory

Codepaths, implementation patterns, library locations, and key decisions accumulated across CID
iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Code Locations

- Rust core: `crates/iscc-lib/src/` — lib.rs (crate root, Tier 1 re-exports), codec.rs, cdc.rs,
    minhash.rs, simhash.rs, dct.rs, wtahash.rs, utils.rs, streaming.rs, conformance.rs
- Conformance vectors: `crates/iscc-lib/tests/data.json` (50 total: 20+5+3+5+3+2+4+3+5, v1.3.0)
- Python wrapper: `crates/iscc-py/python/iscc_lib/__init__.py`
- Node.js: `crates/iscc-napi/src/lib.rs`
- WASM: `crates/iscc-wasm/src/lib.rs`
- C FFI: `crates/iscc-ffi/src/lib.rs`
- JNI: `crates/iscc-jni/src/lib.rs` + `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/`
- Ruby: `crates/iscc-rb/` — src/lib.rs (Magnus bridge), lib/iscc_lib.rb (Ruby wrapper + Result
    classes), lib/iscc_lib/version.rb, extconf.rb, Rakefile, Gemfile, iscc-lib.gemspec,
    test/test_smoke.rb. Cargo lib name `iscc_rb` (not `iscc_lib` — matches package name for rb_sys)
- UniFFI: `crates/iscc-uniffi/` — src/lib.rs (proc macro interface for Swift/Kotlin). 32 Tier 1
    symbols, 11 result Records, IsccUniError enum, DataHasher/InstanceHasher Objects. Uses
    `uniffi::setup_scaffolding!()`, no UDL or build.rs. `publish = false`
- Go pure: `packages/go/` — codec.go, utils.go, cdc.go, minhash.go, simhash.go, dct.go, wtahash.go,
    xxh32.go, code_content_text.go, code_meta.go, code_data.go, code_instance.go,
    code_content_image.go, code_content_audio.go, code_content_video.go, code_content_mixed.go,
    code_iscc.go, conformance.go. WASM bridge removed — pure Go only

## Build and Tooling

- `cargo build -p iscc-jni` must run before `mvn test` (native library prerequisite)
- Maven POM is at `crates/iscc-jni/java/pom.xml` — run `mvn test` from `crates/iscc-jni/java/`
- CI workflow at `.github/workflows/ci.yml` has 15 jobs: version-check, rust, python-test, python,
    nodejs, wasm, c-ffi, dotnet, java, go, ruby, cpp, swift, bench. `bench` runs
    `cargo bench --no-run` (compile-only). `swift` runs on `macos-14` (Apple Silicon)
- Ruby CI job: libclang-dev required, ruby/setup-ruby@v1 `working-directory` is an action `with:`
    param (not step-level), bundler-cache auto-installs gems
- `rust` CI job includes feature matrix testing: clippy + test for `--no-default-features`,
    `--all-features`, and `--no-default-features --features text-processing` (issue #16)
- `version-check` job: lightweight (checkout + setup-python only), runs
    `python scripts/version_sync.py --check` to catch manifest version drift
- Go CI job has zero Rust dependencies — only checkout, setup-go, test, vet (4 steps)
- Version sync: `scripts/version_sync.py` — `--check` mode exits 1 on mismatch
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` for Python dev builds
- Release workflow (`release.yml`): 7 registry inputs (crates-io, pypi, npm, maven, ffi, rubygems,
    nuget). Pattern: boolean input → build job → **smoke test job** → publish job (version-exists
    skip). 7 smoke test jobs gate publish. NuGet reuses `build-ffi` artifacts (shared `if` condition
    `inputs.ffi || inputs.nuget`), then `pack-nuget` → `test-nuget` → `publish-nuget`. NuGet uses
    `NUGET_API_KEY` secret (not OIDC). Ruby uses `GEM_HOST_API_KEY` for auth (not OIDC)

## WASM/WASI

- `iscc-wasm` has `[features] conformance = []` — gates `conformance_selftest` WASM export. Release
    build uses `--features conformance` so smoke test can call `conformance_selftest()`
- wasm-pack `--features` must go AFTER the path, NOT after `--`

## Go Pure Go (Summary)

- Pure Go in `packages/go/` — all 10 gen functions + codec + algorithms. Zero WASM deps
- 156 Go tests total. CI: 4 steps (checkout, setup-go, test, vet) — no Rust deps
- Conformance: `//go:embed testdata/data.json`, `parseConformanceData()` with two-pass parsing

## gen_sum_code_v0

- `gen_sum_code_v0(path: &Path, bits: u32, wide: bool, add_units: bool)` in `lib.rs`. Single-pass
    file I/O, feeds DataHasher + InstanceHasher, composes via `gen_iscc_code_v0`
- `iscc_decode` returns tuple `(u8, u8, u8, u8, Vec<u8>)` — use tuple destructuring, not field
    access. `MainType` is `pub(crate)`, not accessible from test modules
- All 32 Tier 1 symbols implemented. All 7 bindings implement `gen_sum_code_v0`

## Benchmarks

- `crates/iscc-lib/benches/benchmarks.rs` — all 10 `gen_*_v0` + DataHasher streaming + CDC chunks
- `bench_sum_code` uses `tempfile::NamedTempFile` since `gen_sum_code_v0` takes `&Path` (not
    `&[u8]`)
- `tempfile` is a dev-dependency only (workspace dep `tempfile = "3"`)

## Codec Internals

- `decode_header` and `decode_varnibble_from_bytes` operate directly on `&[u8]` with bitwise
    extraction — no intermediate `Vec<bool>`. `get_bit`/`extract_bits` helpers (MSB-first)

## Streaming

- `DataHasher`: persistent `buf: Vec<u8>` reused across `update()` calls. CDC → BLAKE3 chunk hash →
    MinHash pipeline. Tail: `copy_within` + `truncate`. ~1.1 GiB/s at 64 KiB chunks
- `InstanceHasher`: wraps BLAKE3, outputs ISCC multihash format (64-byte digest truncated)

## API Design

- Video API uses `<S: AsRef<[i32]> + Ord>` generics — FFI passes `&[&[i32]]` (zero-copy), other
    bindings pass `&[Vec<i32>]`
- Tier 1 `encode_component` wrapper in `lib.rs` takes `u8` for enum fields + validates with
    `TryFrom<u8>`. Delegates to `codec::encode_component`
- `iscc_decode` strips "ISCC:" prefix and dashes, returns exact digest bytes (not full tail)
- `json_to_data_url` combines `parse_meta_json` + `build_meta_data_url`. JCS canonical, media type
    depends on `@context` key

## Documentation

- Tabbed syntax: `=== "Language"` with 4-space indent, blank line before code block
- Tab order: tutorial (Python, Rust, Node.js, Java, Go, WASM), landing (Rust, Python, ...)
- mdformat reformats JS imports to multi-line style — run format before commit
- `docs/architecture.md` and `docs/development.md` share identical trees — keep in sync
- All 5 Reference pages complete: Rust API, Python API, C FFI, Java API, Ruby API

## Binding Constant Export Patterns

- NAPI: `#[napi(js_name = "CONST_NAME")] pub const CONST_NAME: u32 = iscc_lib::CONST_NAME as u32;`
- WASM: `#[wasm_bindgen(js_name = "CONST_NAME")] pub fn const_name() -> u32 { ... }` (getter fn, not
    const — wasm-bindgen limitation)
- C FFI: `#[unsafe(no_mangle)] pub extern "C" fn iscc_const_name() -> u32 { ... }` + inline
    `#[test]` in same file. cbindgen auto-generates the C header
- NAPI JS tests: `describe('CONST_NAME', () => { it('equals X'); it('is a number'); })`
- WASM tests: `#[wasm_bindgen_test]` in `tests/unit.rs` (requires wasm-pack to run)
- C tests: `ASSERT_EQ(iscc_const_name(), value, "label")` in `tests/test_iscc.c`
- 5 constants currently exported: META_TRIM_NAME, META_TRIM_DESCRIPTION, META_TRIM_META,
    IO_READ_SIZE, TEXT_NGRAM_SIZE

## Documentation Files

- Howto guides: `docs/howto/{rust,python,ruby,nodejs,wasm,go,java,dotnet,c-cpp}.md`
- API reference: `docs/{rust-api,api,c-ffi-api,java-api,ruby-api}.md`
- Per-package READMEs: `packages/dotnet/README.md`, `packages/cpp/README.md`
- zensical.toml nav: howto order is Rust, Python, Ruby, Node.js, WASM, Go, Java, C#/.NET, C/C++
- `scripts/gen_llms_full.py`: generates `site/llms-full.txt` + per-page `.md` files. Uses
    `ORDERED_PAGES` list + auto-discovery (`discover_pages()`). Excludes `docs/includes/`. Run after
    `zensical build` in docs CI pipeline

## Feature Flags

- `crates/iscc-lib/Cargo.toml` defines: `default = ["meta-code"]`, `text-processing` (unicode deps),
    `meta-code` (implies text-processing + JCS canonicalizer)
- `text-processing` gates: `text_clean`, `text_collapse`, `gen_text_code_v0`, `sliding_window_strs`
- `meta-code` gates: META_TRIM constants, meta helpers, `gen_meta_code_v0`, `json_to_data_url`,
    `run_meta_tests` in conformance, `sliding_window_bytes`
- `conformance` module is always available (not feature-gated). `conformance_selftest()` skips
    disabled code types (meta, text) via `#[cfg]` blocks — does not fail for missing features
- When gating `pub(crate)` functions, their tests must also be gated — dead-code lint fires in
    library builds even if test modules use them
- Integration tests in `crates/iscc-lib/tests/test_text_utils.rs` also need per-function gating
- `serde_json` stays as a regular (non-optional) dep because `conformance.rs` uses it for parsing
    `data.json`. Gating it requires restructuring conformance (future work)

## Ruby Bindings (Magnus) — see MEMORY-archive.md for full details

- Magnus 0.7.1 (not 0.8) — Ruby 3.1 compat. `function!` macro: no `&Ruby` param, use `Ruby::get()`
- rb_sys: `ExtensionTask.new("iscc-rb")` — task name = Cargo package name. `extconf.rb` at crate
    root
- 32/32 Tier 1 symbols exposed. 111 tests (61 unit + 50 conformance)
- Streaming: `RefCell<Option<inner>>` for one-shot finalize. `_` prefix for methods, NOT class names
- Linting: Standard Ruby + rubocop-minitest. Pre-commit hook needs portable PATH for `bundle`

## .NET Bindings (P/Invoke) — Summary (details in MEMORY-archive.md)

- Package: `packages/dotnet/Iscc.Lib/` + `packages/dotnet/Iscc.Lib.Tests/`
- 32/32 Tier 1 symbols. P/Invoke over `iscc_ffi` shared library
- `dotnet test` requires `-e LD_LIBRARY_PATH=<path>` (absolute, not relative)
- CI: `actions/setup-dotnet@v4`, `dotnet-version: '8.0'`
- Empty span fix for 7 functions (same pattern as C++ `safe_data`)

## C++ Bindings (Header-Only) — see MEMORY-archive.md for full details

- `packages/cpp/` — header-only C++17, depends on `iscc-ffi`. CMake build + ASAN tests
- `detail::safe_data()` returns non-null sentinel for empty vectors (C FFI rejects nullptr)
- Package manager manifests: `vcpkg.json` + `portfile.cmake` + `conanfile.py`

## UniFFI Bindings (Swift/Kotlin)

- `crates/iscc-uniffi/` — shared scaffolding crate, `uniffi = "0.31"` (workspace dep)
- Proc macro approach only: `#[uniffi::export]`, `#[derive(uniffi::Record)]`,
    `#[derive(uniffi::Object)]`, `#[uniffi::constructor]`. No UDL files, no build.rs
- `crate-type = ["cdylib", "staticlib", "lib"]` — cdylib for dynamic, staticlib for XCFramework
- Error: `#[derive(uniffi::Error)] enum IsccUniError` with `From<iscc_lib::IsccError>` impl
- Streaming: `Mutex<Option<Inner>>` pattern (same as Ruby's `RefCell<Option<Inner>>` but
    thread-safe)
- UniFFI doesn't support: `const` exports (use getter fns), `usize` (use u64), borrowed refs (owned)
- Result records need `Debug` derive for test `unwrap_err()`. Hashers need `Default` impl (clippy)
- 21 unit tests in-crate. Conformance testing happens in Swift/Kotlin test suites
- Binding generation: `uniffi-bindgen.rs` (3-line main), `[features] bindgen = ["uniffi/cli"]`,
    `[[bin]] required-features = ["bindgen"]`
- Generate Swift:
    `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen -- generate   --library target/debug/libiscc_uniffi.so --language swift --out-dir <dir>`
- Generated files: `iscc_uniffi.swift` (~72KB), `iscc_uniffiFFI.h` (~38KB),
    `iscc_uniffiFFI.modulemap` (rename to `module.modulemap` for SPM)

## Swift Package

- `packages/swift/` — SPM package with IsccLibFFI (C header + modulemap) + IsccLib (Swift bindings)
- Conformance tests: `ConformanceTests.swift` — 9 test methods, 50 vectors. Requires macOS runner
- CI job (`swift:`) on `macos-14`: `cargo build -p iscc-uniffi` → `swift build` → `swift test` with
    `-Xlinker -L` (link-time) and `-Xlinker -rpath` (runtime) pointing to `target/debug`
- `module.modulemap` simplified from generated version (removed Darwin-specific `use` directives)

## Gotchas

- Ruby constants must start with uppercase — `_DataHasher` is NOT a valid constant name
- After adding new symbols to `crates/iscc-py/src/lib.rs`, MUST rebuild the `.so` with
    `uv run maturin develop -m crates/iscc-py/Cargo.toml` before `pytest` will work
