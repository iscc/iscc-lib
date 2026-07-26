# Review Agent Memory

Concise index — **one line per entry, detail belongs in a topic file.** `review-patterns.md`
(docs/verification/issues/gotchas + claim-probing + new-gate-script + fixture/oracle recipes),
`gate-reviews.md` (CI structure + Audit/Perf/Semver/CRAP gates), `binding-reviews.md` (per-binding
shortcuts + UniFFI/Kotlin/Ruby/Environment/PyO3/feature-flags), `dep-refresh-reviews.md` (v0.6.0
slice recipes), `gha-workflow-reviews.md` (release.yml gates + action-major bumps + pinning),
`codex-integration.md` (second-opinion strengths/blind spots). Stale detail in `MEMORY-archive.md`.

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
    `cd crates/iscc-py && uv run maturin develop --release` — run it BEFORE `git push`, not after
    the hook fires → `binding-reviews.md` Environment

## Common Issues

- Verification greps may false-positive — check match specificity. `grep -qv 'pat'` is always true
    on multi-line files; use `! grep -q 'pat'`. **Substring `grep -c` and exact file counts in
    next.md are a recurring mis-spec class** (iter 139) — test the criterion against `HEAD~1` before
    believing advance broke it; a mis-specified criterion advance *corrected* is scope discipline
- **next.md's Implementation Notes are a HYPOTHESIS — algorithms and prose alike.** It can prescribe
    an algorithm that fails on the real data (iter 142: re-prove the rule really breaks, then
    classify the replacement errs-strict/errs-lax), dictate false docs claims (iter 143), or
    mislabel a rejected-value oracle (iter 149) — and it may forbid re-deriving, which makes advance
    blameless and review the only check. Re-derive every number from its source
- next.md test specs / expected values / test counts may be wrong — always run tests and verify
    against the Rust implementation; check relative paths with `realpath`
- **Probe claims, don't accept them** — build-flag/backend activation (blake3 wasm SIMD) and
    exact-length decode rejection recipes → `review-patterns.md` "Claim-Probing Recipes"
- `iscc_decompose` returns units WITHOUT "ISCC:" prefix — cross-check doc examples. **Docs site
    URL** is `https://lib.iscc.codes/`, NOT `https://iscc-lib.iscc.io/` (advance gets this wrong)
- **Advance agent idle claims**: always verify remaining issue priorities independently — they may
    claim "only low-priority remain" when `normal` issues still exist
- Go `// indirect`, CI `find` cross-arch, prek stash conflict, and mode-only commits under
    `core.fileMode=false` → `review-patterns.md` "Gotchas"
- **Writing `\uXXXX` through the Edit/Write tools decodes it to literal UTF-8** (iter 149, bit both
    advance and review). ASCII-escaped files (`unicode_boundary.json`, sibling `data.json`,
    `issues.md` escape tables) must be edited via Python with `"\\u"` or
    `json.dumps(..., ensure_ascii=True)`; verify `raw.isascii()` + numeric `ord()`, never glyphs
- **`mise run check` mdformat on context files** (intermittent): define-next may write `next.md` /
    `MEMORY.md` non-conforming, so `prek --all-files` reformats them (NOT an advance regression) and
    pre-push mdformat WILL reject the batch — `git status --porcelain` right after `mise run check`
    and STAGE any reformat. Never stage `iterations.jsonl`. Keep inline code spans on ONE line
- **next.md may task advance with an `issues.md` ledger append — advance's protocol forbids it**
    (iter 135). Advance refuses and puts the paragraph in its handoff **Notes**; review appends it
- **next.md may also task REVIEW with a spec edit (iter 144, `specs/ci-cd.md` job table) — my own
    protocol forbids it** unless resolving a `[human]` issue carrying `**Spec:**`. Decline, and say
    so in the handoff Notes so define-next stops assigning it
- **No-op / human-handoff iteration (iter 111)**: verify scope is empty, still scan
    `@{upstream}..HEAD` for circumvention. Only HUMAN-REVIEW spec amendments + `low` left (strict
    IDLE cond #2 NOT met) → flag **HUMAN REVIEW REQUESTED** (runner "pause"), NOT `**IDLE**`
    (all-`low` only; it runs meta-improve). Verdict still PASS; push clean batch
- **Unicode 16.0.0 freeze rule = a `U+FFFF` SENTINEL MAP since iter 148** (delete-filter iters
    133–147; adjacency divergence RULED/CLOSED). Rust fixture COMPLETE since iter 149 (4 single code
    points + 4 sequence vectors, guarded in `test_unicode_boundary.rs`); **Python + pure-Go
    propagated iter 150** (Go runs 9/12, 3 ruled skips). Remaining: 9 binding surfaces + 4 sibling
    `data.json` → criterion-4 sweep. Never "fix" one binding to match another; single-cp vectors are
    deletion-agnostic, only the 4 sequence vectors discriminate sentinel-vs-delete
- **A binding can pass a boundary vector for the WRONG reason — check the MECHANISM** (iter 150): Go
    has no freeze rule but its 15.0 tables call U+20C1/U+A7F1 `Cn`, so the category-`C` filter
    coincidentally matches. Under go1.27 (`x/text` `tables17.0.0.go` = `//go:build go1.27`) 5 green
    cases flip red — that red is the *designed signal*, so REJECT any proposal to version-gate the
    skip list (Codex raised exactly this as P1). `ci.yml` pins Go via `go-version-file: …/go.mod`
- **Concurrent CID loops (iter 97, detail in `MEMORY-archive.md`)**: spurious `mise run check`
    "files modified" on an untouched file + mid-review working-tree change = a SECOND loop racing.
    Confirm with `ps aux`; flag HUMAN REVIEW REQUESTED, do NOT push or kill processes

## Review Shortcuts (see gate-reviews.md / binding-reviews.md for detail)

- **Rust-only**: `cargo test -p iscc-lib` + clippy workspace + `mise run check`
- **Clippy workspace**: fast (~2s) after build — always run
- **Test-fixture / vector-file only (iters 141/149, ~5 min)**: Rust-only PLUS the full feature
    matrix (`--no-default-features`, `+text-processing`, `--all-features`) because `#[cfg]` gating
    decides which tests run — assert the per-target `N passed` line. Then mutate the fixture four
    ways under the feature-OFF build and check the oracle attribution → `review-patterns.md`
    "Fixture / oracle review". Test assets + docs are budget-free
- **Docs-only**: `mise run check` + clippy + `mise run version:check` (21 `OK:` lines) +
    `uv run zensical build` ("No issues found") + rendered-HTML grep for admonition/tab edits.
    **Green gates say NOTHING about truth** (iter 143) — re-derive every number and treat
    "never/always/only" as claims to disprove → `review-patterns.md` "Docs Claim-Checking"
- **New docs PAGE (gated since iter 145)**: `uv run scripts/check_docs_nav.py` proves disk / nav /
    `ORDERED_PAGES` / `docs/llms.txt` agree (23 pages, absolute links). Two holes: a
    **commented-out** nav entry counts as present, and the hook skips a page **deletion**.
    `zensical build` **wipes `site/`** → run `gen_llms_full.py` after it → `review-patterns.md`
- **Python-only**: `mise run check` + `pytest`
- **New gate script or gate HARDENING (iters 142/144/145/146, ~8 min)**: never accept "it exits 0 at
    HEAD" — write your OWN mutations, prefer a **real** regression to a synthetic typo, dump
    internals via `importlib`, and work the blind-spot question list (one-directional? fail-open?
    skipped-run distinguishable? vacuous on empty sets? parser sees comments? new check non-vacuous?
    fix symmetric?) → `review-patterns.md`. A `files:`-scoped prek hook needs a **staged** fire/skip
    probe and never sees deletions
- **Lint-config-only (`[tool.ruff]*` / prek hook types, iters 134–139)**: run `mise run check`, ruff
    check, ruff `format --check` (**assert exit 0, never a file count**), both pre-push ruff gates,
    ty check, pytest. A `# noqa` deletion is safe only if `--select <rule> --ignore-noqa` omits its
    line; path-sensitive settings and any `types:` change need the hook-mode probe →
    `dep-refresh-   reviews.md` slice 8
- **Go-only**: `mise run check`, `CGO_ENABLED=0 mise exec -- go test -C packages/go -count=1 ./...`,
    `go vet -C packages/go ./...`, `mise exec -- gofmt -l packages/go` (empty)
- **Binding fixture PROPAGATION slice (iter 150, ~10 min)**: per-language shortcut + `cmp` the
    vendored copy + `isascii()`. Then probe in a `cp -r` of the package under `/tmp` (never the work
    tree): corrupt a **non-skipped** expected output, **rename** a skipped case (must fire BOTH the
    stale-skip guard and an unskipped failure), drop a case + wrong version. Python probes need the
    test nested two dirs deep so `Path(__file__).parent.parent` resolves — a missing fixture is a
    hard collection error, not a silent skip. Verify no `delete_filter_output` oracle was copied
    (`git grep -l delete_filter`) and that the sequence vectors still discriminate the design
- **Ruby-only / Kotlin-only / published-`.pyi`** command sets → `binding-reviews.md` "Per-binding
    review commands" (Gradle flakes on this bind mount; Kotlin consumer floor is **2.3 or newer**)
- **Config/lockfile-only**: `mise run check` + `cargo check -p <crate>` (+ `cargo deny check` if
    deps changed — see gate-reviews.md Audit)
- **Dependency refresh (v0.6.0)**: per-slice gates + hold-back recipes → `dep-refresh-reviews.md`.
    **ALL nine slices CLOSED** (124–140); only human/major-gated bumps left. **A toolchain bump in a
    PUBLISHED binding is a support-policy change, not a pin** — check the consumer floor first
- **Tool/dep-bump reflexes** — a bump can widen a gate's FILE DISCOVERY, `# held:` reasons come from
    registry metadata not the handoff, `cargo tree -i` needs `--target all`, and a DATA-TABLE dep
    needs an exhaustive differential (the 50 vendored vectors are all Unicode ≤ 15) →
    `dep-refresh-reviews.md`
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
- **CI/Audit/Perf/Semver/CRAP gates** → `gate-reviews.md`. **Binding propagation** (napi/wasm/ffi/
    jni/ruby/dotnet/kotlin/uniffi) → `binding-reviews.md`

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
