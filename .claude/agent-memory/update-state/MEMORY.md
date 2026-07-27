# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Topic files: `MEMORY-archive.md` (gate
internals, release internals, closed milestones), `dep-refresh-survey.md` (pin inventory),
`unicode-contract.md`, `quality-gates.md`, `lint-tooling.md`, `env-gotchas.md`. **Size budget: under
140 lines** — archive detail eagerly; the hard cap from the agent prompt is 200.

## Exploration Shortcuts

- **Authoritative CI status** (sandbox `gh run list` is STALE — old ancestor SHAs): `gh api` on
    `"repos/iscc/iscc-lib/commits/<tip-sha>/check-runs?per_page=100"` for the real origin/develop
    tip — **QUOTE the path** (zsh globs `?`). Failed logs: `gh run view --log-failed`.
- **Unpushed check**: `git log --oneline origin/develop..HEAD`, then
    `git diff --stat origin/develop..HEAD -- . ':!.claude'`. Empty = green CI covers HEAD.
    **NON-empty = it does NOT** — report the gap instead of calling CI green-for-HEAD (bit at 148).
- **ALWAYS `tail -5 .claude/context/iterations.jsonl`** — the ONLY place a crashed role shows up
    (also the `audit` role, every 10th iteration). **A non-OK status does NOT mean no work:
    corroborate with `git log` for the `cid(<role>):` commit** — infra crash vs benign overrun and
    the crashed-review fingerprint → `MEMORY-archive.md`.
- **ALWAYS `git status --porcelain` too.** A TIMEOUT role dies before committing but **leaves its
    written file dirty** — at 155 define-next timed out with 0 turns yet left a 440-line `next.md`
    carrying the iteration's most important finding. Treat as an unverified lead, not fact.
- **Counts** (159): crate/pkg READMEs & CLAUDE.md 12 each (**scope the glob to
    `crates/*/   packages/*/`** — bare grep gives 14); pytest-benchmark 18; UniFFI 32; docs pages
    **23**; `docs/howto/*.md` 11; speedups 1.3x-158x; ffi extern **47** (`'#\[unsafe(no_mangle)\]'`;
    bare `no_mangle` gives 48); iscc-lib `#[test]` **342** (glob
    `git ls-files 'crates/iscc-lib/**/*.rs'`; `src/` alone gives 288); `packages/go` `^func Test`
    **177**; CRAP `entries` **105**; pytest **393** via `uv run pytest --collect-only -q` (counts
    without running the suite).
- **The project venv is a ready-made conformance oracle**: `iscc_core` AND `iscc_lib` are both
    importable under `uv run python`, so any reference-vs-core probe needs **no build** — but check
    `crates/iscc-py/python/iscc_lib/_lowlevel.abi3.so` mtime first (stale `.so` silently measures
    the previous commit). Load `scripts/unicode_sweep.py` via `importlib` to probe its
    guards/`sweep()` directly in seconds instead of re-inventing a sweep — **since 158 the CLI
    refuses without `--rebuilt`**, so drive `main(["--rebuilt"])` in-process and `setattr`
    `scalar_values` / `EXPECTED_*` / `FUNCTION_PAIRS` to exercise the failure path at 30-scalar
    scale (a full run needs a `--release` rebuild, ~60 s + build).
- **version_sync TARGETS** = **21** (`scripts/version_sync.py --check`) = the authoritative set of
    version-synced files; absentees rot.
- **Issue headers**: `grep -nE '^## ' issues.md` (priority+source tag). **Trace a dep**:
    `cargo tree -i <crate> --target all`. **Tier 1 fns**: `grep -n "pub fn gen_" .../src/lib.rs`.
- **Specs live at `.claude/context/specs/`, NOT `specs/`** — `git diff -- specs/` returns empty for
    ANY commit and silently looks like "no spec change" (near-miss 144).

## Quality Gates — details in `quality-gates.md`, read it before reporting CI status

- **ENFORCING:** iai perf (>10% Ir, 16 baseline entries); coverage + CRAP (105 entries —
    `--fail-regression` is **CI-ONLY**, so a green `mise run check` proves nothing); cargo-deny;
    docs page-list parity (prek hook + a pytest in `python-test`); the `unicode-sweep` job (157).
- **Red with NO code change:** cargo-deny (live advisory DB). **Fails open:** the release.yml static
    gate (job log must show zero `warning: skipped`). **Informational:** cargo-semver-checks.

## Codebase Landmarks

- `crates/` — **8 crates** (lib, py, napi, wasm, ffi, jni, rb, uniffi), all 32/32 symbols;
    iscc-uniffi has 21 tests, `publish=false`. `packages/` layout → `MEMORY-archive.md`;
    `packages/go` is the ONLY binding not inheriting the core's Unicode behaviour.
- `ci.yml` — **21 YAML job entries → 22 jobs → 23 check names** (was 20/21/22 before 157 added
    `unicode-sweep`): `python-test` = 3.10/3.14 matrix, `python` (L72) is an `if: always()`
    AGGREGATOR; `push:` under `on:` is NOT a job. Enumerate with a PyYAML one-liner over
    `d['jobs']`. `release.yml` — 8 registry toggles; Swift XCFramework is a `prepare-release` step,
    NOT a toggle. **`specs/ci-cd.md`'s job table is DRIFTED** (14 rows vs 21 keys) — owned by an
    authorized issue.
- **Unicode = 16.0.0 + TWO freeze layers + the sweep gate → read `unicode-contract.md` before ANY
    Unicode call** (SENTINEL for category/normalization, the `Final_Sigma` CASE freeze from 156, the
    criterion-4 gate from 157; criterion status, fixture-plumbing table, 2 SUPERSEDED designs that
    must NOT be implemented). Inline landmark: **surefire's default CWD is the pom basedir**, NOT
    the `mvn -f` dir — why the Java suites' `../../iscc-lib/tests/…` resolve.
- **Adding a docs page = 3 hand edits** (`zensical.toml` nav, `ORDERED_PAGES`, `docs/llms.txt`), now
    gated. `streaming.rs`: `DataHasher`/`InstanceHasher` at crate root, `SumHasher` only via
    `streaming::`. iscc-wasm's `blake3 wasm32_simd` dep is feature-unification — **don't prune**.
    `iscc-py` has **12** `.detach(` sites. `benches/`: 12 criterion (0.7) + iai 0.16 (11 fns, 16
    cases). `crates/iscc-lib/src/utils/` holds TWO generated data modules: `unicode16.rs` (731
    unassigned ranges) and `unicode16_case.rs` (152 `Cased` + 452 `Case_Ignorable`). **Inline
    `# held:`** = pin rationale — root `Cargo.toml` **4**, `pyproject.toml` zero.
- **Ruff/prek/mdformat → `lint-tooling.md`.** ruff **0.16.0** since 137; local prek is a strict
    SUPERSET of CI. Probe hooks with `prek run <hook> --files <f>`.
- **Dependency-pin inventory + slice history** → `dep-refresh-survey.md`. All GHA refs CURRENT
    (140); release.yml keeps **97 `uses:`**, `rubygems/configure-rubygems-credentials@main` (L895)
    the ONLY unpinned one. No Dependabot/Renovate; **`rb_sys` pinned in THREE linked places**.

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward the rest, CI via check-runs API on the real tip. **Always diff
    `.claude/context/specs/`** (132, 147) and read issues.md *bodies* (145) — either flips met→unmet
    with ZERO code change, as can pure MEASUREMENT with an empty diff (156).
- **When a ruling lands, re-verify the CODE against the NEW spec**; grep `decisions.md` for
    `supersede`. At 147 a 14-iteration "met" went unmet.
- **Reproduce/refute inherited claims yourself, ideally by a DIFFERENT method** — cheap probes beat
    inherited text (bugs confirmed 147/156, stale-napi-`.node` refuted 151-153, tables re-derived
    157, both filed sweep blind spots reproduced 158). **A generator can never be its own oracle.**
    Verify numbers a PREDECESSOR flagged as unverified (157 cleared 155's 152/452). Deliberate
    deviations from a fix sketch are legitimate: read the in-source comment first.
- **Spec checkboxes are NOT a progress signal** — most specs sit at 0/N checked though MET; only
    `ci-cd.md` (44/52) + `rust-core.md`'s semver box are kept up. Spec *prose* rots too, and can be
    outright FALSIFIED by measurement (156: "Rust `str::to_lowercase()` does the same").
- **Cost-rank a propagation slice on FOUR axes** (153/154): fixture plumbing × target-API coverage ×
    **local buildability** (`command -v` — demotes Swift/C++ to CI-only proof) × **is the fixture a
    DECLARED INPUT of that build system?** Gradle/MSBuild/CMake can report UP-TO-DATE and skip the
    suite — 13 Kotlin tests silently did not run while every gate was green. Only a
    mutate-then-rerun probe (no `clean`) exposes it; CI is immune.

## Current State (assessed-at: 0b8f2e2, iter 159)

- **IN_PROGRESS — CI GREEN and covering all code.** `origin/develop` == `5581696`: **45 check-runs,
    23 names, 0 non-success**; HEAD `0b8f2e2` is 1 `.claude`-only commit ahead, code diff vs origin
    empty. PR **#44 (develop→main) OPEN** (v0.6.0 NOT shipped; version **0.5.0**).
- **Iteration 158 HARDENED the sweep gate; verdict PASS** (4 roles OK, 193 turns). 5 code files,
    zero Rust source / zero baseline: `unicode_sweep.py` (`--rebuilt` refusal + bounded samples),
    its tests 10 → **14**, `ci.yml` + `mise.toml` one line each, `docs/unicode.md`. Detail →
    `unicode-contract.md` crit 4.
- **Statuses:** Rust-core partially met — crit 1, 2, 4 MET; **crit 3 stuck at 8 of 11** surfaces (C
    FFI, C++, Swift ungated) + the pure-Go port. All else met except CI/CD (partial). Only count
    that moved at 158: pytest **393** (was 389); all others in Shortcuts held.
- **Propagation invariant:** `git ls-files -- '*data.json' '*unicode_boundary.json'` = **7** paths,
    matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py` (152 drift gate, rides
    `python-test`). Register new copies; keep canonical basenames (the gate discovers by basename).
- **Next = propagation slice 5 (C FFI)** — last surface buildable here (`gcc`; `cmake`/`swift`
    absent). Re-verified 159: `include/iscc.h` declares `iscc_text_clean` (L383) /
    `iscc_text_collapse` (L434), but `tests/test_iscc.c` (459 lines) has **zero** hits for either
    and no JSON reader; the `c-ffi` job is one bare `gcc -I include` + a cbindgen freshness check.
    Recommended: PEP 723 generator → `unicode_boundary_vectors.h` of `static const char *`, gated by
    regenerate-then-`git status` empty (the `gen_unicode16_*.py` pattern). A *generated* header is
    derived → must **NOT** go in `VENDORED_COPIES`. Do NOT extend `unicode_boundary.json` — nine
    suites assert exactly 7/5 counts.
- **`specs/rust-core.md` L149-157 is STALE** (Go `Final_Sigma` fixed 147; `str::to_lowercase` claim
    wrong twice over); crit-1/-3/-4 boxes unchecked though 1/2/4 are met. Human-owned — don't edit.
- **Issues: 8** (4 `normal`, 4 `low`, 0 critical, 0 `HUMAN REVIEW REQUESTED`); human backlog
    CLEARED. CLOSED at 158: "Harden the Unicode differential sweep gate"; the Unicode umbrella
    entry's remainder is now only **(b) propagation**. AUTHORIZED for CID: rubygems `@v2.1.0` pin
    (still `@main` at `release.yml:895`); major dep bumps **one per step** (magnus 0.8 / jni 0.22 =
    source rewrites; xunit 3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x); exhaustive
    `specs/ci-cd.md` job table. DEFERRED: npm OIDC.
- **Don't re-flag as DONE**: sweep-gate hardening 158, sweep gate 157, `Final_Sigma` 156, C#+Kotlin
    154, napi+Java 153, drift gate 152, WASM+Ruby 151, Python+Go 150 (≤149 → `MEMORY-archive.md`).

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat** aborts commits on rewrap edge cases (list → `lint-tooling.md`). Recipe:
    `cp f /tmp/c.md && uv run mdformat --wrap 100 --number /tmp/c.md && diff`, adopt, re-run once.
- **Absent toolchains (`cmake`/`swift`/`gradle`), invisible exec bits, case-sensitive binding-API
    greps, csbindgen/UniFFI/JNA side effects, the Go probe recipe → `env-gotchas.md`.**
