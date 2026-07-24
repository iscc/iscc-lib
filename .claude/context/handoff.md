# Handoff

## 2026-07-24 — Review of: Rust direct-pin evaluation — bump criterion to 0.7, document held-back majors

**Verdict:** PASS

**Summary:** Slice 3 of the dependency-refresh issue landed exactly as scoped: `criterion` 0.5 → 0.7
in `[workspace.dependencies]`, the deprecated `criterion::black_box` import swapped for
`std::hint::black_box` in `benchmarks.rs` (all 30 call sites byte-identical), and inline `# held:`
reasons committed beside the four deliberately held pins plus a `pyo3`/#41 note. Three non-generated
files touched (`Cargo.toml`, `benchmarks.rs`, plus generated `Cargo.lock`), no library source, all
eight gates green including the enforcing `audit` and `bench:iai:check` against unmodified
baselines.

**Verification:**

- [x] `grep -A1 '^name = "criterion"' Cargo.lock` → `version = "0.7.0"`; exactly one `criterion`
    package entry, no 0.5.x remains
- [x] `grep -c '# held' Cargo.toml` → **4**; `grep -n 'held' Cargo.toml` confirms one adjacent to
    each of criterion (L44), jni (L49), magnus (L53), uniffi (L58) — plus a `# note:` on pyo3 (L35)
- [x] `grep -c 'use std::hint::black_box' benchmarks.rs` → `1`;
    `grep -c 'criterion::{[^}]*black_box' benchmarks.rs` → `0`
- [x] `cargo bench --no-run` exit **0** (compiles both `benchmarks.rs` and `iai_benches.rs`)
- [x] `mise run lint` exit **0** — `clippy --workspace --all-targets -D warnings` clean, no
    deprecation warning; ruff "All checks passed!", 24 files already formatted
- [x] `mise run test` exit **0** — Rust workspace 426 tests across 15 suites (270 iscc-lib, 85 ffi,
    28 + 22 integration, 21 uniffi), 0 failed, 0 ignored; **314 pytest passed**
- [x] `mise run audit` exit **0** — "advisories ok, bans ok, licenses ok, sources ok" on the
    refreshed lockfile (criterion 0.7's clap/criterion-plot 0.6/itertools 0.13 subtree cleared
    cargo-deny)
- [x] `mise run bench:iai:check` exit **0** — 16/16 within 10% of the **unmodified** baseline (max
    +1.96% on `bench_mixed_code.two_codes`); both baseline JSON files clean per
    `git status --porcelain`
- [x] `mise run check` exit **0** — all 15 pre-commit hooks pass, **no file left rewritten** (taplo
    preserved every `# held:` comment; mdformat clean on `next.md` this cycle)
- [x] `git status --porcelain crates/*/src` empty — no library source touched

**Independent claim checks (beyond next.md):**

- `cargo info criterion@0.8.2` → `rust-version: 1.86` — the MSRV hold-back reason is factually
    correct against the declared `rust-version = "1.85"`
- `grep -c 'exception::runtime_error' crates/iscc-rb/src/lib.rs` → 5, and 41 uses of the old
    `JNIEnv`/`GlobalRef`/`AutoLocal` types in `crates/iscc-jni/src/lib.rs` — both hold-back comments
    describe real, non-trivial migrations, not procrastination
- Gate-circumvention sweep over the full unpushed range (`@{upstream}..HEAD`, 14 files): **no**
    added `#[allow]`, `#[ignore]`, `noqa`, `type: ignore`, `--no-verify`, threshold reduction, hook
    removal, or ignore-list widening

**Issues found:**

- **(fixed in this review, doc-only)** The advance handoff attributed the `proc-macro-error2 v2.0.1`
    future-incompat warning to "the magnus/rb-sys subtree". That is wrong:
    `cargo tree -i proc-macro-error2 --target all` shows it comes from `iai-callgrind-macros` →
    `iai-callgrind` (dev-dependency of `iscc-lib`). Left uncorrected this would have sent the Ruby
    slice chasing a dependency it does not own. Recorded correctly in issues.md and learnings.md; no
    fixed upstream release exists (`iai-callgrind` 0.16.1 is latest), so it stays a warning.
- No correctness, scope, or architecture problems. The `@napi-rs/cli: ^3` survey line in next.md's
    pin table was accurate.

**Codex review:** Clean — "The Criterion upgrade and black_box migration compile cleanly, including
with the declared Rust 1.85 MSRV. Formatting, clippy, workspace tests, and benchmark compilation
pass without regressions." No actionable findings.

**Next:** Continue the dependency-refresh issue. The remaining per-binding-manifest work is thinner
than it looks — the only JS manifest is `crates/iscc-napi/package.json` with a single
`@napi-rs/cli: ^3` devDependency already covered by its caret, and there is no
`packages/*/package.json` at all. Recommend making **slice 4 the tooling pins**, which is the
meatiest self-contained remainder and directly touches gate integrity:

- `.pre-commit-config.yaml`: only two pinned repos — `pre-commit/pre-commit-hooks` (rev `v6.0.0`)
    and `executablebooks/mdformat` (rev `1.0.0`); everything else is `repo: local` driven by
    `uv`/`cargo`
- GitHub Actions majors that look behind: `astral-sh/setup-uv@v4` and `actions/checkout@v4` (v5
    lines exist for both); the rest (`upload/download-artifact@v4`, `setup-python@v5`,
    `setup-go@v5`, `rust-cache@v2`, `install-action@v2`, …) should be confirmed individually
- `mise.toml` has **no `[tools]` section** — tool versions are not pinned there, so scope the slice
    to `.pre-commit-config.yaml` + `.github/workflows/`, not `mise.toml`

Then split the remaining manifests by ecosystem (jni `pom.xml` + kotlin `build.gradle.kts`; dotnet
`.csproj`; go `go.mod`; rb `Gemfile`/gemspec), and keep the three flagged migration steps separate:
ruff 0.16 adoption, `magnus` 0.8 (`Ruby::exception_runtime_error()` refactor in `iscc-rb`), and
`jni` 0.22 (`iscc-jni` rewrite per upstream `docs/0.22-MIGRATION.md`).

**Notes:**

- **Risk to flag for the tooling-pins slice:** bumping the `mdformat` rev can reformat every
    Markdown file in the repo, and a `pre-commit-hooks` bump can add new default checks. Define-next
    should either raise the file budget explicitly for the mechanical reformat churn or hold the
    formatter revs back with a documented `# held:`-equivalent comment — do **not** let a reformat
    wave hide a substantive change.
- The `# held:` comments in the root `Cargo.toml` are now the authoritative source for why
    criterion/jni/magnus/uniffi are pinned below latest. Future dep steps should read them first
    instead of re-deriving the blockers, and must update them when a hold-back is lifted.
- MSRV pre-check recipe: `cargo info <crate>@<version>` prints `rust-version` directly — cheaper
    than cloning or reading a changelog when evaluating a major bump against
    `rust-version = "1.85"`.
- Raising the workspace MSRV to 1.86 (which would unlock criterion 0.8) remains a human policy call
    tied to the v1.0.0 cut — CID must not take it as a dependency-refresh side effect.
- `.claude/context/iterations.jsonl` is modified in the working tree (CID runner's log) and is not
    staged, per protocol.
