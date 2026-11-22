# Advance Agent Memory

Detail lives in topic files: [ci-gates.md](ci-gates.md), [data-edge.md](data-edge.md),
[doc-checks.md](doc-checks.md), UniFFI/Swift/Kotlin (Gradle+JNA):
[uniffi-swift-kotlin.md](uniffi-swift-kotlin.md), [deps-refresh.md](deps-refresh.md),
[wasm-simd.md](wasm-simd.md), [go-idv1.md](go-idv1.md), [release-gate.md](release-gate.md),
[api-design.md](api-design.md), [bindings-detail.md](bindings-detail.md), Archived phases:
[MEMORY-archive.md](MEMORY-archive.md).

**Size budget:** Keep under 140 lines. Move detail to topic files; archive stale entries.

- [issues.md ledger rule](feedback-issues-ledger.md) — never edit issues.md; ledger → handoff Notes

## Code Locations

- Rust core: `crates/iscc-lib/src/` — lib.rs (crate root, Tier 1 re-exports), codec.rs, cdc.rs,
    minhash.rs, simhash.rs, dct.rs, wtahash.rs, utils.rs, streaming.rs, conformance.rs
- Conformance vectors: `crates/iscc-lib/tests/data.json` (iscc-core 1.5.0: 50 `gen_*` + 4
    `encode_seq` + 4 `decode_seq`, outputs `bytes:<hex>`/list). Vendored copies
    (packages/{dotnet,go,kotlin,swift}) gated by `tests/test_vendored_fixtures.py` — any NEW tracked
    `data.json` copy → register in `VENDORED_COPIES` or the test reds
- Data-edge vectors (215-282, 9 sections; fixture hand-formatted, splice as text):
    `crates/iscc-lib/tests/data-edge.json`, 12 harnesses, C/C++ via generated header, sections,
    output subset, Swift scalars, throwaway proof → data-edge.md
- GOTCHA: `clippy --all-targets --no-default-features` fails on benches (ungated imports) —
    `--tests`
- Doc example checks (238/243/244): Java/C++/Rust doc blocks compiled in /tmp → doc-checks.md
- C FFI docs (221): C type names `iscc_`-prefixed, gated by `tests/test_c_ffi_docs.py`
- C FFI slices (255): every ptr+len input via private `slice_or_empty` (len 0 → `&[]`, NULL+len>0 →
    named error) / `rows_or_empty` (parallel row arrays, `name[i]`). Never a bare `from_raw_parts`.
    .NET passes `fixed` over empty spans as-is. C++ cmake: `uv run --with cmake`,
    `-S packages/cpp -B /tmp/...`. Bulk exact edits: a `/tmp` Python script asserting match counts
- Bindings: Python `crates/iscc-py/python/iscc_lib/__init__.py`; Node `crates/iscc-napi/src/lib.rs`;
    WASM `crates/iscc-wasm/src/lib.rs`; C FFI `crates/iscc-ffi/src/lib.rs`; JNI
    `crates/iscc-jni/src/lib.rs` + `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/`
- Ruby: `crates/iscc-rb/` — src/lib.rs (Magnus), lib/iscc_lib.rb (wrapper); Cargo lib name `iscc_rb`
- Ruby floor/dev-Ruby pin (270/271), devcontainer skip-worktree edit (271) → bindings-detail.md
- UniFFI: `crates/iscc-uniffi/` — proc-macro interface for Swift/Kotlin, `publish = false`. Full
    detail → uniffi-swift-kotlin.md
- Go pure: `packages/go/` — one `.go` per algorithm/code-type (codec, utils, cdc, minhash, simhash,
    dct, wtahash, xxh32, `code_*.go`, conformance.go); pure Go only. ISCC-IDv1, decode guards, gofmt
    caveat → go-idv1.md
- Go 386 CI step (267), Go hashers (261), CDC params (263) → go-idv1.md
- Conformance selftest (276): ONE runner per language — Rust `run_section` + generic `report_case`
    over `Case<T> = Option<(IsccResult<T>, T)>` (section logic in closures: not CRAP-counted); Go
    `runSection[V any]` + `run<X>Case(tc) (got, want, err)`, `errUnparsable`, `unmarshalInputs`. A
    new section = one closure/case fn. rustfmt reflow shifts lines → regen CRAP baseline AFTER check

## Build and Tooling

- `cargo build -p iscc-jni` before `mvn clean test -f crates/iscc-jni/java/pom.xml` (native prereq;
    plain `mvn test` reuses stale classes). jni 0.22 → deps-refresh.md
- JNI errors (230): `throw_and_default(env, &e)` — `JavaError` trait picks class (NoMemory → OOM);
    never `&e.to_string()`, `String` has no impl (`.as_str()`). jni `*_with_catch*` macros CLEAR the
    JVM OOM. OOM test = child JVM (`OomProbe`, `-Xmx16m`); never null-test unfixed (SEGV)
- JNI loader (278/279): child-JVM `LoaderProbe` tests get their fallback dir from
    `assertLoaderProbePasses` (lib-path entry, else bundled resource copied to temp) — works in CI +
    release layouts. CI `java` job's 2nd step runs the release layout. Probe tree:
    `git ls-files … |   tar -cf - -T - | tar -x` (no rsync; carries uncommitted edits)
- CI `.github/workflows/ci.yml`: 21 jobs (version-check, rust, python-test, python, nodejs, wasm,
    c-ffi, dotnet, java, go, ruby, ruby-source-gem, cpp, swift, kotlin, bench, perf, semver,
    coverage, audit, release-workflow). `bench`=`cargo bench --no-run`; `swift` macos-15 (macos-14
    retired 2026-11-02, iter 196); `kotlin` ubuntu JDK17
- Enforcing gates: `coverage` (CRAP baseline `.crap-baseline.json`), `audit` (cargo-deny), `perf`
    (iai vs `.iai-baseline.json`); `semver` informational until v1.0.0. CRAP gate is CI-ONLY —
    refresh `mise run crap:baseline` in the SAME step as any branch-adding change → ci-gates.md
- Audit: advisory w/ patched release → `cargo update -p <crate>`, never `deny.toml` `ignore`.
    cargo-deny not installed: prebuilt 0.19.9 musl tarball from GitHub releases → /tmp (235)
- Go toolchain (235): `go.mod` has `go 1.26.1` (consumer floor, human-only) + `toolchain go1.26.11`
    (setup-go `go-version-file` uses it); devcontainer `GO_VERSION` mirrors the toolchain line.
    `govulncheck@latest` via `go run golang.org/x/vuln/cmd/govulncheck@latest ./...`
- Dependency refresh: NEVER blanket ruff `--fix` (deletes `# noqa: S603/S607`) → deps-refresh.md
- GOTCHA (233): `prek run --all-files` (= `mise run check`) skips UNTRACKED files — `git add -N` new
    scripts/tests BEFORE the check, else ruff findings surface only at commit
- Commit quirks (217): Ruby `standardrb --fix` hook takes minutes (long timeout); trust
    `prek run --files` over bare `uv run mdformat`; never edit files while `mise run check` runs.
    WSL `index.lock` race after prek's stash-restore: retry `git commit` in a sleep loop (226)
- GOTCHAs: ci.yml `cancel-in-progress: true` per ref — a second develop push cancels the previous
    sha's in-flight CI run; system `python3` lacks PyYAML (use `uv run python`); piping cargo-crap
    or cargo-deny to a pager makes `$?` the pager's exit — redirect to a file instead
- Ruby CI job: libclang-dev required; ruby/setup-ruby@v1 `working-directory` is a `with:` param.
    `rust` job matrix: `--no-default-features` / `--all-features` / `--features text-processing`
- Adding/renaming a ci.yml job requires updating the job table in `.claude/context/specs/ci-cd.md`
    same commit — parity-gated by `scripts/check_ci_job_table.py` (prek `check-ci-job-table` +
    pytest anchor; backticked first-column keys, count floor 10)
- `version-check` job: `scripts/version_sync.py --check` (`TARGETS` list; exits 1 on mismatch). Go
    CI job has zero Rust deps. napi lock (284) is a target: get/sync both root version fields
- napi (284): `crates/iscc-napi/package-lock.json` committed, CI + release use `npm ci`; lock
    @napi-rs/cli 3.10.12 `napi build` FAILS on the 9p `/workspace` mount (fs-transaction
    AggregateError, deterministic) — prove builds in a `/tmp` worktree with
    `CARGO_TARGET_DIR=/workspace/iscc-lib/target`; clean the `.napi-rs-filesystem-transaction.swp`
    and `.*.tmp` leftovers
- Generated-artifact gates (274), pytest addopts (254), Kotlin plugins (262), .NET CI (260) →
    ci-gates.md
- Python dev builds: `uv run maturin develop -m crates/iscc-py/Cargo.toml` (always via uv run).
    Single `cp310-abi3` wheel. GOTCHA: `uv sync` uninstalls the editable ext — re-run maturin after
- PyO3 pin = single source: root `Cargo.toml` (`pyo3` "0.29", `abi3-py310`); ONLY `crates/iscc-py`
    consumes it. KEEP `#[pymodule(name="_lowlevel", gil_used=true)]` (lib.rs:697) — 0.28+ defaults
    `gil_used` false, unsafe for raw `PyList_GetItem` ptrs. GIL release (#39/#41): 12 `py.detach`
    sites; video detach opens after frame-sig extraction. Recipe → MEMORY-archive.md
- release.yml: static gate `scripts/check_release_workflow.py` (prek hook, opt-in
    `--check-action-inputs`) + shape (9 inputs, 97 `uses:` refs) — keep green → release-gate.md
- WASM returns (234) / JS ints (256, 268, 269) / published web pkg + Node init (283) →
    bindings-detail.md
- WASM SIMD (#42): `blake3/wasm32_simd` + RUSTFLAGS simd128 + wasm-opt `--enable-simd` →
    wasm-simd.md

## API Design

- `iscc_decode` strips "ISCC:" + dashes, returns digest bytes `(u8,u8,u8,u8,Vec<u8>)`
- Python strip set `codec::is_py_strip_char` (`pub(crate)`) / Go `isPyStripChar`: codec + text
    trim/split (203). Base32 strict (199) + ASCII-only casing (201)
- Codec input cleaning `iscc_clean` (Rust 191-198; Go `isccClean` 200, 4 sites) + perf →
    codec-clean.md
- ISCC-IDv1 #43 COMPLETE (iter 190, no dedicated decoder; Tier-1 = 35 since 211) → go-idv1.md
- ISCC-SEQ #45: core (195) + all 11 surfaces bound (197-210, UniFFI last); docs swept 33→35 (211) →
    iscc-seq.md. iscc-ffi tests in `src/tests.rs` (231); exports MUST stay in lib.rs (csbindgen
    reads only it)
- Video ragged input (250), Python int range (265), Go meta JCS (251/266), `alg_simhash([])` (225),
    `json_to_data_url` → api-design.md

## Documentation Files / Feature Flags / Spec path gate → MEMORY-archive.md

- Docs page = disk + `zensical.toml` nav + `ORDERED_PAGES` + `docs/llms.txt` (`check_docs_nav.py`)
- Tier 1 doc gate (233) `scripts/check_tier1_docs.py`: FULL/EXACT/EXEMPT registry + discovery (≥30
    symbols). New API symbol → update every FULL doc + both `tier1:begin` blocks in the same step.
    Fail-closed lib.rs parse (242): `scan_lib` classifies each column-0 `pub` by offset; a new
    non-derived crate-root item needs an `EXCLUDED_ITEMS` entry (named, never kind-wide)
- Per-crate `CLAUDE.md` files carry no file trees/tables (246): a rule goes in prose against a file
    name. Line-range edits via an anchored `/tmp` Python script; mdformat reflows inserted lines
- Spec drift sweeps (259): mdformat also reformats \`\`\`js blocks; release-trigger truth =
    release.yml `on:` + each job's `if:` — grep `refs/tags`/`github.ref` too, next.md greps miss
    YAML conditions
- Vector access per surface (280): C test = hard-coded expected values + generated
    `data_edge_vectors.h` (no data.json); C++ = library `conformance_selftest` + header; .NET/Swift/
    Kotlin = vendored copies; Go gen tests read `../../crates/iscc-lib/tests/data.json`, selftest
    embeds `testdata/`. Sum-Code types: napi `NapiSumCodeResult`, Java `SumCodeResult`
- Docs facts probed (277): Sum-Code / `gen_iscc_code_v0([data,inst])` 64-bit prefix = `KUA`; Ruby
    integer params + list elements go through `to_int` (Float truncates toward zero, NaN →
    `FloatDomainError`). Release auth per job: read `permissions`/`secrets.*` via a PyYAML dump of
    release.yml jobs; `publish-ffi` is the only publish job without an exists-check
- Release-history claims (275): verify with `git grep '<pattern>' <tag> -- <path>` per tag — next.md
    said Go IDv1 old symbols "removed in 0.6.0"; they were never tagged
- Guidance docs carry no counts/`path:NN` (245): write "every binding", "all Tier 1 symbols". Line
    greps miss wrapped counts — also grep each file joined via `tr '\n' ' '`
- Spec path gate `scripts/check_spec_paths.py`: `*` one segment, `**` crosses `/`; `NOT_PATHS`
    backticked spans only (228). Fix phantoms in the spec, never by adding a file. Every spec tree
    needs a following paragraph with `not every file` (247); fixture line numbers shift with notes

## Bindings detail → [bindings-detail.md](bindings-detail.md)

- Ruby source gem / Ruby `i32` / .NET jagged + NUL / C++ empty vectors / npm metadata gotchas
