# Next Work Package

## Step: Pin the development Ruby in `mise.toml` and bundle under it at container setup

## Goal

Fix the `normal` issue "A rebuilt devcontainer cannot run the Ruby gates": name the development Ruby
once (a `[tools]` pin in `mise.toml`), make the devcontainer setup install the bundle under that
Ruby, and point the standardrb hooks and the two docs at the pin. After a rebuild, `mise run check`
and the Ruby suite must work with no manual step.

## Alternatives Considered

- **Chosen:** devcontainer Ruby pin. It protects the loop's own gate (after a rebuild every commit
    that touches a `.rb` file fails), it is small, and it is the handoff's "Next"
- **Rejected:** UniFFI `text_trim` saturation (`normal`, silent wrong result on 32-bit Android). It
    has real user impact but is not latent-gate breakage, and it is a self-contained next step that
    touches neither file set

## Scope

- **Modify**: `mise.toml` (add a `[tools]` table: `ruby = "3.4.13"`), `.pre-commit-config.yaml`
    (both standardrb hooks), `.devcontainer/devcontainer.json` (the committed `postCreateCommand`,
    see Implementation Notes: the file is `skip-worktree` locally)
- **Modify (docs)**: `crates/iscc-rb/CLAUDE.md` (~line 137-139),
    `.claude/context/specs/ruby-bindings.md` (~line 293-295), `.claude/context/specs/ci-cd.md` →
    "Tool Management" (~line 393 says mise holds only task definitions; add that it pins the
    development Ruby)
- **Modify (local, untracked)**: `.devcontainer/post-create.sh`. It is listed in `.git/info/exclude`
    and its header says its body is identical to the committed `postCreateCommand`. Apply the same
    edit so this container picks it up on rebuild. It is not committed
- **Reference**: `crates/iscc-rb/Gemfile`, `crates/iscc-rb/iscc-lib.gemspec`,
    `tests/test_ruby_gem_docs.py` (where a pin test fits)

## Not In Scope

- The gemspec floor, the CI `ruby` job, `release.yml` cross-gem versions: the floor stays 3.3. CI
    never uses mise, so a `mise.toml` pin does not reach CI
- Installing Ruby in the `Dockerfile` (the issue allows it, but the mise pin is the chosen route)
- Pinning any other tool in `[tools]` (Rust, Python, Node and so on)
- Clearing the human's `skip-worktree` flag on `devcontainer.json` or replacing the local array-form
    `postCreateCommand` on disk (it works around a Zed bug)
- `learnings.md` (it names `ruby@3.4.13`). It belongs to review, so mention it in the handoff
- The Ruby `low` issues (Ruby 4.0 precompiled gem, `test-gem` matrix)

## Implementation Notes

- **The pin:** `[tools]` with `ruby = "3.4.13"` (this exact patch is already installed under
    `~/.local/share/mise/installs/ruby`, so the container downloads nothing new).
    `mise exec -- <cmd>` run from `crates/iscc-rb` finds the root `mise.toml` and uses the pin, and
    installs a missing tool by itself. Mise installs Ruby as a prebuilt binary (learnings: about 30
    s)
- **Hooks:** change `mise exec ruby@3.4 -- bundle exec standardrb[ --fix]` to
    `mise exec -- bundle exec standardrb[ --fix]`, and update the two comments to say the Ruby comes
    from the `mise.toml` pin
- **Setup command:** in the committed `postCreateCommand`, the Ruby group becomes something like
    `{ (cd crates/iscc-rb && mise install ruby && mise exec -- bundle config set --local path vendor/bundle && mise exec -- bundle install --quiet) || echo '…non-fatal…'; }`.
    Keep it non-fatal and after `mise trust`. Update the echo hint so it names `mise exec --`
- **Staging the skip-worktree file:** `git ls-files -v .devcontainer` shows `S` on
    `devcontainer.json`. The on-disk copy is the local array form (`["bash", …post-create.sh]`), and
    HEAD has the string form. Never `git add` the disk copy. Instead:
    `git show HEAD:.devcontainer/devcontainer.json > /tmp/dc.json`, edit `/tmp/dc.json`, then
    `git update-index --cacheinfo 100644,$(git hash-object -w /tmp/dc.json),.devcontainer/devcontainer.json`,
    commit, and afterwards run `git update-index --skip-worktree .devcontainer/devcontainer.json`
    so the flag is back and `git status` stays clean
- **Docs:** replace `mise exec ruby@3.4.13 -- …` with `mise exec -- …` and say the version lives in
    `mise.toml`
- **Pin test** (outside the file budget): one pytest case in `tests/test_ruby_gem_docs.py` that
    reads `mise.toml` with `tomllib`, asserts a `tools.ruby` pin of at least the gemspec floor, and
    asserts that neither `.pre-commit-config.yaml` nor the two docs name a `ruby@<version>`. Add one
    negative case: an injected `ruby@3.4` hook string fails the check
- **Fresh-setup simulation** (the decisive check, because a container rebuild cannot run here):
    `git archive HEAD | tar -x -C /tmp/rbfresh` (after the commit; empty `vendor/bundle`), then
    `mise trust /tmp/rbfresh/mise.toml` and run the committed Ruby setup group there, then
    `mise exec -- bundle exec standardrb` in `/tmp/rbfresh/crates/iscc-rb`. Record the output in the
    handoff and remove `/tmp/rbfresh` (untrust it too)
- `LANG=C.UTF-8` is needed for the Ruby suite (user memory). Never `rake clobber` (learnings)

## Verification

- `mise ls --current ruby` run from the repo root shows `3.4.13` sourced from `mise.toml`
- `grep -nE 'ruby@[0-9]' .pre-commit-config.yaml crates/iscc-rb/CLAUDE.md .claude/context/specs/ruby-bindings.md`
    prints nothing, and `git show HEAD:.devcontainer/devcontainer.json` contains
    `mise exec -- bundle install` and parses with `python3 -m json.tool`
- The fresh-setup simulation in `/tmp/rbfresh` exits 0 for both the setup group and
    `bundle exec standardrb`, and the output is in the handoff
- `cd crates/iscc-rb && LANG=C.UTF-8 mise exec -- bundle exec rake compile test` reports 0 failures
    and 0 errors
- `uv run pytest tests/test_ruby_gem_docs.py` passes with the pin test and its negative case;
    `mise run check` exits 0
- `git ls-files -v .devcontainer/devcontainer.json` starts with `S`, and `git status --porcelain`
    does not list `.devcontainer/`

## Done When

The advance agent is done when all verification criteria pass and the handoff records the
fresh-setup simulation output and the local `post-create.sh` edit.
