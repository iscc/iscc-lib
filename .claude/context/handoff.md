## 2026-02-23 — Infrastructure setup and release workflow update (Titusz + Claude)

**Done:**

- **GitHub Pages enabled** with custom domain `lib.iscc.codes` — docs workflow now fully operational
- **`@iscc` npm org created** on npmjs.com — scope ready for `@iscc/lib` and `@iscc/wasm`
- **`NPM_TOKEN` repo secret added** — granular access token for first npm release
- **Release workflow updated**: bumped Node.js from 20 to 22 in all npm-related jobs (`build-napi`,
    `publish-npm-lib`, `publish-npm-wasm`) to satisfy npm trusted publishing requirement (npm >=
    11.5.1, Node >= 22.14.0)

**Files changed:**

- `.github/workflows/release.yml`: Node version 20 → 22 in three jobs
- `.claude/context/state.md`: updated to reflect infrastructure setup
- `.claude/context/handoff.md`: this file

**Post-first-release TODO (for Titusz):**

1. After v0.1.0 release creates `@iscc/lib` and `@iscc/wasm` on npm, go to **npmjs.com → Package →
    Settings → Trusted publishing → GitHub Actions** for each package and configure:
    - Repository: `iscc/iscc-lib`
    - Workflow filename: `release.yml`
    - Environment: (leave blank)
2. Then update `release.yml` to remove `NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}` from both
    `publish-npm-lib` and `publish-npm-wasm` jobs
3. Delete the `NPM_TOKEN` secret from GitHub repo settings

**Unresolved:**

- `crates/iscc-napi/` has untracked local build artifacts (index.d.ts, index.js, .node binary,
    node_modules/, package-lock.json) — consider adding to `.gitignore`

**Next:** Project is ready for first release. Clean up napi build artifacts, then tag v0.1.0.
