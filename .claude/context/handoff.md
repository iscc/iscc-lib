## 2026-11-22 — `version:check` checks every version occurrence and fails on a vanished target

**Done:** Every getter in `scripts/version_sync.py` now returns a list holding each version its sync
function rewrites (`[]` = pattern not found). `check_mode` fails on a missing file, an unmatched
pattern or any stale occurrence. `sync_mode` rewrites a file when any occurrence differs, reports a
missing or unmatched target as `FAILED:` and returns False. `main()` exits 1 in both modes when that
happens.

**Files changed:**

- `scripts/version_sync.py`: the three snippet regexes capture the version as group 2 (the sync
    backrefs moved from `\2` to `\3`). Added the `_first` helper for count=1 targets. Multi-site
    getters use `findall`/`finditer`: docs (Maven+Gradle+SPM, all matches), mise (both `--version`),
    test_install (fallback + docstring `--version`, `[]` without a fallback), Ruby, Swift, csproj.
    The lock getter returns `[top, root]` (its `MISMATCH(...)` string is gone). The pom getter
    reuses `MAVEN_DEP_RE`. A shared `_inspect` helper feeds both modes. Module docstring states the
    contract
- `tests/test_version_sync.py`: module docstring rewritten. The lock tests assert the list contract.
    13 new tests on synthetic trees (`monkeypatch.setattr` on `vs.ROOT`/`vs.TARGETS`), one bad case
    each: check fails on (a) a current Maven + stale SPM doc, (b) a missing target, (c) an unmatched
    pattern, (d) a stale second mise `--version`. Sync rewrites a stale second snippet (file read
    back equals the all-current text), leaves a current file byte-identical (same mtime), fails on a
    missing or unmatched target, and `main()` exits 1. Getter tests: docs/index.md gives 4 versions,
    the mise getter gives both flags, the test_install getter gives the docstring flag

**Verification:**

- `uv run pytest tests/test_version_sync.py -q`: 19 passed
- `python3 scripts/version_sync.py --check`: exit 0, 22 lines starting with `OK:` (`grep -c` = 22)
- `uv run ruff check scripts/version_sync.py tests/test_version_sync.py`: All checks passed;
    `uv run ty check scripts/version_sync.py`: All checks passed
- `mise run check`: exit 0
- Mutation probe in a `/tmp` copy, three mutants. (1) Check only the first occurrence: the stale-SPM
    and stale-mise tests go red. (2) Treat a missing file as found: the missing-target tests go red.
    (3) Make the mise getter return only the first flag: the mise getter and mise check tests go
    red. The real-tree tests also fail in that copy, but only because it has no repo files

**Next:** The three remaining `normal` issues all need a recorded rule first: `alg_minhash_256([])`,
the WASM `finalize()` consumption rule, and Go `EncodeComponent` on a short digest. The `low` issue
about partial file lists in `docs/development.md` / release SKILL.md is now easier to fix: point
both at `TARGETS`.

**Notes:**

- Missing-file output is `MISMATCH: <path> missing`. Unmatched-pattern output is
    `MISMATCH: <path> pattern not found` (sync prints `FAILED:` instead). The OK line keeps its
    exact prefix and prints the workspace version
- The package-lock getter returns `[]` when either root field is missing. `_sync_package_lock` would
    raise a KeyError on such a lock, so sync must not call it
- I did not add a post-sync re-check (re-running the getter on the rewritten text). Sync and getter
    share the same regexes, so it would add nothing
- `learnings.md` line "`version_sync.py` covers only the Maven/Gradle/SPM doc snippets" is still
    true (no snippet kinds were added)
- No Rust touched: no CRAP or iai impact
