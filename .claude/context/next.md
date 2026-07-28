# Next Work Package

## Step: Pin the .NET test dependencies and commit a NuGet lock file

## Goal

`packages/dotnet` is the only ecosystem in the repo without a committed lockfile, and its three test
package references float on `18.*` / `3.*`, so any upstream release silently changes what CI runs.
Pin them to what restore resolves today, commit `packages.lock.json`, and make CI restore in
`--locked-mode`. The Ruby half of the doc-drift issue rides along as a one-line doc fix.

## Alternatives Considered

- **Chosen:** the .NET reproducibility gap — every other ecosystem (Cargo.lock, two `uv.lock`s,
    Gemfile.lock, go.sum) commits its resolution; this one re-resolves on every restore, and the
    xunit v3 major it floats on landed only three iterations ago.
- **Rejected:** the `release.yml` action-freshness pass recommended by the handoff — probed live
    2026-07-28 and it is already a no-op. All 24 distinct `uses:` refs across the three workflow
    files sit on their publisher's latest major, and `git/matching-refs/tags/v<N+1>` is empty for
    every major in use. Nothing to bump; advance records that evidence in the handoff instead.

## Scope

- **Create**: `packages/dotnet/Iscc.Lib.Tests/packages.lock.json` (restore-generated, tracked)
- **Modify**: `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj`, `.github/workflows/ci.yml`
    (the `dotnet` job, lines 140-158), `packages/dotnet/CLAUDE.md` (doc, line 34),
    `crates/iscc-rb/CLAUDE.md` (doc, line 108)
- **Reference**: `.claude/context/decisions.md` → 2026-07-27 xunit v3 entry,
    `crates/iscc-rb/src/lib.rs:269` (`ruby.str_from_slice`)

## Not In Scope

- Crossing a major on any .NET package (`xunit.v3` stays 3.x, `Microsoft.NET.Test.Sdk` stays 18.x)
    or switching to the Microsoft.Testing.Platform runner — settled by decision 2026-07-27.
- The `dotnet pack` / publish / smoke jobs in `release.yml`, and every other workflow edit.
- `.claude/context/specs/java-bindings.md` (the human-gated half of the doc-drift issue) and
    `issues.md` (review owns issue resolution — put progress in the handoff).
- Lockfiles for other ecosystems, a new CI job, or any test-source edit.

## Implementation Notes

- Derive the pins, do not retype them: add
    `<RestorePackagesWithLockFile>true</RestorePackagesWithLockFile>` to the test csproj, run
    `dotnet restore packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj --force-evaluate`, then
    copy each direct ref's resolved version from the generated lock file into its `Version=`
    attribute. Plain `Version="3.2.2"` form is enough (NuGet resolves a direct ref to the lowest
    applicable version). Sanity check — resolution here today is `xunit.v3` 3.2.2,
    `xunit.runner.visualstudio` 3.1.5, `Microsoft.NET.Test.Sdk` 18.8.1.
- CI `dotnet` job: add a restore step with `--locked-mode` before the build, and `--no-restore` to
    the existing build and test steps so the locked restore is the only one. Keep `dotnet test`'s
    `-e LD_LIBRARY_PATH=…` form — `-e` is a VSTest feature the P/Invoke path depends on.
- If `--locked-mode` rejects the referenced `Iscc.Lib` project (it declares zero
    `PackageReference`s), give it the same property and commit its lock file too; that is the only
    extra file allowed.
- Prove the gate is not false-green: in a throwaway tree (`git archive HEAD | tar -x -C /tmp/x`),
    bump one csproj version and confirm `dotnet restore --locked-mode` exits non-zero. Report it in
    the handoff.
- Ruby doc fix: `crates/iscc-rb/CLAUDE.md:108` must teach `ruby.str_from_slice(&bytes)`; while
    there, check the rest of the "Magnus Patterns" list against `crates/iscc-rb/src/lib.rs`.
- Handoff note (never `issues.md`): re-run the action-freshness loop over the distinct refs in
    `.github/workflows/*.yml` (`gh api repos/<o>/<r>/releases/latest` plus
    `gh api repos/<o>/<r>/git/matching-refs/tags/v<N+1>`) and record the result, so review can
    retire that bullet of the dependency issue.

## Verification

- `grep -E 'Version="[^"]*\*"' packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` exits 1 — no
    floating version range survives in the working tree
- `git ls-files --error-unmatch packages/dotnet/Iscc.Lib.Tests/packages.lock.json` exits 0, and the
    lock file's `resolved` version for each of the three direct refs equals its csproj `Version`
- `dotnet restore packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj --locked-mode` exits 0 and
    leaves `git status --porcelain packages/dotnet` empty
- `cargo build -p iscc-ffi`, then
    `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug` reports 0
    failed and 104 total (the count recorded in decisions.md 2026-07-27)
- `grep -n "RString::from_slice" crates/iscc-rb/CLAUDE.md` exits 1 while `str_from_slice` is present
- `mise run check` exits 0 and the tracked tree is unchanged by it

## Done When

All six verification criteria pass: the .NET test project restores from a committed, tracked lock
file in locked mode, its tests still report 104 passing, and the Ruby binding doc teaches the magnus
0.8 API the code actually calls.
