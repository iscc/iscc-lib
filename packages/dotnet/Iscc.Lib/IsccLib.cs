// ISCC library — ISO 24138:2024 International Standard Content Code.
// P/Invoke bindings over the iscc-ffi shared library.

using System.Runtime.InteropServices;

namespace Iscc.Lib;

/// <summary>ISCC library — ISO 24138:2024 International Standard Content Code.</summary>
public static partial class IsccLib
{
    /// <summary>Native library name resolved by .NET to platform-specific binary.</summary>
    private const string LibName = "iscc_ffi";

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    private static extern bool iscc_conformance_selftest();

    /// <summary>Run all conformance tests against vendored test vectors.</summary>
    /// <returns>True if all conformance tests pass, false otherwise.</returns>
    public static bool ConformanceSelftest() => iscc_conformance_selftest();
}
