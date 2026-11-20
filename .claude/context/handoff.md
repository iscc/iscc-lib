## 2026-11-20 — Review of: Spec path gate stops accepting paths the specs do not have

**Verdict:** PASS

**Summary:** `scripts/check_spec_paths.py` now matches globs with its own `glob_regex` (`**` crosses
`/`, `*` stays in one segment), honours `NOT_PATHS` for backticked spans only, and the two replay
tests run on a frozen path set. The diff touches one script and its test file, exactly as scoped,
and the real specs pass unchanged. One new test assertion was vacuous; fixed in this review (see
below). The issue is resolved and deleted.

**Verification:**

- [x] `uv run pytest tests/test_check_spec_paths.py` exits 0 with the three new tests collected: 18
    passed, `test_single_star_stays_in_one_segment`, `test_not_paths_does_not_exempt_tree_entry` and
    `test_glob_double_star_crosses_segments_single_star_does_not` shown by `-v`
- [x] `uv run scripts/check_spec_paths.py` exits 0 on the real specs: "OK … in 15 specs resolves".
    All 7 `*` paths in the specs (`packages/go/code_*.go`, six `**` spans in `swift-bindings.md`)
    resolve through the new matcher
- [x] `grep -n 'tracked_paths(csp.ROOT)' tests/test_check_spec_paths.py` shows lines 139 and 150
    only, inside `test_real_repo_passes` and `test_tracked_paths_include_ancestors_not_build_output`
- [x] `grep -n fnmatchcase scripts/check_spec_paths.py` prints nothing
- [x] `uv run prek run --files scripts/check_spec_paths.py tests/test_check_spec_paths.py` exits 0
- [x] `uv run ty check` exits 0 ("All checks passed!")
- [x] `mise run check` exits 0; no spec, hook config or `test_real_repo_passes` change in the diff

Extra probes (3 of 3):

- Real index: the issue's own example `packages/go/*.json` no longer resolves, while
    `packages/go/testdata/*.json` and `packages/go/**/*.json` do
- Previous script (`git show HEAD~1:…`) against the new tests: exactly the 3 new tests fail, the 2
    replay tests pass. This confirms advance's mutation claim
- Dropping `re.escape` from `glob_regex`: **all tests stayed green**. Fixed, see below

**Issues found:**

- Vacuous assertion, fixed in this review: the "regex metacharacters are literal" check used
    `packages/go/code_meta.g.`, which has no `*`, so `resolves` never built the regex. It now uses
    `packages/go/code_*.g.`, and the no-`re.escape` mutation fails that test. I also rewrapped one
    115-character comment in the same file
- Not filed: `packages/demo/**/*.json` does not match `packages/demo/x.json`, because `**/` needs at
    least one directory. That is stricter than gitignore, but it fails closed, no spec uses `**/`,
    and next.md excluded further glob semantics

**Codex review:** No actionable findings. It re-ran the 18 tests, the real-spec check, ruff and the
type check, all green. It did not notice the vacuous assertion.

**Next:** Ruby negative-Bignum truncation (`normal`): a negative Bignum element in
`gen_audio_code_v0`, `gen_video_code_v0` and `soft_hash_video_v0` must raise `RangeError`. It needs
a native gem rebuild to verify. The two JNI issues (null-handle guard, pending exception) share
`crates/iscc-jni/src/lib.rs` and make a natural step after it; whether they fit one step with the
Ruby fix is define-next's call.

**Notes:**

- Six `normal` issues remain: Ruby Bignum, JNI null handle, JNI pending exception,
    `crates/iscc-ffi/src/lib.rs` split, derived facts drift, `data-edge.json` harness compares
- No Rust code, public API or benchmarked path changed, so CRAP, iai and the feature matrix were not
    run
- `learnings.md` sits at its 200-line budget: the next new learning needs an entry archived first
