# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Topic files: `MEMORY-archive.md` (gate
internals, release internals, closed milestones), `counts.md` (artifact counts + how to reproduce
them), `dep-refresh-survey.md` (pin inventory), `unicode-contract.md`, `quality-gates.md`,
`lint-tooling.md`, `env-gotchas.md`. **Size budget: under 140 lines** — archive detail eagerly.

## Exploration Shortcuts

- **Authoritative CI status** (sandbox `gh run list` is STALE — old ancestor SHAs): `gh api` on
    `"repos/iscc/iscc-lib/commits/<tip-sha>/check-runs?per_page=100"` for the real origin/develop
    tip — **QUOTE the path** (zsh globs `?`). Failed logs: `gh run view --log-failed`.
- **Unpushed check**: `git log --oneline origin/develop..HEAD`, then
    `git diff --stat origin/develop..HEAD -- . ':!.claude'`. Empty = green CI covers HEAD.
    **NON-empty = it does NOT** — report the gap (bit at 148; fired for real at 162).
- **ALWAYS `tail -6 .claude/context/iterations.jsonl`** — the ONLY place a crashed role (or the
    every-10th `audit` role) shows up. **A non-OK status does NOT mean no work: corroborate with
    `git log`** (fingerprints → `MEMORY-archive.md`). Conversely **work with NO jsonl entry**:
    out-of-loop `cid(loop):` commits land between iterations — always diff them.
- **ALWAYS `git status --porcelain` too.** A TIMEOUT role dies before committing but **leaves its
    written file dirty** — at 155 define-next timed out with 0 turns yet left a 440-line `next.md`
    carrying the iteration's most important finding. Treat as an unverified lead, not fact.
- **Counts + the glob/grep trap for each → `counts.md`** (12 READMEs, 12 CLAUDE.md, 23 docs pages,
    342 iscc-lib `#[test]`, 441 pytest, 177 Go, 47 ffi externs, 105 CRAP, 21 version_sync targets, 8
    fixture copies; JVM annotations ≠ cases). Re-verified 161; check there before counting by hand.
- **YAML probes need `uv run python`** — the bare system `python3` has NO `yaml` module. Job-table
    parity is gated since 163; probe by hand only if the gate itself is suspect.
- **The project venv is a ready-made conformance oracle**: `iscc_core` AND `iscc_lib` are both
    importable under `uv run python`, so any reference-vs-core probe needs **no build** — but check
    `crates/iscc-py/python/iscc_lib/_lowlevel.abi3.so` mtime first (stale `.so` silently measures
    the previous commit). Sweep-probing recipe → `unicode-contract.md`.
- **Issue headers**: `grep -nE '^## ' issues.md` (priority+source tag). **Trace a dep**:
    `cargo tree -i <crate> --target all`. **Tier 1 fns**: `grep -n "pub fn gen_" .../src/lib.rs`.
- **Specs live at `.claude/context/specs/`, NOT `specs/`** — `git diff -- specs/` returns empty for
    ANY commit and silently looks like "no spec change" (near-miss 144).

## Quality Gates — details in `quality-gates.md`, read it before reporting CI status

- **ENFORCING:** iai perf (>10% Ir); coverage + CRAP (`--fail-regression` is **CI-ONLY**, so a green
    `mise run check` proves nothing); cargo-deny; docs page-list parity; `unicode-sweep` (157); CI
    job-table parity (163). The last three share one pattern: **prek hook + a pytest**, and the
    pytest is what makes it a CI gate (a hook's `files:` sees only changed paths, so row-only
    deletions slip past it).
- **Red with NO code change:** cargo-deny (live advisory DB). **Fails open:** the release.yml static
    gate (job log must show zero `warning: skipped`). **Informational:** cargo-semver-checks.
- NOT a gap to report: a "no floating branch ref" assertion in `check_release_workflow.py` is a NEW
    policy needing Titusz's sign-off (must still allow deliberate pointers like `@stable`).

## Codebase Landmarks

- `crates/` — **8 crates** (lib, py, napi, wasm, ffi, jni, rb, uniffi), all 32/32 symbols;
    iscc-uniffi has 21 tests, `publish=false`. `packages/` layout → `MEMORY-archive.md`;
    `packages/go` is the ONLY binding not inheriting the core's Unicode behaviour.
- `ci.yml` — **21 YAML job entries → 22 jobs → 23 check names**: `python-test` = 3.10/3.14 matrix,
    `python` (L72) is an `if: always()` AGGREGATOR; `push:` under `on:` is NOT a job. Enumerate with
    a PyYAML one-liner over `d['jobs']`. `release.yml` — 8 registry toggles; Swift XCFramework is a
    `prepare-release` step, NOT a toggle. **`specs/ci-cd.md`'s job table is EXHAUSTIVE since 163**
    (21 rows == 21 keys, gated); its prose hardcodes the counts and the gate pins none (≥10 floor),
    so a job addition reds the gate but the prose number needs a HAND edit.
- **Unicode = 16.0.0 + TWO freeze layers + the sweep gate → read `unicode-contract.md` before ANY
    Unicode call** (fixture-plumbing table, 2 SUPERSEDED designs that must NOT be implemented).
    Inline landmark: **surefire's default CWD is the pom basedir**, NOT the `mvn -f` dir — why the
    Java suites' `../../iscc-lib/tests/…` resolve.
- **Adding a docs page = 3 hand edits** (`zensical.toml` nav, `ORDERED_PAGES`, `docs/llms.txt`), now
    gated. `streaming.rs`: `DataHasher`/`InstanceHasher` at crate root, `SumHasher` only via
    `streaming::`. iscc-wasm's `blake3 wasm32_simd` dep is feature-unification — **don't prune**.
    `iscc-py` has **12** `.detach(` sites. `benches/`: 12 criterion (0.7) + iai 0.16 (11 fns, 16
    cases). `src/utils/` holds two generated data modules (→ `unicode-contract.md`). **Inline
    `# held:`** = pin rationale — root `Cargo.toml` **3** since 167 (criterion 0.8, jni 0.22, uniffi
    0.32; the magnus hold went with the bump), `pyproject.toml` zero.
- **Ruff/prek/mdformat → `lint-tooling.md`.** ruff **0.16.0** since 137; local prek is a strict
    SUPERSET of CI. Probe hooks with `prek run <hook> --files <f>`.
- **Dependency-pin inventory + slice history** → `dep-refresh-survey.md`. All GHA refs CURRENT
    (140); release.yml keeps **97 `uses:`**; since 162 **ZERO `@main` action refs** across
    ci/docs/release. No Dependabot/Renovate; **`rb_sys` pinned in THREE linked places**.

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward the rest, CI via check-runs API on the real tip. **Always diff
    `.claude/context/specs/`** (132, 147) and read issues.md *bodies* (145) — either flips met→unmet
    with ZERO code change, as can pure MEASUREMENT with an empty diff (156).
- **When a ruling lands, re-verify the CODE against the NEW spec**; grep `decisions.md` for
    `supersede`. At 147 a 14-iteration "met" went unmet.
- **Reproduce/refute inherited claims yourself, by a DIFFERENT method** — cheap probes beat
    inherited text (bugs confirmed 147/156; stale-napi-`.node`, "C++ unbuildable", "Swift not
    verifiable" all REFUTED). **A generator can never be its own oracle**; a suite printing N passes
    may still cover a SUBSET — compare against the PRE-change tree, not a floor (164).
- **Spec checkboxes are NOT a progress signal** — most specs sit at 0/N checked though MET; only
    `ci-cd.md` (44/52) + `rust-core.md`'s semver box are kept up. Spec *prose* rots too, and can be
    outright FALSIFIED by measurement (156: "Rust `str::to_lowercase()` does the same").
- **Ask whether a fixture is a DECLARED INPUT of the build system.** Gradle/MSBuild can report
    UP-TO-DATE and skip a suite (13 Kotlin tests silently did not run while every gate was green);
    only a mutate-then-rerun probe exposes it. All 12 suites now probed; CI is immune.
- **A "not verifiable in this container" claim is UNPROVEN, not true** — cmake, Kotlin, C++ and
    Swift each fell to a second look after the docs said otherwise → `env-gotchas.md`.

## Current State (assessed-at: 8ea3e02, iter 168)

- **IN_PROGRESS — CI green and it COVERS HEAD.** `origin/develop` == `90bde13` (167 review): **45
    check-runs, 23 names, 0 non-success**. HEAD `8ea3e02` is one `cid(log)` commit ahead with an
    EMPTY non-`.claude` diff. PR **#44 (develop→main) OPEN** (v0.6.0 NOT shipped; **0.5.0**).
- **THE UNICODE WORK IS FINISHED** (161, 11 of 11 surfaces); 163 closed the ci-cd job-table drift,
    164 the dotnet dep major, 165 the Kotlin Gradle wrapper 9.6.1, 166 JUnit 6.1.2 in both JVM
    manifests, **167 magnus 0.7 → 0.8** (lock 0.8.2; `magnus::exception::` + `RString::from_slice` →
    `ruby.exception_runtime_error()` / `ruby.str_from_slice()`; 33 `define_*` regs, tests
    untouched). Rust core met except `>= 1.0.0` (human-HELD); everything met except CI/CD (partial).
- **ONE CID-schedulable item left, full stop: `jni` 0.22** (`crates/iscc-jni/src/lib.rs`, ~41 sites
    — `JNIEnv` → `Env`/`EnvUnowned`, `GlobalRef` → `Global`, `AutoLocal` → `Auto`, closure-based
    attach, mandatory per-fn `ErrorPolicy`; upstream `docs/0.22-MIGRATION.md`). Everything else in
    the dep issue is human/release-gated (release.yml actions, uniffi 0.32, criterion 0.8).
- **Crate `CLAUDE.md` files rot after a dep migration** — 167 updated the "Magnus 0.7.1" line but
    left `RString::from_slice` in the patterns list (L108). Diff the crate's CLAUDE.md against the
    code after any binding-API bump; it is doc drift worth one line, not an issue.
- **Propagation invariant:** `git ls-files -- '*data.json' '*unicode_boundary.json'` = **8** paths,
    matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py` (152 drift gate, rides
    `python-test`). Register new copies; keep canonical basenames (the gate discovers by basename).
    Generated artifacts are derived and must stay OUT. A 13th vector now costs **12 suites**.
- **Loop infra (162):** `ARTIFACT_BUDGETS` in `tools/cid.py` caps **state 200** (next 120, handoff
    100, issues 300); `decisions.md` rotates into `decisions-archive.md` (**grep BOTH**); role
    context = `.claude/skills/cid-ctx-<role>/SKILL.md` packs.
- **`specs/rust-core.md` L149-157 is STALE** (Go `Final_Sigma` fixed 147; `str::to_lowercase` claim
    wrong twice over); criterion boxes unchecked though all four hold. Human-owned — don't edit.
- **Issues: 7** (2 `normal`, 5 `low`, 0 critical, 0 `HUMAN REVIEW REQUESTED`; ~140 lines — count
    headers, NOT priority tags: lines 3-4 are a legend that inflates a naive `grep -c`). **The issue
    BODY is the authority on what is left**, not the handoff's "Next" (the 166 handoff wrongly
    implied release.yml action bumps remain). go1.27 = standing tripwire, not schedulable; npm OIDC
    DEFERRED; `iscc-core#137` human-only.
- **Don't re-flag as DONE**: magnus 0.8 167, JUnit 6.1.2 166, Kotlin Gradle wrapper 165, dotnet
    xunit v3 164, ci-cd job table + parity gate 163, rubygems `@v2.1.0` pin 162, Swift vectors 161
    (≤160 → archive).

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat**: always run `uv run prek run mdformat --files <f>` right after writing (bare
    `uv run mdformat` is NOT the hook), then **grep for `` `…  …` `` (2+ spaces inside backticks)**
    — a code span straddling a line break gets joined with the indent whitespace and corrupts the
    path. Reword so each span fits on one line. Edge cases → `lint-tooling.md`.
- **Toolchain presence (and the `$PATH`-is-not-the-whole-story fallbacks), invisible exec bits,
    case-sensitive binding-API greps, csbindgen/UniFFI/JNA side effects, the Go probe recipe →
    `env-gotchas.md`.**
