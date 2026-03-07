## 2026-03-07 — C# / .NET documentation (howto guide, package README, root README section)

**Done:** Created complete C# / .NET documentation: `docs/howto/dotnet.md` howto guide (~370 lines
covering all 10 gen functions, streaming, codec, text utils, encoding utils, algorithm primitives,
constants, conformance, and error handling), `packages/dotnet/README.md` for NuGet (~80 lines), C#
sections in root `README.md` (install + quickstart + NuGet badge + polyglot bullet), and nav entry
in `zensical.toml`.

**Files changed:**

- `docs/howto/dotnet.md`: New howto guide following ruby.md/java.md pattern, with PascalCase C#
    examples matching actual IsccLib API surface
- `packages/dotnet/README.md`: New package README following iscc-rb README pattern
- `README.md`: Added NuGet badge, C# to polyglot bullet and "What is iscc-lib" paragraph, C# / .NET
    install subsection, C# / .NET quickstart subsection
- `zensical.toml`: Added `{ "C# / .NET" = "howto/dotnet.md" }` nav entry after Java

**Verification:**

- `uv run zensical build` exits 0 — docs site builds including `/howto/dotnet/` page
- `test -f docs/howto/dotnet.md` — OK
- `test -f packages/dotnet/README.md` — OK
- `grep -c 'C# / .NET' README.md` → 2 (install + quickstart sections)
- `grep -c 'NuGet' README.md` → 1 (badge)
- `grep -c 'dotnet.md' zensical.toml` → 1 (nav entry)
- `grep -c 'GenMetaCodeV0' docs/howto/dotnet.md` → 4
- `grep -c 'IsccDataHasher' docs/howto/dotnet.md` → 3
- `grep -c 'GenMetaCodeV0' packages/dotnet/README.md` → 2
- `mise run check` — all 15 pre-commit hooks pass
- `mise run lint` — all checks pass

**Next:** NuGet publish pipeline in `release.yml`, or version sync integration for `.csproj`, or C#
API reference page (`docs/dotnet-api.md`).

**Notes:** All code examples use the actual API signatures from `IsccLib.cs`, `Results.cs`,
`IsccDataHasher.cs`, and `IsccInstanceHasher.cs`. Verified against `SmokeTests.cs` usage patterns.
The howto uses `using Iscc.Lib;` namespace consistently and PascalCase method names matching the
actual public API. No source code was modified — documentation only.
