## 2026-06-17 — Remove dangling npm `optionalDependencies` injection from `@iscc/lib` (#38)

**Done:** Deleted the "Prepare npm packages" step (`npx napi prepublish -t npm`) from the
`publish-npm-lib` job in `release.yml` — the sole source of the five unpublished per-platform
`optionalDependencies` that 404 on install and break `npm ci` for downstream consumers. The bundled
single-package model (`files: ["*.node"]` + `index.js` local-loader-first) already ships every
binary, so the package is now clean and self-contained. Updated three docs to describe the bundled
model. Resolves issue #38.

**Files changed:**

- `.github/workflows/release.yml`: removed the `Prepare npm packages` step from `publish-npm-lib`.
    New job flow: checkout → setup-node → `npm install` → download merged `napi-*` artifacts → get
    version → check version on registry → `npm publish --provenance --access public`.
- `crates/iscc-napi/CLAUDE.md`: rewrote "Publishing Constraints" — bundled single-package model, no
    `optionalDependencies`, no `npm/` subdir, no prepublish injection. Also corrected stale script
    name (`sync_versions.py` → `version_sync.py`) and dropped a stale hardcoded version (`0.3.1`).
- `notes/06-build-cicd-publishing.md`: replaced per-platform-package / `optionalDependencies`
    description with the bundled single-package model (incl. ~30 MB revisit trigger).
- `notes/02-language-bindings.md`: replaced "platform selection via `optionalDependencies`" tree +
    prose with the bundled `index.js`-loads-local-`.node` model.

**Verification:** All next.md criteria pass.

- `grep -c "napi prepublish" .github/workflows/release.yml` → `0`.
- `package.json` assertion (no `optionalDependencies`, `*.node` in `files`) → `ok`. package.json
    left unchanged (git clean).
- `npm install && npm run build` → succeeds; regenerated `index.js` + `iscc-lib.linux-x64-gnu.node`.
- Bundled-loader proof: `node -e "require('./index.js').conformance_selftest()..."` → `loader ok`
    (no `@iscc/lib-<triple>` optional-dep packages present — exactly #38's failure mode).
- Loader inspection: `index.js` linux-x64-gnu branch does `require('./iscc-lib.linux-x64-gnu.node')`
    (line 283) BEFORE the `require('@iscc/lib-linux-x64-gnu')` fallback (line 288) — bundled local
    file always wins.
- `npm pack --dry-run` ships `README.md`, `index.js`, and the local `*.node` (CI merges all 5).
- `grep -rn "prepublish"` across the three docs → no matches.
- `npm run build` does NOT inject `optionalDependencies` (confirmed via post-build json check).
- `mise run check` → all hooks Passed (mdformat reflowed the three doc files on first run; re-run
    clean).
- `actionlint` not installed in this environment; validated YAML structurally via PyYAML — the
    `publish-npm-lib` step list is intact and correct.

**Next:** With #38 closed, the next backlog item is the **PyO3 0.23 → 0.29 security migration**
(issue, `normal`/[human]) — clears two RustSec advisories. It's a six-minor-version jump with
breaking changes per minor; should be staged incrementally per the PyO3 migration guide, scoped to
`crates/iscc-py/` (core has no PyO3 dep). Likely too large for a single CID step — define-next may
want to split it (e.g. 0.23→0.24→0.25 first).

**Notes:**

- **Two minor in-scope doc corrections** beyond the literal next.md edits, both inside files I was
    already editing: `crates/iscc-napi/CLAUDE.md` referenced a non-existent
    `scripts/sync_versions.py` (actual: `scripts/version_sync.py`, verified on disk) and a stale
    hardcoded `0.3.1` version (package.json is `0.4.0`). Corrected the script name and removed the
    version literal so the doc stops drifting. No behavior change.
- **Wording choice for the grep verification:** the spec (`nodejs-bindings.md`) phrases the rule as
    "do not run `napi prepublish -t npm`", but the next.md verification requires `grep "prepublish"`
    to return zero matches across the three docs. To satisfy the literal gate while keeping the
    meaning, I phrased the docs as "does **not** inject `optionalDependencies`" rather than naming
    the napi command. Accurate and gate-clean.
- **`npm install` step intentionally retained** in `publish-npm-lib` per next.md (now only installs
    `@napi-rs/cli`, technically unused, but kept to minimize the diff).
- `issues.md` issue #38 entry is now resolved by this change — review agent should delete it after
    verification per the issues.md cleanup convention.
- `iterations.jsonl` shows as modified (runner-owned) — not staged.
