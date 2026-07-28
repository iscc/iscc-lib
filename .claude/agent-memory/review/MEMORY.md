# Review Agent Memory

Concise index — **one line per entry, detail belongs in a topic file.** `review-patterns.md` (docs,
verification, issues, gotchas, claim-probing, gate scripts, fixture oracles, parity gates),
`gate-reviews.md` (CI structure + Audit/Perf/Semver/CRAP), `binding-reviews.md` (per-binding
commands + `-Xcheck:jni`; JNI natives CLOSED 33/33 at 169, propagation 12/12 at 161),
`dep-refresh-reviews.md` (v0.6.0 slices, framework and build-tool-wrapper majors),
`gha-workflow-reviews.md` (release.yml, action bumps, pinning), `unicode-reviews.md` (freeze rule,
sweep + differential gates, boundary vectors), `codex-integration.md` (weighing a finding). Stale
entries in `MEMORY-archive.md`.

## Quality Gate Details

- `mise run check` = the pre-commit hooks (hygiene, formatting, lint incl. Ruby, release-workflow
    static checks) — never assert the hook count, it drifts. Pre-push hooks (clippy, cargo test,
    pytest, `ty`, ruff `S`/`C901`) are NOT in it — verify clippy with
    `cargo clippy --workspace --all-targets -- -D warnings`. Added files must be ≤256kb
- **Pre-push needs `iscc_lib` built** (`ty check`/`pytest` import it) — run
    `uv run maturin develop --release` in `crates/iscc-py` BEFORE `git push` → `binding-reviews.md`
- Proper scoping, NOT circumvention: `[tool.ty.src] exclude` for Python importing non-venv packages
    (`conanfile.py`), and `[tool.ruff.lint.per-file-ignores]` waiving `S101`/`S603`/`S607` in tests
- **After ANY step-9 edit to a `.py` file, re-run the PRE-PUSH gates** (`uv run ty check`, ruff
    `S`/`C901`) — not in `mise run check`, so a defect surfaces only at `git push` (157: attribute
    assignment on an importlib-loaded module; use `monkeypatch.setattr`)

## Common Issues

- Verification greps false-positive: `grep -qv 'pat'` is always true on multi-line files (use
    `! grep -q`). **Substring `grep -c` and exact counts in next.md are a recurring mis-spec class**
    (139) — test the criterion against `HEAD~1` before believing advance broke it
- **next.md's Implementation Notes are a HYPOTHESIS — algorithms and prose alike** (142 an algorithm
    that fails on real data, 143 false docs claims, 149 a mislabelled oracle). Re-derive every
    number, test spec, expected value and count from source → `review-patterns.md` "Claim-Probing"
- `iscc_decompose` returns units WITHOUT "ISCC:" prefix — cross-check doc examples. **Docs site
    URL** is `https://lib.iscc.codes/`, NOT `https://iscc-lib.iscc.io/` (advance gets this wrong)
- **A `#[deprecated]` claim in next.md is a hypothesis like any other** (168) — read the attribute
    in `~/.cargo/registry/src/index.crates.io-*/<crate>-<ver>/src/`; jni 0.22.4 never deprecated
    `Env::byte_array_from_slice`, so a "must not appear" grep criterion cost a zero-copy path
    (restored 169)
- **Advance agent idle claims**: verify remaining issue priorities independently — it may claim
    "only low-priority remain" when `normal` issues still exist
- Go `// indirect`, CI `find` cross-arch, prek stash conflict, and mode-only commits under
    `core.fileMode=false` → `review-patterns.md` "Gotchas"
- **Writing `\uXXXX` through Edit/Write decodes it to literal UTF-8** (149) — edit ASCII-escaped
    files via Python (`isascii()` + numeric `ord()`); same trap in **bash**, so run such a criterion
    from a quoted heredoc before calling it failed (159)
- **`mise run check` mdformat reformats context files** (intermittent): a non-conforming `next.md` /
    `MEMORY.md` is NOT an advance regression, but pre-push mdformat WILL reject the batch — check
    `git status --porcelain` right after and STAGE it. Never stage `iterations.jsonl`; keep inline
    code spans on ONE line
- **next.md may task advance with an `issues.md` append (135) or REVIEW with a spec edit (144) —
    both protocols forbid it.** advance puts the paragraph in its handoff **Notes** for me to
    append; I may edit a spec only when resolving a `[human]` issue carrying `**Spec:**`
- **No-op / human-handoff iteration (111)**: verify scope is empty, still scan `@{upstream}..HEAD`;
    HUMAN-REVIEW spec amendments + `low` left → **HUMAN REVIEW REQUESTED**, never `**IDLE**`
    (all-`low` only). Verdict still PASS; push the batch
- **Unicode freeze-rule work has its own playbook → `unicode-reviews.md`** — read it before any diff
    under `utils/unicode16*`, `unicode_boundary.json` or `scripts/gen_unicode16_*`: `U+FFFF`
    sentinel contract, no bare `.to_lowercase()`, the `mise run unicode:sweep` gate (a bare script
    run REFUSES since 158), propagation COMPLETE at 161, "a binding can pass for the WRONG reason"
- **"files were modified by this hook" often means SOMETHING ELSE WAS WRITING** (97/164): a second
    CID loop (confirm `ps aux` → HUMAN REVIEW REQUESTED, do not push or kill), or my own Edits
    landing while a backgrounded prek run held its snapshot. Never edit while prek runs

## Review Shortcuts (detail in the topic files named above)

- **Rust-only**: `cargo test -p iscc-lib` + clippy workspace + `mise run check`. Clippy workspace is
    ~2s after a build — always run it
- **Test-fixture / vector-file only (141/149, ~5 min)**: Rust-only PLUS the full feature matrix
    (`#[cfg]` gating picks the tests — assert the per-target `N passed`), then the four fixture
    mutations → `review-patterns.md`
- **Docs-only**: `mise run check` + clippy + `mise run version:check` (21 `OK:` lines) +
    `uv run zensical build` ("No issues found") + rendered-HTML grep for admonition/tab edits.
    **Green gates say NOTHING about truth** (143) → `review-patterns.md` "Docs Claim-Checking"
- **New docs PAGE (gated since 145)**: `uv run scripts/check_docs_nav.py` proves disk / nav /
    `ORDERED_PAGES` / `docs/llms.txt` agree (23 pages); holes: commented-out nav entries count as
    present, deletions skipped. `zensical build` **wipes `site/`** → run `gen_llms_full.py` after it
- **Python-only**: `mise run check` + `pytest`; a new `.py` file adds the pre-push `ty`/ruff gates
- **New gate script, gate HARDENING, or spec↔config PARITY gate (142–146/163, ~10 min)**: never
    accept "it exits 0 at HEAD" — write your OWN mutations (cheapest **real** regression:
    `git show HEAD~1:<path>`), probe both set-equality blind spots (equal-*empty* vacuous pass,
    **duplicated** row), and give a `files:`-scoped hook a staged probe → `review-patterns.md`
- **DIFFERENTIAL gate (157/158) / vendored DERIVED-property table (156), ~25 min each**: a
    case-count pin catches a shrunken set, never a swapped one; drive `main()` in-process →
    `unicode-reviews.md`
- **Repo-state gate that reads the git index (152, ~8 min)**: Python-only PLUS mutations in a
    THROWAWAY repo (`git archive HEAD | tar -x -C /tmp/x && git init`; `git clone` fails here on
    `safe.directory`) — never mutate the real index → `review-patterns.md`
- **Lint-config-only (`[tool.ruff]*` / prek hook types, 134–139)**: `mise run check`, ruff check +
    `format --check` (**assert exit 0, never a file count**), both pre-push ruff gates, ty, pytest
- **Go-only**: `mise run check`, `CGO_ENABLED=0 mise exec -- go test -C packages/go -count=1 ./...`,
    `go vet -C packages/go ./...`, `mise exec -- gofmt -l packages/go` (empty)
- **Binding boundary-fixture slices (150–161) — CLOSED, all 12 suites gated.** Commands, mutations
    and standing traps (`cargo test -p iscc-wasm` = 0 tests; the **Gradle UP-TO-DATE stale green**;
    verify a generated artifact by **decoding it back**) → `binding-reviews.md`
- **Every "not locally verifiable" toolchain claim has been FALSE so far** — `cmake` (160), `swift`
    via swift.org's Debian 12 tarball (161), the release-only Gradle publish (165). Probe before
    repeating; CMake sets no `-Wall`, so back "no warning" with `g++ -Wall -Wextra -Wpedantic -c`
- **Ruby-only / Kotlin-only / published-`.pyi`** command sets → `binding-reviews.md` "Per-binding
    review commands" (Gradle flakes on this bind mount; Kotlin consumer floor is **2.3 or newer**)
- **Config/lockfile-only**: `mise run check` + `cargo check -p <crate>` (+ `cargo deny check` if
    deps changed — see `gate-reviews.md` Audit)
- **Dependency refresh (v0.6.0)**: per-slice gates, hold-backs, reflexes → `dep-refresh-reviews.md`.
    **Nine slices + xunit.v3 + Gradle 9.6.1 + JUnit 6.1.2 + magnus 0.8 + jni 0.22 CLOSED**; left:
    `release.yml` actions (uniffi 0.32 / criterion 0.8 are human-gated). A toolchain bump in a
    PUBLISHED binding moves the **consumer floor** — re-derive it from the crate's own
    README/`Cargo.toml`, never from a `# held:` comment (167: the comment claimed a Ruby 3.2+ floor
    magnus 0.8 never had); a DATA-TABLE dep needs an exhaustive differential (all 50 vendored
    vectors predate Unicode 16). **Prove a gitignored native artifact was rebuilt against the
    bump**: `strings <artifact> | grep -o '<dep>-[0-9.]*'`
- **Build-tool WRAPPER major / any committed BINARY blob (165, ~15 min)**: never review it by
    reading the diff — pin the blob to the publisher's checksum AND regenerate it in a throwaway
    dir, then diff every artifact; also probe the release-only publish task →
    `dep-refresh-reviews.md`
- **Test-FRAMEWORK major (164 xunit, 166 JUnit 6, ~10 min)**: the "≥ N passed" floor in next.md
    proves nothing — demand the **same** total as the pre-bump tree (`git archive HEAD~1`), or
    cheaper, derive it from the FIXTURE (166: surefire dynamic-case counts == `data.json`
    per-function vector counts + static `@Test` count). Also resolve the dependency graph
    (`gradlew dependencies --configuration testRuntimeClasspath`) and force a cold recompile —
    `mvn test` reuses stale test classes ("Nothing to compile"), only `mvn clean test` proves the
    new framework COMPILES → `dep-refresh-reviews.md`
- **Prek-hook-scope review (138–139)**: NEVER accept `git ls-files` arithmetic as a hook's surface —
    `.pyi` is tagged `pyi`, not `python`; a *widened* tag needs a **staged, dirty** file probe
- **Core text/codec change + generated data (133/148, ≈12 min)**: Rust-only PLUS the **full feature
    matrix** PLUS the two CI-only gates — `mise run coverage` then the CI-exact `cargo crap` line
    (BARE `--fail-above`; CI uses BOTH flags) and `mise run bench:iai:check`; re-run any generator,
    assert `git status --porcelain <output>` is empty, + the sequence differential →
    `review-patterns.md`
- **Version sync**: + `version_sync.py --check`. **Shell script**: + `bash -n <script>`
- **Action *pin* change (162, ~6 min)**: the four static gates + actionlint, then probe the pin at
    the source — `git/matching-refs/tags` (does `@vN` exist at all?) and `compare/<tag>...main`
    `.ahead_by` (162: `@main` was 31 commits + a rebuilt `dist/` ahead) → `gha-workflow-reviews.md`
- **release.yml / any GHA action bump**: NEVER exercised by CID pushes → static-verify only. Run the
    committed gates `uv run scripts/check_release_workflow.py` and `… --check-action-inputs`, never
    a retyped heredoc; **zero `warning: skipped` lines is part of the pass**; `always()` =
    NEEDS_WORK. Manual checks (actionlint, `runs.using`, majors' defaults) →
    `gha-workflow-reviews.md`
- **Pre-flight `uv run prek run --hook-stage pre-push --all-files` before pushing a batch carrying
    never-CI'd code** (exit 0 ⇒ `git push` will not be rejected; needs `iscc_lib` built). Run it
    **last and sequentially** — see the "files were modified" entry above
- **CI/Audit/Perf/Semver/CRAP gates** → `gate-reviews.md`

## Codex Review Integration

- Advisory only, never sets the verdict; `--commit HEAD`; empty/unavailable ≠ clean. Strengths,
    blind spots, dismiss-list, the 3-step **how to weigh a finding** → **`codex-integration.md`**.
    Expect a real finding wherever a green run proves less than it appears to: matching/parsing
    rules, exception contracts, user-facing factual claims, build-config lines (154 Gradle)
- **The background Codex run executes real build/test commands in the SAME tree** (164
    `dotnet   build`/`test`, 165 a full Gradle build) — a build tool that fails once while Codex
    runs may just be losing a race on `bin/`/`obj/`/`build/`. Re-run sequentially before calling it
    a defect
