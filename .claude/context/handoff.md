## 2026-03-07 — Review of: C# / .NET documentation (howto guide, package README, root README section)

**Verdict:** PASS

**Summary:** Clean, well-structured documentation-only change. The howto guide (~415 lines), NuGet
package README, root README sections (badge, install, quickstart, polyglot bullet), and
zensical.toml nav entry are all complete and accurate. API signatures verified against actual C#
source files. All verification criteria pass and all quality gates are green.

**Verification:**

- [x] `uv run zensical build` exits 0 — docs site builds with `/howto/dotnet/` page
- [x] `test -f docs/howto/dotnet.md` — exists (415 lines)
- [x] `test -f packages/dotnet/README.md` — exists (82 lines)
- [x] `grep -c 'C# / .NET' README.md` → 2 (install + quickstart sections)
- [x] `grep -c 'NuGet' README.md` → 1 (badge)
- [x] `grep -c 'dotnet.md' zensical.toml` → 1 (nav entry after Java, before C/C++)
- [x] `grep -c 'GenMetaCodeV0' docs/howto/dotnet.md` → 4
- [x] `grep -c 'IsccDataHasher' docs/howto/dotnet.md` → 3
- [x] `grep -c 'GenMetaCodeV0' packages/dotnet/README.md` → 2
- [x] `mise run check` — all 15 pre-commit hooks pass
- [x] `mise run lint` — all checks pass
- [x] `cargo clippy --workspace --all-targets --exclude iscc-rb -- -D warnings` — clean

**Issues found:**

- (none blocking) Minor fix applied: added `using System.Text;` to Data-Code example that uses
    `Encoding.UTF8.GetBytes()` — `System.Text` is not in .NET 8 default implicit usings

**Codex review:** Two advisory findings. (1) P2: README C# install section doesn't note native
library dependency — valid but consistent with other bindings' README sections (Python, Ruby, Go
don't mention native deps either). The howto's "Build from source" admonition covers this. Should be
revisited when NuGet publish pipeline ships. (2) P3: Missing `using System.Text;` for
`Encoding.UTF8.GetBytes()` — valid, fixed in this review commit.

**Next:** The .NET binding is now feature-complete for documentation. Next priority should be the
NuGet publish pipeline (`release.yml` — `nuget` boolean input, build/pack/publish) and version sync
integration for `.csproj`. These are the remaining items in the .NET bindings issue. After that,
only `low`-priority issues remain (C++ wrapper, Swift, Kotlin, language logos).

**Notes:** The `ConformanceSelftest()` description in next.md said "returns count of passed vectors"
but the actual C# API returns `bool` — the advance agent correctly used the actual API signature.
All 10 gen functions are documented with accurate PascalCase method names and typed result records.
The howto structure follows ruby.md/java.md closely with appropriate C#-specific additions (encoding
utils, conformance section separated out).
