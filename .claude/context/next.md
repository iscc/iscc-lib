# Next Work Package

## Step: `publish-ffi` prunes `starter` FFI assets before uploading and fails unless every staged archive is `uploaded`

## Goal

Fix the `normal` issue "`publish-ffi` skips a half-uploaded FFI archive on a rerun". After a failed
upload (502), `gh run rerun --failed` must re-upload the empty `starter` asset instead of skipping
it by name and reporting success. The job must also fail when any staged archive is not on the
release in state `uploaded`.

## Alternatives Considered

- **Chosen:** prune + verify in `publish-ffi`. It is on the release path that v0.8.0 is cut through,
    and it is small (1 script, 1 workflow, 1 docs page, plus tests).
- **Rejected:** Rust API tier docs (`normal`, 4-5 files). It is docs only and does not risk a broken
    release. It goes next.
- **Rejected:** verify-only with no pruning. The rerun would still skip the `starter` asset, so the
    job would fail on every retry and never recover.

## Scope

- **Create**: `scripts/ffi_release_assets.py`, `tests/test_ffi_release_assets.py`
- **Modify**: `.github/workflows/release.yml` (`publish-ffi` job only), `docs/development.md` (the
    "The FFI job has no such check" sentence and the re-publish paragraph after it)
- **Reference**: `scripts/update_vcpkg_hashes.py` and `tests/test_update_vcpkg_hashes.py` (style,
    stdlib-only rule, wiring-test helpers `_jobs`/`_step_index`),
    `scripts/check_release_workflow.py`, `packages/cpp/CLAUDE.md:146` (names the rerun recovery)

## Not In Scope

- Replacing or digest-comparing an asset that is already `uploaded`. An `ffi`-only re-trigger runs
    in a new workflow run and rebuilds the archives. Those bytes can differ from the ones whose
    SHA512s the tagged portfile pins, so an uploaded asset is the correct one and must be kept. Keep
    `overwrite_files: false`.
- The release skill's post-release asset grep (`.claude/skills/release/SKILL.md`). The job now
    checks the result itself.
- Other `low` FFI/vcpkg issues (vcpkg post-build check, Windows CI).

## Implementation Notes

- Facts measured while scoping: `gh release view v0.7.0 --json assets` returns, per asset, `name`,
    `state` (`"uploaded"`), `size`, `digest` (`sha256:…`) and `apiUrl`
    (`https://api.github.com/repos/iscc/iscc-lib/releases/assets/<numeric id>`). `id` is a GraphQL
    node id, so delete through `apiUrl`. Do not use `id`.
- The script is **stdlib only**: it runs as `python3` on the runner. It does no network I/O. It
    reads an assets JSON file (the `gh release view --json assets` output) and the staging dir
    (`ffi-staging`, files matching `iscc-ffi-v*.*`, the same glob as the upload step). Suggested
    shape: two subcommands made of pure functions.
    - `stale <staging-dir> <assets.json>` prints the `apiUrl` of each asset whose name is a staged
        file and whose `state` is not `uploaded`, one per line.
    - `verify <staging-dir> <assets.json>` exits 1 and names every staged file that is missing from
        the release or is present in a state other than `uploaded`. Exit 0 otherwise.
    - Assets that are not staged (the XCFramework zip, archives of other targets) are ignored in both
        modes. An empty staging dir is an error (exit 1): a set check that passes on empty sets proves
        nothing.
- Workflow wiring in `publish-ffi`. The job already has a checkout and `contents: write`.
    - Add a step **before** the softprops upload. It writes the release's assets to a JSON file and
        runs `stale`, then deletes each printed URL with `gh api -X DELETE <url>`. It needs
        `env: GH_TOKEN: ${{ github.token }}` and the tag from `steps.version.outputs.version` passed
        through `env`, not inlined into `run`. If the release does not exist yet, there is nothing to
        prune and the step must not fail. Any other failure should still fail the step.
    - Add a step **after** the upload. It fetches the assets again and runs `verify`, with no
        fallback.
    - Docs comment: one line per step saying why.
- Wiring tests (put them in the new test file and reuse the YAML-reading pattern from
    `test_update_vcpkg_hashes.py`) must assert the order prune < upload < verify in `publish-ffi`,
    that each `run` calls the script with `ffi-staging`, and that `overwrite_files` is still
    `False`.
- Unit tests use real JSON fixtures (copy the asset shape above). Cover:
    - a `starter` asset is listed by `stale`;
    - an `uploaded` asset is not listed;
    - an unstaged `starter` asset is ignored;
    - `verify` fails on a missing archive and on a `starter` one, and names the file, with one bad
        case per call;
    - `verify` passes when every archive is uploaded;
    - an empty staging dir fails.
- Docs: replace the sentence at `docs/development.md:391` with what the job does now. It prunes
    `starter` leftovers, uploads only the missing archives, keeps existing uploaded ones (pinned by
    the portfile) and fails unless all of them end up `uploaded`. Then say what an `ffi` re-trigger
    does to archives that already exist. Repo-wide `git grep -n "overwrite"` and
    `git grep -n "uploads its tarballs"` afterwards (outside `.claude`) catch sibling claims.
- `release.yml` is never run by CI, so the gates are `scripts/check_release_workflow.py` (also a
    prek hook) and the wiring tests. No Rust change, so CRAP, iai and semver are untouched.

## Verification

- `uv run pytest tests/test_ffi_release_assets.py tests/test_update_vcpkg_hashes.py tests/test_check_release_workflow.py`
    passes
- `uv run python scripts/check_release_workflow.py` exits 0
- `publish-ffi` in `release.yml` has a step running `ffi_release_assets.py stale` before the
    `softprops/action-gh-release` step and one running `ffi_release_assets.py verify` after it
    (asserted by the wiring test)
- `git grep -n 'has no such check: it uploads' -- ':!.claude'` prints nothing
- `uv run ty check` and
    `uv run ruff check scripts/ffi_release_assets.py tests/test_ffi_release_assets.py` exit 0
- `uv run zensical build` exits 0 and `mise run check` exits 0

## Done When

All six verification criteria pass with `publish-ffi` pruning `starter` assets before upload and
failing unless every staged FFI archive is `uploaded` on the release.
