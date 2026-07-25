# Handoff

## 2026-07-25 — Review of: ruff 0.16 sub-slice E — drop the `ruff<0.16` pin

**Verdict:** PASS_WITH_NOTES

**Summary:** A two-line change executed exactly as scoped: the `ruff<0.16` pin and its `# held:`
comment are gone from `pyproject.toml`, and `uv lock --upgrade-package ruff` moved only ruff
(0.15.22 → 0.16.0) in the lock — verified by diffing every non-hash line, no other package block
moved. Every criterion in next.md reproduced green in this session, including the full 15-hook
`mise run check`, `pytest` (314), `ty check`, both pre-push ruff gates and a byte-identical Unicode
freeze-table regeneration. Slice 8 of the dependency-refresh issue is closed. The note: the
documented Markdown-formatting widening leaves a *local vs CI gate asymmetry* that I verified
empirically and filed as a new issue — nothing is red today, but a future docs change can pass every
local command and red CI.

**Verification:**

- [x] `grep -c 'ruff<0.16' pyproject.toml` → 0; `grep -c 'held:' pyproject.toml` → 0;
    `grep -q '^  "ruff",$' pyproject.toml` exit 0
- [x] `grep -c 'specifier = "<0.16"' uv.lock` → 0; `grep -A1 '^name = "ruff"$' uv.lock` →
    `version = "0.16.0"`
- [x] `uv run ruff --version` → `ruff 0.16.0`
- [x] `git diff --stat uv.lock` shows only ruff hunks — stronger check run: the diff contains
    **zero** `[+-]name = ` lines and exactly 2 hunks (the requirement entry + the ruff package
    block), so no other package moved
- [x] `uv run ruff check` → `All checks passed!`, exit 0 (the exact CI command)
- [x] `uv run ruff format --check` → exit 0, `153 files already formatted` (expected widening; see
    Notes)
- [x] `uv run ruff format --check $(git ls-files '*.md')` → `129 files already formatted`
- [x] `uv run ruff check --select S --force-exclude` and `--select C901 --force-exclude` → both
    `All checks passed!` (the two pre-push gates)
- [x] `git grep -o 'noqa: S60[0-9]' -- '*.py' | wc -l` → 13 (no blanket `--fix` was run)
- [x] `uv run ty check` → `All checks passed!`
- [x] `uv run pytest -q` → 314 passed in 25.1s
- [x] `uv run --script scripts/gen_unicode16_unassigned.py` → `wrote … (731 ranges)`, exit 0;
    `git status --porcelain crates/` empty (freeze table byte-identical)
- [x] `mise run check` → all 15 hooks Passed; afterwards `git status --porcelain` shows only the
    runner-owned `.claude/context/iterations.jsonl`
- [x] `git status --porcelain -- crates/ packages/ .github/ mise.toml .pre-commit-config.yaml` →
    empty (only `pyproject.toml` + `uv.lock` touched)
- [x] Extra: `cargo clippy --workspace --all-targets -- -D warnings` clean (only the known dev-only
    `proc-macro-error2` future-incompat note); ruff 0.16.0 declares `requires-python >=3.7`, so the
    CI `python-test` 3.10 matrix leg installs it fine
- [x] Gate integrity: `git diff @{upstream}..HEAD` over all unpushed commits contains no
    suppression, skip, threshold-reduction, hook-weakening or scope-exclusion pattern. Source budget
    1 non-generated file (`pyproject.toml`); `uv.lock` is generated
- [x] Nothing from `## Not In Scope` was touched — no unqualified `uv lock --upgrade`, no blanket
    `--fix`, no Markdown exclude, no `extend-select`/isort/pre-push-hook edits, no mdformat rev bump

**Issues found:**

- **New `normal` `[review]` issue — `ruff format` covers Markdown in CI but no local hook does.**
    Since 0.16, bare `uv run ruff format --check` (CI + `mise run lint`) reaches Python code blocks
    inside `.md`, but the prek `ruff-check`/`ruff-format` hooks declare `types: [python]` and the
    pre-push stage never runs `ruff format`. Verified by probe: a repo-local `probe.md` with `x=1`
    in a `python` fence → `uv run prek run ruff-format --files probe.md` reports
    `(no files to check) Skipped` and leaves it unchanged, while
    `uv run ruff format --check   probe.md` exits 1 with `1 file would be reformatted`. So
    `mise run format`, `mise run check` and `git push` all pass and CI's `Run ruff format check` step
    goes red. Not a defect in this step — next.md analysed the widening and deliberately declined to
    add an exclude (rationale now in `decisions.md`) — but the gate parity needs closing. Two
    candidate fixes are in the issue.
- **Minor fix applied by review:** `pyproject.toml` line 27's comment still said the `I`/`RUF022`/
    `RUF100` rules are enforced "under the pinned ruff", which the pin removal made stale. Reworded
    to state the evergreen reason (explicit selection so a future ruff bump cannot silently drop
    them). Comment-only, no behaviour change.

**Codex review:** Ran clean — "The Ruff constraint and lockfile are updated consistently to 0.16.0.
Lockfile validation, Ruff linting, and formatting checks all pass." No actionable findings; it did
not surface the local/CI Markdown asymmetry.

**Next:** Two candidates, roughly equal size, both fully locally verifiable:

1. **Close the ruff/Markdown gate-parity gap** (the new `[review]` issue above) — one
    `.pre-commit-config.yaml` edit plus a probe that the two hooks and the CI command now agree.
    Prefer this: it is the direct follow-on from this step and it protects every future docs
    iteration from a surprise CI red. Verify by re-running the `probe.md` experiment and asserting
    the hook now *fixes* the fence.
2. **Bundle the `release.yml` GHA refs with the `if:`-guard fix** — the two remaining release.yml
    issues touch the same file and neither is exercised by a CID push, so static verification (YAML
    parse + matrix/artifact-name consistency + guard presence on every `test-*`/`publish-*` job) is
    the whole gate. Doing them together avoids re-reading 97 `uses:` refs twice.

Everything else left in the dependency-refresh issue is human/major-gated (jni 0.22, magnus 0.8,
xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x, Gradle wrapper, JUnit 6.x) and each needs its own step
with a source migration, not a pin bump. The parked Unicode boundary-vector work still waits on the
open `[review]` freeze-rule-ordering issue (HUMAN REVIEW REQUESTED) — do not wire boundary vectors
into 11 bindings before that ruling.

**Notes:**

- **The Markdown widening is real and now documented in three places** so it cannot be mistaken for
    accidental scope: the slice-8 ledger in `issues.md`, a `decisions.md` entry (why no exclude was
    added and what the consequence is), and the ruff bullet in `learnings.md`. Until the parity gap
    closes, anyone editing Markdown that contains Python fences should run
    `uv run ruff format --check` by hand.
- Advance correctly refused to write the slice-8 ledger paragraph into `issues.md` (its protocol
    forbids it) and put the text in its handoff Notes instead — I appended it, marked slice 8
    CLOSED, and rewrote the trailing "remaining" paragraph that still described ruff adoption as in
    progress.
- `learnings.md` was at its 200-line budget; the ruff-adoption *history* (sub-slices A–E) moved to
    `learnings-archive.md` and only the live rules stayed. It is back at exactly 200 lines.
- No Rust, Go, JVM, .NET, Ruby or WASM surface was touched, so the CRAP baseline, iai-callgrind perf
    gate, `cargo-deny` and `cargo-semver-checks` are all correctly untouched — no baseline refresh
    is owed.
- Watch the CI run for this push: it is the first time `uv run ruff format --check` runs against the
    153-file surface in CI rather than locally. The local run is authoritative (same command, same
    lockfile), so a red here would mean a file that is gitignored locally but present in CI — not
    expected.
