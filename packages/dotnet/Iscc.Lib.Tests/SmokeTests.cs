// Smoke tests for ISCC .NET bindings — validates P/Invoke into iscc-ffi.

using Iscc.Lib;
using Xunit;

namespace Iscc.Lib.Tests;

/// <summary>Validates that the P/Invoke bridge to the Rust FFI library works end-to-end.</summary>
public class SmokeTests
{
    // ── Conformance ─────────────────────────────────────────────────────────

    [Fact]
    public void ConformanceSelftest_ReturnsTrue()
    {
        bool result = IsccLib.ConformanceSelftest();
        Assert.True(result, "Conformance selftest should pass against vendored test vectors");
    }

    // ── Constants ───────────────────────────────────────────────────────────

    [Fact]
    public void MetaTrimName_Returns128()
    {
        Assert.Equal(128u, IsccLib.MetaTrimName);
    }

    [Fact]
    public void MetaTrimDescription_Returns4096()
    {
        Assert.Equal(4096u, IsccLib.MetaTrimDescription);
    }

    [Fact]
    public void MetaTrimMeta_Returns128000()
    {
        Assert.Equal(128000u, IsccLib.MetaTrimMeta);
    }

    [Fact]
    public void IoReadSize_Returns4194304()
    {
        Assert.Equal(4194304u, IsccLib.IoReadSize);
    }

    [Fact]
    public void TextNgramSize_Returns13()
    {
        Assert.Equal(13u, IsccLib.TextNgramSize);
    }

    // ── Text Utilities ──────────────────────────────────────────────────────

    [Fact]
    public void TextClean_NormalizesWhitespace()
    {
        string result = IsccLib.TextClean("  Hello   World  ");
        Assert.Equal("Hello   World", result);
    }

    [Fact]
    public void TextRemoveNewlines_CollapsesToSingleLine()
    {
        string result = IsccLib.TextRemoveNewlines("Hello\nWorld");
        Assert.Equal("Hello World", result);
    }

    [Fact]
    public void TextTrim_TruncatesToByteLimit()
    {
        string result = IsccLib.TextTrim("Hello World", 5);
        Assert.Equal("Hello", result);
    }

    [Fact]
    public void TextCollapse_SimplifiesText()
    {
        string result = IsccLib.TextCollapse("Hello, World!");
        Assert.Equal("helloworld", result);
    }

    // ── Gen Functions ───────────────────────────────────────────────────────

    [Fact]
    public void GenMetaCodeV0_ReturnsIsccString()
    {
        string result = IsccLib.GenMetaCodeV0("Test Title");
        Assert.StartsWith("ISCC:", result);
    }

    [Fact]
    public void GenMetaCodeV0_WithDescription()
    {
        string result = IsccLib.GenMetaCodeV0("Test Title", description: "A test description");
        Assert.StartsWith("ISCC:", result);
    }

    [Fact]
    public void GenTextCodeV0_ReturnsIsccString()
    {
        string result = IsccLib.GenTextCodeV0(
            "This is a reasonably long text that should be enough for generating a text code.");
        Assert.StartsWith("ISCC:", result);
    }

    [Fact]
    public void GenDataCodeV0_ReturnsIsccString()
    {
        byte[] data = "Hello World"u8.ToArray();
        string result = IsccLib.GenDataCodeV0(data);
        Assert.StartsWith("ISCC:", result);
    }

    [Fact]
    public void GenInstanceCodeV0_ReturnsIsccString()
    {
        byte[] data = "Hello World"u8.ToArray();
        string result = IsccLib.GenInstanceCodeV0(data);
        Assert.StartsWith("ISCC:", result);
    }

    // ── Error Handling ──────────────────────────────────────────────────────

    [Fact]
    public void IsccException_CanBeThrown()
    {
        var ex = new IsccException("test error");
        Assert.Equal("test error", ex.Message);
        Assert.IsType<IsccException>(ex);
    }
}
