---
name: dep-refresh-survey
description: Per-ecosystem dependency-pin inventory (GHA refs, tooling pins, per-binding manifests) backing the sliced "Dependency review and refresh" issue
metadata:
  type: project
---

# Dependency-refresh survey (verified iteration 128)

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

## Per-binding manifests (remaining slices)

- `crates/iscc-jni/java/pom.xml` — junit-jupiter 5.11.4, gson 2.11.0, maven-compiler-plugin 3.13.0,
    maven-surefire-plugin 3.5.2. Release-only profile plugins (maven-source 3.3.1, maven-javadoc
    3.11.2, maven-gpg 3.2.7, central-publishing 0.7.0) are **not** CI-exercised.
- `packages/kotlin/build.gradle.kts` — Kotlin JVM plugin 2.1.10, JNA 5.16.0. `java-version: '17'`
    and the jvmToolchain are support-policy decisions, not dependency pins.
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
