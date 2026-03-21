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
- CI workflow at `.github/workflows/ci.yml` has 16 jobs: version-check, rust, python-test, python,
    nodejs, wasm, c-ffi, dotnet, java, go, ruby, cpp, swift, kotlin, bench. `bench` runs
    `cargo bench --no-run` (compile-only). `swift` runs on `macos-14` (Apple Silicon). `kotlin` runs
    on `ubuntu-latest` with JDK 17 + `cargo build -p iscc-uniffi` + `./gradlew test`
- Ruby CI job: libclang-dev required, ruby/setup-ruby@v1 `working-directory` is an action `with:`
    param (not step-level), bundler-cache auto-installs gems
- `rust` CI job includes feature matrix testing: clippy + test for `--no-default-features`,
    `--all-features`, and `--no-default-features --features text-processing` (issue #16)
- `version-check` job: lightweight (checkout + setup-python only), runs
    `python scripts/version_sync.py --check` to catch manifest version drift
- Go CI job has zero Rust dependencies — only checkout, setup-go, test, vet (4 steps)
- Version sync: `scripts/version_sync.py` — 15 targets (incl. Swift, Kotlin). `--check` mode exits 1
    on mismatch
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` for Python dev builds
- Release workflow (`release.yml`): 8 registry inputs (crates-io, pypi, npm, maven, ffi, rubygems,
    nuget, maven-kotlin). Pattern: boolean input → build job → **smoke test job** → publish job
    (version-exists skip). NuGet uses `NUGET_API_KEY` secret (not OIDC). Ruby uses OIDC
- Kotlin Maven Central: `build-kotlin-native` (5-platform matrix) → `assemble-kotlin` +
    `test-kotlin-release` → `publish-maven-kotlin`. Publish uses Gradle `maven-publish` to local
    staging dir + curl bundle upload to Sonatype Central Portal REST API. Signing via
    `useInMemoryPgpKeys` (env vars `SIGNING_KEY`/`SIGNING_PASSWORD`)

## WASM/WASI

- wasm-pack `--features` must go AFTER the path, NOT after `--`

## gen_sum_code_v0

- `gen_sum_code_v0(path: &Path, bits: u32, wide: bool, add_units: bool)` in `lib.rs`. Single-pass
    file I/O, feeds DataHasher + InstanceHasher, composes via `gen_iscc_code_v0`
- `iscc_decode` returns tuple `(u8, u8, u8, u8, Vec<u8>)` — use tuple destructuring, not field
    access. `MainType` is `pub(crate)`, not accessible from test modules
- All 32 Tier 1 symbols implemented. All 7 bindings implement `gen_sum_code_v0`

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

## Binding Constant Export Patterns — see MEMORY-archive.md for per-binding details

- 5 constants exported: META_TRIM_NAME, META_TRIM_DESCRIPTION, META_TRIM_META, IO_READ_SIZE,
    TEXT_NGRAM_SIZE

## Documentation Files

- Howto guides: `docs/howto/{rust,python,ruby,nodejs,wasm,go,java,dotnet,c-cpp,swift,kotlin}.md`
- API reference: `docs/{rust-api,api,c-ffi-api,java-api,ruby-api}.md`
- Per-package READMEs: `packages/dotnet/README.md`, `packages/cpp/README.md`,
    `packages/swift/README.md`, `packages/kotlin/README.md`
- Per-package CLAUDE.md: `packages/dotnet/CLAUDE.md`, `packages/swift/CLAUDE.md`,
    `packages/kotlin/CLAUDE.md`
- zensical.toml nav: howto order is Rust, Python, Ruby, Node.js, WASM, Go, Java, C#/.NET, C/C++,
    Swift, Kotlin
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

## .NET / C++ Bindings — see MEMORY-archive.md for full details

- .NET: `packages/dotnet/` — P/Invoke over `iscc_ffi`, 32/32 Tier 1 symbols, `dotnet-version: 8.0`
- C++: `packages/cpp/` — header-only C++17, depends on `iscc-ffi`, CMake + ASAN tests

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

- Two `Package.swift` files coexist: root (for SPM consumers adding the repo URL) and
    `packages/swift/Package.swift` (for CI and local dev). SPM always reads root for dependency
    resolution; `cd packages/swift && swift build` uses the subdirectory one. No conflict
- Root `Package.swift` omits testTarget — tests stay in `packages/swift/` for CI only
- Version constant: `packages/swift/Sources/IsccLib/Constants.swift` — `public let isccLibVersion`
    synced by `scripts/version_sync.py`
- Conformance tests: `ConformanceTests.swift` — 9 test methods, 50 vectors. Requires macOS runner
- CI job (`swift:`) on `macos-14`: `cargo build -p iscc-uniffi` → `swift build` → `swift test` with
    `-Xlinker -L` (link-time) and `-Xlinker -rpath` (runtime) pointing to `target/debug`
- `module.modulemap` simplified from generated version (removed Darwin-specific `use` directives)
- Swift install docs reference version `0.3.1` (when Swift package was first added)

## Kotlin Bindings (UniFFI/JVM)

- `packages/kotlin/` — Gradle JVM project, UniFFI-generated Kotlin via JNA
- Generated file: `src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt` (~3217 lines,
    `package uniffi.iscc_uniffi`). Do NOT manually edit — regenerate via uniffi-bindgen
- Generate Kotlin:
    `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen -- generate   --language kotlin --no-format --out-dir packages/kotlin/src/main/kotlin/   target/debug/libiscc_uniffi.so`
- Gradle wrapper must be bootstrapped AFTER settings.gradle.kts exists (fails without it)
- Gradle 8.12.1 via mise, Kotlin 2.1.10, JNA 5.16.0
- `build/` covered by root `.gitignore`; `.gradle/` needs local `.gitignore`
- JNA native lib loading: `java.library.path` alone is NOT sufficient for JNA `Native.register()`.
    Must also set `jna.library.path` JVM property AND `LD_LIBRARY_PATH` env var in test task
- Conformance tests: `ConformanceTest.kt` — 9 test methods, 50 vectors. JUnit 5 + Gson for JSON
- Test deps: JUnit 5.11.4, Gson 2.11.0 (`com.google.code.gson` groupId, NOT `com.google.gson`)
- Maven Central publishing: `build.gradle.kts` has `maven-publish` + `signing` plugins, POM with
    `io.iscc:iscc-lib-kotlin`. Staging repo at `build/staging-deploy/`. Central Portal bundle upload
    via curl (`https://central.sonatype.com/api/v1/publisher/upload?publishingType=AUTOMATIC`)
- JNA resource paths for bundled native libs: `linux-x86-64`, `linux-aarch64`, `darwin-aarch64`,
    `darwin-x86-64`, `win32-x86-64` (matches JNA `Platform.RESOURCE_PREFIX`). JNA discovers libs
    from classpath even when `jna.library.path` points to a missing directory

## Gotchas

- Ruby constants must start with uppercase — `_DataHasher` is NOT a valid constant name
- After adding new symbols to `crates/iscc-py/src/lib.rs`, MUST rebuild the `.so` with
    `uv run maturin develop -m crates/iscc-py/Cargo.toml` before `pytest` will work
- Gradle `wrapper` task requires `settings.gradle.kts` to exist first — create it before running
