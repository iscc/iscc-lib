## 2026-11-22 — napi installs from a committed `package-lock.json` with `npm ci` in CI and release

**Done:** I generated the napi lockfile from scratch
(`rm -rf node_modules package-lock.json && npm install`, npm 10.9.9 / Node v22.23.5) and committed
it. The `nodejs` CI job and both `release.yml` napi install steps now run `npm ci`.
`version_sync.py` treats the lock as a 22nd target and keeps both of its root version fields in
sync.

**Files changed:**

- `crates/iscc-napi/package-lock.json`: created, lockfileVersion 3, 114 packages, `@napi-rs/cli`
    3.10.12 (what `^3` resolves to today)
- `crates/iscc-napi/.gitignore`: dropped the `package-lock.json` line, kept `node_modules/`
- `.github/workflows/ci.yml` (`nodejs`), `.github/workflows/release.yml` (`build-napi` line 325,
    `napi-publish` line 382): `npm install` → `npm ci`
- `scripts/version_sync.py`: `_get_package_lock_version` returns the version only when the top-level
    and `packages[""]` copies agree. Otherwise it returns a `MISMATCH(...)` string, which makes
    `--check` fail. `_sync_package_lock` writes both copies with `json.dumps(indent=2)` plus a
    trailing newline. Also a TARGETS entry and a docstring line
- `tests/test_version_sync.py`: created, 6 tests: the lock is registered, sync rewrites both fields,
    a stale `packages[""]` alone is caught, a stale top-level alone is caught, syncing the real lock
    leaves it byte-identical, the real tree passes `check_mode`
- `.claude/context/specs/ci-cd.md`: the `nodejs` row now says `npm ci`

**Verification:**

- `git ls-files` lists the lock (it is staged). `grep -c package-lock crates/iscc-napi/.gitignore`
    prints 0
- `grep -nE 'run: npm install'` over both workflows prints nothing. Lines 93, 325 and 382 now read
    `run: npm ci`
- Lock platforms: 43 packages carry `os`, 6 of them darwin and 9 win32 (for example
    `@napi-rs/lzma-darwin-arm64` and `@napi-rs/lzma-win32-arm64-msvc`). No package has
    `hasInstallScript`
- Round trip: `json.dumps(json.loads(lock), indent=2) + "\n" == lock` is True
- `mise run version:check` exits 0 and prints `OK: crates/iscc-napi/package-lock.json = 0.7.0`
- `uv run pytest tests/test_version_sync.py tests/test_check_ci_job_table.py tests/test_check_release_workflow.py`:
    42 passed
- `mise run check`: exit 0
- **napi build criterion, with a caveat.** In the checkout, `rm -rf node_modules && npm ci`
    succeeds. `npx napi build --platform` then fails twice in a row with
    `AggregateError:   Filesystem transaction failed and its recovery state could not be removed from   .../.napi-rs-filesystem-transaction.swp`.
    The cause is the cli 3.10.x filesystem transaction on this devcontainer's Windows 9p (drvfs)
    `/workspace` mount. To check that, I ran the same lock in a throwaway `git worktree` under
    `/tmp` (overlay fs), with `CARGO_TARGET_DIR=/workspace/iscc-lib/target`: `npm ci` →
    `napi build --platform` exit 0 → `npm test` `# pass 238`, `# fail 0`. `npm ci` left the lock
    byte-identical (`cmp`). I removed the worktree, and `git worktree list` shows only the checkout.
    I deleted the leftover `.napi-rs-filesystem-transaction.swp/` and `.*.tmp` files from the
    checkout, so `git status --porcelain crates/iscc-napi` shows only this step's `.gitignore` and
    the lock

**Next:** Vcpkg portfile hashes (`normal` `[audit]`), then the napi README result-type sentence.

**Notes:**

- **Devcontainer napi builds are broken on the 9p mount with any cli 3.10.x.** This did not start
    with this step. The old local, gitignored lock pinned cli 3.5.1, and any fresh unlocked
    `npm install` already pulled 3.10.12, published 2026-11-03. CI runners are unaffected (the probe
    above passed on overlay fs). Review may want to file it (`low`). Possible fixes: build from a
    Docker volume, or pin the cli below the 3.10 transaction code. Both are out of scope here: no
    bumps or pins
- The fresh lock has 43 `os` packages, the stale local lock had 46. The difference comes from the
    cli version (3.10.12 vs 3.5.1), not from dropped platforms: darwin and win32 entries are present
- Left alone as out of scope (next.md limited `ci-cd.md` to line 26): the "Manifest surface" table
    row `npm` → `crates/iscc-napi/package.json`, and "Lockfiles (`Cargo.lock`, `uv.lock`,
    `Gemfile.lock`) are regenerated" in the same spec. Both now leave out
    `crates/iscc-napi/package-lock.json`. Two one-line spec edits for review
- `crates/iscc-napi/CLAUDE.md` does not describe the install step, so it is unchanged. The other
    `npm install` hits in specs (`nodejs-bindings.md:175`, `wasm-bindings.md:191`) are about
    consumers installing published packages
- No Rust change: no CRAP, iai, semver or API impact
