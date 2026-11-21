## 2026-11-21 — Review of: Pin the development Ruby in `mise.toml` and bundle under it at container setup

**Verdict:** PASS

**Summary:** The root `mise.toml` pins `ruby = "3.4.13"`, both standardrb hooks and the two docs run
`mise exec -- …`, and the committed `postCreateCommand` installs the pinned Ruby and its bundle. I
re-ran every criterion, including the fresh-setup simulation with an empty mise data directory, and
all pass. The diff stays in scope (3 non-test, non-doc files) and weakens no gate.

**Verification:**

- [x] `mise ls --current ruby` from the repo root shows `3.4.13` sourced from `mise.toml` →
    `ruby  3.4.13  /workspace/iscc-lib/mise.toml  3.4.13`
- [x] `grep -nE 'ruby@[0-9]'` on the hook file and the two docs prints nothing (exit 1);
    `git show HEAD:.devcontainer/devcontainer.json` contains `mise exec -- bundle install` and
    parses with `python3 -m json.tool`
- [x] Fresh-setup simulation (`git archive HEAD` → `/tmp/rbfresh`, empty `MISE_DATA_DIR`, the Ruby
    group extracted from HEAD's `postCreateCommand`, run with `sh -c`): Ruby 3.4.13 downloaded in
    12.9 s, setup group exit 0 after 75 s with no "failed" line, `.bundle/config` holds
    `BUNDLE_PATH: "vendor/bundle"`, `mise exec -- bundle exec standardrb` exit 0. Untrusted and
    removed afterwards. The advance handoff recorded the same output
- [x] `LANG=C.UTF-8 mise exec -- bundle exec rake compile test` in `crates/iscc-rb` →
    `168 runs, 481 assertions, 0 failures, 0 errors, 0 skips`
- [x] `uv run pytest tests/test_ruby_gem_docs.py` → 28 passed (pin test and negative case included);
    `mise run check` exit 0. Pre-push gates on the edited test file: `uv run ty check` and
    `ruff --select S,C901` clean
- [x] `git ls-files -v .devcontainer/devcontainer.json` starts with `S`; `git status --porcelain`
    lists only the runner-owned `iterations.jsonl`

Extra probes (3 of 3): the pin-test helpers fed the previous real files report all four old
spellings and the missing pin; the local `post-create.sh` contains the committed command verbatim;
`mise run version:check` with an empty `MISE_DATA_DIR` installs Ruby before the task runs.

**Issues found:**

- Not filed (nits): `docs/development.md` → "Dev Container Setup" lists what `postCreateCommand`
    does and the included tools without the Ruby bundle or Ruby; it reads as an excerpt and was so
    before this step
- Not filed: no test reads the committed `postCreateCommand`. A test on the disk copy cannot work
    here, because the disk copy is the human's local array form (`skip-worktree`)
- The deviation from next.md (regex instead of `tomllib`) is correct: CI's `python-test` matrix runs
    3.10, which has no `tomllib`. Same call as the 2026-07-26 docs-nav decision

**Codex review:** No actionable findings. It ran the 57 targeted Python tests, the 168 Ruby tests,
standardrb and the Python static checks, all green (run with `--base` on the define-next commit,
because the advance work is two commits).

**Resolved:** the `normal` issue "A rebuilt devcontainer cannot run the Ruby gates" is deleted from
issues.md. A real container rebuild cannot run inside the loop; the simulation stands in for it.

**Next:** UniFFI `text_trim` truncates `nbytes` on 32-bit Android and documents a word boundary
(`normal`, audit, 3 non-test files): saturate, add a `u64::MAX` test, fix the doc comment and
regenerate both UniFFI bindings (compare modulo trailing whitespace, per learnings). After that the
decided cleanup "C++ `safe_data` and .NET `PinnedRows` padding".

**Notes:**

- New decision (decisions.md): the development Ruby is the only `[tools]` pin. Accepted cost: every
    `mise run <task>` on a machine without Ruby 3.4.13 installs it first (92 MB, about 13 s on
    x86_64 Linux), and inside the repo the pinned Ruby precedes system Ruby on PATH (`which ruby` is
    the mise shim). Whether mise has a prebuilt Ruby on Windows was not checked; the documented
    development environment is the devcontainer
- decisions.md went over budget with the new entry; the two oldest entries rotated into
    `decisions-archive.md` through `cid.archive_overflow`
- learnings.md → "Ruby floor" now says `mise exec -- …` and names the pin (still 200 lines)
- `git update-index --cacheinfo` clears `skip-worktree`; advance and define-next memory both carry
    the recipe for the next edit of `devcontainer.json`
