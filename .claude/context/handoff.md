## 2026-11-22 — Review of: napi installs from a committed `package-lock.json` with `npm ci` in CI and release

**Verdict:** PASS_WITH_NOTES

**Summary:** The napi lockfile is committed, the `nodejs` CI job and both `release.yml` napi jobs
install with `npm ci`, and `version_sync.py` keeps the lock's two root version fields in step. CI is
green again: run 44966745935 on `33d9b437` finished with every job successful. The one note: the
napi build does not work in the devcontainer checkout with the locked cli, only on a normal
filesystem (filed `low`).

**Verification:**

- [x] `git ls-files crates/iscc-napi/package-lock.json` prints the path, and `grep -c package-lock`
    on the napi `.gitignore` prints `0`
- [x] `grep -nE 'run: npm install'` over `ci.yml` and `release.yml` prints nothing. Lines 93, 325
    and 382 read `run: npm ci`
- [x] `rm -rf node_modules && npm ci && npx napi build --platform && npm test`, then a clean
    `git status` — passes in a `git archive HEAD` copy under `/tmp` (`added 73 packages`,
    `# pass 238`, `# fail 0`, lock byte-identical, status empty) and in the CI `nodejs` job. **It
    fails in the checkout itself**: `napi build` exits 1 on the 9p mount (see Issues found)
- [x] The lock has 6 packages with `darwin` and 9 with `win32` in `os`. Every dependency edge of
    every locked package resolves inside the lock (only optional peers are absent), so no platform
    package was dropped
- [x] `mise run version:check` exits 0 and lists the lock, and the three pytest files pass (42
    tests)
- [x] `mise run check` exits 0. The pre-push gates passed too (clippy, cargo test, ty, pytest)

Beyond next.md: `npx npm@11 ci` installs the lock unchanged (the publish job runs Node 24, and no CI
run exercises it). A version bump through `version_sync.py` rewrites both lock fields, and `npm ci`
then leaves the bumped lock byte-identical.

**Issues found:**

- **`napi build` fails in the devcontainer checkout** with the locked `@napi-rs/cli` 3.10.12. The
    cli publishes its outputs through hard links, and the Windows 9p mount answers `link()` with
    `ENOENT`. Each failed run leaves untracked files behind, one of them a 20 MB copy of the addon.
    The step did not choose the version (`^3` resolved to it for every fresh install already). Filed
    `low`, untriaged. Until it is fixed, build napi in a `git archive` copy under `/tmp`
    (learnings.md)
- **Partial version-sync file lists** in `docs/development.md` (9 of 22 targets) and the release
    skill (13 of 22) now also lack the lockfile. Filed `low`, untriaged
- Fixed here as file-list realignment, as advance asked: `specs/ci-cd.md` (the npm manifest row and
    the lockfile list of the dependency-freshness criterion) and the "Version Sync" table in
    `specs/nodejs-bindings.md` now name the lock
- Not filed: `publish-npm-lib` runs `npm ci` although `npm publish` needs no dev dependency (the
    package has no `prepack` or `prepublishOnly` script). Dropping that step would remove the last
    install from the job that holds the OIDC token. next.md called this job `napi-publish`

**Codex review:** No actionable findings. Codex ran the 42 Python tests, the version check, clean
installs with npm 10 and npm 11, the addon build and the 238 Node tests itself, all green.

**Next:** The vcpkg portfile hashes (`normal` `[audit]`): `packages/cpp/portfile.cmake` still pins
the SHA512 values of the v0.2.0 tarballs, so the port has failed its hash check for five releases.
After that, the `@iscc/lib` npm README result-type sentence (`normal`), a docs-only step.

**Notes:**

- I pushed the define-next and advance commits before writing this handoff, to read the CI result.
    The `nodejs` job log shows `npm ci` → `added 73 packages`, `found 0 vulnerabilities`, then
    `# pass 238`. The `version-check` job prints `OK: crates/iscc-napi/package-lock.json = 0.7.0`
- The lock pins `@napi-rs/cli` 3.10.12, which is what CI has resolved since 2026-11-03. Its
    generated `index.d.ts` differs from the one cli 3.5.1 wrote: it adds an exported constant
    `__napiBindingTarget` and writes `export declare const` for the constants. The next `@iscc/lib`
    release carries that; a step that documents or gates the napi type surface should build with the
    locked cli
- The lock is refreshed only from scratch (`rm -rf node_modules package-lock.json && npm install`).
    A lock written over an existing `node_modules` can drop other platforms' packages, and
    `build-napi` runs `npm ci` on macOS and Windows, where no CI run would notice
- New decision: "napi dev dependencies install from a committed lockfile with `npm ci`". No Rust
    change, so no CRAP, iai, semver or API impact
