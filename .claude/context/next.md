# Next Work Package

## Step: Spec path gate stops accepting paths the specs do not have

## Goal

Fix the `normal` issue "`scripts/check_spec_paths.py` accepts paths the specs do not have": `*`
stays inside one path segment, `NOT_PATHS` exempts only backticked spans, and the two replay tests
stop reading the live index. The gate then checks what its docstring says it checks.

## Alternatives Considered

- **Chosen:** spec path gate leniency. The handoff named it, it was second in line in the last two
    handoffs, and it touches one script plus its test file, with no binding build needed.
- **Rejected:** Ruby negative-Bignum truncation (`normal`). It also fits one step, but it needs a
    native gem rebuild to verify. It comes next, together with the two JNI issues, which share one
    crate.

## Scope

- **Modify**: `scripts/check_spec_paths.py` (glob matcher, `NOT_PATHS` scope, docstring Rules)
- **Modify**: `tests/test_check_spec_paths.py` (fixed path set for the two replay tests + 3 new
    tests)
- **Reference**: `.claude/context/specs/swift-bindings.md` (lines ~331-338, 429: the `**` spans),
    `.claude/context/specs/go-bindings.md` (tree entry `code_*.go`)

## Not In Scope

- No spec edits. The real specs must still pass unchanged, and if one fails that is a finding to
    report, not something to work around by loosening the gate again.
- No new `GENERATED`/`NOT_PATHS` entries, no `?`/`[...]` glob classes beyond what falls out of the
    matcher, and no change to the floors or the tree parser.
- Do not touch the `.pre-commit-config.yaml` hook or `test_real_repo_passes`. The live-index check
    stays in that one test on purpose.

## Implementation Notes

- **Glob:** replace `fnmatch.fnmatchcase` with a small pure function that turns the pattern into an
    anchored regex: `**` → `.*` (may cross `/`), `*` → `[^/]*`, everything else `re.escape`d.
    Translate `**` before `*`. `crates/iscc-lib/**` and `packages/swift/Sources/**` must still
    resolve against the real index, and `packages/go/code_*.go` (go-bindings.md tree) must still
    resolve. Drop the `fnmatch` import if it becomes unused. Update the docstring Rules bullet ("A
    path containing `*`…") to state both rules.
- **`NOT_PATHS`:** move the `path in NOT_PATHS` test out of `resolves()` into the backticked-span
    loop of `check_spec` (or give `resolves` a keyword flag). Tree entries must not consult it.
    `GENERATED` stays valid for both forms. Update the docstring sentence on the exemption dicts to
    say `NOT_PATHS` covers backticked spans only.
- **Replay tests:** `test_old_go_tree_reports_its_eleven_phantoms` must check against a frozen set
    built with the existing `_known(...)` helper. Use the `packages/go/` files the old tree names
    that exist (go.mod, go.sum, codec.go, codec_test.go, dct.go, dct_test.go, testdata/data.json,
    README.md). Keep its `counts == (1, 20, 0)` and `OLD_GO_PHANTOMS` assertions exactly.
    `test_main_exits_nonzero_on_phantom` must `monkeypatch.setattr(csp, "tracked_paths", …)` to
    return that same frozen set (a lambda taking `root`) next to the existing `SPEC_DIR` patch. The
    `GOOD_PROSE` it appends needs `scripts/tool.py` and `packages/demo`, so include them or use the
    union with `KNOWN`. Its expected message stays the same.
- **Three new tests**, one bad case each, written in the style of the existing `_errors` mutation
    tests:
    1. `*` does not cross `/`: with `KNOWN` (which holds `packages/demo/testdata/data.json` and no
        top-level JSON), a backticked `packages/demo/*.json` fires, while `packages/demo/**/*.json`
        or `packages/demo/**` resolves. Assert the exact single error string.
    2. `NOT_PATHS` does not exempt a tree entry: a fence with root `scripts/` and an entry
        `└── gen_ffi_boundary_vectors.py` fires with "tree entry ... is not tracked". The same string
        backticked in prose still passes (it already does through `GOOD_PROSE` for
        `packages/go/vX.Y.Z`, so the tree case is the new assertion).
    3. A unit test on the glob helper/`resolves`: `**` crosses segments and `*` does not.
- Update the module docstring of the test file only where it describes the replay test as running
    against "the real tracked set".
- pytest, script-only step: no Rust, CRAP or iai impact.

## Verification

- `uv run pytest tests/test_check_spec_paths.py` exits 0, and the three new tests are among those
    collected (`-v` shows them)
- `uv run scripts/check_spec_paths.py` exits 0 on the real specs (the `**` spans and `code_*.go`
    still resolve)
- `grep -n 'tracked_paths(csp.ROOT)' tests/test_check_spec_paths.py` shows only lines inside
    `test_real_repo_passes` and `test_tracked_paths_include_ancestors_not_build_output`
- `grep -n fnmatchcase scripts/check_spec_paths.py` prints nothing
- `uv run prek run --files scripts/check_spec_paths.py tests/test_check_spec_paths.py` exits 0
- `uv run ty check` exits 0

## Done When

The advance agent is done when all six verification checks pass with the real specs unchanged.
