---
name: deps-refresh
description: Dependency-refresh issue — held-back Rust majors with reasons, ruff 0.16 adoption history, slice history
metadata:
  type: project
---

# Dependency refresh (issue: "Dependency review and refresh across the project")

Slices done: 1 `Cargo.lock` (iter 124), 2 `uv.lock` (iter 125), 3 direct Rust pins (iter 126), 4 GHA
actions in ci.yml + docs.yml (iter 127: checkout v7, setup-python v7, setup-uv v9, setup-node v7,
setup-java v5, setup-go v7, setup-dotnet v6, upload-artifact v7, upload-sarif v4,
upload-pages-artifact v5 + deploy-pages v5 — paired), 5 JVM manifests (iter 128: pom.xml junit
5.14.4 + gson 2.14.0 + 5 maven plugins; build.gradle.kts kotlin 2.4.10, jna 5.19.1, junit/gson
lockstep), 6 Go module (iter 129: x/text 0.40.0, cpuid/v2 2.4.0 + new indirect x/sys 0.47.0;
conformance stayed green despite x/text Unicode-table risk; kotlin README added to version_sync
TARGETS; napi `^3` + dotnet `17.*`/`2.*` wildcards verified current, no edit), 7 Ruby Gemfile (iter
130: rb_sys pinned exactly `0.9.123` with `# held:` in Gemfile — must move together with cross-gem
`tag:` in release.yml; minitest `~> 5.0` held (6.x needs Ruby >= 3.2); bundle update moved rake
13.4.2, standard 1.56.0, rubocop 1.88.2, rubocop-minitest 0.40.0 + transitives; zero standardrb
fallout, 111 tests green). GOTCHA: `astral-sh/setup-uv` has NO floating major tag past v7 —
v8.x/v9.0.0 are exact release tags only, so write `@v9.0.0` not `@v9` (iter-127 CI failure: "Unable
to resolve action"). `releases/latest` proves a release exists, NOT that a floating `@vN` tag exists
— confirm via `gh api repos/<o>/<r>/git/matching-refs/tags/v<N>`. Slice 8 (ruff 0.16, sub-slices
A–E) CLOSED iter 137. xunit.v3 + Test.Sdk 18.x DONE iter 164 (see below). Gradle wrapper 8.12.1 →
9.6.1 DONE iter 165 (see below). JUnit 6.1.2 DONE iter 166 (see below). magnus 0.8 DONE iter 167
(see below). jni 0.22 DONE iter 168 (see below). .NET test-dep pin + lockfile DONE iter 170 (see
below). Action-freshness pass probed NO-OP 2026-07-28: all 25 distinct `uses:` refs across the three
workflow files are on their publisher's latest major (`matching-refs/tags/v<N+1>` empty for all 23
versioned refs; both exact pins == `releases/latest`). Remaining (all human/major-gated): criterion
0.8, uniffi 0.32 (see Held-back section).

## .NET test-dep pin + NuGet lockfile (iter 170 — DONE)

- `Iscc.Lib.Tests.csproj`: `<RestorePackagesWithLockFile>true</...>` + exact pins (Test.Sdk 18.8.1,
    xunit.v3 3.2.2, runner.visualstudio 3.1.5); `packages.lock.json` COMMITTED; CI `dotnet` job runs
    `dotnet restore … --locked-mode` then build/test with `--no-restore`. Any version bump must
    regen the lock in the same commit: `dotnet restore <csproj> --force-evaluate`.
- `--locked-mode` ACCEPTS a referenced project with zero `PackageReference`s and no lock file
    (`Iscc.Lib` needed nothing). Drift → `NU1004`, exit 1 — but `| tail` masks the exit code,
    redirect to a file.
- NuGet emits the lock file WITHOUT a trailing newline; prek `end-of-file-fixer` adds one and NuGet
    never rewrites it back (restore skips writing when the graph is unchanged) — no tug-of-war.

## jni 0.22 (iter 168 — DONE)

- jni 0.22.4 (0.22.0/0.22.1 yanked), `rust-version = 1.85` == workspace MSRV — no floor move.
    jni-sys 0.4 makes `jboolean = bool` (drop all `!= 0` conversions). Lock delta:
    +jni-macros/simd_cesu8/simdutf8, −cesu8/thiserror 1.x/windows-sys 0.45 stack, all jni-only.
- `EnvOutcome::resolve::<P>()` requires `T: Default` — raw `jstring`/`jobject` aliases implement
    nothing, so extern fns must RETURN the `#[repr(transparent)]` wrappers (`JString<'local>`,
    `JObject`, `JByteArray`, `JObjectArray<_, E>`); ABI-identical, Java-invisible.
- 0.22 removed `From<JObject> for JString` — take element-typed `JObjectArray<'local, JString>`
    (`<JIntArray>`, `<JByteArray>`) params so `get_element` returns the right type; kills every
    `unsafe from_raw` in the extractors.
- `find_class`/`throw_new` take `AsRef<JNIStr>`, `new_object` takes `AsRef<MethodSignature>` —
    `&str` literals must become `jni_str!`/`jni_sig!`; dynamic msgs `JNIString::from(msg)`.
    `jni::objects::JValue` deprecated → `jni::JValue`.
    `Env::byte_array_from_slice(&mut self, &[u8])` SURVIVES un-deprecated (0.22.4 env.rs:3349,
    internally `JByteArray::new` + transmuted `set_region` — no extra alloc) and is THE `byte[]`
    return path; iter 168 wrongly believed it removed and hand-rolled a `Vec<i8>`-copying helper,
    reverted iter 169 (the deprecation note read belonged to the neighbouring
    `set_object_array_element`). `push/pop_local_frame` → `env.with_local_frame(16, |env| ...)` per
    loop iteration (closure `E: From<Error>`, so helpers return `jni::errors::Result`, not
    `Result<_, String>`).
- Exception contract preserved WITHOUT the policy: throw helpers keep `env.throw_new` +
    `Ok(T::default())` — pending exception survives `resolve` (`ThrowRuntimeExAndDefault` checks
    `exception_check` first, fires only on panic/unhandled Err). 82/82 Maven tests unchanged.

## JUnit 6.1.2 (iter 166 — DONE)

- JUnit 6 unifies Platform/Jupiter/Vintage under one version number — the Gradle
    `junit-platform-launcher` pin is now identical to junit-jupiter (6.1.2), no more 1.x lockstep.
- Zero test-source changes needed (suites use only `@Test`/`@TestFactory`/`DynamicTest`/
    `@BeforeAll`/Assertions); Maven totals 82 = 82 pre/post (69 IsccLibTest + 13 Boundary), Kotlin 9
    \+ 13. Surefire 3.5.6 auto-resolves the aligned launcher — no explicit pom launcher dep needed.
- No consumer floor moves: both artifacts test-scoped; JUnit 6 baselines (Java 17, Kotlin 2.2) met.
- GOTCHA: the Gradle wrapper script lives at `packages/kotlin/gradlew` — from repo root run
    `packages/kotlin/gradlew -p packages/kotlin …`, a bare `./gradlew` is not found.

## Gradle wrapper 9.6.1 (iter 165 — DONE)

- Two-pass `gradlew -p packages/kotlin wrapper --gradle-version 9.6.1` so jar/scripts are
    regenerated BY 9.6.1. Gradle 9 wrapper adds `retries=0`/`retryBackOffMs=500` to the properties
    and emits `gradlew.bat` with CRLF — `mise run format` (mixed-line-ending) must fix it to LF
    before staging. Gradle 9 DELETES the distribution zip after extraction (only `.ok` marker in
    `~/.gradle/wrapper/dists`), so a cached-zip sha256 provenance check is impossible post-install.
- KGP 2.4.10 `GradleCompatibilityCheck` has NO upper bound (min 7.6.3, nextMin 8.14.4) — 9.6.1
    accepted, deprecation warning retired. Gradle 9 no longer auto-injects junit-platform-launcher;
    the explicit `testRuntimeOnly` launcher in build.gradle.kts is now REQUIRED, not a workaround
    (comment updated accordingly). Kotlin tests: 9 conformance + 13 boundary, unchanged.

## xunit v3 migration (iter 164 — DONE)

- `packages/dotnet/Iscc.Lib.Tests`: `xunit` 2.\* → `xunit.v3` 3.\* (resolved 3.2.2),
    `xunit.runner.visualstudio` 3.\* (3.1.5), `Microsoft.NET.Test.Sdk` 18.\* (18.8.1). Only csproj
    edit needed: swap package names + add `<OutputType>Exe</OutputType>` (v3 test projects are
    stand-alone executables). ZERO test-source changes — `using Xunit;` and every `Assert` member
    survived, and `[MemberData]` yielding non-serializable `JsonElement` in `object[]` still
    enumerates row-level at execution under VSTest (104 individual results, not collapsed).
- `dotnet test <dir> -e LD_LIBRARY_PATH=…` still reaches the v3 out-of-process test host — the CI
    invocation stayed byte-identical. dotnet SDK 8.0.423 builds it fine; no consumer-floor move.

## JVM manifests (iter 128)

- GOTCHA: junit-jupiter ≥ 5.12 under Gradle 8.12.1 needs an explicit
    `testRuntimeOnly("org.junit.platform:junit-platform-launcher:1.x.y")` (1.x.y lockstep with
    5.x.y) — Gradle injects its own bundled launcher which predates platform 1.12, failing test
    discovery with "OutputDirectoryCreator not available … unaligned platform jars". Maven is
    unaffected (surefire resolves the aligned launcher itself).
- Held: `central-publishing-maven-plugin` 0.7.0 (0.11.0 exists; deploy goal only runs in a real
    Maven Central publish — bump during a human-supervised release). JUnit 6 migration landed iter
    166 (see above).
- Kotlin 2.4.10 compiles the UniFFI-generated bindings unchanged; KGP 2.4.x supports Gradle
    7.6.3–9.5.0 so the 8.12.1 wrapper needed no change.
- Kotlin compiler bumps move the CONSUMER floor (metadata forward-compat ≈ one minor): 2.4.10 ⇒
    floor "Kotlin 2.3 or newer" (verbatim, grep-checkable), documented iter 132 in root README
    Kotlin install section, `packages/kotlin/README.md` `## Requirements`, `docs/howto/kotlin.md`
    admonition. Spec rule (kotlin-bindings.md "Supported consumer Kotlin version"): any bump that
    moves the floor must update all three + the spec in the same step. Holding an older floor via
    `languageVersion` alone does NOT work — transitive `kotlin-stdlib` in the POM raises the same
    error.
- Verify pins exist before editing:
    `curl -s -o /dev/null -w "%{http_code}"   https://repo1.maven.org/maven2/<g/a/v/a-v.pom>` → 200.

Verified current on 2026-07-24 (no bump needed): `.pre-commit-config.yaml` (pre-commit-hooks v6.0.0,
mdformat 1.0.0), `dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2`,
`taiki-e/install-action@v2`, `ruby/setup-ruby@v1`, `obi1kenobi/cargo-semver-checks-action@v2`.
`mise.toml` has no `[tools]` section — nothing to pin there.

## Held-back workspace majors (`# held:` comments in root Cargo.toml, iter 126)

- **criterion 0.7** pinned; 0.8 needs rustc 1.86 > workspace MSRV 1.85 (MSRV raise = human policy
    decision at the v1.0.0 cut). 0.6+ deprecated `criterion::black_box` — benches use
    `std::hint::black_box` instead (deprecation is a clippy `-D warnings` hard error).
- **jni 0.22** DONE iter 168 (see section above); no `GlobalRef`/`AutoLocal`/`Executor` usage
    existed in the crate, so only the `EnvUnowned`/`Env` + typed-array parts of the migration guide
    applied.
- **magnus 0.8** DONE iter 167 (0.7 → 0.8.2): mechanical — `magnus::exception::runtime_error()` →
    `ruby.exception_runtime_error()` (5 sites) and `RString::from_slice(&b)` →
    `ruby.str_from_slice(&b)` (5 sites), handle via `Ruby::get().expect("called from Ruby")`.
    `RString::as_slice` NOT deprecated. Lock delta was magnus + magnus-macros + **rb-sys-env
    0.1.2→0.2.3** (build-dep, one more than predicted). No consumer floor moved (Ruby 3.0-3.4, MSRV
    1.65). rb_sys gem pin `0.9.123` untouched (moves only with cross-gem Docker tag).
- **uniffi 0.31**; 0.32 requires regenerating + re-verifying Swift/Kotlin bindings — unverifiable
    locally (no Swift toolchain in the Linux devcontainer).
- **pyo3 0.29** is current; a `# note:` reminds that bumps must re-verify `gil_used = true` and the
    12 `py.detach` sites (issue #41).
- taplo preserves the `# held:` comments. Pre-existing `proc-macro-error2 v2.0.1` future-incompat
    warning comes from `iai-callgrind-macros` → `iai-callgrind` (dev-dep of iscc-lib), NOT
    magnus/rb_sys (iter-126 review corrected this via `cargo tree -i`). No fixed release exists
    (iai-callgrind 0.16.1 is latest) — stays a warning, do not chase it in the Ruby slice.

## ruff 0.16 adoption (slice 8, sub-slices A–E — DONE iter 137)

- Hold-back was `ruff<0.16` in root `pyproject.toml` (iter 125): ruff 0.16 expands default lint
    rules. Baseline was 104 errors; slice A (iter 131) cleared 78 config-free ones: deleted the 36
    lone `...` stub bodies in `_lowlevel.pyi` (PIE790+PYI048 double-report the same line —
    docstring-only body is valid) and `_`-prefixed 6 RUF059 unused unpackings in
    `tests/test_new_symbols.py`.
- Slice B (iter 134): `[tool.ruff.lint] extend-select = ["S", "C901"]` in pyproject.toml (NEVER
    `select` — it replaces the pyflakes defaults). All 15 `RUF100` cleared: 14 `# noqa: S603/S607`
    became recognised, one genuinely-unused `S603` deleted in `tools/metrics.py` `git_sha()` (review
    correction: the directive was unused under BOTH versions — what changed in 0.16 is that `RUF100`
    joined the default select; dynamic-argv directives MUST stay), plus a stale `PLC0415` in
    `tools/cid.py:980`. Pre-push `--select S` / `--select C901` hooks kept intentionally (redundant
    but name the failing gate; recorded in decisions.md 2026-07-25).
- Slice C (iter 135): isort cluster cleared. `[tool.ruff] src = [".", "crates/iscc-py/python"]`
    (makes `iscc_lib` first-party) + `[tool.ruff.lint.isort] combine-as-imports = true` (keeps the
    ~35-member `_lowlevel` re-export block as ONE statement — without it isort shatters it into a
    110-line diff). `extend-select` now `["S", "C901", "I", "RUF022", "RUF100"]`. Fixes applied with
    pinned ruff, scoped: `uv run ruff check --fix --select I,RUF022 .` (7 fixed). NEVER blanket
    `--fix` under unpinned/0.16 ruff — it deletes load-bearing `# noqa: S603/S607`.
- Sub-slice D (iter 136): last 3 findings cleared — `RUF007` (`itertools.pairwise` in
    `gen_unicode16_unassigned.py`; generator re-run, `unicode16.rs` byte-identical), `PLW1510`
    (`check=False` in `test_install.py` — callers inspect `returncode` at 20 sites), `EXE001`
    (`tools/cid.py` exec bit: `chmod +x` + `git update-index --chmod=+x` because
    `core.fileMode=false` on the 9p bind mount). `uvx ruff@0.16.0 check .` now exits 0.
- Sub-slice E (iter 137): pin + `# held:` comment dropped, `uv lock --upgrade-package ruff` →
    0.16.0. Zero findings, zero reformats at the flip. Known behaviour change: `ruff format` now
    also checks Python code blocks in Markdown — bare `ruff format --check` reports 153 files (was
    25); mdformat prek hook still owns Markdown auto-fixing. NEVER blanket `--fix` — it deletes the
    13 load-bearing `# noqa: S603/S607` directives.
- `uvx ruff@X.Y.Z check .` runs any ruff version without touching the lock (useful for previewing
    future majors).
- zensical ≥0.0.51 warns (non-fatal) on broken anchors; the `docs/howto/c-cpp.md` anchor was fixed
    during iter-125 review.

## ruff hook gate parity (iters 138–139)

- `.pre-commit-config.yaml`: `ruff-format` uses `types_or: [python, pyi, markdown]` and `ruff-check`
    uses `types_or: [python, pyi]` so the local prek surface matches CI's bare `uv run ruff check` /
    `ruff format --check`. prek classifies `.pyi` as the `pyi` type, NOT `python` — a bare
    `types: [python]` silently skips `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` (closed iter
    139). Markdown stays format-only — ruff 0.16 formats Markdown fences but does not *lint* them.
- mdformat hook is declared before `ruff-format`, so the project-pinned ruff has the last word; the
    two formatters converge on this tree (no ping-pong — `mise run check` twice stays clean).
- With `pyi` added, local `--all-files` is a strict superset of CI's recursive discovery (extra: the
    tracked-but-gitignored `.claude/plans/*.md`). Bare `ruff format --check` file count grows
    whenever tracked `.md`/`.py`/`.pyi` files are added (155 as of iter 139 — earlier-phase CID
    memory files count too), so never pin an exact count in a verification. Do NOT add
    `--force-exclude`.
- mdformat's `mdformat-ruff` (via `mdformat-mkdocs[recommended]`) only formats fences tagged
    `python`; ruff 0.16 additionally reaches `py`/`python3`/`pycon` — that residual gap is what the
    widened hook closes.
- GOTCHA: prek reports "files were modified by this hook" only for *tracked* files — an untracked
    probe file gets fixed but the hook reports Passed. `git add` probe files before
    `uv run prek run <hook> --files <probe>` when a verification expects Failed.
