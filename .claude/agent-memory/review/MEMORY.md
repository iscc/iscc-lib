# Review Agent Memory

Concise index. Detail in topic files: `review-patterns.md` (docs/verification/issues/gotchas +
claim-probing recipes), `gate-reviews.md` (CI structure + Audit/Perf/Semver/CRAP gate recipes),
`binding-reviews.md` (per-binding shortcuts + UniFFI/Kotlin/Ruby/Environment/PyO3/feature-flags),
`dep-refresh-reviews.md` (v0.6.0 dependency-refresh slice recipes). Stale detail in
`MEMORY-archive.md`.

## Quality Gate Details

- `mise run check` runs 15 pre-commit hooks (file hygiene, formatting, linting incl. Ruby)
- Pre-push hooks (clippy, cargo test, pytest, etc.) NOT in `mise run check` — verify clippy with
    `cargo clippy --workspace --all-targets -- -D warnings`
- Java tests: `mvn test`. Go tests: `mise exec -- go test -C packages/go ./...` (`-C`, never `cd`)
- `check-added-large-files` threshold is `--maxkb=256`
- **`ty check` external Python files**: files importing packages not in the venv (e.g.
    `conanfile.py` → `conan`) fail. Fix: `[tool.ty.src] exclude` — proper scoping, not circumvention
- **Pre-push needs `iscc_lib` built** for `ty check`/`pytest`:
    `cd crates/iscc-py && uv run maturin develop --release` (see `binding-reviews.md` Environment)

## Common Issues

- Verification grep patterns may false-positive — verify match specificity. `grep -qv 'pattern'` is
    always true for multi-line files — use `! grep -q 'pattern'` to verify absence. **Substring
    `grep -c` and exact file counts in next.md are a recurring mis-spec class** (iter 139:
    `grep -c 'exclude'` hit the pre-existing `--force-exclude` flags; `format --check` counted 155
    not 153). Check the criterion against `HEAD~1` before believing advance broke it — a
    mis-specified criterion advance *corrected* is scope discipline, not a failure
- next.md test specs / expected values / test counts may be wrong — always run tests, verify against
    Rust implementation
- **Probe claims, don't accept them** — build-flag/backend activation (blake3 wasm SIMD) and
    exact-length decode rejection recipes → `review-patterns.md` "Claim-Probing Recipes"
- `iscc_decompose` returns units WITHOUT "ISCC:" prefix — cross-check doc examples
- **Docs site URL**: `https://lib.iscc.codes/` NOT `https://iscc-lib.iscc.io/`. Advance agents
    consistently get this wrong — always verify
- **next.md relative path errors**: verify relative paths with `realpath` during review
- **Advance agent idle claims**: always verify remaining issue priorities independently — they may
    claim "only low-priority remain" when `normal` issues still exist
- Go `// indirect`, CI `find` cross-arch, prek stash conflict, and mode-only commits under
    `core.fileMode=false` → `review-patterns.md` "Gotchas"
- **`mise run check` mdformat on context files** (intermittent): define-next sometimes writes
    `next.md` / `define-next/MEMORY.md` non-conforming, so `prek --all-files` reformats them (NOT an
    advance regression). Staged-only `git commit` is unaffected, but the pre-push mdformat hook runs
    on the whole push range and WILL reject the batch — run `git status --porcelain` right after
    `mise run check` and STAGE any reformatted file. Never stage `iterations.jsonl` (runner-owned)
- **next.md may task advance with an `issues.md` ledger append — advance's protocol forbids it**
    (iter 135). Advance refuses and puts the paragraph in its handoff **Notes**; review must append
    it to the issue. Scope *discipline*, never an incomplete criterion; worth a learnings.md note
- **No-op / human-handoff iteration (iter 111)**: verify scope is empty, still scan
    `@{upstream}..HEAD` for circumvention. Only HUMAN-REVIEW spec amendments + `low` left (strict
    IDLE cond #2 NOT met) → flag **HUMAN REVIEW REQUESTED** (runner "pause"), NOT `**IDLE**`
    (all-`low` only; it runs meta-improve). Verdict still PASS; push clean batch
- **Unicode 16.0.0 freeze rule — Rust core landed iter 133**: `text_clean`/`text_collapse` strip
    16.0-unassigned code points before normalization. Remaining: boundary vectors in 11 bindings
    (`packages/go` is on 15.0 tables — needs a delta or a documented skip) and the differential
    sweep. Never let a step "fix" one binding to match another. **Open, human-gated:** the
    pre-normalization order diverges from `iscc-core` on sequences (adjacency) — probe recipe in
    `review-patterns.md`, ruling in `issues.md`, rationale in `decisions.md`
- **Concurrent CID loops (iter 97, detail in `MEMORY-archive.md`)**: spurious `mise run check`
    "files modified" on an untouched file + mid-review working-tree change = a SECOND loop racing.
    Confirm with `ps aux`; flag HUMAN REVIEW REQUESTED, do NOT push or kill processes

## Review Shortcuts (see gate-reviews.md / binding-reviews.md for detail)

- **Rust-only**: `cargo test -p iscc-lib` + clippy workspace + `mise run check`
- **Clippy workspace**: fast (~2s) after build — always run
- **Docs-only**: `mise run check` + clippy + `mise run version:check` (21 `OK:` lines) +
    `uv run zensical build` ("No issues found", ~13s) + rendered-HTML grep for admonition/tab edits
    (recipe → `review-patterns.md`)
- **Python-only**: `mise run check` + `pytest`
- **Lint-config-only (`[tool.ruff]*` / prek hook types, iters 134–139)**: `mise run check` (twice if
    two formatters share a surface), `uv run ruff check`, `format --check` (**assert exit 0, never a
    file count** — it grows with every tracked `.md`/`.py`/`.pyi`, incl. CID's own memory files; 155
    at iter 139), both pre-push gates (`--select S` / `--select C901 --force-exclude`), `ty check`,
    `pytest`. A `# noqa` deletion is only safe if `--select <rule> --ignore-noqa` does NOT list its
    line; path-sensitive settings (`src`, isort, `exclude`) and any `types:` change need the
    hook-mode probe. Recipes → `dep-refresh-reviews.md` slice 8 + `review-patterns.md`. Docs edit →
    `uv run zensical build`
- **Go-only**: `mise run check`, `CGO_ENABLED=0 mise exec -- go test -C packages/go -count=1 ./...`,
    `go vet -C packages/go ./...`
- **Ruby-only**: `mise run check` + `cargo clippy -p iscc-rb -- -D warnings` +
    `pushd crates/iscc-rb && bundle exec rake test; popd` + `bundle exec standardrb` (needs
    `PATH="$(ruby -e "puts Gem.user_dir")/bin:$PATH"`)
- **Kotlin-only**: `cargo build -p iscc-uniffi` + `cd packages/kotlin && ./gradlew test` + clippy
    workspace + `mise run check`. Gradle flakes on this bind mount — read
    `build/test-results/test/*.xml` and re-run after `./gradlew clean` before calling it a failure.
    A KGP-version diff needs the consumer-floor test (`dep-refresh-reviews.md`); published floor is
    **Kotlin 2.3 or newer**, documented in 4 places that must move together
- **Config/lockfile-only**: `mise run check` + `cargo check -p <crate>` (+ `cargo deny check` if
    deps changed — see gate-reviews.md Audit)
- **Dependency refresh slices (v0.6.0 issue)**: per-slice gate sets + hold-back recipes for slices
    1–8 (iters 124–137; all locally-verifiable slices CLOSED, ruff is 0.16.0 project-wide) →
    `dep-refresh-reviews.md`. Never use the lockfile-only shortcut on these. **A toolchain/compiler
    bump inside a PUBLISHED binding is a support-policy change, not a pin** — check the consumer
    floor (iter 128 recipe) before passing it
- **A tool bump can widen a gate's FILE DISCOVERY, not just its rules** (iter 137): ruff 0.16
    formats Python fences inside Markdown, so bare `ruff format --check` went 25 → 153 files. Always
    diff the file count before/after, then ask **which local gate covers the new surface** (closed
    iters 138–139: `ruff-format` = `types_or: [python, pyi, markdown]`, `ruff-check` =
    `types_or: [python, pyi]`). Single-package relock proof:
    `git diff HEAD~1..HEAD -- uv.lock | grep -E '^[+-]name = '` → empty
- **Prek-hook-scope review (iters 138–139)**: NEVER accept `git ls-files` arithmetic as a hook's
    surface — a `types:` tag is prek's classification, and `.pyi` is tagged `pyi`, not `python`
    (that hole is CLOSED: both ruff hooks carry `pyi` since iter 139; local is now a strict superset
    of CI). Probing a *widened* tag needs a **staged, deliberately dirty** file — a clean file gives
    `Passed` whether or not the hook saw it; only `files were modified by this hook` proves it
    bites. Staged-probe + ping-pong + alternate-config recipes → `review-patterns.md`
- **A published `.pyi` needs mypy + pyright, not just `ty`** (iter 131): the wheel ships `py.typed`
    beside `_lowlevel.pyi`, so it is consumer-facing. `uvx mypy@1.18.2 --strict` +
    `uvx pyright@1.1.407` ≈ 30s. Prefer an `ast.parse` body assertion over greps for bulk stub edits
- **Verify a `# held:` claim from registry metadata, never from the handoff** (iters 126/130, ~30s):
    `cargo info <crate>@<ver>` (rust-version), `gem specification <gem> -v <ver> --remote`,
    `https://rubygems.org/api/v1/versions/<gem>.json` (`ruby_version`; v2 returns null). A wrong
    stated reason survives as folklore — worse than no comment
- **A dep shipping DATA TABLES (Unicode, locale, tz) needs an exhaustive differential, not green
    vectors** (iter 129): the 50 vendored vectors are all Unicode ≤ 15 → `dep-refresh-reviews.md`
- **`cargo tree -i <crate>` prints "nothing to print"** for proc-macro / target-specific deps — add
    `--target all`. Disproved an advance-handoff attribution with it: the `proc-macro-error2 v2.0.1`
    future-incompat warning is from `iai-callgrind-macros` (dev-only), NOT magnus/rb-sys
- **Core text/codec change + generated data (iter 133)**: Rust-only shortcut PLUS the two CI-only
    gates — `mise run coverage` then the `cargo crap` baseline check (~2 min, exact command in
    `gate-reviews.md`), and `mise run bench:iai:check` (~1 min). Re-run any checked-in generator and
    assert `git status --porcelain <output>` is empty. Total ≈ 6 min
- **Version sync addition**: `mise run check` + `uv run scripts/version_sync.py --check` + clippy
- **Script-only (shell)**: `bash -n <script>` + `mise run check` + clippy (when no Rust changes)
- **release.yml-only (iters 123/139)**: NOT exercised by CID pushes → static-verify only: YAML parse
    (`yaml.safe_load` + regex over every job's `if:`), `actionlint@v1.7.7` (cached, offline),
    `prek run check-yaml`/`yamlfix --files`, matrix-entry presence, artifact-name consistency across
    build→test→publish. Adding a wheel target = build+test matrix only (`publish-pypi` collects via
    `pattern: wheels-*`). **All 28 non-`prepare-release` jobs now carry
    `${{ !cancelled() && !failure() && (…) }}`** (iter 139 closed the re-trigger bug). When a step
    adds a job, check the **invariant**: its `needs` chain must be gated by the same registry flag
    or a superset (`build-ffi` = `ffi || nuget`) — otherwise the relaxed `success()` lets it run
    against artifacts never built. `always()` instead of `!failure()` = NEEDS_WORK. Rationale +
    invariant in `decisions.md` 2026-07-25
- **CI/Audit/Perf/Semver/CRAP gate reviews**: see `gate-reviews.md`
- **Binding propagation (napi/wasm/ffi/jni/ruby/dotnet/kotlin/uniffi)**: see `binding-reviews.md`
- Cross-platform CI: bash syntax needs `shell: bash` if matrix includes Windows

## Codex Review Integration

- Codex findings are advisory — cross-reference with your own analysis. Use `--commit HEAD` (verify
    with `git log` first). A clean/empty verdict is a note, never grounds for NEEDS_WORK
- Dismiss: Go codec findings (Go mirrors the Rust reference faithfully); "use `8.0.x`" for
    `dotnet-version: '8.0'` (valid for `setup-dotnet@v4`). Large generated Kotlin/Swift diffs
    confuse it — advisory only
- **Trust Codex on dependency-internals findings** (117-118): it correctly read blake3's feature
    gating and `#[target_feature]` attributes. When it cites a dep's build.rs / feature wiring,
    check the dep source (and build it) before dismissing
- **It catches downstream-consumer breakage local gates cannot** (128, KGP consumer floor) **and
    input-validation edge cases** (119, trailing-byte decode). Convergence with my own suspicion is
    the cue to VERIFY EMPIRICALLY. **But it does not probe tool-internal classification** — it
    passed the iter-138 hook-types diff clean while missing the `.pyi` tag hole
