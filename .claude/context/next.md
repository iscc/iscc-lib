# Next Work Package

## Step: napi installs from a committed `package-lock.json` with `npm ci` in CI and release

## Goal

CI is red on `bd79145`. The `nodejs` job's unlocked `npm install` fetched `@inquirer/ansi@2.1.1` two
minutes after it was published and got a registry 404. The tarball serves HTTP 200 now, so the red
was transient. Commit the napi lockfile and switch every napi install to `npm ci`, as
`specs/nodejs-bindings.md` → "CI Job" already shows. Then a just-published transitive version can no
longer break a run, and it can no longer reach the `id-token: write` publish job.

## Alternatives Considered

- **Chosen:** lockfile + `npm ci`. Red CI comes first, and an external-flake red still gets
    hardened. The spec already says `npm ci`. Locking also closes a supply-chain hole:
    `napi-publish` installs an unlocked tree while it holds an OIDC token.
- **Rejected:** vcpkg portfile hashes (`normal`, the handoff's Next). They are the next user-facing
    step, but they come after CI is green and protected. Also rejected: a `min-release-age`
    `.npmrc`. It needs npm >= 11.10, and Node 22 ships npm 10.9, which ignores the key without a
    word.

## Scope

- **Create**: `crates/iscc-napi/package-lock.json` (generated, not counted),
    `tests/test_version_sync.py`
- **Modify**: `crates/iscc-napi/.gitignore`, `.github/workflows/ci.yml` (`nodejs` job),
    `.github/workflows/release.yml` (the two `npm install` steps in `crates/iscc-napi`, lines ~325
    and ~382), `scripts/version_sync.py`, `.claude/context/specs/ci-cd.md` (line 26 table row),
    `crates/iscc-napi/CLAUDE.md` (only if it describes the install step)
- **Reference**: `.claude/context/specs/nodejs-bindings.md` → "CI Integration",
    `scripts/check_ci_job_table.py` (the gate that parses the ci-cd.md table)

## Not In Scope

- Bumping `@napi-rs/cli` or any other dependency. Lock what `^3` resolves to today
- Adding Dependabot, a `packageManager` field, `setup-node` caching, or `--ignore-scripts`. No
    locked package has an install script today
- Any `@iscc/wasm` npm step (`release.yml` ~432-501 uses no `npm install`) and
    `scripts/test_install.py` (it installs the published packages into scratch projects)
- The napi README result-type sentence (`normal`). That is its own docs step

## Implementation Notes

- Generate the lock **fresh**: `rm -rf node_modules package-lock.json && npm install` in
    `crates/iscc-napi`. A lock written over an existing `node_modules` can drop other platforms'
    optional packages (npm/cli#4828). The local stale lock (root version 0.6.0, @napi-rs/cli 3.5.1)
    has 46 `os`/`cpu` packages. The fresh lock must also carry darwin and win32 entries, because
    `build-napi` runs `npm ci` on macOS and Windows, where no CI run exercises it.
- Remove the `package-lock.json` line from `crates/iscc-napi/.gitignore` and keep `node_modules/`.
- The lock repeats the package version twice: top-level `version` and `packages[""].version`.
    Register the lock as a `TARGETS` entry in `scripts/version_sync.py` (21 → 22). Use a get/sync
    helper pair that reads and writes **both** fields; check fails if either one differs. Keep
    `json.dumps(indent=2)` + trailing newline, which is the byte format npm writes; confirm the sync
    leaves the file byte-identical when the versions already match. Update the module docstring
    list. `tests/test_version_sync.py` loads the script by path (pattern:
    `tests/test_check_ci_job_table.py`) and covers: sync rewrites both fields, get reports a
    mismatch if only one field is stale, and the real tree passes check.
- `setup-node@v7` turns caching on only when `package.json` has a `packageManager` /
    `devEngines.packageManager` field. A lockfile does not trigger it, so leave the `setup-node`
    steps alone (https://github.com/actions/setup-node/blob/v7/README.md).
- `npm publish` never packs `package-lock.json`, so the published tarball does not change.
- Update the `nodejs` row in `specs/ci-cd.md` to `npm ci`. Then run `git grep -n 'npm install'` over
    the workflows, specs and the napi `CLAUDE.md` for any other claim about the napi install.

## Verification

- `git ls-files crates/iscc-napi/package-lock.json` prints the path, and
    `grep -c package-lock crates/iscc-napi/.gitignore` prints `0`
- `grep -nE 'run: npm install' .github/workflows/ci.yml .github/workflows/release.yml` prints
    nothing
- In `crates/iscc-napi`: `rm -rf node_modules && npm ci && npx napi build --platform && npm test`
    exits 0, and afterwards `git status --porcelain crates/iscc-napi` prints nothing
- `python3 -c` over the lock finds at least one package with `"os"` containing `darwin` and one with
    `win32`
- `mise run version:check` exits 0, and
    `uv run pytest tests/test_version_sync.py tests/test_check_ci_job_table.py tests/test_check_release_workflow.py`
    passes
- `mise run check` exits 0

## Done When

All six verification criteria pass: napi dependencies install only from the committed,
version-synced lockfile, in CI and in both release jobs.
