---
name: dep-refresh-survey
description: Per-ecosystem dependency-pin inventory (GHA refs, tooling pins, per-binding manifests) backing the sliced "Dependency review and refresh" issue
metadata:
  type: project
---

# Dependency-refresh survey (verified iteration 129)

Backing detail for the sliced `normal` `[human]` issue "Dependency review and refresh across the
project" (spec: `.claude/context/specs/ci-cd.md` → Dependency Freshness). MEMORY.md keeps only a
pointer here.

**Why:** the project has no Dependabot/Renovate, so manifests drift between releases and the refresh
is a manual per-release pass, sliced one ecosystem at a time to keep each step reviewable.

**How to apply:** when assessing the CI/CD section, re-verify the lines relevant to the slice that
just landed instead of re-surveying everything; refresh this file when a slice lands.

## GitHub Actions

- **ci.yml + docs.yml — CURRENT since slice 4 (iter 127)**: `actions/checkout@v7`,
    `setup-python@v7`, `setup-node@v7`, `setup-java@v5`, `setup-go@v7`, `setup-dotnet@v6`,
    `upload-artifact@v7`, `github/codeql-action/upload-sarif@v4`, `upload-pages-artifact@v5` +
    `deploy-pages@v5` (paired), `astral-sh/setup-uv@v9.0.0`.
- `astral-sh/setup-uv` uses an **exact tag** — upstream publishes no floating major past v7, so
    `@v9` does not resolve. Confirm floating tags with
    `gh api repos/<owner>/<repo>/git/matching-refs/tags/v<N>`; `releases/latest` is not proof.
- Deliberately untouched and already current: `dtolnay/rust-toolchain@stable`,
    `Swatinem/rust-cache@v2`, `taiki-e/install-action@v2`, `ruby/setup-ruby@v1`,
    `obi1kenobi/cargo-semver-checks-action@v2`.
- **release.yml still lags** (deferred, human-timed — nothing in it is exercised by a CID push):
    `checkout@v4` (23), `download-artifact@v4` (20) ↔ `upload-artifact@v4` (11) must move as a pair,
    `setup-java@v4` (6), `setup-node@v5` (5), `setup-dotnet@v4` (3), `setup-python@v5` (2),
    `cache@v4` (1). **It contains NO `astral-sh/setup-uv` step** — issues.md and the iteration-127
    handoff claim otherwise; that claim is wrong.
- `docs.yml` only triggers on push to `main`, so its bumps are statically verified until the next
    develop→main merge.

## Tooling pins

- `.pre-commit-config.yaml`: exactly 2 pinned repos — `pre-commit/pre-commit-hooks` `rev: v6.0.0`
    and `executablebooks/mdformat` `rev: 1.0.0`, both current. Everything else is `repo: local`.
    Risk if a future mdformat rev lands: it can reformat every Markdown file in the repo.
- `mise.toml`: **no `[tools]` section** — out of scope for the refresh.
- Only 2 `package.json` files: `crates/iscc-napi/` (hand-written, single `@napi-rs/cli: ^3` dep) and
    `crates/iscc-wasm/pkg/` (generated).

## Slice history

1. Rust `Cargo.lock` via `cargo update` (iter 124, ~100 crates).
2. Python `uv.lock` via `uv lock --upgrade` (iter 125; iscc-core 1.3.0 = zero vector drift).
3. Rust direct pins (iter 126; criterion 0.5→0.7 + `std::hint::black_box`; 4 `# held:` comments).
4. GHA refs in ci.yml + docs.yml (iter 127; 9 refs, setup-uv exact tag).
5. JVM manifests (iter 128; 11 pins — see below; raised the Kotlin consumer floor → `[review]`
    issue).

Remaining: napi `package.json` + dotnet `.csproj` (near-empty confirmation), rb manifests, `go.mod`,
ruff 0.16, magnus 0.8, jni 0.22 — each its own step. Deferred/human-timed: release.yml GHA refs,
Gradle wrapper 8.12.1 major, JUnit 6.x.

## Per-binding manifests

- `crates/iscc-jni/java/pom.xml` — **refreshed iter 128**: junit-jupiter 5.14.4, gson 2.14.0,
    maven-compiler-plugin 3.15.0, maven-surefire-plugin 3.5.6; release-only profile maven-source
    3.4.0, maven-javadoc 3.12.0, maven-gpg 3.2.8. `central-publishing-maven-plugin` **held at
    0.7.0** (inline `held:` XML comment) — its `deploy` goal only runs in a real Central publish, so
    nothing local or in CI can validate a bump. `mvn -Prelease package` exercises source+javadoc but
    **not** gpg or central-publishing.
- `packages/kotlin/build.gradle.kts` — **refreshed iter 128**: `kotlin("jvm") 2.4.10`, JNA 5.19.1,
    junit-jupiter 5.14.4 + required `testRuntimeOnly junit-platform-launcher:1.14.4`, gson 2.14.0;
    JUnit 6.x `held:`. **Consumer-floor trap**: the KGP bump stamps jar metadata `mv=[2,4,0]` and
    publishes `kotlin-stdlib:2.4.10` → Kotlin < 2.3 consumers fail to compile (2.1.10 and 2.2.21
    fail, 2.3.21 passes). `java-version: '17'`, the jvmToolchain, `required_ruby_version`, the `go`
    directive and `net8.0` are all support-policy decisions, not dependency pins — never move them
    in a refresh step. Detect this class with `javap -v -p <class> | grep mv=` plus a throwaway
    consumer project.
- `crates/iscc-rb/Gemfile` — pessimistic constraints only (minitest `~> 5.0`, rake `~> 13.0`,
    rake-compiler `~> 1.2`, rb_sys `~> 0.9`, standard `~> 1.0`, rubocop-minitest `~> 0.36`); gemspec
    declares `required_ruby_version >= 3.1.0`. rb_sys must match the `oxidize-rb/actions/cross-gem`
    Docker image tag.
- `packages/go/go.mod` — `go 1.26.1`, `github.com/zeebo/blake3 v0.2.4`, `golang.org/x/text v0.34.0`,
    indirect `github.com/klauspost/cpuid/v2 v2.0.12`.
- `packages/dotnet/*/*.csproj` — `net8.0`; test project refs already float
    (`Microsoft.NET.Test.Sdk 17.*`, `xunit 2.*`, `xunit.runner.visualstudio 2.*`) → near-empty
    slice.

## Documented hold-backs (do not "fix" these)

- Root `Cargo.toml`, 4 `# held:` comments: criterion 0.8 (needs rustc 1.86 vs `rust-version 1.85`),
    jni 0.22 (wholesale iscc-jni API rework), magnus 0.8 (drops `old-api`, deprecates
    `exception::runtime_error()` — 5 call sites), uniffi 0.32 (needs Swift/Kotlin regen). Plus a
    `# note:` on pyo3 tying bumps to issue #41.
- `pyproject.toml`: `ruff<0.16` (0.16 expands default lint rules → 104 new errors, 72 in
    `_lowlevel.pyi`) — deferred adoption, tracked as its own step.
