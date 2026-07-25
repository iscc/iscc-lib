# Review Agent Memory

Concise index — **one line per entry, detail belongs in a topic file.** `review-patterns.md`
(docs/verification/issues/gotchas + claim-probing recipes), `gate-reviews.md` (CI structure +
Audit/Perf/Semver/CRAP gates), `binding-reviews.md` (per-binding shortcuts +
UniFFI/Kotlin/Ruby/Environment/PyO3/feature-flags), `dep-refresh-reviews.md` (v0.6.0 slice recipes),
`gha-workflow-reviews.md` (release.yml static gates + action-major bumps + pinning). Stale detail in
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

- Verification greps may false-positive — check match specificity. `grep -qv 'pat'` is always true
    on multi-line files; use `! grep -q 'pat'`. **Substring `grep -c` and exact file counts in
    next.md are a recurring mis-spec class** (iter 139) — test the criterion against `HEAD~1` before
    believing advance broke it; a mis-specified criterion advance *corrected* is scope discipline,
    not a failure
- next.md test specs / expected values / test counts may be wrong — always run tests and verify
    against the Rust implementation; check relative paths with `realpath`
- **Probe claims, don't accept them** — build-flag/backend activation (blake3 wasm SIMD) and
    exact-length decode rejection recipes → `review-patterns.md` "Claim-Probing Recipes"
- `iscc_decompose` returns units WITHOUT "ISCC:" prefix — cross-check doc examples. **Docs site
    URL** is `https://lib.iscc.codes/`, NOT `https://iscc-lib.iscc.io/` (advance gets this wrong a
    lot)
- **Advance agent idle claims**: always verify remaining issue priorities independently — they may
    claim "only low-priority remain" when `normal` issues still exist
- Go `// indirect`, CI `find` cross-arch, prek stash conflict, and mode-only commits under
    `core.fileMode=false` → `review-patterns.md` "Gotchas"
- **`mise run check` mdformat on context files** (intermittent): define-next sometimes writes
    `next.md` / `define-next/MEMORY.md` non-conforming, so `prek --all-files` reformats them (NOT an
    advance regression). Staged-only commits are unaffected, but the pre-push mdformat hook covers
    the whole push range and WILL reject the batch — `git status --porcelain` right after
    `mise run check` and STAGE any reformatted file. Never stage `iterations.jsonl` (runner-owned)
- **next.md may task advance with an `issues.md` ledger append — advance's protocol forbids it**
    (iter 135). Advance refuses and puts the paragraph in its handoff **Notes**; review appends it.
    Scope *discipline*, never an incomplete criterion
- **No-op / human-handoff iteration (iter 111)**: verify scope is empty, still scan
    `@{upstream}..HEAD` for circumvention. Only HUMAN-REVIEW spec amendments + `low` left (strict
    IDLE cond #2 NOT met) → flag **HUMAN REVIEW REQUESTED** (runner "pause"), NOT `**IDLE**`
    (all-`low` only; it runs meta-improve). Verdict still PASS; push clean batch
- **Unicode 16.0.0 freeze rule — Rust core landed iter 133**: remaining are boundary vectors in 11
    bindings (Go is on 15.0 tables → needs a delta or documented skip) + the differential sweep;
    never let a step "fix" one binding to match another. **Open, human-gated:** pre-normalization
    stripping diverges from `iscc-core` on *sequences* (adjacency) — probe recipe in
    `review-patterns.md`, ruling in `issues.md`, rationale in `decisions.md`
- **Concurrent CID loops (iter 97, detail in `MEMORY-archive.md`)**: spurious `mise run check`
    "files modified" on an untouched file + mid-review working-tree change = a SECOND loop racing.
    Confirm with `ps aux`; flag HUMAN REVIEW REQUESTED, do NOT push or kill processes

## Review Shortcuts (see gate-reviews.md / binding-reviews.md for detail)

- **Rust-only**: `cargo test -p iscc-lib` + clippy workspace + `mise run check`
- **Clippy workspace**: fast (~2s) after build — always run
- **Test-fixture / vector-file only (iter 141, ~4 min)**: Rust-only shortcut PLUS the full feature
    matrix (`--no-default-features`, `+text-processing`, `--all-features`) because `#[cfg]` gating
    decides which tests actually run — assert the per-target `N passed` line, not just exit 0. Then
    two probes from `review-patterns.md`: the fixture-CONTENT guard (mutation-probe it) and
    live-vs-hypothetical vector proof. Test assets + docs don't consume the 3-file scope budget
- **Docs-only**: `mise run check` + clippy + `mise run version:check` (21 `OK:` lines) +
    `uv run zensical build` ("No issues found", ~13s) + rendered-HTML grep for admonition/tab edits
    (recipe → `review-patterns.md`)
- **Python-only**: `mise run check` + `pytest`
- **Lint-config-only (`[tool.ruff]*` / prek hook types, iters 134–139)**: `mise run check` (twice if
    two formatters share a surface), `uv run ruff check`, `format --check` (**assert exit 0, never a
    file count**), both pre-push gates (`--select S` / `--select C901 --force-exclude`), `ty check`,
    `pytest`. A `# noqa` deletion is safe only if `--select <rule> --ignore-noqa` omits its line;
    path-sensitive settings and any `types:` change need the hook-mode probe. Recipes →
    `dep-refresh-reviews.md` slice 8 + `review-patterns.md`
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
    1–9 (iters 124–140; **ALL nine slices CLOSED** — ruff 0.16 project-wide, release.yml refs at
    current majors; only human/major-gated bumps left: xunit 3.x, Test.Sdk 18.x, Gradle wrapper,
    JUnit 6.x, jni 0.22, magnus 0.8) → `dep-refresh-reviews.md`. Never use the lockfile-only
    shortcut on these. **A toolchain/compiler bump inside a PUBLISHED binding is a support-policy
    change, not a pin** — check the consumer floor (iter 128 recipe) before passing it
- **A tool bump can widen a gate's FILE DISCOVERY, not just its rules** (iter 137): ruff 0.16
    formats Python fences in Markdown → bare `ruff format --check` went 25 → 153 files. Diff the
    file count before/after, then ask **which local gate covers the new surface**. Single-package
    relock proof: `git diff HEAD~1..HEAD -- uv.lock | grep -E '^[+-]name = '` → empty
- **Prek-hook-scope review (iters 138–139)**: NEVER accept `git ls-files` arithmetic as a hook's
    surface — a `types:` tag is prek's classification, and `.pyi` is tagged `pyi`, not `python`
    (hole CLOSED iter 139; local is now a strict superset of CI). Probing a *widened* tag needs a
    **staged, deliberately dirty** file — a clean file gives `Passed` either way; only
    `files were modified by this hook` proves it bites. Recipes → `review-patterns.md`
- **A published `.pyi` needs mypy + pyright, not just `ty`** (iter 131; the wheel ships `py.typed`):
    `uvx mypy@1.18.2 --strict` + `uvx pyright@1.1.407` ≈ 30s. Prefer an `ast.parse` body assertion
    over greps for bulk stub edits
- **Verify a `# held:` claim from registry metadata, never from the handoff** (iters 126/130, ~30s):
    `cargo info <crate>@<ver>`, `gem specification <gem> -v <ver> --remote`,
    `https://rubygems.org/api/v1/versions/<gem>.json`. A wrong stated reason survives as folklore
- **A dep shipping DATA TABLES (Unicode, locale, tz) needs an exhaustive differential, not green
    vectors** (iter 129): the 50 vendored vectors are all Unicode ≤ 15 → `dep-refresh-reviews.md`
- **`cargo tree -i <crate>` prints "nothing to print"** for proc-macro / target-specific deps — add
    `--target all` (this disproved an advance-handoff attribution: `proc-macro-error2` comes from
    dev-only `iai-callgrind-macros`, NOT magnus/rb-sys)
- **Core text/codec change + generated data (iter 133)**: Rust-only shortcut PLUS the two CI-only
    gates — `mise run coverage` + the `cargo crap` baseline check (~2 min, command in
    `gate-reviews.md`) and `mise run bench:iai:check` (~1 min); re-run any checked-in generator and
    assert `git status --porcelain <output>` is empty. ≈ 6 min
- **Version sync**: + `version_sync.py --check`. **Shell script**: + `bash -n <script>`
- **release.yml / any GHA action bump (iters 123/139/140)**: NEVER exercised by CID pushes →
    static-verify only; five release.yml gates, the action-major recipe (tag existence is the
    *weakest* check), cleared-defaults table, pinning conventions, and the `shell: bash`-on-Windows
    rule → **`gha-workflow-reviews.md`**. `always()` for `!failure()` = NEEDS_WORK
- **CI/Audit/Perf/Semver/CRAP gates** → `gate-reviews.md`. **Binding propagation** (napi/wasm/ffi/
    jni/ruby/dotnet/kotlin/uniffi) → `binding-reviews.md`

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
    passed the iter-138 hook-types diff clean while missing the `.pyi` tag hole, and (141) it read a
    new fixture as "accurate" without asking what happens if the fixture itself is weakened. It
    reviews what the code *does*, not what the gates *cannot catch* — that gap is mine to cover
