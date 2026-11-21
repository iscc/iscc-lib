# Next Work Package

## Step: Pin the .NET CI legs to the runtimes they name (CI red on the .NET 10 leg)

## Goal

CI on `develop` is red: the `C# / .NET 10.0` leg runs on the .NET 11 runtime, and
`SmokeTests.Runtime_MatchesExpectedMajorVersion` catches it (expected 10, got 11). Make each
`dotnet-test` leg see only the runtimes it installs, so the result does not change when the runner
image adds a newer .NET.

## Alternatives Considered

- **Chosen:** install the matrix SDKs into a private directory (`DOTNET_INSTALL_DIR`) and set
    `fail-fast: false`. Changes only `ci.yml`, keeps the decided matrix (net8.0 build on .NET 8 and
    10), and stays correct when .NET 12 ships.
- **Rejected:** multi-target the test project `net8.0;net10.0` and run `--framework net10.0`. It
    changes the csproj and lock file, and needs a .NET 10 SDK for local builds (the devcontainer has
    8.0.423 only).
- **Rejected:** move the leg to .NET 11 or loosen the assert to `>= 10`. The 2026-11-21 triage
    decision names .NET 10, and the assert is what found this.

## Scope

- **Modify**: `.github/workflows/ci.yml` (the `dotnet-test` job only)
- **Reference**: `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` (the guard, unchanged),
    `decisions.md` → "Triage: Python 3.10 and .NET 8 floors stay …", the failing run
    https://github.com/iscc/iscc-lib/actions/runs/44903592630 (job 136844400624)

## Not In Scope

- Changing `SmokeTests.cs`, the matrix runtimes, `major` values or the `dotnet` aggregator job
- The three `setup-dotnet` sites in `release.yml`: they do not roll forward, so a newer preinstalled
    runtime cannot change their result
- Raising the `net8.0` floor. .NET 8 reached end of support on 2026-11-10, but per the triage
    decision only the human raises a floor. Note it in the handoff for Titusz; do not file an issue
- The Ruby 3.3 floor issue and every other `normal` issue (next steps)

## Implementation Notes

- Cause, from the job log: `setup-dotnet` installs into `/usr/share/dotnet` on Ubuntu, where the
    image preinstalls runtimes, so it reports "already installed". Since the runner-images releases
    of 2026-11-11 that directory also holds .NET 11 (GA 2026-11-10, actions/runner-images#15288).
    `DOTNET_ROLL_FORWARD=LatestMajor` picks the newest installed major, which is 11.
- Fix: on the `actions/setup-dotnet@v6` step of `dotnet-test`, set
    `env: DOTNET_INSTALL_DIR: ${{ runner.temp }}/dotnet`. At v6 the action installs there, runs
    `core.addPath(DOTNET_INSTALL_DIR)` and exports `DOTNET_ROOT` to it (`src/installer.ts` around
    lines 456–457 at tag `v6`; README "Environment variables"). Later steps then use that `dotnet`
    with only the requested SDKs and runtimes: 8 on the 8.0 leg, 8 and 10 on the 10.0 leg. Keep both
    SDKs on the 10.0 leg: the net8.0 targeting pack comes from the 8.0 SDK install.
- Add `fail-fast: false` under `strategy:`. The 8.0 leg was cancelled in the failing run, so its
    result is unknown. The aggregator still fails unless every leg succeeded.
- Rewrite the matrix comment so it names the private install directory and why it exists. Keep the
    roll-forward values: `LatestMajor` is now deterministic because the newest major in the
    directory is 10.
- The fix can only be proven in CI: review pushes and reads the run. Nothing local changes, so do
    not run the .NET suite only to compare.

## Verification

- `uv run python -c` loading `.github/workflows/ci.yml` with `yaml.safe_load` asserts:
    `jobs['dotnet-test']['strategy']['fail-fast'] is False`, and the step whose `uses` starts with
    `actions/setup-dotnet@` has `env['DOTNET_INSTALL_DIR'] == '${{ runner.temp }}/dotnet'`
- The same script asserts the matrix still has exactly the `runtime` values `'8.0'` and `'10.0'`,
    with `major` 8 and 10
- `git show --name-only --format= HEAD` (the advance commit) lists only `.github/workflows/ci.yml`
    and files under `.claude/`
- `mise run check` passes (yamlfix, check-yaml)
- After review's push: the `C# / .NET 8.0`, `C# / .NET 10.0` and `C# / .NET` jobs on the pushed head
    conclude `success` (`gh run view <id> --json jobs`)

## Done When

Both `dotnet-test` legs and the `dotnet` aggregator are green on the pushed `develop` head, with the
matrix and the runtime guard unchanged.
