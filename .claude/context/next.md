# Next Work Package

## Step: Fix WASM howto package name and create PR to main

## Goal

Fix the incorrect npm package name in `docs/howto/wasm.md` (`@iscc/iscc-wasm` → `@iscc/wasm`) and
create a PR from `develop` → `main` so the WASM release build fix and docs correction can be merged,
unblocking npm publishing.

## Scope

- **Create**: (none)
- **Modify**: `docs/howto/wasm.md` (replace all 20 occurrences of `@iscc/iscc-wasm` with
    `@iscc/wasm`)
- **Reference**: `docs/index.md` (already uses the correct `@iscc/wasm` name — use as confirmation),
    `.claude/context/learnings.md` (documents that correct name is `@iscc/wasm`)

## Not In Scope

- Triggering the release workflow — that's a human-gated operation (NPM_TOKEN and crates.io OIDC may
    not be configured yet)
- Bumping the version from 0.0.1 — npm packages haven't been published at 0.0.1 yet, so the current
    version is correct
- Merging the PR — create it and let CI run; the human will merge after reviewing
- Fixing any other documentation content (focus only on the package name replacement)

## Implementation Notes

1. **Package name fix**: Use `replace_all` in `docs/howto/wasm.md` to change `@iscc/iscc-wasm` →
    `@iscc/wasm`. This is a mechanical find-and-replace across all 20 occurrences (import
    statements, install commands, and prose references). The correct name `@iscc/wasm` is confirmed
    in:

    - `docs/index.md` (already correct)
    - `crates/iscc-wasm/README.md`
    - The release workflow's npm package patching logic
    - `learnings.md` ("WASM npm package name is `@iscc/wasm` (not `@iscc/iscc-wasm`)")

2. **Pre-format before committing**: Run `mise run format` before staging and committing. This
    ensures the markdown passes pre-commit hooks (mdformat, trailing whitespace, etc.).

3. **Create PR**: After committing and pushing the fix, create the PR:

    ```
    gh pr create -B main -H develop --title "Fix WASM howto package name and merge release fixes" --body "..."
    ```

    The PR body should summarize both changes on develop since the last merge to main:

    - WASM release build fix (wasm-opt bulk-memory config in `crates/iscc-wasm/Cargo.toml`)
    - WASM howto package name correction (`@iscc/iscc-wasm` → `@iscc/wasm`)

4. Do NOT merge the PR — just create it. CI must pass first, and the human should review.

## Verification

- `grep -c '@iscc/iscc-wasm' docs/howto/wasm.md` returns `0`
- `grep -c '@iscc/wasm' docs/howto/wasm.md` returns `20` (all occurrences corrected)
- `uv run zensical build` exits 0 (docs still build)
- `gh pr list --json number,title,state -B main -H develop` shows exactly one open PR
- `mise run check` passes (all pre-commit hooks clean)

## Done When

All verification criteria pass: the package name is corrected in `docs/howto/wasm.md`, docs build
succeeds, pre-commit hooks are clean, and a PR from `develop` → `main` is open.
