// ISCC library — ISO 24138:2024 International Standard Content Code.
// Idiomatic C# wrappers over the csbindgen-generated NativeMethods P/Invoke layer.

using System.Runtime.InteropServices;
using System.Text;

namespace Iscc.Lib;

/// <summary>ISCC library — ISO 24138:2024 International Standard Content Code.</summary>
public static partial class IsccLib
{
    // ── Constants ───────────────────────────────────────────────────────────

    /// <summary>Maximum byte length for the name field after trimming.</summary>
    public static uint MetaTrimName => NativeMethods.iscc_meta_trim_name();

    /// <summary>Maximum byte length for the description field after trimming.</summary>
    public static uint MetaTrimDescription => NativeMethods.iscc_meta_trim_description();

    /// <summary>Maximum byte length for the meta field payload after decoding.</summary>
    public static uint MetaTrimMeta => NativeMethods.iscc_meta_trim_meta();

    /// <summary>Default read buffer size for streaming I/O (4 MB).</summary>
    public static uint IoReadSize => NativeMethods.iscc_io_read_size();

    /// <summary>Sliding window width for text n-gram generation.</summary>
    public static uint TextNgramSize => NativeMethods.iscc_text_ngram_size();

    // ── Text Utilities ──────────────────────────────────────────────────────

    /// <summary>Clean and normalize text for display.</summary>
    public static string TextClean(string text)
    {
        byte[] nativeText = ToNativeUtf8(text)!;
        unsafe
        {
            fixed (byte* pText = nativeText)
            {
                byte* result = NativeMethods.iscc_text_clean(pText);
                return ConsumeNativeString(result);
            }
        }
    }

    /// <summary>Remove newlines and collapse whitespace to single spaces.</summary>
    public static string TextRemoveNewlines(string text)
    {
        byte[] nativeText = ToNativeUtf8(text)!;
        unsafe
        {
            fixed (byte* pText = nativeText)
            {
                byte* result = NativeMethods.iscc_text_remove_newlines(pText);
                return ConsumeNativeString(result);
            }
        }
    }

    /// <summary>Trim text so its UTF-8 encoded size does not exceed nbytes.</summary>
    public static string TextTrim(string text, uint nbytes)
    {
        byte[] nativeText = ToNativeUtf8(text)!;
        unsafe
        {
            fixed (byte* pText = nativeText)
            {
                byte* result = NativeMethods.iscc_text_trim(pText, (nuint)nbytes);
                return ConsumeNativeString(result);
            }
        }
    }

    /// <summary>Normalize and simplify text for similarity hashing.</summary>
    public static string TextCollapse(string text)
    {
        byte[] nativeText = ToNativeUtf8(text)!;
        unsafe
        {
            fixed (byte* pText = nativeText)
            {
                byte* result = NativeMethods.iscc_text_collapse(pText);
                return ConsumeNativeString(result);
            }
        }
    }

    // ── Gen Functions ───────────────────────────────────────────────────────

    /// <summary>Generate a Meta-Code from name and optional metadata.</summary>
    public static string GenMetaCodeV0(
        string name,
        string? description = null,
        string? meta = null,
        uint bits = 64)
    {
        byte[] nativeName = ToNativeUtf8(name)!;
        byte[]? nativeDesc = ToNativeUtf8(description);
        byte[]? nativeMeta = ToNativeUtf8(meta);
        unsafe
        {
            fixed (byte* pName = nativeName)
            fixed (byte* pDesc = nativeDesc)
            fixed (byte* pMeta = nativeMeta)
            {
                byte* result = NativeMethods.iscc_gen_meta_code_v0(
                    pName, pDesc, pMeta, bits);
                return ConsumeNativeString(result);
            }
        }
    }

    /// <summary>Generate a Text-Code from plain text content.</summary>
    public static string GenTextCodeV0(string text, uint bits = 64)
    {
        byte[] nativeText = ToNativeUtf8(text)!;
        unsafe
        {
            fixed (byte* pText = nativeText)
            {
                byte* result = NativeMethods.iscc_gen_text_code_v0(pText, bits);
                return ConsumeNativeString(result);
            }
        }
    }

    /// <summary>Generate a Data-Code from raw byte data.</summary>
    public static string GenDataCodeV0(ReadOnlySpan<byte> data, uint bits = 64)
    {
        unsafe
        {
            fixed (byte* pData = data)
            {
                byte* result = NativeMethods.iscc_gen_data_code_v0(
                    pData, (nuint)data.Length, bits);
                return ConsumeNativeString(result);
            }
        }
    }

    /// <summary>Generate an Instance-Code from raw byte data.</summary>
    public static string GenInstanceCodeV0(ReadOnlySpan<byte> data, uint bits = 64)
    {
        unsafe
        {
            fixed (byte* pData = data)
            {
                byte* result = NativeMethods.iscc_gen_instance_code_v0(
                    pData, (nuint)data.Length, bits);
                return ConsumeNativeString(result);
            }
        }
    }

    // ── Diagnostics ────────────────────────────────────────────────────────

    /// <summary>Run all conformance tests against vendored test vectors.</summary>
    /// <returns>True if all conformance tests pass, false otherwise.</returns>
    public static bool ConformanceSelftest() => NativeMethods.iscc_conformance_selftest();

    // ── Private Helpers ─────────────────────────────────────────────────────

    /// <summary>Convert a C# string to a null-terminated UTF-8 byte array for native calls.</summary>
    private static byte[]? ToNativeUtf8(string? s)
    {
        if (s is null)
            return null;
        int len = Encoding.UTF8.GetByteCount(s);
        byte[] buf = new byte[len + 1];
        Encoding.UTF8.GetBytes(s, 0, s.Length, buf, 0);
        return buf;
    }

    /// <summary>Marshal a native UTF-8 string to managed, free the native pointer, or throw on null.</summary>
    private static unsafe string ConsumeNativeString(byte* ptr)
    {
        if (ptr is null)
            throw new IsccException(GetLastError());
        string result = Marshal.PtrToStringUTF8((IntPtr)ptr) ?? string.Empty;
        NativeMethods.iscc_free_string(ptr);
        return result;
    }

    /// <summary>Read the last error message from the native library.</summary>
    private static unsafe string GetLastError()
    {
        byte* err = NativeMethods.iscc_last_error();
        if (err is null)
            return "Unknown ISCC error";
        return Marshal.PtrToStringUTF8((IntPtr)err) ?? "Unknown ISCC error";
    }
}
