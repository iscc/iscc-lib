## 2026-11-22 — Review of: `publish-ffi` prunes `starter` FFI assets before uploading and fails unless every staged archive is `uploaded`

**Verdict:** PASS

**Summary:** `publish-ffi` now deletes the assets of its staged archives that are not in state
`uploaded`, uploads with `overwrite_files: false` as before, and then fails unless every staged
archive is `uploaded` on the release. The logic is in the stdlib-only
`scripts/ffi_release_assets.py`, with 8 unit tests and 4 wiring tests on the real `release.yml`. The
diff stays inside the scope of next.md (2 non-test, non-doc files) and weakens no gate.

**Verification:**

- [x] `uv run pytest tests/test_ffi_release_assets.py tests/test_update_vcpkg_hashes.py tests/test_check_release_workflow.py`
    passes — 52 passed
- [x] `uv run python scripts/check_release_workflow.py` exits 0 — "passed release-workflow static
    checks"
- [x] `publish-ffi` has a `ffi_release_assets.py stale` step before the
    `softprops/action-gh-release` step and a `verify` step after it — asserted by
    `test_publish_ffi_prunes_before_upload_and_verifies_after`, and read in the job
- [x] `git grep -n 'has no such check: it uploads' -- ':!.claude'` prints nothing
- [x] `uv run ty check` and `uv run ruff check` on both new files exit 0 (also clean with
    `--select S,C901`, the pre-push rule set)
- [x] `uv run zensical build` exits 0 and `mise run check` exits 0 (every hook passed, nothing
    reformatted)

Three probes beyond next.md's list, all clean:

- The script against the real payload: `gh release view v0.7.0 --repo iscc/iscc-lib --json assets`
    has the keys `name`, `state` and `apiUrl`. With the five real archive names staged, `verify`
    exits 0 and `stale` prints nothing; one extra staged name makes `verify` exit 1 and name it.
- Both `run` bodies extracted with PyYAML and run under `bash -e` with a stub `gh`: two stale assets
    (`starter` and `open`) give two `gh api -X DELETE` calls, a failing DELETE fails the step, and
    an empty staging dir fails the step. Real `gh api` does not read stdin, so the `while read` loop
    sees every line.
- The action source at `v3`: with `overwrite_files: false` an existing name is skipped, and a 422
    `already_exists` is not retried, so the action fails. `prepare-release` creates the release
    (XCFramework) before `publish-ffi` runs.

**Issues found:**

- (none filed)
- Style only: `docs/development.md` says the job deletes archives "left in state `starter`"; the
    script deletes every staged-name asset that is not `uploaded` (`open` too). The docs sentence
    names the documented case and is not wrong.

**Codex review:** No actionable findings. Codex ran the 52 tests, the static gate, lint and its own
mocked shell failure paths. It could not check live GitHub state (its `gh` is not authenticated).

**Next:** "Rust public API docs misclassify `IsccError`, omit `SumHasher`, and contradict
`gen_sum_code_v0`" (`normal`, `[audit]`, 4-5 non-test files). next.md already named it as the
following step. It needs one design call first: classify `streaming::SumHasher` as a Tier 1
re-export or as documented Tier 2. Scope every false sentence with a repo-wide `git grep` on its key
phrase.

**Notes:**

- Deleted from issues.md: "`publish-ffi` skips a half-uploaded FFI archive on a rerun". Its
    "Resolved when" allowed removing `starter` assets before the upload instead of a size or digest
    check. The wiring tests pin the order, and `docs/development.md` says what an `ffi` re-trigger
    does to existing archives.
- New decisions.md entry (2026-11-22): `publish-ffi` checks the asset state only. An uploaded
    archive is never compared or replaced, because an `ffi` re-trigger rebuilds bytes that can
    differ from the ones the tagged portfile pins. Do not add a digest check. The runner helper
    rotated the oldest entry into `decisions-archive.md` to keep the file under 400 lines.
- Not proven, and not provable without a failed upload: that GitHub lists a live `starter` asset and
    lets it be deleted. Every other outcome fails the job: an unlisted leftover gives a 422 on
    upload, or the verify step reports the archive missing.
- `release.yml` is never run by CI. The gates for this change are the static check (prek hook) and
    the wiring tests.
- No Rust change: the CRAP, iai and semver gates are untouched. 5 `normal` issues remain.
