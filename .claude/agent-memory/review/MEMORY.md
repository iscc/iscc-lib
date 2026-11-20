# Review Agent Memory

Concise index — **one line per entry, detail belongs in a topic file.** `review-patterns.md` (docs,
verification, claim-probing, gate scripts, parity gates), `gate-reviews.md` (CI +
Audit/Perf/Semver/CRAP), `binding-reviews.md` (per-binding cmds + `-Xcheck:jni`),
`dep-refresh-reviews.md` (bumps, lockfiles, framework majors), `gha-workflow-reviews.md`
(release.yml, action bumps), `codex-integration.md`; Stale → `MEMORY-archive.md`.

## Quality Gate Details

- `mise run check` = the pre-commit hooks only — never assert the hook count. Pre-push hooks
    (clippy, cargo test, pytest, `ty`, ruff `S`/`C901`) are NOT in it: run
    `cargo clippy --workspace --all-targets -- -D warnings`. Added files must be ≤256kb
- **`mise run bench:iai:check` is NOT in `mise run check`** — run it on ANY benchmarked hot-path
    change (191's +36% reached red CI) → `gate-reviews.md`
- **Pre-push needs `iscc_lib` built**: `uv run maturin develop --release` in `crates/iscc-py` BEFORE
    `git push` → `binding-reviews.md`
- Proper scoping, NOT circumvention: `[tool.ty.src] exclude` for non-venv imports (`conanfile.py`),
    `per-file-ignores` waiving `S101`/`S603`/`S607` in tests, a commented `# noqa: S603` in scripts
- **After ANY step-9 edit to a `.py` file, re-run the PRE-PUSH gates** (`uv run ty check`, ruff
    `S`/`C901`); use `monkeypatch.setattr` on importlib-loaded modules (157)

## Common Issues

- Verification greps false-positive: `grep -qv 'pat'` is always true on multi-line files (use
    `! grep -q`). Substring `grep -c` and exact counts in next.md are a recurring mis-spec class
    (139) — test the criterion against `HEAD~1` before believing advance broke it
- **next.md's Implementation Notes are a HYPOTHESIS — algorithms, prose and quoted commands alike**
    (142, 143, 149; 239 a misquoted third-party grep). Re-derive every number, command and expected
    value from source → `review-patterns.md` "Claim-Probing"
- `iscc_decompose` returns units WITHOUT "ISCC:" prefix. Docs site: `https://lib.iscc.codes/`
- **A docs-only sweep can FABRICATE a per-surface return type** (190); a `#[deprecated]` claim is a
    hypothesis (168) → `review-patterns.md` "Moved from the index"
- **Codec changes**: probe empty/garbage inputs (191), `MDFZAAAAAAAAAAAAAA` on a header-gate change
    (174), case-fold aliases + every reference entry point (201) → `review-patterns.md`
- **next.md under-scoping a spec's "Verified when"**, `.crap-baseline.json` scope cap, differential
    / repo-state / lint-config / prek-scope reviews → `review-patterns.md` "Moved … (204)"
- **6-file cap counts files whose CODE changes** (225, decisions.md): comment-only binding edits +
    regenerated outputs don't count, IF next.md lists them as such — confirm from the diff
- **Core behaviour change with flipped binding tests (225, ~25 min)**: all suites from ONE logged
    script + grep docs for touched signatures → `review-patterns.md` "Moved from the index (239)"
- **Run the full CI feature matrix on any core change** — only `--no-default-features` catches a
    feature-gated `#[test]` compile break (174) → `review-patterns.md`
- Go `// indirect`, CI `find` cross-arch, prek stash conflict, mode-only commits under
    `core.fileMode=false` → `review-patterns.md` "Gotchas"
- **`\uXXXX` decodes to literal UTF-8 through Edit/Write and bash**; who may edit `issues.md` / a
    spec is protocol-fixed (135/144/172) → `review-patterns.md` "Artifact-editing traps"
- **`mise run check` mdformat may reformat context files**: check `git status --porcelain` right
    after and STAGE it; never stage `iterations.jsonl`; format only with
    `uv run prek run mdformat --files <f>` → `review-patterns.md` "Moved from the index (239)"
- **No-op / human-handoff iteration** (111/173): verdict PASS; `low`-only + human spec work or an
    upstream-gated `normal` → HUMAN REVIEW REQUESTED, never `**IDLE**`. Verify an advance "only low
    issues remain" claim independently → `review-patterns.md`
- **Push message "GitHub found N vulnerabilities" is about `main`** (234), not `develop` →
    `review-patterns.md` "Moved from the index (239)"
- **Unicode freeze rule REMOVED 2026-07-30** — reintroducing any freeze mechanism = NEEDS_WORK
- **`git push` failing "index.lock: File exists" while prek stashes is a WSL2 race, NOT a hook
    rejection** (207): confirm no 2nd loop in `ps aux`, then retry once
- **"files were modified by this hook" often means SOMETHING ELSE WAS WRITING** (97/164): a second
    CID loop, or my own Edits during a backgrounded prek run. Never edit while prek runs
- **Spec paths are gated (224)**; a "mechanism does not exist" realignment is a hypothesis. **Mutate
    each helper the step adds** (228) → `review-patterns.md` "Moved from the index (239)"
- **Spec-hygiene (223), docs fan-out (211), issues.md at budget, uncommitted human edits (212)** →
    `review-patterns.md` "Moved from the index (223)"
- **learnings.md sits AT its 200-line budget**: every new bullet needs an equal cut or a move to
    `learnings-archive.md` in the same commit (239); check `wc -l` after mdformat

## Review Shortcuts (detail in the topic files named above)

- **Rust-only**: `cargo test -p iscc-lib` + clippy workspace (~2 s warm) + `mise run check`. Version
    sync adds `version_sync.py --check`, a shell script `bash -n <script>`
- **Test-fixture / vector-file only (141/149, ~5 min)**: Rust-only + full feature matrix (assert
    per-target `N passed`) + the four fixture mutations → `review-patterns.md`
- **Docs-only**: `mise run check` + clippy + `mise run version:check` + `uv run zensical build`;
    green gates say NOTHING about truth (143) → `review-patterns.md` "Moved from the index (239)"
- **Python-only**: `mise run check` + `pytest`; a new `.py` adds pre-push `ty`/ruff; rebuild with
    `maturin develop` before any `__doc__` test (222) → same section
- **New gate script / gate hardening / parity gate (142–146/163, ~10 min)**: write your OWN
    mutations, probe equal-empty and duplicated-row blind spots → `review-patterns.md`
- **Go-only**: `mise run check`, `CGO_ENABLED=0 mise exec -- go test -C packages/go -count=1 ./...`,
    `go vet -C packages/go ./...`, `mise exec -- gofmt -l packages/go` (empty)
- **Config/lockfile-only**: `mise run check` + `cargo check -p <crate>` (+ `cargo deny check` if
    deps changed → `gate-reviews.md` Audit)
- **Bumps** (generated artifact, MSRV `cargo +1.85.0 check -p <c> --locked`, lockfile, wrapper,
    framework major) → `dep-refresh-reviews.md`
- **Security-pass / advisory-bump step (235, ~15 min)**: cargo-deny + govulncheck + ONE osv-scanner
    pass; `diff -rq` the Go module cache → `dep-refresh-reviews.md`
- **Binding-harness / data-edge / .NET wrapper / C FFI docs / generated C table (215-221)**, new
    data-edge SECTION = 12-surface script (226) → `binding-reviews.md` "Moved from the index (223)"
- **C++ wrapper-only step (227, ~4 min)**: ASAN build in an own `build-*/` dir + previous-header
    mutation + strict `g++ -Werror -fsyntax-only` → `binding-reviews.md`
- **A `[human]` issue with a `**Spec:**` field**: on closing it, the reviewer corrects the spec rows
    it names (227) — stage the spec file
- **Waiting on a background run** (bare `sleep` is blocked):
    `timeout 580 bash -c 'until grep -q "^exit" <log>; do sleep 15; done'`
- **Core text/codec change + generated data (≈12–20 min)**: Rust-only + feature matrix + CRAP +
    `bench:iai:check` + generator no-op + pre-push gates on new `.py` (203) → `gate-reviews.md`
- **JNI error-path step (230, ~12 min)**: shadow-class probe of the pending-exception guard;
    `-Xcheck:jni` is silent on `ThrowNew` over a pending exception → `binding-reviews.md`
- **Verbatim code-move step (231, ~5 min)**: prefix `diff`, de-indented body `diff`, test-name set
    `diff`; cbindgen to a temp file + `cmp` → `binding-reviews.md`
- **Test-harness-only step (232, ~8 min)**: mutate from throwaway copies; macOS proof = the `swift`
    CI job after the push → `binding-reviews.md`
- **Doc-parity gate / doc-listing step (233, ~25 min)**: restore previous docs in a `git archive`
    repo, feed the parser unlisted forms (fail-open?), RUN every return type the docs add →
    `review-patterns.md` "Doc-parity gate (233)"
- **WASM return-shape step (234, ~10 min)**: `#[wasm_bindgen_test]` never crosses the JS glue —
    probe the built `--target nodejs` pkg from Node + read the `.d.ts` → `binding-reviews.md`
- **data-edge rule / validator / C-generator step (236, ~8 min)**: two pytest files, regen +
    `git status` on the header, `gen.render` on a mutated fixture copy → `binding-reviews.md`
- **Harness-compare fan-out step (237, ~6 min)**: four suites from one logged script, .NET mutant
    from a `git archive` copy → `binding-reviews.md`
- **Docs install/packaging-claim step (238, ~12 min)**: install the PUBLISHED artifact in a clean
    prefix → `review-patterns.md` "Install-claim probe (238)"
- **Ruby gem packaging step (239, NEEDS_WORK)**: a gemspec `rb_sys` dependency unpinned cross-gem
    (unanchored first-match grep; next.md misquoted it, Codex caught it). Run the action's real
    pipeline; Ruby 3.4 via mise in 24 s; work owed at the PASS review → `binding-reviews.md`
- **Search with `git grep`, never `grep -r`** over dirs with build output (230) →
    `review-patterns.md` "Gotchas"
- **Action pin / release.yml / any GHA bump**: static-verify only via the committed gates
    (`check_release_workflow.py` + `--check-action-inputs`); `always()` = NEEDS_WORK →
    `gha-workflow-reviews.md`
- **Pre-flight `uv run prek run --hook-stage pre-push --all-files` before pushing never-CI'd code**
    (~15 min, background, last) → `review-patterns.md` "Moved from the index (239)"
- **Per-binding commands + traps** ("not locally verifiable" claims: all FALSE so far) →
    `binding-reviews.md`; CI/Audit/Perf/Semver/CRAP gates → `gate-reviews.md`
- **decisions.md over budget → rotate with `cid.archive_overflow`**, never hand-compress (command in
    `gate-reviews.md`); stage `decisions-archive.md` too (195)

## Codex Review Integration

- Advisory only, never sets the verdict; empty/unavailable ≠ clean. How to weigh a finding →
    **`codex-integration.md`**. It finds what a green run does not prove (parsing rules, exception
    contracts, factual claims, build config, a third-party action's real behaviour: 239)
- **`--commit HEAD` reviews ONLY the last commit**: `git log @{upstream}..HEAD` first; for a range
    use ONE run with `codex exec review --base <sha-before-the-range>` (217)
- **The background Codex run executes real builds in the SAME tree** (164/165/172): a build tool
    failing once may be losing a race. Re-run sequentially → `review-patterns.md` "Moved from the
    index (239)"
