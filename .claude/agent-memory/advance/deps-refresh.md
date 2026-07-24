---
name: deps-refresh
description: Dependency-refresh issue — held-back Rust majors with reasons, ruff hold-back, slice history
metadata:
  type: project
---

# Dependency refresh (issue: "Dependency review and refresh across the project")

Slices done: 1 `Cargo.lock` (iter 124), 2 `uv.lock` (iter 125), 3 direct Rust pins (iter 126).
Remaining: per-binding manifests (napi package.json, rb Gemfile/gemspec, jni pom.xml, kotlin
build.gradle.kts, dotnet .csproj, go go.mod), tooling pins (mise.toml, .pre-commit-config.yaml, GHA
actions), ruff 0.16 adoption.

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
    warning comes from the magnus/rb_sys subtree — candidate cleanup in the Ruby slice.

## Python hold-back

- `ruff<0.16` in root `pyproject.toml` (iter 125): ruff 0.16 expands default lint rules → 104 errors
    (72 in `_lowlevel.pyi`: PIE790/PYI048; also RUF100/I001/RUF059); `--fix` clears 60. Adoption is
    a dedicated step, then drop the pin.
- zensical ≥0.0.51 warns (non-fatal) on broken anchors; the `docs/howto/c-cpp.md` anchor was fixed
    during iter-125 review.
