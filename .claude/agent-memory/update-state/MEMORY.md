# Update-State Agent Memory

Codepaths, patterns, and key findings accumulated across CID iterations.

## Exploration Shortcuts

- **Java files**: `find crates/iscc-jni -type f | sort` — lists all JNI bridge files
- **Per-crate READMEs**:
    `ls crates/iscc-lib/README.md crates/iscc-py/README.md crates/iscc-napi/README.md crates/iscc-wasm/README.md crates/iscc-ffi/README.md crates/iscc-jni/README.md 2>&1`
    — check existence (batches 1+2 done: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-jni)
- **CI jobs in a run**: `gh run view <id> --json jobs --jq '.jobs[] | {name, conclusion}'`
- **Latest CI runs**: `gh run list --branch main --limit 3 --json status,conclusion,url,databaseId`
- **Java native method count**:
    `grep -c 'native ' crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`
- **Incremental diff**: `git diff <assessed-at-hash>..HEAD --stat`
- **napi test count**: node:test runner counts sub-tests separately; 57+9 `it()` calls = 103
    reported by runner (conformance.test.mjs generates sub-tests from loop)
- **Go files**: `ls packages/go/` — check scaffold; `wc -l packages/go/iscc.go` for function count
- **Go in CI**: `grep -n "go\|Go\|golang" .github/workflows/ci.yml` — check if Go job exists

## Codebase Landmarks

- `crates/iscc-jni/src/lib.rs` — 866-line Rust JNI bridge, 29 `extern "system"` functions, 0
    unwrap(), 72 throw_and_default call sites, 3 jint negative guards, 5 push/pop_local_frame loops
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` — 331-line Java wrapper, 29
    native methods
- `crates/iscc-jni/java/pom.xml` — Maven build config, JDK 17, JUnit 5 + Gson, Surefire 3.5.2 with
    `java.library.path=target/debug`
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` — 362-line JUnit 5 test
    class, 9 `@TestFactory` conformance methods (46 vectors) + 3 `@Test` negative-value methods = 49
    total tests (CI-verified at HEAD)
- `crates/iscc-napi/package.json` — has `"files"` allowlist: `index.js`, `index.d.ts`, `*.node`,
    `README.md`; version `0.0.1`; `index.js` generated with matching `0.0.1` (gitignored, rebuilt at
    CI)
- `crates/iscc-napi/src/lib.rs` — `alg_cdc_chunks` uses `.into_iter().map(Buffer::from)` (no clone)
- `crates/iscc-wasm/src/lib.rs` — `alg_cdc_chunks` now returns `Result<JsValue, JsError>` with
    `.map_err(|e| JsError::new(&e.to_string()))` — silent null issue RESOLVED in iteration 5
- `.devcontainer/Dockerfile` — includes `openjdk-17-jdk-headless` and `maven`
- `.github/workflows/ci.yml` — 7 jobs: Rust, Python, Node.js, WASM, C FFI, Java, Go (Go job added
    iteration 9, commit eb5085d); Go job builds iscc-ffi → wasm32-wasip1, copies .wasm, runs go test
    \+ go vet
- `crates/` — 6 crates: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni
- `docs/howto/` — 6 files: rust.md, python.md, nodejs.md, wasm.md, go.md, java.md (all complete as
    of iteration 16)

## Recurring Patterns

- **Incremental review**: compare `assessed-at` hash vs HEAD `--stat` first, then re-verify only
    affected sections. Always carry forward sections where no relevant files changed.
- **Tier 1 symbol count**: target says "22" but implementation has 23 (target.md counting error)
- **CI now has 7 jobs**: Rust, Python, Node.js, WASM, C FFI, Java, Go. All 7 pass at HEAD (run
    22376568235, iteration 9). Go job added to ci.yml in iteration 9 (commit eb5085d).
- **Registry readme metadata**: `Cargo.toml` `readme = "README.md"` in iscc-lib; `pyproject.toml`
    `readme = "README.md"` in iscc-py; npm auto-detects README.md (no explicit field needed in
    package.json)
- **Java `target/` directory**: Maven compile output in `crates/iscc-jni/java/target/` — covered by
    root `.gitignore`'s `target/` pattern, not committed

## Gotchas

- `packages/go/` (iteration 13, commit c22fa53): `iscc.go` now 1,165 lines — 23/23 Tier 1 symbols
    including `DataHasher` + `InstanceHasher` streaming types (New\*/Update/Finalize/Close lifecycle
    wrapping WASM FfiDataHasher/FfiInstanceHasher). `iscc_test.go` 1,069 lines — 36 func
    declarations (TestMain + 35 tests including 8 streaming hasher tests). Update takes `[]byte`,
    NOT `io.Reader` — architecture gap noted. WASM binary gitignored; TestMain skips if missing.
- Per-crate READMEs: all 6 publishable packages done (iscc-lib, iscc-py, iscc-napi, iscc-wasm,
    iscc-jni, packages/go). iscc-ffi not published separately (lower priority).
    packages/go/README.md created in iteration 10 (commit a60a375).
- Root README NOW COMPLETE as of iteration 14 (commit 200ffb1): Go Reference badge, Go installation
    section, Go quick-start example added. "What is iscc-lib" body text fixed to "Python, Java, Go,
    Node.js, WebAssembly, and C". Key Features updated to "Python, Java, Go, Node.js, WASM, and C
    FFI". README section status → MET. Maven Central badge not added (Java not yet published).
- JNI unwrap() issue resolved in iteration 7 (commit a573475). All 21 unwrap() calls replaced with
    throw_and_default. No critical issues remain in issues.md as of f24a31f.
- Python bytes-like + unbounded read issues resolved in iteration 2 (commit 29fb142). Fixed in
    `crates/iscc-py/python/iscc_lib/__init__.py`: stream detection via
    `isinstance(data, (bytes, bytearray, memoryview))`, chunked 64 KiB reads via
    `_DataHasher`/`_InstanceHasher`, 10 new tests in `tests/test_streaming.py`. Total pytest count:
    157 tests, 115 test functions (up from 105).
- JNI jint validation (3 sites) and local-ref overflow (5 loops) both resolved in iteration 3
    (commit df618f7). Both removed from issues.md.
- Three napi normal-priority issues resolved in iteration 4 (commit 75ff07e): version skew (index.js
    regenerated with 0.0.1), npm packaging (`"files"` allowlist added), alg_cdc_chunks clone
    (`.into_iter().map(Buffer::from)`). All removed from issues.md.
- WASM silent null on alg_cdc_chunks resolved in iteration 5 (commit a908f95): return type changed
    to `Result<JsValue, JsError>`, tests updated with `.unwrap()`. WASM test count is 54 (9
    conformance + 45 unit), NOT 56 (previous state.md overcounted).
- Latest CI run IDs (iteration 18): tests = 22384156126 (7/7 pass), docs = 22384156114 (pass)
- `iscc-ffi` now has `iscc_alloc`/`iscc_dealloc` exported (added iteration 6, commit 2ebca17); crate
    compiles to `wasm32-wasip1` (~10.5 MB debug). Total exported C functions: 25 (23 Tier 1 + 2
    alloc helpers). File is now 1,934 lines.
- Go bindings 23/23 Tier 1 COMPLETE (iteration 13, commit c22fa53). Root README Go section DONE
    (iteration 14, commit 200ffb1). `docs/howto/go.md` DONE (iteration 15, commit 9af5e54, 388
    lines). Go entry in zensical.toml DONE. io.Reader wrapper is optional per verified-when
    criteria.
- `docs/howto/java.md` (319 lines) DONE (iteration 16, commit f9a590f). Java entry in zensical.toml
    DONE. Documentation section now fully MET.
- `NativeLoader.java` (169 lines) DONE (iteration 17, commit 0cc7d51): detects OS/arch, extracts
    `META-INF/native/{os}-{arch}/{libname}` from JAR to temp dir, falls back to System.loadLibrary.
    `IsccLib.java` now delegates to NativeLoader.load(). Extraction path is inactive until native
    binaries are bundled into META-INF/native/ (next iteration target).
- Remaining Java gaps: platform-specific native library bundling inside JAR (CI matrix needed),
    Maven Central publishing configuration.
- **DataHasher buffer optimization (iteration 19, commit a2bbe28)**: `tail: Vec<u8>` replaced with
    `buf: Vec<u8>`. No `to_vec()` or `.concat()` in streaming.rs production path. Tail shifted with
    `copy_within` + `truncate`. `bench_data_hasher_streaming` Criterion benchmark added (~1.0
    GiB/s). `[normal]` DataHasher issue removed from issues.md. 261 total tests unchanged (208 src +
    53 tests/).
- **Codec optimization (iteration 18)**: `decode_header` and `decode_varnibble` now use direct
    bitwise extraction from `&[u8]` via `get_bit`/`extract_bits` helpers. `bytes_to_bits` and
    `bits_to_u32` are `#[cfg(test)]`-gated. 2 new tests added: iscc-lib src tests 208 (was 206). 261
    total tests in iscc-lib (208 src + 53 tests/).
- **Video frame allocation eliminated (iteration 20, commit 12478fd)**: `gen_video_code_v0` and
    `soft_hash_video_v0` now generic `S: AsRef<[i32]> + Ord`. FFI passes `Vec<&[i32]>` (borrowed)
    instead of `Vec<Vec<i32>>` (copied). `.to_vec()` count in iscc-ffi is now 1 (only
    `alg_cdc_chunks`). No `[normal]` issues remain — only `[low]` remain in issues.md.
- Latest CI run IDs (iteration 20): tests = 22385938552 (7/7 pass), docs = 22385938553 (pass)
- **alg_dct and alg_wtahash validation (iteration 21, commit 0edb950)**: `alg_dct` now enforces
    `n.is_power_of_two()` (was: non-empty even-or-1); `alg_wtahash` changed return type from
    `Vec<u8>` to `IsccResult<Vec<u8>>` with guards on `vec.len() >= 380` and bits constraints.
    `soft_hash_video_v0` propagates error directly. Both issues removed from issues.md. Total tests:
    216 in src/ (was 208) + 53 in tests/ = 269 total (was 261).
- Latest CI run IDs (iteration 21): tests = 22386854867 (7/7 pass), docs = 22386854880 (pass)
- **iscc-py __version__ and docstring fix (iteration 22, commit 590f8f5)**:
    `__version__ =   version("iscc-lib")` added to `__init__.py` via `importlib.metadata`;
    `"__version__"` added to `__all__`; module docstring in `src/lib.rs` corrected from
    `iscc._lowlevel` to `iscc_lib._lowlevel`; 2 new tests added (`test_version_exists_and_correct`,
    `test_version_in_all`). Both `[low]` Python issues removed from issues.md. DeepWiki badge added
    to README.md. mise.toml `go = "latest"` tool entry removed.
- Latest CI run IDs (iteration 22): tests = 22387850893 (7/7 pass), docs = 22387850902 (pass)
- Python pytest count: 159 tests (was 157); Python `[low]` issues → all resolved (Python status
    remains MET with no open issues)
- The `state.md` section order must include both Go Bindings and Per-Crate READMEs sections (added
    to target in commit `0a10f73`)
- `gh run list` does NOT need `--repo iscc/iscc-lib` when running from within the workspace (repo
    auto-detected); but `--json` fields are needed to avoid GraphQL deprecation error
