# Define-Next Agent Memory — Archive

Archived scoping decisions from completed project phases. Moved here to reduce per-invocation
context loading. Full history preserved in git.

See MEMORY.md for current active entries.

## C# / .NET Binding Details (archived iteration 10 — phase complete)

- Three layers: C FFI → P/Invoke (NativeMethods.g.cs, csbindgen) → Idiomatic wrapper (IsccLib.cs)
- Package lives in `packages/dotnet/`, DLL name `"iscc_ffi"`
- csbindgen in build.rs, NativeMethods.g.cs committed (929 lines, 47 externs, 6 structs)
- Wrapper scoping: batched by marshaling complexity (string→string → byte[]→string → structs →
    arrays → streaming IDisposable)
- Jagged array marshaling: GCHandle.Alloc per inner array + fixed on outer + finally cleanup
- SafeHandle + IDisposable for IsccDataHasher / IsccInstanceHasher
- 91 tests (41 smoke + 50 conformance), System.Text.Json for data.json parsing
- NuGet pipeline: pack-nuget → test-nuget → publish-nuget in release.yml
- Cross-arch find bug fix: scope path pattern by target name `-path "*-${target}/*"`
- .csproj relative paths: count `../` from csproj location, not project root

## Streaming SumHasher (#37) + Python GIL release (#39) — archived iter 93, all closed

- **iter 88: SumHasher CORE** (`iscc_lib::streaming::SumHasher`). Composes the existing
    `DataHasher`+`InstanceHasher` (holds both, `update` feeds the same slice to both);
    `finalize(bits, wide, add_units) -> SumCodeResult` ports `gen_sum_code_v0`'s composition (calls
    `crate::gen_iscc_code_v0`); `gen_sum_code_v0` refactored to read file → drive the hasher.
    Intentionally **NOT** promoted to crate-root Tier 1 — Tier 1 (32 symbols) means "bound in all
    languages", but #37 only adds it to Python+WASM, so a count bump would imply false cross-binding
    parity. Kept README/rust-core.md/target.md counts at 32.
- **iter 89: Python SumHasher WRAPPER** — `crates/iscc-py/src/lib.rs` (PySumHasher mirrors
    PyDataHasher `Option<inner>`), `__init__.py` (wrapper class + `__all__`), `_lowlevel.pyi`.
    Stream handling lives in the Python wrapper; `_lowlevel` update takes `&[u8]` only.
- **iter 90: WASM SumHasher WRAPPER** — `crates/iscc-wasm/src/lib.rs`. Holds
    `Option<iscc_lib::streaming::SumHasher>` (full path, no re-export); finalize maps core
    `SumCodeResult` → `WasmSumCodeResult` (cast `filesize: u64 as f64`). Tests in `tests/unit.rs`
    via `wasm-pack test --node`. Closed #37 across all bindings.
- **iter 91: Python GIL release (#39)** — wrapped pure-Rust compute in `py.allow_threads(...)` at 7
    sites in `crates/iscc-py/src/lib.rs` (4 one-shot fns + 3 `update()` methods; added
    `py: Python<'_>` to the latter, invisible to Python). Keep `PyDict` construction OUTSIDE the
    closure. `allow_threads` is the PyO3 0.23 name (NOT `detach`, which is 0.25+).
- `gen_sum_code_v0` has a full test suite in lib.rs; streaming.rs has its own `#[test]`s; Python
    streaming tests live in project-root `tests/test_streaming.py`.

## npm #38 fix (iter 92) + semver-checks gate scoping (iter 93) — archived iter 95, both landed

- **iter 92: npm #38 (CLOSED)** — removed the `napi prepublish` step from `release.yml`, the only
    injector of dangling `@iscc/lib-<triple>` optionalDependencies. The bundled model ships all 5
    `.node` via `files: ["*.node"]`; the generated `index.js` loader `require`s the local
    `./iscc-lib.<triple>.node` first. Doc realignment in iscc-napi CLAUDE.md + notes 02/06.
- **iter 93: `cargo-semver-checks` gate (LANDED, informational)** — `ci.yml` `semver` job uses
    `obi1kenobi/cargo-semver-checks-action@v2` (`package: iscc-lib`, baseline auto-detected from
    crates.io 0.4.0) with `continue-on-error: true`. It WILL report the post-0.4.0
    `pub mod`→`pub(crate) mod` narrowing (cdc/conformance/dct/minhash/simhash/utils/wtahash) as
    breaking — expected; enforcing mode would turn CI red. Flip `continue-on-error` off only at the
    v1.0.0 cut. `mise run semver` mirrors it. Default features cover feature-gated Tier 1 symbols
    (`default = ["meta-code"]`).

## CRAP gate Phase 1/2 scoping (iters 94–96, landed)

- **iter 94 Phase 1 (LCOV)**: `coverage` job — `taiki-e/install-action@v2` (`tool: cargo-llvm-cov`)
    - `components: llvm-tools-preview`, `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info`,
        upload-artifact. `mise run coverage`. `lcov.info` gitignored.
- **iter 95/96 Phase 2 (report-only `cargo crap`)**: chose over iai-callgrind (valgrind-blocked) +
    PyO3. Scope = `.cargo-crap.toml` (create), `ci.yml` crap steps, `mise.toml [tasks.crap]`,
    ci-cd.md checkboxes 411/415/416/417. Install via `cargo binstall -y cargo-crap@0.2.2`. Two
    report-only steps `--format github` + `--format sarif --output crap.sarif` → `upload-sarif@v3`;
    job needs `security-events: write`. iter 96 RE-AFFIRMED same scope (loop ran update-state twice
    with no advance between). Landed `6ed51c5`, reviewed PASS `cbc0d14`.
- **Phase 2 exclude rationale**: LCOV is `-p iscc-lib` only → `cargo crap --path .` marks binding
    crates 0% (pessimistic) = noise. `.cargo-crap.toml` excludes all 7 binding crates +
    `packages/**`
    - `scripts/**` + `crates/iscc-lib/benches/**` (nested benches not covered by default exclude;
        leaked `bench_cdc_chunks` at CRAP 42.0). Use exclude globs NOT `--path crates/iscc-lib` to
        satisfy the "configures excluded binding crates" checkbox.

## Coverage/CRAP gate Phase 1 + network/env corrections (archived iter 98 — phases landed)

- **CORRECTION (iter 93): cargo registry network IS available.** `cargo search` returned live
    results; earlier `curl https://crates.io` 403 was Cloudflare blocking curl's user-agent, NOT a
    network block. The advance agent CAN `cargo install` tools + fetch new crate versions locally.
    Genuine local blockers: no valgrind (apt has no candidate — re-confirmed iter 98), no
    preinstalled CI dev tools. (Superseded/consolidated by the iter-98 env fact in MEMORY.md.)
- **iter 94: scoped Phase 1 of the coverage/CRAP gate (`cargo llvm-cov` LCOV +
    `mise run coverage`).** Chose this over iai-callgrind (no valgrind locally). Scope = 3 files
    (ci.yml `coverage` job, mise.toml `[tasks.coverage]`, .gitignore `lcov.info`) + ci-cd.md Phase 1
    checkbox. Used `taiki-e/install-action@v2` (`tool: cargo-llvm-cov`) +
    `components: llvm-tools-preview` in CI.
- **Env fact (iter 94)**: devcontainer has the `llvm-tools-x86_64-unknown-linux-gnu` rustup
    component preinstalled, so `cargo install cargo-llvm-cov` + `cargo llvm-cov -p iscc-lib` works
    locally.

## PyO3 #1 incremental migration hop-by-hop (iters 98–104; archived iter 105)

- 0.23→0.24 (iter 98), 0.24→0.25 (iter 99): PASSED with ZERO source changes.
- 0.25→0.26 (iter 101, review PASS iter 102): FIRST source-touching hop — 7 `allow_threads`→`detach`
    - 17 `PyObject`→`Py<PyAny>`.
- 0.26→0.27 (review PASS iter 103): SECOND source-touching hop — 2 sites in `to_pylist`:
    `downcast`→`cast`, `downcast_into_unchecked`→`cast_into_unchecked`.
- 0.27→0.28 (scoped iter 104, review PASS iter 104/12eb49f): compiled clean but PyO3 0.28 silently
    flipped the unspecified `#[pymodule]` `gil_used` default `true`→`false` (macros-backend
    `module.rs`: 0.27 `map_or(true,…)` → 0.28 `is_some_and(…)`). Review restored
    `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697) to keep the raw-FFI
    `extract_frame_sigs` path safe. Impact narrow (ships abi3 wheel, no free-threaded target) but
    bumps must be behavior-neutral.
- Lesson carried forward to the current entry: each minor is one reviewed step; advisories clear
    ONLY at 0.29; "compiles clean" ≠ behavior-neutral.
