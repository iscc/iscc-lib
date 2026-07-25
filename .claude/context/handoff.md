# Handoff

## 2026-07-25 — Close the ruff/Markdown gate-parity gap in the prek hooks

**Done:** Widened the local `ruff-format` prek hook from `types: [python]` to
`types_or: [python, markdown]` (plus an evergreen comment explaining why), so the local formatter
surface matches CI's bare `uv run ruff format --check`. Added the one-sentence qualification to the
hook lists in `docs/development.md` and `CLAUDE.md`. Closes the `normal` `[review]` issue
"`ruff format` covers Markdown in CI but no local hook does" via fix option (a), exactly as scoped.

**Files changed:**

- `.pre-commit-config.yaml`: `ruff-format` hook now `types_or: [python, markdown]` + 2-line comment;
    `ruff-check` deliberately untouched (`types: [python]` — ruff 0.16 formats Markdown but does not
    lint it)
- `docs/development.md`: ruff bullet in the pre-commit hook list states `ruff format` also covers
    Python code blocks inside Markdown, matching the CI `ruff format --check` step (mdformat
    rewrapped the sentence at 100 cols during `mise run format`)
- `CLAUDE.md`: same qualification in the Pre-commit Hooks stage list

**Verification:** Every criterion in next.md reproduced green in this session:

- `grep -A7 'id: ruff-format' ... types_or: \[python, markdown\]` → exit 0;
    `grep -A6 'id: ruff-check' ... types: \[python\]` → exit 0
- `uv run prek run check-yaml --files .pre-commit-config.yaml` → Passed;
    `yamlfix --files .pre-commit-config.yaml` → Passed, `git diff` shows only the intended 2-line
    comment + 1-line type change (yamlfix made no edits)
- Probe: **staged** `probe_gate.md` with a `py` fence containing `x=1` →
    `uv run prek run ruff-format --files probe_gate.md` reports
    `Failed / files were modified by this hook / 1 file reformatted`, `grep 'x = 1'` → FIXED, file
    removed, tree clean afterwards
- `uv run ruff format --check` → exit 0, `153 files already formatted` (exact CI command + surface)
- `uv run ruff format --check $(git ls-files '*.md' '*.py' '*.pyi')` → exit 0,
    `154 files already formatted` (the widened hook's surface, strict superset of CI)
- `uv run ruff check` → `All checks passed!`, exit 0
- `mise run check` → all 15 pre-commit-stage hooks Passed, **twice in a row**, exit 0 both times;
    afterwards `git status --porcelain` shows only the 3 intentional edits + runner-owned
    `.claude/context/iterations.jsonl` — no mdformat/ruff ping-pong over the 129 tracked `.md` files
- `uv run zensical build` → exit 0, `No issues found`
- `grep -n 'ruff format' docs/development.md CLAUDE.md` → both files mention Markdown code blocks in
    the ruff bullet (development.md:137/140, CLAUDE.md:179/180)
- `git status --porcelain -- crates/ packages/ .github/ mise.toml pyproject.toml uv.lock` → empty
    (no source, CI or dependency file touched)

**Next:** The other candidate from the previous handoff — bundle the two remaining `release.yml`
issues (GHA `uses:` refs refresh + the `if:`-guard fix) into one step, since both touch the same
file and are verified statically (YAML parse + matrix/artifact-name consistency + guard presence).
The remaining dependency-refresh majors (jni 0.22, magnus 0.8, xunit 3.x, Gradle wrapper, JUnit 6.x)
are each a dedicated migration step; the Unicode boundary-vector work stays parked on the open HUMAN
REVIEW freeze-rule-ordering ruling.

**Notes:**

- **Probe nuance not in next.md:** prek only reports `files were modified by this hook` for
    *tracked* files. With an untracked `probe_gate.md`, the hook still *fixes* the fence but reports
    `Passed`. next.md's scoping probe used a staged file; I reproduced the exact expected
    `Failed / 1 file reformatted` output after `git add`-ing the probe, then removed it from the
    index and the tree. The gate itself is unaffected — at commit time every file prek sees is
    staged by definition.
- The `ruff-check` hook was left at `types: [python]` per the explicit Not-In-Scope entry (ruff 0.16
    does not lint Markdown; widening would only add "No Python files found" warning noise).
- No `--force-exclude` added: the hook's explicit-path surface (154 tracked files) is a deliberate
    strict superset of CI's recursive discovery (153 — one tracked-but-gitignored file under
    `.claude/plans/` is skipped by discovery). Superset is the safe direction.
- **For the review agent (issues.md ledger):** my protocol forbids me editing `issues.md`. Please
    mark the `normal` `[review]` issue "`ruff format` covers Markdown in CI but no local hook does"
    CLOSED with: *Fixed via option (a) — `.pre-commit-config.yaml` `ruff-format` hook widened to
    `types_or: [python, markdown]`; `ruff-check` left Python-only (ruff does not lint Markdown);
    verified by staged-probe (hook fixes a `py` fence) and double `mise run check` (no mdformat/ruff
    ping-pong); local surface (154 tracked md+py+pyi files) is a strict superset of CI's 153-file
    discovery.*
- No Rust, Python, Go, JVM, .NET, Ruby or WASM source touched — no CRAP baseline, iai perf,
    cargo-deny or semver refresh owed. No benchmarked hot path involved.
- Agent memory was compacted this iteration (MEMORY.md 157 → 139 lines) to absorb the new entries;
    detail moved to `deps-refresh.md` (new "ruff-format hook gate parity" section).
