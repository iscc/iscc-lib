# Review Agent Memory

Concise index — **one line per entry, detail belongs in a topic file.** `review-patterns.md` (docs,
verification, issues, gotchas, claim-probing, new-gate-script, fixture-oracle, index-gate),
`gate-reviews.md` (CI structure + Audit/Perf/Semver/CRAP), `binding-reviews.md` (per-binding
shortcuts, UniFFI/Kotlin/Ruby/Environment/PyO3/feature-flags, the 7 propagation slices — CLOSED at
12/12 suites iter 161), `dep-refresh-reviews.md` (v0.6.0 slices), `gha-workflow-reviews.md`
(release.yml gates, action-major bumps, pinning), `unicode-reviews.md` (freeze rule, sweep gate,
differential gate, boundary vectors, derived tables), `codex-integration.md` (how to weigh a
finding). Stale in `MEMORY-archive.md`.

## Quality Gate Details

- `mise run check` = the pre-commit hooks (hygiene, formatting, lint incl. Ruby, release-workflow
    static checks) — never assert the hook count, it drifts
- Pre-push hooks (clippy, cargo test, pytest, `ty`, ruff `S`/`C901`) are NOT in it — verify clippy
    with `cargo clippy --workspace --all-targets -- -D warnings`. Added files must be ≤256kb
- **Pre-push needs `iscc_lib` built** (`ty check`/`pytest` import it) — run
    `uv run maturin develop --release` in `crates/iscc-py` BEFORE `git push` → `binding-reviews.md`
- Two config exclusions that are proper scoping, NOT circumvention: `[tool.ty.src] exclude` for
    Python files importing non-venv packages (`conanfile.py` → `conan`), and
    `[tool.ruff.lint.per-file-ignores]` waiving `S101`/`S603`/`S607` under `tests/**`
- **After ANY step-9 edit to a `.py` file, re-run the PRE-PUSH gates** (`uv run ty check`, ruff
    `S`/`C901`) — not in `mise run check`, so a defect surfaces only at `git push` (157: attribute
    assignment on an importlib-loaded module; use `monkeypatch.setattr`)

## Common Issues

- Verification greps false-positive: `grep -qv 'pat'` is always true on multi-line files (use
    `! grep -q`). **Substring `grep -c` and exact counts in next.md are a recurring mis-spec class**
    (139) — test the criterion against `HEAD~1` before believing advance broke it
- **next.md's Implementation Notes are a HYPOTHESIS — algorithms and prose alike**: an algorithm
    that fails on real data (142), false docs claims (143), a mislabelled oracle (149). It may even
    forbid re-deriving, which makes advance blameless and review the only check. Re-derive every
    number, test spec, expected value and test count from its source (paths via `realpath`)
- **Probe claims, don't accept them** → `review-patterns.md` "Claim-Probing Recipes"
- `iscc_decompose` returns units WITHOUT "ISCC:" prefix — cross-check doc examples. **Docs site
    URL** is `https://lib.iscc.codes/`, NOT `https://iscc-lib.iscc.io/` (advance gets this wrong)
- **Advance agent idle claims**: verify remaining issue priorities independently — it may claim
    "only low-priority remain" when `normal` issues still exist
- Go `// indirect`, CI `find` cross-arch, prek stash conflict, and mode-only commits under
    `core.fileMode=false` → `review-patterns.md` "Gotchas"
- **Writing `\uXXXX` through Edit/Write decodes it to literal UTF-8** (149) — edit ASCII-escaped
    files via Python, assert `isascii()` + numeric `ord()`; `tests/test_vendored_fixtures.py` gates
    the canonical fixture. Same trap in **bash** (double quotes eat one backslash) — run such a
    criterion from a quoted heredoc before calling it failed (159) → `review-patterns.md`
- **`mise run check` mdformat reformats context files** (intermittent): a non-conforming `next.md` /
    `MEMORY.md` is NOT an advance regression, but pre-push mdformat WILL reject the batch — check
    `git status --porcelain` right after and STAGE it. Never stage `iterations.jsonl`; keep inline
    code spans on ONE line
- **next.md may task advance with an `issues.md` append (135) or REVIEW with a spec edit (144) —
    both protocols forbid it.** advance puts the paragraph in its handoff **Notes** for me to
    append; I may edit a spec only when resolving a `[human]` issue carrying `**Spec:**`
- **No-op / human-handoff iteration (111)**: verify scope is empty, still scan `@{upstream}..HEAD`.
    HUMAN-REVIEW spec amendments + `low` left (IDLE cond #2 NOT met) → flag **HUMAN REVIEW
    REQUESTED**, NOT `**IDLE**` (all-`low` only). Verdict still PASS; push the batch
- **Unicode freeze-rule work has its own playbook → `unicode-reviews.md`** — read it before any diff
    under `utils/unicode16*`, `unicode_boundary.json` or `scripts/gen_unicode16_*`: the `U+FFFF`
    sentinel contract, the ban on a bare `.to_lowercase()` in `text_collapse`, the
    `mise run unicode:sweep` gate (a bare script run REFUSES since 158), propagation **COMPLETE at
    161** (a 13th vector now costs 12 suites), "a binding can pass for the WRONG reason"
- **Concurrent CID loops (iter 97, detail → `MEMORY-archive.md`)**: spurious `mise run check` "files
    modified" on an untouched file + a mid-review working-tree change = a SECOND loop racing.
    Confirm with `ps aux`; flag HUMAN REVIEW REQUESTED, do NOT push or kill processes

## Review Shortcuts (see gate-reviews.md / binding-reviews.md for detail)

- **Rust-only**: `cargo test -p iscc-lib` + clippy workspace + `mise run check`. **Clippy
    workspace** is ~2s after a build — always run it
- **Test-fixture / vector-file only (141/149, ~5 min)**: Rust-only PLUS the full feature matrix
    (`#[cfg]` gating decides which tests run — assert the per-target `N passed` line), then the four
    fixture mutations → `review-patterns.md`. Test assets + docs are budget-free
- **Docs-only**: `mise run check` + clippy + `mise run version:check` (21 `OK:` lines) +
    `uv run zensical build` ("No issues found") + rendered-HTML grep for admonition/tab edits.
    **Green gates say NOTHING about truth** (143) — re-derive every number and treat
    "never/always/only" as claims to disprove → `review-patterns.md` "Docs Claim-Checking"
- **New docs PAGE (gated since 145)**: `uv run scripts/check_docs_nav.py` proves disk / nav /
    `ORDERED_PAGES` / `docs/llms.txt` agree (23 pages); holes: commented-out nav entries count as
    present, deletions skipped. `zensical build` **wipes `site/`** → run `gen_llms_full.py` after it
- **Python-only**: `mise run check` + `pytest` (+ the pre-push `ty check` / ruff `S` / `C901` gates
    when a new `.py` file lands)
- **New gate script or gate HARDENING (142–146, ~8 min)**: never accept "it exits 0 at HEAD" — write
    your OWN mutations, prefer a **real** regression to a synthetic typo, work the blind-spot list →
    `review-patterns.md`. A `files:`-scoped prek hook needs a **staged** fire/skip probe; it never
    sees deletions
- **DIFFERENTIAL gate (157/158) / vendored DERIVED-property table (156), ~25 min each**: a case-
    count pin catches a shrunken case set, never a swapped one; a behavioural generator is not its
    own oracle; drive `main()` in-process to test the reporting path → `unicode-reviews.md`
- **Repo-state gate that reads the git index (152, ~8 min)**: Python-only PLUS six mutations in a
    THROWAWAY repo (`git archive HEAD | tar -x -C /tmp/x && git init`; `git clone` fails here on
    `safe.directory`) — never mutate the real index mid-iteration → `review-patterns.md`
- **Lint-config-only (`[tool.ruff]*` / prek hook types, 134–139)**: `mise run check`, ruff check,
    ruff `format --check` (**assert exit 0, never a file count**), both pre-push ruff gates, ty
    check, pytest; `# noqa` and hook-mode probes → `dep-refresh-reviews.md` slice 8
- **Go-only**: `mise run check`, `CGO_ENABLED=0 mise exec -- go test -C packages/go -count=1 ./...`,
    `go vet -C packages/go ./...`, `mise exec -- gofmt -l packages/go` (empty)
- **Binding boundary-fixture slices (150–161) — CLOSED, all 12 suites gated.** Per-slice commands
    and mutations → `binding-reviews.md`. Standing traps: `cargo test -p iscc-wasm` = 0 tests; the
    **Gradle UP-TO-DATE stale green** (always ask "is the fixture a declared build input?" — all 12
    probed, Gradle was the only offender); a **generated** tracked artifact (C and C++ share ONE
    header) is verified by **decoding it back**, never by `render() == tracked`, and is NOT in
    `VENDORED_COPIES`; Swift's 12 tests ≠ 12 vectors and its `String ==` folds canonical equivalence
    (decisive probe: mutate the fixture **and** swap `scalars()` for `==` — green ⇒ load-bearing)
- **Every "not locally verifiable" toolchain claim has been FALSE so far** — `cmake` via
    `uv run --with cmake cmake …` (4.4.0, 160), `swift` via swift.org's Debian 12 tarball at
    `/tmp/swifttc/…/usr/bin` (161). Probe before repeating. The CMake build sets **no `-Wall`**, so
    back "no warning" with a manual `g++ -Wall -Wextra -Wpedantic -c`; prove a test-only include dir
    with an `add_subdirectory()` leak probe that MUST fail to compile → `binding-reviews.md`
- **Ruby-only / Kotlin-only / published-`.pyi`** command sets → `binding-reviews.md` "Per-binding
    review commands" (Gradle flakes on this bind mount; Kotlin consumer floor is **2.3 or newer**)
- **Config/lockfile-only**: `mise run check` + `cargo check -p <crate>` (+ `cargo deny check` if
    deps changed — see `gate-reviews.md` Audit)
- **Dependency refresh (v0.6.0)**: per-slice gates, hold-backs and the four reflexes →
    `dep-refresh-reviews.md`. **ALL nine slices CLOSED** (124–140); only human/major-gated bumps
    left. **A toolchain bump in a PUBLISHED binding is a support-policy change** — check the
    consumer floor. A DATA-TABLE dep needs an exhaustive differential: all 50 vendored vectors
    predate Unicode 16
- **Prek-hook-scope review (138–139)**: NEVER accept `git ls-files` arithmetic as a hook's surface —
    `.pyi` is tagged `pyi`, not `python`. Probing a *widened* tag needs a **staged, deliberately
    dirty** file → `review-patterns.md`
- **Core text/codec change + generated data (133/148, ≈12 min)**: Rust-only PLUS the **full feature
    matrix** PLUS the two CI-only gates — `mise run coverage` then the CI-exact
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    (BARE flag; CI uses BOTH) and `mise run bench:iai:check`; re-run any generator and assert
    `git status --porcelain <output>` is empty, plus the sequence differential in
    `review-patterns.md`
- **Version sync**: + `version_sync.py --check`. **Shell script**: + `bash -n <script>`
- **release.yml / any GHA action bump**: NEVER exercised by CID pushes → static-verify only. Run the
    committed gates `uv run scripts/check_release_workflow.py` and `… --check-action-inputs`, never
    a retyped heredoc; **zero `warning: skipped` lines is part of the pass**. Manual: actionlint,
    `runs.using`, majors' *defaults* → **`gha-workflow-reviews.md`**. `always()` = NEEDS_WORK
- **CI/Audit/Perf/Semver/CRAP gates** → `gate-reviews.md`

## Codex Review Integration

- Advisory only, never sets the verdict; `--commit HEAD`; empty/unavailable ≠ clean. Strengths,
    blind spots, dismiss-list, the 3-step **how to weigh a finding** → **`codex-integration.md`**.
    Expect a real finding wherever a green run proves less than it appears to: matching/parsing
    rules, exception contracts, user-facing factual claims, build-config lines (154 Gradle)
