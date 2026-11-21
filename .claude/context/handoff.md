## 2026-11-21 — Pin the development Ruby in `mise.toml` and bundle under it at container setup

**Done:** The root `mise.toml` pins `ruby = "3.4.13"` in a `[tools]` table. Both standardrb hooks
now run `mise exec -- bundle exec standardrb[ --fix]` and the two docs say `mise exec -- …`. The
committed `postCreateCommand` (and the local `post-create.sh`) now installs the pinned Ruby and runs
the bundle under it, still non-fatal. A pin test in `tests/test_ruby_gem_docs.py` checks that the
pin is at or above the gemspec floor and that the hooks and docs name no `ruby@<version>`.

**Files changed:**

- `mise.toml`: `[tools]` table with `ruby = "3.4.13"` and a comment saying CI does not use mise
- `.pre-commit-config.yaml`: both standardrb entries use `mise exec --`, comments name the pin
- `.devcontainer/devcontainer.json` (committed via `update-index --cacheinfo`, not from disk): the
    Ruby group is now
    `{ (cd crates/iscc-rb && mise install ruby && mise exec -- bundle config set --local path vendor/bundle && mise exec -- bundle install --quiet) || echo '… run mise exec -- bundle install in crates/iscc-rb manually'; }`
- `.devcontainer/post-create.sh` (local, untracked, excluded): the same edit, so its body still
    matches the committed `postCreateCommand`. Not committed
- `crates/iscc-rb/CLAUDE.md`, `.claude/context/specs/ruby-bindings.md`: `mise exec -- …`, pin named
    in `mise.toml`; `.claude/context/specs/ci-cd.md` "Tool Management": mise also pins the
    development Ruby
- `tests/test_ruby_gem_docs.py`: `mise_ruby_pin`, `check_dev_ruby_pin`, `inline_ruby_pins` and 8
    tests (2 on the real files, the rest negative or edge cases, including an injected `ruby@3.4`
    hook string)

**Verification:**

- `mise ls --current ruby` → `ruby  3.4.13  /workspace/iscc-lib/mise.toml  3.4.13`
- `grep -nE 'ruby@[0-9]' .pre-commit-config.yaml crates/iscc-rb/CLAUDE.md .claude/context/specs/ruby-bindings.md`
    prints nothing (exit 1). The same regex finds `ruby@3.4` twice in HEAD~1's hook file and
    `ruby@3.4.13` in HEAD~1's CLAUDE.md, so the check catches the old spellings
- `git show HEAD:.devcontainer/devcontainer.json | python3 -m json.tool` → OK; contains
    `mise exec -- bundle install --quiet`
- **Fresh-setup simulation**: `git archive HEAD` → `/tmp/rbfresh` (no `vendor/`), with an EMPTY
    `MISE_DATA_DIR`, so Ruby was really downloaded. The Ruby group was extracted from HEAD's
    `postCreateCommand`:
    ```
    mise ✓ ruby@3.4.13  10.5s  ruby-3.4.13.x86_64_linux.tar.gz
    setup-group exit=0 (66s)      # no "failed" line in the log (grep -c failed → 0)
    ruby 3.4.13 … [x86_64-linux]   /tmp/rbfresh-mise/installs/ruby/3.4.13/bin/ruby
    standardrb exit=0
    ```
    `.bundle/config` held `BUNDLE_PATH: "vendor/bundle"`. Cleanup done: untrusted, then removed
    `/tmp/rbfresh` and `/tmp/rbfresh-mise`
- `LANG=C.UTF-8 mise exec -- bundle exec rake compile test` →
    `168 runs, 481 assertions, 0 failures, 0 errors, 0 skips`
- `uv run pytest tests/test_ruby_gem_docs.py` → 28 passed; also 28 passed under Python 3.10
    (`uv run --no-project --python 3.10 --with pytest --with pyyaml`); `uv run ty check` on the file
    is clean
- `mise run check` → exit 0 (the `Ruby linting (fix)` hook passed under the pin)
- `git ls-files -v .devcontainer/devcontainer.json` → `S …`; `git status --porcelain` lists only
    `.claude/context/iterations.jsonl` (runner-owned)

**Next:** UniFFI `text_trim` saturation (`normal`), then the decided cleanup "C++ `safe_data` and
.NET `PinnedRows` padding".

**Notes:**

- **Deviation from next.md:** the pin test parses `mise.toml` with a regex, not `tomllib`. The first
    version used `tomllib`, and ruff flagged it as third-party under `requires-python >=3.10`.
    `tomllib` does not exist in 3.10, and CI's `python-test` matrix runs 3.10 (as
    `scripts/check_docs_nav.py` and `build_ruby_source_gem.py` note). Two extra tests check that the
    regex reads only the `[tools]` table
- `git update-index --cacheinfo` clears the skip-worktree flag. I set it again before
    `mise run check` and the commit, so prek never saw the local array-form `devcontainer.json`. The
    on-disk file is still the array form
- With the pin, any shell where mise is active puts Ruby 3.4.13 first on PATH inside the repo, ahead
    of system Ruby 3.1. That applies to `tests/test_build_ruby_source_gem.py` too, which needs
    `ruby` on PATH
- For review: `learnings.md` (Tooling → "Ruby floor is 3.3 (270)") still says
    `mise exec ruby@3.4.13 -- …`. It should say `mise exec -- …` (the pin is in `mise.toml`)
- Two commits: the implementation commit, then this handoff and memory. The simulation needed the
    committed tree
