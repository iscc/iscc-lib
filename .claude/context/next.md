# Next Work Package

## Step: Remove dangling npm `optionalDependencies` injection from `@iscc/lib` (#38)

## Goal

Stop the release workflow from injecting five unpublished per-platform `optionalDependencies`
(`@iscc/lib-<triple>`) into the published `@iscc/lib` package, which 404 on install and break
`npm ci` (`EUSAGE` / missing-from-lockfile) for every downstream consumer. Resolves issue #38. The
bundled `files: ["*.node"]` model already ships all binaries and the generated `index.js` loader
already prefers them, so dropping the injection produces a clean, self-contained package.

## Scope

- **Modify**: `.github/workflows/release.yml` — delete the "Prepare npm packages" step
    (`run: npx napi prepublish -t npm`, ~line 378) from the `publish-npm-lib` job. This is the only
    code/config change.
- **Modify (docs, excluded from file limit)**:
    - `crates/iscc-napi/CLAUDE.md` — "Publishing Constraints" (~line 114) currently says
        `optionalDependencies` and the `npm/` subdirectory are "generated at publish time by
        `npx napi prepublish -t npm`". Rewrite to the bundled single-package model (all `.node`
        bundled via `files`, no `optionalDependencies`, no prepublish).
    - `notes/06-build-cicd-publishing.md` (~lines 288–291) — replace the per-platform-package /
        `optionalDependencies` description with the bundled single-package model.
    - `notes/02-language-bindings.md` (~lines 82–88) — same: replace "platform selection via
        `optionalDependencies`" with the bundled `index.js`-loads-local-`.node` model.
- **Reference**:
    - `.claude/context/specs/nodejs-bindings.md` → "Native Binary Distribution" (lines 107–132) — the
        authoritative target wording for the bundled model; reuse its phrasing in the doc edits.
    - `crates/iscc-napi/package.json` — already correct (`files: ["*.node"]`, no
        `optionalDependencies`); confirm, do not change.
    - `crates/iscc-napi/index.js` (generated loader) — each platform branch does
        `require('./iscc-lib.<triple>.node')` FIRST, only falling back to
        `require('@iscc/lib-<triple>')` on failure.
    - `issues.md` → issue #38 (fix decision + acceptance).

## Not In Scope

- **Do NOT publish to npm** or touch any other registry's publish job. Verification is entirely
    local (build + pack inspection + loader smoke test) — no real publish needed.
- **Do NOT modify `crates/iscc-napi/package.json`** — it already bundles all binaries and declares
    no `optionalDependencies`. Verify only.
- **Do NOT remove the `npm install` step** in `publish-npm-lib`. It is now technically unused (it
    only installed `@napi-rs/cli` for prepublish) but is harmless; leaving it keeps the diff minimal
    and avoids surprising reviewers.
- **Do NOT add per-platform sibling package publishing** or recreate the `npm/<triple>/` directory
    model. The bundled model is the deliberate, spec-mandated choice.
- **Do NOT bump napi-rs**, edit `crates/iscc-napi/src/lib.rs`, or touch the `build-napi` /
    `test-napi` jobs.
- **Do NOT start the PyO3 0.23 → 0.29 migration** or any other backlog item — one step only.

## Implementation Notes

- The exact step to delete from the `publish-npm-lib` job:
    ```yaml
      - name: Prepare npm packages
        run: npx napi prepublish -t npm
        working-directory: crates/iscc-napi
    ```
    After removal the job flow is: checkout → setup-node → `npm install` → download merged `napi-*`
    artifacts → get version → check version on registry →
    `npm publish --provenance --access public`.
- **Why this is safe (the crux of #38):** `napi prepublish` is the *only* thing that injects
    `optionalDependencies` into `package.json`. The generated `index.js` loader resolves the native
    addon by trying the bundled local file (`require('./iscc-lib.<triple>.node')`) first and only
    falls back to the per-platform package on failure. Because `files: ["*.node"]` ships all five
    `.node` binaries in the tarball, the local require always succeeds and the (now-undeclared)
    sibling packages are never needed at install or runtime.
- The `publish-npm-lib` job already merges all five `napi-*` artifacts into `crates/iscc-napi/`
    (plus `index.js`/`index.d.ts`), and `checkout` provides `README.md`, so `npm publish` ships
    exactly `files: [index.js, index.d.ts, *.node, README.md]` — a complete bundled package with no
    `optionalDependencies`.
- For the doc edits, mirror the wording already in `nodejs-bindings.md` → "Native Binary
    Distribution" so the binding CLAUDE.md and the architecture notes agree with the spec. Keep
    edits tight — just replace the stale per-platform/`optionalDependencies` sentences.

## Verification

- `grep -c "napi prepublish" .github/workflows/release.yml` returns `0`.
- `python -c "import json; d=json.load(open('crates/iscc-napi/package.json')); assert 'optionalDependencies' not in d; assert '*.node' in d['files']; print('ok')"`
    exits 0.
- `cd crates/iscc-napi && npm install && npm run build` succeeds (regenerates `index.js` + local
    `.node`).
- **Bundled-loader proof** (no `@iscc/lib-<triple>` optional-dep packages are installed in this
    environment):
    `cd crates/iscc-napi && node -e "const m=require('./index.js'); if(!m.conformance_selftest()) throw new Error('selftest failed'); console.log('loader ok')"`
    prints `loader ok` — proving the bundled `.node` loads with zero optional-dep packages present
    (the exact failure mode #38 reports).
- `cd crates/iscc-napi && npm pack --dry-run 2>&1 | grep -E "index\.js|\.node|README"` shows
    `index.js`, the local `*.node`, and `README.md` are included in the tarball.
- `grep -rn "prepublish" crates/iscc-napi/CLAUDE.md notes/06-build-cicd-publishing.md notes/02-language-bindings.md`
    returns no matches (stale per-platform wording removed from all three docs).
- If `actionlint` is available: `actionlint .github/workflows/release.yml` is clean (workflow YAML
    still valid after the step deletion).

## Done When

The `publish-npm-lib` job no longer runs `napi prepublish`, the bundled `index.js` loads the local
`.node` with no optional-dep packages present, `npm pack` ships all binaries with no
`optionalDependencies`, the three doc files describe the bundled single-package model, and all
verification commands pass.
