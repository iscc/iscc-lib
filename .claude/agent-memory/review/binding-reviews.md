# Detailed Binding Review Recipes

Moved from MEMORY.md to keep the index concise. Referenced from MEMORY.md "Binding State".

## Binding State (all 9 crates met, 32/32 Tier 1)

- 9 binding crates (py, napi, wasm, ffi, jni, go, rb, uniffi). NAPI `index.js`/`.d.ts`/`*.node`
    gitignored (auto-gen by `napi build`). npm `@iscc/lib` bundled single-package (#38 closed):
    `files: ["*.node"]` ships all 5 binaries, NO `optionalDependencies`; `napi prepublish` must NOT
    run in `publish-npm-lib`. FFI constant count in module docstring must match additions (now 5)
- .NET + Swift bindings fully complete (32/32 Tier 1, CI, version sync, docs, release)

## Binding Propagation Shortcuts

- napi-rs: `npm test` + clippy + `mise run check`
- WASM: `wasm-pack test --node` (with and without `--features conformance`) + clippy. To run one
    test file: `--test <name>` goes BEFORE the `--` (cargo arg); after `--` the runner rejects it
- C FFI: `cargo test -p iscc-ffi` + clippy. Header tracked in git
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

- Magnus 0.7.1 pinned for Ruby 3.1 compat — 0.8 needs Ruby 3.2+
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
