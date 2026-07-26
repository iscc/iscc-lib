// Unicode 16.0.0 boundary conformance tests for the .NET binding.
//
// Runs the canonical boundary fixture (crates/iscc-lib/tests/unicode_boundary.json,
// linked into the test output directory by the csproj) against TextClean and
// TextCollapse, gating the declared-Unicode-version freeze rule: code points
// unassigned in Unicode 16.0.0 are mapped to the noncharacter sentinel U+FFFF
// before normalization and removed by the unchanged category-C filter, while
// assigned code points flow through the regular pipeline. The sequence vectors
// additionally pin the sentinel map against the superseded delete-filter design.

using System.Text.Json;
using Iscc.Lib;
using Xunit;

namespace Iscc.Lib.Tests;

/// <summary>Validates TextClean and TextCollapse against the canonical Unicode boundary vectors.</summary>
public class UnicodeBoundaryTests
{
    /// <summary>Cached parsed unicode_boundary.json for all test methods.</summary>
    private static readonly Lazy<JsonElement> Boundary = new(LoadBoundary);

    /// <summary>Load and parse testdata/unicode_boundary.json from the test output directory.</summary>
    private static JsonElement LoadBoundary()
    {
        string path = Path.Combine(AppContext.BaseDirectory, "testdata", "unicode_boundary.json");
        string json = File.ReadAllText(path);
        return JsonDocument.Parse(json).RootElement;
    }

    /// <summary>Guard against a truncated or renamed fixture silently defining zero vector tests.</summary>
    [Fact]
    public void BoundaryFixtureMetadata()
    {
        JsonElement metadata = Boundary.Value.GetProperty("_metadata");
        Assert.Equal("16.0.0", metadata.GetProperty("unicode_data_version").GetString());
        Assert.Equal(7, Boundary.Value.GetProperty("text_clean").EnumerateObject().Count());
        Assert.Equal(5, Boundary.Value.GetProperty("text_collapse").EnumerateObject().Count());
    }

    // ── text_clean ───────────────────────────────────────────────────────────

    /// <summary>Yield boundary cases for text_clean (7 vectors).</summary>
    public static IEnumerable<object[]> TextCleanVectors()
    {
        foreach (JsonProperty kv in Boundary.Value.GetProperty("text_clean").EnumerateObject())
            yield return [kv.Name, kv.Value];
    }

    [Theory]
    [MemberData(nameof(TextCleanVectors))]
    public void TextCleanBoundary(string _, JsonElement tc)
    {
        string input = tc.GetProperty("inputs")[0].GetString()!;
        string expected = tc.GetProperty("outputs").GetProperty("result").GetString()!;
        Assert.Equal(expected, IsccLib.TextClean(input));
    }

    // ── text_collapse ────────────────────────────────────────────────────────

    /// <summary>Yield boundary cases for text_collapse (5 vectors).</summary>
    public static IEnumerable<object[]> TextCollapseVectors()
    {
        foreach (JsonProperty kv in Boundary.Value.GetProperty("text_collapse").EnumerateObject())
            yield return [kv.Name, kv.Value];
    }

    [Theory]
    [MemberData(nameof(TextCollapseVectors))]
    public void TextCollapseBoundary(string _, JsonElement tc)
    {
        string input = tc.GetProperty("inputs")[0].GetString()!;
        string expected = tc.GetProperty("outputs").GetProperty("result").GetString()!;
        Assert.Equal(expected, IsccLib.TextCollapse(input));
    }
}
