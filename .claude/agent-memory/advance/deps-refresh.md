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
A–E) CLOSED iter 137. Remaining (all human/major-gated): `release.yml` actions (97 `uses:` refs;
upload/download-artifact@v4 must move together; setup-uv there needs `@v9.0.0` too; only truly
validated by a release run), Gradle wrapper major, JUnit 6.x migration, xunit 3.x + Test.Sdk 18.x
majors, jni 0.22, magnus 0.8.

## JVM manifests (iter 128)

- GOTCHA: junit-jupiter ≥ 5.12 under Gradle 8.12.1 needs an explicit
    `testRuntimeOnly("org.junit.platform:junit-platform-launcher:1.x.y")` (1.x.y lockstep with
    5.x.y) — Gradle injects its own bundled launcher which predates platform 1.12, failing test
    discovery with "OutputDirectoryCreator not available … unaligned platform jars". Maven is
    unaffected (surefire resolves the aligned launcher itself).
- Held: `central-publishing-maven-plugin` 0.7.0 (0.11.0 exists; deploy goal only runs in a real
    Maven Central publish — bump during a human-supervised release). JUnit 6.x deferred (renumbered
    platform artifacts, removed Platform APIs; its Kotlin ≥ 2.2 prereq is now met by 2.4.10).
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
- **jni 0.21**; 0.22 is a wholesale API rework (JNIEnv → EnvUnowned/Env, GlobalRef → Global,
    AutoLocal → Auto, closure-based thread attachment, mandatory ErrorPolicy) that rewrites
    `crates/iscc-jni/src/lib.rs` per upstream `docs/0.22-MIGRATION.md`. Dedicated step.
- **magnus 0.7**; 0.8 drops the default `old-api` feature → `magnus::exception::runtime_error()`
    (used in `crates/iscc-rb/src/lib.rs`) becomes deprecated = clippy failure; refactor to
    `Ruby::exception_runtime_error()`. Do in the Ruby slice (rb_sys gem must keep matching the
    `oxidize-rb/actions/cross-gem` Docker tag).
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
