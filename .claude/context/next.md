# Next Work Package

## Step: Rust direct-pin evaluation — bump criterion to 0.7, document held-back majors

## Goal

Complete slice 3 of the tracked `normal` `[human]` issue **"Dependency review and refresh across the
project"**: evaluate every `workspace.dependencies` pin in the root `Cargo.toml`, bump the one major
that is safe to take (`criterion` 0.5 → 0.7, a dev-only bench dependency), and record an inline
documented reason next to each pin that is deliberately held back — exactly what
`.claude/context/specs/ci-cd.md` → "Dependency Freshness" requires.

## Scope

- **Modify**: `Cargo.toml` (root — `[workspace.dependencies]`: bump `criterion`, add `# held:`
    comments), `crates/iscc-lib/benches/benchmarks.rs` (swap the deprecated `criterion::black_box`
    import for `std::hint::black_box`)
- **Generated (does not count toward the file budget)**: `Cargo.lock`
- **Reference**: `.claude/context/specs/ci-cd.md` → "Dependency Freshness" (§423),
    `.claude/context/issues.md` → "Dependency review and refresh across the project", `mise.toml`
    (`lint`, `test`, `audit`, `bench:iai:check` tasks), `.github/workflows/ci.yml` (the `bench` job
    runs `cargo bench --no-run`)

## Not In Scope

- **Do NOT bump `magnus`, `jni`, or `uniffi`.** Each needs its own step with a source migration (see
    Implementation Notes for the exact blockers). Documenting the hold-back is this step's
    deliverable; performing the migration is not.
- **Do NOT raise the workspace `rust-version = "1.85"`** in order to take criterion 0.8. Raising the
    declared MSRV of the published crate is a human policy decision (it belongs with the v1.0.0
    cut), not a dependency-refresh side effect.
- Do NOT touch `pyproject.toml` or adopt ruff 0.16 — that is its own dedicated step.
- Do NOT touch the per-binding manifests (`crates/iscc-napi/package.json`, `crates/iscc-rb/Gemfile`
    \+ gemspec, `crates/iscc-jni/java/pom.xml`, `packages/kotlin/build.gradle.kts`,
    `packages/dotnet/*/*.csproj`, `packages/go/go.mod`) or the tooling pins (`mise.toml`,
    `.pre-commit-config.yaml`, GHA action versions) — later slices of the same issue.
- Do NOT add, remove, rename or restructure benchmarks. The only edit to `benchmarks.rs` is the
    import line; all 30 `black_box(...)` call sites stay byte-identical.
- Do NOT modify any file under `crates/*/src/` — this slice changes no library behaviour.
- Do NOT refresh `.crap-baseline.json` or `.iai-baseline.json`. No library source changes, so
    neither can legitimately drift; if `bench:iai:check` fails, investigate and report rather than
    rebaseline.
- Do NOT edit `issues.md` (the review agent records slice progress after verification).

## Implementation Notes

**Pin survey (already done — crates.io latest as of 2026-07-24; no need to redo):**

| Pin             | Current  | Latest | Action                                   |
| --------------- | -------- | ------ | ---------------------------------------- |
| `criterion`     | `0.5`    | 0.8.2  | **bump to `0.7`** (0.8 held — see below) |
| `magnus`        | `0.7`    | 0.8.2  | hold + document                          |
| `jni`           | `0.21`   | 0.22.4 | hold + document                          |
| `uniffi`        | `0.31`   | 0.32.0 | hold + document                          |
| `pyo3`          | `0.29`   | 0.29.0 | already current                          |
| `iai-callgrind` | `0.16`   | 0.16.1 | caret already covers it                  |
| `napi` family   | `3`, `2` | 3.11.0 | caret already covers it                  |

Every other `workspace.dependencies` entry (`blake3`, `data-encoding`, `hex`, `serde`, `serde_json`,
`serde_json_canonicalizer`, `thiserror`, `unicode-normalization`, `unicode-general-category`,
`xxhash-rust`, `wasm-bindgen`, `wasm-bindgen-test`, `serde-wasm-bindgen`, `tempfile`) is already at
the newest release its caret range admits — the iteration-124 `Cargo.lock` refresh covered them, so
no pin text changes.

**1. criterion 0.5 → 0.7.** Set `criterion = { version = "0.7", features = ["html_reports"] }`. The
`html_reports` feature still exists in 0.7. Breaking changes between 0.5 and 0.7 (per the upstream
CHANGELOG) that touch us: exactly one — in 0.6 `criterion::black_box` became
`#[deprecated(note = "use std::hint::black_box() instead")]` and the `real_blackbox` feature became
a no-op. Because `mise run lint` runs `cargo clippy --workspace --all-targets -- -D warnings`, that
deprecation is a hard error. Fix by editing only the import at
`crates/iscc-lib/benches/benchmarks.rs:6`:

```rust
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;
```

Keep the rest of the imports rustfmt-stable (`cargo fmt`, or let `mise run format` do it).
`BenchmarkId`, `Throughput`, `criterion_group!`/`criterion_main!` and the
`[[bench]] harness = false` setup are unchanged in 0.6/0.7. `crates/iscc-lib/benches/iai_benches.rs`
does not use criterion and must not be touched.

**2. Hold-back comments.** Add a short inline `# held:` comment adjacent to each held pin in
`[workspace.dependencies]`. Use these verified reasons (1–3 comment lines each, taplo-safe):

- `criterion` → 0.8 requires rustc **1.86**; the workspace declares `rust-version = "1.85"`. Revisit
    when the MSRV is raised.
- `uniffi` → 0.32 requires regenerating **and re-verifying** the Swift + Kotlin bindings; there is
    no Swift toolchain in the Linux devcontainer, so it cannot be validated locally.
- `magnus` → in 0.8 the `old-api` feature is no longer default, which makes
    `magnus::exception::runtime_error()` (used in `crates/iscc-rb/src/lib.rs`) `#[deprecated]` and
    therefore a `clippy -D warnings` failure; it needs a call-site refactor to
    `Ruby::exception_runtime_error()`. Deferred to the Ruby slice (which also refreshes
    `Gemfile`/gemspec, where the `rb_sys` gem must keep matching the `oxidize-rb/actions/cross-gem`
    Docker tag).
- `jni` → 0.22 is a wholesale API rework (`JNIEnv` → `EnvUnowned`/`Env`, `GlobalRef` → `Global`,
    `AutoLocal` → `Auto`, closure-based thread attachment, mandatory `ErrorPolicy`); per upstream's
    `docs/0.22-MIGRATION.md` it rewrites `crates/iscc-jni/src/lib.rs`. Needs a dedicated step.
- `pyo3` is already at the latest (0.29.0) — no hold-back comment required, but a one-line note that
    bumps must re-verify `gil_used = true` and the `py.detach` call sites (issue #41) is welcome.

**3. Lock refresh.** `cargo update -p criterion` (or a plain `cargo build`) re-resolves; commit the
resulting `Cargo.lock`. criterion 0.7 pulls a slightly different dev-dependency subtree
(criterion-plot, clap, plotters, …), so the enforcing `Audit (cargo-deny)` gate is the real risk
here: run `mise run audit` and, if a new transitive crate trips a license or advisory rule, prefer
`cargo update -p <crate> --precise <version>` over adding a `deny.toml` exception. cargo-deny is not
preinstalled — `cargo binstall cargo-deny@0.19.9 --force`.

## Verification

- `grep -A1 '^name = "criterion"' Cargo.lock` reports `version = "0.7.` (no 0.5.x criterion remains
    in the lockfile).
- `grep -c '# held' Cargo.toml` returns **≥ 4**, and `grep -n 'held' Cargo.toml` shows a reason
    adjacent to each of the `criterion`, `uniffi`, `magnus`, and `jni` pins.
- `grep -c 'use std::hint::black_box' crates/iscc-lib/benches/benchmarks.rs` returns `1` **and**
    `grep -c 'criterion::{[^}]*black_box' crates/iscc-lib/benches/benchmarks.rs` returns `0`.
- `cargo bench --no-run` exits 0 (exactly what the CI `Bench (compile check)` job runs).
- `mise run lint` clean — in particular `cargo clippy --workspace --all-targets -- -D warnings`
    passes with no deprecation warning from the benches.
- `mise run test` passes (full Rust workspace + pytest, unchanged counts).
- `mise run audit` exits 0 (enforcing `Audit (cargo-deny)` gate against the refreshed `Cargo.lock`).
- `mise run bench:iai:check` passes against the **unmodified** `.iai-baseline.json`.
- `mise run check` — all pre-commit hooks pass (taplo keeps `Cargo.toml` formatted; no file left
    rewritten in the tree).
- `git status --porcelain crates/iscc-lib/src crates/iscc-rb/src crates/iscc-jni/src` is empty (this
    slice changes no library source).

## Done When

`criterion` is pinned at 0.7 with the bench import migrated to `std::hint::black_box`, every
deliberately held-back workspace pin carries an inline documented reason, and all verification
commands above pass on the working tree.
