# Review Agent Memory

Concise index — **one line per entry, detail belongs in a topic file.** `review-patterns.md` (docs /
verification / issues / gotchas + claim-probing + new-gate-script + fixture-oracle + index-gate
recipes), `gate-reviews.md` (CI structure + Audit/Perf/Semver/CRAP gates), `binding-reviews.md`
(per-binding shortcuts + UniFFI/Kotlin/Ruby/Environment/PyO3/feature-flags + propagation slices),
`dep-refresh-reviews.md` (v0.6.0 slice recipes), `gha-workflow-reviews.md` (release.yml gates +
action-major bumps + pinning), `codex-integration.md` (second-opinion strengths/blind spots). Stale
detail in `MEMORY-archive.md`.

## Quality Gate Details

- `mise run check` = the pre-commit hooks (hygiene, formatting, lint incl. Ruby, release-workflow
    static checks) — never assert the hook count, it drifts
- Pre-push hooks (clippy, cargo test, pytest, `ty`, ruff `S`/`C901`) are NOT in it — verify clippy
    with `cargo clippy --workspace --all-targets -- -D warnings`
- Java tests: `mvn test`. Go tests: `mise exec -- go test -C packages/go ./...` (`-C`, never `cd`)
- `check-added-large-files` threshold is `--maxkb=256`
- **`ty check` external Python files**: files importing packages not in the venv (e.g.
    `conanfile.py` → `conan`) fail. Fix: `[tool.ty.src] exclude` — proper scoping, not circumvention
- **Pre-push needs `iscc_lib` built** (`ty check`/`pytest` import it):
    `cd crates/iscc-py && uv run maturin develop --release` — BEFORE `git push`, not after the hook
    fires → `binding-reviews.md` Environment
- `[tool.ruff.lint.per-file-ignores]` waives `S101`/`S603`/`S607` for `tests/**` — a test shelling
    out to `git` with no `# noqa` is correct, not an ungated suppression

## Common Issues

- Verification greps may false-positive — check match specificity. `grep -qv 'pat'` is always true
    on multi-line files; use `! grep -q 'pat'`. **Substring `grep -c` and exact counts in next.md
    are a recurring mis-spec class** (iter 139): test the criterion against `HEAD~1` before
    believing advance broke it — a mis-specified criterion advance *corrected* is scope discipline
- **next.md's Implementation Notes are a HYPOTHESIS — algorithms and prose alike.** It can prescribe
    an algorithm that fails on real data (iter 142), dictate false docs claims (iter 143), or
    mislabel a rejected-value oracle (iter 149), and may forbid re-deriving — which makes advance
    blameless and review the only check. Re-derive every number from its source; test specs,
    expected values and test counts included (check relative paths with `realpath`)
- **Probe claims, don't accept them** — build-flag/backend activation (blake3 wasm SIMD) and
    exact-length decode rejection recipes → `review-patterns.md` "Claim-Probing Recipes"
- `iscc_decompose` returns units WITHOUT "ISCC:" prefix — cross-check doc examples. **Docs site
    URL** is `https://lib.iscc.codes/`, NOT `https://iscc-lib.iscc.io/` (advance gets this wrong)
- **Advance agent idle claims**: verify remaining issue priorities independently — it may claim
    "only low-priority remain" when `normal` issues still exist
- Go `// indirect`, CI `find` cross-arch, prek stash conflict, and mode-only commits under
    `core.fileMode=false` → `review-patterns.md` "Gotchas"
- **Writing `\uXXXX` through Edit/Write decodes it to literal UTF-8** (iter 149, bit both advance
    and review) — edit ASCII-escaped files via Python and assert `isascii()` + numeric `ord()` →
    `review-patterns.md` "Escape-decoding trap". `tests/test_vendored_fixtures.py` now gates the
    canonical `unicode_boundary.json` for this
- **`mise run check` mdformat on context files** (intermittent): define-next may write `next.md` /
    `MEMORY.md` non-conforming, so `prek --all-files` reformats them (NOT an advance regression) and
    pre-push mdformat WILL reject the batch — `git status --porcelain` right after `mise run check`
    and STAGE any reformat. Never stage `iterations.jsonl`. Keep inline code spans on ONE line
- **next.md may task advance with an `issues.md` ledger append — advance's protocol forbids it**
    (iter 135); it refuses and puts the paragraph in its handoff **Notes** for review to append. It
    may likewise task REVIEW with a spec edit (iter 144) — my protocol forbids that unless resolving
    a `[human]` issue carrying `**Spec:**`. Decline, and say so in the handoff Notes
- **No-op / human-handoff iteration (iter 111)**: verify scope is empty, still scan
    `@{upstream}..HEAD` for circumvention. Only HUMAN-REVIEW spec amendments + `low` left (strict
    IDLE cond #2 NOT met) → flag **HUMAN REVIEW REQUESTED** (runner "pause"), NOT `**IDLE**`
    (all-`low` only; it runs meta-improve). Verdict still PASS; push clean batch
- **Unicode 16.0.0 freeze rule = a `U+FFFF` SENTINEL MAP since iter 148** (delete-filter design
    RULED out, iters 133–147). Gated so far: Rust (iters 141/149), Python + pure-Go (150, Go 9/12
    with 3 ruled skips), WASM + Ruby (151, 12/12). Live tally + remaining surfaces → `issues.md`.
    Never "fix" one binding to match another; single-cp vectors are deletion-agnostic, only the 4
    sequence vectors discriminate sentinel-vs-delete
- **A binding can pass a boundary vector for the WRONG reason — check the MECHANISM** (iter 150): Go
    has no freeze rule but its 15.0 tables call U+20C1/U+A7F1 `Cn`, so its category-`C` filter
    coincidentally matches; under go1.27 five green cases flip red. That red is the *designed
    signal* — REJECT any proposal to version-gate the skip list (`decisions.md` 2026-07-26)
- **Concurrent CID loops (iter 97, detail → `MEMORY-archive.md`)**: spurious `mise run check` "files
    modified" on an untouched file + a mid-review working-tree change = a SECOND loop racing.
    Confirm with `ps aux`; flag HUMAN REVIEW REQUESTED, do NOT push or kill processes

## Review Shortcuts (see gate-reviews.md / binding-reviews.md for detail)

- **Rust-only**: `cargo test -p iscc-lib` + clippy workspace + `mise run check`. **Clippy
    workspace** is ~2s after a build — always run it
- **Test-fixture / vector-file only (iters 141/149, ~5 min)**: Rust-only PLUS the full feature
    matrix (`--no-default-features`, `+text-processing`, `--all-features`) because `#[cfg]` gating
    decides which tests run — assert the per-target `N passed` line. Then mutate the fixture four
    ways under the feature-OFF build and check the oracle attribution → `review-patterns.md`. Test
    assets + docs are budget-free
- **Docs-only**: `mise run check` + clippy + `mise run version:check` (21 `OK:` lines) +
    `uv run zensical build` ("No issues found") + rendered-HTML grep for admonition/tab edits.
    **Green gates say NOTHING about truth** (iter 143) — re-derive every number and treat
    "never/always/only" as claims to disprove → `review-patterns.md` "Docs Claim-Checking"
- **New docs PAGE (gated since iter 145)**: `uv run scripts/check_docs_nav.py` proves disk / nav /
    `ORDERED_PAGES` / `docs/llms.txt` agree (23 pages, absolute links). Two holes: a
    **commented-out** nav entry counts as present, and the hook skips a page **deletion**.
    `zensical build` **wipes `site/`** → run `gen_llms_full.py` after it → `review-patterns.md`
- **Python-only**: `mise run check` + `pytest` (+ the pre-push `ty check` / ruff `S` / `C901` gates
    when a new `.py` file lands)
- **New gate script or gate HARDENING (iters 142–146, ~8 min)**: never accept "it exits 0 at HEAD" —
    write your OWN mutations, prefer a **real** regression to a synthetic typo, dump internals via
    `importlib`, and work the blind-spot list (one-directional? fail-open? skipped-run
    distinguishable? vacuous on empty sets? parser sees comments? fix symmetric?) →
    `review-patterns.md`. A `files:`-scoped prek hook needs a **staged** fire/skip probe and never
    sees deletions
- **Repo-state gate that reads the git index (iter 152, ~8 min)**: Python-only PLUS six mutations in
    a THROWAWAY repo (`git archive HEAD | tar -x -C /tmp/x && git init` — `git clone` fails here on
    `safe.directory`; never mutate the real index mid-iteration): drift, staged extra file,
    `git rm`, decoded fixture, emptied table, `mv .git .git-off` → `review-patterns.md`
- **Lint-config-only (`[tool.ruff]*` / prek hook types, iters 134–139)**: `mise run check`, ruff
    check, ruff `format --check` (**assert exit 0, never a file count**), both pre-push ruff gates,
    ty check, pytest. A `# noqa` deletion is safe only if `--select <rule> --ignore-noqa` omits its
    line; path-sensitive settings and any `types:` change need the hook-mode probe →
    `dep-refresh-reviews.md` slice 8
- **Go-only**: `mise run check`, `CGO_ENABLED=0 mise exec -- go test -C packages/go -count=1 ./...`,
    `go vet -C packages/go ./...`, `mise exec -- gofmt -l packages/go` (empty)
- **Binding boundary-fixture slices** — WASM+Ruby (151) and the vendored-copy propagation probe
    (150), incl. the `cargo test -p iscc-wasm` = 0-tests trap → `binding-reviews.md`
- **Ruby-only / Kotlin-only / published-`.pyi`** command sets → `binding-reviews.md` "Per-binding
    review commands" (Gradle flakes on this bind mount; Kotlin consumer floor is **2.3 or newer**)
- **Config/lockfile-only**: `mise run check` + `cargo check -p <crate>` (+ `cargo deny check` if
    deps changed — see `gate-reviews.md` Audit)
- **Dependency refresh (v0.6.0)**: per-slice gates + hold-back recipes → `dep-refresh-reviews.md`.
    **ALL nine slices CLOSED** (124–140); only human/major-gated bumps left. **A toolchain bump in a
    PUBLISHED binding is a support-policy change, not a pin** — check the consumer floor first.
    Reflexes: a bump can widen a gate's FILE DISCOVERY, `# held:` reasons come from registry
    metadata not the handoff, `cargo tree -i` needs `--target all`, and a DATA-TABLE dep needs an
    exhaustive differential (the 50 vendored vectors are all Unicode ≤ 15)
- **Prek-hook-scope review (iters 138–139)**: NEVER accept `git ls-files` arithmetic as a hook's
    surface — `.pyi` is tagged `pyi`, not `python`. Probing a *widened* tag needs a **staged,
    deliberately dirty** file → `review-patterns.md`
- **Core text/codec change + generated data (iters 133/148, ≈12 min)**: Rust-only PLUS the **full
    feature matrix** PLUS the two CI-only gates — `mise run coverage` then the CI-exact
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    (`--fail-above` is a BARE flag and CI uses BOTH) and `mise run bench:iai:check`; re-run any
    generator and assert `git status --porcelain <output>` is empty. `text_clean`/`text_collapse`
    edits also need the sequence differential in `review-patterns.md`
- **Version sync**: + `version_sync.py --check`. **Shell script**: + `bash -n <script>`
- **release.yml / any GHA action bump**: NEVER exercised by CID pushes → static-verify only. Run the
    committed gates `uv run scripts/check_release_workflow.py` and `… --check-action-inputs`, never
    a retyped heredoc; **zero `warning: skipped` lines is part of the pass**. Manual: actionlint,
    `runs.using`, intervening majors' *default* changes → **`gha-workflow-reviews.md`**. `always()`
    = NEEDS_WORK
- **CI/Audit/Perf/Semver/CRAP gates** → `gate-reviews.md`

## Codex Review Integration

- Advisory only, never sets the verdict; `--commit HEAD`; empty/unavailable ≠ clean. Strengths,
    blind spots and the standing dismiss-list → **`codex-integration.md`**. Expect a real finding
    whenever a diff adds a matching/parsing rule, an exception-handling contract, or a user-facing
    factual claim — iter 149 it independently found the same oracle mis-attribution I derived
- **A no-findings Codex verdict sometimes asserts its own evidence** (iter 148: "match a Unicode
    16.0 reference across all Unicode scalar values") — an unverifiable claim in a one-paragraph
    report is not a substitute for your own probe. Run the differential anyway
- **Codex reasons about FUTURE toolchains** (iter 150: ran the Go suite under `go1.27rc1`). Verify
    the mechanism yourself, then judge the *remedy* separately — its fix was "version-gate the
    skips", i.e. pre-emptive test skipping. A correct diagnosis can carry a gate-weakening cure;
    record the rejection in `decisions.md` so the next agent does not re-adopt it
