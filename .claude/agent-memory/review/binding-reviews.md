# Detailed Binding Review Recipes

Moved from MEMORY.md to keep the index concise. Referenced from MEMORY.md "Binding State".

## Binding State (all 9 crates met, 32/32 Tier 1)

- 9 binding crates (py, napi, wasm, ffi, jni, go, rb, uniffi). NAPI `index.js`/`.d.ts`/`*.node`
    gitignored (auto-gen by `napi build`). npm `@iscc/lib` bundled single-package (#38 closed):
    `files: ["*.node"]` ships all 5 binaries, NO `optionalDependencies`; `napi prepublish` must NOT
    run in `publish-npm-lib`. FFI constant count in module docstring must match additions (now 5)
- .NET + Swift bindings fully complete (32/32 Tier 1, CI, version sync, docs, release)

## IDv1 fan-out slices — probe validation ORDER (183; jni fixed 184)

- Spec (rust-core.md §Validation, criterion ~L542) makes ts→hub→realm ordering **normative on every
    surface**, "first failing check wins", asserted even for MULTI-invalid inputs. A wide-int
    binding (napi/wasm/jni, and rb/dotnet if they take wider-than-needed ints) must validate the
    three SEMANTIC thresholds (`2^52`/`4096`/`2`) in that order IN THE BINDING before narrowing —
    napi does `checked(v,thresh,name)?` ×3. A *wide narrowing guard* (hub 0-65535, realm 0-255) that
    skips ts and defers to core is WRONG → 183 jni NEEDS_WORK (`(2^52,65536,0)` reported hub not
    ts). **Always probe two multi-invalid inputs** — next.md's Impl Notes prescribed the flawed
    narrowing guard, so the golden/single-field tests pass while ordering silently diverges. Go/ffi
    exact-width types make out-of-narrowing values unrepresentable → they delegate ordering to core
    safely. **184 jni redo verified**: reference order is `iscc_id.py:127-133`
    (`timestamp>=2^52`→`hub_id>=2^12`→`realm not in (0,1)`, first-raise wins); confirm the two
    multi-invalid probes assert the WINNING field's substring (msg `"hubId (hub)"` satisfies
    `contains("hub")`). jni takes `jlong`/`jint`, so also reject negative before narrowing

## Binding Propagation Shortcuts

- napi-rs: `npm test` + clippy + `mise run check`
- WASM: `wasm-pack test --node` (with and without `--features conformance`) + clippy. To run one
    test file: `--test <name>` goes BEFORE the `--` (cargo arg); after `--` the runner rejects it
- C FFI: `cargo test -p iscc-ffi` + clippy. Header tracked in git. For a header/symbol change (182)
    verify freshness by RE-RUNning cbindgen
    (`cbindgen --config crates/iscc-ffi/cbindgen.toml --crate   iscc-ffi --output crates/iscc-ffi/include/iscc.h`)
    → `git diff --exit-code iscc.h` must be empty, then compile+run the C test:
    `gcc -o /tmp/test_iscc crates/iscc-ffi/tests/test_iscc.c -I   crates/iscc-ffi/include -L target/debug -liscc_ffi -lpthread -ldl -lm && LD_LIBRARY_PATH=target/debug /tmp/test_iscc`
    (exit 0)
- Java JNI: `cargo build -p iscc-jni` + clippy + `mvn test`
- Ruby: `pushd crates/iscc-rb && bundle exec rake compile && bundle exec rake test; popd`
- .NET: `cargo build -p iscc-ffi` + `dotnet build packages/dotnet/Iscc.Lib/` +
    `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=/workspace/iscc-lib/target/debug`
    - `mise run check`
- Kotlin: `cargo build -p iscc-uniffi` + `cd packages/kotlin && ./gradlew test` + clippy workspace
    - `mise run check`

## UniFFI Review

- `crates/iscc-uniffi/` — shared scaffolding for Swift+Kotlin. `uniffi = "0.31"`, proc macros only
    (no UDL/build.rs/uniffi.toml). 32 `#[uniffi::export]` (30 free fns + 2 impl blocks).
    `publish = false`. `bindgen` feature: `uniffi/cli` → `uniffi-bindgen` binary
- Review shortcut: `cargo test -p iscc-uniffi` + `cargo clippy -p iscc-uniffi -- -D warnings` +
    `cargo clippy --workspace --all-targets -- -D warnings` + `mise run check`

## Kotlin Binding Review

- **Gradle multi-JAR artifact**: `withSourcesJar()` + `withJavadocJar()` produce 3 JARs in
    `build/libs/`. When selecting runtime JAR from glob, filter out `-sources.jar`/`-javadoc.jar` —
    alphabetical `head -1` picks `-javadoc.jar` first
- Generated `iscc_uniffi.kt` (~112KB, 3214 lines) — do NOT manually edit, regenerate via
    uniffi-bindgen. `@file:Suppress("NAME_SHADOWING")` is UniFFI boilerplate, not gate circumvention
- JNA native lib loading: `java.library.path` alone NOT sufficient for JNA `Native.register()`. Must
    also set `jna.library.path` JVM property AND `LD_LIBRARY_PATH` env var in test task
- Kotlin bindings fully complete: CI job, gradlew perms, version sync, docs/README, release
    workflow. Codex is confused by large generated Kotlin/Swift diffs — findings advisory
- Kotlin Maven Central: `useInMemoryPgpKeys` (not `useGpgCmd`), staging to `build/staging-deploy/`,
    curl bundle upload to Central Portal REST API. JNA resource dirs differ from JNI (linux-x86-64
    vs linux-x86_64)

## Ruby Binding Review

- Magnus **0.8.2** since iter 167 (the old "0.8 needs Ruby 3.2+" note was wrong — 0.8's README says
    Ruby 3.0-3.4 fully supported, MSRV 1.65; gemspec floor `>= 3.1.0`, CI 3.1, cross-gem 3.1/3.2/3.3
    all fit). 0.8 drops `old-api`: `ruby.exception_runtime_error()` and `ruby.str_from_slice()`
    replace `magnus::exception::*` / `RString::from_slice`; `RString::as_slice` is NOT deprecated
- Gates (~8 min): `cargo clippy -p iscc-rb --all-targets -- -D warnings` + `mise run audit` +
    `(cd crates/iscc-rb && bundle exec rake compile && bundle exec rake test)` — the `.so` is
    gitignored and stale-prone, so confirm the linked dep with `strings lib/iscc_lib/iscc_rb.so`.
    Suite total is dynamic (`test_conformance.rb`): 124 runs / 314 assertions at 167; it can only
    move if a test file, `Gemfile*` or a fixture moved in the diff — check that instead of a floor
- `function!` macro does NOT accept `&Ruby` — use `Ruby::get().expect("called from Ruby")`
- Ruby `JSON.generate` ignores `sort_keys: true` — use `.sort.to_h` before generate
- Streaming classes: `#[magnus::wrap(class = "...")]` + `RefCell<Option<inner>>` for one-shot
    finalize

## Environment

- Python `iscc_lib`: compile with `cd crates/iscc-py && uv run maturin develop --release`
- `.pyi` stub sync: `ty check` catches mismatches, `mise run check` does not
- **Pre-push needs iscc_lib built**: `ty check` and `pytest` hooks import `iscc_lib` — build before
    pushing (same maturin command above, ~30s), else push fails. **Build BEFORE `git push`, not
    after the hook fires** (iter 148): a fresh container rejects the push with 8
    `ModuleNotFoundError: No module named 'iscc_lib'` collection errors, which reads like a
    NEEDS_WORK gate rejection but is a missing local artifact. Build, re-run `uv run pytest -q` +
    `uv run ty check` yourself, then push — that makes the gate *evaluate* the code instead of
    bypassing it. Bonus: pytest then exercises the change through the PyO3 binding too (353 tests)
- **PyO3 is `0.29`, migration COMPLETE** (#1 closed): per-hop recipe + GIL-detach pattern + gotcha
    catalog in `learnings-archive.md`. Verify pin: `cargo tree -p iscc-py -i pyo3` (single 0.29.0).
    Keep explicit `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697) — 0.28 silently
    flipped that default `true`→`false`. LESSON for future bumps: "compiles clean under
    `-D warnings`" ≠ behavior-neutral; diff pyo3-macros-backend default handling each hop. Advisory
    clearance now IS tool-confirmable via `cargo deny check` (enforcing Audit gate, iter 114+)

## Feature Flag Review

- Use Rust-only shortcut. Verify 3 configs: default, no-default, text-processing only
- `serde_json` non-optional (conformance.rs dependency)
- **`--no-default-features --all-targets` fails on `benchmarks` bench** (pre-existing, NOT a
    regression): benches import `gen_meta`/`gen_text` needing default features. Scope no-default
    clippy to the lib (`--no-default-features -- -D warnings`, no `--all-targets`). CI never runs it
- `streaming::SumHasher` is core-only (full path `iscc_lib::streaming::SumHasher`), NOT a Tier 1
    re-export; `gen_sum_code_v0` wraps `SumHasher::finalize(bits,wide,add_units)`. #37 fully closed

## Per-binding review commands (moved from MEMORY.md index, iter 149)

- **Ruby-only**: `mise run check` + `cargo clippy -p iscc-rb -- -D warnings` +
    `pushd crates/iscc-rb && bundle exec rake test; popd` + `bundle exec standardrb` (the last needs
    `PATH="$(ruby -e "puts Gem.user_dir")/bin:$PATH"`).
- **Kotlin-only**: `cargo build -p iscc-uniffi` + `cd packages/kotlin && ./gradlew test` + clippy
    workspace + `mise run check`. Gradle flakes on this bind mount — read
    `build/test-results/test/*.xml` and re-run after `./gradlew clean` before calling it a failure.
    Published consumer floor is **Kotlin 2.3 or newer**, documented in 4 places that move together.
- **A published `.pyi` needs mypy + pyright, not just `ty`** (iter 131; the wheel ships `py.typed`):
    `uvx mypy@1.18.2 --strict` + `uvx pyright@1.1.407` ≈ 30s; prefer `ast.parse` over greps.

## Unicode boundary-fixture propagation slices (iters 150–151)

- **WASM slice** (~4 min): `wasm-pack test --node crates/iscc-wasm --features conformance`. **Pipe
    to a FILE, never `| tail`** — the per-target `test result:` lines you must read are mid-log.
    Targets at iter 151: lib 0, conformance 9, unicode_boundary 3, unit 78, doctests 0. On the host
    target **all 5 report `0 passed`** — `#[wasm_bindgen_test]` does not register there, so host
    `cargo test`/`clippy --all-targets` prove compilation only. That is expected and is the reason
    CI has a separate `WASM (wasm-pack test)` job (ci.yml ~line 113).
- **Ruby slice** (~2 min): `bundle exec rake compile` (mandatory — the `.so` is gitignored and a
    pre-sentinel build returns `aSb` for `text_clean("a"+U+A7F1+"b")`) then `rake test` +
    `standardrb`. CI compiles fresh before testing, so a local red is usually staleness.
- **Mutation probe for both at once** — both read the **canonical**
    `crates/iscc-lib/tests/unicode_boundary.json` in place, so ONE fixture edit probes both
    surfaces. Rewrite it with Python (`json.dumps(..., ensure_ascii=True, indent=2)` + `\n`), run,
    then `git checkout` and `cmp` against a saved copy. Two mutations worth doing: (1) set a U+A7F1
    expected value to the pre-freeze `aSb` → each suite must red and NAME the vector; (2) delete one
    case → the metadata guard (counts 7/5) must red. Verified iter 151 on both.

### napi + JNI/Java slice (iter 153, ~8 min)

- Both read the **canonical** fixture in place, so ONE fixture mutation probes both (same two
    mutations as WASM/Ruby: a delete-filter-shaped expected value must red and NAME the case; a
    deleted case must red the 7/5 metadata guard). Verified iter 153.
- napi: `node --test crates/iscc-napi/__tests__/unicode_boundary.test.mjs` (13 tests) then
    `npm --prefix crates/iscc-napi test` (152). No `cd` needed — the test's imports are relative to
    the file, and `npm --prefix` sets the script cwd.
- JNI: `cargo build -p iscc-jni` — **"Finished … 1.11s" with no recompilation is the freshness
    proof** (cargo considers the `.so` current w.r.t. HEAD sources) — then
    `timeout 600 mvn -o -B test -f crates/iscc-jni/java/pom.xml` (~4 s offline, 82 tests).
    Surefire's working directory is the pom basedir, so the test's `../../iscc-lib/tests/...` path
    resolves even when mvn is invoked from the repo root — same as CI.
- Java is discovered by surefire's `*Test.java` glob and napi by the
    `node --test __tests__/*.test.mjs` npm script, so neither slice needs a `ci.yml` edit — but
    *verify* that in ci.yml, don't assume it.
- `pom.xml` sets no `project.build.sourceEncoding`; non-ASCII in comments is pre-existing and
    harmless, non-ASCII in *string literals* would be platform-dependent.

### JNI crate review — beyond `mvn clean test` (iters 168–169, ~12 min)

- Baseline: `cargo build -p iscc-jni` then `mvn clean test -f crates/iscc-jni/java/pom.xml` = **93
    tests** (80 `IsccLibTest` + 13 `UnicodeBoundaryTest`) since 169; the totals are fixture- and
    `@Test`-derived, so they cannot move while `java/` and `data.json` are untouched.
    `strings target/debug/libiscc_jni.so |   grep -oE 'jni(-sys)?-0\.[0-9.]+'` proves which jni
    version is actually linked.
- **Native coverage CLOSED at 169 — all 33 have a JUnit caller.** Re-assert it, don't assume; a grep
    loop misses a renamed native, so parse instead:
    `re.findall(r'native\s+[\w\[\]<>., ]+?\s+(\w+)\s*\(', IsccLib.java)` and check each name for
    `IsccLib.<name>(` across `java/src/test/**/*.java`. Expect 33 declared, 0 uncovered.
- **Run the suite under `-Xcheck:jni` on any marshalling change** —
    `mvn test -f crates/iscc-jni/java/pom.xml -DargLine="-Xcheck:jni -Djava.library.path=$PWD/target/debug"`;
    surefire's `argLine` is **overridden, not merged**, so the library path must be repeated or
    every test errors with `UnsatisfiedLinkError`. Grep the log for `WARNING: JNI` /
    `FATAL ERROR in   native`; zero at 168 and 169. (The older standalone `Probe.java` route via
    `javac -cp crates/iscc-jni/java/target/classes` still works and is only needed for natives the
    suite cannot reach.)
- **HotSpot type-checks nothing a native returns** — assert `getClass().getName()` is
    `[Ljava.lang.String;` / `[[B` / `[B`, and stress the five `with_local_frame` sites past the ~512
    local-ref limit (2 MiB through `algCdcChunks` → 1025 chunks, 5000-char `slidingWindow`, 1000
    codes through `genMixedCodeV0`, 1000 sigs through `genVideoCodeV0`).
- **Verify every `#[deprecated]` claim next.md makes against the vendored crate source** —
    `~/.cargo/registry/src/index.crates.io-*/jni-0.22.4/src/env.rs`. At 168 `byte_array_from_slice`
    (env.rs:3350) was NOT deprecated though next.md said so — it is the zero-copy `byte[]` return
    path (`JByteArray::new` + transmuted `set_region`, no `Vec`), restored at 169; `get_string`,
    `get_array_length`, `get_int_array_region`, `get_/set_object_array_element` were, and
    `push_/pop_local_frame` were removed outright. The error policy is at
    `src/errors/policy.rs:188`.

### Vendored-copy propagation slice (iter 150 — Python + pure-Go; moved from MEMORY.md index)

Per-language shortcut, then `cmp` the vendored copy against the canonical and assert
`read_bytes().isascii()`. Probe in a `cp -r` of the package under `/tmp`, **never the work tree**:
corrupt a **non-skipped** expected output; **rename** a skipped case (must fire BOTH the stale-skip
guard and an unskipped failure); drop a case; set a wrong fixture version. Python probes need the
test nested two dirs deep so `Path(__file__).parent.parent` resolves — a missing fixture is a hard
collection error, not a silent skip. Verify no `delete_filter_output` oracle was copied
(`git grep -l delete_filter`) and that the sequence vectors still discriminate the design. Since
iter 152 every new tracked copy must also be registered in `VENDORED_COPIES` of
`tests/test_vendored_fixtures.py`, and must keep the basename `data.json` / `unicode_boundary.json`
— that gate discovers copies by basename only.

### C# + Kotlin slice (iter 154, ~10 min)

- Both read the **canonical** fixture in place (no vendored copy — csproj `<Content Link=…>` for
    .NET, `iscc.fixtureDir` system property for Gradle), so ONE fixture mutation probes both. Same
    two mutations as slices 2/3; both suites named the failing case. Verified iter 154.
- .NET: `cargo build -p iscc-ffi` then
    `LD_LIBRARY_PATH=$PWD/target/debug dotnet test packages/dotnet/Iscc.Lib.Tests/` (104 total, ~160
    ms) and `--filter FullyQualifiedName~UnicodeBoundary` (13). `--list-tests` shows theories at
    **method** level only (3 names) — that is pre-discovery, not a missing test; the runtime failure
    line does carry the case name (`TextCleanBoundary(_: "test_0006_…", tc: {`).
- Kotlin: `cargo build -p iscc-uniffi`, then
    `packages/kotlin/gradlew -p packages/kotlin cleanTest test --offline` (`-p` avoids `cd`;
    `rootProject.rootDir` stays `packages/kotlin`). Read
    `build/test-results/test/TEST-uniffi.iscc_uniffi.UnicodeBoundaryTest.xml` — assert the
    `<testsuite … tests="13" skipped="0" failures="0" errors="0">` attributes AND the timestamp.
- **GRADLE UP-TO-DATE TRAP (found by Codex, confirmed, fixed iter 154):** a `Test` task only tracks
    its own project tree. A fixture outside it (here `crates/iscc-lib/tests/…`) is NOT an input, so
    after one green run an edit leaves `./gradlew test` `UP-TO-DATE` and the suite silently does not
    run — a stale green that survives a mutation probe if you forget `cleanTest`. Fix committed:
    `inputs.file(…).withPathSensitivity(PathSensitivity.NONE)` in the `tasks.withType<Test>` block.
    **3-run probe:** cleanTest+test (green) → test (must be UP-TO-DATE, no spurious re-run) → mutate
    fixture → test (must re-execute and red). CI is immune either way (fresh checkout, no gradle
    build-dir cache — the `kotlin` job is just checkout + `cargo build` + `./gradlew test`).
- Apply the same "is the fixture a declared build input?" question to the remaining surfaces:
    SwiftPM resources, CMake, and any C-FFI generated table.
- `packages/dotnet/{bin,obj}` and `packages/kotlin/build/` are gitignored, so build outputs of these
    slices never reach `git ls-files` or the drift gate.
- Kotlin's old "Deprecated Gradle Version" warning (wrapper 8.12.1 vs KGP 2.4.10) is GONE since the
    9.6.1 wrapper bump (iter 165) — any `Deprecat` line in a Kotlin build log is now a real finding.

### C FFI slice (iter 159, ~15 min) — generated tracked artifact, NOT a vendored copy

The C program has no JSON reader, so the fixture arrives as a **generated, tracked** pure-ASCII
header `crates/iscc-ffi/tests/unicode_boundary_vectors.h` (renderer
`scripts/gen_ffi_boundary_vectors.py`, PEP 723 stdlib-only). It is deliberately **absent** from
`VENDORED_COPIES` — that gate is byte-identity only. Rationale → `decisions.md` 2026-07-27.

- Commands: `cargo build -p iscc-ffi`; then the **verbatim ci.yml `c-ffi` gcc line** (proves no new
    `-I`); `LD_LIBRARY_PATH=target/debug /tmp/test_iscc` → `80 passed, 0 failed`;
    `gcc -Wall -Wextra -fsyntax-only …`; `uv run --script scripts/gen_ffi_boundary_vectors.py` then
    `git status --porcelain -- <header>` empty. `iscc-ffi` has **no `[features]`**, so there is no
    feature matrix to sweep and no risk the text symbols vanish from a non-default build.
- **Decode the header independently — do not trust `render() == header`.** Parse the two arrays,
    unescape the 3-digit octal (`\360`) back to code points, and diff every name/input/expected
    against the fixture, plus both `#define …_COUNT` macros and the version macro. A self-consistent
    generator and a *correct* one are different claims; the octal escaping is where a bug would
    hide.
- **Five mutations, all verified to red** (probe in `/tmp` — copy `test_iscc.c` + the header there,
    a quoted `#include` still resolves): (1) delete-filter-shaped expected value (Final Sigma
    `03C2`→`03C3`) → names the vector; (2) drop a case *and* its count macro → the hard-coded 7/5
    metadata guard reds; (3) version macro → `17.0.0` → guard reds; (4) hand-edit one octal digit in
    the tracked header → pytest drift anchor reds; (5) add a fixture vector without regenerating →
    same anchor reds. (4)/(5) touch the work tree — wrap in a `trap … EXIT` `git checkout`.
- Blind spots to state, not to fail on: the generator's `SECTIONS` dict silently ignores a *new*
    fixture section (class-wide — no binding suite gates that), and deleting block 29 of
    `test_iscc.c` outright reds nothing (also class-wide).
- gcc 12.2 (this container) does **not** warn on `strcmp`-macro literal-vs-`NULL`, so the
    `const char *` local advance added "for `-Waddress`" is harmless defensiveness, not a fix —
    probe such a claim by compiling the un-localised form before repeating it.

### C++ slice (iter 160, ~12 min) — reuses the C FFI header, no second artifact

**`cmake` IS available in this container** despite being absent from `$PATH`:
`uv run --with cmake cmake …` resolves the PyPI wheel (4.4.0). state.md's long-standing "C++ is not
buildable here" is wrong; the Swift half still stands (`uv --with swift` installs the unrelated
OpenStack package, not a toolchain).

- Commands (from repo root, **fresh** build dir — `packages/cpp/build*/` are all gitignored, and the
    stale `build/` holds a cmake 3.25 cache incompatible with 4.4.0): `cargo build -p iscc-ffi`;
    `uv run --with cmake cmake -S packages/cpp -B packages/cpp/build-revNNN -DCMAKE_BUILD_TYPE=Debug -DFFI_LIB_DIR=$PWD/target/debug -DSANITIZE_ADDRESS=ON`;
    `uv run --with cmake cmake --build …`;
    `LD_LIBRARY_PATH=$PWD/target/debug packages/cpp/build-revNNN/tests/test_iscc` →
    `69 passed, 0 failed`, exit 0 (ASAN/LSan clean ⇔ exit 0). `rm -rf` the dir afterwards.
- **The CMake build sets NO `-Wall -Wextra`** — "no compiler warning" is a near-vacuous criterion.
    Back it with a manual
    `g++ -std=c++17 -Wall -Wextra -Wpedantic -c packages/cpp/tests/test_iscc.cpp -o /tmp/tw.o -Ipackages/cpp/include -Icrates/iscc-ffi/include -Icrates/iscc-ffi/tests`.
- **Three mutations, all verified to red** (edit the tracked header, `cp` a backup first, restore +
    `git status` after): delete-filter-shaped expected value → names the vector; dropped case +
    decremented count macro → 7/5 guard reds; version macro → guard reds.
- **CMake depfiles DO track the cross-package header** — a bare `cmake --build` after a header edit
    recompiles, no reconfigure needed. So C/C++ are immune to the Gradle UP-TO-DATE stale-green
    class; mutation 1 proves this for free.
- **Public-INTERFACE leak probe** (the decisive check for a test-only include dir): write a
    throwaway `/tmp` project that `add_subdirectory`s `packages/cpp`, links only `iscc::iscc` and
    `#include`s the header — it MUST fail with `fatal error: … No such file or directory`. Note
    `packages/cpp/CMakeLists.txt` already puts `../../crates/iscc-ffi/include` on the **public**
    target, so cross-package relative paths are the house convention; vcpkg/conan never configure
    this project at all (both ship pre-built release tarballs), so the blast radius is
    `add_subdirectory` consumers only.
- `add_subdirectory(tests)` is unconditional in `packages/cpp/CMakeLists.txt` — pre-existing; a
    consumer configuring the package also configures the test target. Not introduced by this slice.

### Swift slice (iter 161, ~8 min) — the last surface; Swift IS locally runnable

**There is a working Swift toolchain in this container** at
`/tmp/swifttc/swift-6.1.2-RELEASE-debian12/usr/bin` (swift.org's Debian 12 x86_64 tarball; re-fetch
recipe in `packages/swift/CLAUDE.md`, 784 MB, no `sudo`). Every "Swift is macOS-only /
CI-proof-only" claim in state.md, handoff.md and the package CLAUDE.md was false. Pattern: two
consecutive iterations refuted a "not verifiable here" claim (cmake 160, swift 161) — treat the next
one as unproven.

- Commands (repo root): `cargo build -p iscc-uniffi`; then with the toolchain on `PATH`, from
    `packages/swift`:
    `swift test --scratch-path /tmp/swiftbuild-revNNN -Xlinker -L<repo>/target/debug -Xlinker -rpath -Xlinker <repo>/target/debug`
    → `Executed 12 tests, with 0 failures` (9 `ConformanceTests` + 3 `UnicodeBoundaryTests`; the
    trailing `Test run with 0 tests passed` is swift-testing finding no `@Test`, not a failure).
    Always use `--scratch-path` **outside** the repo. `swift package dump-package` at the repo root
    checks the *other* `Package.swift`.
- **12 tests ≠ 12 vectors.** Each vector method loops a whole section, so a section that resolves
    empty passes silently — the value of the separate 7/5 metadata guard. Probe it.
- **Three mutations in a `cp -r packages/swift /tmp/swiftprobe` copy** (never the work tree), all
    verified: (1) fixture expectation → the delete-filter value `U+00E9` reds with
    `("[101, 769]") is not equal to ("[233]")`; (2) *same mutated fixture* + `scalars(...)` swapped
    for a plain `actual, expected` → **suite goes GREEN**, which is the only way to prove the scalar
    comparison is load-bearing rather than decorative; (3) drop a `text_clean` case → metadata guard
    reds `("6") is not equal to ("7")`.
- **SwiftPM `.copy(...)` resources ARE declared build inputs** — mutation (1) red an *incremental*
    `swift test` with no clean, so Swift does not have the Gradle UP-TO-DATE hazard. That closes the
    "is the fixture a build input?" question for all 12 suites; Gradle was the only offender.
- Swift `String ==` folds canonical equivalence (`"e"+U+0301 == U+00E9`, `U+1100 U+1161 == U+AC00`),
    so any Unicode assertion must compare `text.unicodeScalars.map { $0.value }` arrays. Reject a
    "simplification" back to `XCTAssertEqual(actual, expected)` on strings.
- CI (`ci.yml` `swift` job, `macos-14`) runs `swift test` over the whole target, so a new
    `*Tests.swift` file needs no workflow edit — but confirm that in ci.yml.
- `.build/` is gitignored since iter 161; a bare `swift build` can no longer dirty the tree.
- `try!` / `as!` / `!` force-unwraps in these test files mirror `ConformanceTests.swift` and are
    fine — a crash is a test failure. Not a review finding.

## #43 ISCC-IDv1 fan-out (Part 2, started iter 177–178)

- **Python `gen_iscc_id_v1` landed iter 178** — `#[pyfunction]` returning `PyDict{"iscc"}`,
    `IsccIdResult(IsccResult)` wrapper, `_lowlevel.pyi` stub. Reviewed clean. Golden
    `gen_iscc_id_v1(1751831876325218,1,0)["iscc"] == "ISCC:MAIGHFECJMOPMIAB"`; realm nibble MA=0 /
    ME=1.
- **Minting ≠ decode round-trip** (Codex P1, iter 178): core `codec::Version` now has `V1`
    (`#[non_exhaustive]`) so low-level `_iscc_decode` returns `(6,0,1,0,<8b>)`, but each surface's
    OWN version enum (Python `VS` IntEnum, `__init__.py:82`) lists only `V0` → `iscc_decode(...)`
    raises `1 is not a valid VS`. A minting-only step legitimately leaves this; the decode
    round-trip is a separate #43 track needing the enum widened + a round-trip test per surface. #43
    body corrected.
- **Oracle needs no wheel**: `uv run --python 3.13 --no-project --with iscc-core python -c "..."` —
    the reference `gen_iscc_id_v1` is pure Python. Cross-check the binding against it directly.
- **napi minting landed iter 180** —
    `#[napi(js_name="gen_iscc_id_v1")] (timestamp: f64, hub_id: u16,   realm: u8) -> Result<String>`,
    `.map(|r| r.iscc)`. napi `iscc_decode` returns `version: u8`, so the decode round-trip works
    with NO enum widening (Go same). Reviewed PASS_WITH_NOTES.
- **JS-number surfaces silently coerce garbage → probe every f64/JS-number minting fn** (Codex P2,
    180): `f64 as u64` truncates/saturates (`-1`/`NaN`/`0.9` → `0`) and napi's `u16`/`u8` params
    coerce via JS ToUint32 (`0.9`→0, `2**32`→0), so invalid input mints a *different valid* ID
    instead of throwing. Core only guards the HIGH end (≥2^52 / ≥2^12 / realm∉{0,1}). Run the 6-case
    node probe (`-1`,`NaN`,`0.9` ts; `0.9`,`2**32` hub; `0.9` realm) — all should throw, currently
    don't. Filed as a normal `[review]` issue; **wasm will inherit this**, so expect the validation
    fix there. A minting slice that only covers next.md's high-end cases is still PASS_WITH_NOTES,
    not NEEDS_WORK (experimental fn, out of the slice's stated scope).
