# Handoff

## 2026-07-26 — Review of: Gate byte-identity of the vendored test-vector copies (pytest anchor)

**Verdict:** PASS

**Summary:** A 99-line pytest anchor (`tests/test_vendored_fixtures.py`) now gates all five vendored
fixture copies against their two canonical sources, plus a `git ls-files` set-equality check that
makes the table self-extending and an ASCII guard on the boundary fixture. Scope is exactly one test
file + one doc line — zero source, fixture or baseline changes. I re-proved the gate against six
independent mutations in a throwaway repo (never the work tree): it reds on drift, on an
unregistered copy, on a **deletion**, on decoded `\uXXXX`, on an emptied table, and it fails
*closed* when `git` is unavailable.

**Verification:**

- [x] `uv run pytest tests/test_vendored_fixtures.py -q` — **8 passed, 0 skips**
- [x] `--collect-only -q` — 8 tests, **5** byte-identity ids (one per copy path), ≥ 5 as required
- [x] `uv run pytest` (full suite) — **379 passed** in 27.0s
- [x] `git ls-files -z -- '*data.json' '*unicode_boundary.json' | …| grep -c .` = **7**; all 7 paths
    appear literally in the test file (`grep -F -c` ≥ 1 each; the boundary canonical appears twice —
    table + ASCII guard)
- [x] **Drift probe** — `printf ' ' >> packages/kotlin/…/data.json` reds exactly
    `test_vendored_copy_is_byte_identical[packages/kotlin/src/test/resources/data.json]` with
    `fix with: cp crates/iscc-lib/tests/data.json …`. Run in a throwaway repo, so the work tree was
    never dirtied and no git index operation was needed.
- [x] `git status --porcelain` over `packages/`, `crates/iscc-lib/tests/`, `.crap-baseline.json` and
    `.iai-baseline.json` — empty; over `'crates/*/src'` — empty
- [x] `mise run check` — exit 0, all 17 hooks Passed, no reformats (`git status` clean apart from
    the runner's `iterations.jsonl`)
- [x] `uv run ruff check .` — "All checks passed"; `uv run ruff format --check .` — exit 0
- [x] `grep -F -c 'test_vendored_fixtures.py' crates/iscc-lib/CLAUDE.md` = 1
- [x] **Extra — pre-push gates** (not in `mise run check`): `uv run ty check`,
    `ruff check --select S .`, `ruff check --select C901 .` all exit 0. The `subprocess.run` needs
    no `# noqa`: `[tool.ruff.lint.per-file-ignores]` already waives `S603`/`S607` for `tests/**`
    (pre-existing and commented — no suppression was added by this diff).
- [x] **Extra — five review mutations beyond the drift probe** (throwaway repo built with
    `git archive HEAD | tar -x -C /tmp/x && git init`): (1) a *staged* unregistered copy
    `packages/cpp/testdata/data.json` reds `test_no_unregistered_tracked_copy` with
    `unregistered=[…]`; (2) `git rm packages/go/testdata/unicode_boundary.json` reds **both** the
    identity case and the set check with `gone=[…]` — the delete case a `files:`-scoped prek hook
    cannot see, so the pytest-anchor choice is empirically the right one; (3) re-encoding the
    canonical boundary fixture with `ensure_ascii=False` reds `test_boundary_fixture_is_pure_ascii`
    *and* the Go copy's identity case; (4) emptying `VENDORED_COPIES` reds the non-vacuity floor and
    the set check (the identity test degrades to pytest's "got empty parameter set" **skip** — the
    exact vacuous-pass hazard the floor exists to catch); (5) `mv .git .git-off` raises
    `CalledProcessError` — the gate fails closed, never skips.
- [x] **Extra — gate reality.** The pytest prek hook is `always_run: true`, `pass_filenames: false`,
    `stages: [pre-push]`, so the whole suite fires on every push regardless of what changed — a
    deletion-only push is covered. CI `python-test` (ubuntu, 3.10 + 3.14) runs it too, in an
    `actions/checkout` git tree. `.gitattributes` pins `*.json` to `eol=lf`, so byte-identity cannot
    be broken by checkout normalization.
- [x] **Extra — gate integrity.** Full unpushed range (`@{upstream}..HEAD`, 4 commits) adds no lint
    suppression, no skip/ignore, no threshold change; no file matching `pre-commit-config`,
    `.github/`, `pyproject.toml`, `deny.toml`, `Cargo.toml`, `mise.toml` or `*baseline*` is touched.

**Issues found:**

- (none blocking) **Known blind spot, recorded not fixed:** discovery keys on the two basenames
    `data.json` / `unicode_boundary.json`, so a future slice that vendors a copy under a *different*
    filename escapes `test_no_unregistered_tracked_copy` entirely. Generalizing to content-based
    duplicate detection was explicitly out of scope, and the mitigation is cheap — keep the
    canonical basenames when propagating. Noted in learnings.md and appended to the Unicode issue in
    issues.md so the next propagation slice sees it.
- The `len(PAIRS) >= 5` floor stays at 5 as copies grow, so it does not track reality — but it
    cannot be gamed: dropping a live entry from the table reds the set-equality check instead. Belt
    and braces, no action needed.
- Resolved and deleted from issues.md: "Gate byte-identity of the vendored test-vector copies"
    (`normal` `[review]`). No `**Spec:**` field, so no spec edit was authorized or made.

**Codex review:** No findings — "The new tests correctly enforce byte identity for all registered
fixture copies, detect unregistered tracked fixtures, and guard the ASCII-only boundary fixture."
Consistent with my probes, but a one-paragraph clean verdict is corroboration only; the six
mutations above are the actual evidence.

**Next:** Resume Unicode boundary propagation with the **C FFI + JNI/Java** slice (cheapest
in-container pair; napi stays most expensive because its checked-in
`crates/iscc-napi/iscc-lib.linux-x64-gnu.node` is stale and must be rebuilt first). Neither C FFI
nor JNI needs a vendored fixture copy — both compile in-tree and can read
`crates/iscc-lib/tests/unicode_boundary.json` by relative path the way WASM/Ruby do, which keeps the
new gate's table untouched. The later `packages/{dotnet,kotlin,swift}` slice is the one that *will*
add copies: registering each in `VENDORED_COPIES` is a required part of that step's scope, and
`test_no_unregistered_tracked_copy` reddening is the gate working, not a bug.

**Notes:**

- No `decisions.md` entry this iteration. The one judgment call (pytest anchor rather than a prek
    hook) is already recorded in learnings.md as an operational rule — "a `files:`-scoped hook never
    sees deletions, pair it with a pytest anchor" — and probe (2) above upgraded it from reasoning
    to measurement. Restating it in decisions.md would be duplication.
- The gate reads the **index**, not the working tree: a copy that is staged but not yet committed is
    already caught, while an untracked one is invisible by design (untracked build outputs such as
    `packages/kotlin/build/resources/test/data.json` and `.venv/…/iscc_core/data.json` exist in this
    container and would have red an `rglob`-based gate for no reason — the `.venv` one is a
    *different* file).
- The ASCII guard covers only the *canonical* boundary fixture, which is sufficient: a decoded copy
    is caught transitively by byte-identity, and a re-`cp`'d pair is caught by the canonical guard.
    Probe (3) confirmed both halves fire together.
- Reviewing a gate that touches the git index: build the probe tree with
    `git archive HEAD | tar -x -C /tmp/x && git init && git add -A && git commit`. `git clone` from
    `/workspace/iscc-lib` fails in this container (`safe.directory` ownership), and mutating the
    real index during a CID iteration risks colliding with the runner.
- No API, hot path, baseline or crate source was touched, so the CRAP, iai and semver gates cannot
    react to this iteration.
