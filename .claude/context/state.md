<!-- assessed-at: f5f821c05d28cc21d369704772e297286c95bb81 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — CI GREEN; dependency refresh underway (Rust lock, Python lock, Rust pins done)

v0.5.0 is released across all registries and all 12 language bindings meet their core criteria. This
iteration (126) landed **slice 3 of the dependency refresh**: `criterion` 0.5 → 0.7 in
`[workspace.dependencies]` (with the deprecated `criterion::black_box` import migrated to
`std::hint::black_box`) plus inline `# held:` comments documenting the four deliberately held
majors. **CI is fully green on the develop tip (30 check-runs, 0 non-success).** Remaining
CID-doable v0.6.0 work is the rest of the dependency refresh: tooling pins, per-binding manifests,
and three separate migration steps (ruff 0.16, jni 0.22, magnus 0.8).

## Rust Core Crate

**Status**: partially met — CI GREEN; only the held semver-enforcing / v1.0.0 criterion is unmet

- Core API met: all 10 `gen_*_v0` functions (incl. `gen_sum_code_v0`), 32 Tier 1 symbols,
    conformance vs `iscc-core/data.json` passing — `Rust (fmt, clippy, test)` job GREEN. Workspace
    version `0.5.0` (unchanged). **No library source touched this iteration** — only
    `Cargo.toml`/`Cargo.lock` and the bench harness.
- **Rust direct-pin evaluation done (iter 126, dependency-refresh slice 3)**: verified in
    `Cargo.toml` — `criterion = "0.7"` (`Cargo.lock` confirms exactly one entry at `0.7.0`, no
    0.5.x), and `grep -c '# held' Cargo.toml` → **4** hold-back comments (criterion 0.8 needs rustc
    1.86 vs declared `rust-version = "1.85"`; jni 0.22 is a wholesale API rework; magnus 0.8 drops
    `old-api` and deprecates `exception::runtime_error()`; uniffi 0.32 needs Swift+Kotlin
    regeneration with no local Swift toolchain) plus a `# note:` on `pyo3` tying bumps to issue #41
    (`gil_used` / `py.detach`). These comments are now the authoritative record of why each pin sits
    below latest.
- **Cargo.lock refreshed (iter 124, slice 1)** — ~100 transitive crates at latest semver-compatible
    versions; still standing, further adjusted by the criterion 0.7 subtree (clap, criterion-plot
    0.6, itertools 0.13) which cleared cargo-deny.
- **Trailing-byte hardening intact (iter 121)**: `iscc_decode` (`crates/iscc-lib/src/lib.rs`) has
    both "too short" (~line 235) and "too long" (~line 241) rejection branches. ISCC-IDv1 is
    rejected at the header level in the Rust core (`codec::Version` is V0-only) — IDv1 support
    remains Go-only.
- **CRAP regression gate green** (iter 122 baseline refresh holds): max CRAP well below the 30.0
    `--fail-above` cap. NOTE: the `--fail-regression` gate is CI-only (not in `mise run check`/
    pre-commit) — any source change adding a branch/loop to a covered function must refresh
    `.crap-baseline.json` in the same step.
- **Perf gate — COMPLETE, ENFORCING**: `Perf (iai-callgrind)` GREEN against the **unmodified**
    baseline after the criterion bump (max drift +1.96%).
- **Semver gate present (informational)**: `Semver (cargo-semver-checks)` is
    `continue-on-error:   true`, reports `success`; the enforcing-at-v1.0.0 criterion
    (`rust-core.md` semver box `[ ]`) stays unmet — deliberately held by Titusz until the v1.0.0
    cut.
- **Known constraint (not a regression)**: the `proc-macro-error2 v2.0.1` future-incompat warning on
    `cargo test`/`cargo bench` comes from `iai-callgrind-macros` → `iai-callgrind` (dev-only), not
    from magnus/rb-sys. No fixed upstream release exists; re-check when bumping `iai-callgrind`
    (which must stay in lockstep with the CI-installed `iai-callgrind-runner`).

## Python Bindings

**Status**: met

- Core met: all 32 symbols exported, `Python 3.10` and `Python 3.14` CI jobs GREEN (plus the
    aggregator `Python (ruff, pytest)` gate job), ruff clean, streaming `SumHasher` wrapper present.
    PyO3 pinned `0.29` (`abi3-py310`), single lockfile resolution. Untouched this iteration.
- **GIL release COMPLETE (issue #41 RESOLVED)**: `grep -c '\.detach('` in
    `crates/iscc-py/src/lib.rs` = **12** (re-verified) — data/instance/image/sum/text/video plus 3
    streaming `update()` paths. Video detach opens strictly after frame-signature extraction;
    meta/audio/mixed stay attached by design.
- **`uv.lock` refreshed (iter 125, slice 2)**: 40 packages bumped incl. iscc-core `1.3.0` (matches
    the vendored `data.json` vectors, zero output drift), ty 0.0.63, maturin 1.14.1, prek 0.4.11,
    zensical 0.0.51.
- **Documented hold-back**: `pyproject.toml:27` pins `ruff<0.16` with an inline `# held:` comment
    (0.16 expands default lint rules → 104 new errors, mostly `_lowlevel.pyi`). Deferred adoption,
    not gate weakening — the locked ruff 0.15.22 enforces the same rule set as before.
- **aarch64 wheels wired (issue #49 DONE, iter 123)**: `release.yml` `build-wheels` has the
    `ubuntu-24.04-arm`/`aarch64` entry; first real aarch64 wheel ships at v0.6.0.

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols with TypeScript declarations; `Node.js (napi build, test)` job GREEN.
    Bundled model in `package.json`. Note for the manifest slice: `crates/iscc-napi/package.json` is
    the **only** hand-maintained JS manifest in the repo (verified — no `packages/*/package.json`),
    and its single `@napi-rs/cli: ^3` devDependency is already caret-covered.

## WASM Bindings

**Status**: met — issue #42 (SIMD) resolved iteration 118, CI-verified

- All 32 Tier 1 symbols via `#[wasm_bindgen]`, streaming `SumHasher` class, `WASM (wasm-pack test)`
    job GREEN.
- **SIMD backend active**: `crates/iscc-wasm/Cargo.toml` carries a direct
    `blake3 = { features = ["wasm32_simd"] }` dep that exists solely for feature unification (no
    `use blake3` in source — must not be pruned as "unused"). `simd128` RUSTFLAGS (ci.yml +
    release.yml) and the `--enable-simd` wasm-opt flag remain in place.

## C FFI

**Status**: met

- cbindgen header committed + freshness check, C test passes, csbindgen generates
    `NativeMethods.g.cs`; `C FFI (cbindgen, gcc, test)` job GREEN.

## Java Bindings

**Status**: met

- 32 Tier 1 symbols via JNI, native libs bundled in JAR; `Java (JNI build, mvn test)` job GREEN.
    `jni` stays pinned at 0.21 with a documented hold-back (0.22 rewrites
    `crates/iscc-jni/src/lib.rs`).

## Go Bindings

**Status**: met — ISCC-IDv1 (#43) landed iter 119, trailing-byte hardening landed iter 120

- Pure Go (no CGO), all 32 Tier 1 symbols, `Go (go test, go vet)` job GREEN, `CGO_ENABLED=0` holds.
- `packages/go/iscc_id.go` provides `EncodeIsccID` / `DecodeIsccID` + `IsccIDv1Result`; `codec.go`
    adds `VSV1` and a MainType-ID-only Version=1 relaxation in `decodeHeader`.
- `IsccDecode` (`codec.go`) has both "too short" (~line 594) and "too long" (~line 597) rejection
    branches; `DecodeIsccID` inherits via delegation; `IsccDecompose` untouched.

## Ruby Bindings

**Status**: met

- 32 Tier 1 symbols via Magnus; `Ruby (magnus build, test)` job GREEN; version synced. `magnus`
    stays pinned at 0.7 with a documented hold-back (0.8 needs an `exception::runtime_error()` →
    `Ruby::exception_runtime_error()` call-site refactor across 5 sites).

## C# / .NET Bindings

**Status**: met

- 32 public symbols via P/Invoke over the C FFI; `C# / .NET (dotnet build, test)` job GREEN.

## C++ Bindings

**Status**: met

- C++17 header-only wrapper, all 32 Tier 1 symbols, ASAN clean, vcpkg + Conan;
    `C++ (cmake, ASAN, test)` job GREEN.

## UniFFI Scaffolding Crate

**Status**: complete (internal, not published)

- 32 `#[uniffi::export]` annotations; shared by Swift + Kotlin. Pinned at 0.31 with a documented
    hold-back (0.32 requires Swift/Kotlin regeneration, unverifiable in the Linux devcontainer).

## Swift Bindings

**Status**: met

- SPM package with UniFFI bindings, all 32 Tier 1 symbols, XCFramework build;
    `Swift (swift build, swift test)` job GREEN. XCFramework checksum current for v0.5.0.

## Kotlin Bindings

**Status**: met

- `packages/kotlin/` with JNA-loaded UniFFI bindings, 9 desktop + Android targets;
    `Kotlin (gradle build, test)` job GREEN.

## README

**Status**: met

- Polyglot README with CI + registry badges, per-language install/quick start for all 12 languages,
    architecture + MainTypes.

## Per-Crate READMEs

**Status**: met

- READMEs present for all 12 crates/packages; registry metadata references them.

## Documentation

**Status**: met (one low-priority cosmetic gap)

- Docs site, **11** `docs/howto/*.md` language guides, tabbed examples, llms-full.txt (22 pages),
    benchmarks page with speedup factors all present. Untouched this iteration.
- The `docs/howto/c-cpp.md:9` anchor fix (iter 125) holds; `zensical build` reports no issues.
- **Gap (low, CID skips)**: language logos in `docs/index.md` / howto headers — cosmetic only.

## Benchmarks

**Status**: met

- Criterion benches for all 10 `gen_*_v0` (+2) — **12** `bench_*` functions in `benchmarks.rs`;
    `Bench (compile check)` job GREEN on criterion 0.7. 18 pytest-benchmark functions, speedup
    factors published (1.3x-158x) in `docs/benchmarks.md`.
- **Bench harness modernized (iter 126)**: `black_box` now imported from `std::hint` instead of the
    deprecated `criterion::black_box` re-export; all 30 call sites byte-identical, clippy
    `-D warnings` clean.
- Second harness `iai_benches.rs` (iai-callgrind 0.16, **11** `bench_*` → 16 cases) enforcing the
    > 10% Ir regression gate — GREEN against the unmodified baseline.

## CI/CD and Publishing

**Status**: partially met — **CI GREEN**; one CID-doable gap remains (dependency freshness, in
progress)

- **LATEST CI RUN — SUCCESS.** origin/develop tip `4bf6ff3` (HEAD `f5f821c` is a +1 log-only commit
    touching only `iterations.jsonl`, unpushed). **30 check-runs, 0 non-success, 0 still running** —
    verified via `gh api repos/iscc/iscc-lib/commits/4bf6ff3/check-runs`. The criterion 0.7 bump
    regressed no gate: `Coverage + CRAP`, `Perf (iai-callgrind)`, `Audit (cargo-deny)`,
    `Bench (compile check)` and `Rust (fmt, clippy, test)` all GREEN.
- ci.yml has 19 job entries → 20 jobs (`python-test` is a 3.10/3.14 matrix, `python` is an
    `if: always()` aggregator gate). No workflow file changed this iteration.
- **Dependency refresh progress**: ✅ slice 1 Rust `Cargo.lock` (iter 124), ✅ slice 2 Python
    `uv.lock` (iter 125), ✅ slice 3 Rust direct pins (iter 126). Remaining under the `normal`
    `[human]` "Dependency review and refresh" issue: **tooling pins** (`.pre-commit-config.yaml` has
    exactly 2 pinned repos — `pre-commit/pre-commit-hooks` rev `v6.0.0` and
    `executablebooks/mdformat` rev `1.0.0`; everything else is `repo: local`; GHA action versions —
    `astral-sh/setup-uv@v4` and `actions/checkout@v4` look behind, 25 distinct `uses:` refs total;
    `mise.toml` has **no `[tools]` section**, so it is out of scope), **per-binding manifests** (rb
    `Gemfile`/gemspec, jni `pom.xml`, kotlin `build.gradle.kts`, dotnet `.csproj`, go `go.mod`; napi
    `package.json` is trivial), plus three separate migration steps: ruff 0.16 adoption, `magnus`
    0.8, `jni` 0.22.
- **cargo-deny gate enforcing** (unchanged): root `deny.toml`, `Audit (cargo-deny)` job GREEN. A
    fresh live advisory can flip this red on any push with no code change (prefer
    `cargo update -p <crate>` over a `deny.toml` ignore).
- **Gap (v0.6.0)**: no automated dependency updates — `.github/dependabot.yml` and `renovate.json`
    both absent (deliberate; refresh is a manual per-release pass).
- v0.5.0 published; release workflow with 8 per-registry toggles + version sync (16 targets) in
    place. Two release-workflow reliability issues remain open (npm OIDC migration, single-registry
    re-trigger bug) — both human-gated `normal`.

## Open Issues (issues.md lists 5 — 0 critical, 3 normal, 2 low; all `[human]`)

CI is green — no open issue is CI-blocking. There is currently **no CID-actionable `[review]` or
`[audit]` issue**. Any open issue keeps the project IN_PROGRESS. Issue headers are unchanged from
iteration 125; only the dependency issue's progress body grew (slice 3 + the `proc-macro-error2`
constraint note).

CID-doable now (`normal`, `[human]`, spec'd):

- Dependency review and refresh across the project (spec: `.claude/context/specs/ci-cd.md` →
    "Dependency Freshness"). Slices 1-3 DONE; remaining slices listed above.

Release-workflow reliability (`normal`, `[human]`, human-gated):

- Migrate npm publishing to OIDC Trusted Publishing (needs npm-side trusted-publisher config first)
- Fix broken single-registry re-trigger in `release.yml`

Low (human-directed, CID skips):

- Release core as v1.0.0 — **held** by Titusz (stay on 0.5.x; flip Semver to enforcing at the cut).
- Add programming language logos to docs site — cosmetic.

## Next Milestone

**CI is green — no CI fix needed.** Continue the **project-wide dependency refresh** where slice 3
(Rust direct pins) left off, one small per-ecosystem step at a time (this issue cites no `[audit]`
issue, so no 8-file escape valve applies). Recommended order:

1. **Tooling pins (slice 4)** — `.pre-commit-config.yaml` (2 pinned repos) + GitHub Actions versions
    (`astral-sh/setup-uv@v4` → v5, `actions/checkout@v4` → v5; confirm the rest individually).
    **Risk to scope explicitly:** an `executablebooks/mdformat` rev bump can reformat every
    Markdown file in the repo and a `pre-commit-hooks` bump can add new default checks — either
    raise the file budget for the mechanical churn or hold the formatter revs back with a
    documented `# held:` comment. Do not let a reformat wave hide a substantive change. `mise.toml`
    is out of scope (no `[tools]` section).
2. **Per-binding manifests** — rb `Gemfile`/gemspec (rb_sys must match the
    `oxidize-rb/actions/cross-gem` Docker tag), jni `pom.xml`, kotlin `build.gradle.kts`, dotnet
    `.csproj`, go `go.mod` — one small step each. napi `package.json` is a one-line check.
3. **ruff 0.16 adoption** — `ruff check --fix` (≈60 auto-fixable), hand-fix the rest (mostly
    `_lowlevel.pyi` stub-style PIE790/PYI048/RUF022), then drop the `ruff<0.16` pin.
4. **`magnus` 0.8 and `jni` 0.22 migrations** — each its own dedicated step with a source rewrite
    (`crates/iscc-rb/src/lib.rs`, `crates/iscc-jni/src/lib.rs`); update the corresponding `# held:`
    comment in `Cargo.toml` when a hold-back is lifted.

Do NOT cut v1.0.0, flip the `Semver` gate to enforcing, or raise the workspace MSRV to 1.86 (which
would unlock criterion 0.8) — all three are human policy calls tied to the v1.0.0 cut. The npm OIDC
migration and single-registry re-trigger fixes stay human-gated. Guards: any source change that adds
a branch/loop to a covered function must refresh `.crap-baseline.json` in the same step (the CRAP
regression gate is CI-only, not in `mise run check`); and watch for the enforcing
`Audit (cargo-deny)` gate turning red on a fresh live advisory.
