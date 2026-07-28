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
    `audit` role) shows up. **A non-OK status does NOT mean no work: corroborate with `git log`**
    (fingerprints → `MEMORY-archive.md`). Conversely **work with NO jsonl entry**: out-of-loop
    `cid(loop):` commits land between iterations — always diff them.
- **Audit cadence is NOT reliable**: jsonl entries at **130/140/150/160** but **none at 170**
    (iteration verdict IDLE). Never park a finding "for the next audit" — check the jsonl, and if
    the audit skipped, the finding is still unfiled. It commits only `cid(audit): metrics snapshot`.
- **ALWAYS `git status --porcelain` too.** A TIMEOUT role dies before committing but **leaves its
    written file dirty** (155: a 440-line `next.md`). Treat as an unverified lead, not fact.
- **Counts + the glob/grep trap for each → `counts.md`** (12 READMEs, 12 CLAUDE.md, 23 docs pages,
    342 iscc-lib `#[test]`, 441 pytest, 177 Go, 47 ffi externs, 105 CRAP, 21 version_sync targets, 8
    fixture copies). Check there before counting by hand.
- **YAML probes need `uv run python`** — the bare system `python3` has NO `yaml` module. Job-table
    parity is gated since 163; probe by hand only if the gate itself is suspect.
- **The project venv is a ready-made conformance oracle**: `iscc_core` AND `iscc_lib` both import
    under `uv run python`, so a reference-vs-core probe needs **no build** — but check
    `crates/iscc-py/python/iscc_lib/_lowlevel.abi3.so` mtime first (a stale `.so` silently measures
    the previous commit). Sweep recipe → `unicode-contract.md`.
- **Issue headers**: `grep -nE '^## ' issues.md`. **Trace a dep**: `cargo tree -i <crate>`.
- **Specs live at `.claude/context/specs/`, NOT `specs/`** — `git diff -- specs/` returns empty for
    ANY commit and silently looks like "no spec change" (near-miss 144).

## Quality Gates — details in `quality-gates.md`, read it before reporting CI status

- **ENFORCING:** iai perf (>10% Ir); coverage + CRAP (`--fail-regression` is **CI-ONLY**, so a green
    `mise run check` proves nothing); cargo-deny; docs page-list parity; `unicode-sweep` (157); CI
    job-table parity (163). The last three are **prek hook + a pytest**, and the pytest is what
    makes it a CI gate (a hook's `files:` sees only changed paths).
- **Red with NO code change:** cargo-deny (live advisory DB). **Fails open:** the release.yml static
    gate (job log must show zero `warning: skipped`). **Informational:** cargo-semver-checks. A "no
    floating branch ref" assertion in `check_release_workflow.py` is NOT a gap — it is new policy.

## Codebase Landmarks

- `crates/` — **8 crates** (lib, py, napi, wasm, ffi, jni, rb, uniffi), all 32/32 symbols;
    iscc-uniffi has 21 tests, `publish=false`. `packages/` layout → `MEMORY-archive.md`;
    `packages/go` is the ONLY binding not inheriting the core's Unicode behaviour.
- `ci.yml` — **21 YAML job entries → 22 jobs → 23 check names**: `python-test` = 3.10/3.14 matrix,
    `python` (L72) is an `if: always()` AGGREGATOR; `push:` under `on:` is NOT a job. `release.yml`
    — 8 registry toggles; Swift XCFramework is a `prepare-release` step, NOT a toggle.
    **`specs/ci-cd.md`'s job table is EXHAUSTIVE since 163** (21 rows == 21 keys, gated), but its
    prose counts are unpinned (≥10 floor) and need a HAND edit when a job is added.
- **Unicode = 16.0.0 + TWO freeze layers + the sweep gate → read `unicode-contract.md` before ANY
    Unicode call** (fixture-plumbing table, 2 SUPERSEDED designs; surefire CWD landmark).
- **Adding a docs page = 3 hand edits** (`zensical.toml` nav, `ORDERED_PAGES`, `docs/llms.txt`), now
    gated. `streaming.rs`: `DataHasher`/`InstanceHasher` at crate root, `SumHasher` via
    `streaming::` only. iscc-wasm's `blake3 wasm32_simd` dep is feature-unification — **don't
    prune**. `iscc-py` has **12** `.detach(` sites. `benches/`: 12 criterion (0.7) + iai 0.16 (11
    fns, 16 cases). `src/utils/` holds two generated data modules (→ `unicode-contract.md`). Pin
    rationale in root `Cargo.toml`: **zero `# held:` since 171**, 2 `authorized …` comments instead
    — grep BOTH.
- **Ruff/prek/mdformat → `lint-tooling.md`.** ruff **0.16.0** since 137; local prek is a strict
    SUPERSET of CI. Probe hooks with `prek run <hook> --files <f>`.
- **Dependency-pin inventory + slice history** → `dep-refresh-survey.md`. All GHA refs CURRENT
    (re-probed 171); release.yml keeps **97 `uses:`**, zero `@main`. No Dependabot/Renovate;
    **`rb_sys` pinned in THREE linked places**.

## Recurring Patterns

- **Incremental review**: assessed-at vs HEAD `--stat` first, re-verify only affected sections,
    carry forward the rest, CI via check-runs API on the real tip. **Always diff
    `.claude/context/specs/`** (132, 147) and read issues.md *bodies* (145) — either flips met→unmet
    with ZERO code change, as can pure MEASUREMENT with an empty diff (156).
- **When a ruling lands, re-verify the CODE against the NEW spec**; grep `decisions.md` for
    `supersede`. At 147 a 14-iteration "met" went unmet.
- **Reproduce/refute inherited claims by a DIFFERENT method** (bugs confirmed 147/156; three
    "unbuildable" claims REFUTED). **A generator is never its own oracle**; a suite printing N
    passes may cover a SUBSET.
- **Spec checkboxes are NOT a progress signal** — most sit at 0/N though MET; spec *prose* rots.
- **Ask whether a fixture is a DECLARED INPUT of the build system.** Gradle/MSBuild can report
    UP-TO-DATE and skip a suite; only a mutate-then-rerun probe exposes it. All 12 probed, CI
    immune.
- **A "not verifiable in this container" claim is UNPROVEN, not true** (cmake, Kotlin, C++, Swift
    all fell to a second look) → `env-gotchas.md`.

## Current State (assessed-at: 1b7b792, iter 171)

- **IN_PROGRESS — CI green.** `origin/develop` == `6ee27dc` (170 review): **45 check-runs, 23 names,
    0 non-success**. HEAD `1b7b792` is 2 commits ahead; the only non-`.claude` change is a
    **comment-only** root `Cargo.toml` edit, so green still covers all code. Tree CLEAN. PR **#44
    (develop→main) OPEN** (v0.6.0 NOT shipped; **0.5.0**).
- **HEAD is a human `cid(loop):` commit (2026-07-28) that reset the goal.** `target.md` gained a
    **"Current Release Milestone — v0.6.0"** section with explicit ready-for-release criteria; both
    dep majors AUTHORIZED; `specs/java-bindings.md` drift fixed by hand. Always diff `target.md` — a
    milestone section can appear with no code change.
- **THE UNICODE WORK IS FINISHED** (161, 11 of 11 surfaces); slice history 163-170 →
    `dep-refresh-survey.md`. Rust core met except `>= 1.0.0`; all sections met except CI/CD.
- **DEP REFRESH = THE LAST v0.6.0 CRITERION**, 2 items, both authorized, neither landed
    (`Cargo.lock` still criterion **0.7.0** / uniffi **0.31.2**): criterion 0.8 keeps
    `rust-version = "1.85"` (dev-dep only); uniffi 0.32 must be a PURE REGENERATION of the two
    checked-in bindings — Kotlin verifies locally, **Swift only via the `swift` CI job**. The
    `release.yml` action bullet is GONE (re-probed no-op 2026-07-28, all 25 `uses:` current).
- **Both doc-drift surfaces are CLOSED** (Ruby 170, Java spec by the human) — but keep diffing a
    crate's CLAUDE.md + spec after any dep bump; that rot recurs.
- **170 closed the .NET lockfile gap** (the audit never fired): `packages.lock.json` tracked, 3
    exact pins, `dotnet` job = `--locked-mode` restore + `--no-restore` build/test, job COUNT
    unchanged. Every ecosystem now has a lockfile; a .NET bump needs `--force-evaluate` same commit.
- **Review policy widened at 171:** `review` may correct a *mechanically checkable fact* (version,
    path, file list, count) in a sub-spec under `.claude/context/specs/` with NO escalation;
    `target.md` and rationale/criteria/scope still need the human. So stale spec FACTS are now
    CID-doable — report them as actionable, not human-gated.
- **Propagation invariant:** `git ls-files -- '*data.json' '*unicode_boundary.json'` = **8** paths,
    matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py` (152 drift gate, rides
    `python-test`); gate discovers by basename, generated artifacts stay OUT, a 13th vector costs 12
    suites.
- **Loop infra (162):** `ARTIFACT_BUDGETS` in `tools/cid.py` caps **state 200**; `decisions.md`
    rotates into `decisions-archive.md` (**grep BOTH**) — dirty `decisions*.md` = runner, not crash.
- **`specs/rust-core.md` L149-157 is STALE** (Go `Final_Sigma` fixed 147; `str::to_lowercase` claim
    wrong twice over); criterion boxes unchecked though all four hold. Under the 171 policy the
    factual half is now correctable by `review` — nobody has filed it.
- **Issues: 8** (2 `normal`, 6 `low`, 0 critical, **zero HUMAN REVIEW REQUESTED**; 178 lines — count
    headers, NOT priority tags: lines 3-4 are a legend that inflates a naive `grep -c`). New at 171:
    "declared MSRV asserted but never verified" (`low`, `[human]`, v1.0.0 prerequisite — CID must
    not act unprompted). go1.27 is its OWN entry (tripwire, upstream at rc2); npm OIDC DEFERRED.
    Never carry an issue count forward — re-grep every iteration (7→10→8→8 over 167-171).
- **Don't re-flag as DONE**: .NET lockfile 170, JNI cleanup 169, jni 0.22 168, magnus 0.8 167, JUnit
    6.1.2 166, Gradle 9.6.1 165, dotnet xunit v3 164, ci-cd job table 163 (≤162 → archive).

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat**: always run `uv run prek run mdformat --files <f>` right after writing (bare
    `uv run mdformat` is NOT the hook), then **grep for `` `…  …` `` (2+ spaces inside backticks)**
    — a code span straddling a line break gets joined with the indent whitespace and corrupts the
    path. Reword so each span fits on one line. Edge cases → `lint-tooling.md`.
- **Toolchain presence (and the `$PATH`-is-not-the-whole-story fallbacks), invisible exec bits,
    case-sensitive binding-API greps, csbindgen/UniFFI/JNA side effects, the Go probe recipe →
    `env-gotchas.md`.**
