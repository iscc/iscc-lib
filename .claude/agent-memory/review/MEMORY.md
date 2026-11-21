# Review Agent Memory

Concise index — **one line per entry, detail belongs in a topic file.** `review-patterns.md` (docs,
verification, claim-probing, gate scripts, parity gates), `gate-reviews.md` (CI +
Audit/Perf/Semver/CRAP), `binding-reviews.md` (per-binding cmds + `-Xcheck:jni`),
`dep-refresh-reviews.md` (bumps, lockfiles, framework majors), `gha-workflow-reviews.md`
(release.yml, action bumps), `codex-integration.md`; Stale → `MEMORY-archive.md`. The long form of
every entry below: `review-patterns.md` "Moved from the index (251)".

## Quality Gate Details

- `mise run check` = the pre-commit hooks only — never assert the hook count. Pre-push hooks
    (clippy, cargo test, pytest, `ty`, ruff `S`/`C901`) are NOT in it: run
    `cargo clippy --workspace --all-targets -- -D warnings`. Added files must be ≤256kb
- **`mise run bench:iai:check` is NOT in `mise run check`** — run it on ANY benchmarked hot-path
    change (191's +36% reached red CI) → `gate-reviews.md`
- **Pre-push needs `iscc_lib` built**: `uv run maturin develop --release` in `crates/iscc-py` BEFORE
    `git push` after a core or binding change → `binding-reviews.md`
- Proper scoping, NOT circumvention: `[tool.ty.src] exclude` for non-venv imports (`conanfile.py`),
    `per-file-ignores` waiving `S101`/`S603`/`S607` in tests, a commented `# noqa: S603` in scripts
- **After ANY step-9 edit to a `.py` file, re-run the PRE-PUSH gates** (`uv run ty check`, ruff
    `S`/`C901`); use `monkeypatch.setattr` on importlib-loaded modules (157)

## Common Issues

- Verification greps false-positive (`grep -qv`, substring `grep -c`, exact counts, 139): test the
    criterion against `HEAD~1` before believing advance broke it
- **next.md's Implementation Notes are a HYPOTHESIS** (142, 143, 149, 239): re-derive every number,
    command and expected value from source → `review-patterns.md` "Claim-Probing"
- `iscc_decompose` returns units WITHOUT "ISCC:" prefix. Docs site: `https://lib.iscc.codes/`
- **A docs-only sweep can FABRICATE a return type** (190); a `#[deprecated]` claim is a hypothesis
    (168) → `review-patterns.md` "Moved from the index"
- **Codec changes**: probe empty/garbage inputs (191), `MDFZAAAAAAAAAAAAAA` (174), case-fold aliases
    \+ every reference entry point (201) → `review-patterns.md`
- **next.md under-scoping a spec's "Verified when"**, `.crap-baseline.json` scope cap →
    `review-patterns.md` "Moved … (204)"
- **6-file cap counts files whose CODE changes** (225, decisions.md): comment-only edits and
    regenerated outputs don't count IF next.md lists them as such — confirm from the diff
- **Core behaviour change with flipped binding tests (225)**: all suites from ONE logged script +
    grep docs for touched signatures → `review-patterns.md` "Moved from the index (239)"
- **Run the full CI feature matrix on any core change** (174) → `review-patterns.md`
- Go `// indirect`, CI `find` cross-arch, prek stash conflict, mode-only commits →
    `review-patterns.md` "Gotchas". **Search with `git grep`, never `grep -r`** (230)
- **Backslash-u escapes decode to raw UTF-8 through Edit/Write**, and an Edit containing them fails
    to match (251): write such text with a Python script that builds the backslash as `chr(92)`. Who
    may edit `issues.md` / a spec is protocol-fixed → "Artifact-editing traps"
- **`mise run check` mdformat may reformat context files**: check `git status --porcelain` after and
    STAGE it; never stage `iterations.jsonl`; format with `uv run prek run mdformat --files <f>`
- **No-op / human-handoff iteration** (111/173/248): PASS; `low`-only + human work or an
    upstream-gated `normal` → HUMAN REVIEW REQUESTED, never `**IDLE**`; all-`low` `Step: NONE` →
    PASS + `**IDLE**` → `review-patterns.md`
- **Push message "GitHub found N vulnerabilities" is about `main`** (234), not `develop`
- **Unicode freeze rule REMOVED 2026-07-30** — reintroducing any freeze mechanism = NEEDS_WORK
- **`git push` failing "index.lock: File exists" is a WSL2 race, NOT a hook rejection** (207):
    confirm no 2nd loop in `ps aux`, then retry once
- **"files were modified by this hook" often means SOMETHING ELSE WAS WRITING** (97/164): a second
    CID loop, or my own Edits during a backgrounded prek run. Never edit while prek runs
- **Spec paths are gated (224)**; a "mechanism does not exist" realignment is a hypothesis. **Mutate
    each helper the step adds** (228) → `review-patterns.md` "Moved from the index (239)"
- **Spec-hygiene (223), docs fan-out (211), uncommitted human edits (212)** → `review-patterns.md`
    "Moved from the index (223)"
- **Budgets**: learnings.md sits AT 200 lines (259) — every new bullet needs an equal cut in the
    same commit; issues.md budget is 400; check `wc -l` after mdformat. This index: under 140 lines
    (a hook enforces it) — move the long form to a topic file

## Review Shortcuts (detail in the topic files named above)

- **Rust-only**: `cargo test -p iscc-lib` + clippy workspace + `mise run check`. Version sync adds
    `version_sync.py --check`, a shell script `bash -n <script>`
- **Core + Go behaviour / integer-width fix (250/263)**: boundary differential, bare `rustc` =
    edition 2015, host-type check for a follow-up vector → `review-patterns.md` "Core + Go … (250)"
- **Go-only canonicalizer / serializer fix (251/266, ~8–15 min)**: Go criteria + ONE differential
    against `iscc_lib` from a `git archive` copy (`go test -v`, or the output is hidden) →
    `review-patterns.md` "Go-only serializer fix (251)"
- **Go-only**: `mise run check`, `CGO_ENABLED=0 mise exec -- go test -C packages/go -count=1 ./...`,
    `go vet -C packages/go ./...`, `mise exec -- gofmt -l packages/go`; API-shape (261) → topic
- **Test-fixture / vector-file only (141/149, ~5 min)**: Rust-only + feature matrix + the four
    fixture mutations → `review-patterns.md`
- **Docs-only**: `mise run check` + clippy + `mise run version:check` + `uv run zensical build`;
    green gates say NOTHING about truth (143) → `review-patterns.md` "Moved from the index (239)"
- **Python-only**: `mise run check` + `pytest`; a new `.py` adds pre-push `ty`/ruff; rebuild with
    `maturin develop` before any test; PyO3 marshalling step (265, ~8 min) → `binding-reviews.md`
- **New gate script / gate hardening / parity gate (142–146/163, ~10 min)**: write your OWN
    mutations, probe equal-empty and duplicated-row blind spots → `review-patterns.md`
- **Config/lockfile-only**: `mise run check` + `cargo check -p <crate>` (+ `cargo deny check` if
    deps changed → `gate-reviews.md` Audit)
- **Pytest config / test-fixture speed (254, ~6 min)**: an ini-wide `addopts` reaches EVERY pytest
    invocation → `review-patterns.md` "Pytest config / test-fixture speed step (254)"
- **Bumps** (generated artifact, MSRV, lockfile, wrapper, framework major) and **security-pass /
    advisory-bump steps (235)** → `dep-refresh-reviews.md`
- **Binding-harness / data-edge / .NET wrapper / C FFI docs / generated C table (215-221)**, new
    data-edge SECTION = 12-surface script (226, 264: both iscc-core builds) + **diff each new
    `UNCOMPARED` row against the function's `data.json` keys (252: Codex caught a missed key)**,
    **validator / C-generator (236)**, **harness-compare fan-out (237)**, **throwaway-vector proof
    (253)** → `binding-reviews.md`
- **C FFI marshalling-helper step (255, ~35 min)**: read per-row closures for allocations (no perf
    gate covers `iscc-ffi`), ASAN-probe new C macros with a forced mismatch → `binding-reviews.md`
- **JS integer-range / marshalling step (256, ~15 min)**: Node probes on the built WASM package,
    surefire XML for JNI counts, message-asserting negatives → `binding-reviews.md`
- **C++ wrapper-only (227)**, **JNI error-path (230)**, **verbatim code-move (231)**,
    **test-harness-only (232)**, **WASM return-shape (234)**, **Ruby gem packaging (239–241)** →
    `binding-reviews.md`
- **A `[human]` issue with a `**Spec:**` field**: on closing it, the reviewer corrects the spec rows
    it names (227) — stage the spec file
- **Waiting on a background run** (bare `sleep` is blocked):
    `timeout 580 bash -c 'until grep -q "^exit" <log>; do sleep 15; done'`
- **Core text/codec change + generated data (≈12–20 min)**: Rust-only + feature matrix + CRAP +
    `bench:iai:check` + generator no-op + pre-push gates on new `.py` (203) → `gate-reviews.md`
- **Doc-parity gate / doc-listing (233)**, **parser-only (242)**, **Rust docs completion (243)**,
    **C++ how-to / doc-example (244)** → `review-patterns.md` "Doc-parity gate (233)"
- **Derived-fact removal (245)**, **file-list removal (246)**, **spec-list gate (247)**, **spec
    drift realignment (259: every "so that" clause is its own claim; sweep the claim repo-wide)** →
    `review-patterns.md`, sections of the same names
- **Docs install/packaging-claim step (238, 257)**: install the PUBLISHED artifact in a clean
    prefix; simulate a foreign platform for any fallback sentence (`java -Dos.arch=…`) →
    `review-patterns.md` "Install-claim probe (238)"
- **A test that shells out adds a tool to the `python-test` CI job** (240: `ruby`): check the runner
    image readme
- **Action pin / release.yml / any GHA bump**: static-verify only via the committed gates
    (`check_release_workflow.py` + `--check-action-inputs`); `always()` = NEEDS_WORK →
    `gha-workflow-reviews.md`
- **ci.yml fix only CI can prove (260/262, ~15 min)**: read the action's source at the tag, push the
    advance commits FIRST, read the job LOG, then write the handoff → `gha-workflow-reviews.md`
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
    failing once may be losing a race. Re-run sequentially
