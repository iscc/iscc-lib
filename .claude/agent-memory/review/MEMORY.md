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
    `conanfile.py` → `conan`) fail `ty check`. Fix: `[tool.ty.src] exclude` in `pyproject.toml` —
    proper scope exclusion, not circumvention
- **Pre-push needs `iscc_lib` built** for `ty check`/`pytest`:
    `cd crates/iscc-py && uv run maturin develop --release` (see `binding-reviews.md` Environment)

## Common Issues

- Go `go get` adds deps as `// indirect` — run `go mod tidy` after
- Verification grep patterns may false-positive — verify match specificity. `grep -qv 'pattern'` is
    always true for multi-line files — use `! grep -q 'pattern'` to verify absence
- next.md test specs / expected values / test counts may be wrong — always run tests, verify against
    Rust implementation
- **Probe claims, don't accept them** — build-flag/backend activation (blake3 wasm SIMD) and
    exact-length decode rejection recipes → `review-patterns.md` "Claim-Probing Recipes"
- `iscc_decompose` returns units WITHOUT "ISCC:" prefix — cross-check doc examples
- **Docs site URL**: `https://lib.iscc.codes/` NOT `https://iscc-lib.iscc.io/`. Advance agents
    consistently get this wrong — always verify
- **next.md relative path errors**: verify relative paths with `realpath` during review
- **CI find cross-arch bug**: when multiple targets extract to same CWD with same lib name, `find`
    patterns must include target name (generic wildcards match all extracted dirs)
- **Advance agent idle claims**: always verify remaining issue priorities independently — they may
    claim "only low-priority remain" when `normal` issues still exist
- **A mode-only commit is invisible to `git status` here** (iter 136): `core.fileMode=false` on the
    9p bind mount, so review a `chmod`-style change with `git diff HEAD~1 --summary`
    (`mode change 100644 => 100755`) and `git ls-files -s <path>`, never a content diff or
    porcelain. Confirm the blob hash is unchanged to prove it really is mode-only
- **prek stash conflict**: untracked files with formatting issues break prek stash/restore during
    commit. Fix: move untracked files to /tmp before committing, restore after
- **`mise run check` mdformat on context files** (intermittent — clean in 135, dirty before):
    define-next sometimes writes `next.md` / `define-next/MEMORY.md` non-conforming, so
    `prek --all-files` reformats them (NOT an advance regression). Staged-only `git commit` is
    unaffected, but the pre-push mdformat hook runs on the whole push range and WILL reject the
    batch — always run `git status --porcelain` right after `mise run check` and STAGE any
    reformatted file into the review commit. Never stage `iterations.jsonl` (runner-owned)
- **next.md may task advance with an `issues.md` ledger append — advance's protocol forbids it**
    (iter 135). Advance correctly refuses and puts the exact paragraph in its handoff **Notes**; the
    review agent must then append it to the issue's progress log. This is scope *discipline*, never
    an incomplete criterion — and it is worth a learnings.md note aimed at define-next
- **No-op / human-handoff iteration (iter 111)**: verify scope is empty
    (`git diff HEAD~1..HEAD --stat -- crates/ packages/ scripts/ docs/ notes/ .claude/context/specs/`)
    and still scan `@{upstream}..HEAD` for gate circumvention. Only HUMAN-REVIEW spec amendments +
    `low` left (strict IDLE cond #2 NOT met) → flag **HUMAN REVIEW REQUESTED** (runner "pause"), NOT
    `**IDLE**` (all-`low` only; it runs meta-improve). Verdict still PASS; push clean batch
- **Unicode 16.0.0 freeze rule — Rust core landed iter 133**: `text_clean`/`text_collapse` strip
    16.0-unassigned code points before normalization (vendored 731-range table + PEP 723 generator).
    Remaining: boundary vectors in 11 bindings (`packages/go` is on 15.0 tables and needs a delta or
    a documented skip) and the differential sweep. Never let a step "fix" one binding to match
    another. **Open, human-gated:** the pre-normalization order diverges from `iscc-core` on
    sequences (adjacency change) — probe recipe in `review-patterns.md`, ruling tracked in
    `issues.md`, rationale in `decisions.md`
- **Concurrent CID loops (iter 97, resolved iter 98 — in `MEMORY-archive.md`)**: spurious
    `mise run check` "files modified" on an untouched file + mid-review working-tree change = a
    SECOND loop racing. Confirm `ps aux | grep -E 'cid:run|claude -p CID iteration'`; flag HUMAN
    REVIEW REQUESTED, do NOT push or kill processes

## Review Shortcuts (see gate-reviews.md / binding-reviews.md for detail)

- **Rust-only**: `cargo test -p iscc-lib` + clippy workspace + `mise run check`
- **Clippy workspace**: fast (~2s) after build — always run
- **Docs-only**: `mise run check` + clippy + `mise run version:check` (21 `OK:` lines) +
    `uv run zensical build` ("No issues found", ~13s) + rendered-HTML grep for admonition/tab edits
    (recipe → `review-patterns.md`)
- **Python-only**: `mise run check` + `pytest`
- **Lint-config-only (`[tool.ruff]*`, iters 134/135/136)**: `mise run check` + `uv run ruff check` +
    `format --check` + both pre-push gates (`--select S` / `--select C901 --force-exclude`) +
    `uv run ty check`, `uvx ruff@0.16.0 check . --output-format concise` (count must match next.md)
    and `pytest`. A `# noqa` deletion is only safe if `--select <rule> --ignore-noqa` does NOT list
    its line; a path-sensitive setting (`src`, isort, `exclude`) needs the hook-mode probe — the
    pre-commit hook passes FILENAMES and has no `--force-exclude`. Both recipes →
    `dep-refresh-reviews.md` slice 8. Docs edit → `uv run zensical build`
- **Go-only**: `mise run check` + `CGO_ENABLED=0 mise exec -- go test -C packages/go -count=1 ./...`
    - `go vet -C packages/go ./...`
- **Ruby-only**: `mise run check` + `cargo clippy -p iscc-rb -- -D warnings` +
    `pushd crates/iscc-rb && bundle exec rake test; popd` + `bundle exec standardrb` (needs
    `PATH="$(ruby -e "puts Gem.user_dir")/bin:$PATH"`)
- **Kotlin-only**: `cargo build -p iscc-uniffi` + `cd packages/kotlin && ./gradlew test` + clippy
    workspace + `mise run check`. Gradle flakes on this bind mount — read
    `build/test-results/test/*.xml` and re-run after `./gradlew clean` before calling it a failure.
    If the diff touches the KGP version, run the consumer-floor test in `dep-refresh-reviews.md`.
    Published floor is **Kotlin 2.3 or newer** (documented iter 132 in root README, package README,
    howto + `specs/kotlin-bindings.md` — a bump moving it must update all four in one step)
- **Config/lockfile-only**: `mise run check` + `cargo check -p <crate>` (+ `cargo deny check` if
    deps changed — see gate-reviews.md Audit)
- **Dependency refresh slices (v0.6.0 issue)**: per-slice gate sets + hold-back-verification recipes
    for Cargo.lock (124), uv.lock (125), Rust pins (126), GH Actions (127), JVM (128), Go (129),
    Ruby (130), ruff 0.16 A–D (131/134/135/136, tree now clean under 0.16) + E (the pin drop, watch
    for 0.16 `ruff format` drift) → `dep-refresh-reviews.md`. Never use the lockfile-only shortcut
    on these. **A toolchain/compiler bump inside a PUBLISHED binding is a support-policy change, not
    a pin** — check whether the consumer floor moved (iter 128 recipe) before passing it
- **A published `.pyi` needs mypy + pyright, not just `ty`** (iter 131): the package ships
    `py.typed` beside `_lowlevel.pyi`, so it is consumer-facing. `uvx mypy@1.18.2 --strict` +
    `uvx pyright@1.1.407` ≈ 30s. Prefer an `ast.parse` body assertion over greps for bulk stub edits
- **Verify a `# held:` claim from registry metadata, never from the handoff** (iters 126/130, ~30s):
    `cargo info <crate>@<ver>` (rust-version), `gem specification <gem> -v <ver> --remote`
    (transitive pins), `https://rubygems.org/api/v1/versions/<gem>.json` (`ruby_version`; v2 returns
    null). A hold-back with a *wrong* stated reason survives as folklore — worse than no comment
- **A dep shipping DATA TABLES (Unicode, locale, tz) needs an exhaustive differential, not green
    vectors** (iter 129, `golang.org/x/text`): the 50 vendored vectors are all Unicode ≤ 15. Recipe
    (throwaway module + `replace` to the old version, dump whole input space, `diff`) →
    `dep-refresh-reviews.md` slice 6
- **`cargo tree -i <crate>` prints "nothing to print"** for proc-macro / target-specific deps — add
    `--target all`. Disproved an advance-handoff attribution with it: the `proc-macro-error2 v2.0.1`
    future-incompat warning is from `iai-callgrind-macros` (dev-only), NOT magnus/rb-sys
- **Core text/codec change + generated data (iter 133)**: Rust-only shortcut PLUS the two CI-only
    gates — `mise run coverage` then
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above` (~2
    min), and `mise run bench:iai:check` (~1 min). Re-run any checked-in generator and assert
    `git status --porcelain <output>` is empty. Total ≈ 6 min; catches the CRAP/perf gates that
    `mise run check` does not run
- **Version sync addition**: `mise run check` + `uv run scripts/version_sync.py --check` + clippy
- **Script-only (shell)**: `bash -n <script>` + `mise run check` + clippy (when no Rust changes)
- **release.yml-only (iter 123, #49)**: NOT exercised by CID pushes → static-verify only: YAML parse
    - matrix-entry presence + artifact-name consistency across build→test→publish. Adding a wheel
        target = build+test matrix only (`publish-pypi` collects via `pattern: wheels-*`). `test-*`/
        `publish-*` jobs still lack the `!cancelled()&&!failure()` guard that only `build-*` and
        `publish-crates-io` carry (tracked re-trigger bug, re-verified iter 132) — don't flag as new
- **CI/Audit/Perf/Semver/CRAP gate reviews**: see `gate-reviews.md`
- **Binding propagation (napi/wasm/ffi/jni/ruby/dotnet/kotlin/uniffi)**: see `binding-reviews.md`
- Cross-platform CI: bash syntax needs `shell: bash` if matrix includes Windows

## Codex Review Integration

- Codex findings are advisory — cross-reference with your own analysis. Use `--commit HEAD` (verify
    with `git log` first). A clean/empty verdict is a note, never grounds for NEEDS_WORK
- Go codec findings: dismiss — Go mirrors Rust reference faithfully. `dotnet-version: '8.0'` is
    valid for `setup-dotnet@v4` — dismiss "use `8.0.x`". Large generated Kotlin/Swift diffs confuse
    it — advisory only
- **Trust Codex on dependency-internals findings** (117-118): it correctly read blake3's feature
    gating and `#[target_feature]` attributes. When it cites a dep's build.rs / feature wiring,
    check the dep source (and build it) before dismissing
- **It catches downstream-consumer breakage local gates cannot** (128, the KGP consumer-floor raise)
    **and input-validation edge cases** (119, trailing-byte decode acceptance). Convergence with my
    own suspicion is the cue to VERIFY EMPIRICALLY — Codex reports the risk, only the experiment
    gives the number; then characterize regression-vs-pre-existing
