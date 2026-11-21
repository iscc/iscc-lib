## 2026-11-21 — Review of: Pin the .NET CI legs to the runtimes they name (CI red on the .NET 10 leg)

**Verdict:** PASS_WITH_NOTES

**Summary:** The `dotnet-test` job now installs into a private `DOTNET_INSTALL_DIR` and sets
`fail-fast: false`. CI proves the fix: on the pushed advance commit `6dd5518` all three .NET jobs
are green, and the 10.0 leg runs on .NET 10 again. One claim from next.md does not hold: the action
also installs the latest LTS runtime into the private directory, so the 10.0 leg will fail again
when .NET 12 ships. Filed as a `normal` issue.

**Verification:**

- [x] `yaml.safe_load` script: `fail-fast is False`, and the one `actions/setup-dotnet@` step has
    `env['DOTNET_INSTALL_DIR'] == '${{ runner.temp }}/dotnet'`
- [x] Matrix still has exactly `runtime` `'8.0'` / `'10.0'` with `major` 8 / 10 (roll-forward
    `Minor` / `LatestMajor` unchanged)
- [x] `git show --name-only --format= HEAD` lists only `.github/workflows/ci.yml` and two files
    under `.claude/`
- [x] `mise run check` passes: exit 0, every hook passed (check yaml and YAML formatting included)
- [x] After the push: run 44904543921 on `6dd5518` — `C# / .NET 8.0`, `C# / .NET 10.0` and
    `C# / .NET` conclude `success`, 154 tests passed on each leg. Both setup-dotnet logs show
    `DOTNET_INSTALL_DIR: /home/runner/work/_temp/dotnet` and real downloads, not "already installed"

Beyond next.md: read `src/installer.ts`, `src/setup-dotnet.ts`, `action.yml` and the README of
`actions/setup-dotnet` at tag `v6`, and the setup-dotnet step logs of both legs.

**Issues found:**

- `normal`, filed: "The .NET 10 CI leg depends on which .NET is the latest LTS". setup-dotnet runs
    `install-dotnet.sh --runtime dotnet --channel LTS` before each requested version, also into the
    private directory. Both legs extracted runtime 10.0.15, so the 8.0 leg holds runtimes 8 and 10.
    Green today; the 10.0 leg (`LatestMajor`) turns red when .NET 12 is released, with no change in
    the repository. The matrix comment ("a leg sees only the runtimes it lists") is wrong for the
    same reason. next.md's note "stays correct when .NET 12 ships" was the source
- next.md's note "the net8.0 targeting pack comes from the 8.0 SDK install" was not checked; the
    10.0 leg keeps both SDKs, as scoped

**Codex review:** No actionable findings. It confirmed that setup-dotnet honours the private
directory and passes it on to later steps. It did not see the LTS pre-pass.

**Next:** Two candidates, both small:

1. The Ruby floor 3.1 → 3.3 (`normal` [human], decided): the larger release blocker.
2. The new .NET issue: a `ci.yml`-only step. The action's input `skip-lts-prepass: true` stops the
    pre-pass. Decide the SDK order on the 10.0 leg in the same step: with the pre-pass off, the
    `dotnet` host comes from the first SDK installed (8.0 today), because later installs run with
    `--skip-non-versioned-files`. Listing `10.0` before `8.0` is the likely answer. This is
    unverified, and again only a CI run proves it: the setup-dotnet log must show no
    `--channel LTS` line, and the guard must pass on both legs.

**Notes:**

- **For Titusz:** .NET 8 reached end of support on 2026-11-10, and the package still targets
    `net8.0` as its floor. Per the triage decision of 2026-11-21 only you raise a floor, so nothing
    was filed or changed.
- Process for a fix that only CI can prove: review pushed the define-next and advance commits first
    (pre-push gates green), read the run, and then wrote this handoff. The push of the review commit
    starts one more CI run on the final head.
- The whole CI run 44904543921 on `6dd5518` concluded `success` (every job): `develop` is green
    again.
- `release.yml`'s three `setup-dotnet` sites are untouched (out of scope: they do not roll forward).
- Three stale `git worktree` records under `/tmp/iscc-review-*` (directories already gone) were
    pruned; `git worktree list` shows only the checkout.
