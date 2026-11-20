## 2026-11-20 — Spec path gate stops accepting paths the specs do not have

**Done:** `scripts/check_spec_paths.py` now matches globs with its own `glob_regex` (`**` → `.*`,
`*` → `[^/]*`, all else `re.escape`d, used with `fullmatch`) in place of `fnmatch.fnmatchcase`.
`NOT_PATHS` is checked only in the backticked-span loop of `check_spec`, so tree entries no longer
consult it. The two replay tests run on a frozen path set instead of the live index. The real specs
are unchanged and still pass.

**Files changed:**

- `scripts/check_spec_paths.py`: `glob_regex` added; `resolves` drops `NOT_PATHS` and uses
    `glob_regex`; `check_spec` span loop does `path not in NOT_PATHS and not resolves(...)`; the
    `fnmatch` import is removed; the docstring Rules state both glob rules and say that `NOT_PATHS`
    covers backticked spans only, while `GENERATED` covers both forms
- `tests/test_check_spec_paths.py`: `OLD_GO_KNOWN` (built with `_known`, from the 8 `packages/go/`
    files next.md lists); `test_old_go_tree_reports_its_eleven_phantoms` uses it, and its
    `counts == (1, 20, 0)` and `OLD_GO_PHANTOMS` assertions are unchanged;
    `test_main_exits_nonzero_on_phantom` monkeypatches `tracked_paths` to `OLD_GO_KNOWN | KNOWN`.
    Three new tests: `test_single_star_stays_in_one_segment`,
    `test_not_paths_does_not_exempt_tree_entry`,
    `test_glob_double_star_crosses_segments_single_star_does_not`. The module docstring's
    replay-test sentence now says "frozen snapshot"

**Verification:**

- `uv run pytest tests/test_check_spec_paths.py -v`: 18 passed, including the three new tests by
    name
- `uv run scripts/check_spec_paths.py`: exit 0, prints "OK: every tree entry and backticked path in
    15 specs resolves." (this covers the swift-bindings `**` spans and the go-bindings `code_*.go`)
- `grep -n 'tracked_paths(csp.ROOT)' tests/test_check_spec_paths.py` prints lines 139 and 150 only,
    which sit inside `test_real_repo_passes` and
    `test_tracked_paths_include_ancestors_not_build_output`
- `grep -n fnmatchcase scripts/check_spec_paths.py` prints nothing
- `uv run prek run --files <both files>`: exit 0. The pre-push stage on the same files (clippy,
    cargo test, ty, ruff S, C901, pytest) passed too
- `uv run ty check`: "All checks passed!"
- Mutation probe: I put the HEAD version of the script plus the new test file in `/tmp`. All 3 new
    tests FAIL there: `[] == [...]` for the `*` case and the tree-entry case, and `AttributeError`
    on `glob_regex`. The 2 replay tests pass there, as expected, since they only changed their data
    source

**Next:** Ruby negative-Bignum truncation together with the two JNI issues (null-handle guard,
pending exception), as next.md's Alternatives section planned.

**Notes:**

- With the stricter matcher, `**/` needs at least one directory: `packages/demo/**/*.json` does not
    match `packages/demo/x.json`. That is stricter than gitignore's `**/`. No spec uses `**/` today,
    and next.md ruled out extra glob semantics, so I left it as is
- Advance `MEMORY.md` is down from 153 to 129 lines: the Documentation Files and Feature Flags
    sections moved to `MEMORY-archive.md`
- No Rust code, no public API and no hot path was touched, so CRAP and iai are not affected
