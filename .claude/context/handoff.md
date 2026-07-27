# Handoff

## 2026-07-27 — Review of: migrate the .NET test project to xunit v3 and Test.Sdk 18.x

**Verdict:** PASS

**Summary:** A minimal, correct framework major — two package swaps plus
`<OutputType>Exe</OutputType>` in one csproj, and a one-line doc correction. Zero test-source edits
were needed. Both hazards next.md flagged (`JsonElement` inside `[MemberData]`, and `-e` reaching
the out-of-process v3 host) were probed and neither materialized. Scope was exactly one test
manifest plus one doc file.

**Verification:**

- [x] `cargo build -p iscc-ffi` then
    `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug` — exit 0,
    `Failed: 0, Passed: 104` (floor was 85). Also re-ran the exact CI two-step from wiped
    `bin/`/`obj/` (`dotnet build …csproj`, then `dotnet test …/`): `0 Warning(s)`, 104 passed
- [x] **Row-level enumeration survived — proved against the pre-bump tree, not the floor.** Ran the
    suite from `git archive HEAD~1` (xunit 2.\* / Test.Sdk 17.\*): also *exactly* 104. That equality
    is the real evidence; a silent collapse of the 12 boundary rows would still have cleared 85
- [x] csproj greps — `Include="xunit"` count `0`; file matches `Include="xunit.v3"`,
    `Include="Microsoft.NET.Test.Sdk" Version="18` and still `<TargetFramework>net8.0`. Resolved
    versions match the handoff exactly: xunit.v3 3.2.2, runner.visualstudio 3.1.5, Test.Sdk 18.8.1
- [x] `grep -ri 'xunit 2' packages/dotnet/` — no match
- [x] `mise run check` — exit 0, 18 hooks Passed, no file modified
- [x] `check_ci_job_table.py` criterion N/A — `git diff HEAD~1..HEAD -- .github/workflows/` is
    empty, so ci.yml was genuinely untouched; the always-on `CI job table parity` prek hook passed
- [x] Not-In-Scope held: `Iscc.Lib/Iscc.Lib.csproj` untouched (still `net8.0`), no
    `TestingPlatformDotnetTestSupport` / `UseMicrosoftTestingPlatformRunner`, `release.yml` and
    `issues.md` untouched. No gate weakening anywhere in `origin/develop..HEAD`

**Issues found:**

- (none blocking). Two notes, deliberately not filed:
    - The handoff's "12 boundary vectors report as individual results" is accurate but the arithmetic
        is not self-evident: executed rows are 50 conformance + 41 smoke + 13 boundary = 104, where
        boundary 13 = 12 fixture vectors (7 `text_clean` + 5 `text_collapse`) + 1 metadata-guard fact.
        `--list-tests` shows 53 *methods*, not rows.
    - Floating wildcards (`3.*`, `18.*`) plus no .NET lock file mean CI can resolve a newer 3.x/18.x
        than was reviewed. Pre-existing style, explicitly preserved by next.md — see **Next**.
- issues.md: the `xunit` 3.x / Test.Sdk 18.x bullet is removed from "Dependency review and refresh";
    the entry stays for Gradle/JUnit 6, `jni` 0.22, `magnus` 0.8, `release.yml` actions.

**Codex review:** No findings — it independently ran a clean build and the exact CI test command and
confirmed 104 passing. Operational note: Codex executes real build commands in the *same* tree. My
first `dotnet test` raced it and died with vstest `The argument <dll> is invalid`; 5/5 sequential
runs afterwards and the clean-tree CI sequence were all green, so that was environmental.

**Next:** The JVM slice is the next authorized major, but **consider splitting it** — the issue
bundles "Gradle wrapper 8.12.1 and JUnit 6.x" across two build systems (`packages/kotlin` via
Gradle, `crates/iscc-jni/java/pom.xml` via Maven). The wrapper bump and the JUnit 6 migration are
independent and separately verifiable; JUnit 6 additionally renumbers the platform artifacts 1.x→6.x
in both manifests. Sequencing the wrapper first keeps each step to one build system and one failure
mode. The Kotlin consumer floor is already settled at 2.3+ (`decisions.md` 2026-07-25) — do not
re-open it, but do check the wrapper does not move the *supported* floor further.

**Notes:**

- **Gate maintenance candidate for define-next to weigh:** `packages/dotnet` is the only ecosystem
    in the repo with no lockfile — Cargo, uv, Gemfile and Gradle all pin.
    `RestorePackagesWithLockFile` would make the .NET restore reproducible and turn a surprise
    3.x/18.x into a reviewable diff. Not filed as an issue because the wildcard style is
    long-standing and next.md preserved it deliberately; worth one small step if a floated package
    ever reds CI.
- `dotnet test -e` is a VSTest feature and is now load-bearing for the P/Invoke library path — see
    the new `decisions.md` entry before anyone proposes Microsoft.Testing.Platform runner mode.
- No Rust source touched, so the CRAP baseline and `.iai-baseline.json` correctly did not move, and
    no benchmarked path or public API was affected.
