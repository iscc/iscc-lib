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
- `.github/workflows/ci.yml` — 6 jobs: Rust, Python, Node.js, WASM, C FFI, Java (no Go yet)
- `crates/` — 6 crates: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni
- `docs/howto/` — 4 files: rust.md, python.md, nodejs.md, wasm.md (no java.md or go.md yet)

## Recurring Patterns

- **Incremental review**: compare `assessed-at` hash vs HEAD `--stat` first, then re-verify only
    affected sections. Always carry forward sections where no relevant files changed.
- **Tier 1 symbol count**: target says "22" but implementation has 23 (target.md counting error)
- **CI now has 6 jobs**: Rust, Python, Node.js, WASM, C FFI, Java. All 6 pass at HEAD (run
    22370973644). Go job pending.
- **Registry readme metadata**: `Cargo.toml` `readme = "README.md"` in iscc-lib; `pyproject.toml`
    `readme = "README.md"` in iscc-py; npm auto-detects README.md (no explicit field needed in
    package.json)
- **Java `target/` directory**: Maven compile output in `crates/iscc-jni/java/target/` — covered by
    root `.gitignore`'s `target/` pattern, not committed

## Gotchas

- `packages/go/` does not exist — Go bindings are not started (new target section as of 0a10f73)
- Per-crate READMEs: batches 1+2 complete (iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-jni).
    iscc-ffi not published separately (lower priority). Go README blocked by Go bindings not
    started.
- Root README now has Java sections (installation + quick start) as of iteration 6 (commit 8012a7f).
    "What is iscc-lib" body text (line 47) still says "Python, Node.js, WebAssembly, and C" — minor
    gap. Go sections and Maven Central/Go badges still missing.
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
- Latest CI run IDs (iteration 5): tests = 22372060652 (6/6 pass), docs = 22372060644 (pass)
- Next normal-priority issues: FFI video frame allocation, codec header `Vec<bool>` expansion,
    DataHasher allocation overhead
- The `state.md` section order must include both Go Bindings and Per-Crate READMEs sections (added
    to target in commit `0a10f73`)
- `gh run list` does NOT need `--repo iscc/iscc-lib` when running from within the workspace (repo
    auto-detected); but `--json` fields are needed to avoid GraphQL deprecation error
