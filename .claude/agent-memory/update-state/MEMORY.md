# Update-State Agent Memory

Codepaths, patterns, key findings across CID iterations. Topic files: `MEMORY-archive.md` (gate
internals, release internals, closed milestones), `counts.md` (artifact counts + how to reproduce
them), `dep-refresh-survey.md` (pin inventory), `unicode-contract.md`, `quality-gates.md`,
`lint-tooling.md`, `env-gotchas.md`. **Size budget: under 150 lines** — archive detail eagerly.

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
- **Audit cadence is NOT reliable**: jsonl entries at 130/140/150/160 but **none at 170**. Never
    park a finding "for the next audit". It commits only `cid(audit): metrics snapshot`.
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
    gate (job log must show zero `warning: skipped`). A "no floating branch ref" assertion in
    `check_release_workflow.py` is NOT a gap — it is new policy.
- **cargo-semver-checks IS an ENFORCING red check-run** (exit 100, non-success conclusion), NOT
    "informational" — corrected at 176. It diffs the crate's WHOLE public API vs the LAST RELEASE
    (0.5.0), so a Tier-2 change still trips it: marking public `enum Version` `#[non_exhaustive]`
    fired `enum_marked_non_exhaustive` "semver requires new major version". `mise run check` does
    NOT run it → a green local check proves nothing about semver.

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
    prune**. `iscc-py` has **12** `.detach(` sites. Benches live in **`crates/iscc-lib/benches/`**
    (NOT a root `benches/`): 12 criterion fns (0.8.2) + iai 0.16 (11 fns, 16 cases). `src/utils/`
    holds two generated data modules (→ `unicode-contract.md`). Root `Cargo.toml` has **zero
    `# held:` and zero `authorized …` comments since 172** — grep BOTH before claiming a hold.
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
    UP-TO-DATE and skip a suite; only mutate-then-rerun exposes it. All 12 probed, CI immune.
- **A "not verifiable in this container" claim is UNPROVEN, not true** (cmake, Kotlin, C++, Swift
    all fell to a second look) → `env-gotchas.md`.

## Current State (assessed-at: dcc45a2, iter 176)

- **CI IS RED on develop.** The iters 172–175 IDv1-decode batch was PUSHED at 175 (origin/develop =
    HEAD-code tip `3cdb00a`); CI ran on it for the FIRST time and **failed 2 enforcing gates**
    (check-runs: 45/23names/**4 failures = 2 job types**): (1) **Semver** — `enum Version` now
    `#[non_exhaustive]` (codec.rs:103) trips `enum_marked_non_exhaustive`, exit 100; (2) **CRAP** —
    `decode_header` CRAP +4.0 (12→16, CC 16, 100% covered), `--fail-regression` rejects the rise vs
    `.crap-baseline.json`. NEITHER visible to local `mise run check`. This is the classic "review's
    green local check ≠ green CI" miss. TOP priority = get CI green.
- **HEAD `dcc45a2` = cid(log) only; its code == origin/develop**
    (`git diff origin/develop..HEAD --   . ':!.claude'` empty), so the red DOES cover HEAD — no
    unpushed-code gap this time.
- **175 fixed the critical `decode_header` truncation** (range-checked `u8::try_from`, reviewed
    PASS, issue deleted). But the SAME fix is what raised `decode_header` CC → the CRAP red.
- **`gen_iscc_id_v1` minting STILL exists NOWHERE** (only comments). Go rename NOT done —
    `EncodeIsccID`/`DecodeIsccID`/`IsccIDv1Result` still in `packages/go/iscc_id.go`, no
    `GenIsccIDV1`. 32/33 Tier-1 symbols.
- **Issues 12→11: 0 critical, 4 normal, 7 low** (resolved critical deleted). The new CI redness is
    NOT yet an issue.
- **SCOPE CONTEXT (still live from 174):** out-of-loop non-`cid()` commit `2c4e487` rewrote
    `target.md` + EVERY binding spec to demand **33 Tier 1 symbols** (`gen_iscc_id_v1`) +
    experimental ISCC-IDv1 on core + all 11 surfaces. Lesson: a non-`cid()` commit CAN carry both
    target AND code.

## Durable Facts (carried, not per-iteration)

- **`target.md` carries a "Current Release Milestone — v0.6.0" section** — always diff `target.md`.
    Unicode work FINISHED (161, 11/11 surfaces). Dep refresh DONE (172, uniffi 0.32 + criterion 0.8;
    root `Cargo.toml` has **zero `# held:`/`authorized …`** comments). `iscc-uniffi/Cargo.toml`
    declares `rust-version = "1.91"` (real floor); other 7 crates inherit workspace 1.85. PR **#44
    develop→main OPEN**, version **0.5.0**.
- **Review policy widened at 171:** `review` may correct a *mechanically checkable fact* (version,
    path, count) in a sub-spec under `.claude/context/specs/` with NO escalation; `target.md` +
    rationale/criteria/scope still need the human. Stale spec FACTS = report as actionable.
- **Every ecosystem has a lockfile** (170: .NET `packages.lock.json`, `--locked-mode`; a .NET bump
    needs `--force-evaluate` same commit). Propagation invariant:
    `git ls-files -- '*data.json'   '*unicode_boundary.json'` = **8** == `VENDORED_COPIES` in
    `tests/test_vendored_fixtures.py`.
- **Loop infra (162):** `ARTIFACT_BUDGETS` in `tools/cid.py` caps **state 200**; `decisions.md`
    rotates into `decisions-archive.md` (**grep BOTH**) — dirty `decisions*.md` = runner, not crash.
- **Issue count: never carry forward** — re-grep `^## ` headers every iteration (lines 3-4 are a
    legend that inflates a naive `grep -c`); count held 8 over 167-173 while composition flipped.
- **Don't re-flag as DONE**: uniffi 0.32 172, criterion 0.8 171, .NET lockfile 170, JNI 169, jni
    0.22 168, magnus 0.8 167, JUnit 166, Gradle 165 (≤164 → archive).

## Gotchas

- **state.md Write** = permission error → only `cat > file << 'EOF' ... EOF` via Bash works.
- **mdformat**: always run `uv run prek run mdformat --files <f>` right after writing (bare
    `uv run mdformat` is NOT the hook), then **grep for `` `…  …` `` (2+ spaces inside backticks)**
    — a code span straddling a line break gets joined with the indent whitespace and corrupts the
    path. **Also grep for `\`** — a `)` that reflows to the start of a continuation line becomes
    `172\)` (escaped ordered-list marker). Reword so no span/paren straddles a break. Edge cases →
    `lint-tooling.md`.
- **Toolchain presence (and the `$PATH`-is-not-the-whole-story fallbacks), invisible exec bits,
    case-sensitive binding-API greps, csbindgen/UniFFI/JNA side effects, the Go probe recipe →
    `env-gotchas.md`.**
