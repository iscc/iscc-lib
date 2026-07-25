<!-- assessed-at: 8358ebad5dd901c4496c30595414ceef63ad12d1 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — CI GREEN; both parked policy calls DECIDED, opening real implementation work

v0.5.0 is released across all registries and CI is fully green. The defining change this iteration
is **not code — it is the target**: two `human(decide)` commits (`8d267ff`, `8358eba`) resolved both
HUMAN REVIEW REQUESTED policy calls and wrote the outcomes into `specs/rust-core.md` and
`specs/kotlin-bindings.md` as **four new verification criteria**. The loop went from "waiting on
Titusz with only ruff 0.16 left" to having a substantial, fully-specified implementation backlog: a
Unicode 16.0.0 freeze rule plus boundary conformance vectors, and a docs-only Kotlin floor note.

## Rust Core Crate

**Status**: partially met — CI GREEN, but the target **grew**: 3 new unmet Unicode criteria, plus
the still-held semver/v1.0.0 criterion

- Core API met (unchanged, no Rust source touched this iteration): all 10 `gen_*_v0` functions, 32
    Tier 1 symbols, all vendored `iscc-core/data.json` vectors passing, `Rust (fmt, clippy, test)`
    GREEN. Workspace version `0.5.0`, 11 source modules, **320** `#[test]` functions (re-counted).
    `grep -c '# held' Cargo.toml` → **4**.
- **The Unicode divergence is no longer a conformance failure — it is now a declared contract with
    unimplemented machinery.** `specs/rust-core.md` gained a "Unicode data version is part of the
    conformance contract" section: **declared version 16.0.0** (CPython 3.14's tables), and the spec
    explicitly states that divergence from `iscc-core` on runtimes with non-16.0 tables is "expected
    and documented, not a conformance failure". The Rust core's current output is therefore *correct
    by the new spec*; what is missing is the machinery that makes it stable and gated.
- **Unmet criterion 1 — the freeze rule is NOT implemented.** Verified in
    `crates/iscc-lib/src/utils.rs`: `is_c_category` (L26-33) and `is_cmp_category` (L38-59) still
    call `get_general_category` on the live `unicode-general-category` tables and strip
    `GeneralCategory::Unassigned` inline. There is **no** pre-normalization filter, **no** vendored
    unassigned-ranges table (the spec calls for 731 ranges / 819,533 code points) and **no**
    generator script — `ls scripts/` shows only `build_xcframework.sh`, `gen_llms_full.py`,
    `iai_regression.py`, `test_install.py`, `version_sync.py`. `unicodedata2` appears nowhere in
    `pyproject.toml` or `uv.lock`.
- **Unmet criterion 2 — boundary conformance vectors do not exist anywhere.** A repo-wide grep for
    the three code points the spec names (`U+1FAE9` retained, `U+113C5` decomposing, `U+20C1`
    stripped) returns **zero hits** across all `.rs/.py/.go/.json/.kt/.swift/.js/.ts` files. The
    criterion requires them in the Rust suite **and in every binding's conformance test**, so this
    one item spans all 12 bindings.
- **Unmet criterion 3 — no full-code-space differential sweep** proving the freeze-rule
    implementation output-equivalent to uniform Unicode 16.0.0 tables (nothing to sweep yet). The
    spec also makes this sweep mandatory on *every* future Unicode-table dependency bump.
- Per the spec, **no dependency change is needed**: `unicode-general-category` 1.1.0 (16.0.0) and
    `unicode-normalization` 0.1.25 (17.0.0) both qualify once the freeze filter runs first. Upstream
    counterpart filed as <https://github.com/iscc/iscc-core/issues/137>.
- **Unmet (held): semver enforcing at v1.0.0** — the `Semver (cargo-semver-checks)` job still
    carries `continue-on-error: true`; the `rust-core.md` semver box stays `[ ]`, held by Titusz.
- Rust pin hold-backs standing (criterion 0.8 needs rustc 1.86, jni 0.22, magnus 0.8, uniffi 0.32,
    plus the `pyo3` `# note:`). Trailing-byte hardening intact. CRAP + perf gates GREEN.
- **Known constraint (not a regression)**: the `proc-macro-error2 v2.0.1` future-incompat warning on
    `cargo test`/`bench` comes from `iai-callgrind-macros` (dev-only).

## Python Bindings

**Status**: met — plus a share of the new cross-cutting boundary-vector criterion

- All 32 symbols exported; `Python 3.10` + `Python 3.14` matrix jobs GREEN behind the `if: always()`
    aggregator `Python (ruff, pytest)`. PyO3 `0.29`, `abi3-py310`. GIL release complete (12
    `.detach(` sites). aarch64 wheels wired.
- **Changed this iteration (ruff 0.16 slice A)**: `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`
    lost its 36 lone `...` placeholder lines (docstring-only bodies now); re-verified: `0`
    bare-`...` lines, 27 top-level `def`, 440 lines. `tests/test_new_symbols.py` `_`-prefixed 6
    unused unpacked bindings. Validated against ty/mypy/pyright in review — the stub ships with
    `py.typed`, so it is consumer-facing.
- **`ruff<0.16` hold-back still in `pyproject.toml:27`** with its `# held:` comment. Re-measured
    live: `uvx ruff@0.16.0 check . --statistics` → **26** errors (RUF100 15, I001 8, EXE001 1,
    PLW1510 1, RUF022 1), 24 auto-fixable. Confirmed independently: `[tool.ruff.lint]` declares **no
    `select` key** — only `mccabe` and `per-file-ignores` subsections exist — so `S` and `C901` run
    *only* via the two pre-push hooks' explicit `--select`, never in `mise run check`.
- Pending: Unicode boundary vectors in the Python conformance test (new criterion, see Rust Core).

## Node.js Bindings

**Status**: met — plus the cross-cutting boundary-vector criterion

- All 32 Tier 1 symbols with TypeScript declarations; `Node.js (napi build, test)` GREEN
    (`setup-node@v7`, node 20). `package.json` closed as verified current. Untouched this iteration.

## WASM Bindings

**Status**: met — plus the cross-cutting boundary-vector criterion

- All 32 Tier 1 symbols via `#[wasm_bindgen]`, streaming `SumHasher` class, `WASM (wasm-pack test)`
    GREEN. SIMD backend active — the `blake3 = { features = ["wasm32_simd"] }` dep exists solely for
    feature unification (no `use blake3`; must not be pruned). Untouched this iteration.

## C FFI

**Status**: met — plus the cross-cutting boundary-vector criterion

- **47** `#[unsafe(no_mangle)]` extern entry points, committed cbindgen header + freshness check, C
    test passing, csbindgen generating `NativeMethods.g.cs`; `C FFI (cbindgen, gcc, test)` GREEN.

## Java Bindings

**Status**: met — plus the cross-cutting boundary-vector criterion

- 32 Tier 1 symbols via JNI, native libs bundled in the JAR; `Java (JNI build, mvn test)` GREEN
    (`setup-java@v5`, temurin 17). `jni` held at 0.21. `pom.xml` pins current;
    `central-publishing-maven-plugin` deliberately held at 0.7.0 (only exercised by a real publish).

## Go Bindings

**Status**: met — but the **most exposed** binding under the new Unicode criteria

- Pure Go (no CGO), all 32 Tier 1 symbols, `Go (go test, go vet)` GREEN, `CGO_ENABLED=0` holds.
    `go.mod` fully refreshed (slice 6): `x/text v0.40.0`, `zeebo/blake3 v0.2.4`. ISCC-IDv1 support
    and both `IsccDecode` length-rejection branches intact. Untouched this iteration.
- **Caveat now written into the target**: Go's stdlib/`x/text` tables are Unicode **15.0.0**
    (`tables17.0.0.go` is `//go:build go1.27`, ~Aug 2026), so Go cannot satisfy the boundary vectors
    without help. The issue prescribes the choice: vendor the 15.0→16.0 assigned delta (5,813 code
    points with their 16.0 categories) **or** skip the boundary vectors for Go with a tracking note.
    Its current agreement with `iscc-core` on post-15 text is incidental and inverts under go1.27.

## Ruby Bindings

**Status**: met — plus the cross-cutting boundary-vector criterion

- 32 Tier 1 symbols via Magnus; `Ruby (magnus build, test)` GREEN. `magnus` held at 0.7 (0.8 drops
    `old-api` → 5 `exception::runtime_error()` call sites). `Gemfile.lock` refreshed (slice 7) with
    two `held:` pins: `rb_sys` exact **`0.9.123`** (must move together with `Gemfile.lock` **and**
    `tag:` at `release.yml:853`) and `minitest ~> 5.0`. Untouched this iteration.

## C# / .NET Bindings

**Status**: met — plus the cross-cutting boundary-vector criterion

- 32 public symbols via P/Invoke over the C FFI; `C# / .NET (dotnet build, test)` GREEN
    (`setup-dotnet@v6`, SDK 8.0). Manifests verified current; only xunit 3.x / Test.Sdk 18.x majors
    remain, each its own step.

## C++ Bindings

**Status**: met — plus the cross-cutting boundary-vector criterion

- C++17 header-only wrapper, all 32 Tier 1 symbols, ASAN clean, vcpkg + Conan;
    `C++ (cmake, ASAN, test)` GREEN.

## UniFFI Scaffolding Crate

**Status**: complete (internal, `publish=false`)

- **32** `#[uniffi::export]` annotations, shared by Swift + Kotlin. Held at 0.31 (0.32 needs
    Swift/Kotlin regeneration, unverifiable in the Linux devcontainer).

## Swift Bindings

**Status**: met — plus the cross-cutting boundary-vector criterion

- SPM package with UniFFI bindings, all 32 Tier 1 symbols, XCFramework build;
    `Swift (swift build, swift test)` GREEN on macos-14. Checksum current for v0.5.0.

## Kotlin Bindings

**Status**: partially met — a **new, explicit, docs-only** criterion is unmet (was "met + open
`[review]` issue" last iteration)

- `packages/kotlin/` with JNA-loaded UniFFI bindings, 9 desktop + Android targets;
    `Kotlin (gradle build, test)` GREEN. Refreshed pins: `kotlin("jvm") 2.4.10` (confirmed in
    `build.gradle.kts:2`), JNA `5.19.1`, junit-jupiter `5.14.4`. Untouched by code this iteration.
- **DECIDED by Titusz**: keep `kotlin("jvm") 2.4.10`; the supported consumer floor is **Kotlin 2.3
    or newer**. `specs/kotlin-bindings.md` gained a "Supported consumer Kotlin version" policy
    section, refreshed its stale `jna:5.16.0@aar` → `5.19.1` lines (the drift noted last iteration
    is now fixed), and a new verification criterion. **Do not touch `build.gradle.kts`.**
- **Unmet criterion (verified by grep, all three files)**: the floor is stated **nowhere** in
    `packages/kotlin/README.md`, `docs/howto/kotlin.md`, or the root README Kotlin section — each
    shows only the `jna:5.19.1` dependency line, with no Kotlin version requirement. This is the
    cheapest open work package in the project and is **release-blocking** for the next Maven Central
    publish of `io.iscc:iscc-lib-kotlin`.

## README

**Status**: met (pending the Kotlin floor line above)

- Polyglot README with CI + registry badges, per-language install/quick start for all 12 languages,
    architecture + MainTypes. The Kotlin section needs the "requires Kotlin 2.3+" sentence.

## Per-Crate READMEs

**Status**: met (pending the Kotlin floor line above)

- READMEs present for all 12 crates/packages. **Standing correction re-verified**:
    `scripts/version_sync.py` `TARGETS` holds **21** entries (`--check` → 21 `OK:` lines). issues.md
    still says "22 targets now" at line 55 — that number is wrong; do not propagate it.

## Documentation

**Status**: met (one low-priority cosmetic gap; pending the Kotlin floor line)

- Docs site, **11** `docs/howto/*.md` guides, tabbed examples, llms-full.txt (22 pages), benchmarks
    page with speedup factors. No docs content changed this iteration.
- `docs.yml` action bumps remain statically verified only — the workflow triggers on push to `main`,
    so `upload-pages-artifact@v5` / `deploy-pages@v5` get their first real run on the next
    develop→main merge (PR #44 "Release 0.6.0" is open). First suspect if the docs deploy breaks.
- **Gap (low, CID skips)**: language logos in `docs/index.md` / howto headers — cosmetic.

## Benchmarks

**Status**: met

- **12** criterion `bench_*` functions in `benchmarks.rs`; `Bench (compile check)` GREEN on
    criterion 0.7 with `black_box` from `std::hint`. **18** pytest-benchmark functions, speedups
    (1.3x-158x) published in `docs/benchmarks.md`.
- Second harness `iai_benches.rs` (iai-callgrind 0.16, 11 fns → 16 cases) enforcing the Ir gate at
    10% — GREEN against the unmodified `.iai-baseline.json`.

## CI/CD and Publishing

**Status**: partially met — **CI GREEN**; dependency freshness is the only CID-doable gap left here

- **LATEST CI RUN — SUCCESS.** origin/develop tip `feb5ea4` (the iteration-131 review commit): **41
    check-runs, 21 distinct names, 0 non-success, 0 still running** — verified via
    `gh api "repos/iscc/iscc-lib/commits/feb5ea4.../check-runs?per_page=100"`. 41 rather than ~20
    because the open develop→main PR **#44** makes every develop commit fire both a `push` and a
    `pull_request` run.
- **The 3 unpushed commits on HEAD are context-only** — diffing `origin/develop..HEAD` with
    `.claude/` excluded returns an **empty** stat, so no code sits outside CI coverage (`90ccabd`
    iteration log + the two `human(decide)` commits, which touch only
    specs/issues/decisions/learnings).
- ci.yml: **19 job entries → 20 jobs → 21 distinct check names** (`python-test` is a 3.10/3.14
    matrix, `python` is an `if: always()` aggregator). Untouched this iteration.
- **Dependency refresh — all 7 per-ecosystem slices DONE** (Cargo.lock 124, uv.lock 125, Rust pins
    126, GHA refs 127, JVM 128, go.mod 129, Ruby 130). Remaining under the `normal` `[human]` issue:
    **ruff 0.16 slices B/C/D** (the last fully-local, CID-doable item), `release.yml` action refs
    (human-timed), and the deferred majors — magnus 0.8, jni 0.22, uniffi 0.32, criterion 0.8, xunit
    3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x.
- **Standing correction**: `release.yml` contains **no `astral-sh/setup-uv` step** — the issues.md
    claim is wrong. Its real lag: `checkout@v4`, `download-artifact@v4`/`upload-artifact@v4` (must
    move as a pair), `setup-java@v4`, `setup-node@v5`, `setup-dotnet@v4`, `setup-python@v5`,
    `cache@v4`. Nothing there is exercised by a CID push, so it stays human-timed.
- **cargo-deny enforcing** — `Audit (cargo-deny)` GREEN; a fresh live advisory can flip it red with
    no code change (prefer `cargo update -p <crate>` over a `deny.toml` ignore).
- **Gap (v0.6.0)**: no automated dependency updates — `.github/dependabot.yml` and `renovate.json`
    both confirmed absent (deliberate; refresh is a manual per-release pass).
- v0.5.0 published; release workflow with 8 per-registry toggles + version sync (**21** `TARGETS`).
    Two release-workflow reliability issues open (npm OIDC, single-registry re-trigger) — both
    human-gated `normal`.

## Open Issues (issues.md lists 7 — 0 critical, 5 normal, 2 low)

CI is green — no open issue is CI-blocking. Any open issue keeps the project IN_PROGRESS. **Net
change this iteration: both `[review]` issues became DECIDED `[human]` work packages — there are now
ZERO `HUMAN REVIEW REQUESTED` blocks.** No issue was opened or closed.

CID-doable now (`normal`, `[human]`, spec'd):

- **Document the Kotlin consumer floor as 2.3+ (DECIDED)** — docs-only, 3 files, release-blocking.
    Spec: `specs/kotlin-bindings.md` → "Supported consumer Kotlin version".
- **Declare and gate a Unicode data version (DECIDED)** — 16.0.0 + freeze rule. The issue gives an
    explicit implementation order: (a) vendored unassigned-ranges table + checked-in generator
    script + freeze filter in the Rust core, proven by a full-code-space differential sweep, then
    (b) boundary vectors in the Rust suite and every binding (Go caveat above). Spec:
    `specs/rust-core.md` → "Unicode data version is part of the conformance contract".
- **Dependency review and refresh** — ruff 0.16 slices B/C/D remain. Spec: `specs/ci-cd.md` →
    "Dependency Freshness".

Human-gated release-workflow reliability (`normal`, `[human]`):

- Migrate npm publishing to OIDC Trusted Publishing (needs npm-side config first)
- Fix broken single-registry re-trigger in `release.yml`

Low (human-directed, CID skips):

- Release core as v1.0.0 — **held** by Titusz (stay on 0.5.x; flip Semver to enforcing at the cut).
- Add programming language logos to docs site — cosmetic.

## Next Milestone

**CI is green — no CI fix needed.** For the first time in several iterations there are no decision
blocks: both formerly-parked items are decided, specified, and actionable. Recommended order
(cheapest-and-release-blocking first, then the largest target gap):

1. **Kotlin floor docs — one small step, do it first.** State "requires Kotlin 2.3+" in
    `packages/kotlin/README.md`, `docs/howto/kotlin.md`, and the root README Kotlin section,
    phrased consistently with the spec's policy section. Do **not** touch `build.gradle.kts`
    (2.4.10 is the decided compiler) and do not redo the spec edits — they are already done.
2. **Unicode freeze rule, step (a) — the largest remaining target gap.** Vendored unassigned-ranges
    table (731 ranges) + checked-in generator script (from `unicodedata2==16.0.0`) + the
    pre-normalization filter in `text_clean`/`text_collapse`, with a full-code-space differential
    sweep as the evidence. Keep it to step (a); step (b) is a separate iteration.
3. **Unicode freeze rule, step (b)** — boundary vectors (`U+1FAE9` retained, `U+113C5` decomposing,
    `U+20C1` stripped) in the Rust suite and every binding's conformance test. Decide the Go
    approach explicitly (vendor the 15.0→16.0 delta, or skip-with-tracking-note until go1.27).
4. **ruff 0.16 slices B → C → D.** B: the isort cluster (`I001` 8 + `RUF022` 1) — settle
    `known-first-party` first. C: add `S` and `C901` to `[tool.ruff.lint] select` (verified absent
    today), which both promotes the security/complexity scans into the fast local loop **and**
    clears all 15 `RUF100` at once — never delete the `# noqa` directives; `EXE001` + `PLW1510`
    ride along. D: drop the `<0.16` pin only once `uvx ruff@0.16.0 check .` exits 0. Live count
    today: 26 findings.
5. **magnus 0.8 / jni 0.22 migrations** — each its own step with a source rewrite.
6. **Housekeeping (cheap)**: fix the "22 targets" figure at issues.md line 55 — the real count is 21
    entries.

**Guards for the Unicode work specifically** (it touches hot, fully-covered functions — the two
gates most likely to bite):

- **`.iai-baseline.json` Ir gate fails above 10%.** A per-character range-table lookup added
    *before* normalization in `text_clean`/`text_collapse` will show up in the meta/text benches.
    Expect to justify and refresh the affected baseline entries **in the same step**, and prefer a
    binary-search over sorted ranges (or a fast ASCII/BMP short-circuit) over a linear scan.
- **The CRAP `--fail-regression` gate is CI-only, not in `mise run check`.** Adding branches/loops
    to the covered `utils.rs` functions will fail CI despite a green local check — refresh
    `.crap-baseline.json` in the same step.

Other standing guards: do NOT cut v1.0.0, flip `Semver` to enforcing, or raise the MSRV to 1.86 —
all three are human policy calls tied to the v1.0.0 cut. Watch for the enforcing
`Audit (cargo-deny)` gate turning red on a fresh live advisory; any `rb_sys` move must update
`Gemfile`, `Gemfile.lock` **and** `tag:` in `release.yml` together; and run `./gradlew clean` before
believing a Kotlin build failure (this bind mount flakes on incremental state).
