# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Topic files: `MEMORY-archive.md` (gate
internals, release internals, closed milestones), `dep-refresh-survey.md` (pin inventory),
`unicode-contract.md`, `quality-gates.md`, `lint-tooling.md`. **Size budget: under 140 lines** —
archive detail eagerly; the hard cap from the agent prompt is 200.

## Exploration Shortcuts

- **Authoritative CI status** (sandbox `gh run list` is STALE — old ancestor SHAs): `gh api` on
    `"repos/iscc/iscc-lib/commits/<tip-sha>/check-runs?per_page=100"` for the real origin/develop
    tip — **QUOTE the path** (zsh globs `?`). Failed logs: `gh run view --log-failed`.
- **Unpushed check**: `git log --oneline origin/develop..HEAD`, then
    `git diff --stat origin/develop..HEAD -- . ':!.claude'`. Empty = green CI covers HEAD.
    **NON-empty = it does NOT** — report the gap instead of calling CI green-for-HEAD (bit at 148).
- **ALWAYS `tail -4 .claude/context/iterations.jsonl`** — the ONLY place a crashed role shows up
    (also the `audit` role, which runs every 10th iteration). **A non-OK status does NOT mean no
    work: corroborate with `git log` for the `cid(<role>):` commit.** Infra crash =
    `"status":"FAIL","turns":1,"cost_usd":~0.0006` AND no commit (147); benign overrun = `TIMEOUT`
    WITH the commit (148), now logged as `recovered`. A genuine review crash leaves no verdict, a
    handoff with only the advance section, and resolved issues still in issues.md.
- **Verify a Go-package claim in ~60s**: `/tmp` module requiring
    `github.com/iscc/iscc-lib/packages/go v0.0.0` + a `replace` to the repo, then
    `GOFLAGS=-mod=mod CGO_ENABLED=0 go mod tidy && CGO_ENABLED=0 go run main.go`.
- **Tier 1 pub fns**: `grep -rn "pub fn gen_\|pub const " crates/iscc-lib/src/lib.rs`.
- **Counts** (re-verified 151): crate/pkg READMEs & CLAUDE.md 12 each (**scope the glob to
    `crates/*/ packages/*/`** — bare `grep README.md` gives 14); pytest-benchmark 18; UniFFI 32;
    tracked docs `.md` **24** (23 pages + 1 `includes/`), ORDERED_PAGES + llms.txt **23**;
    `docs/howto/*.md` 11; speedups 1.3x-158x; ffi extern **47** (`'#\[unsafe(no_mangle)\]'`; bare
    `no_mangle` gives 48); iscc-lib `#[test]` **335** (`grep -rc --include="*.rs"`, sum);
    `packages/go` `^func Test` **177**.
- **GOTCHA — `git ls-files 'docs/**/*.md'` returns 13, NOT 24**: it misses top-level `docs/*.md`.
    Use both globs, or run `uv run scripts/check_docs_nav.py` (prints the authoritative page count).
- **version_sync TARGETS** = **21** (`scripts/version_sync.py --check | grep -c '^OK'`) = the
    authoritative set of version-synced files; absentees rot.
- **Issue headers**: `grep -nE '^## ' issues.md | grep -vE 'Add issues below'` (priority+source
    tag). **Trace a dep**: `cargo tree -i <crate> --target all`.
- **Specs live at `.claude/context/specs/`, NOT `specs/`** — `git diff -- specs/` returns empty for
    ANY commit and silently looks like "no spec change" (near-miss 144).

## Quality Gates — details in `quality-gates.md`, read it before reporting CI status

- **ENFORCING:** iai-callgrind perf (>10% Ir vs `.iai-baseline.json`, 16 entries); coverage + CRAP
    (`.crap-baseline.json`, **100** entries — `--fail-regression` is **CI-ONLY**, so a green
    `mise run check` proves nothing); cargo-deny; docs page-list parity (prek hook + a pytest in
    `python-test`, **no dedicated CI job**).
- **Can go red with NO code change:** cargo-deny (live advisory DB). **Fails open:** the release.yml
    static gate — check the JOB LOG for zero `warning: skipped`, not the badge.
- **Informational:** cargo-semver-checks (`continue-on-error` until v1.0.0, HELD).

## Codebase Landmarks

- `crates/` — **8 crates** (lib, py, napi, wasm, ffi, jni, rb, uniffi), all 32/32 symbols;
    iscc-uniffi has 21 tests and `publish=false`. `packages/` layout → `MEMORY-archive.md`;
    `packages/go` is the ONLY binding not inheriting the freeze rule.
- `ci.yml` — **20 YAML job entries → 21 jobs → 22 check names**: `python-test` = 3.10/3.14 matrix,
    `python` (L72) is an `if: always()` AGGREGATOR on `needs.python-test.result`; `push:` under
    `on:` is NOT a job. `release.yml` — 8 registry toggles; the Swift XCFramework step lives in
    `prepare-release` and is NOT a toggle.
- **Unicode = 16.0.0 + SENTINEL freeze rule → read `unicode-contract.md` before ANY Unicode call.**
    Criteria 1+2 MET (148); crit 3's Rust fixture COMPLETE since 149; propagation at **4 of 11
    native surfaces** (Rust, Python, WASM, Ruby) + the pure-Go port. Still open: **7 surfaces + 4
    sibling `data.json`** and the differential sweep (crit 4, no harness in `scripts/`).
    `decisions.md` keeps 2 SUPERSEDED designs that must NOT be implemented.
- **Adding a docs page = 3 hand edits** (`zensical.toml` nav, `ORDERED_PAGES`, `docs/llms.txt`), now
    gated. `streaming.rs`: `DataHasher`/`InstanceHasher` at crate root, `SumHasher` only via
    `streaming::`. iscc-wasm's `blake3 wasm32_simd` dep is feature-unification — **don't prune**.
    `iscc-py` has **12** `.detach(` GIL-release sites. `benches/`: `benchmarks.rs` 12 criterion
    (0.7, `black_box` from `std::hint`), `iai_benches.rs` iai-callgrind 0.16 (11 fns, 16 cases).
- **Inline `# held:` comments = authoritative pin rationale**: root `Cargo.toml` has **4**
    (criterion 0.8, jni 0.22, magnus 0.8, uniffi 0.32) + a pyo3 `# note:`; `pyproject.toml` has
    zero. MSRV: `cargo info <crate>@<v>`.
- **Ruff/prek/mdformat → `lint-tooling.md`.** ruff **0.16.0** since 137; prek/CI parity holes CLOSED
    (138, 139) — local prek is a strict SUPERSET. Probe hooks with `prek run <hook> --files <f>`.
- **Dependency-pin inventory, GHA histograms + slice history** → `dep-refresh-survey.md`. All GHA
    refs CURRENT (slice 9, 140); release.yml keeps **97 `uses:`** (18 distinct) and **no setup-uv**.
    `rubygems/configure-rubygems-credentials@main` (`release.yml:895`) = the ONLY unpinned ref —
    RULED `@v2.1.0`, CID-authorized, not yet applied. No Dependabot/Renovate; **`rb_sys` pinned in
    THREE places that move together** — Gemfile (`0.9.123`), `Gemfile.lock`, `release.yml:853`.

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward the rest, CI via check-runs API on the real tip. **Always diff
    `.claude/context/specs/`** (132, 147) and read issues.md *bodies* (145) — either flips met→unmet
    with ZERO code change.
- **When a ruling lands, re-verify the CODE against the NEW spec**; grep `decisions.md` for
    `supersede` (losing designs are logged chronologically). At 147 a 14-iteration "met" went unmet.
- **Reproduce/refute binding claims yourself** — cheap probes beat inherited text (confirmed a
    `critical` bug at 147, its fix at 148; at 151 refuted "the stale napi `.node` is *checked in*" —
    `git check-ignore -v` shows `crates/iscc-napi/.gitignore:4:*.node`, it is a local artifact CI
    rebuilds). Deliberate deviations from a fix sketch are legitimate: read the in-source comment
    first.
- **Spec checkboxes are NOT a progress signal** — cpp/docs/dotnet/java/kotlin/nodejs/ruby/swift sit
    at 0/N checked though MET; only `ci-cd.md` (44/52) + `rust-core.md`'s semver box are kept up.
    Spec *prose* also rots: at 149 `rust-core.md` still described a defect fixed 2 iterations back.
- **Two 149 lessons, both about oracles:** a "must NOT be" value is meaningful only if the DESIGN
    producing it is named (a mislabelled column reached published docs with every gate green), and a
    fixture can be structurally incapable of gating what it claims to. Ask what a test must
    *distinguish*, not whether it passes — green gates say nothing about truth.

## Current State (assessed-at: c548399, iter 152)

- **IN_PROGRESS — CI GREEN AND COVERING ALL CODE.** `origin/develop` == `b4e4f1f`: 43 check-runs, 22
    names, 0 non-success, 0 in progress. HEAD `c548399` is ONE commit ahead (a `cid(log)` commit)
    but `git diff --stat origin/develop..HEAD -- . ':!.claude'` is EMPTY, so the green run covers
    every line of code. ~2x runs because PR **#44 (develop→main) is OPEN** ("Release 0.6.0" — NOT
    shipped; version **0.5.0**).
- Iteration 151 ran clean: 4 roles all `OK`, verdict `PASS`. Diff since = 5 non-`.claude` files (2
    new tests + 2 CLAUDE.md + `docs/unicode.md`); specs + `.github/` byte-unchanged; no
    `crates/*/src/` file and neither baseline moved.
- **Statuses:** Rust-core partially met (crit 1+2 MET; crit 3 at **4 of 11 native surfaces** + the
    pure-Go port; crit 4 not started); everything else met except CI/CD (partial). **Next = the
    byte-identity drift gate FIRST** (sequenced before the `packages/{dotnet,kotlin,swift}` slice,
    which adds 3 more copies), then propagation slice 3 — **C# + C++ is the cheapest pair needing no
    addon rebuild**. Ledger + slice-cost survey → `unicode-contract.md`.
- All four gated suites ride EXISTING CI jobs (`python-test`, `Go`, `WASM`, `ruby`) — no new job was
    needed and none should be added.
- **`specs/rust-core.md` is STALE on Go `Final_Sigma`** (present tense at L149-157, box unchecked)
    though fixed at 147; crit-1/crit-3 boxes also unchecked though met — human-owned, CID doesn't
    edit specs.
- **Issues: 9** (5 `normal`, 4 `low`, 0 critical, 0 `HUMAN REVIEW REQUESTED`) — none opened or
    closed at 151. Still open from 150: `[review]` **"Gate byte-identity of the vendored test-vector
    copies"** — 6 vendored copies (5 × `data.json`, one md5 `4f17639ab1dd…` + 1 ×
    `unicode_boundary.json`, SHA `3ccc4418…`) match by convention only; all agree at HEAD and
    `tests/` holds no such anchor yet.
- **Human backlog CLEARED.** AUTHORIZED for CID: rubygems `@v2.1.0` pin (still `@main` at
    `release.yml:895`); major dep bumps **one per step** (magnus 0.8, jni 0.22 as source rewrites;
    xunit 3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x); exhaustive `specs/ci-cd.md` job table.
    DEFERRED: npm OIDC (not v0.6.0; don't even prepare the diff). The 3 gate-script remainders are
    `low` + trigger-contingent.
- **Don't re-flag as DONE**: WASM+Ruby fixture propagation 151; Python+Go 150; sequence vectors +
    fixture guard 149; sentinel conversion + the `docs/unicode.md` rewrite 148; Go `Final_Sigma`
    147; gate blind-spots 146; docs-list gate 145 (pre-145 → `MEMORY-archive.md` / issues.md slice
    log).

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat** aborts commits on several rewrap edge cases; abort list → `lint-tooling.md`. Recipe:
    `cp f /tmp/c.md && uv run mdformat --wrap 100 --number /tmp/c.md && diff`, adopt, re-run once.
- **Exec-bit changes are INVISIBLE here** (`core.fileMode=false` bind mount): a mode-only commit
    shows nothing in `git status`/`diff` (`--stat` = a 0-line file). Use `git ls-files -s <path>`.
- **`go run` rejects a file named `*_test.go`** — name throwaway probes `main.go`.
- **Grep binding APIs CASE-INSENSITIVELY** (near-miss 152): napi exports are **snake_case**
    (`text_clean`), so a `textClean` grep wrongly reported napi as having no text tests;
    Java/Kotlin/ Swift would be camelCase, C# PascalCase. Also `crates/iscc-napi/__tests__` (two
    underscores).
- **Env gotchas → `MEMORY-archive.md`**: metrics.jsonl counts gitignored artifacts; Gradle
    bind-mount flakes; `proc-macro-error2` future-incompat warning (dev-only, not a regression).
- **csbindgen** runs on every `cargo build` (`crates/iscc-ffi/build.rs`). **UniFFI** = proc-macro,
    no uniffi.toml/build.rs. **Kotlin** uses JNA (not JNI) — needs BOTH `java.library.path` AND
    `jna.library.path`. **Release-workflow deep internals** → `MEMORY-archive.md`.
