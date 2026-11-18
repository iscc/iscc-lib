# Review Agent Memory

Concise index — **one line per entry, detail belongs in a topic file.** `review-patterns.md` (docs,
verification, claim-probing, gate scripts, parity gates), `gate-reviews.md` (CI +
Audit/Perf/Semver/CRAP), `binding-reviews.md` (per-binding cmds + `-Xcheck:jni`; JNI CLOSED 33/33 at
169), `dep-refresh-reviews.md` (v0.6.0 slices COMPLETE at 172 + framework/wrapper/lockfile/ regen
majors), `gha-workflow-reviews.md` (release.yml, action bumps), `codex-integration.md`;
removed-feature history in `archive/`. Stale → `MEMORY-archive.md`.

## Quality Gate Details

- `mise run check` = the pre-commit hooks (hygiene, formatting, lint incl. Ruby, release-workflow) —
    never assert the hook count, it drifts. Pre-push hooks (clippy, cargo test, pytest, `ty`, ruff
    `S`/`C901`) are NOT in it — verify clippy with
    `cargo clippy --workspace --all-targets -- -D warnings`. Added files must be ≤256kb
- **`mise run bench:iai:check` is NOT in `mise run check`** — run it on ANY benchmarked hot-path
    change (191's +36% slipped past 192 into red CI) → `gate-reviews.md`
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
- **A docs-only sweep can FABRICATE a per-surface return type** (190); a `#[deprecated]` claim is a
    hypothesis (168) → `review-patterns.md` "Moved from the index"
- **On ANY codec input-cleaning change, probe empty/garbage inputs** (191: cleaned `""` →
    `iscc_decompose` `Ok([])`) → `review-patterns.md`
- **On ANY canonical-form/decode change, probe non-ASCII case-fold aliases** (199: `ı`/`ſ`/`ß`/`ﬁ`
    upper-case to ASCII, so Rust+Go alias codes the ASCII-only ref `b32decode` rejects; a re-encode
    compare after `ToUpper` cannot see it). Probe via the Python binding — build it first
- **"The fix matches the reference" — probe EVERY entry point it touches** (201: ref `iscc_decode`
    rejects `ı`/`ſ`, ref `encode_seq` `.upper()`s first and ACCEPTS — the issue + next.md missed it,
    Codex caught it). A new divergence → pin it in `STRICT_BASE32_DIVERGENCES`, file a `**Spec:**`
    issue, escalate (spec exception lists are acceptance criteria)
- **next.md can under-scope the spec's "Verified when" criterion** (197): it asked only for the seq
    base32 divergence, while `rust-core.md` demands "both sides reject the same inputs". Read the
    feature's spec checklist too; a missing cheap differential test → add it in review (step 6)
- **Python-reference differentials: Python `strip()`/`split()` ⊋ Rust `trim`/`split_whitespace`** by
    U+001C–U+001F: codec (198/200) + text utils (203) fixed in Rust AND Go. A one-way sweep proves
    only ⊇ — demand the reverse (live `Z*`/`Cc`/`Cf` − strip set, 0.6s); `binascii.Error` ⊂
    `ValueError`
- **A metamorphic "display forms" suite: demand one form per trim/clean CALL SITE** (200: Codex saw
    the outer-only separator forms miss the 2 post-colon trims; mutation in a `/tmp` copy of
    `packages/go` — conformance tests need `../../crates`, so `-run <family>` only)
- **Scope cap: `.crap-baseline.json` is a GENERATED artifact, not a counted file** (3 src + baseline
    passed at 195 `5d2840e` and 203); a refresh with only `line:` shifts is correct, not a deviation
- **A Go `Fields`→`FieldsFunc` swap costs ~2× (ASCII fast path) and allocs only past 32 fields** —
    an `AllocsPerRun` probe needs >32 fields to see it; accepted in decisions.md (203)
- **Advance agent idle claims**: verify remaining issue priorities independently — it may claim
    "only low-priority remain" when `normal` issues still exist
- **Run the full CI feature matrix on any core change** — only `--no-default-features` catches a
    feature-gated `#[test]` compile break (174) → `review-patterns.md`
- **Probe `MDFZAAAAAAAAAAAAAA` on any codec header-gate change** (174, wrap) → `review-patterns.md`
- Go `// indirect`, CI `find` cross-arch, prek stash conflict, and mode-only commits under
    `core.fileMode=false` → `review-patterns.md` "Gotchas"
- **`\uXXXX` decodes to literal UTF-8 through Edit/Write and bash**, and **who may edit `issues.md`
    / a spec** is protocol-fixed (135/144/172) → `review-patterns.md` "Artifact-editing traps"
- **`mise run check` mdformat reformats context files** (intermittent): not an advance regression,
    but pre-push mdformat WILL reject the batch — check `git status --porcelain` right after and
    STAGE it. Never stage `iterations.jsonl`; keep inline code spans on ONE line
- **No-op / human-handoff iteration** (111/173): verdict PASS; `low`-only + human spec work or an
    upstream-gated `normal` → HUMAN REVIEW REQUESTED, never `**IDLE**` → `review-patterns.md`
- **Unicode freeze rule REMOVED 2026-07-30** — reintroducing any freeze mechanism = NEEDS_WORK
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
    `archive/unicode-reviews.md`
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
- **Every "not locally verifiable" toolchain claim has been FALSE so far** (cmake, swift, Gradle
    publish, MSRV) → `binding-reviews.md` "Moved from the index"
- **Ruby-only / Kotlin-only / published-`.pyi`** command sets → `binding-reviews.md` "Per-binding
    review commands" (Gradle flakes on this bind mount; Kotlin consumer floor is **2.3 or newer**)
- **Config/lockfile-only**: `mise run check` + `cargo check -p <crate>` (+ `cargo deny check` if
    deps changed — see `gate-reviews.md` Audit)
- **CI-restore (advisory ignore + runner-label swap, 196, ~10 min)**: prove an ignore's "no upgrade"
    claim at crates.io (`gate-reviews.md` Audit), runner arch from the `actions/runner-images`
    README, then grep `specs/` + per-crate `CLAUDE.md` for the old label
- **A GENERATED-artifact bump** (172): regenerate into `/tmp`, never read the diff →
    `dep-refresh-reviews.md`
- **An MSRV claim is locally decidable**: `cargo +1.85.0 check -p <crate> --locked` →
    `dep-refresh-reviews.md` "Moved from the index"
- **Other bump types** (lockfile, wrapper/binary blob, test-framework major) →
    `dep-refresh-reviews.md`
- **Prek-hook-scope review (138–139)**: NEVER accept `git ls-files` arithmetic as a hook's surface —
    `.pyi` is tagged `pyi`, not `python`; a *widened* tag needs a **staged, dirty** file probe
- **Core text/codec change + generated data (≈12 min)**: Rust-only + full feature matrix +
    coverage/CRAP + `bench:iai:check` + generator no-op + pre-push `ty`/ruff/pytest on any new `.py`
    test (203, ~20 min) → `gate-reviews.md`
- **Version sync**: + `version_sync.py --check`. **Shell script**: + `bash -n <script>`
- **Action *pin* change (162, ~6 min)**: four static gates + actionlint, then probe the pin at
    source — `git/matching-refs/tags` (does `@vN` exist?) + `compare/<tag>...main` `.ahead_by` (162:
    `@main` was 31 commits ahead) → `gha-workflow-reviews.md`
- **release.yml / any GHA action bump**: NEVER exercised by CID pushes → static-verify only via the
    committed gates `scripts/check_release_workflow.py` + `… --check-action-inputs` (never a
    heredoc); zero `warning: skipped` is part of the pass; `always()` = NEEDS_WORK →
    `gha-workflow-reviews.md`
- **Pre-flight `uv run prek run --hook-stage pre-push --all-files` before pushing never-CI'd code**
    (exit 0 ⇒ `git push` will not be rejected; needs `iscc_lib` built). Run it last and
    sequentially, with `run_in_background` — `--all-files` also runs `standardrb`, ~15 min total on
    the bind mount (200)
- **CI/Audit/Perf/Semver/CRAP gates** → `gate-reviews.md`
- **ISCC-SEQ surface port (202, ~20 min)**: surface tests + corpus differential vs `iscc_lib` +
    `test_seq.rs` name diff + drop-SEMANTIC mutation (no vector carries one) → `binding-reviews.md`
- **decisions.md over budget → rotate with `cid.archive_overflow`**, never hand-compress (command in
    `gate-reviews.md`); stage `decisions-archive.md` too (195)

## Codex Review Integration

- Advisory only, never sets the verdict; `--commit HEAD`; empty/unavailable ≠ clean. Strengths,
    blind spots, dismiss-list, the 3-step **how to weigh a finding** → **`codex-integration.md`**.
    Expect a real finding wherever a green run proves less than it appears to: matching/parsing
    rules, exception contracts, user-facing factual claims, build-config lines (154), and any
    declared-but-ungated fact such as MSRV (172)
- **`--commit HEAD` reviews ONLY the last commit** — when advance splits its work (195: main commit
    \+ a docs-only fixup), `git log @{upstream}..HEAD` first and launch a 2nd Codex run on the main
    commit (`--commit <sha>`, own output file); every real 195 finding came from that run
- **The background Codex run executes real build/test commands in the SAME tree** (164 dotnet, 165 +
    172 Gradle) — a build tool failing once while Codex runs may be losing a race on
    `bin/`/`obj/`/`build/`. Re-run sequentially; hold any Gradle `clean` until it exits
