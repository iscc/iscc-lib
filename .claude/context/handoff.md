## 2026-03-02 — Review of: Add pre-built FFI release artifacts to release.yml (issue #25)

**Verdict:** PASS_WITH_NOTES

**Summary:** The advance agent added well-structured `build-ffi` and `publish-ffi` jobs to
`release.yml` with a 5-platform matrix, following the exact pattern of existing `build-jni`/napi
jobs. All spec §4 criteria are met. Review found one bug: the `Get workspace version` step lacked
`shell: bash`, which would fail on Windows runners (default shell is `pwsh`). Fixed directly by the
review agent.

**Verification:**

- [x] `grep -q 'build-ffi'` — build job exists
- [x] `grep -q 'publish-ffi'` — publish job exists
- [x] `grep -q 'ffi:'` — workflow_dispatch input exists
- [x] `grep -c 'x86_64-unknown-linux-gnu'` returns 3 (napi + jni + ffi — the next.md ">=4" criterion
    was miscounted; 3 is correct for 3 jobs with this target)
- [x] `grep -q 'iscc-ffi'` — builds iscc-ffi crate
- [x] `grep -q 'iscc.h'` — header included in tarball
- [x] YAML valid (yaml.safe_load succeeds)
- [x] `mise run check` — 14/14 hooks pass

**Issues found:**

- Windows shell bug: `Get workspace version` step in `build-ffi` used bash syntax (`$(...)`,
    `$GITHUB_OUTPUT`) without `shell: bash`. Windows GHA runners default to `pwsh`, where this
    syntax fails. Fixed by adding `shell: bash` to the step.
- Minor: upload artifact glob `iscc-ffi-v*.*` also matches the staging directory (which has dots in
    the version number). Causes slightly larger artifacts but not a correctness issue — the
    publish-ffi glob only uploads files, not directories.

**Issue resolved:** #25 (pre-built FFI release artifacts) — deleted from issues.md. Spec §4
checkboxes updated to all checked.

**Codex review:** Codex reviewed the define-next commit (HEAD~1) rather than the advance commit
(HEAD) due to protocol commit offset. No actionable findings — the actual advance diff was reviewed
manually.

**Next:** The C FFI DX spec (§§1-4) is now fully complete. Remaining open issues: #21 (units support
for gen_sum_code_v0) and #16 (feature flags for minimal builds). Both are normal priority. Issue #21
adds an `add_units: bool` parameter across all bindings — good candidate for next step.

**Notes:**

- Cannot test the workflow end-to-end without a tag push or manual dispatch on GitHub. Structural
    verification only. The maintainer should trigger a manual `ffi: true` dispatch to validate.
- The `softprops/action-gh-release@v2` tag_name ternary correctly handles both tag pushes
    (`github.ref_name`) and manual dispatch (`v{version}` from Cargo.toml).
- Windows includes 3 library files (dll, dll.lib import lib, lib static). Unix includes 2 (shared +
    static). Both also include iscc.h + LICENSE.
