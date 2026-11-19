// Unit tests for embedded-NUL rejection: string inputs containing '\0' must throw IsccException
// instead of being truncated at the NUL by the native library.

using Iscc.Lib;
using Xunit;

namespace Iscc.Lib.Tests;

/// <summary>Validates that every string-input path rejects an embedded NUL character.</summary>
public class NulInputTests
{
    /// <summary>Substring of the IsccException message raised for an embedded NUL.</summary>
    private const string Needle = "embedded NUL";

    /// <summary>Text with an embedded NUL whose prefix "Hello" would otherwise be hashed.</summary>
    private const string HelloNulWorld = "Hello\0World";

    /// <summary>Build a valid composite ISCC-CODE (Data-Code + Instance-Code of "Hello World").</summary>
    private static (string Data, string Instance, string Code) HelloWorldCodes()
    {
        byte[] data = "Hello World"u8.ToArray();
        string dataCode = IsccLib.GenDataCodeV0(data).Iscc;
        string instanceCode = IsccLib.GenInstanceCodeV0(data).Iscc;
        string code = IsccLib.GenIsccCodeV0([dataCode, instanceCode]).Iscc;
        return (dataCode, instanceCode, code);
    }

    [Fact]
    public void GenTextCodeV0_NulFree_ReturnsDocumentedCode()
    {
        Assert.Equal("ISCC:EAASKDNZNYGUUF5A", IsccLib.GenTextCodeV0("Hello World").Iscc);
    }

    [Fact]
    public void GenTextCodeV0_EmbeddedNul_Throws()
    {
        var ex = Assert.Throws<IsccException>(() => IsccLib.GenTextCodeV0(HelloNulWorld));
        Assert.Contains(Needle, ex.Message);
    }

    [Fact]
    public void TextClean_EmbeddedNul_Throws()
    {
        var ex = Assert.Throws<IsccException>(() => IsccLib.TextClean(HelloNulWorld));
        Assert.Contains(Needle, ex.Message);
    }

    [Fact]
    public void IsccDecompose_EmbeddedNul_Throws()
    {
        string code = HelloWorldCodes().Code;
        var ex = Assert.Throws<IsccException>(() => IsccLib.IsccDecompose(code + "\0junk"));
        Assert.Contains(Needle, ex.Message);
    }

    [Fact]
    public void IsccDecode_EmbeddedNul_Throws()
    {
        string unit = HelloWorldCodes().Data;
        var ex = Assert.Throws<IsccException>(() => IsccLib.IsccDecode(unit + "\0junk"));
        Assert.Contains(Needle, ex.Message);
    }

    [Fact]
    public void EncodeSeq_EmbeddedNul_Throws()
    {
        string unit = HelloWorldCodes().Data;
        var ex = Assert.Throws<IsccException>(() => IsccLib.EncodeSeq([unit, unit + "\0junk"]));
        Assert.Contains(Needle, ex.Message);
    }

    [Fact]
    public void GenMetaCodeV0_EmbeddedNulInDescription_Throws()
    {
        var ex = Assert.Throws<IsccException>(
            () => IsccLib.GenMetaCodeV0("Title", description: "Desc\0junk"));
        Assert.Contains(Needle, ex.Message);
    }

    [Fact]
    public void GenIsccCodeV0_EmbeddedNulInElement_Throws()
    {
        var (dataCode, instanceCode, _) = HelloWorldCodes();
        var ex = Assert.Throws<IsccException>(
            () => IsccLib.GenIsccCodeV0([dataCode, instanceCode + "\0junk"]));
        Assert.Contains(Needle, ex.Message);
    }

    [Fact]
    public void GenMixedCodeV0_EmbeddedNulInElement_Throws()
    {
        string textCode = IsccLib.GenTextCodeV0("Hello World").Iscc;
        string imageCode = IsccLib.GenImageCodeV0(new byte[1024]).Iscc;
        var ex = Assert.Throws<IsccException>(
            () => IsccLib.GenMixedCodeV0([textCode, imageCode + "\0junk"]));
        Assert.Contains(Needle, ex.Message);
    }
}
