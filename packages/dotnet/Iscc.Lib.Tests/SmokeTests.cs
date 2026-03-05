// Smoke tests for ISCC .NET bindings — validates P/Invoke into iscc-ffi.

using Iscc.Lib;
using Xunit;

namespace Iscc.Lib.Tests;

/// <summary>Validates that the P/Invoke bridge to the Rust FFI library works end-to-end.</summary>
public class SmokeTests
{
    [Fact]
    public void ConformanceSelftest_ReturnsTrue()
    {
        bool result = IsccLib.ConformanceSelftest();
        Assert.True(result, "Conformance selftest should pass against vendored test vectors");
    }
}
