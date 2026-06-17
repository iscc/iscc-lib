## 2026-06-17 — Review of: Remove dangling npm `optionalDependencies` injection from `@iscc/lib` (#38)

**Verdict:** PASS

**Summary:** The advance agent deleted the single `Prepare npm packages`
(`npx napi prepublish -t npm`) step from the `publish-npm-lib` job — the only source of the five
unpublished `@iscc/lib-<triple>` `optionalDependencies` that 404 on install and break `npm ci`.
Three docs were updated to describe the bundled single-package model, matching the
`nodejs-bindings.md` spec. The change is minimal, in scope, and fully verified locally; issue #38 is
resolved.

**Verification:**

- [x] `grep -c "napi prepublish" .github/workflows/release.yml` → `0`
- [x] package.json assertion (`optionalDependencies` absent, `*.node` in `files`) → `ok`;
    package.json git-clean
- [x] `npm install && npm run build` → INSTALL OK / BUILD OK (regenerated `index.js` + local
    `.node`)
- [x] Bundled-loader proof → `loader ok` (no `@iscc/lib-<triple>` optional-dep packages present)
- [x] `npm pack --dry-run` ships `README.md`, `index.js`, and `iscc-lib.linux-x64-gnu.node`
- [x] `grep -rn "prepublish"` across the three docs → no matches
- [x] build does NOT inject `optionalDependencies` (post-build check → `absent (good)`)
- [x] `mise run check` → all 15 hooks Passed (clean, no reflow this cycle)
- [x] actionlint not installed → validated YAML structurally via PyYAML; `publish-npm-lib` step list
    intact and correct (checkout → setup-node → npm install → download napi-\* → version → check →
    publish)

**Issues found:**

- (none) — the two minor in-scope doc corrections (stale `sync_versions.py` → `version_sync.py`,
    dropped hardcoded `0.3.1`) are justified fixes inside a file already being edited; no behavior
    change.

**Codex review:** Clean — "The workflow change cleanly removes the prepublish step that injected
dangling npm optionalDependencies… I did not find a blocking regression introduced by this commit."
No actionable findings.

**Next:** Three `normal`/[human] backlog items remain, all good candidates for define-next:

1. **PyO3 0.23 → 0.29 security migration** (clears two RustSec advisories) — security-relevant but a
    six-minor-version jump with breaking changes per minor. Should be split (e.g. 0.23→0.24 first),
    scoped to `crates/iscc-py/` only (core has no PyO3 dep). Likely too large for one CID step.
2. **`cargo-semver-checks` API backward-compat CI gate** — the most self-contained of the CI-gate
    issues; wants to land before v1.0.0.
3. **Rust coverage + CRAP gate** and **`iai-callgrind` perf gate** — larger CI build-outs.

Recommend define-next pick either the PyO3 0.23→0.24 first increment (security) or the
`cargo-semver-checks` gate (smallest, pre-1.0 enabler).

**Notes:**

- npm bundled-model is now spec-aligned across `nodejs-bindings.md`, `iscc-napi/CLAUDE.md`,
    `notes/02-language-bindings.md`, and `notes/06-build-cicd-publishing.md`. No per-platform
    sibling publishing exists anywhere; revisit only if the tarball exceeds ~30 MB.
- No code, API, conformance, or performance surface was touched — release-workflow + docs only.
- `npm install` step retained in `publish-npm-lib` (now only installs `@napi-rs/cli`, technically
    unused) per next.md's minimal-diff intent. Harmless; could be removed in a future cleanup but
    not worth a dedicated step.
- Generated `index.js`/`*.node` are gitignored; the local `npm run build` left no tracked changes.
