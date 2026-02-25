# Update-State Agent Memory

Codepaths, patterns, and key findings accumulated across CID iterations.

## Exploration Shortcuts

- **Java files**: `find crates/iscc-jni -type f | sort` — lists all JNI bridge files
- **Per-crate READMEs**:
    `ls crates/iscc-lib/README.md crates/iscc-py/README.md crates/iscc-napi/README.md crates/iscc-wasm/README.md crates/iscc-ffi/README.md crates/iscc-jni/README.md 2>&1`
    — check existence (batches 1+2 done: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-jni)
- **CI jobs in a run**: `gh run view <id> --json jobs --jq '.jobs[] | {name, conclusion}'`
- **Latest CI runs**:
    `gh run list --branch "$(git branch --show-current)" --limit 3 --json status,conclusion,url,databaseId`
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
- **Tier 1 symbol count**: target.md and specs/rust-core.md previously said "22" but had uncommitted
    edits correcting to "23" (the actual implementation count). Working-tree-only edits visible in
    `git diff` (HEAD is clean at 23 in both files as of iteration 30).
- **CI now has 7 jobs**: Rust, Python, Node.js, WASM, C FFI, Java, Go. All 7 pass at HEAD when
    Python formatting is clean. Post-interactive-session: Python ruff format check can fail even if
    local `mise run check` passes (CI uses global `uv run ruff format --check`, pre-commit may only
    check staged files).
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
- Per-crate READMEs: all 7 done (iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-jni, packages/go,
    iscc-ffi). iscc-ffi/README.md created in iteration 29 (123 lines). packages/go/README.md created
    in iteration 10 (commit a60a375).
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
- **WASM conformance feature gate (iteration 23, commit fe2e3bf)**: `conformance_selftest` gated
    behind `#[cfg(feature = "conformance")]` in `src/lib.rs` and `tests/unit.rs`;
    `[features]   conformance = []` added to `Cargo.toml`; CI now uses
    `wasm-pack test --node crates/iscc-wasm   --features conformance` (NOT `-- --features` — that
    passes to test runner, not cargo). `[low]` conformance_selftest binary-size issue resolved. WASM
    `[low]` open issue count: 1 (stale CLAUDE.md only).
- **New critical issues (iteration 23)**: Two `[critical]` issues added to issues.md — (1) selective
    publishing inputs for release.yml `workflow_dispatch` (spec:
    `.claude/context/specs/ci-cd.md#release-workflow--selective-publishing`); (2) idempotency checks
    for all publish jobs (spec: `.claude/context/specs/ci-cd.md#idempotency`). One `[normal]` issue:
    `mise run version:sync` / `version:check` tooling. These were NOT in issues.md at iteration 22's
    assessed commit — added by human in between.
- Latest CI run IDs (iteration 23): tests = 22388979767 (7/7 pass), docs = 22388979768 (pass)
- **Selective publishing inputs resolved (iteration 24, commit 06a9ed6)**: `release.yml`
    `workflow_dispatch` now has `inputs:` block with three boolean checkboxes (`crates-io`, `pypi`,
    `npm`) and `if:` conditions on all 8 jobs. Review agent confirmed PASS. First `[critical]` issue
    deleted from issues.md. Remaining `[critical]`: idempotency checks. `[normal]`: version sync
    tooling. Three `[low]` issues remain.
- Latest CI run IDs (iteration 24): tests = 22390109706 (7/7 pass), docs = 22390109757 (pass)
- **Idempotency checks resolved (iteration 25, commits fc103f1+596e0a6)**: All 4 publish jobs in
    `release.yml` now have pre-publish version-existence checks: crates.io uses
    `cargo info   iscc-lib`, PyPI uses `curl -sf "https://pypi.org/pypi/iscc-lib/$VERSION/json"`,
    npm lib/wasm use `npm view "@iscc/lib@$VERSION"` / `npm view "@iscc/wasm@$VERSION"`.
    `skip=true/false` output used; all publish/auth/test steps conditioned on
    `steps.check.outputs.skip != 'true'`. Review agent confirmed PASS. Last `[critical]` issue
    deleted. `ci.yml` now triggers on `develop` branch too (1-line change). `mise.toml` has
    `pr:main` task. CLAUDE.md has branching model section.
- Latest CI run IDs (iteration 25): tests = 22391282792 (7/7 pass); new run 22391326755 in progress
    (6/7 done, all success, Go still running)
- **No [critical] issues remain**. Only `[normal]` (version sync tooling) + 3 `[low]` items remain.
    Next target: implement `scripts/version_sync.py` + `mise run version:sync` / `version:check`.
- **Version sync tooling resolved (iteration 26, commits dc985d2+98fa278)**:
    `scripts/version_sync.py` created (120 lines, stdlib only — `json`, `re`, `pathlib`). Reads
    workspace version from root `Cargo.toml` via regex, updates `crates/iscc-napi/package.json`
    (json loads/dumps with `indent=2`) and `crates/iscc-jni/java/pom.xml` (regex replacement scoped
    to `groupId io.iscc` + `artifactId iscc-lib`). `--check` mode prints OK/MISMATCH and exits 1 on
    mismatch. `mise run version:sync` and `mise run version:check` tasks in `mise.toml` (lines 77,
    81). `pom.xml` version updated from `0.0.1-SNAPSHOT` → `0.0.1`. All 8 review criteria passed.
    `[normal]` issue deleted from issues.md. **All remaining issues are `[low]`**.
- Latest CI run IDs (iteration 26): tests = 22391904404 (7/7 pass); docs = 22390109757 (pass)
- Handoff from review (iteration 26): project ready for `v0.0.1` release — consider PR develop →
    main via `mise run pr:main` before next iteration.
- **JNI IllegalStateException resolved (iteration 27, commit 2083287)**: Added `throw_state_error`
    helper (`env.throw_new("java/lang/IllegalStateException", msg)`); updated 4 call sites
    (DataHasherUpdate, DataHasherFinalize, InstanceHasherUpdate, InstanceHasherFinalize); updated 2
    doc comments; added 2 Java tests. `IsccLibTest.java` now 51 total tests (was 49). All 7 CI jobs
    pass (run 22392431920 triggered by PR #1 develop → main). `[low]` JNI exception issue deleted
    from issues.md. Only 2 `[low]` issues remain: TypeScript evaluation + WASM CLAUDE.md staleness.
- **PR #1 open** (develop → main): CI passes on all 7 jobs; ready to merge for v0.0.1 release.
- `throw_and_default` call sites: now 68 (was 72); `throw_state_error` call sites: 4 (new).
- **WASM CLAUDE.md stale docs resolved (iteration 28, commit 53b0289)**: Updated
    `crates/iscc-wasm/CLAUDE.md` to say "23 Tier 1 symbols plus 2 streaming types"; removed "not yet
    bound" text for DataHasher/InstanceHasher; added "2 streaming types: DataHasher, InstanceHasher"
    to export list. Issue deleted from issues.md. Only 1 `[low]` issue remains: TypeScript port
    evaluation. Latest CI run: 22393043406 (all 7 jobs success). PR #1 still open.
- **iscc-ffi README created (iteration 29, commit e22b4fa)**: `crates/iscc-ffi/README.md` (123
    lines) created. Pattern: "Building" instead of "Installation"; "Memory Management" section
    unique to C FFI; `iscc_`-prefixed function names. All 7 per-crate READMEs now complete →
    Per-Crate READMEs section status → MET. CI still green (run 22394253866, all 7 jobs pass). No
    open issues remain — only the `[low]` TypeScript port evaluation. PR #1 (develop → main) still
    open.
- **docs/index.md Quick Start expanded (iteration 30, commits 746b038+0699ea1)**: Quick Start tabs
    now include all 6 languages: Rust, Python, Node.js, Java, Go, WASM (was only Rust and Python).
    Available Bindings table now includes Java and Go rows (7 total). Documentation target "All code
    examples use tabbed multi-language format" now met for landing page. PR #1 MERGED (develop →
    main). Latest CI: develop run 22395380785 (7/7 pass), main run 22395922655 (all pass), Docs run
    22395922643 (pass). Only 1 `[low]` issue remains. v0.0.1 tag is the logical next step.
- **Interactive session (post-iteration-30, commits 52d1c88+3bed859+5461a65+69bb36c+c4e3657)**:
    Pre-push hooks changed to `always_run: true`; Rust style fixes (inline format args); CPython C
    API optimisation for video code extraction in `crates/iscc-py/src/lib.rs`; 2 new Python-specific
    flat-buffer functions `gen_video_code_v0_flat` + `soft_hash_video_v0_flat`; type signatures
    updated to `Sequence[Sequence[int]]`; API symbol count in target.md and specs/rust-core.md
    updated 22→23 (correcting a pre-existing under-count). **RESULT: CI FAILING** — Python ruff
    format check fails (runs 22401304896 + 22401336439). Fixed by running
    `uv run ruff format crates/iscc-py/python/` and committing.
- **ruff format fix (iteration 31, commit 3c0d70b)**: `_lowlevel.pyi` `gen_video_code_v0` signature
    split across multiple lines to satisfy ruff line-length limit. Review PASS. CI now fully green:
    all 7 jobs SUCCESS on develop (runs 22401871901 + 22401873404). Python status: MET. All
    subsections passing. Next logical step: PR develop → main + tag v0.0.1.
- **v0.0.1 release (iteration 32, commit 56e274d)**: PR #2 merged (develop → main, commit
    `4bdc899`). v0.0.1 tag pushed. Release workflow run 22402189532: PyPI published ✅ (all 4 wheel
    platforms + sdist), crates.io failed (OIDC not configured on registry — human task), WASM build
    failed (wasm-opt rejects `memory.copy` without `--enable-bulk-memory` — fix in release.yml), npm
    @iscc/lib + @iscc/wasm skipped. CI on develop: all 7 jobs pass (run 22402375410). CI on main:
    all jobs pass (run 22402167393). Docs on main: pass (run 22402167413).
- **WASM release build bug FIXED (iteration 4, commit f1ada07)**:
    `[package.metadata.wasm-pack.profile.release]` section added to `crates/iscc-wasm/Cargo.toml`
    with `wasm-opt = ["-O", "--enable-bulk-memory",   "--enable-nontrapping-float-to-int"]`. Both
    flags required: `--enable-bulk-memory` for `memory.copy` (Rust uses bulk memory for memcpy),
    `--enable-nontrapping-float-to-int` for DCT/codec float-to-int conversions. Fix verified locally
    (29.36s release build success) and in CI WASM job (run 22403019335, SUCCESS). Fix is on
    `develop`; needs PR → main + re-release to publish `@iscc/wasm`.
- **wasm-pack profile config**: The correct way to configure wasm-opt flags for wasm-pack is via
    `[package.metadata.wasm-pack.profile.release]` in `Cargo.toml`, NOT via command-line args in CI.
    This keeps the config close to the code and works for both local and CI builds.
- **docs/howto/wasm.md package name fixed (iteration 5, commit 1023080)**: All 20 occurrences of
    `@iscc/iscc-wasm` replaced with `@iscc/wasm`. PR #3 open (develop → main) with both the wasm-opt
    fix and this package name fix. CI on develop fully green: all 7 jobs pass (run 22403499473). Two
    CI runs in progress: push to develop (22403598203) and PR #3 check (22403597692). Loop is in
    maintenance mode — no CID-actionable code work pending. Human actions needed: merge PR #3, then
    re-trigger release to publish @iscc/wasm and @iscc/lib to npm; crates.io OIDC setup also human.
