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
        var result = IsccLib.GenMetaCodeV0("Test Title");
        Assert.StartsWith("ISCC:", result.Iscc);
    }

    [Fact]
    public void GenMetaCodeV0_WithDescription()
    {
        var result = IsccLib.GenMetaCodeV0("Test Title", description: "A test description");
        Assert.StartsWith("ISCC:", result.Iscc);
    }

    [Fact]
    public void GenTextCodeV0_ReturnsIsccString()
    {
        var result = IsccLib.GenTextCodeV0(
            "This is a reasonably long text that should be enough for generating a text code.");
        Assert.StartsWith("ISCC:", result.Iscc);
    }

    [Fact]
    public void GenImageCodeV0_ReturnsIsccString()
    {
        byte[] pixels = new byte[1024];
        var result = IsccLib.GenImageCodeV0(pixels);
        Assert.StartsWith("ISCC:", result.Iscc);
    }

    [Fact]
    public void GenAudioCodeV0_ReturnsIsccString()
    {
        int[] cv = new int[64];
        for (int i = 0; i < cv.Length; i++)
            cv[i] = i * 1000;
        var result = IsccLib.GenAudioCodeV0(cv);
        Assert.StartsWith("ISCC:", result.Iscc);
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
        var result = IsccLib.GenVideoCodeV0(frameSigs);
        Assert.StartsWith("ISCC:", result.Iscc);
    }

    [Fact]
    public void GenMixedCodeV0_ReturnsIsccString()
    {
        var textCode = IsccLib.GenTextCodeV0(
            "This is a reasonably long text that should be enough for generating a text code.");
        byte[] pixels = new byte[1024];
        var imageCode = IsccLib.GenImageCodeV0(pixels);
        string[] codes = [textCode.Iscc, imageCode.Iscc];
        var result = IsccLib.GenMixedCodeV0(codes);
        Assert.StartsWith("ISCC:", result.Iscc);
    }

    [Fact]
    public void GenIsccCodeV0_ReturnsIsccString()
    {
        byte[] data = "Hello World"u8.ToArray();
        var dataCode = IsccLib.GenDataCodeV0(data);
        var instanceCode = IsccLib.GenInstanceCodeV0(data);
        string[] codes = [dataCode.Iscc, instanceCode.Iscc];
        var result = IsccLib.GenIsccCodeV0(codes);
        Assert.StartsWith("ISCC:", result.Iscc);
    }

    [Fact]
    public void GenDataCodeV0_ReturnsIsccString()
    {
        byte[] data = "Hello World"u8.ToArray();
        var result = IsccLib.GenDataCodeV0(data);
        Assert.StartsWith("ISCC:", result.Iscc);
    }

    [Fact]
    public void GenInstanceCodeV0_ReturnsIsccString()
    {
        byte[] data = "Hello World"u8.ToArray();
        var result = IsccLib.GenInstanceCodeV0(data);
        Assert.StartsWith("ISCC:", result.Iscc);
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

    // ── Codec ────────────────────────────────────────────────────────────

    [Fact]
    public void IsccDecode_ReturnsDecodedComponents()
    {
        var metaCode = IsccLib.GenMetaCodeV0("Test Title");
        DecodeResult decoded = IsccLib.IsccDecode(metaCode.Iscc);
        Assert.Equal(0, decoded.Maintype); // META = 0
        Assert.NotEmpty(decoded.Digest);
        Assert.True(decoded.Digest.Length > 0);
    }

    [Fact]
    public void IsccDecompose_ReturnsUnitArray()
    {
        byte[] data = "Hello World"u8.ToArray();
        var dataCode = IsccLib.GenDataCodeV0(data);
        var instanceCode = IsccLib.GenInstanceCodeV0(data);
        string[] codes = [dataCode.Iscc, instanceCode.Iscc];
        var isccCode = IsccLib.GenIsccCodeV0(codes);
        string[] units = IsccLib.IsccDecompose(isccCode.Iscc);
        Assert.True(units.Length >= 2);
        foreach (string unit in units)
            Assert.NotEmpty(unit);
    }

    [Fact]
    public void EncodeComponent_ReturnsIsccString()
    {
        byte[] digest = new byte[8];
        string result = IsccLib.EncodeComponent(0, 0, 0, 64, digest);
        Assert.NotEmpty(result);
    }

    // ── Utilities ──────────────────────────────────────────────────────────

    [Fact]
    public void SlidingWindow_ReturnsNgrams()
    {
        string[] ngrams = IsccLib.SlidingWindow("Hello World", 4);
        Assert.Equal(8, ngrams.Length);
        Assert.Equal("Hell", ngrams[0]);
        Assert.Equal("ello", ngrams[1]);
        Assert.Equal("llo ", ngrams[2]);
        Assert.Equal("lo W", ngrams[3]);
        Assert.Equal("o Wo", ngrams[4]);
        Assert.Equal(" Wor", ngrams[5]);
        Assert.Equal("Worl", ngrams[6]);
        Assert.Equal("orld", ngrams[7]);
    }

    // ── Algorithm Primitives ──────────────────────────────────────────────

    [Fact]
    public void AlgSimhash_ReturnsByteArray()
    {
        byte[] d1 = [0xFF, 0x00, 0xAA, 0x55];
        byte[] d2 = [0x00, 0xFF, 0x55, 0xAA];
        byte[][] digests = [d1, d2];
        byte[] result = IsccLib.AlgSimhash(digests);
        Assert.Equal(4, result.Length);
    }

    [Fact]
    public void AlgSimhash_EmptyInput_Returns32Bytes()
    {
        byte[][] digests = [];
        byte[] result = IsccLib.AlgSimhash(digests);
        Assert.Equal(32, result.Length);
        Assert.All(result, b => Assert.Equal(0, b));
    }

    [Fact]
    public void AlgMinhash256_ReturnsByteArray()
    {
        uint[] features = [1u, 2u, 3u, 4u, 5u];
        byte[] result = IsccLib.AlgMinhash256(features);
        Assert.Equal(32, result.Length);
    }

    [Fact]
    public void AlgCdcChunks_SplitsData()
    {
        byte[] data = "Hello World"u8.ToArray();
        byte[][] chunks = IsccLib.AlgCdcChunks(data);
        Assert.True(chunks.Length >= 1);
        // Concatenated chunks must equal original data
        byte[] reassembled = chunks.SelectMany(c => c).ToArray();
        Assert.Equal(data, reassembled);
    }

    [Fact]
    public void AlgCdcChunks_EmptyData_ReturnsOneChunk()
    {
        byte[] data = [];
        byte[][] chunks = IsccLib.AlgCdcChunks(data);
        Assert.Single(chunks);
        Assert.Empty(chunks[0]);
    }

    [Fact]
    public void SoftHashVideoV0_ReturnsByteArray()
    {
        int[] frame1 = new int[380];
        int[] frame2 = new int[380];
        for (int i = 0; i < 380; i++)
        {
            frame1[i] = i;
            frame2[i] = 380 - i;
        }
        int[][] frameSigs = [frame1, frame2];
        byte[] result = IsccLib.SoftHashVideoV0(frameSigs, bits: 64);
        Assert.Equal(8, result.Length);
    }

    // ── Streaming Hashers ──────────────────────────────────────────────────

    [Fact]
    public void DataHasher_MatchesGenDataCodeV0()
    {
        byte[] data = "Hello World"u8.ToArray();
        var expected = IsccLib.GenDataCodeV0(data);
        using var hasher = new IsccDataHasher();
        hasher.Update(data);
        string result = hasher.Finalize();
        Assert.Equal(expected.Iscc, result);
    }

    [Fact]
    public void DataHasher_ChunkedUpdate_MatchesSingleUpdate()
    {
        byte[] data = "Hello World"u8.ToArray();
        using var single = new IsccDataHasher();
        single.Update(data);
        string expected = single.Finalize();

        using var chunked = new IsccDataHasher();
        chunked.Update(data.AsSpan(0, 5));
        chunked.Update(data.AsSpan(5));
        string result = chunked.Finalize();
        Assert.Equal(expected, result);
    }

    [Fact]
    public void InstanceHasher_MatchesGenInstanceCodeV0()
    {
        byte[] data = "Hello World"u8.ToArray();
        var expected = IsccLib.GenInstanceCodeV0(data);
        using var hasher = new IsccInstanceHasher();
        hasher.Update(data);
        string result = hasher.Finalize();
        Assert.Equal(expected.Iscc, result);
    }

    [Fact]
    public void DataHasher_DisposeIsIdempotent()
    {
        var hasher = new IsccDataHasher();
        hasher.Dispose();
        hasher.Dispose(); // Should not throw
    }

    [Fact]
    public void DataHasher_UpdateAfterFinalize_Throws()
    {
        using var hasher = new IsccDataHasher();
        hasher.Update("data"u8);
        hasher.Finalize();
        Assert.Throws<InvalidOperationException>(() => hasher.Update("more"u8));
    }

    [Fact]
    public void DataHasher_FinalizeAfterFinalize_Throws()
    {
        using var hasher = new IsccDataHasher();
        hasher.Update("data"u8);
        hasher.Finalize();
        Assert.Throws<InvalidOperationException>(() => hasher.Finalize());
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
