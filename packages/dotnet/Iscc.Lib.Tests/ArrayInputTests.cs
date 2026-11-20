// Unit tests for jagged-array inputs — empty inputs reach the core, null inputs throw ArgumentNullException.

using Iscc.Lib;
using Xunit;

namespace Iscc.Lib.Tests;

/// <summary>Validates empty and null handling across the six jagged-array methods.</summary>
public class ArrayInputTests
{
    /// <summary>A valid Content-Code ISCC-UNIT used as the non-null leading element.</summary>
    private const string Unit = "ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI";

    /// <summary>The FFI's NULL-pointer message, which an empty input must never surface.</summary>
    private const string FfiNullMessage = "must not be NULL";

    /// <summary>A valid 380-element frame signature used as the non-null leading row.</summary>
    private static readonly int[] Frame = new int[380];

    // ── Empty input surfaces the core's error ───────────────────────────────

    [Fact]
    public void GenMixedCodeV0_Empty_ThrowsCoreError()
    {
        var ex = Assert.Throws<IsccException>(() => IsccLib.GenMixedCodeV0([]));
        Assert.Contains("at least 2 Content-Codes required for mixing", ex.Message);
        Assert.DoesNotContain(FfiNullMessage, ex.Message);
    }

    [Fact]
    public void GenIsccCodeV0_Empty_ThrowsCoreError()
    {
        var ex = Assert.Throws<IsccException>(() => IsccLib.GenIsccCodeV0([]));
        Assert.Contains("at least 2 ISCC unit codes required", ex.Message);
        Assert.DoesNotContain(FfiNullMessage, ex.Message);
    }

    [Fact]
    public void GenVideoCodeV0_Empty_ThrowsCoreError()
    {
        var ex = Assert.Throws<IsccException>(() => IsccLib.GenVideoCodeV0([]));
        Assert.Contains("frame_sigs must not be empty", ex.Message);
        Assert.DoesNotContain(FfiNullMessage, ex.Message);
    }

    [Fact]
    public void SoftHashVideoV0_Empty_ThrowsCoreError()
    {
        var ex = Assert.Throws<IsccException>(() => IsccLib.SoftHashVideoV0([]));
        Assert.Contains("frame_sigs must not be empty", ex.Message);
        Assert.DoesNotContain(FfiNullMessage, ex.Message);
    }

    [Fact]
    public void EncodeSeq_Empty_ThrowsCoreError()
    {
        var ex = Assert.Throws<IsccException>(() => IsccLib.EncodeSeq([]));
        Assert.Contains("at least one ISCC-UNIT", ex.Message);
        Assert.DoesNotContain(FfiNullMessage, ex.Message);
    }

    [Fact]
    public void AlgSimhash_Empty_Returns32ZeroBytes()
    {
        Assert.Equal(new byte[32], IsccLib.AlgSimhash([]));
    }

    // ── Null array ──────────────────────────────────────────────────────────

    [Fact]
    public void GenVideoCodeV0_NullArray_ThrowsArgumentNull()
    {
        var ex = Assert.Throws<ArgumentNullException>(() => IsccLib.GenVideoCodeV0(null!));
        Assert.Equal("frameSigs", ex.ParamName);
    }

    [Fact]
    public void SoftHashVideoV0_NullArray_ThrowsArgumentNull()
    {
        var ex = Assert.Throws<ArgumentNullException>(() => IsccLib.SoftHashVideoV0(null!));
        Assert.Equal("frameSigs", ex.ParamName);
    }

    [Fact]
    public void AlgSimhash_NullArray_ThrowsArgumentNull()
    {
        var ex = Assert.Throws<ArgumentNullException>(() => IsccLib.AlgSimhash(null!));
        Assert.Equal("digests", ex.ParamName);
    }

    [Fact]
    public void GenMixedCodeV0_NullArray_ThrowsArgumentNull()
    {
        var ex = Assert.Throws<ArgumentNullException>(() => IsccLib.GenMixedCodeV0(null!));
        Assert.Equal("codes", ex.ParamName);
    }

    [Fact]
    public void GenIsccCodeV0_NullArray_ThrowsArgumentNull()
    {
        var ex = Assert.Throws<ArgumentNullException>(() => IsccLib.GenIsccCodeV0(null!));
        Assert.Equal("codes", ex.ParamName);
    }

    [Fact]
    public void EncodeSeq_NullArray_ThrowsArgumentNullWithParamName()
    {
        var ex = Assert.Throws<ArgumentNullException>(() => IsccLib.EncodeSeq(null!));
        Assert.Equal("units", ex.ParamName);
    }

    // ── Null element after a valid one (pinning stops halfway) ──────────────

    [Fact]
    public void GenVideoCodeV0_NullElement_ThrowsArgumentNull()
    {
        var ex = Assert.Throws<ArgumentNullException>(
            () => IsccLib.GenVideoCodeV0([Frame, Frame, null!]));
        Assert.Equal("frameSigs", ex.ParamName);
        Assert.Contains("frameSigs[2]", ex.Message);
    }

    [Fact]
    public void SoftHashVideoV0_NullElement_ThrowsArgumentNull()
    {
        var ex = Assert.Throws<ArgumentNullException>(
            () => IsccLib.SoftHashVideoV0([Frame, Frame, null!]));
        Assert.Equal("frameSigs", ex.ParamName);
        Assert.Contains("frameSigs[2]", ex.Message);
    }

    [Fact]
    public void AlgSimhash_NullElement_ThrowsArgumentNull()
    {
        var ex = Assert.Throws<ArgumentNullException>(
            () => IsccLib.AlgSimhash([new byte[32], new byte[32], null!]));
        Assert.Equal("digests", ex.ParamName);
        Assert.Contains("digests[2]", ex.Message);
    }

    [Fact]
    public void GenMixedCodeV0_NullElement_ThrowsArgumentNull()
    {
        var ex = Assert.Throws<ArgumentNullException>(
            () => IsccLib.GenMixedCodeV0([Unit, Unit, null!]));
        Assert.Equal("codes", ex.ParamName);
        Assert.Contains("codes[2]", ex.Message);
    }

    [Fact]
    public void GenIsccCodeV0_NullElement_ThrowsArgumentNull()
    {
        var ex = Assert.Throws<ArgumentNullException>(
            () => IsccLib.GenIsccCodeV0([Unit, Unit, null!]));
        Assert.Equal("codes", ex.ParamName);
        Assert.Contains("codes[2]", ex.Message);
    }

    [Fact]
    public void EncodeSeq_NullElement_ThrowsArgumentNullWithIndex()
    {
        var ex = Assert.Throws<ArgumentNullException>(
            () => IsccLib.EncodeSeq([Unit, Unit, null!]));
        Assert.Equal("units", ex.ParamName);
        Assert.Contains("units[2]", ex.Message);
    }

    [Fact]
    public void GenMixedCodeV0_NullAfterEmbeddedNul_ThrowsArgumentNull()
    {
        var ex = Assert.Throws<ArgumentNullException>(
            () => IsccLib.GenMixedCodeV0([Unit + "\0junk", null!]));
        Assert.Contains("codes[1]", ex.Message);
    }

    // ── Pinned inputs still work after a failed call ────────────────────────

    [Fact]
    public void GenVideoCodeV0_AfterNullElementFailure_StillSucceeds()
    {
        Assert.Throws<ArgumentNullException>(() => IsccLib.GenVideoCodeV0([Frame, null!]));
        Assert.StartsWith("ISCC:", IsccLib.GenVideoCodeV0([Frame]).Iscc);
    }
}
