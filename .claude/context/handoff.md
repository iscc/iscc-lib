## 2026-11-22 — Every WASM snippet initializes the module; WASM agent guidance names only the `--target web` build

**Done:** The five getting-started WASM tabs that called exports without initializing now
`import init` and `await init();`. Every bare `await init();` on the three snippet pages (the
tutorial's six tabs, README, docs home) has a comment above it:
`// Browsers and bundlers; for Node.js see the WASM how-to (initSync)`. The iscc-wasm `CLAUDE.md`
and `specs/wasm-bindings.md` now name only the published `--target web` build. They give the release
command exactly as `release.yml` runs it and say how each environment initializes the module.
`tests/test_wasm_docs.py` guards both rules.

**Files changed:**

- `docs/tutorials/getting-started.md`: 5 tabs gain `init` import + labelled `await init();`; verify
    tab gains the comment
- `README.md`, `docs/index.md`: the comment above the bare `await init();`
- `crates/iscc-wasm/CLAUDE.md`: "Browser vs Node.js Targets" → "Published Target and Initialization"
    (one target, browsers/bundlers `init()`, Node `initSync({ module })`, link to how-to). "Build
    Commands" has a dev `--target web` build and the `release.yml` release line. The gotcha at the
    old :202 now says to prove marshalling on a release `--target web` build loaded with
    `initSync({ module })`
- `.claude/context/specs/wasm-bindings.md`: "Build Process" now has a paragraph on the one published
    target and a single `--target web` command. "(`web`, plus `bundler` if published)" became "the
    published `web` target", and the Verified-when bullet "for all published targets" became "for
    the published `web` target"
- `tests/test_wasm_docs.py`: helpers `indented_javascript_blocks` (finds fences at any indent and
    dedents them), `snippet_problems` and `unpublished_targets`, plus 9 tests: the snippet rule over
    `SNIPPET_PAGES`, a ≥6-tab reach check on the tutorial, the guidance rule over `AGENT_GUIDANCE`,
    and one negative case for each rule (missing init, unlabelled bare init, positional init, other
    package ignored, indented fence, target named across a line break)

**Verification:**

- `uv run pytest tests/test_wasm_docs.py` → 17 passed (8 existing + 9 new)
- Real regression check: I ran the helpers over the `git show HEAD:` copies of the 5 pages. The
    tutorial reports 5× "no init() or initSync() call" plus 1× unlabelled bare init. README and
    index report the unlabelled bare init. Both guidance files report
    `['--target nodejs', '--target bundler']`
- `grep -nE -- '--target (nodejs|bundler)' crates/iscc-wasm/CLAUDE.md .claude/context/specs/wasm-bindings.md`
    → no output (exit 1)
- `uv run zensical build` → exit 0, "No issues found"
- `uv run pytest tests/test_check_spec_paths.py tests/test_check_tier1_docs.py` → 58 passed
- `mise run check` → exit 0 on the second run. The first run only applied mdformat (reflowed the two
    guidance files) and ruff-format (one test file). I re-ran the WASM test after the reformat: 17
    passed

**Next:** Part 2 of the runner issue (one wrong-type rule for `conformance.rs` and `conformance.go`,
needs a decisions.md entry and a CRAP baseline check), or the `version:check` gaps.

**Notes:**

- The guidance rule applies the "no unpublished target" check only to the two `AGENT_GUIDANCE`
    files. Before writing it I grepped the tree: the strings also appear in `notes/`, agent memory
    and archives, which are outside the check
- The snippet rule counts a bare `init()` as labelled when any `//` comment line in the block
    contains `Node`. That is deliberately loose: it checks that a label exists, not its wording
- `docs/howto/wasm.md` per-function snippets are untouched, as Not In Scope says. They are not in
    `SNIPPET_PAGES`, so the rule does not apply to them
- No Rust, no hot path, no public API change
