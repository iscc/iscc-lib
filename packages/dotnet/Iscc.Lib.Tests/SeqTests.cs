// Unit tests for ISCC-SEQ (IEP-0020) — EncodeSeq / DecodeSeq error paths and input forms.

using Iscc.Lib;
using Xunit;

namespace Iscc.Lib.Tests;

/// <summary>Validates EncodeSeq / DecodeSeq error surfacing and accepted input forms.</summary>
public class SeqTests
{
    /// <summary>A valid Content-Code ISCC-UNIT used across tests.</summary>
    private const string Unit = "ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI";

    /// <summary>Bytes of the data.json decode_seq vector test_0001_four_units.</summary>
    private static readonly byte[] FourUnitsSeq = Convert.FromHexString(
        "0007578c7c8fbfbdb58f29f63efeb3871553faa75ff97458d49eff5ee4ddf36bb6c92007a88248c53a852c8d7e9c547047c9a36c4052b6acabef1e2a4ce15e586e0cb305300705b41f3472493225d27edcbdb63236846a53ee28deca5e3561acc0c9bbdf81b54007a522af4a2c9cbf6a808924afd2acfa0793347b936678f5982e22fdd3dd4a0e37");

    [Fact]
    public void EncodeSeq_SingleUnit_DecodesBack()
    {
        byte[] seq = IsccLib.EncodeSeq([Unit]);
        Assert.Equal([Unit], IsccLib.DecodeSeq(seq));
    }

    [Fact]
    public void EncodeSeq_LowercaseWithoutPrefix_MatchesPrefixedForm()
    {
        byte[] got = IsccLib.EncodeSeq(["ead2rasiyu5iklenp2ofi4chzgrwyqcsw2wkx3y6fjgocxsynyglgbi"]);
        Assert.Equal(IsccLib.EncodeSeq([Unit]), got);
    }

    [Fact]
    public void EncodeSeq_Empty_ThrowsAtLeastOneUnit()
    {
        var ex = Assert.Throws<IsccException>(() => IsccLib.EncodeSeq([]));
        Assert.Contains("at least one ISCC-UNIT", ex.Message);
    }

    [Fact]
    public void EncodeSeq_InvalidBase32_Throws()
    {
        var ex = Assert.Throws<IsccException>(() => IsccLib.EncodeSeq(["ISCC:NOT-A-VALID-UNIT!"]));
        Assert.Contains("base32 decode error", ex.Message);
    }

    [Fact]
    public void EncodeSeq_IsccCode_ThrowsNotPermitted()
    {
        var ex = Assert.Throws<IsccException>(() => IsccLib.EncodeSeq(
            ["ISCC:KACYPXW445FTYNJ3CYSXHAFJMA2HUWULUNRFE3BLHRSCXYH2M5AEGQY"]));
        Assert.Contains("not permitted in an ISCC-SEQ", ex.Message);
    }

    [Fact]
    public void EncodeSeq_NullArray_ThrowsArgumentNull()
    {
        Assert.Throws<ArgumentNullException>(() => IsccLib.EncodeSeq(null!));
    }

    [Fact]
    public void EncodeSeq_NullElement_ThrowsArgumentNull()
    {
        Assert.Throws<ArgumentNullException>(() => IsccLib.EncodeSeq([Unit, null!]));
    }

    [Fact]
    public void DecodeSeq_Empty_ThrowsEmptySeq()
    {
        var ex = Assert.Throws<IsccException>(() => IsccLib.DecodeSeq([]));
        Assert.Contains("Empty ISCC-SEQ", ex.Message);
    }

    [Fact]
    public void DecodeSeq_Truncated_Throws()
    {
        byte[] truncated = FourUnitsSeq[..^1];
        var ex = Assert.Throws<IsccException>(() => IsccLib.DecodeSeq(truncated));
        Assert.Contains("Truncated", ex.Message);
    }

    [Fact]
    public void DecodeSeq_FourUnitsVector_ReturnsFourUnits()
    {
        Assert.Equal(4, IsccLib.DecodeSeq(FourUnitsSeq).Length);
    }
}
