# Review Agent Memory

Concise index — **one line per entry, detail belongs in a topic file.** `review-patterns.md`
(docs/verification/issues/gotchas + claim-probing + new-gate-script recipes), `gate-reviews.md` (CI
structure + Audit/Perf/Semver/CRAP gates), `binding-reviews.md` (per-binding shortcuts +
UniFFI/Kotlin/Ruby/Environment/PyO3/feature-flags), `dep-refresh-reviews.md` (v0.6.0 slice recipes),
`gha-workflow-reviews.md` (release.yml gates + action-major bumps + pinning), `codex-integration.md`
(second-opinion strengths/blind spots). Stale detail in `MEMORY-archive.md`.

## Quality Gate Details

- `mise run check` = the pre-commit hooks (hygiene, formatting, lint incl. Ruby, release-workflow
    static checks) — never assert the hook count, it drifts
- Pre-push hooks (clippy, cargo test, pytest, `ty`, ruff `S`/`C901`) are NOT in it — verify clippy
    with `cargo clippy --workspace --all-targets -- -D warnings`
- Java tests: `mvn test`. Go tests: `mise exec -- go test -C packages/go ./...` (`-C`, never `cd`)
- `check-added-large-files` threshold is `--maxkb=256`
- **`ty check` external Python files**: files importing packages not in the venv (e.g.
    `conanfile.py` → `conan`) fail. Fix: `[tool.ty.src] exclude` — proper scoping, not circumvention
- **Pre-push needs `iscc_lib` built** for `ty check`/`pytest`:
    `cd crates/iscc-py && uv run maturin develop --release` (see `binding-reviews.md` Environment)

## Common Issues

- Verification greps may false-positive — check match specificity. `grep -qv 'pat'` is always true
    on multi-line files; use `! grep -q 'pat'`. **Substring `grep -c` and exact file counts in
    next.md are a recurring mis-spec class** (iter 139) — test the criterion against `HEAD~1` before
    believing advance broke it; a mis-specified criterion advance *corrected* is scope discipline
- **next.md's Implementation Notes can prescribe an algorithm that fails on the real data** (iter
    142): re-prove the prescribed rule really breaks before crediting a documented deviation, and
    classify the replacement by direction — errs-strict (note it) vs errs-lax (NEEDS_WORK)
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
- **Unicode 16.0.0 freeze rule — Rust core iter 133, boundary fixture iter 141**: remaining are the
    11 bindings (Go on 15.0 tables) + the differential sweep; never let a step "fix" one binding to
    match another. **Human-gated:** the sequence-adjacency divergence (probe → `review-patterns.md`,
    ruling → `issues.md`), whose outcome must also revisit the deliberate "single-code-point"
    placeholder in `docs/unicode.md`
- **Concurrent CID loops (iter 97, detail in `MEMORY-archive.md`)**: spurious `mise run check`
    "files modified" on an untouched file + mid-review working-tree change = a SECOND loop racing.
    Confirm with `ps aux`; flag HUMAN REVIEW REQUESTED, do NOT push or kill processes

## Review Shortcuts (see gate-reviews.md / binding-reviews.md for detail)

- **Rust-only**: `cargo test -p iscc-lib` + clippy workspace + `mise run check`
- **Clippy workspace**: fast (~2s) after build — always run
- **Test-fixture / vector-file only (iter 141, ~4 min)**: Rust-only PLUS the full feature matrix
    (`--no-default-features`, `+text-processing`, `--all-features`) because `#[cfg]` gating decides
    which tests run — assert the per-target `N passed` line. Then the fixture-CONTENT guard and
    live-vs-hypothetical vector probes → `review-patterns.md`. Test assets + docs are budget-free
- **Docs-only**: `mise run check` + clippy + `mise run version:check` (21 `OK:` lines) +
    `uv run zensical build` ("No issues found") + rendered-HTML grep for admonition/tab edits.
    **Green gates say NOTHING about truth** (iter 143) — re-derive every number from its source and
    treat "never/always/only" sentences as claims to disprove; next.md prose is a HYPOTHESIS and
    dictated two false claims advance shipped verbatim → `review-patterns.md` "Docs Claim-Checking"
- **New docs PAGE**: also verify the 3 hand-wired lists agree — `zensical.toml` nav, `ORDERED_PAGES`
    in `scripts/gen_llms_full.py`, `docs/llms.txt`. Nothing gates their parity, and they are
    **already drifted** (measured iter 144: 24 tracked pages, nav 24, `ORDERED_PAGES` 23, `llms.txt`
    **17** — c-cpp/dotnet/kotlin/ruby/swift how-tos + `ruby-api.md` missing). Filed as a `normal`
    issue; note `llms.txt` links are absolute `https://lib.iscc.codes/<path>.md`, not relative
- **Python-only**: `mise run check` + `pytest`
- **New gate script (iters 142/144, ~6 min)**: never accept "it exits 0 at HEAD" — write your OWN
    mutations, prefer a **real** regression to a synthetic typo, dump internals via `importlib`, and
    answer the three blind-spot questions (one-directional? fail-open really fail-open?
    fully-skipped run distinguishable from a pass?) → `review-patterns.md`
- **Lint-config-only (`[tool.ruff]*` / prek hook types, iters 134–139)**: run `mise run check`, ruff
    check, ruff `format --check` (**assert exit 0, never a file count**), both pre-push ruff gates,
    ty check, pytest. A `# noqa` deletion is safe only if `--select <rule> --ignore-noqa` omits its
    line; path-sensitive settings and any `types:` change need the hook-mode probe. Recipes →
    `review-patterns.md` + `dep-refresh-reviews.md` slice 8
- **Go-only**: `mise run check`, `CGO_ENABLED=0 mise exec -- go test -C packages/go -count=1 ./...`,
    `go vet -C packages/go ./...`
- **Ruby-only**: `mise run check` + `cargo clippy -p iscc-rb -- -D warnings` +
    `pushd crates/iscc-rb && bundle exec rake test; popd` + `bundle exec standardrb` (needs
    `PATH="$(ruby -e "puts Gem.user_dir")/bin:$PATH"`)
- **Kotlin-only**: `cargo build -p iscc-uniffi` + `cd packages/kotlin && ./gradlew test` + clippy
    workspace + `mise run check`. Gradle flakes on this bind mount — read
    `build/test-results/test/*.xml` and re-run after `./gradlew clean` before calling it a failure.
    Published consumer floor is **Kotlin 2.3 or newer**, documented in 4 places that move together
- **Config/lockfile-only**: `mise run check` + `cargo check -p <crate>` (+ `cargo deny check` if
    deps changed — see gate-reviews.md Audit)
- **Dependency refresh (v0.6.0)**: per-slice gates + hold-back recipes → `dep-refresh-reviews.md`.
    **ALL nine slices CLOSED** (124–140); only human/major-gated bumps left. **A toolchain bump in a
    PUBLISHED binding is a support-policy change, not a pin** — check the consumer floor first
- **A tool bump can widen a gate's FILE DISCOVERY, not just its rules** (iter 137: ruff 0.16 →
    `ruff format --check` went 25 → 153 files). Diff the count, then ask **which local gate covers
    the new surface**. Relock proof: `git diff HEAD~1..HEAD -- uv.lock | grep -E '^[+-]name = '`
- **Prek-hook-scope review (iters 138–139)**: NEVER accept `git ls-files` arithmetic as a hook's
    surface — `.pyi` is tagged `pyi`, not `python`. Probing a *widened* tag needs a **staged,
    deliberately dirty** file → `review-patterns.md`
- **A published `.pyi` needs mypy + pyright, not just `ty`** (iter 131; the wheel ships `py.typed`):
    `uvx mypy@1.18.2 --strict` + `uvx pyright@1.1.407` ≈ 30s; prefer `ast.parse` over greps
- **Verify a `# held:` claim from registry metadata, never from the handoff** (iters 126/130, ~30s):
    `cargo info <crate>@<ver>`, `gem specification <gem> -v <ver> --remote`,
    `https://rubygems.org/api/v1/versions/<gem>.json`. A wrong stated reason survives as folklore
- **A dep shipping DATA TABLES (Unicode, locale, tz) needs an exhaustive differential, not green
    vectors** (iter 129): the 50 vendored vectors are all Unicode ≤ 15 → `dep-refresh-reviews.md`
- **`cargo tree -i <crate>` prints "nothing to print"** for proc-macro / target-specific deps — add
    `--target all` (this disproved an advance-handoff attribution: `proc-macro-error2` comes from
    dev-only `iai-callgrind-macros`, NOT magnus/rb-sys)
- **Core text/codec change + generated data (iter 133, ≈6 min)**: Rust-only PLUS the two CI-only
    gates — `mise run coverage` + `cargo crap` baseline and `mise run bench:iai:check` (commands →
    `gate-reviews.md`); re-run any generator, assert `git status --porcelain <output>` is empty
- **Version sync**: + `version_sync.py --check`. **Shell script**: + `bash -n <script>`
- **release.yml / any GHA action bump**: NEVER exercised by CID pushes → static-verify only. **All
    four checks are committed gates (iters 142 + 144)** — run
    `uv run scripts/check_release_workflow.py` and `… --check-action-inputs`, never a retyped
    heredoc. **Zero `warning: skipped` lines is part of the pass** (a rate-limited run is
    green-but-useless). Still manual: actionlint, `runs.using`, and every intervening major's
    *default* changes. Recipes, blind spots, cleared-defaults table and pinning conventions →
    **`gha-workflow-reviews.md`**. `always()` = NEEDS_WORK
- **CI/Audit/Perf/Semver/CRAP gates** → `gate-reviews.md`. **Binding propagation** (napi/wasm/ffi/
    jni/ruby/dotnet/kotlin/uniffi) → `binding-reviews.md`

## Codex Review Integration

- Advisory only, never sets the verdict; `--commit HEAD`; empty/unavailable ≠ clean. What it
    reliably catches, what it reliably misses, and the standing dismiss-list →
    **`codex-integration.md`**. Expect a real finding whenever a diff adds a matching/parsing rule,
    an exception-handling contract, or a user-facing factual claim
