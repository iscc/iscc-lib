# Handoff

> **HUMAN REVIEW REQUESTED**: with both gate blind-spot issues closed, **no unblocked `normal` work
> remains for CID**. Every open `normal` issue is human-gated or parked: the freeze-rule
> sequence-ordering wording ruling, the Go 15.0-tables call, the
> `rubygems/configure-rubygems-credentials` tag-vs-SHA choice, npm OIDC trusted publishing, and the
> human/major-gated dependency remainder. This is not `**IDLE**` (those issues are `normal`, not
> `low`), so the loop should pause rather than manufacture churn. Titusz: the two Unicode rulings
> are short, fully written up in issues.md, and together unblock Rust-core criteria 3 and 4 plus the
> parked `docs/unicode.md` sentence — the fourth consecutive iteration to say so.

## 2026-07-26 — Review of: Close the known blind spots in the two gate scripts

**Verdict:** PASS_WITH_NOTES

**Summary:** Both `[review]` gate blind-spot issues are genuinely closed, not merely green. The
docs-nav comment stripper is string-aware (the real `{ "C# / .NET" = "howto/dotnet.md" }` line
survives), and the action-input gate is now bidirectional, parse-failure-tolerant and reports a
resolved/total counter. I re-proved every new branch with my own mutations against the real
`release.yml` and a real `zensical.toml` — the checks are non-vacuous in both directions. Scope was
exactly the 3 non-test/non-doc files next.md budgeted; the two deliberately-deferred sub-items are
re-filed as one `low` issue rather than silently dropped.

**Verification:**

- [x] `pytest tests/test_check_docs_nav.py tests/test_check_release_workflow.py` — **39 passed** (≥
    33 required), all named cases present
- [x] `uv run pytest` — **353 passed** (≥ 347 required), no failures
- [x] `uv run scripts/check_docs_nav.py` — exit 0,
    `OK: 23 documentation pages consistent across nav, ORDERED_PAGES and llms.txt.`
- [x] `--check-action-inputs` online — exit 0, **empty stderr**, zero `action:` lines, and the
    summary `action-inputs: resolved 18 of 18 action refs (0 skipped)`
- [x] Fails open offline (dead proxy) — exit **0**, `resolved 0 of 18 action refs (18 skipped)`, 18
    `warning: skipped` lines, no traceback
- [x] `uv run scripts/check_release_workflow.py` (no flag, network-free) — exit 0
- [x] `prek run check-docs-nav --all-files` and `check-release-workflow --all-files` — both Passed
- [x] `grep -c 'git rm' .pre-commit-config.yaml` — 1
- [x] `mise run check` — all 17 hooks Passed; `ruff check`, `ruff format --check` (exit 0),
    `ty check` all clean
- [x] `git status --porcelain .github/workflows/ .claude/context/specs/ docs/unicode.md crates/ packages/`
    — empty
- [x] Scope discipline — 3 non-test/non-doc files (`scripts/check_docs_nav.py`,
    `scripts/check_release_workflow.py`, `.pre-commit-config.yaml`), within the standard budget; no
    gate weakened anywhere in `@{upstream}..HEAD`

**Independent mutation probes (review, not next.md):**

- Docs nav, against the **real** `zensical.toml` in a `git archive HEAD` copy: commenting out
    `{ "Kotlin" = "howto/kotlin.md" },` → fires `missing 1 page(s)`; commenting out the
    `{ "C# / .NET" = … }` line → fires (the `#` in the *title* does not shield it); a trailing
    `# was "ghost.md"` comment → clean; a comment line with an unbalanced quote → clean. Deleting
    `docs/howto/kotlin.md` from disk → 3 `unexpected 1 page(s)` errors, confirming the anchor test
    really covers the `git rm` case the prek hook cannot see.
- Action inputs, against the **real** `release.yml`: the reverse check is non-vacuous — 4 of the 18
    refs declare `required`-without-default inputs (`cache@v6` path/key, `upload-artifact@v7` path,
    `cross-gem@v1` platform, `setup-ndk@v1` ndk-version) and all are provided at HEAD. Dropping each
    fires exactly the right job; removing a `with:` block entirely fires both keys; a realistic
    `path:` → `paths:` typo fires **both** directions and exits 1 with the summary line still
    printed. `args` on a non-docker action still fires (the docker allowance is correctly
    conditional).

**Issues found:**

- (minor, fixed in this commit) The module docstring and `docs/development.md` both claimed "the run
    ends with an `action-inputs: …` summary line". It does not — the summary is followed by the
    error lines and/or the final `OK: …` line. Reworded to "the check prints an …".
- (deferred by design, re-filed as one `low` `[review]` issue) A zero-resolved run still exits 0
    (policy flip needs Titusz per `decisions.md` 2026-07-26); job-level `uses:` is still unscanned;
    and the YAML 1.1 boolean fix is one-sided — non-string `with:` keys are skipped, but an action
    whose *metadata* declares an input literally named `on`/`off`/`yes`/`no` would still false-
    positive against a quoted workflow key. None of the three bites at HEAD, each has an explicit
    trigger condition recorded.

**Codex review:** Clean — "The new gate behavior matches the documented scope, and the targeted test
suite, linting, and type checks pass. No qualifying correctness regressions were found." No
actionable findings.

**Next:** There is no unblocked `normal` work left (see the escalation above). If define-next agrees
after its own sweep, write `## Step: NONE` naming the blockers; do **not** downgrade to `**IDLE**` —
the blockers are `normal`, and the correct signal is the human pause already flagged here. If Titusz
lands either Unicode ruling, the immediate next step is the one it unblocks: (a) the ordering ruling
→ reword `specs/rust-core.md` criterion 4 + unfreeze the scoped CPython-3.14 sentence in
`docs/unicode.md`; (b) the Go 15.0-tables call → propagate `unicode_boundary.json` into the binding
conformance suites.

**Notes:**

- Do **not** re-task the review agent with editing `.claude/context/specs/ci-cd.md` (the 14-row job
    table vs 21 jobs drift noted in state.md). The review protocol forbids spec edits without a
    `[human]`-sourced issue carrying a `**Spec:**` field; this is the third iteration the drift has
    been carried as a note. It reads as descriptive rather than exhaustive, so it is not filed as an
    issue — if Titusz wants it exhaustive, that is a `[human]` issue.
- Cadence: 142 tooling / 143 docs / 144 tooling / 145 docs+gate / 146 gate. Five tooling-ish steps
    in a row is now the *consequence* of the blockade, not a choice — another reason the pause is
    right.
- The `--check-action-inputs` job costs 18 unauthenticated `raw.githubusercontent.com` requests per
    run. It resolved 18/18 from the devcontainer today; a rate-limited CI run would print
    `resolved 0 of 18` and still pass. That summary line is now the thing to read in the job log.
- `.claude/context/learnings.md` is at the 200-line budget; two entries were rewritten in place
    (docs-page gate, release.yml gate) rather than appended.
