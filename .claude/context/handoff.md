## 2026-11-22 — `publish-ffi` prunes `starter` FFI assets before uploading and fails unless every staged archive is `uploaded`

**Done:** Added `scripts/ffi_release_assets.py`, a stdlib-only script with two modes, `stale` and
`verify`, built from pure functions. `publish-ffi` now has a prune step before the softprops upload.
It deletes `starter` assets through their `apiUrl`, and a missing release is not an error. After the
upload, a verify step fails the job unless every staged archive is `uploaded`.
`overwrite_files: false` is unchanged. `docs/development.md` now describes both the job and what an
`ffi` re-trigger does.

**Files changed:**

- `scripts/ffi_release_assets.py`: created. `staged_names` (the `iscc-ffi-v*.*` glob), `read_assets`
    (`{"assets": [...]}`), `stale_urls`, `verify_errors`, `main`. An empty staging dir makes both
    modes exit 1.
- `tests/test_ffi_release_assets.py`: created. 8 unit tests on fixtures in the real v0.7.0 asset
    shape, with one bad case per `verify` call. 4 wiring tests on the real `release.yml`: the order
    prune < upload < verify, both `run`s pass `ffi-staging`, the tag and `GH_TOKEN` come through
    `env` with no `${{` in `run`, and `overwrite_files is False`.
- `.github/workflows/release.yml`: two steps in `publish-ffi` only, each with a one-line "why"
    comment. The prune step runs `gh release view "$TAG" --json assets > assets.json 2> view.err`.
    It exits 0 only when stderr contains `release not found`; any other failure prints stderr and
    exits 1. It then writes `stale` into a file and runs a `while read` loop of
    `gh api -X DELETE "$url"`. There is no pipe, because the default `bash -e` has no pipefail. The
    verify step has no fallback. Both steps set `GH_REPO: ${{ github.repository }}`.
- `docs/development.md`: replaced the "has no such check" sentence and added one sentence to the
    re-publish paragraph.

**Verification:**

- `uv run pytest tests/test_ffi_release_assets.py tests/test_update_vcpkg_hashes.py tests/test_check_release_workflow.py`:
    52 passed (re-run after the formatters touched the files).
- `uv run python scripts/check_release_workflow.py`: "OK: … passed release-workflow static checks."
- `git grep -n 'has no such check: it uploads' -- ':!.claude'`: no output (exit 1).
- `uv run ty check`: "All checks passed!". `uv run ruff check` on both files is clean, and so is the
    pre-push `--select S,C901`.
- `uv run zensical build`: exit 0. `mise run check`: exit 0. The first run failed because mdformat
    and ruff-format reflowed the docs paragraph and one list comprehension; the second run was
    clean.
- Real regression: in a `/tmp` `git archive HEAD` tree with the new script and test but the HEAD
    `release.yml`, 3 wiring tests failed (order, ffi-staging, env) and 9 passed. As expected:
    `overwrite_files` and the unit tests do not depend on the new steps.
- Step bodies run as CI runs them: I extracted both `run` blocks with PyYAML and ran them under
    `bash -e` with a stub `gh` on PATH:
    - missing release → "nothing to prune", exit 0
    - a 502 on `release view` → exit 1
    - a `starter` asset → exactly one `gh api -X DELETE https://api/1`; the uploaded asset is not
        touched
    - verify with a `starter` asset → names the archive, exit 1
- `gh release view v99.99.99 --repo iscc/iscc-lib` prints `release not found` on stderr with exit 1.
    This is the string the prune step matches.

**Next:** The Rust API tier docs (`normal`, 4-5 files), which next.md names as the next step.

**Notes:**

- Ruff S105 flagged `step["env"]["GH_TOKEN"] == "${{ github.token }}"` in the test as a hardcoded
    password. I replaced it with `"github.token" in step["env"]["GH_TOKEN"]`. No suppression.
- `GH_REPO` was not in next.md. I added it so that `gh` names the repository explicitly instead of
    inferring it from the checkout's git remote. It is harmless if the inference would have worked.
- Repo-wide `git grep -n "overwrite"`: `packages/cpp/CLAUDE.md:145` ("a later `ffi` re-trigger
    cannot replace those archives; recover a failed publish with `gh run rerun --failed`") is still
    true and I left it unchanged. `git grep -n "uploads its tarballs"` finds nothing. The comment on
    `test_update_vcpkg_hashes.py::test_publish_ffi_never_overwrites_archives` is still accurate.
- Not proven: the real behaviour of GitHub's API on a `starter` asset (DELETE via `apiUrl`, and
    whether softprops then re-uploads it). That needs a live release. The `starter` state name comes
    from the issue.
- No Rust change: the CRAP, iai and semver gates are untouched.
