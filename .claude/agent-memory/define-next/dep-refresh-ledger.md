---
name: dep-refresh-ledger
description: Per-slice ledger and hard-won gotchas for the v0.6.0 "Dependency review and refresh" issue — what each slice covered, what is held back and why, and what remains
metadata:
  type: project
---

# Dependency-refresh slice ledger (v0.6.0)

Fact: the `normal` `[human]` issue "Dependency review and refresh across the project" is worked
**one ecosystem per CID iteration** (~12 manifests). It cites no `[audit]` tag → no 8-file escape
valve; lockfiles are generated and cost 0 source-file budget.

**Why:** manifests drift between releases because there is no Dependabot/Renovate (deliberate). A
whole-repo refresh in one step would blow the 3-file cap and make a conformance regression
un-bisectable.

**How to apply:** pick the next unstarted slice below; hold a dep back with an inline documented
comment (`# held:` / `// held:` / XML `held:`) beside the pin, never by disabling a rule or gate.

## Golden rules learned the hard way

- **Never move a consumer floor inside a refresh slice.** MSRV, the `go` directive,
    `required_ruby_version`, `java-version`, `net8.0`, and a *published* binding's compiler version
    are support-policy decisions reserved for Titusz. Iter 128 broke this accidentally: KGP
    2.1.10→2.4.10 stamped `mv=[2,4,0]` into the published Kotlin jar and raised the consumer floor
    to Kotlin 2.3 (open `[review]` issue, HUMAN REVIEW REQUESTED). Cheap detectors:
    `javap -v -p <class> | grep mv=` on a built jar, plus a throwaway consumer project.
- **A deprecation IS a hard error in any dep bump** — `mise run lint` is
    `clippy --all-targets -D warnings` (iter 126, `criterion::black_box`).
- **A floating `@vN` GitHub Action tag is a publisher convention, NOT a guarantee.** Confirm with
    `gh api repos/<o>/<r>/git/matching-refs/tags/v<N>`; `releases/latest` is not proof. `setup-uv`
    publishes no floating major past v7 → exact tag `@v9.0.0`. Getting this wrong reddened CI
    mid-127.
- Every Rust slice is verified with the 4-gate set: `test` / `lint` / `audit` / `bench:iai:check`.

## Slices done

1. **124 — Rust `Cargo.lock`** (`cargo update`, ~100 transitive crates, all pins held).
2. **125 — Python `uv.lock`** (`uv lock --upgrade`, 40 pkgs incl. iscc-core 1.3.0, ty 0.0.63).
    Hold-back: `ruff<0.16` in `pyproject.toml`.
3. **126 — Rust direct pins.** criterion 0.5→0.7 (bench import moved to `std::hint::black_box`). The
    4 surviving `# held:` comments in root `Cargo.toml` are the authoritative record: criterion 0.8
    (needs rustc 1.86 > declared 1.85 — never raise MSRV for a dev-dep), magnus 0.8 (`old-api` off
    by default → `exception::runtime_error()` deprecated at 5 sites in
    `crates/iscc-rb/src/ lib.rs`), jni 0.22 (wholesale `JNIEnv`→`Env`/`EnvUnowned` rework per
    upstream `docs/0.22-MIGRATION.md`), uniffi 0.32 (needs Swift+Kotlin regen; no Swift toolchain
    locally). pyo3 0.29 is already latest.
4. **127 — GitHub Actions in `ci.yml` + `docs.yml`.** Residue: `.pre-commit-config.yaml` needs NO
    bump (both pinned repos already latest → the feared mdformat reformat wave is moot).
5. **128 — JVM manifests** (`crates/iscc-jni/java/pom.xml`, `packages/kotlin/build.gradle.kts`).
    Devcontainer HAS JDK 17 + Maven 3.8.7 (no `gradle` binary, but `./gradlew` works, `~/.gradle`
    warm ~516 MB; `~/.m2` empty → first `mvn` run downloads). Landed junit-jupiter 5.14.4, gson
    2.14.0, compiler 3.15.0, surefire 3.5.6, source 3.4.0, javadoc 3.12.0, gpg 3.2.8, KGP 2.4.10,
    JNA 5.19.1. HELD: `central-publishing-maven-plugin` 0.7.0 (its `deploy` goal runs only in a
    real Central publish — unverifiable locally and in CI). JUnit 6.1.2 deferred (platform
    artifacts renumbered 1.x→6.x). **JNA version is duplicated in 3 doc files** (`README.md`,
    `packages/kotlin/README.md`, `docs/howto/kotlin.md`) + junit/gson in
    `crates/iscc-jni/CLAUDE.md` → sync in the same step; `.claude/context/specs/kotlin-bindings.md`
    is human-owned → leave.
6. **129 — Go module + `version_sync.py` TARGETS fix.** Proven output-neutral across all 1.1M code
    points; surfaced the pre-existing Rust-core Unicode divergence (`[review]`, human-gated).
7. **130 — Ruby manifests** (`crates/iscc-rb/Gemfile` + `Gemfile.lock`; `rb_sys` pinned exactly).

## Slice 6 facts (Go)

- **Go 1.26.1 IS in the devcontainer.** Run from repo root with `-C` — never `cd`:
    `CGO_ENABLED=0 go test -C packages/go -count=1 ./...`, `go vet -C packages/go ./...`,
    `go mod tidy -C packages/go -diff` (exits 0/1 — a clean tidiness criterion).
- **`go list -m -u all` OVER-reports**: it lists test-only deps of deps (x/tools, x/mod,
    zeebo/assert) that never enter the build list, so "no updates available" is NOT a reachable
    verification criterion — grep explicit versions instead.
- Actual gaps at scoping: `golang.org/x/text` 0.34.0→0.40.0 (its own `go 1.25.0` < our 1.26.1, so no
    `toolchain` line should appear), `klauspost/cpuid/v2` 2.0.12→2.4.0 (indirect).
    `zeebo/blake3 v0.2.4` is already latest.
- **Conformance risk worth naming in next.md**: x/text ships Unicode tables and
    `packages/go/utils.go` uses `unicode/norm` NFKC inside `TextClean`, which feeds every
    `Gen*CodeV0`. The vendored `packages/go/testdata/data.json` vectors are the guard.

## Slice 7 facts (Ruby)

- **The devcontainer runs the Ruby gates end to end** (verified iter 130): ruby 3.1.2 (= CI's
    `ruby-version: '3.1'`), bundler 2.6.9, `libclang-14`, warm `crates/iscc-rb/vendor/bundle` (51
    MB, gitignored via `BUNDLE_PATH` in `crates/iscc-rb/.bundle/config`), prebuilt `iscc_rb.so`.
    `standardrb` exits 0 and `rake test` gives 111 runs / 299 assertions in \<1 s. No `-C` flag
    exists → use a subshell `(cd crates/iscc-rb && bundle …)`.
- The gemspec is **`iscc-lib.gemspec`** (hyphen), not `iscc_lib.gemspec` as older notes said.
- `bundle outdated --strict` is the good boolean freshness criterion (only versions the Gemfile
    constraints actually allow); plain `bundle outdated` over-reports held majors.
- **`rb_sys` must be pinned EXACTLY, not `~> 0.9`**: 0.9.123 bundles `rake-compiler-dock = 1.10.0`
    whose `cross_rubies` map matches the `oxidize-rb/actions/cross-gem` `tag: 0.9.123` in
    `release.yml`; 0.9.124+ ships a Ruby 4.0 build tool that breaks `RbSys::ExtensionTask` (see
    commit `1e4a30e`). Change the Gemfile pin and the workflow `tag:` together or not at all.
- **`minitest` 6.x requires Ruby >= 3.2** — blocked by the gem's `required_ruby_version >= 3.1.0` (a
    consumer floor, human-only). Document as held; never "solve" it by raising the floor.

## Empty / near-empty slices (verified iter 129 — do not churn)

- `crates/iscc-napi/package.json`: `@napi-rs/cli: ^3` already covers latest 3.7.4.
- `packages/dotnet/*/*.csproj`: test refs float on `17.*` / `2.*` wildcards. Only majors remain
    (xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x).

## Slice 8 — ruff 0.16 adoption, split into 3 sub-slices (scoped iter 131)

`uvx ruff@0.16.0 check .` = **104 errors over 5 non-test files** (0.16 widened the *default* rule
set — the repo has no `lint.select`, so PIE/PYI/RUF/I/EXE/PLW arrived for free). Over the 3-file
budget → sliced by *decision content*, not by file count:

- **A (iter 131) — config-free, mechanical.** `_lowlevel.pyi` 72 (PIE790+PYI048: every stub body is
    docstring + a lone `...`; delete the 36 `...` lines → both rules clear, `ruff format` stays a
    no-op, `ty` passes — prototyped) + `tests/test_new_symbols.py` 6 RUF059 (`_`-prefix 4 unpacked
    tuples; fix is "unsafe" only because it renames).
- **B (scoped iter 134) — the `# noqa` / security-gate cluster.** Resolution measured with
    `--config` overrides: `[tool.ruff.lint] extend-select = ["S", "C901"]` (**`extend-select`, not
    `select` — `select` replaces the E4/E7/E9/F default**) + delete exactly two dead directives
    (`tools/metrics.py` `# noqa: S603` on the static-list `git rev-parse` call — 0.16 refined S603
    to skip literal argv, verified unused at 0.15.22 *and* 0.16.0; `tools/cid.py`
    `# noqa: PLC0415`). 27 → 12 findings, 0 RUF100, 0.15.22 stays "All checks passed". **Real value:
    `S`/`C901` were pre-push-hook-only — CI's `uv run ruff check` never ran them.** `PLC0415` must
    NOT be selected (adds 5 `tests/` findings). **TRAP: a blanket `ruff@0.16 check --fix .` deletes
    the load-bearing `# noqa: S603/S607`** and reddens the pre-push `--select S` gate. Never plain
    `--fix`.
- **C (scoped iter 135) — isort config only.** Re-probed live: the two fidelity settings hold
    (`[tool.ruff] src = [".", "crates/iscc-py/python"]` else `iscc_lib` is third-party;
    `lint.isort.combine-as-imports = true` else the `__init__.py` fix explodes the single
    `from iscc_lib._lowlevel import (X as X, …)` re-export block into ~60 one-member imports — a
    110-line diff instead of one blank line). **Setting `src` moves the finding set, it does not
    only shrink it**: 3 test files go clean and `benchmarks/python/bench_iscc_lib.py` becomes dirty
    → 7 fixes over 6 files (3 non-test = exactly at budget), so the three one-liners could NOT ride
    along. Pinned 0.15.22 + `--extend-select I,RUF022,RUF100` reproduces the identical 7 findings,
    so the selection is enforceable before the pin drops, and `ruff format --check` stays clean
    after the fix (probed on a scratch copy).
- **D (scoped iter 136) — the three one-liners** = exactly 3 non-test files, so the `# held:`
    comment in `pyproject.toml` (a 4th file) could NOT ride along and stays stale for one more
    iteration. Probed live: `zip(ranges, ranges[1:])` → `pairwise` is exactly equivalent and the
    generator re-runs in \<1 s producing a byte-identical `unicode16.rs`; `run()` in
    `test_install.py` has 20 callers testing `.returncode`, so `check=False` (not `True`) is the
    behaviour-preserving fix; `tools/cid.py` is invoked only as `uv run tools/cid.py …` from 10
    `mise.toml` tasks, so the exec bit is purely additive and no hook
    (`check-executables-have-shebangs` is absent) reacts.
- **E (scoped iter 137) — drop `ruff<0.16` + `uv lock --upgrade-package ruff`** + retire the
    `# held:` comment. Probed live: linter side is a pure no-op (0.16 `check .`, hook-mode per-file,
    `--select S`, `--select C901` all exit 0; 13 `# noqa: S60x` intact). **The only real change is
    the formatter's file discovery: ruff 0.16 formats Python code blocks inside `.md`**, so the bare
    `uv run ruff format --check` used by `mise run lint` + `ci.yml` widens 25 → ~153 files. All 129
    tracked `.md` are already 0.16-clean → free coverage, no exclude config needed. `ruff check`
    does NOT lint Markdown. Gitignored `reference/` (32 dirty) and `cauldron/` (1 dirty) stay
    excluded from a bare `.` run — never pass them explicitly.

Reproduce baselines with `uvx ruff@0.16.0 …` — it needs no lockfile change and the cache is warm.

## Remaining after slice 8

`release.yml` GHA refs (97 `uses:`; `upload-artifact@v4` ↔ `download-artifact@v4` move as a pair;
nothing in it is exercised by a CID push → human-timed), the magnus 0.8 / jni 0.22 / uniffi 0.32
migrations (each its own step with a source rewrite), and the Gradle wrapper 8.12.1 + JUnit 6.x
majors.

## Handy version-lookup commands

- crates.io: `cargo search <crate> --limit 1`; changelog via
    `curl -sL https://static.crates.io/crates/<c>/<c>-<ver>.crate | tar xz`
- Maven **stable**: `repo1.maven.org/maven2/<path>/maven-metadata.xml` filtered by
    `^[0-9]+(\.[0-9]+)*$` (the `<latest>` field includes betas/milestones)
- npm: `curl -s https://registry.npmjs.org/<pkg>` → `.dist-tags.latest`
- Go: `curl -s https://proxy.golang.org/<module>/@v/list | sort -V | tail`, and
    `https://proxy.golang.org/<module>/@v/<ver>.mod` for its `go` directive
- GitHub Actions: `gh api repos/<o>/<r>/git/matching-refs/tags/v<N>`

Network access works from the devcontainer.
