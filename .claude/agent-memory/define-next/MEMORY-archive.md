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

## cargo-crap install-flake fix (iter 100→101) — RESOLVED, CI GREEN

`Swatinem/rust-cache@v2` restores cargo's `.crates.toml`/`.crates2.json` metadata WITHOUT the
`~/.cargo/bin/cargo-crap` binary, so `cargo binstall cargo-crap@0.2.2` (no `--force`) saw "already
installed", skipped, and the next `cargo crap` step died `no such command: crap` → CI RED every run.
Fix: `--force` on that binstall line (ci.yml:314). CI confirmed GREEN run 27685108728. Rule kept in
MEMORY.md CI-red-preempts-feature-work; generalized binstall+rust-cache gotcha lives in
learnings.md.

## iscc-py / PyO3 internals (archived iter 113 — PyO3 migration COMPLETE, issue #1 closed)

- iscc-py is ONE file (`crates/iscc-py/src/lib.rs`, ~735 lines); pyo3 used by NO other crate (grep
    `crates/*/Cargo.toml` → only iscc-py). Version pin lives ONLY at root `Cargo.toml` line ~35;
    iscc-py consumes via `workspace = true, features = ["extension-module"]`;
    `crates/iscc-py/pyproject.toml` line ~38 holds maturin config
    `features = ["pyo3/extension-module"]` (NO version → never edited on a hop). Has raw
    `pyo3::ffi::*` CPython-C-API calls (8 sites: PySequence_List, PyList_GetItem, PyList_Size,
    PyLong_AsLong, PyErr_Occurred, PyList_Check) + `Bound::from_owned_ptr().cast_into_unchecked()`
    (lib.rs:24) — map to stable CPython C API, rarely break across pyo3 minors. Local verify =
    `maturin develop -m crates/iscc-py/Cargo.toml` (maturin 1.12.4 via uv) + `uv run pytest` (286).
- Explicit `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:~697) MUST be preserved (PyO3
    0.28 silently flipped that default true→false).

## v0.6.0 iters 115–119 detail (archived iteration 121 — done + CI-verified)

- **iter 115: CI RED — enforcing `cargo-deny` caught RUSTSEC-2026-0204** (null-ptr deref in
    `crossbeam-epoch`, dev-only via criterion→rayon→crossbeam-deque, never shipped). Fix:
    `cargo update -p crossbeam-epoch` (0.9.18→0.9.20, patched `>= 0.9.20`, no manifest change).
    **Fix-the-root-cause > suppress**: when a patched release exists, bump the lockfile — `ignore`
    is the fallback only when unpatched. Verify the advisory `patched` range from
    `rustsec/advisory-db/main/crates/<crate>/<ID>.md` before choosing bump vs ignore. Recurring: the
    enforcing cargo-deny gate WILL periodically go red on fresh advisories vs dev/bench deps — each
    is a CI-red-first priority.
- **iter 116: #41 (Python text/video GIL) DONE** — single-file `crates/iscc-py/src/lib.rs`, wrap
    compute in `py.detach(|| ...)`. Injected `py` param not exposed → signature/conformance-neutral,
    no doc change. Video detach must open AFTER frame-sig extraction (borrowed `PyList_GetItem` ptrs
    not free-threading-safe; module keeps `gil_used = true`). Verify via
    `grep -c '\.detach(' lib.rs`.
- **iter 117: #42 (WASM simd128) — NEEDS_WORK.** RUSTFLAGS `simd128` alone does NOT activate
    blake3's wasm SIMD backend; `v128` opcode-counting is a FALSE-POSITIVE gate (LLVM
    auto-vectorizes the portable path too). Verify the actual reference/source before asserting a
    mechanism.
- **iter 118: reframed #42 (first NEEDS_WORK → reframe, not repeat) DONE.** wasm SIMD backend gated
    behind the `blake3/wasm32_simd` **Cargo feature** (`build.rs` emits `blake3_wasm32_simd` cfg
    only when `is_wasm32() && CARGO_FEATURE_WASM32_SIMD`; `platform.rs detect()` returns
    `WASM32_SIMD` unconditionally under that cfg). Fix = add
    `blake3 = { workspace = true, features =   ["wasm32_simd"] }` to `crates/iscc-wasm/Cargo.toml`.
    Both feature AND landed simd128 RUSTFLAGS required. Deterministic verify:
    `cargo tree -p iscc-wasm --target wasm32-unknown-unknown -f   "{p} {f}" -i blake3 | grep -q wasm32_simd`.
    No `unused_crate_dependencies` lint → no `use blake3 as _;` silencer needed.
- **iter 119: #43 (Go ISCC-IDv1) DONE** — pure-Go, self-contained (`packages/go`). `codec.go` adds
    `VSV1 Version = 1` + a MainType-ID-only Version=1 relaxation in `decodeHeader`; new `iscc_id.go`
    (`EncodeIsccID`/`DecodeIsccID`/`IsccIDv1Result`). Algorithm from `iscc_id.py::gen_iscc_id_v1`:
    `body=(timestamp<<12)|hubID`, big-endian 8 bytes, header MT=6/ST=realm/VS=1/len-index=0. Build
    the ID header via internal `encodeHeader`/`encodeLength` — do NOT relax public
    `EncodeComponent`. Vector: `EncodeIsccID(0,1,1751831876325218)` → `ISCC:MAIGHFECJMOPMIAB`. Go
    tests run from `packages/go/` (separate module); CI `working-directory: packages/go` +
    `CGO_ENABLED=0`.

## Dependency-refresh slices 1–2 + aarch64 wheels (iters 123–125) — archived from MEMORY.md (iter 126)

- **iter 123 (#49 aarch64 Python wheels, CI GREEN)**: 1 file `.github/workflows/release.yml` — added
    `ubuntu-24.04-arm`/`aarch64`/`python3.10` `build-wheels` entry (native ARM, NOT QEMU) +
    matrixified `test-wheels`. **Release-only infra → verification is STATIC** (pyyaml `safe_load`
    via `uv run python` + grep presence + `mise run check`); CID can't dispatch a real ARM release.
    Dev env: no actionlint/yamllint/system-pyyaml; pyyaml IS reachable via `uv run python`.
- **iter 124 — dep-refresh slice 1 = pure `cargo update`** (Cargo.lock only, generated → 0 source
    files). Caret ranges stayed put: `blake3 1.8.3→1.8.5`, `napi 3.8.3→3.11.0`,
    `uniffi 0.31.0→0.31.2`, `wasm-bindgen 0.2.125→0.2.126`, ~100 transitive. Two real risks handled:
    (a) cargo-deny can flip red on a new transitive license/advisory → `mise run audit`, prefer
    `cargo update -p X --precise <patched>` over a deny.toml ignore; (b) iai perf gate (CI-only,
    ≤10% Ir) can drift on blake3 → `mise run bench:iai:check`, refresh `.iai-baseline.json` via
    `mise run bench:iai:baseline` in-step if legit. Dev env has cargo-deny 0.19.9 + libclang-14 +
    valgrind, so `mise run test/lint/audit/bench:iai:check` all run locally.
- **iter 125 — dep-refresh slice 2 = Python `uv.lock`**: `uv lock --upgrade` at repo ROOT
    (regenerates `/uv.lock`, 2035 lines, generated → 0 source files). **Two separate uv projects:**
    root `/uv.lock` (dev tools + `iscc-core` + zensical/docs — the real one) and
    `crates/iscc-py/uv.lock` (7 lines, NO runtime deps → refresh is a no-op; don't touch). **Dev
    deps are all UNCONSTRAINED** (`"ruff"`, `"pytest"`, `"ty"`, `"mdformat"`, `"zensical"`…), so
    `--upgrade` pulls absolute latest → biggest risk is a tool major changing behavior. Handling:
    pin the ONE offending tool back in `pyproject.toml` `[dependency-groups] dev` with an inline
    hold-back comment (keeps the diff lockfile-only), NEVER disable a rule/skip a test/weaken a
    gate. Verify: `uv lock --check` + `mise run test/lint/check` + docs (`uv run zensical build`,
    `uv run python scripts/gen_llms_full.py`). CI Python job installs via `uv sync --group dev`;
    `docs.yml` runs zensical. `iscc-core` conformance is vs vendored `data.json` (authoritative).
    Landed with a documented `ruff<0.16` hold-back. uv 0.11.32.

## Archived from MEMORY.md at iteration 138

- **A stale in-repo comment is not worth blowing the file budget for** (iter 136, the `# held:` note
    on the `ruff<0.16` pin): leave it for the step that retires the thing it annotates, and say so
    in `## Not In Scope` so review reads it as deliberate, not missed.
- **`core.fileMode=false` (9p Windows bind mount) means `chmod +x` alone never lands in a commit** —
    `git update-index --chmod=+x <path>` is also required, run *after* `git add`, verified with
    `git diff --cached --summary`. The `+x`/`-x` round-trip is safe to probe while scoping. (Also
    recorded in the session-level `devcontainer-exec-bit.md` memory.)

## Parked-work scoping lessons (archived iter 148 — no issue is parked on Titusz right now)

- **A parked HUMAN REVIEW issue does not stall the loop — it re-prioritises it** (iters 134–139: the
    Unicode ruling parked 2 criteria, so steps went to ruff slices, then hook parity, then
    release.yml). If a blocked slice's only consumer is the parked propagation, take other backlog.
- **When a propagation step is parked, its source artifact can usually still land** (iter 141: the
    12-binding Unicode vector rollout was blocked on a human ruling + a Go decision, but the
    Rust-core fixture + loader was ruling-independent as long as the vectors avoided the disputed
    construct). Scope the artifact, name the parked half in `Not In Scope`, don't tick the spec box.
- **A parked behaviour ruling does not block *documenting* the behaviour that already shipped** —
    but the docs step must forbid the disputed claims by name in `Not In Scope` (iter 143: no
    output-equivalence claim, no multi-code-point/sequence statements).
- **A multi-part spec criterion slices along its own checkboxes** — `specs/rust-core.md`'s Unicode
    contract → separate steps (core change / sweep proof / binding propagation), not one.
