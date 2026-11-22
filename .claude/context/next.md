# Next Work Package

## Step: Every WASM snippet initializes the module; WASM agent guidance names only the `--target web` build

## Goal

Picks up "WASM init and build-target guidance is wrong outside the two pages a test guards"
(`normal`, `[audit]`). The getting-started WASM tabs are the first code a WASM user runs and fail
without `init()`; the agent guidance tells roles to build and probe targets that are never
published. Guard both with `tests/test_wasm_docs.py`.

## Alternatives Considered

- **Chosen:** WASM init docs — user-facing defect on the first page a WASM user reads; the last
    three iterations (289-291) were internal dedup, so a user-facing item is due. Docs and one test
    only: no hot path, no CRAP or iai exposure
- **Rejected:** Part 2 of the runner issue (one wrong-type rule for `conformance.rs` and
    `conformance.go`) — test-infra only, needs a decisions.md entry and probably a CRAP baseline
    refresh; next in line. `version:check` gaps (1 script) also waits a turn

## Scope

- **Modify**: `docs/tutorials/getting-started.md`, `README.md`, `docs/index.md`,
    `crates/iscc-wasm/CLAUDE.md`, `.claude/context/specs/wasm-bindings.md`,
    `tests/test_wasm_docs.py`
- **Reference**: `docs/howto/wasm.md` → "Setup" (the correct three forms),
    `crates/iscc-wasm/README.md` (already correct, guarded), `.github/workflows/release.yml:439`
    (the published build line)

## Not In Scope

- `docs/howto/wasm.md` per-function snippets: they omit `init()` under the page's "Setup" section,
    which says to initialize first. That page-level note is the accepted form; leave them
- Adding `crates/iscc-wasm/README.md` or the root README to the existing `WASM_DOCS` list as is:
    `test_wasm_docs_name_sum_code_result` would then demand `WasmSumCodeResult` on pages that need
    not name it. Use a separate list for the snippet rule
- `notes/` (frozen design record), agent memory files, `learnings-archive.md`
- Changing the published build (`release.yml`) or publishing a second target

## Implementation Notes

Facts: `@iscc/wasm` is ONE `wasm-pack build --target web --release` build (with
`RUSTFLAGS=-C target-feature=+simd128`, `--features conformance`). A bare `await init()` works in
browsers and bundlers and fails under Node (fetch of a `file:` URL); Node needs
`initSync({ module: readFileSync(require.resolve("@iscc/wasm/iscc_wasm_bg.wasm")) })`, exactly as
`docs/howto/wasm.md` → "### Node.js" shows.

Pages and sites (line numbers at HEAD):

- `getting-started.md`: WASM tabs at ~182, ~267, ~340, ~408, ~503 import from `"@iscc/wasm"` with no
    init — add `init` to the import and `await init();` before the first call. The verify tab
    (~99-107) has a bare `await init();`. Every bare `await init();` gets a preceding comment line
    that names browsers/bundlers and points Node users to the WASM how-to (`initSync`), e.g.
    `// Browsers and bundlers; for Node.js see the WASM how-to (initSync)`. Keep it under 100
    columns inside the 4-space tab indent (mdformat does not reflow code, but stay readable)
- `README.md:279-286` and `docs/index.md:111-120`: same comment above the bare `await init();`
- `crates/iscc-wasm/CLAUDE.md`: rewrite "## Browser vs Node.js Targets" (:49-61) to state the one
    published target (`--target web`) and how each environment initializes it (link the how-to); "##
    Build Commands" (:63-77) shows only `--target web` builds, the release line being the
    `release.yml` command. :202-203: prove a marshalling change from Node on a release
    `--target web` build loaded with `initSync({ module })`, not on a `--target nodejs` package
- `specs/wasm-bindings.md`: "## Build Process" (:86-91) lists only the `--target web` build; :128
    "(`web`, plus `bundler` if published)" → `web` only; check :137 wording still reads right

Test (`tests/test_wasm_docs.py`, keep the existing tests):

- `javascript_blocks` matches only column-0 fences; tab content is indented 4 spaces. Add a helper
    (or extend it) that also finds indented fences and dedents the body
- `SNIPPET_PAGES = [README.md, docs/index.md, docs/tutorials/getting-started.md]`: every javascript
    block that imports from `"@iscc/wasm"` calls `init(` or `initSync(`; a block whose only init is
    a bare `init()` has a `//` comment line containing `Node`; no positional init
- `AGENT_GUIDANCE = [crates/iscc-wasm/CLAUDE.md, specs/wasm-bindings.md]`: neither contains
    `--target nodejs` or `--target bundler` (normalise whitespace first)
- One negative case per rule on a synthetic string (block without init; bare init without the Node
    comment; an indented tab block), so each rule is shown to fire
- Before writing an absence criterion, grep the current tree: the strings above appear in the two
    guidance files only (`notes/`, `.claude/agent-memory/`, `issues.md` are outside the check)

## Verification

- `uv run pytest tests/test_wasm_docs.py` passes, including the new snippet, guidance and negative
    tests
- `grep -nE -- '--target (nodejs|bundler)' crates/iscc-wasm/CLAUDE.md .claude/context/specs/wasm-bindings.md`
    prints nothing
- `uv run zensical build` exits 0
- `uv run pytest tests/test_check_spec_paths.py tests/test_check_tier1_docs.py` pass (spec and doc
    gates still green after the edits)
- `mise run check` exits 0

## Done When

All verification criteria pass and every `@iscc/wasm` snippet on the three snippet pages initializes
the module in a form labelled for where it works.
