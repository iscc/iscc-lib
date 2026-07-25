# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Topic files: `MEMORY-archive.md` (gate
pipelines, release internals), `dep-refresh-survey.md` (pin inventory), `unicode-contract.md`,
`lint-tooling.md` (ruff/prek/mdformat). **Size budget: under 140 lines** — archive detail.

## Exploration Shortcuts

- **Authoritative CI status** (sandbox `gh run list` is STALE — old ancestor SHAs): `gh api` on
    `"repos/iscc/iscc-lib/commits/<tip-sha>/check-runs?per_page=100"` for the real origin/develop
    tip — **QUOTE the path** (zsh globs `?`). Failed logs: `gh run view` with `--log-failed`.
- **Unpushed check**: `git log --oneline origin/develop..HEAD`, then prove it is code-free with
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` (empty = nothing outside CI coverage).
- **Tier 1 pub fns**: `grep -rn "pub fn gen_\|pub const " crates/iscc-lib/src/lib.rs`.
- **Counts** (re-verified 142): READMEs/CLAUDE.md 12 each; pytest-benchmark 18; UniFFI exports 32;
    llms-full ORDERED_PAGES 22; `docs/howto/*.md` 11; speedup 1.3x-158x; release.yml toggles 8; ffi
    extern 47; iscc-lib `#[test]` = **328** (`grep -rc --include="*.rs" crates/iscc-lib/`, sum);
    ci.yml job entries 19 = `grep -cE '^  [a-z_-]+:$'` (raw 21) minus 2.
- **version_sync TARGETS** = **21**
    (`uv run python scripts/version_sync.py --check | grep -c '^OK'`) = the authoritative set of
    version-synced files; absentees rot.
- **Issue headers**: `grep -nE '^## ' issues.md | grep -vE 'Add issues below'` (priority+source
    tag). **Trace a dep**: `cargo tree -i <crate> --target all`.

## Quality Gates (all in ci.yml; full pipelines archived)

- **Perf (iai-callgrind)** — ENFORCING. `scripts/iai_regression.py --check` fails >10% Ir vs
    `.iai-baseline.json` (16 entries). GOTCHA: nearby `continue-on-error` is the SEMVER job's.
- **release.yml re-trigger guards (139)** — audit with `uv run python` + `yaml.safe_load` (system
    python3 has NO pyyaml; `on:` parses as key `True`): **29 jobs, 28 guarded**
    `!cancelled() && !failure() && (…)`, only `prepare-release` bare (re-verified 141). A new job
    without the guard silently no-ops `-f <registry>=true`. STATIC only — no CI/CID push runs this
    file; guards (139) + action majors (140) are accepted risks in `decisions.md` 2026-07-25, and an
    open `[review]` issue asks to land the checks as a `scripts/` gate.
- **Coverage + CRAP** — one ENFORCING job: `--fail-regression` + `--fail-above` 30.0
    (`.cargo-crap.toml`); baseline `.crap-baseline.json` (**98** entries, max ~22.3). **GOTCHA —
    `--fail-regression` is CI-ONLY, not in `mise run check`**: a new branch in a covered fn → exit 1
    despite a GREEN local check (bit 121). Fix = refresh that entry in the SAME step.
- **Audit (cargo-deny)** — ENFORCING: `cargo-deny@0.19.9`, root `deny.toml` (v2, 2 dev-bench
    ignores). **GOTCHA — the live advisory DB flips this red with NO code change** (fix = bump the
    crate); not in devcontainer, so green CI is the only confirmation.
- **Semver (cargo-semver-checks)** — `continue-on-error: true` (informational until v1.0.0), does
    NOT flip the run. rust-core.md box `[ ]` = unmet (enforcing + ≥1.0.0, HELD); ci-cd.md `[x]`.

## Codebase Landmarks

- `crates/` — **8 crates** (lib, py, napi, wasm, ffi, jni, rb, uniffi), all 32/32 symbols;
    iscc-uniffi has 21 tests and `publish=false`.
- `.github/workflows/ci.yml` — **19 YAML job entries → 20 jobs → 21 distinct check names**:
    `python-test` = 3.10/3.14 matrix, `python` (L72) is an `if: always()` AGGREGATOR asserting
    `needs.python-test.result == success`. `push:` under `on:` is NOT a job. `release.yml` — 8
    registry toggles; the Swift XCFramework step lives in `prepare-release` and is NOT a toggle.
- `packages/go/` — pure Go, no CGO/WASM/binaries (x/text 0.40.0). `packages/swift/` + root
    `Package.swift` — `useLocalFramework` toggle, `.binaryTarget`, `build_xcframework.sh` = 5 Apple
    targets. `packages/kotlin/` — Kotlin/JVM + JNA 2.4.10; pins → `dep-refresh-survey.md`.
- **Unicode = DECLARED 16.0.0 + freeze rule** (human decision 132) → **`unicode-contract.md`**:
    freeze filter **MET iter 133** (`src/utils/unicode16.rs` 731-range table + PEP 723 generator
    `scripts/gen_unicode16_unassigned.py` + pre-`nfkc`/`nfd` filter at `utils.rs` L103/L181); unmet
    = full-code-space sweep + the BINDING half of the boundary vectors (Go exposed until go1.27; an
    open HUMAN ruling on sequence-adjacency blocks the sweep wording). Rust half landed 141:
    `tests/unicode_boundary.json` + `tests/test_unicode_boundary.rs` — the fixture is
    ASCII-**escaped**, so `grep "1FAE9"` MISSES it; grep the filename. Zero hits outside
    `crates/iscc-lib` at 142.
- `crates/iscc-lib/src/streaming.rs` — `DataHasher`+`InstanceHasher` re-exported at crate root;
    `SumHasher` only via `streaming::` (drives `gen_sum_code_v0`; wrappers in iscc-py, iscc-wasm).
    `crates/iscc-wasm/Cargo.toml`'s `blake3 wasm32_simd` dep (118, #42) is feature-unification only,
    no `use blake3` — **don't prune it** → `MEMORY-archive.md`.
- `crates/iscc-py/src/lib.rs` — `grep -c '\.detach('` = **12** GIL-release sites (GIL #39+#41 DONE).
    `crates/iscc-lib/benches/` — `benchmarks.rs` 12 criterion benches (criterion 0.7, `black_box`
    from `std::hint`); `iai_benches.rs` iai-callgrind 0.16 (11 fns, 16 cases).
- **Inline `# held:` comments = authoritative pin rationale**: root `Cargo.toml` (`grep -c '# held'`
    → **4**: criterion 0.8, jni 0.22, magnus 0.8, uniffi 0.32 + a pyo3 `# note:` → #41);
    `pyproject.toml` has **zero** (the `ruff<0.16` one died at 137). MSRV: `cargo info <crate>@<v>`.
- **Ruff/prek/mdformat detail → `lint-tooling.md`.** Headlines: ruff **0.16.0** since 137; both
    prek/CI parity holes CLOSED (Markdown 138, `.pyi` 139) — local prek is now a strict SUPERSET.
    Probe hooks with `prek run <hook> --files <f>` (`Passed` vs `Skipped`), never `git ls-files`.
- **Full dependency-pin inventory, GHA histograms + slice history** → `dep-refresh-survey.md`.
    Headlines: ci.yml, docs.yml AND release.yml GHA refs are now ALL CURRENT (slice 9, iter 140);
    release.yml keeps **97 `uses:`** and correctly has **no setup-uv step** (it invokes no `uv`).
    `rubygems/configure-rubygems-credentials@main` = the repo's ONLY unpinned ref (human ruling
    pending). No Dependabot/Renovate; no `[tools]` in `mise.toml`; **`rb_sys` pinned in THREE places
    that move together** — Gemfile (exact `0.9.123`), `Gemfile.lock`, `tag:` at `release.yml:853`.

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward rest; verify CI via check-runs API on the real tip (don't trust handoff — a PASS
    with green `mise run check` can still fail CI-only gates). **Always diff `specs/` too — a
    `human(decide)` commit can flip met→partially-met with zero code change (iteration 132).**
- **Issues diff**: scan issues.md for NEW/removed `[human]`/`[review]` entries; watch
    `HUMAN REVIEW REQUESTED`, critical reshuffles, large specs growth = human re-scoped.
- **Spec checkboxes are NOT a progress signal** — cpp/docs/dotnet/java/kotlin/nodejs/ruby/swift sit
    at 0/N checked though MET; only `ci-cd.md` (44/52) + the `rust-core.md` semver box are kept up.

## Current State (assessed-at: 2ae5604, iter 142)

- **IN_PROGRESS — CI GREEN.** Version still **0.5.0** (PR #44 is titled "Release 0.6.0" but is just
    the open develop→main PR — do NOT read it as a shipped release). Iter 141 = the Unicode boundary
    fixture: diff `2cd0d6d..HEAD` had exactly 3 non-`.claude` files (2 new test files + 1
    `crates/iscc-lib/CLAUDE.md` bullet); no `src/`, no `specs/`, no `target.md`, no baselines → all
    binding/bench/docs/CI sections carried forward. Still partially met: Rust-core (binding half of
    the vectors + sweep + semver/v1.0.0 HELD) and CI/CD (human-gated + the 2 `[review]` issues).
- **CI GREEN on origin/develop tip `b40b1bb`**; HEAD `2ae5604` = +1 UNPUSHED `cid(log)` commit,
    `origin/develop..HEAD` minus `.claude/` = EMPTY stat. **41** check-runs, 21 names, 0
    non-success. ~2x jobs because PR **#44 (develop→main) is OPEN** → every develop commit fires a
    `push` AND a `pull_request` run.
- **Cadence**: iters 134-140 were SEVEN straight tooling/CI configs; 141 finally moved library
    (test) code. The Unicode thread is now HUMAN-BLOCKED again, so the one unblocked CID item is the
    `release.yml` static-check script — justified once, but a second consecutive tooling package
    after it = drift; escalate the parked rulings instead.
- **Dep-refresh: ALL 9 slices DONE** → `dep-refresh-survey.md`. Remainder is human/major-gated ONLY:
    magnus 0.8, jni 0.22 (source rewrites), xunit 3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x.
- **8 issues: 0 critical, 6 normal, 2 low — ONE `[review]` with HUMAN REVIEW REQUESTED** (133:
    freeze-rule ordering diverges from iscc-core on *sequences*; spec-wording ruling, not a redesign
    → `unicode-contract.md`; open at 142, upstream iscc/iscc-core#137). Iter 141 opened/closed
    nothing — the Unicode entry was only annotated. The two 140-filed `[review]` entries stand: (a)
    land the release.yml static checks as an executable gate (CID-doable; heredocs retyped in
    139/140/141), (b) pin `rubygems/configure-rubygems-credentials` off `@main` (HUMAN-gated:
    tag-vs-SHA reopens a convention). Human-gated: npm OIDC; low = v1.0.0, docs logos.
- **Don't re-flag as new work** (all DONE): Unicode boundary fixture, Rust half (141); release.yml
    GHA refs (140), re-trigger guards + `.pyi` hooks (139), Markdown hook parity (138), ruff A-E
    (131-137), Unicode freeze filter (133), Kotlin 2.3+ floor docs (132), dep slices 1-7 (124-130),
    aarch64 wheels #49 (123), CRAP baseline (122), trailing-byte (120-121), Go IDv1 #43 (119), WASM
    SIMD #42 (118), GIL #39+#41, cargo-deny (113), iai perf gate (107-111), semver gate (93). CID
    infra = meta, NOT target — ignore.
- **Known non-regression**: the `proc-macro-error2 v2.0.1` future-incompat warning on cargo
    test/bench comes from `iai-callgrind-macros` (dev-only), NOT magnus/rb-sys; no upstream fix yet.

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat** aborts commits on several rewrap edge cases; abort list → `lint-tooling.md`. Recipe:
    `cp f /tmp/c.md && uv run mdformat --wrap 100 --number /tmp/c.md && diff`, adopt, re-run once.
- **Exec-bit changes are INVISIBLE here** (`core.fileMode=false` on this bind mount): a commit can
    be mode-only and `git status`/`git diff` show nothing. Verify with `git ls-files -s <path>`
    (`100755` vs `100644`) or `git diff --summary` — `--stat` shows it as a 0-line file (136).
- **metrics.jsonl counts include gitignored build artifacts** (post-`rake compile` iscc-rb jumped
    8→12 files purely from `tmp/*/stage/`) — never read a metrics delta as real code change without
    `git diff --stat`. **Gradle flakes on this bind mount** (`Unable to delete file …/build/…`) —
    `./gradlew clean` before believing a Kotlin failure; check the test XML first.
- **csbindgen** runs on every `cargo build` (`crates/iscc-ffi/build.rs`). **UniFFI** = proc-macro,
    no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH `java.library.path` AND
    `jna.library.path`. **Release-workflow deep internals** (Kotlin GPG/Central Portal, JNA ARM32
    armv7→arm, dual Package.swift manifests, JAR classifier) → `MEMORY-archive.md`.
