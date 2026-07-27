# Review Agent Memory

Concise index — **one line per entry, detail belongs in a topic file.** `review-patterns.md` (docs /
verification / issues / gotchas + claim-probing + new-gate-script + fixture-oracle + index-gate
recipes), `gate-reviews.md` (CI structure + Audit/Perf/Semver/CRAP gates), `binding-reviews.md`
(per-binding shortcuts + UniFFI/Kotlin/Ruby/Environment/PyO3/feature-flags + propagation slices),
`dep-refresh-reviews.md` (v0.6.0 slice recipes), `gha-workflow-reviews.md` (release.yml gates +
action-major bumps + pinning), `unicode-reviews.md` (freeze-rule contract + sweep gate +
differential-gate + boundary-vector + derived-table recipes), `codex-integration.md` (second-opinion
strengths / blind spots / how to weigh a finding). Stale in `MEMORY-archive.md`.

## Quality Gate Details

- `mise run check` = the pre-commit hooks (hygiene, formatting, lint incl. Ruby, release-workflow
    static checks) — never assert the hook count, it drifts
- Pre-push hooks (clippy, cargo test, pytest, `ty`, ruff `S`/`C901`) are NOT in it — verify clippy
    with `cargo clippy --workspace --all-targets -- -D warnings`
- Java `mvn test`; Go `mise exec -- go test -C packages/go ./...` (`-C`, never `cd`); files ≤256kb
- **`ty check` external Python files**: files importing packages not in the venv (e.g.
    `conanfile.py` → `conan`) fail. Fix: `[tool.ty.src] exclude` — proper scoping, not circumvention
- **Pre-push needs `iscc_lib` built** (`ty check`/`pytest` import it):
    `cd crates/iscc-py && uv run maturin develop --release` — BEFORE `git push`, not after the hook
    fires → `binding-reviews.md` Environment
- `[tool.ruff.lint.per-file-ignores]` waives `S101`/`S603`/`S607` for `tests/**` — a test shelling
    out to `git` with no `# noqa` is correct, not an ungated suppression
- **After ANY step-9 edit to a `.py` file, re-run the PRE-PUSH gates** (`uv run ty check`, ruff
    `S`/`C901`), not just `mise run check` — they are not in it, so a defect surfaces only at
    `git push` (157: my own added test used `us.ATTR = x` on an importlib-loaded module → `ty`
    `unresolved-attribute on type ModuleType`; use `monkeypatch.setattr(mod, "ATTR", x)`)

## Common Issues

- Verification greps may false-positive — check match specificity. `grep -qv 'pat'` is always true
    on multi-line files; use `! grep -q 'pat'`. **Substring `grep -c` and exact counts in next.md
    are a recurring mis-spec class** (iter 139): test the criterion against `HEAD~1` before
    believing advance broke it — a mis-specified criterion advance *corrected* is scope discipline
- **next.md's Implementation Notes are a HYPOTHESIS — algorithms and prose alike.** It can prescribe
    an algorithm that fails on real data (142), dictate false docs claims (143), or mislabel a
    rejected-value oracle (149), and may forbid re-deriving — which makes advance blameless and
    review the only check. Re-derive every number from its source; test specs, expected values and
    test counts included (check relative paths with `realpath`)
- **Probe claims, don't accept them** → `review-patterns.md` "Claim-Probing Recipes"
- `iscc_decompose` returns units WITHOUT "ISCC:" prefix — cross-check doc examples. **Docs site
    URL** is `https://lib.iscc.codes/`, NOT `https://iscc-lib.iscc.io/` (advance gets this wrong)
- **Advance agent idle claims**: verify remaining issue priorities independently — it may claim
    "only low-priority remain" when `normal` issues still exist
- Go `// indirect`, CI `find` cross-arch, prek stash conflict, and mode-only commits under
    `core.fileMode=false` → `review-patterns.md` "Gotchas"
- **Writing `\uXXXX` through Edit/Write decodes it to literal UTF-8** (iter 149, bit both advance
    and review) — edit ASCII-escaped files via Python and assert `isascii()` + numeric `ord()` →
    `review-patterns.md` "Escape-decoding trap". `tests/test_vendored_fixtures.py` gates the
    canonical `unicode_boundary.json` for this
- **`mise run check` mdformat on context files** (intermittent): define-next may write `next.md` /
    `MEMORY.md` non-conforming, so `prek --all-files` reformats them (NOT an advance regression) and
    pre-push mdformat WILL reject the batch — `git status --porcelain` right after `mise run check`
    and STAGE any reformat. Never stage `iterations.jsonl`. Keep inline code spans on ONE line
- **next.md may task advance with an `issues.md` append (135) or REVIEW with a spec edit (144) —
    both protocols forbid it.** advance puts the paragraph in its handoff **Notes** for me to
    append; I may edit a spec only when resolving a `[human]` issue carrying `**Spec:**`
- **No-op / human-handoff iteration (111)**: verify scope is empty, still scan `@{upstream}..HEAD`
    for circumvention. HUMAN-REVIEW spec amendments + `low` left (IDLE cond #2 NOT met) → flag
    **HUMAN REVIEW REQUESTED**, NOT `**IDLE**` (all-`low` only). Verdict still PASS; push the batch
- **Unicode freeze-rule work has its own playbook → `unicode-reviews.md`**: the `U+FFFF` sentinel
    contract (delete-filter and category-override both RULED OUT), the ban on a bare
    `.to_lowercase()` in `text_collapse`, the `mise run unicode:sweep` gate (a bare script run
    REFUSES since 158 — `--rebuilt` is a *trusted* caller assertion), boundary-vector propagation
    (**8 of 11** bindings gated; C FFI / C++ / Swift left — the C FFI does export
    `iscc_text_clean`/`iscc_text_collapse`, so slice 5 is feasible), and "a binding can pass for the
    WRONG reason — check the MECHANISM". Read it before reviewing any diff under `utils/unicode16*`,
    `unicode_boundary.json` or `scripts/gen_unicode16_*`
- **Concurrent CID loops (iter 97, detail → `MEMORY-archive.md`)**: spurious `mise run check` "files
    modified" on an untouched file + a mid-review working-tree change = a SECOND loop racing.
    Confirm with `ps aux`; flag HUMAN REVIEW REQUESTED, do NOT push or kill processes

## Review Shortcuts (see gate-reviews.md / binding-reviews.md for detail)

- **Rust-only**: `cargo test -p iscc-lib` + clippy workspace + `mise run check`. **Clippy
    workspace** is ~2s after a build — always run it
- **Test-fixture / vector-file only (141/149, ~5 min)**: Rust-only PLUS the full feature matrix
    (`--no-default-features`, `+text-processing`, `--all-features`) — `#[cfg]` gating decides which
    tests run, so assert the per-target `N passed` line; then the four fixture mutations →
    `review-patterns.md`. Test assets + docs are budget-free
- **Docs-only**: `mise run check` + clippy + `mise run version:check` (21 `OK:` lines) +
    `uv run zensical build` ("No issues found") + rendered-HTML grep for admonition/tab edits.
    **Green gates say NOTHING about truth** (143) — re-derive every number and treat
    "never/always/only" as claims to disprove → `review-patterns.md` "Docs Claim-Checking"
- **New docs PAGE (gated since 145)**: `uv run scripts/check_docs_nav.py` proves disk / nav /
    `ORDERED_PAGES` / `docs/llms.txt` agree (23 pages). Two holes: a **commented-out** nav entry
    counts as present, and page **deletions** are skipped. `zensical build` **wipes `site/`** → run
    `gen_llms_full.py` after it → `review-patterns.md`
- **Python-only**: `mise run check` + `pytest` (+ the pre-push `ty check` / ruff `S` / `C901` gates
    when a new `.py` file lands)
- **New gate script or gate HARDENING (142–146, ~8 min)**: never accept "it exits 0 at HEAD" — write
    your OWN mutations, prefer a **real** regression to a synthetic typo, and work the blind-spot
    list → `review-patterns.md`. A `files:`-scoped prek hook needs a **staged** fire/skip probe and
    never sees deletions
- **DIFFERENTIAL gate (157/158) / vendored DERIVED-property table (156), ~25 min each**: a
    case-count pin catches a shrunken case set, never a swapped one; a behaviourally-derived
    generator is not its own oracle; a *unit* test on a bounded result proves the cap but never the
    **reporting** path — drive `main()` in-process for that. Both recipes → `unicode-reviews.md`
- **Repo-state gate that reads the git index (152, ~8 min)**: Python-only PLUS six mutations in a
    THROWAWAY repo (`git archive HEAD | tar -x -C /tmp/x && git init` — `git clone` fails here on
    `safe.directory`; never mutate the real index mid-iteration) → `review-patterns.md`
- **Lint-config-only (`[tool.ruff]*` / prek hook types, 134–139)**: `mise run check`, ruff check,
    ruff `format --check` (**assert exit 0, never a file count**), both pre-push ruff gates, ty
    check, pytest; `# noqa` and hook-mode probes → `dep-refresh-reviews.md` slice 8
- **Go-only**: `mise run check`, `CGO_ENABLED=0 mise exec -- go test -C packages/go -count=1 ./...`,
    `go vet -C packages/go ./...`, `mise exec -- gofmt -l packages/go` (empty)
- **Binding boundary-fixture slices** (150/151/153/154) → `binding-reviews.md`, incl. the
    `cargo test -p iscc-wasm` = 0-tests trap and the **Gradle UP-TO-DATE stale-green trap** — always
    ask "is the fixture a declared build input?" for SwiftPM / CMake / C-FFI next
- **Ruby-only / Kotlin-only / published-`.pyi`** command sets → `binding-reviews.md` "Per-binding
    review commands" (Gradle flakes on this bind mount; Kotlin consumer floor is **2.3 or newer**)
- **Config/lockfile-only**: `mise run check` + `cargo check -p <crate>` (+ `cargo deny check` if
    deps changed — see `gate-reviews.md` Audit)
- **Dependency refresh (v0.6.0)**: per-slice gates, hold-back recipes and the four reflexes →
    `dep-refresh-reviews.md`. **ALL nine slices CLOSED** (124–140); only human/major-gated bumps
    left. **A toolchain bump in a PUBLISHED binding is a support-policy change, not a pin** — check
    the consumer floor first. A DATA-TABLE dep needs an exhaustive differential (the 50 vendored
    vectors are all Unicode ≤ 15)
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
    `runs.using`, intervening majors' *default* changes → **`gha-workflow-reviews.md`**. `always()`
    = NEEDS_WORK
- **CI/Audit/Perf/Semver/CRAP gates** → `gate-reviews.md`

## Codex Review Integration

- Advisory only, never sets the verdict; `--commit HEAD`; empty/unavailable ≠ clean. Strengths,
    blind spots, the standing dismiss-list and the 3-step **how to weigh a finding** recipe →
    **`codex-integration.md`**. Expect a real finding wherever a green run proves less than it
    appears to: matching/parsing rules, exception-handling contracts, user-facing factual claims,
    build-config lines (154 — the Gradle stale green no surface gate could see)
