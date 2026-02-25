## 2026-02-25 — Fix WASM howto package name and create PR to main

**Done:** Replaced all 20 occurrences of `@iscc/iscc-wasm` with `@iscc/wasm` in
`docs/howto/wasm.md`. Created a PR from `develop` → `main` covering both the WASM release build fix
and the docs correction.

**Files changed:**

- `docs/howto/wasm.md`: replaced `@iscc/iscc-wasm` → `@iscc/wasm` (20 occurrences via `replace_all`)

**Verification:**

- [x] `grep -c '@iscc/iscc-wasm' docs/howto/wasm.md` returns 0 — no old name remaining
- [x] `grep -c '@iscc/wasm' docs/howto/wasm.md` returns 20 — all occurrences corrected
- [x] `uv run zensical build` exits 0 — docs build succeeds
- [x] `gh pr list --json number,title,state -B main -H develop` — PR #3 open
- [x] `mise run check` passes — all 14 pre-commit hooks clean

**Next:** Merge the PR after CI passes (human-gated). Then tag a release on `main` to trigger the
release workflow. Human still needs to configure `NPM_TOKEN` repository secret and crates.io OIDC
trusted publishing before the release workflow can publish.

**Notes:** mdformat made a minor whitespace adjustment to the file during `mise run format` (line
76: removed a line break within the tip admonition). This is cosmetic and correct.
