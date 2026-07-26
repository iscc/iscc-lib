# Update-State Agent Memory — Archive

Archived iteration-by-iteration findings from completed project phases. Moved here to reduce
per-invocation context loading. Full history preserved in git.

See MEMORY.md for current active entries. See git log for the complete historical record of all
archived entries.

## Closed milestones

- **Core SumHasher (iteration 88, `3fc44d2`)**: `pub struct SumHasher` in `streaming.rs:157`
    (new/update/finalize(bits,wide,add_units)/Default). `gen_sum_code_v0` (lib.rs:997) drives it.
    Reachable as `iscc_lib::streaming::SumHasher` but NOT a crate-root re-export (only
    `DataHasher`/`InstanceHasher` are, lib.rs:24). SumHasher NOT promoted to Tier 1 — counts stay
    32\.
- **Module visibility (iteration 86, `3f6a61d`)**:
    `cdc/conformance/dct/minhash/simhash/utils/wtahash` = `pub(crate) mod`; only
    `codec/streaming/types` = `pub mod`. Issue swept.
- **PyO3 0.23→0.29 migration hop history (issue #1, iters 98-105)**: incremental one-minor-per-step,
    completed iter 105. Edits per hop: 0.23→0.24, 0.24→0.25 = ZERO source edits; 0.25→0.26 =
    `allow_threads`→`detach` (7 sites) + `PyObject`→`Py<PyAny>` (17 sites); 0.26→0.27 (`acf9277`) =
    `downcast`→`cast` / `downcast_into_unchecked`→`cast_into_unchecked` (2 sites in `to_pylist`);
    0.27→0.28 (iter 104) = compiled CLEAN but SILENT `#[pymodule] gil_used` default flip
    `true`→`false`, restored with explicit `gil_used = true` (lib.rs:697); 0.28→0.29 (iter 105
    `8df611f`) = ZERO source edits, gil_used default did NOT flip again. Recipe: bump pin →
    `cargo update -p pyo3` → build/clippy(-D warnings)/fmt → `uv run maturin develop` →
    `uv run pytest` (286). 0.29 ships both RustSec advisory fixes; issue #1 closed.
- **cargo-crap install flake mechanism (iter 100, FIXED iter 101 `628c5d9`)**: the install step
    lacked `--force`; `Swatinem/rust-cache@v2` (ci.yml:304) restored cargo's `.crates` metadata
    WITHOUT the binary, so binstall SKIPPED ("already installed") and `crap` died (exit 101),
    recurring every run after the first green one. Fix = `--force` on the binstall.
- **Release-workflow gotchas (deep internals, rarely needed for state assessment)**: Kotlin release
    uses `useInMemoryPgpKeys` (env vars), Central Portal upload via curl REST (no Gradle plugin);
    JNA 5.16.0 `getNativeLibraryResourcePrefix()` maps `armv7`→`arm` so resource dir is
    `android-arm/` not `android-armv7/`; two Package.swift manifests coexist (root for distribution
    binaryTarget, packages/swift for CI dev; `releaseChecksum="PLACEHOLDER"` until first swift-input
    release); Kotlin JAR pick `ls *.jar | head -1` grabs `-javadoc.jar` — must `grep -v`
    classifiers.

## Gate pipeline detail (condensed out of MEMORY.md iter 115)

- **iai perf gate full pipeline** (`Perf (iai-callgrind)` ci.yml:281-333, enforcing): valgrind →
    binstall `iai-callgrind-runner@0.16.1 --force` → run benches (`IAI_CALLGRIND_ALLOW_ASLR=true`) →
    `Assert non-zero instruction collection` (`grep -rEq '^summary: [1-9]' target/iai/`) →
    `Check perf regression` (`python3 scripts/iai_regression.py --check`) → upload `iai-baseline`
    artifact `if: always()`. Baseline `.iai-baseline.json` (repo root, NOT gitignored):
    `{metric:"Ir", tolerance_pct:10.0, benches:{16 entries}}`. Tasks bench:iai / bench:iai:check /
    bench:iai:baseline (mise.toml ~126-144). GOTCHA: the `continue-on-error: true` near this block
    belongs to the SEPARATE semver job, NOT Perf.
- **CRAP gate full pipeline** (`Coverage + CRAP` ci.yml ~348, enforcing, job-level
    `security-events: write`): llvm-cov → cargo-binstall → `Install cargo-crap`
    (`cargo binstall -y --force cargo-crap@0.2.2`) → `cargo llvm-cov -p iscc-lib --lcov` → upload
    lcov → Phase 2 report-only (`--format github` + `--format sarif` →
    `codeql-action/upload-sarif@v3`) → Phase 3 enforcing (ci.yml:392-393)
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`.
    `--fail-above` = boolean keyed off `.cargo-crap.toml threshold = 30.0` (no numeric arg).
    Baseline `.crap-baseline.json` (repo root): 97 entries / 10 src files, regen
    `mise run crap:baseline`. `.cargo-crap.toml` excludes 7 binding crates +
    packages/scripts/benches. Max CRAP ~22.3 < 30. GOTCHA: yamlfix folds the long run: scalar onto 2
    physical lines — confirm via yaml.safe_load.
- **iai_regression.py** (248 lines, stdlib-only): `--check` parses `target/iai/**/*.out` summary
    lines vs `.iai-baseline.json`, fails >10% Ir. `check_regressions(allow_missing)`: shared over
    tolerance FAIL; shared zero-Ir FAILS regardless of flag; baselined-missing FAILS unless
    allow_missing; run-only new benches WARN. 11 fixture tests in tests/test_iai_regression.py.
- **iai_benches.rs**: 11 `bench_*` fns (9 gen + cdc + minhash) in
    `library_benchmark_group!(iscc_benches)`, 16 parametrized cases. `[[bench]] harness=false`.
    `[profile.bench] strip=false debug=true` (Cargo.toml:61) preserves `__iai_callgrind_wrapper`.
    `#[library_benchmark]` fns use `//` not `///` (macro abort!s on doc).
- **PyO3 migration** (issue #1 closed iter 105): pinned 0.29, lockfile single 0.29.0. Load-bearing
    `#[pymodule(name="_lowlevel", gil_used=true)]` (iscc-py lib.rs:697) — explicit b/c PyO3 0.28
    flipped default true→false.

## Archived iter 130 (stable, done work — pointers kept in MEMORY.md)

- **WASM SIMD (#42, iter 118)**: `crates/iscc-wasm/Cargo.toml` carries
    `blake3 = { features = ["wasm32_simd"] }` SOLELY for feature-unification — there is no
    `use blake3` in lib.rs, so do NOT prune it as unused. That **Cargo feature**, not RUSTFLAGS, is
    what activates the SIMD backend; `v128` opcodes in the .wasm are a WEAK signal — prove with
    `cargo tree -p iscc-wasm --target wasm32-unknown-unknown -i blake3`. Plus
    `wasm-opt --enable-simd` and simd128 RUSTFLAGS in ci.yml (wasm test) + release.yml (build).
- **Go ISCC-IDv1 (#43, iter 119)**: `packages/go/iscc_id.go` = `EncodeIsccID` / `DecodeIsccID` /
    `IsccIDv1Result`; `codec.go` adds `VSV1` and lets `decodeHeader` (~L271) accept Version=1 ONLY
    for MTId. Rust core rejects IDv1 at the header level (`codec::Version` is V0-only) by design.
- **Trailing-byte hardening**: Go `IsccDecode` (iter 120) has BOTH "too short" (L594) and "too long"
    (L597) branches; the Rust-core `iscc_decode` parallel fix landed iter 121. `iscc_decompose`
    legitimately consumes trailing units — never harden it.

## Closed work — do not re-flag as new (pre-iter-124, archived at 143)

aarch64 wheels #49 (123), CRAP baseline (122), trailing-byte hardening (120-121), Go ISCC-IDv1 #43
(119), WASM SIMD #42 (118), GIL release #39+#41, cargo-deny audit gate (113), iai-callgrind perf
gate (107-111), cargo-semver-checks job (93). CID loop infra = meta, NOT part of target.md — ignore
it when assessing.

## Environment gotchas (archived at 144)

- **Known non-regression**: the `proc-macro-error2 v2.0.1` future-incompat warning on cargo
    test/bench comes from `iai-callgrind-macros` (dev-only), NOT magnus/rb-sys; no upstream fix yet.
- **metrics.jsonl counts include gitignored build artifacts** — never read a metrics delta as real
    code change without `git diff --stat`.
- **Gradle flakes on this bind mount** (`Unable to delete file …/build/…`) — `./gradlew clean`
    before believing a Kotlin failure; check the test XML.

## Archived from MEMORY.md (iter 145) — package layout detail

- `packages/go/` — pure Go, no CGO/WASM (`golang.org/x/text` 0.40.0, `zeebo/blake3` v0.2.4, go
    1.26.1 consumer floor). The only binding that does NOT inherit the Rust freeze rule.
- `packages/swift/` + root `Package.swift` — `useLocalFramework` toggle, `.binaryTarget`,
    `build_xcframework.sh` builds 5 Apple targets.
- `packages/kotlin/` — Kotlin/JVM + JNA 2.4.10 (JNA, not JNI); consumer floor Kotlin 2.3+ since the
    iter-128 plugin bump; pin detail → `dep-refresh-survey.md`.

## Archived from MEMORY.md at iteration 146 (compaction)

- **release.yml static gate internals**: `scripts/check_release_workflow.py` = 23 fns; checks 1-2
    (guard shape 29 jobs/28 guarded, only `prepare-release` bare; input wiring; artifact wiring with
    `matrix.include` expansion; `needs:` graph) landed iter 142, all offline.
    `--check-action-inputs` (opt-in, NETWORK) landed iter 144: validates `with:` keys +
    `steps.<id>.outputs.<x>` against 18 distinct published `action.yml`. Prek hook
    `check-release-workflow` is release.yml-scoped and offline;
    `tests/test_check_release_workflow.py` = 19 tests, fetcher-injected so network-free; `pyyaml` is
    a dev dep.
- **Historical docs-list drift (measured iter 144, FIXED iter 145)**: `docs/llms.txt` carried only
    17 of 23 page links, missing `howto/{c-cpp,dotnet,kotlin,ruby,swift}.md` + `ruby-api.md` — 5 of
    11 languages invisible to LLM consumers. Downgraded Documentation to partially met for one
    iteration. Fixed plus gated by `scripts/check_docs_nav.py` at 145.
- **`docs.yml` ordering**: `zensical build` wipes `site/`, so `gen_llms_full.py` must run AFTER it
    (docs.yml has the right order). Verifying per-page `site/**/*.md` in the wrong order looks like
    every page is missing.
- **Iteration cadence log**: 140 tooling / 141 tests / 142 tooling / 143 docs / 144 tooling / 145
    docs+tooling.

## Archived from MEMORY.md at iteration 153 (compaction)

**`iterations.jsonl` crash fingerprints** (the ONLY place a crashed role surfaces; a non-OK status
does NOT mean no work — corroborate with `git log` for the `cid(<role>):` commit):

- Infra crash = `"status":"FAIL","turns":1,"cost_usd":~0.0006` **and no commit** (seen at 147).
- Benign overrun = `"status":"TIMEOUT"` **with** the commit present (148); the runner now logs these
    as `recovered`.
- A genuine *review* crash leaves three marks together: no verdict in handoff.md, a handoff
    containing only the advance section, and issues the review had resolved still in issues.md.
