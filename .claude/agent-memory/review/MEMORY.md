# Review Agent Memory

Concise index — **one line per entry, detail belongs in a topic file.** `review-patterns.md` (docs,
verification, claim-probing, gate scripts, parity gates), `gate-reviews.md` (CI +
Audit/Perf/Semver/CRAP), `binding-reviews.md` (per-binding cmds + `-Xcheck:jni`; JNI CLOSED 33/33 at
169), `dep-refresh-reviews.md` (v0.6.0 slices COMPLETE at 172 + framework/wrapper/lockfile/ regen
majors), `gha-workflow-reviews.md` (release.yml, action bumps), `codex-integration.md`; Stale →
`MEMORY-archive.md`.

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
- **Codec/string-input differentials** — probe non-ASCII case-fold aliases, EVERY reference entry
    point (201), strip/split ⊋ trim by U+001C–U+001F both ways (198–203) → `review-patterns.md`
    "Codec probes"
- **next.md under-scoping a spec's "Verified when"**, display-form suites, `.crap-baseline.json`
    scope cap, Go `FieldsFunc` cost (197–203) → `review-patterns.md` "Moved from the index (204)"
- **6-file cap counts files whose CODE changes** (225, decisions.md): doc-comment-only binding edits
    \+ regenerated outputs don't count, IF next.md lists them as such — confirm from the diff
- **Core behaviour change with flipped binding tests (225, ~25 min)**: Rust feature matrix, Go,
    pytest, napi build+test, `wasm-pack test`, `mvn clean test`, `dotnet test`, cbindgen no-op, CRAP
    gate; run the suites from ONE logged script. grep docs for signatures the step makes relevant
    (`docs/howto/rust.md` had two missing `IsccResult`)
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
    STAGE it. Never stage `iterations.jsonl`; keep inline code spans on ONE line. **Never format
    with bare `uv run mdformat`** (221: other wrap + list numbering than the hook) — use
    `uv run prek run mdformat --files <f>`
- **No-op / human-handoff iteration** (111/173): verdict PASS; `low`-only + human spec work or an
    upstream-gated `normal` → HUMAN REVIEW REQUESTED, never `**IDLE**` → `review-patterns.md`
- **Unicode freeze rule REMOVED 2026-07-30** — reintroducing any freeze mechanism = NEEDS_WORK
- **`git push` failing "index.lock: File exists" while prek stashes is a WSL2 race, NOT a hook
    rejection** (207): no check ran — confirm no 2nd loop in `ps aux`, then retry once
- **"files were modified by this hook" often means SOMETHING ELSE WAS WRITING** (97/164): a second
    CID loop (confirm `ps aux` → HUMAN REVIEW REQUESTED, do not push or kill), or my own Edits
    landing while a backgrounded prek run held its snapshot. Never edit while prek runs
- **Spec paths are gated by `scripts/check_spec_paths.py` (224)** — a "mechanism does not exist"
    realignment is a hypothesis: grep package recipes (Conan `pkg_config_name`/`cmake_file_name`
    generate `.pc`/config at install time; 224 advance + next.md both missed it)
- **Spec-hygiene (phantom paths: check TREES vs `git ls-files`, 223), multi-surface docs fan-out
    (211), issues.md at budget, uncommitted human edits mid-review (212)** → `review-patterns.md`
    "Moved from the index (223)"

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
- **Python-only**: `mise run check` + `pytest`; a new `.py` file adds the pre-push `ty`/ruff gates.
    Python docstrings exist 3× (wrapper, PyO3 `///` → `_lowlevel.X.__doc__`, `.pyi`): rebuild with
    `maturin develop` before any `__doc__` test (222, ~3 min)
- **New gate script, gate HARDENING, or spec↔config PARITY gate (142–146/163, ~10 min)**: never
    accept "exits 0 at HEAD" — write your OWN mutations (cheapest real regression:
    `git show HEAD~1:<path>`), probe both set-equality blind spots (equal-empty vacuous pass,
    duplicated row), give a `files:`-scoped hook a staged probe → `review-patterns.md`
- **Differential / repo-state / lint-config / prek-scope / CI-restore reviews** (134–196) →
    `review-patterns.md` "Moved from the index (204)"
- **Go-only**: `mise run check`, `CGO_ENABLED=0 mise exec -- go test -C packages/go -count=1 ./...`,
    `go vet -C packages/go ./...`, `mise exec -- gofmt -l packages/go` (empty)
- **Config/lockfile-only**: `mise run check` + `cargo check -p <crate>` (+ `cargo deny check` if
    deps changed — see `gate-reviews.md` Audit)
- **Bumps** (generated artifact, MSRV `cargo +1.85.0 check -p <c> --locked`, lockfile, wrapper,
    framework major) → `dep-refresh-reviews.md`
- **Binding-harness / data-edge / .NET wrapper / C FFI docs / generated C table recipes (215-221)**,
    **new data-edge SECTION = 12-surface script + `uvx cmake` + 4-surface mutation (226, ~12 min)**
    → `binding-reviews.md` "Moved from the index (223)"
- **C++ wrapper-only step (227, ~4 min)**: ASAN build in an own `build-*/` dir + previous-header
    mutation + strict `g++ -Werror -fsyntax-only` → `binding-reviews.md` "Per-surface step commands"
- **A `[human]` issue with a `**Spec:**` field**: on closing it, the reviewer corrects the spec rows
    it names (227: `rust-core.md` return types) — stage the spec file
- **Waiting on a background run**:
    `timeout 580 bash -c 'until grep -q "^exit" <log>; do sleep 15;   done'` in the foreground works
    (bare `sleep` is blocked); never edit while prek runs
- **Core text/codec change + generated data (≈12 min)**: Rust-only + full feature matrix +
    coverage/CRAP + `bench:iai:check` + generator no-op + pre-push `ty`/ruff/pytest on any new `.py`
    test (203, ~20 min) → `gate-reviews.md`
- **Version sync**: + `version_sync.py --check`. **Shell script**: + `bash -n <script>`
- **Action pin / release.yml / any GHA bump**: static-verify only via the committed gates
    (`check_release_workflow.py` + `--check-action-inputs`); `always()` = NEEDS_WORK →
    `gha-workflow-reviews.md`
- **Pre-flight `uv run prek run --hook-stage pre-push --all-files` before pushing never-CI'd code**
    (exit 0 ⇒ `git push` will not be rejected; needs `iscc_lib` built). Run it last and
    sequentially, with `run_in_background` — `--all-files` also runs `standardrb`, ~15 min total on
    the bind mount (200)
- **CI/Audit/Perf/Semver/CRAP gates** → `gate-reviews.md`
- **Per-binding commands + standing traps** (seq 202 … UniFFI 210, npm 214, boundary slices 150–161,
    "not locally verifiable" claims — all FALSE so far) → `binding-reviews.md` "Per-surface step
    commands"
- **decisions.md over budget → rotate with `cid.archive_overflow`**, never hand-compress (command in
    `gate-reviews.md`); stage `decisions-archive.md` too (195)

## Codex Review Integration

- Advisory only, never sets the verdict; `--commit HEAD`; empty/unavailable ≠ clean. Strengths,
    blind spots, dismiss-list, the 3-step **how to weigh a finding** → **`codex-integration.md`**.
    Expect a real finding wherever a green run proves less than it appears to: matching/parsing
    rules, exception contracts, user-facing factual claims, build-config lines (154), and any
    declared-but-ungated fact such as MSRV (172)
- **`--commit HEAD` reviews ONLY the last commit** — advance often ends with a handoff-only commit
    (195, 217). `git log @{upstream}..HEAD` first; for a multi-commit range use ONE run with
    `codex exec review --base <sha-before-the-range>` (217: worked with a bare sha, ~5 min, covered
    six per-surface commits) instead of per-commit runs
- **The background Codex run executes real build/test commands in the SAME tree** (164 dotnet, 165 +
    172 Gradle) — a build tool failing once while Codex runs may be losing a race on
    `bin/`/`obj/`/`build/`. Re-run sequentially; hold any Gradle `clean` until it exits
