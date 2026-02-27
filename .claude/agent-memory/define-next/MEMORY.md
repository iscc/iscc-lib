# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

## Scope Calibration

- Java bindings follow the established multi-step pattern: JNI bridge → wrapper class → tests → CI
    job → loader → docs. Each step is independently verifiable. The review agent confirmed this
    progression works well (three PASS verdicts so far).
- CI job additions are small, single-file changes that provide high value (makes existing local
    tests CI-verified). Good candidate for quick iterations. Pattern: copy existing job structure,
    swap language-specific setup action and build/test commands.
- After CI job: the next logical Java steps are native library loader (for JAR distribution), then
    README/docs updates. Go bindings are a separate track that can start in parallel.
- Per-crate READMEs: batch into groups of 2-3 to stay within scope limits. Batch 1 = iscc-lib,
    iscc-py, iscc-napi (primary publishable crates) — done. Batch 2 = iscc-wasm, iscc-jni (two
    remaining publishable crates) — done. iscc-ffi is not published to any registry, so its README
    is lower priority. Batch 3 = packages/go (after Go bindings exist).
- README files are "create" operations (greenfield), not "modify" — they're less risky than code
    changes. Manifest updates are trivial one-liners. Combined, 3 creates + 2 modifies is a
    reasonable single step for documentation work.
- Root README updates: single-file modifications with clear verification. Java sections follow
    established patterns (Rust/Python/Node.js/WASM already present). Section ordering should match
    target.md: Rust, Python, Java, Node.js, WASM. Insert Java after Node.js and before WASM in both
    Installation and Quick Start sections.
- Critical issues always take priority regardless of feature trajectory. The JNI unwrap() issue is a
    safety fix — pure mechanical replacement (21 calls, 3 patterns, 1 file). Good scope: small,
    well-defined, zero behavioral change, easy to verify with `grep -c 'unwrap()'`.
- Python binding issues (bytes-like misclassification + unbounded .read()) are a natural pair: same
    4 call sites, same file, interrelated fix logic. Good scope for a single iteration — one
    production file + one test file, clear verification via grep + pytest. The handoff and state.md
    both recommended this as the highest-priority next step after the JNI safety fix.
- JNI jint validation + local reference overflow are a natural pair: same file, both robustness
    fixes, no behavioral change for valid inputs. Combined scope is ~40 lines of Rust changes + ~15
    lines of Java tests. 2 files modified (1 Rust + 1 Java test), well within 3-file limit.
- Multiple small issues in the same crate are a natural batch. The 3 napi issues (version skew, npm
    packaging, unnecessary clone) touch only 2 files (`package.json` + `src/lib.rs`) and are all
    quick fixes. The version skew requires a `napi build` regeneration step (not a code change)
    which the advance agent must run. Gitignored generated files (`index.js`) don't count toward the
    3-file limit since they're build artifacts.
- After napi cleanup: WASM silent-null is the next single-crate fix (2 files: lib.rs + unit.rs).
    Mechanical: change return type to `Result<JsValue, JsError>`, swap `.unwrap_or(JsValue::NULL)`
    to `.map_err(...)`, add `.unwrap()` in tests. Every other WASM function already uses this
    pattern so consistency is the primary motivation.
- Go bindings multi-step plan: all 8 steps completed (WASI build → module scaffold → gen wrappers →
    CI job → README → remaining 12 wrappers → streaming hashers → root README Go section). Follow-up
    items: Go `io.Reader` streaming (optional, not explicitly in verified-when), docs/howto guides.
- Java how-to guide: differs from Go in key ways — no runtime object (static methods via
    `IsccLib.*`), streaming uses opaque `long` handles (not structs) requiring try-finally for
    `*Free` calls, build-from-source installation since Maven Central publishing isn't wired yet.
- Java native loader: NativeLoader.java is a well-known JNI pattern (sqlite-jdbc, netty-tcnative).
    Two-phase loading: (1) try JAR resource extraction from `META-INF/native/{os}-{arch}/`, (2) fall
    back to `System.loadLibrary`. Existing CI still works via fallback since no native libs are
    bundled in the JAR yet. This is complete — created in iteration 16.
- **Post-feature optimization phase**: All binding targets met (Rust, Python, Node.js, WASM, C FFI,
    Java partial, Go partial). Remaining target gaps are publishing infrastructure (OIDC, Maven
    Central, npm pipelines) which are complex multi-file CI tasks. Normal issues (codec Vec<bool>,
    DataHasher copying, FFI video allocation) are good single-file optimization steps.
- **Codec Vec<bool> optimization complete** (iteration 18): direct bitwise extraction in
    decode_header, verified by review. DataHasher buffer optimization is next natural optimization
    step.
- **DataHasher buffer optimization** (iteration 19, PASS): persistent `Vec<u8>` replacing per-call
    `to_vec()`/`concat()`. Completed successfully.
- **FFI video frame allocation**: the core video API takes `&[Vec<i32>]` but FFI must construct from
    raw pointers. Using Rust generics `<S: AsRef<[i32]> + Ord>` makes the API accept both
    `&[Vec<i32>]` (existing callers unchanged) and `&[&[i32]]` (FFI can pass borrowed slices). This
    is backward-compatible and limits changes to 2 files (core + FFI). The `Ord` bound is needed for
    `BTreeSet` deduplication inside `soft_hash_video_v0`. Both `Vec<i32>` and `&[i32]` implement
    `AsRef<[i32]> + Ord`.
- **Generic API for backward-compatible optimization**: when an internal optimization requires
    accepting a broader type (e.g., borrowed slices instead of owned Vecs), prefer generic bounds
    (`AsRef<T> + OtherTraits`) over concrete type changes. This avoids cascading modifications
    across all binding crates.
- **Low-priority internal validation fixes**: `pub(crate)` functions with incorrect or missing
    validation are good candidates for batching — same crate, no binding changes, clear tests. The
    `alg_dct` + `alg_wtahash` pair is 3 files (dct.rs, wtahash.rs, lib.rs caller) and purely
    additive (new error paths, no behavioral change for valid inputs). When changing a function's
    return type (e.g., `Vec<u8>` → `IsccResult<Vec<u8>>`), check all callers — if the caller already
    returns `IsccResult`, adding `?` is trivial.
- **Low-priority housekeeping phase**: All [normal] issues resolved, only [low] issues remain. These
    are good small iteration targets: WASM conformance feature gate (3 non-test files), stale
    CLAUDE.md updates (1 file), JNI exception type mapping (1 file + test). Feature gates involving
    CI changes count as 3 files (Cargo.toml + lib.rs + ci.yml) — test files don't count against the
    limit. Group related [low] issues in the same crate only if they share files.
- **Release workflow critical issues**: Two [critical] issues in release.yml done sequentially: (1)
    selective publishing inputs (PASS, iteration 24), (2) idempotency checks (PASS, iteration 25).
    Both are single-file changes to `.github/workflows/release.yml`. The spec in `specs/ci-cd.md`
    has exact YAML snippets to follow — point the advance agent there. Verification is structural
    (grep for expected strings + YAML validity), not functional (can't trigger the workflow in CI).
- **Version sync tooling** (iteration 26): 1 create + 1 modify. The script is pure Python stdlib —
    `json` for package.json, regex for Cargo.toml and pom.xml. Key design choice: regex for pom.xml
    instead of xml.etree to avoid file reformatting. The pom.xml version element sits right after
    groupId and artifactId, making a targeted regex safe. Cross-platform requirement means
    `pathlib.Path` everywhere. Verification is straightforward: run --check (exit 0 = in sync).
- **JNI exception mapping** (iteration 27): Targeted fix — only 4 call sites change (the "already
    finalized" messages in hasher update/finalize methods). Add a parallel `throw_state_error`
    helper rather than parameterizing `throw_and_default` — simpler, no API churn. The 2 new Java
    tests follow the established negative test pattern (`assertThrows`). Existing 49 tests are
    unaffected. This is the last meaningful code quality improvement before release.
- **End-of-project prioritization** (iteration 29): When all [normal]/[critical] issues are resolved
    and only [low] issues remain, prefer concrete deliverables (creating missing files, completing
    documentation sets) over research tasks (evaluating external repos). The iscc-ffi README
    (completing 7/7 per-crate READMEs) is more verifiable and closes a target gap, while the
    TypeScript port evaluation is research with uncertain outcomes. Registry-side publishing setup
    (OIDC, Maven Central) is out of CID scope — it requires human action on external services.
- **Pre-release docs polish** (iteration 30): When all code and READMEs are complete, look for
    concrete docs/index.md gaps against target.md. The target says "All code examples use tabbed
    multi-language format (Python, Rust, Java, Go, Node.js, WASM)" — if tabs are missing, that's a
    concrete verifiable gap. Single-file doc modifications are safe, quick steps. Use howto guides
    as reference for idiomatic code examples per language. Prefer state→target gap closure over
    [low] research issues.
- **Cosmetic doc fixes** (iteration 31): After major doc gaps are closed, the review agent's handoff
    often catches small inaccuracies in code examples. These are ideal for batching into a single
    step when they're all in the same file. The README.md Quick Start was already correct
    (established earlier by the root README step) while docs/index.md had stale patterns — always
    cross-check both files when scoping doc fixes. Verification for doc example fixes can use `grep`
    for presence/absence of specific patterns (e.g., `grep -c 'json.loads'` returns 0).
- **Post-binding-completion doc updates** (CID loop 4, iteration 13): When new symbols are added to
    a binding but docs were written for the earlier symbol count, batch the howto guide + per-crate
    README update into a single step (2 files). The howto guide needs new sections (codec
    operations, constants) while the README needs stale "planned" text replaced with actual API
    tables. Verification is all grep-based — check for presence of newly documented symbols and
    absence of stale text. `uv run zensical build` catches broken docs markup.
- **Interactive session CI breakage** (CID iteration 1 on new loop): Interactive sessions that
    modify Python binding stubs/init files can break `ruff format --check` in CI. The handoff from
    the previous review cycle may not flag CI status if it was green at that time — always check
    state.md's CI section for the latest status. When CI is red, that's always the first priority
    regardless of what the handoff "Next" section suggests. Formatting fixes are trivial single-file
    steps — don't over-scope them.
- **Release milestone steps** (iteration 2, second loop): When all code is complete and CI is green,
    the remaining work is operational (merge PRs, tag releases). These are zero-file-change steps
    that use `gh` and `git` commands exclusively. Verify preconditions (PR mergeable, CI green)
    before scoping, and call out what NOT to do (don't squash, don't delete develop, don't wait for
    release workflow). When an existing PR already covers the merge, update its title/body rather
    than creating a new one.
- **Post-release build fixes** (iteration 3, second loop): When a release workflow partially fails,
    the fix often belongs in crate-level config (e.g., `Cargo.toml` metadata sections) rather than
    the workflow YAML. wasm-pack supports `[package.metadata.wasm-pack.profile.release]` for
    configuring wasm-opt flags — this is the portable, documented approach that works both locally
    and in CI. Prefer crate-config fixes over workflow-command-line fixes for reproducibility. After
    fixing, don't re-trigger the release in the same step — that's a separate human-gated operation.
- **Extended Tier 1 API — ordering** (CID loop 3): 7 new symbols added in order: (1) constants +
    encode_component wrapper (5 symbols, iteration 1), (2) iscc_decode (iteration 2), (3)
    json_to_data_url (iteration 3). This ordering worked well — additive constants first, then
    progressively complex functions. After all 30 Rust core symbols: propagate to 6 binding crates.
- **json_to_data_url combines existing helpers**: The function is a thin public wrapper around
    `parse_meta_json` (JCS canonicalization) + `build_meta_data_url` (media type + base64 encoding).
    Both helpers are already private in lib.rs. The conformance vector `test_0016_meta_data_url`
    uses `charset=utf-8` in its data URL while our function omits charset — the payloads should
    match but the full URL prefix will differ. Test the payload, not the exact URL format.
- **Binding propagation ordering** (CID loop 3, iteration 5+): Start with Python because it has the
    most mature test infrastructure (117 tests across 5 files) and issue #5 layer 2 (dict meta
    acceptance) depends on `json_to_data_url` being available. Each binding propagation step is 3
    files (native wrapper + Python wrapper/init + type stubs), well within limits. The 7 symbols
    split into 3 functions + 4 constants — constants are trivial (`m.add()` in PyO3, simple imports
    in Python). Functions follow the established thin-wrapper `map_err(PyValueError)` pattern.
    `iscc_decode` is the trickiest because it returns a tuple with `bytes` (needs `PyBytes`
    wrapping). After Python: Node.js, WASM, C FFI, Java, Go in any order.
- **Python iscc-core drop-in extensions** (CID loop 3, iteration 6+): After all 30/30 Tier 1 symbols
    are in Python, 4 small drop-in extensions remain: (1) dict meta for gen_meta_code_v0 (issue #5)
    — Python wrapper only, uses json_to_data_url from \_lowlevel, (2) PIL pixel data for
    gen_image_code_v0 (issue #4) — Python wrapper only, bytes(pixels), (3) MT/ST/VS IntEnum classes
    (issue #6), (4) core_opts SimpleNamespace (issue #8). All are Python-only, no Rust changes. Each
    is a single-step scope (1-2 files). Natural ordering: dict meta first (uses newly-propagated
    json_to_data_url), then PIL pixels, then enums + core_opts (can batch #6 + #8 since both are
    pure additions to __init__.py).
- **PIL pixel data conversion** (issue #4): The simplest pattern is
    `if not isinstance(pixels, bytes):   pixels = bytes(pixels)` — the `bytes()` constructor handles
    `bytearray`, `memoryview`, and `Sequence[int]` uniformly. No PIL test dependency needed —
    `list(range(256)) * 4` creates 1024 synthetic pixel values. The `Sequence` import is already in
    `__init__.py`. After this: #6 + #8 can be batched as they're both additive to `__init__.py` and
    `__all__`.
- **Batching #6 + #8 + iscc_decode wrapping** (iteration 7, CID loop 4): All three are pure Python
    additions to `__init__.py` — no Rust changes. Total ~30 lines of production code + tests. The
    `ST` IntEnum must include all values 0-7 (not just 0-4 from the spec listing) because
    `iscc_decode` can return subtype values 5 (SUM), 6 (ISCC_NONE), 7 (WIDE) for ISCC-CODE headers.
    Python IntEnum handles `TEXT = 0` as an alias for `NONE = 0` naturally. After this step: all
    Python iscc-core drop-in gaps are closed; next phase is propagating 7 symbols to 5 remaining
    bindings.
- **Binding propagation phase** (CID loop 4, iteration 8+): 5 bindings × 7 symbols each. One binding
    per step. Order: Node.js → WASM → C FFI → Java → Go. Node.js first because it's the most mature
    non-Python binding (103 tests) and napi-rs patterns are well-established. Each step is 2 files
    (native wrapper + tests). For napi-rs: `#[napi]` on `pub const` exports JS constants (cast
    `usize` → `u32`); `#[napi(object)]` struct for `iscc_decode` return (JS has no tuples); `String`
    not `&str` for function args (napi-rs convention). Build regeneration (`napi build`) needed
    before running tests — `index.js`/`index.d.ts` are gitignored artifacts.
- **WASM constant export** (CID loop 4, iteration 9): wasm-bindgen does not support
    `#[wasm_bindgen]` on `pub const` — use getter functions with
    `#[wasm_bindgen(js_name = "CONST_NAME")]` instead. For struct returns (like `iscc_decode`), use
    `#[wasm_bindgen(getter_with_clone)]` on the struct since `Vec<u8>` isn't `Copy`. wasm-bindgen
    accepts `&str` directly (unlike napi-rs which needs owned `String`), so prefer `&str` for string
    args. After WASM: C FFI, Java, Go remain.
- **C FFI propagation** (CID loop 4, iteration 10): C FFI uses `extern "C"` functions with
    `#[unsafe(no_mangle)]`. Constants are best exposed as getter functions (not `pub const`) for
    consistency with WASM pattern and to avoid cbindgen `usize` → C type mapping issues. For
    `iscc_decode`, a `#[repr(C)]` struct `IsccDecodeResult` with `ok: bool` discriminant + reuse of
    `IsccByteBuffer` for digest is the cleanest approach — matches existing FFI memory model. The C
    test file uses macro-based assertions (`ASSERT_STR_EQ`, `ASSERT_EQ`, etc.) and must call
    `iscc_free_*` for every allocated result. After C FFI: Java JNI, then Go.
- **Java JNI propagation** (CID loop 4, iteration 11): 3 files (1 create + 2 modify, excluding
    tests). Constants are pure Java `public static final int` — no JNI needed. `jsonToDataUrl` and
    `encodeComponent` follow existing JNI patterns (`env.get_string`, `extract_byte_array`,
    `throw_and_default`). `isccDecode` is the novel part — requires constructing a Java object from
    JNI via `env.find_class` + `env.new_object` with constructor descriptor `(IIII[B)V`. A separate
    `IsccDecodeResult.java` class is more idiomatic Java than a static inner class. The existing JNI
    crate has 29 `extern "system"` functions; this step adds 3 (total 32). After Java: Go (final
    binding).
- **Go binding propagation** (CID loop 4, iteration 12): Final binding — 7 symbols (4 constants + 3
    functions). Constants are trivial package-level `const` (Go idiomatic — no enum types).
    Functions follow existing Go bridge patterns: `JsonToDataUrl` mirrors `TextClean`
    (string→string); `EncodeComponent` mirrors `EncodeBase64` (bytes+scalars→string); `IsccDecode`
    is the complex one — struct return via sret ABI (16 bytes: bool+4×u8+padding+IsccByteBuffer).
    The `DecodeResult` struct has PascalCase fields (Go convention). The sret is 16 bytes:
    ok(1)+maintype(1)+subtype(1) +version(1)+length(1)+pad(3)+data_ptr(4)+data_len(4). Must free via
    `iscc_free_decode_result(sret_ptr)` then `dealloc(sret_ptr, 16)`. All 7 symbols fit in 1 source
    file + 1 test file.

## Architecture Decisions

- Java conformance tests use `data.json` from `crates/iscc-lib/tests/data.json` (shared across all
    bindings) via relative path from Maven's working directory.
- Maven Surefire plugin sets `java.library.path` to `target/debug/` for finding the native cdylib.
    This means `cargo build -p iscc-jni` must run before `mvn test`.
- Gson chosen as JSON parsing library for Java tests — handles nested arrays (`int[][]` for video
    frame sigs) cleanly and is a well-known, lightweight test dependency.
- Go bindings use WASM/wazero (pure Go, no cgo) per target spec. The WASM module is built from
    iscc-ffi targeting `wasm32-wasip1`. The Go wrapper embeds the `.wasm` binary via `//go:embed`.
    Alloc/dealloc helpers are needed because the WASM host must allocate memory inside the module to
    pass strings and byte buffers.
- The FFI crate's existing `crate-type = ["cdylib", "staticlib"]` works for wasm32-wasip1 — cargo
    produces a `.wasm` from the cdylib target. No Cargo.toml changes needed for the build.
- `thread_local!` in the FFI crate (for error storage) should work on wasm32-wasip1 since WASM is
    single-threaded. The macro compiles but degenerates to a simple static.

## Registry README Patterns

- napi-rs `gen_*_v0` functions return `String` (not structured objects) — Node.js quick start
    examples must show string return, not `result.iscc` pattern.
- Python bindings return `dict` (via PyO3 `PyDict`) — quick start uses `result['iscc']`.
- Rust core returns typed `*CodeResult` structs with `.iscc` field.
- WASM binding (`iscc-wasm`) also returns strings from gen functions (same as napi-rs). It publishes
    to npm as `@iscc/wasm` via wasm-pack, not to crates.io (Cargo.toml has `publish = false`).
- JNI binding (`iscc-jni`) publishes to Maven Central as `io.iscc:iscc-lib`, not crates.io
    (Cargo.toml has `publish = false`). Java method names use camelCase (e.g., `genMetaCodeV0`).
- Go README: Go module proxy (pkg.go.dev) renders README.md. Go gen functions return
    `(string, error)` — quick start uses `iscc, err := rt.GenMetaCodeV0(ctx, ...)` pattern.
- C FFI README: not published to any registry (`publish = false`), so no version badge. Uses
    "Building" section instead of "Installation". Unique among READMEs in needing a "Memory
    Management" section (Rust-allocates/Rust-frees, 4 free functions, `iscc_last_error`). C function
    names use `iscc_` prefix (`iscc_gen_meta_code_v0`). Quick start must show explicit `free` calls.

## CI Workflow Patterns

- All CI jobs share a common preamble: `actions/checkout@v4` → `dtolnay/rust-toolchain@stable` →
    `Swatinem/rust-cache@v2`, then language-specific setup and build/test commands.
- Language-specific setup actions: `actions/setup-python@v5`, `actions/setup-node@v4`,
    `actions/setup-java@v4` (with `distribution: 'temurin'`), `actions/setup-go@v5` (with
    `go-version-file`).
- Never use `mise` in CI — call tools directly per learnings.
- Go CI job is the only one with a cross-compilation pre-step: Rust → wasm32-wasip1 → copy to
    packages/go/ before running Go tests. All other binding jobs build native (same-platform) Rust.

## Recurring Patterns

- All binding conformance tests follow the same structure: load data.json, iterate per-function test
    groups, decode inputs per function signature, compare `.iscc` output field.
- `gen_iscc_code_v0` test vectors have no `wide` parameter in data.json — always pass `false` (the
    Python default).
- `"stream:<hex>"` prefix in data.json denotes hex-encoded byte data for `gen_data_code_v0` and
    `gen_instance_code_v0`. Empty after prefix = empty bytes.

## Gotchas

- JNI function names encode Java package underscores as `_1` (e.g., `iscc_lib` → `iscc_1lib`).
- `gen_image_code_v0` pixels in data.json are JSON int arrays (0-255) that need casting to Java
    `byte` (signed). Java's `byte` range is -128 to 127, so values 128-255 will wrap — this is fine
    because the JNI bridge handles the conversion correctly.
- Maven's working directory is the pom.xml parent directory, not the workspace root.
- ISCC Foundation URL is `https://iscc.io` — not iscc.foundation or other variants.
- WASM howto guide at `docs/howto/wasm.md` has wrong package name `@iscc/iscc-wasm` — correct name
    is `@iscc/wasm` per learnings. Being fixed in iteration 5 (second CID loop).
- **Pairing doc fixes with PRs** (iteration 5, second loop): When the next step is creating a PR
    (develop → main), pair it with any pending doc fixes that would ship in that merge. The WASM
    howto package name fix was a known issue sitting in agent memory — fixing it before the PR means
    the corrected docs deploy from main immediately. This avoids an extra iteration just for the
    fix. Good pairing criteria: same branch, no code risk, verifiable by grep.
- **Remaining state→target gaps after v0.0.1**: Maven Central publishing, crates.io OIDC, npm token
    setup — all require human action on external services. The TypeScript port evaluation (low
    issue) is CID-actionable but low priority.
- **Go io.Reader streaming** (iteration 7, second loop): When the handoff says "maintenance mode"
    but the target architecture description mentions a feature ("io.Reader support for streaming")
    that isn't implemented, that's still a valid gap to close. `UpdateFrom(ctx, io.Reader)` is 2
    methods + 3 tests, well under the 3-file limit. It delegates to existing `Update`, so no WASM
    changes. Prefer concrete code improvements over research tasks (TypeScript evaluation) even when
    the handoff suggests the latter.
- **Research + docs hybrid steps** (iteration 8, second loop): When the only remaining CID-
    actionable item is a research task (evaluating an external repo), combine it with a concrete
    deliverable (a new docs page) so the step is verifiable. The advance agent uses
    WebFetch/deepwiki to examine the external repo, then creates a documentation page with findings.
    Scope: 1 create (docs page) + 1 modify (nav config) = well under the 3-file limit. Key: be
    factual and neutral about third-party conformance — state what was observed, not assumptions.
- **Java native bundling in release workflow** (iteration 5, second loop): The `build-jni` +
    `assemble-jar` pattern mirrors the existing `build-napi` + `publish-npm-lib` pattern. Key
    differences: (1) NativeLoader expects `META-INF/native/{os}-{arch}/{libname}` directory
    convention, (2) Maven `src/main/resources/` is auto-included in JAR (no pom.xml changes), (3)
    the assemble-jar step collects artifacts and runs `mvn package -DskipTests`. This is CID-
    actionable — no human credentials needed for the build step. Maven Central publishing is a
    separate future step (requires GPG + Sonatype credentials).
- **Stale documentation pages after binding additions** (iteration 9, second loop): When new
    bindings are added over multiple iterations (JNI in iter 5, Go in iter 6-7), docs pages written
    earlier (architecture.md, development.md) become stale — they miss the new crates in diagrams,
    layout trees, and summary tables. The state assessment may say "Documentation: met" because
    top-level target verification criteria are met, but detailed spec gaps remain. These are safe,
    docs-only steps (2 files, no code changes). Check for: mermaid diagrams, workspace layout trees,
    crate summary tables, streaming pattern tables, conformance test matrix tables.
- **Cross-language doc parity** (CID loop 5, iteration 1): When all 6 bindings have 30/30 Tier 1
    symbols but only Go's howto guide documents codec/constants, batch all 4 remaining binding
    guides (Python, Node.js, Java, WASM) into one step. Doc files are excluded from the 3-file
    limit, and all guides follow the same Go template — ~60-70 lines per guide. Key
    language-specific details: Python has `core_opts` SimpleNamespace + IntEnum return from
    `iscc_decode`; WASM uses getter functions for constants (not `const`); Java uses camelCase
    method names on static `IsccLib` class. Verification is all grep-based (check for function names
    \+ constant names in each file) plus `zensical build` for docs integrity.
- **Stale internal docs after API promotions** (CID loop 6, iteration 2): When Tier 2 symbols get
    promoted to Tier 1 across multiple iterations, per-crate CLAUDE.md files become stale — they
    still list old symbol counts and tier assignments. This is a concrete maintenance debt because
    agents read CLAUDE.md for API tier rules. Signs: state.md says "30 symbols" but CLAUDE.md says
    "22", Tier 2 section lists functions that are re-exported at crate root. Fix is single-file,
    docs-only, fully grep-verifiable. Low risk, high value for agent accuracy.
- **End-of-project maintenance phase**: When all code is complete, CI green, and only operational
    tasks remain (publishing, issues cleanup), prioritize stale internal docs (CLAUDE.md, agent
    memory) over external operational tasks that require human action (OIDC setup, npm triggers,
    Maven Central). Internal docs are CID-actionable; external operations are not.
- **Go WASM binary distribution gap** (CID loop 6, iteration 3): The Go module uses
    `//go:embed   iscc_ffi.wasm` but the binary was gitignored — blocking `go get` for consumers.
    The fix is simple: remove the gitignore line and commit the release binary (~683KB with LTO).
    The CI Go job builds a fresh debug binary for testing anyway (overwrites the committed one in
    the CI environment), so there's no conflict. This is the standard pattern for Go modules
    embedding WASM via wazero. Debug binary is ~11MB vs ~683KB for release — always commit the
    release build.
