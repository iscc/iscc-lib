# Handoff

## 2026-07-26 — Review of: Gate `release.yml` action-input compatibility in CI (`--check-action-inputs`)

**Verdict:** PASS_WITH_NOTES

**Summary:** The last unautomated `release.yml` invariant is now an executable gate.
`scripts/check_release_workflow.py --check-action-inputs` fetches each of the 18 distinct action
refs' published `action.yml` and validates every `with:` key against declared `inputs` and every
`steps.<id>.outputs.<x>` read against declared `outputs`, wired into a dedicated `release-workflow`
CI job with 8 network-free tests behind an injected fetcher. The design is clean (small pure
functions, fetcher injection, per-ref caching, one justified `# noqa: S310`), and I independently
proved it catches the failure class it exists for. Notes below cover four probed blind spots, all
filed as an issue; none bite at HEAD.

**Verification:**

- [x] `uv run scripts/check_release_workflow.py` exits 0 — default mode unchanged, no network use
- [x] `--check-action-inputs` exits 0 with live network and **zero** `warning: skipped` lines — all
    18 refs resolved and compatible
- [x] Offline (`https_proxy=http_proxy=http://127.0.0.1:9`) exits **0** with **18**
    `warning: skipped … Connection refused` lines — transport failure degrades, never reds
- [x] `uv run pytest tests/test_check_release_workflow.py` → 19 passed (11 existing + 8 new)
- [x] Suite is network-free — identical 19 passed under the dead-proxy env
- [x] `grep -q -- "--check-action-inputs"` matches in both `.github/workflows/ci.yml` and
    `docs/development.md`
- [x] `ci.yml` parses via `yaml.safe_load`; yamlfix folded the `run:` value onto a continuation
    line, but it is still one plain scalar and resolves to the exact single command (checked by
    printing `repr(job["steps"][-1]["run"])`)
- [x] `uv run ruff check` → "All checks passed!"; `uv run ruff format --check` exit 0
- [x] `mise run check` → all 16 hooks Passed; full `uv run pytest` → 333 passed
- [x] `git status --porcelain .github/workflows/release.yml` empty — the workflow under test is
    untouched
- [x] **Independent negative probes (mine, beyond advance's):** reverting
    `actions/download-artifact@v8` → `@v3` in a temp copy fires **14** real errors (`pattern` and
    `merge-multiple` were genuinely dropped across those majors — exactly the release-day failure
    this gate exists to catch); `actions/checkout@v999` fires
    `action: no action.yml or action.yaml found for 'actions/checkout@v999'`, exit 1
- [x] Scope discipline: 2 non-test/non-doc files (`scripts/check_release_workflow.py`, `ci.yml`) —
    within the default budget of 3. Nothing from `## Not In Scope` was touched
- [x] Quality-gate integrity: `git diff @{upstream}..HEAD` adds exactly one suppression, the
    `# noqa: S310` on `urlopen`, with an inline justification (URL built from the `RAW_HOST` https
    literal). No skips, no threshold changes, no hook weakening. `RUF100` keeps it honest

**Issues found:**

- **Fails-open contract has a hole (Codex, confirmed):** `fetch_action` catches `OSError`, but
    `http.client.IncompleteRead` is an `HTTPException`/`ValueError` and a proxy/captive-portal HTML
    body raises `yaml.YAMLError` — both escape and red the CI job with a traceback, contradicting
    the documented "any transport failure degrades to a warning". Real but rare; filed.
- **The check is one-directional:** a *required* input the workflow omits is not caught, so an
    action major that adds a required input stays green and breaks on release day. Filed.
- **Two latent false positives:** a Docker-type action's GitHub-native `args` / `entrypoint`
    overrides (accepted by GitHub, not declared `inputs`) would error, and PyYAML's YAML 1.1
    booleans mangle an input literally named `on`/`off`/`yes`/`no`. Neither exists at HEAD. Filed.
- **Job-level `uses:` is not scanned** — only `job.steps[*].uses`. A future reusable-workflow call's
    `with:` block would be silently unchecked. Filed.
- **Not filed / not fixed by me:** the accepted "a fully rate-limited run is green" trade-off is now
    recorded in `decisions.md` (2026-07-26) rather than left in handoff prose — the CI job's *log*
    is the signal, not its status.
- I did **not** apply the `except` widening myself: it changes runtime behaviour, which step 9 of
    the review protocol excludes.

**Codex review:** three findings, all genuine edge cases, none affecting HEAD; all folded into the
new hardening issue. (1) P2 `IncompleteRead` escaping the `OSError` handler — confirmed correct and
the most valuable of the three, since it defeats the gate's own advertised degradation contract. (2)
P2 YAML 1.1 boolean-like action input identifiers (`on`/`off`/`yes`/`no`) being reported undeclared
— real, extremely rare, and note both sides parse with the same loader so it only bites when one
side quotes. (3) P3 percent-encoding of `#`/`%` in refs and sub-paths — accepted as theoretical; git
ref names allowing `#` is legal but no such ref exists in this ecosystem. Codex again earned its
keep on a parsing/matching diff, consistent with the pattern that it finds real defects whenever a
step adds a new matching rule.

**Next:** Two tracked, fully-unblocked `normal` issues are now on the board, both filed this review:
**(a) Gate parity of the three hand-wired docs page lists** — this one comes with measured drift, so
it is data-fix + gate in one step: `docs/llms.txt` lists only 17 of the 24 tracked `docs/**/*.md`
pages and is missing `howto/c-cpp.md`, `howto/dotnet.md`, `howto/kotlin.md`, `howto/ruby.md`,
`howto/swift.md` and `ruby-api.md`, i.e. five of eleven supported languages are invisible to
`llms.txt` consumers today. **(b) Harden the `--check-action-inputs` gate** — the four blind spots
above, all small. Prefer **(a)**: it fixes a live user-facing gap rather than hardening tooling
against hypotheticals, and it breaks the tooling-heavy run (140 tooling / 141 tests / 142 tooling /
143 docs / 144 tooling) that state.md already flags as over-weighted. Everything Unicode-propagation
and the `rubygems/configure-rubygems-credentials` pin remain human-gated.

**Notes:**

- **CI job count is 19 → 20** (`release-workflow`). Expected, not drift — update-state should record
    it as such.
- **`specs/ci-cd.md`'s CI job table was NOT updated**, contrary to next.md's instruction to review.
    The review protocol forbids the review agent from editing `target.md` or any sub-spec except
    when resolving a `[human]`-sourced issue carrying a `**Spec:**` field; this issue is
    `[review]`-sourced. Note the table is *already* descriptive rather than exhaustive — it has 14
    rows against 20 jobs (no `dotnet`, `cpp`, `swift`, `kotlin`, `coverage`, `release-workflow`
    rows), so this is pre-existing drift for Titusz to reconcile, not a regression from this step.
    define-next should stop assigning spec edits to review.
- **The resolved issue is deleted from `issues.md`** ("Gate `release.yml` action-input compatibility
    in CI") — history is in git; `learnings.md` now records the gate's existence, its two remaining
    holes and the "prove a gate with a REAL regression, not a synthetic typo" rule.
- **`learnings.md` was over budget (201 lines)** — pruned to 199 by compressing seven entries and
    archiving the Swift release-job tag dependency plus the settled `gen_meta_code_v0` / `data.json`
    vector facts to `learnings-archive.md`.
- `.claude/context/iterations.jsonl` is modified in the working tree; left unstaged (runner-owned).
