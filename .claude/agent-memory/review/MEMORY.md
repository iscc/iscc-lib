# Review Agent Memory

Concise index — **one line per entry, detail belongs in a topic file.** `review-patterns.md` (docs,
verification, claim-probing, gate scripts, parity gates), `gate-reviews.md` (CI +
Audit/Perf/Semver/CRAP), `binding-reviews.md` (per-binding cmds + `-Xcheck:jni`; JNI CLOSED 33/33 at
169), `dep-refresh-reviews.md` (v0.6.0 slices COMPLETE at 172 + framework/wrapper/lockfile/ regen
majors), `gha-workflow-reviews.md` (release.yml, action bumps), `unicode-reviews.md` (freeze rule,
sweep + differential gates, boundary vectors), `codex-integration.md`. Stale → `MEMORY-archive.md`.

## Quality Gate Details

- `mise run check` = the pre-commit hooks (hygiene, formatting, lint incl. Ruby, release-workflow) —
    never assert the hook count, it drifts. Pre-push hooks (clippy, cargo test, pytest, `ty`, ruff
    `S`/`C901`) are NOT in it — verify clippy with
    `cargo clippy --workspace --all-targets -- -D warnings`. Added files must be ≤256kb
- **`mise run bench:iai:check` is NOT in `mise run check`** (CI-only enforcing Perf gate): run it
    explicitly on ANY benchmarked hot-path change (codec `iscc_clean`/decode, text utils, cdc) —
    iter 191's +36% four_units regression slipped past a green-`check` review (192) into RED CI. A
    *correctness* change can raise iai cost irreducibly (191/192 reference-correct clean = 2 std
    memchr scans); removing allocs won't restore a baseline set for cheaper-but-wrong code → the fix
    is a deliberate baseline regen (human/gate, escalate) → detail `gate-reviews.md`
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
- **A docs-only sweep can FABRICATE a per-surface return type** (190): grep each binding's
    `src/lib.rs` for the ACTUAL return type of any newly-documented symbol, not the CLAUDE.md/howto
    prose (wasm `gen_iscc_id_v1` returns bare `Result<String,JsError>`, not an invented struct;
    Codex catches these). A `#[deprecated]`/attribute claim in next.md is a hypothesis too — read it
    in `~/.cargo/registry/src/index.crates.io-*/<crate>-<ver>/src/` (168: jni 0.22.4 never
    deprecated `Env::byte_array_from_slice`; that grep criterion cost a zero-copy path)
- **On ANY codec input-cleaning change, probe empty/garbage inputs** (191): a helper that yields
    `""` (blank/`"-"`/`"iscc:"`) makes `iscc_decompose` silently return `Ok([])`
    (`decode_base32("")` is `Ok(empty)`, not an error) — a Tier-1 silent-acceptance regression
    valid-input differential tests miss. Compare against `HEAD~1` + reference; 192 added the empty
    guard (learnings)
- **Advance agent idle claims**: verify remaining issue priorities independently — it may claim
    "only low-priority remain" when `normal` issues still exist
- **A `--no-default-features` CI step catches COMPILE breaks the default gates miss** (174): a
    `#[test]` calling a feature-gated fn fails to *compile* under
    `cargo test -p iscc-lib --no-default-features`; gate the test `#[cfg(feature = "meta-code")]`.
    Run the full CI feature matrix on any core change — `mise run check` and a default `cargo test`
    are both blind to it
- **Probe a WRAPPED multi-nibble header on any codec header-gate change** (174): `decode_header`
    casts varnibbles `as u8` before `TryFrom`, so `MDFZAAAAAAAAAAAAAA` wraps to `Id`/V1 and
    `iscc_decompose` silently rewrites it to `MAIAAAAAAAAAAAAA` (`iscc_core` rejects it) → learnings
- Go `// indirect`, CI `find` cross-arch, prek stash conflict, and mode-only commits under
    `core.fileMode=false` → `review-patterns.md` "Gotchas"
- **`\uXXXX` decodes to literal UTF-8 through Edit/Write and bash**, and **who may edit `issues.md`
    / a spec** is protocol-fixed (135/144/172) → `review-patterns.md` "Artifact-editing traps"
- **`mise run check` mdformat reformats context files** (intermittent): not an advance regression,
    but pre-push mdformat WILL reject the batch — check `git status --porcelain` right after and
    STAGE it. Never stage `iterations.jsonl`; keep inline code spans on ONE line
- **No-op / human-handoff iteration (111)**: verify scope is empty, still scan `@{upstream}..HEAD`;
    HUMAN-REVIEW spec amendments + `low` left → **HUMAN REVIEW REQUESTED**, never `**IDLE**`
    (all-`low` only). Verdict still PASS. Same call when the only `normal` left is trigger-gated on
    an upstream release (173, go1.27): escalate rather than burn a `Step: NONE` cycle
- **Unicode freeze-rule work has its own playbook → `unicode-reviews.md`** — read before any diff
    under `utils/unicode16*`, `unicode_boundary.json`, `scripts/gen_unicode16_*`: `U+FFFF` sentinel,
    no bare `.to_lowercase()`, `mise run unicode:sweep`, "a binding can pass for the WRONG reason"
- **"files were modified by this hook" often means SOMETHING ELSE WAS WRITING** (97/164): a second
    CID loop (confirm `ps aux` → HUMAN REVIEW REQUESTED, do not push or kill), or my own Edits
    landing while a backgrounded prek run held its snapshot. Never edit while prek runs

## Review Shortcuts (detail in the topic files named above)

- **Rust-only**: `cargo test -p iscc-lib` + clippy workspace + `mise run check`. Clippy workspace is
    ~2s after a build — always run it
- **Test-fixture / vector-file only (141/149, ~5 min)**: Rust-only PLUS the full feature matrix
    (assert the per-target `N passed`), then the four fixture mutations → `review-patterns.md`
- **Docs-only**: `mise run check` + clippy + `mise run version:check` (21 `OK:`) +
    `uv run zensical build` ("No issues found") + rendered-HTML grep for admonition/tab edits.
    **Green gates say NOTHING about truth** (143) → `review-patterns.md`. A new PAGE adds
    `scripts/check_docs_nav.py` (disk/nav/`ORDERED_PAGES`/`docs/llms.txt`, 23 pages);
    `zensical build` wipes `site/` → `gen_llms_full.py` after it
- **Python-only**: `mise run check` + `pytest`; a new `.py` file adds the pre-push `ty`/ruff gates
- **New gate script, gate HARDENING, or spec↔config PARITY gate (142–146/163, ~10 min)**: never
    accept "exits 0 at HEAD" — write your OWN mutations (cheapest real regression:
    `git show HEAD~1:<path>`), probe both set-equality blind spots (equal-empty vacuous pass,
    duplicated row), give a `files:`-scoped hook a staged probe → `review-patterns.md`
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
- **Binding boundary-fixture slices (150–161) — CLOSED, all 12 suites gated.** Commands, mutations,
    standing traps (`cargo test -p iscc-wasm` = 0 tests; Gradle UP-TO-DATE stale green; verify a
    generated artifact by decoding it back) → `binding-reviews.md`
- **Every "not locally verifiable" toolchain claim has been FALSE so far** — `cmake` (160), `swift`
    via swift.org's Debian 12 tarball (161; `/tmp/swifttc` gone between sessions, re-download ~5min
    — a pure uniffi *regen* like 186 accepts Swift by transitivity: byte-identical regen + Rust
    golden
    - Codex's Kotlin run), Gradle publish (165), MSRV (172). CMake sets no `-Wall` → back "no warning"
        with `g++ -Wall -Wextra -c`
- **Ruby-only / Kotlin-only / published-`.pyi`** command sets → `binding-reviews.md` "Per-binding
    review commands" (Gradle flakes on this bind mount; Kotlin consumer floor is **2.3 or newer**)
- **Config/lockfile-only**: `mise run check` + `cargo check -p <crate>` (+ `cargo deny check` if
    deps changed — see `gate-reviews.md` Audit)
- **A GENERATED-artifact bump (uniffi 0.32, 172, ~15 min)**: never read the generated diff —
    regenerate into `/tmp`, compare modulo `sed 's/[[:space:]]*$//'` + EOF trim (a literal
    `git status` no-op is unachievable), diff the `^public`/`^open` lists → `dep-refresh-reviews.md`
- **An MSRV claim is locally decidable**: `cargo +1.85.0 check -p <crate> --locked` (1.85
    installed). Source-build floor = max `rust_version` over the non-dev resolve graph
    (`cargo metadata --locked`; `cargo tree -i <dep> -e no-dev --target all` empty ⇒ dev-only). A
    normal-dep major can raise it transitively (172 uniffi 0.32 → `cargo-platform` 1.91 broke
    `iscc-uniffi`, `iscc-lib` untouched, accepted); a per-crate override then makes cargo say
    `requires rustc X`. Bench-harness/consumer/ data-table/native-artifact variants →
    `dep-refresh-reviews.md`
- **Other bump-type shortcuts → `dep-refresh-reviews.md`**: committed-lockfile / `--locked-mode`
    (170, warm `dotnet restore` validates NOTHING); build-tool WRAPPER / BINARY blob (165, checksum
    \+ independent regeneration, never the diff); test-FRAMEWORK major (164/166, demand the pre-bump
    total; only `mvn clean test` proves the new framework compiles)
- **Prek-hook-scope review (138–139)**: NEVER accept `git ls-files` arithmetic as a hook's surface —
    `.pyi` is tagged `pyi`, not `python`; a *widened* tag needs a **staged, dirty** file probe
- **Core text/codec change + generated data (133/148, ≈12 min)**: Rust-only PLUS the **full feature
    matrix** PLUS the two CI-only gates (`mise run coverage` then CI-exact `cargo crap` with BARE
    `--fail-above`, and `mise run bench:iai:check`); re-run generators (`git status` empty) +
    sequence diff
- **Version sync**: + `version_sync.py --check`. **Shell script**: + `bash -n <script>`
- **Action *pin* change (162, ~6 min)**: four static gates + actionlint, then probe the pin at
    source — `git/matching-refs/tags` (does `@vN` exist?) + `compare/<tag>...main` `.ahead_by` (162:
    `@main` was 31 commits ahead) → `gha-workflow-reviews.md`
- **release.yml / any GHA action bump**: NEVER exercised by CID pushes → static-verify only via the
    committed gates `scripts/check_release_workflow.py` + `… --check-action-inputs` (never a
    heredoc); zero `warning: skipped` is part of the pass; `always()` = NEEDS_WORK →
    `gha-workflow-reviews.md`
- **Pre-flight `uv run prek run --hook-stage pre-push --all-files` before pushing never-CI'd code**
    (exit 0 ⇒ `git push` will not be rejected; needs `iscc_lib` built). Run it last and sequentially
- **CI/Audit/Perf/Semver/CRAP gates** → `gate-reviews.md`

## Codex Review Integration

- Advisory only, never sets the verdict; `--commit HEAD`; empty/unavailable ≠ clean. Strengths,
    blind spots, dismiss-list, the 3-step **how to weigh a finding** → **`codex-integration.md`**.
    Expect a real finding wherever a green run proves less than it appears to: matching/parsing
    rules, exception contracts, user-facing factual claims, build-config lines (154), and any
    declared-but-ungated fact such as MSRV (172)
- **The background Codex run executes real build/test commands in the SAME tree** (164 dotnet, 165 +
    172 Gradle) — a build tool failing once while Codex runs may be losing a race on
    `bin/`/`obj/`/`build/`. Re-run sequentially; hold any Gradle `clean` until it exits
