---
name: deps-refresh
description: Dependency-refresh issue — held-back Rust majors with reasons, ruff hold-back, slice history
metadata:
  type: project
---

# Dependency refresh (issue: "Dependency review and refresh across the project")

Slices done: 1 `Cargo.lock` (iter 124), 2 `uv.lock` (iter 125), 3 direct Rust pins (iter 126), 4 GHA
actions in ci.yml + docs.yml (iter 127: checkout v7, setup-python v7, setup-uv v9, setup-node v7,
setup-java v5, setup-go v7, setup-dotnet v6, upload-artifact v7, upload-sarif v4,
upload-pages-artifact v5 + deploy-pages v5 — paired). GOTCHA: `astral-sh/setup-uv` has NO floating
major tag past v7 — v8.x/v9.0.0 are exact release tags only, so write `@v9.0.0` not `@v9` (iter-127
CI failure: "Unable to resolve action"). `releases/latest` proves a release exists, NOT that a
floating `@vN` tag exists — confirm via `gh api repos/<o>/<r>/git/matching-refs/tags/v<N>`.
Remaining: `release.yml` actions (97 `uses:` refs; upload/download-artifact@v4 must move together;
setup-uv there needs `@v9.0.0` too; only truly validated by a release run), per-binding manifests
(napi package.json, rb Gemfile/gemspec, jni pom.xml, kotlin build.gradle.kts, dotnet .csproj, go
go.mod), ruff 0.16 adoption.

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

## Python hold-back

- `ruff<0.16` in root `pyproject.toml` (iter 125): ruff 0.16 expands default lint rules → 104 errors
    (72 in `_lowlevel.pyi`: PIE790/PYI048; also RUF100/I001/RUF059); `--fix` clears 60. Adoption is
    a dedicated step, then drop the pin.
- zensical ≥0.0.51 warns (non-fatal) on broken anchors; the `docs/howto/c-cpp.md` anchor was fixed
    during iter-125 review.
