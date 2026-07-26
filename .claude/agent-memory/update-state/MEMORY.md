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
- **Counts** (re-verified 144): READMEs/CLAUDE.md 12 each; pytest-benchmark 18; UniFFI exports 32;
    llms-full ORDERED_PAGES **23** (was 22 pre-143); `docs/howto/*.md` 11; `docs/*.md` 11 top-level;
    speedup 1.3x-158x; release.yml toggles 8; ffi extern 47; iscc-lib `#[test]` = **328**
    (`grep -rc --include="*.rs" crates/iscc-lib/`, sum); ci.yml job entries 19 =
    `grep -cE '^  [a-z_-]+:$'` (raw 21) minus 2.
- **version_sync TARGETS** = **21**
    (`uv run python scripts/version_sync.py --check | grep -c '^OK'`) = the authoritative set of
    version-synced files; absentees rot.
- **Issue headers**: `grep -nE '^## ' issues.md | grep -vE 'Add issues below'` (priority+source
    tag). **Trace a dep**: `cargo tree -i <crate> --target all`.
- **Specs live at `.claude/context/specs/`, NOT `specs/`** — `git diff -- specs/` returns empty for
    ANY commit and silently looks like "no spec change" (near-miss 144). Always diff
    `.claude/context/specs/`.

## Quality Gates (all in ci.yml; full pipelines archived)

- **Perf (iai-callgrind)** — ENFORCING. `scripts/iai_regression.py --check` fails >10% Ir vs
    `.iai-baseline.json` (16 entries). GOTCHA: nearby `continue-on-error` is the SEMVER job's.
- **release.yml static gate (LANDED 142)** — `scripts/check_release_workflow.py` (`uv run` it; exit
    0 + one `OK:` line) covers checks 1-2: guard shape (**29 jobs, 28 guarded**, only
    `prepare-release` bare) + input wiring, artifact wiring with `matrix.include` expansion,
    `needs:` graph. Prek hook `check-release-workflow` (release.yml only) + CI via
    `tests/test_check_release_workflow.py` (**11** tests, run by the bare `uv run pytest`); `pyyaml`
    is a dev dep. **Check 3 (action-input compat vs published `action.yml`) is NOT built.**
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
- `packages/go/` — pure Go, no CGO/WASM (x/text 0.40.0). `packages/swift/` + root `Package.swift` —
    `useLocalFramework`, `.binaryTarget`, `build_xcframework.sh` = 5 Apple targets.
    `packages/kotlin/` — Kotlin/JVM + JNA 2.4.10; pins → `dep-refresh-survey.md`.
- **Unicode = DECLARED 16.0.0 + freeze rule** (human decision 132) → **`unicode-contract.md`**:
    filter **MET 133** (`src/utils/unicode16.rs` 731 ranges + generator
    `scripts/gen_unicode16_unassigned.py` + pre-`nfkc`/`nfd` filter at `utils.rs` L103/L181); unmet
    = full-code-space sweep + BINDING half of the boundary vectors (Go exposed until go1.27; a HUMAN
    ruling on sequence-adjacency blocks the sweep wording). Rust half landed 141:
    `tests/unicode_boundary.json` + `tests/test_unicode_boundary.rs` — fixture is ASCII-**escaped**,
    so `grep "1FAE9"` MISSES it; grep the filename. Zero hits outside `crates/iscc-lib` at 144.
    **Docs half (step c) landed 143**: `docs/unicode.md` (100 lines) + a `!!! note "Unicode tables"`
    in `docs/howto/go.md`; carries a DELIBERATE placeholder (CPython-3.14 claim scoped to
    single-code-point behaviour) that must move with the ordering ruling → issue point 4.
- **Docs 3-way wiring has NO gate**: a new page must be hand-added to `zensical.toml` nav +
    `scripts/gen_llms_full.py` ORDERED_PAGES + `docs/llms.txt`. Check all three when a `docs/*.md`
    appears in a diff (all three were correct at 144).
- `crates/iscc-lib/src/streaming.rs` — `DataHasher`+`InstanceHasher` re-exported at crate root;
    `SumHasher` only via `streaming::` (drives `gen_sum_code_v0`; wrappers in iscc-py, iscc-wasm).
    `crates/iscc-wasm/Cargo.toml`'s `blake3 wasm32_simd` dep (118, #42) is feature-unification only,
    no `use blake3` — **don't prune it** → `MEMORY-archive.md`.
- `crates/iscc-py/src/lib.rs` — `grep -c '\.detach('` = **12** GIL-release sites (GIL #39+#41 DONE).
    `crates/iscc-lib/benches/` — `benchmarks.rs` 12 criterion benches (criterion 0.7, `black_box`
    from `std::hint`); `iai_benches.rs` iai-callgrind 0.16 (11 fns, 16 cases).
- **Inline `# held:` comments = authoritative pin rationale**: root `Cargo.toml` (`grep -c '# held'`
    → **4**: criterion 0.8, jni 0.22, magnus 0.8, uniffi 0.32 + a pyo3 `# note:` → #41);
    `pyproject.toml` has **zero** hold-backs (`ruff<0.16` died at 137) but its `dev` group now
    carries `pyyaml` with an inline why-comment. MSRV: `cargo info <crate>@<v>`.
- **Ruff/prek/mdformat detail → `lint-tooling.md`.** ruff **0.16.0** since 137; prek/CI parity holes
    CLOSED (Markdown 138, `.pyi` 139) — local prek is a strict SUPERSET. Probe hooks with
    `prek run <hook> --files <f>` (`Passed` vs `Skipped`), never `git ls-files`.
- **Dependency-pin inventory, GHA histograms + slice history** → `dep-refresh-survey.md`. ci.yml,
    docs.yml AND release.yml GHA refs ALL CURRENT (slice 9, iter 140); release.yml keeps **97
    `uses:`** and correctly has **no setup-uv step**. `rubygems/configure-rubygems-credentials@main`
    = the ONLY unpinned ref (human ruling pending). No Dependabot/Renovate; no `[tools]` in
    `mise.toml`; **`rb_sys` pinned in THREE places that move together** — Gemfile (exact `0.9.123`),
    `Gemfile.lock`, `tag:` at `release.yml:853`.

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward rest; verify CI via check-runs API on the real tip (don't trust handoff — a PASS
    with green `mise run check` can still fail CI-only gates). **Always diff `specs/` too — a
    `human(decide)` commit can flip met→partially-met with zero code change (iteration 132).**
- **Issues diff**: scan for NEW/removed `[human]`/`[review]` entries + `HUMAN REVIEW REQUESTED`; a
    rewritten-in-place issue keeps the count flat, so diff issue *bodies*, not just headers.
- **Spec checkboxes are NOT a progress signal** — cpp/docs/dotnet/java/kotlin/nodejs/ruby/swift sit
    at 0/N checked though MET; only `ci-cd.md` (44/52) + `rust-core.md`'s semver box are kept up.

## Current State (assessed-at: 5f75fea, iter 144)

- **IN_PROGRESS — CI GREEN.** Version still **0.5.0** (PR #44 is titled "Release 0.6.0" but is just
    the open develop→main PR — do NOT read it as a shipped release). Iter 143 = DOCS-ONLY: diff
    `b6308e9..HEAD` had exactly 5 non-`.claude` files (`docs/unicode.md` new, `docs/howto/go.md`,
    `docs/llms.txt`, `scripts/gen_llms_full.py`, `zensical.toml`); NO `crates/`, `packages/`,
    `.claude/context/specs/`, `target.md`, `.github/` or manifest change → every library section
    carried forward. Still partially met: Rust-core (binding half of the vectors + sweep +
    semver/v1.0.0 HELD) and CI/CD (check 3 + human-gated items).
- **CI GREEN on origin/develop tip `cd56117`**; HEAD `5f75fea` = +1 UNPUSHED `cid(log)` commit,
    `origin/develop..HEAD` minus `.claude/` = EMPTY stat. **41** check-runs, 21 names, 0
    non-success. ~2x jobs because PR **#44 (develop→main) is OPEN** → every develop commit fires a
    `push` AND a `pull_request` run.
- **Cadence — DRIFT WATCH.** 134-140 = seven straight tooling/CI configs; 141 library test code; 142
    tooling; 143 docs. Check 3 next = 3 of the last 5 iterations workflow-tooling on a file no CI
    run executes. States 143 AND 144 both recommend **escalating the two parked rulings**
    (freeze-rule ordering wording; Go 15.0-tables vendor-or-skip) — 143 finished the LAST Unicode
    sub-task doable without a ruling, so criteria 3+4 are now fully double-blocked.
- **Dep-refresh: ALL 9 slices DONE** → `dep-refresh-survey.md`. Remainder is human/major-gated ONLY:
    magnus 0.8, jni 0.22 (source rewrites), xunit 3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x.
- **8 issues: 0 critical, 6 normal, 2 low — ONE `[review]` with HUMAN REVIEW REQUESTED** (133:
    freeze-rule ordering diverges from iscc-core on *sequences*; spec-wording ruling → open at 143,
    upstream iscc/iscc-core#137). **Count has been flat at 8 for three iterations while bodies moved
    every time** (142 rewrote the gate issue → only check 3 left; 143 amended the Unicode issue with
    step (c) ✅ and added point 4 to the freeze-rule issue) — never read an unchanged count as
    "nothing happened". Also open: pin `rubygems/configure-rubygems-credentials` off `@main` (HUMAN:
    tag-vs-SHA); npm OIDC; low = v1.0.0, docs logos.
- **Don't re-flag as new work** (DONE): Unicode consumer docs page (143); release.yml static checks
    1-2 gate (142); Unicode boundary fixture Rust half (141); release.yml GHA refs (140); re-trigger
    guards + `.pyi` hooks (139); Markdown hook parity (138); ruff 0.16 sub-slices A-E (131-137);
    Unicode freeze filter (133); Kotlin 2.3+ floor docs (132); dep slices 1-7 (124-130). Pre-124 →
    `MEMORY-archive.md`.

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat** aborts commits on several rewrap edge cases; abort list → `lint-tooling.md`. Recipe:
    `cp f /tmp/c.md && uv run mdformat --wrap 100 --number /tmp/c.md && diff`, adopt, re-run once.
- **Exec-bit changes are INVISIBLE here** (`core.fileMode=false` on this bind mount): a commit can
    be mode-only and `git status`/`git diff` show nothing. Verify with `git ls-files -s <path>`
    (`100755` vs `100644`) or `git diff --summary` — `--stat` shows it as a 0-line file (136).
- **Env gotchas → `MEMORY-archive.md`**: metrics.jsonl counts gitignored artifacts; Gradle
    bind-mount flakes; the `proc-macro-error2` future-incompat warning (dev-only, not a regression).
- **csbindgen** runs on every `cargo build` (`crates/iscc-ffi/build.rs`). **UniFFI** = proc-macro,
    no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH `java.library.path` AND
    `jna.library.path`. **Release-workflow deep internals** → `MEMORY-archive.md`.
