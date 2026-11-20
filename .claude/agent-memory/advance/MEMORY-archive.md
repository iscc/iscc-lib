# Advance Agent Memory — Archive

Archived implementation patterns from completed project phases. Moved here to reduce per-invocation
context loading. Full history preserved in git.

## .NET Bindings (P/Invoke) — Detailed (archived iteration 10)

- Package: `packages/dotnet/Iscc.Lib/` (class library) + `packages/dotnet/Iscc.Lib.Tests/` (xUnit)
- P/Invoke DLL name: `"iscc_ffi"` — .NET resolves to `libiscc_ffi.so` / `iscc_ffi.dll` / `.dylib`
- `[return: MarshalAs(UnmanagedType.U1)]` required for C `bool` → C# `bool` marshaling
- `CallingConvention.Cdecl` matches Rust's `extern "C"`
- `dotnet test` requires `-e LD_LIBRARY_PATH=<path>` to pass lib path to vstest host child process
- csbindgen (v1.9.7) generates `NativeMethods.g.cs`. `NativeMethods` is `internal`
- `IsccLib.cs` wrappers: PascalCase public methods, 4 private + 2 internal helpers
- Streaming: SafeHandle + IDisposable pattern, `_finalized` bool for one-shot semantics
- `GCHandle.Alloc(GCHandleType.Pinned)` for jagged arrays
- Empty span fix for 7 functions: GenAudioCodeV0, GenDataCodeV0, GenInstanceCodeV0, GenImageCodeV0,
    AlgMinhash256, AlgCdcChunks, EncodeBase64
- `packages/dotnet/Iscc.Lib.Tests/ConformanceTests.cs` — 9 `[Theory]` + `[MemberData]` tests
- `Results.cs`: 11 sealed records (9 gen + SumCodeResult + DecodeResult)

See MEMORY.md for current active entries.

## Archived 2026-03-21 — Binding Constant Export Patterns (per-binding details)

- NAPI: `#[napi(js_name = "CONST_NAME")] pub const CONST_NAME: u32 = iscc_lib::CONST_NAME as u32;`
- WASM: `#[wasm_bindgen(js_name = "CONST_NAME")] pub fn const_name() -> u32 { ... }` (getter fn)
- C FFI: `#[unsafe(no_mangle)] pub extern "C" fn iscc_const_name() -> u32 { ... }` + inline test
- NAPI JS tests: `describe('CONST_NAME', () => { it('equals X'); it('is a number'); })`
- WASM tests: `#[wasm_bindgen_test]` in `tests/unit.rs` (requires wasm-pack to run)
- C tests: `ASSERT_EQ(iscc_const_name(), value, "label")` in `tests/test_iscc.c`

## Archived 2026-03-02 — Documentation Sweep Patterns

- "N gen" count references exist in: READMEs (9 files), docs/ (14 files), howto/ (6 files), crate
    CLAUDE.md files (5), notes/ (2), source comments (.rs, .py, .mjs, .pyi), benchmarks/ (2)
- The Edit tool requires a full Read call (not offset/limit) before the first edit per file
- mdformat auto-reformats after edits — always run `mise run format` twice after doc changes
- iscc-core-ts is external and may have different function counts than iscc-lib

## Archived 2026-03-02 — C FFI Examples

- `crates/iscc-ffi/examples/iscc_sum.c` — streaming ISCC-SUM example (read file → dual hashers →
    compose → print). C89/C99 compatible style (variables declared at block start)
- `crates/iscc-ffi/examples/CMakeLists.txt` — minimal cmake build targeting `iscc_ffi` library
- gcc compile:
    `gcc -o out iscc_sum.c -I crates/iscc-ffi/include -L target/debug -liscc_ffi -lpthread -ldl -lm`
- Run: `LD_LIBRARY_PATH=target/debug ./out <filepath>`

## Archived 2026-03-02 — C FFI Release Artifacts

- `release.yml` has `build-ffi` (5-platform matrix) + `publish-ffi` (uploads to GitHub Releases)
- Trigger: `startsWith(github.ref, 'refs/tags/v') || inputs.ffi` (same pattern as other jobs)
- Tarball naming: `iscc-ffi-v{version}-{target}.tar.gz` (Unix), `.zip` (Windows)
- Windows includes 3 files: `iscc_ffi.dll`, `iscc_ffi.dll.lib` (import lib), `iscc_ffi.lib` (static)
- Unix includes 2 files: shared lib + static lib. Both also include `iscc.h` + `LICENSE`
- `publish-ffi` needs `contents: write` (top-level is `contents: read`)
- Uses `softprops/action-gh-release@v2` with tag_name ternary for tag push vs manual dispatch

## Archived 2026-03-05 — Ruby Bindings (Magnus) Full Details

- Root `.gitignore` has `lib/` pattern — Ruby crate needs `!lib/` negation in `.gitignore`
- Bundler: local vendor path (`bundle config set --local path vendor/bundle`)
- PATH: `/home/dev/.local/share/gem/ruby/3.1.0/bin` must be in PATH for bundle commands
- `bundle exec rake compile` builds release profile (rb_sys `RB_SYS_CARGO_PROFILE`)
- Gen functions: `_` prefix in Rust bridge, Ruby wrapper provides keyword-arg public API
- Ruby `Result < Hash` enables `result["iscc"]` and `result.iscc` via `method_missing`
- Constants: `module.const_set("NAME", value)` in Magnus init
- Binary data: `RString` param + `unsafe { data.as_slice() }` — copy bytes before Ruby API calls
- Returning arrays: `ruby.ary_new_capa(n)` + `arr.push(val)?` for mixed-type arrays
- Test files: `test/test_smoke.rb`, `test/test_iscc_lib.rb`, `test/test_conformance.rb`

## gen_sum_code_v0 + Streaming GIL detail — Detailed (archived iteration 93)

- `gen_sum_code_v0(path: &Path, bits: u32, wide: bool, add_units: bool)` in `lib.rs`: thin file-I/O
    wrapper — reads `IO_READ_SIZE` chunks into one `streaming::SumHasher`, then
    `hasher.finalize(bits, wide, add_units)`. Composition logic lives solely in `SumHasher`.
- `iscc_decode` returns tuple `(u8,u8,u8,u8,Vec<u8>)` — destructure; `MainType` is `pub(crate)`.
- All 32 Tier 1 symbols implemented; all 7 bindings implement `gen_sum_code_v0`.
- Python GIL release (issue #39, iter 91, closed): 3 streaming `update()` + 4 one-shot byte funcs
    (`gen_image/data/instance/sum_code_v0`) wrap compute in `py.allow_threads(|| ...)`. `update`
    gains injected `py: Python<'_>` (no `.pyi` change); borrow `&mut inner` BEFORE release. Borrowed
    slice and core hashers are `Ungil+Send` (no copy); `finalize` stays GIL-held.

## Release-Job CI Details (archived iter 96 — niche publishing internals)

- `build-xcframework` job: macOS-14, `contents: write`, no `needs` deps. Provenance guard (tag-only)
    fails if main HEAD != tag SHA. Builds XCFramework → checksum → `sed` updates Package.swift →
    auto-commit → force-update tag → upload to GH Release. Uses macOS BSD `sed -E -i ''` (not GNU).
    Dual cache: `Swatinem/rust-cache` + `actions/cache` (key from crate sources/Cargo manifests)
- Kotlin Maven Central: `build-kotlin-native` (9-platform matrix) → `assemble-kotlin` +
    `test-kotlin-release` (validates JAR has all 9 JNA paths) → `publish-maven-kotlin` (Gradle
    `maven-publish` + curl bundle upload to Sonatype Central Portal REST API)
- Kotlin Maven Central publishing: `build.gradle.kts` `maven-publish` + `signing`, POM
    `io.iscc:iscc-lib-kotlin`, staging `build/staging-deploy/`, Central Portal curl bundle upload
- Kotlin JNA resource paths (9, bundled native libs): `linux-x86-64`, `linux-aarch64`,
    `darwin-aarch64`, `darwin-x86-64`, `win32-x86-64`, `android-{aarch64,arm,x86-64,x86}`. JNA
    5.16.0 canonicalizes ARM32 to `arm` (see learnings.md); discovers libs from classpath

## Release / npm packaging (archived iter 97 — stable, also in learnings.md)

- npm `@iscc/lib` (issue #38, iter 92): BUNDLED single-package — ships all 5 `.node` via
    `files: ["*.node"]`, NO `optionalDependencies`/sibling packages. `publish-npm-lib` must NOT run
    `napi prepublish -t npm` (injects dangling optional-deps that break `npm ci`). `index.js` loader
    requires local `./iscc-lib.<triple>.node` first

## Kotlin Bindings detail (archived iter 100 — stable/complete binding)

- Generated file `src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt` (~3217 lines,
    `package uniffi.iscc_uniffi`). Do NOT manually edit — regenerate via uniffi-bindgen
- Generate Kotlin:
    `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen -- generate --language kotlin --no-format --out-dir packages/kotlin/src/main/kotlin/ target/debug/libiscc_uniffi.so`
- Gradle wrapper must be bootstrapped AFTER settings.gradle.kts exists (fails without it)
- Gradle 8.12.1 via mise, Kotlin 2.1.10, JNA 5.16.0
- `build/` covered by root `.gitignore`; `.gradle/` needs local `.gitignore`
- JNA native lib loading: `java.library.path` alone is NOT sufficient for JNA `Native.register()`.
    Must also set `jna.library.path` JVM property AND `LD_LIBRARY_PATH` env var in test task
- Conformance tests: `ConformanceTest.kt` — 9 methods, 50 vectors. JUnit 5.11.4 + Gson 2.11.0
    (`com.google.code.gson` groupId, NOT `com.google.gson`)

## PyO3 Migration 0.23→0.29 — Per-Hop Edit History (archived iteration 105, migration complete)

Incremental one-minor-per-CID-step migration of the workspace `pyo3` pin (root `Cargo.toml` line 35,
consumed only by `crates/iscc-py`). Endpoint 0.29.0 clears two RustSec advisories shipped in the
published wheel → issue #1 closed. Per-hop source edits (all under `cargo clippy -- -D warnings`):

- 0.23→0.24 and 0.24→0.25: ZERO source changes (lockfile-only).
- 0.25→0.26 (iter 101): FIRST hop needing edits — `Python::allow_threads`→`Python::detach` (7 sites,
    pure rename, same GIL-release semantics) + `pyo3::PyObject` alias→`Py<PyAny>` return type (17
    `PyResult<PyObject>` sites; `Ok(dict.into())` bodies unchanged).
- 0.26→0.27 (iter 102, →0.27.2): cast-family rename in `to_pylist` — `Bound::downcast`→`Bound::cast`
    and `downcast_into_unchecked`→`cast_into_unchecked` (identical sigs; error type
    `DowncastError`→`CastError` but discarded by `if let Ok`).
- 0.27→0.28 (iter 104, →0.28.3): ZERO source changes BUT carried a SILENT behavior change
    `-D warnings` does NOT catch — PyO3 0.28 flipped the unspecified `#[pymodule]` `gil_used`
    default from `true` (macros-backend `map_or(true,…)`) to `false` (`is_some_and(…)`). Review
    added explicit `#[pymodule(name="_lowlevel", gil_used=true)]` (lib.rs:697) to restore pre-0.28
    GIL protection for the raw borrowed `PyList_GetItem` ptrs in `extract_frame_sigs`. Also dropped
    transitive `indoc`/`memoffset`/`unindent` from Cargo.lock.
- 0.28→0.29 (iter 105, →0.29.0): ZERO source changes (lockfile-only; 5 pyo3 crates bump together).
    macros-backend `gil_used` default-handling byte-identical to 0.28 (still defaults `false`), so
    the explicit `gil_used=true` stays load-bearing. 0.29 CHANGELOG "Remove all functionality
    deprecated in PyO3 0.27" was a no-op (already off `downcast*`). 286 pytest pass.

Throughout, the 8 raw `pyo3::ffi::*` C-API sites + `Bound::from_owned_ptr().cast_into_unchecked()` +
`.into_pyobject(py)?.into()` + `#[pyo3(signature=...)]` macros survived every hop unchanged — do NOT
pre-emptively rewrite them. LESSON: "compiles clean" is NOT proof of behavior-neutrality; diff the
macros-backend default-handling on every major bump, not just compiler warnings.

## iai-callgrind harness authoring (archived iteration 108)

- API: `#[library_benchmark]` + `#[bench::id(expr)]` (the `expr` args are evaluated in the
    UNMEASURED setup phase) → `library_benchmark_group!(name = g; benchmarks = a, b, ...)` →
    `main!(library_benchmark_groups = g)`. Use `std::hint::black_box`, NOT `criterion::black_box`
- GOTCHA (verified `iai-callgrind-macros-0.6.1/src/lib_bench.rs:258-317`): `#[library_benchmark]`
    iterates EVERY fn attribute and `abort!`s "Invalid attribute: 'doc'" on anything but
    `bench`/`benches` — a `///` docstring lowers to `#[doc=...]` and is REJECTED. Benchmark fns must
    use plain `//` comments; only non-annotated helper fns can keep `///` docstrings
- For borrow-returning primitives (`alg_cdc_chunks` → `Vec<&[u8]>` borrowing the arg), the bench fn
    returns `.len()` (the Vec can't escape the fn); chunking work is fully measured before `len()`

## Archived from MEMORY.md (iter 215)

### Benchmarks

- `crates/iscc-lib/benches/` (`harness = false`): `benchmarks.rs` criterion 0.8
    (`std::hint::black_box`) + `iai_benches.rs` iai-callgrind 0.16.1 (needs valgrind + runner, NOT
    preinstalled) → ci-gates.md

### Streaming

- `DataHasher`: persistent `buf` reused across `update()`; CDC → BLAKE3 chunk hash → MinHash; ~1.1
    GiB/s. `InstanceHasher`: BLAKE3 → ISCC multihash (64-byte digest truncated)

- `SumHasher` (#37): inner `DataHasher` + `InstanceHasher`; `finalize(bits, wide, add_units)`
    composes via `gen_iscc_code_v0`. Path `iscc_lib::streaming::SumHasher` (NOT crate-root). All 7
    bindings implement `gen_sum_code_v0` → MEMORY-archive.md

- Unicode freeze rule REMOVED 2026-07-30 (decisions.md, human): `text_clean`/`text_collapse` mirror
    the reference on dependency tables. Never reintroduce a freeze without a human decision

## Ruby Bindings (Magnus) — summary (moved from MEMORY.md, iter 221)

- Magnus 0.8 (iter 167; Ruby 3.0-3.4, MSRV 1.65); `function!` no `&Ruby` param — use `Ruby::get()`.
    0.8 dropped `old-api`: use `ruby.exception_runtime_error()` + `ruby.str_from_slice()`, never
    `magnus::exception::*` / `RString::from_slice`. `ExtensionTask.new("iscc-rb")` = Cargo pkg name;
    streaming `RefCell<Option<inner>>` one-shot finalize; `_` prefix for methods NOT class names
- Ruby: never Magnus `Vec<T>` params (segfault, 212) — `RArray` + `array_to_vec`/`frames_to_vec`.
    Gem lock bumps: `bundle update <gem> --conservative --patch` (plain moved rubocop)

## Archived from MEMORY.md (iter 228)

### Documentation Files

- Tabbed syntax: `=== "Language"` 4-space indent, blank line before code block. Landing-page tab
    order (11): Python, Rust, Ruby, Node.js, WASM, Go, Java, C#, C++, Swift, Kotlin; zensical.toml
    nav order differs (nav starts Rust, Python)
- Howto guides `docs/howto/{lang}.md`; API refs
    `docs/{rust-api,api,c-ffi-api,java-api,ruby-api}.md`; per-package READMEs + CLAUDE.md under
    `packages/{dotnet,cpp,swift,kotlin}/`; `docs/unicode.md` was removed with the freeze rule
    (2026-07-30)
- `scripts/gen_llms_full.py`: generates `site/llms-full.txt` + per-page `.md`; run after the
    `zensical build` step in docs CI
- Adding a docs page = update `zensical.toml` nav + `ORDERED_PAGES` + `docs/llms.txt` together —
    enforced by `scripts/check_docs_nav.py` (prek `check-docs-nav` + pytest anchor; disk set is the
    reference, `includes/` allowlisted; nav parse is comment-aware `strip_toml_comments` — naive
    `#.*$` breaks on `"C# / .NET"`; prek `files:` misses `git rm`)
- Spec path gate (224): `scripts/check_spec_paths.py` (prek `check-spec-paths` + pytest anchor)
    resolves spec tree entries + backticked paths vs `git ls-files`; build/release output →
    `GENERATED`, non-paths → `NOT_PATHS` dicts. Fix a phantom by editing the spec, never by adding a
    file. ruff S607 on `["git", …]`: use `shutil.which("git")` instead of a noqa. Glob (228): own
    `glob_regex` (`**` crosses `/`, `*` one segment, `fullmatch`); `NOT_PATHS` exempts backticked
    spans only (checked in `check_spec`), `GENERATED` both forms

### Feature Flags

- `crates/iscc-lib/Cargo.toml`: `default = ["meta-code"]`; `text-processing` gates `text_clean`,
    `text_collapse`, `gen_text_code_v0`, `sliding_window_strs`; `meta-code` (implies text-processing
    \+ JCS) gates META_TRIM consts, meta helpers, `gen_meta_code_v0`, `json_to_data_url`,
    `run_meta_tests`
- `conformance` module always available; `conformance_selftest()` skips disabled types via `#[cfg]`;
    `serde_json` non-optional. When gating `pub(crate)` fns, gate their tests too (dead-code lint
    fires in library builds; `tests/test_text_utils.rs` needs per-function gating)
