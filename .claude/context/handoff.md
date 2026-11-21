## 2026-11-21 — Pin the .NET CI legs to the runtimes they name (CI red on the .NET 10 leg)

**Done:** The `dotnet-test` job's `actions/setup-dotnet@v6` step now sets
`DOTNET_INSTALL_DIR: ${{ runner.temp }}/dotnet`, so each leg's `dotnet` sees only the SDKs and
runtimes it installs (8 on the 8.0 leg; 8 and 10 on the 10.0 leg), not the .NET 11 the runner image
preinstalls in `/usr/share/dotnet`. Added `fail-fast: false` and rewrote the matrix comment. Matrix,
`major`/`roll-forward` values, `SmokeTests.cs` and the `dotnet` aggregator are unchanged.

**Files changed:**

- `.github/workflows/ci.yml`: `dotnet-test` only — `fail-fast: false`, three comment lines, step
    `env:` with `DOTNET_INSTALL_DIR` (+6 lines, no deletions)

**Verification:**

- Checked the action source at tag `v6`
    (`gh api 'repos/actions/setup-dotnet/contents/src/installer.ts?ref=v6'`): line 362 reads
    `process.env['DOTNET_INSTALL_DIR']`, lines 456-457 run `core.addPath(DOTNET_INSTALL_DIR)` and
    `core.exportVariable('DOTNET_ROOT', DOTNET_INSTALL_DIR)`, which matches next.md's claim
- `yaml.safe_load` script: `fail-fast is False`; exactly one `actions/setup-dotnet@` step, with
    `env['DOTNET_INSTALL_DIR'] == '${{ runner.temp }}/dotnet'`; matrix (runtime, major) pairs are
    exactly `('8.0', 8)` and `('10.0', 10)`. Printed
    `OK [('8.0', 8, 'Minor'), ('10.0', 10, 'LatestMajor')]`
- `mise run check`: exit 0, all 20 hooks passed (check yaml, YAML formatting included)
- **Not verified locally (cannot be):** the CI result. Review must push and confirm that
    `C# / .NET 8.0`, `C# / .NET 10.0` and `C# / .NET` conclude `success` on the pushed head
    (`gh run view <id> --json jobs`). I did not run the .NET suite locally; nothing local changed

**Next:** If the .NET jobs go green: the Ruby floor 3.1 → 3.3 (`normal` [human], decided). If the
10.0 leg is still red, read the setup-dotnet log: it should report installing into
`/home/runner/work/_temp/dotnet`, not "already installed".

**Notes:**

- **For Titusz:** .NET 8 reached end of support on 2026-11-10 and the package still targets `net8.0`
    as its floor. Per the triage decision only you raise a floor, so nothing was filed or changed.
- `release.yml`'s three `setup-dotnet` sites are untouched (out of scope: they do not roll forward).
- `${{ runner.temp }}` is in step-level `env:`, where the `runner` context is available.
