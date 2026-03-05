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
    public void GenImageCodeV0_ReturnsIsccString()
    {
        byte[] pixels = new byte[1024];
        string result = IsccLib.GenImageCodeV0(pixels);
        Assert.StartsWith("ISCC:", result);
    }

    [Fact]
    public void GenAudioCodeV0_ReturnsIsccString()
    {
        int[] cv = new int[64];
        for (int i = 0; i < cv.Length; i++)
            cv[i] = i * 1000;
        string result = IsccLib.GenAudioCodeV0(cv);
        Assert.StartsWith("ISCC:", result);
    }

    [Fact]
    public void GenVideoCodeV0_ReturnsIsccString()
    {
        int[] frame1 = new int[380];
        int[] frame2 = new int[380];
        for (int i = 0; i < 380; i++)
        {
            frame1[i] = i;
            frame2[i] = 380 - i;
        }
        int[][] frameSigs = [frame1, frame2];
        string result = IsccLib.GenVideoCodeV0(frameSigs);
        Assert.StartsWith("ISCC:", result);
    }

    [Fact]
    public void GenMixedCodeV0_ReturnsIsccString()
    {
        string textCode = IsccLib.GenTextCodeV0(
            "This is a reasonably long text that should be enough for generating a text code.");
        byte[] pixels = new byte[1024];
        string imageCode = IsccLib.GenImageCodeV0(pixels);
        string[] codes = [textCode, imageCode];
        string result = IsccLib.GenMixedCodeV0(codes);
        Assert.StartsWith("ISCC:", result);
    }

    [Fact]
    public void GenIsccCodeV0_ReturnsIsccString()
    {
        byte[] data = "Hello World"u8.ToArray();
        string dataCode = IsccLib.GenDataCodeV0(data);
        string instanceCode = IsccLib.GenInstanceCodeV0(data);
        string[] codes = [dataCode, instanceCode];
        string result = IsccLib.GenIsccCodeV0(codes);
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

    [Fact]
    public void GenSumCodeV0_ReturnsPopulatedResult()
    {
        string tempFile = Path.GetTempFileName();
        try
        {
            File.WriteAllText(tempFile, "Hello World for ISCC sum code test");
            SumCodeResult result = IsccLib.GenSumCodeV0(tempFile);
            Assert.StartsWith("ISCC:", result.Iscc);
            Assert.NotEmpty(result.Datahash);
            Assert.True(result.Filesize > 0);
            Assert.Null(result.Units);
        }
        finally
        {
            File.Delete(tempFile);
        }
    }

    [Fact]
    public void GenSumCodeV0_WithUnits_ReturnsUnitStrings()
    {
        string tempFile = Path.GetTempFileName();
        try
        {
            File.WriteAllText(tempFile, "Hello World for ISCC sum code test with units");
            SumCodeResult result = IsccLib.GenSumCodeV0(tempFile, addUnits: true);
            Assert.StartsWith("ISCC:", result.Iscc);
            Assert.NotNull(result.Units);
            Assert.True(result.Units.Length > 0);
            foreach (string unit in result.Units)
                Assert.StartsWith("ISCC:", unit);
        }
        finally
        {
            File.Delete(tempFile);
        }
    }

    // ── Encoding Utilities ──────────────────────────────────────────────────

    [Fact]
    public void EncodeBase64_ReturnsExpectedString()
    {
        byte[] data = "Hello"u8.ToArray();
        string result = IsccLib.EncodeBase64(data);
        Assert.Equal("SGVsbG8", result);
    }

    [Fact]
    public void JsonToDataUrl_ReturnsDataUrl()
    {
        string result = IsccLib.JsonToDataUrl("{\"hello\":\"world\"}");
        Assert.StartsWith("data:", result);
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
