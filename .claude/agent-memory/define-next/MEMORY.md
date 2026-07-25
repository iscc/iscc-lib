# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

**Size budget:** Keep under 140 lines. Archive stale/completed-phase detail to `MEMORY-archive.md`.

## Scope Calibration Principles

- **CI red always first** — green CI is a prerequisite for all other work; formatting/lint/advisory
    fixes preempt the handoff "Next".
- Critical issues always take priority regardless of feature trajectory.
- **Verify claimed gaps by reading the actual files** — state.md and handoff "IDLE" both go stale;
    always read issues.md directly (review agent can miscount).
- **Generated/tool-output files (Cargo.lock, bindings) don't count toward the 3-file limit**; doc
    files are also excluded — can batch all howto guides in one step.
- Batch related small changes (version sync + docs; several fixes in the same crate/2 files).
- **IDLE is valid** when all target sections met and only `low` issues remain — don't invent work.
    But an already-specced, locally-verifiable target gap is NOT idle work.
- **Prefer boolean-verifiable prerequisites over high-impact-but-risky infra fixes** when both are
    available and no feature is in-progress. But infra/release fixes CAN be locally verifiable (napi
    bundled loader #38, cargo-deny reads Cargo.lock not artifacts) — don't default to "release-only
    → too risky".
- **HUMAN REVIEW override on overwhelming evidence is for BUG fixes, not NEW policy gates** — a gate
    that amends the spec/notes needs human sign-off first (iter 106/112).
- **next.md is a sensitive file** — if the Write tool is blocked, use `cat > file << 'EOF'` via
    Bash.
- Run `mise run format` before committing next.md + memory (pre-push mdformat rejects the batch).

## Architecture & Conformance Facts

- Hub-and-spoke: pure-Rust `iscc-lib` core → binding crates (py, napi, wasm, ffi, jni, rb) + go/
    dotnet/cpp/swift/kotlin packages. Tier 1 = **32** crate-root re-exports, bound in all languages.
- Go bindings are pure Go (no CGO/WASM/binaries). `gen_iscc_code_v0` vectors have no `wide` — pass
    `false`. `"stream:<hex>"` prefix = hex-encoded byte data.
- **data.json copies** (all identical, update together): `crates/iscc-lib/tests/`,
    `packages/go/   testdata/`, `packages/dotnet/Iscc.Lib.Tests/testdata/`,
    `packages/swift/Tests/IsccLibTests/`, `packages/kotlin/src/test/resources/`. Binding crates with
    hardcoded vector-count asserts (Rust core + WASM) must be bumped when vendoring new vectors.
- **SumHasher** lives at `iscc_lib::streaming::SumHasher` (NOT Tier 1; count stays 32).

## Dev Environment Constraints

- **No Swift toolchain / no shellcheck** in the Linux devcontainer — `swift test` + shell lint are
    CI/macOS only.
- `uniffi-bindgen`: `cargo run -p iscc-uniffi --features bindgen --bin uniffi-bindgen`.
- UniFFI 0.31.0; SPM module name MUST be `iscc_uniffiFFI`; UniFFI can't export `const` (getters) or
    `usize`/borrowed/generic exports.
- cargo-crap 0.2.2, cargo-llvm-cov 0.8.7, valgrind 3.19 + iai-callgrind-runner ARE installed.
    cargo-deny is NOT preinstalled but installs via `cargo binstall cargo-deny@0.19.9 --force`.

## CI/Release, Docs, Gotchas

- Release: `workflow_dispatch` with 9 per-registry checkboxes; version_sync.py manages 16 targets
    (`--check` exits 1 on mismatch). `iscc-rb` needs `libclang-dev` (keep `--exclude iscc-rb` in
    Rust CI). XCFramework cache key must hash all build inputs.
- Docs: `zensical.toml nav` + `scripts/gen_llms_full.py ORDERED_PAGES` need an entry per new howto
    guide (template `docs/howto/dotnet.md`; collapsible `??? tip "Build from source"`).
- Gotchas: JNI names encode `_` as `_1`; WASM pkg `@iscc/wasm`, npm lib `@iscc/lib`; Windows GHA →
    `pwsh`, add `shell: bash`; Gson Maven groupId `com.google.code.gson`.
- "10 gen functions" vs "9 conformance functions" — no blanket 9→10 find/replace (corrupts
    conformance-scoped files). See learnings.md.
- Language API name styles (doc examples): Ruby/Rust `snake_case`; C#/Go/Java `PascalCase`; JS/
    Swift/Kotlin `camelCase`; Swift/Kotlin take named `bits`/`u` params.

## v1.0.0 Hardening Phase — COMPLETE (iters 86–114); detail in MEMORY-archive.md + learnings.md

All autonomous v1.0.0-hardening gates landed and are enforcing/green (module visibility, SumHasher
#37, PyO3 0.29 #1, npm bundled loader #38, CRAP `--fail-above`, iai-callgrind #3, cargo-deny #114);
`cargo-semver-checks` stays informational until the human-gated v1.0.0 cut. Residual facts:

- **Semver + Coverage/CRAP + Perf + Audit are the 4 quality gates.** CRAP + Perf + Audit enforcing;
    Semver `continue-on-error: true` until v1.0.0 cut.
- **Baselines (`.crap-baseline.json`, `.iai-baseline.json`) are committed and refreshed only by
    deliberate reviewed `mise run` commits** — never auto-committed from CI; don't widen
    `--epsilon`.
- **cargo-deny** reads Cargo.lock + metadata (NOT compiled artifacts) → a green local
    `cargo deny check` is authoritative. `deny.toml` schema/license-graph detail in learnings.md.

## v0.6.0 Phase (post-v0.5.0, started ~iter 115)

- v0.5.0 released to all registries; all 12 bindings meet core criteria. Human raised the target.md
    bar with 5 spec'd `normal` `[human]` v0.6.0 issues (#41 GIL, #42 WASM SIMD, #43 Go ISCC-IDv1,
    #49 aarch64 wheels — all DONE — plus dependency refresh) + 2 `normal` release-workflow issues
    (npm OIDC, single-registry re-trigger). v1.0.0 cut + Semver-enforcing HELD by Titusz (`low`).
- **iters 115–125 DONE (detail in MEMORY-archive.md + learnings.md)**: 115 cargo-deny advisory bump;
    116 #41 Python GIL detach; 117→118 #42 WASM SIMD (reframe: needs the `blake3/wasm32_simd` Cargo
    feature, not just RUSTFLAGS); 119 #43 Go ISCC-IDv1; 120/121 trailing-byte "too long" guards (Go,
    then Rust core); 122 CI-RED-FIRST `.crap-baseline.json` refresh; 123 #49 aarch64 wheels
    (release-only infra → STATIC verification: pyyaml `safe_load` + grep); 124/125 dep-refresh
    slices 1–2. **Root lesson: the CRAP regression gate is CI-ONLY** (not in
    `mise run check`/pre-commit) — any step adding a branch/loop to a covered fn MUST refresh the
    baseline in the SAME step.
- **Recurring**: the enforcing cargo-deny gate WILL periodically go red on fresh RustSec advisories
    vs dev/bench deps — CI-red-first; prefer `cargo update -p <crate>` over a `deny.toml` ignore
    when a patched release exists.
- **Dep refresh is sliced per-ecosystem** (~12 manifests, cites no `[audit]` → no 8-file valve):
    Rust lock → Rust pins → `uv.lock` → each binding-manifest group → tooling pins. Lockfiles are
    generated → 0 source files; hold a dep back with an inline documented comment, never by
    disabling a rule/gate.
- **iter 126: dep-refresh slice 3 = Rust direct pins — DONE** (criterion 0.5→0.7; bench import moved
    to `std::hint::black_box` because `criterion::black_box` is `#[deprecated]` and `mise run lint`
    = `clippy --all-targets -D warnings` → **a deprecation IS a hard error in any dep bump**). The 4
    surviving `# held:` comments in `Cargo.toml` are the authoritative record: criterion 0.8 (MSRV
    1.86 > declared 1.85 — never raise MSRV for a dev-dep; human policy call at the v1.0.0 cut),
    magnus 0.8 (`old-api` off by default → `exception::runtime_error()` deprecated at 5 sites in
    `crates/iscc-rb/src/lib.rs`; rest of 0.8 fits), jni 0.22 (wholesale `JNIEnv`→`Env`/`EnvUnowned`
    rework per upstream `docs/0.22-MIGRATION.md`, rewrites `crates/iscc-jni/src/lib.rs`), uniffi
    0.32 (needs Swift+Kotlin regen; no Swift toolchain locally). pyo3 0.29 is already latest.
- **ruff 0.16 adoption is OVER the 3-file budget** (iter 126, `uvx ruff@0.16.0 check .`): 104 errors
    over 5 non-test files (`_lowlevel.pyi` 72, `tools/cid.py` 12, `tools/metrics.py`,
    `scripts/test_install.py`, `iscc_lib/__init__.py`) + `pyproject.toml` → slice it.
- **iter 127: dep-refresh slice 4 = GitHub Actions in `ci.yml` + `docs.yml` — DONE.** Residue:
    `.pre-commit-config.yaml` needs NO bump (both pinned repos already latest → the feared mdformat
    reformat wave is moot); `release.yml` (97 `uses:`) is its own human-timed slice
    (`upload-artifact@v4` ↔ `download-artifact@v4` must move as a pair, nothing in it is exercised
    by a CID push). **A floating `@vN` action tag is a convention, NOT a guarantee** — confirm with
    `gh api repos/<o>/<r>/git/matching-refs/tags/v<N>`; `releases/latest` is not proof (setup-uv
    publishes no floating major past v7 → exact tag `@v9.0.0`). That mistake reddened CI mid-127.
- **iter 128: dep-refresh slice 5 = JVM manifests** (`crates/iscc-jni/java/pom.xml` +
    `packages/kotlin/build.gradle.kts`). CI-exercised on every develop push AND locally verifiable —
    the devcontainer HAS **JDK 17 + Maven 3.8.7** (no `gradle` binary, but `./gradlew` works and
    `~/.gradle` is warm at ~516 MB; `~/.m2` is empty → first `mvn` run downloads). Survey (repo1
    `maven-metadata.xml`, 2026-07-25): junit-jupiter 5.11.4→5.14.4, gson 2.14.0, maven-compiler
    3.15.0, surefire 3.5.6, source 3.4.0, javadoc 3.12.0, gpg 3.2.8 (all `prerequisites` maven 3.6.3
    → local 3.8.7 fine), KGP 2.4.10 (supports Gradle 7.6.3–9.5.0, so the 8.12.1 wrapper stays), JNA
    5.19.1. **JUnit 6.1.2 deferred** (major: platform artifacts renumbered 1.x→6.x, needs Kotlin
    ≥2.2 and likely an explicit `testRuntimeOnly junit-platform-launcher` under Gradle 8.12.1).
    **HELD: `central-publishing-maven-plugin` 0.7.0** — its `deploy` goal runs only in a real Maven
    Central publish, so nothing local or in CI can verify a bump.
    - **JNA version is duplicated in 3 doc files** (`README.md`, `packages/kotlin/README.md`,
        `docs/howto/kotlin.md`) + junit/gson in `crates/iscc-jni/CLAUDE.md` → sync in the same step;
        `.claude/context/specs/kotlin-bindings.md` names `jna:5.16.0@aar` but is human-owned → leave.
- **v0.6.0 remaining after slice 5**: `release.yml` GHA bump (human-timed), ruff 0.16 (sliced),
    magnus 0.8 / jni 0.22 / uniffi 0.32 migrations, remaining manifests (rb `Gemfile`+gemspec, go
    `go.mod`, napi `package.json` one-liner, dotnet `.csproj` already wildcard-floating) + 2
    human-gated release-workflow fixes (npm OIDC, single-registry re-trigger). One slice/iteration.
- **Handy**: crates.io latest via `cargo search <crate> --limit 1`; Maven latest **stable** via
    `repo1.maven.org/maven2/<path>/maven-metadata.xml` filtered by `^[0-9]+(\.[0-9]+)*$` (its
    `<latest>` field includes betas/milestones); crate changelogs via
    `curl -sL https://static.crates.io/crates/<c>/<c>-<ver>.crate | tar xz`. Network works.
