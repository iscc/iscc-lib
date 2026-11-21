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
    public static MetaCodeResult GenMetaCodeV0(
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
                return new MetaCodeResult(ConsumeNativeString(result));
            }
        }
    }

    /// <summary>Generate a Text-Code from plain text content.</summary>
    public static TextCodeResult GenTextCodeV0(string text, uint bits = 64)
    {
        byte[] nativeText = ToNativeUtf8(text)!;
        unsafe
        {
            fixed (byte* pText = nativeText)
            {
                byte* result = NativeMethods.iscc_gen_text_code_v0(pText, bits);
                return new TextCodeResult(ConsumeNativeString(result));
            }
        }
    }

    /// <summary>Generate an Image-Code from raw pixel data.</summary>
    public static ImageCodeResult GenImageCodeV0(ReadOnlySpan<byte> pixels, uint bits = 64)
    {
        unsafe
        {
            fixed (byte* pPixels = pixels)
            {
                byte* result = NativeMethods.iscc_gen_image_code_v0(
                    pPixels, (nuint)pixels.Length, bits);
                return new ImageCodeResult(ConsumeNativeString(result));
            }
        }
    }

    /// <summary>Generate an Audio-Code from a Chromaprint feature vector.</summary>
    public static AudioCodeResult GenAudioCodeV0(ReadOnlySpan<int> cv, uint bits = 64)
    {
        unsafe
        {
            fixed (int* pCv = cv)
            {
                byte* result = NativeMethods.iscc_gen_audio_code_v0(
                    pCv, (nuint)cv.Length, bits);
                return new AudioCodeResult(ConsumeNativeString(result));
            }
        }
    }

    /// <summary>Generate a Video-Code from frame signature data.</summary>
    /// <exception cref="ArgumentNullException">Thrown when <paramref name="frameSigs"/> or a frame signature is null.</exception>
    public static VideoCodeResult GenVideoCodeV0(int[][] frameSigs, uint bits = 64)
    {
        using PinnedRows rows = PinnedRows.Pin(frameSigs, nameof(frameSigs));
        unsafe
        {
            fixed (nint* pPtrs = rows.Pointers)
            fixed (nuint* pLens = rows.Lengths)
            {
                byte* result = NativeMethods.iscc_gen_video_code_v0(
                    (int**)pPtrs, pLens, (nuint)rows.Count, bits);
                return new VideoCodeResult(ConsumeNativeString(result));
            }
        }
    }

    /// <summary>Generate a Mixed-Code from multiple Content-Code strings.</summary>
    /// <exception cref="ArgumentNullException">Thrown when <paramref name="codes"/> or an element is null.</exception>
    public static MixedCodeResult GenMixedCodeV0(string[] codes, uint bits = 64)
    {
        using PinnedRows rows = PinnedRows.PinStrings(codes, nameof(codes));
        unsafe
        {
            fixed (nint* pPtrs = rows.Pointers)
            {
                byte* result = NativeMethods.iscc_gen_mixed_code_v0(
                    (byte**)pPtrs, (nuint)rows.Count, bits);
                return new MixedCodeResult(ConsumeNativeString(result));
            }
        }
    }

    /// <summary>Generate a composite ISCC-CODE from multiple unit code strings.</summary>
    /// <exception cref="ArgumentNullException">Thrown when <paramref name="codes"/> or an element is null.</exception>
    public static IsccCodeResult GenIsccCodeV0(string[] codes, bool wide = false)
    {
        using PinnedRows rows = PinnedRows.PinStrings(codes, nameof(codes));
        unsafe
        {
            fixed (nint* pPtrs = rows.Pointers)
            {
                byte* result = NativeMethods.iscc_gen_iscc_code_v0(
                    (byte**)pPtrs, (nuint)rows.Count, wide);
                return new IsccCodeResult(ConsumeNativeString(result));
            }
        }
    }

    /// <summary>Generate an experimental ISCC-IDv1 from a timestamp, HUB-ID, and realm.</summary>
    /// <param name="timestamp">Microseconds since the Unix epoch (must be less than 2^52).</param>
    /// <param name="hubId">HUB identifier (must be less than 2^12).</param>
    /// <param name="realm">Realm selector (0 for testnet, 1 for the first operational mainnet).</param>
    public static IsccIdResult GenIsccIdV1(ulong timestamp, ushort hubId, byte realm)
    {
        unsafe
        {
            byte* result = NativeMethods.iscc_gen_iscc_id_v1(timestamp, hubId, realm);
            return new IsccIdResult(ConsumeNativeString(result));
        }
    }

    /// <summary>Generate a Data-Code from raw byte data.</summary>
    public static DataCodeResult GenDataCodeV0(ReadOnlySpan<byte> data, uint bits = 64)
    {
        unsafe
        {
            fixed (byte* pData = data)
            {
                byte* result = NativeMethods.iscc_gen_data_code_v0(
                    pData, (nuint)data.Length, bits);
                return new DataCodeResult(ConsumeNativeString(result));
            }
        }
    }

    /// <summary>Generate an Instance-Code from raw byte data.</summary>
    public static InstanceCodeResult GenInstanceCodeV0(ReadOnlySpan<byte> data, uint bits = 64)
    {
        unsafe
        {
            fixed (byte* pData = data)
            {
                byte* result = NativeMethods.iscc_gen_instance_code_v0(
                    pData, (nuint)data.Length, bits);
                return new InstanceCodeResult(ConsumeNativeString(result));
            }
        }
    }

    /// <summary>Generate a composite ISCC-CODE from a file path (Data-Code + Instance-Code).</summary>
    public static SumCodeResult GenSumCodeV0(
        string path,
        uint bits = 64,
        bool wide = false,
        bool addUnits = false)
    {
        byte[] nativePath = ToNativeUtf8(path)!;
        unsafe
        {
            IsccSumCodeResult result;
            fixed (byte* pPath = nativePath)
            {
                result = NativeMethods.iscc_gen_sum_code_v0(pPath, bits, wide, addUnits);
            }

            try
            {
                if (!result.ok)
                    throw new IsccException(GetLastError());

                string iscc = Marshal.PtrToStringUTF8((IntPtr)result.iscc) ?? string.Empty;
                string datahash = Marshal.PtrToStringUTF8((IntPtr)result.datahash) ?? string.Empty;

                string[]? units = null;
                if (result.units != null)
                {
                    List<string> unitsList = new();
                    for (int i = 0; result.units[i] != null; i++)
                        unitsList.Add(Marshal.PtrToStringUTF8((IntPtr)result.units[i])!);
                    units = unitsList.ToArray();
                }

                return new SumCodeResult(iscc, datahash, result.filesize, units);
            }
            finally
            {
                NativeMethods.iscc_free_sum_code_result(result);
            }
        }
    }

    // ── Encoding Utilities ──────────────────────────────────────────────────

    /// <summary>Encode raw bytes to a base64url string (no padding).</summary>
    public static string EncodeBase64(ReadOnlySpan<byte> data)
    {
        unsafe
        {
            fixed (byte* pData = data)
            {
                byte* result = NativeMethods.iscc_encode_base64(
                    pData, (nuint)data.Length);
                return ConsumeNativeString(result);
            }
        }
    }

    /// <summary>Convert a JSON string into a data: URL with JCS canonicalization.</summary>
    public static string JsonToDataUrl(string json)
    {
        byte[] nativeJson = ToNativeUtf8(json)!;
        unsafe
        {
            fixed (byte* pJson = nativeJson)
            {
                byte* result = NativeMethods.iscc_json_to_data_url(pJson);
                return ConsumeNativeString(result);
            }
        }
    }

    // ── Codec ────────────────────────────────────────────────────────────

    /// <summary>Decode an ISCC string into its header fields and raw digest bytes.</summary>
    public static DecodeResult IsccDecode(string iscc)
    {
        byte[] nativeIscc = ToNativeUtf8(iscc)!;
        unsafe
        {
            IsccDecodeResult result;
            fixed (byte* pIscc = nativeIscc)
            {
                result = NativeMethods.iscc_decode(pIscc);
            }

            try
            {
                if (!result.ok)
                    throw new IsccException(GetLastError());

                byte[] digestBytes = new Span<byte>(
                    result.digest.data, (int)result.digest.len).ToArray();

                return new DecodeResult(
                    result.maintype, result.subtype,
                    result.version, result.length, digestBytes);
            }
            finally
            {
                NativeMethods.iscc_free_decode_result(result);
            }
        }
    }

    /// <summary>Decompose a composite ISCC-CODE into individual ISCC-UNIT strings.</summary>
    public static string[] IsccDecompose(string isccCode)
    {
        byte[] nativeCode = ToNativeUtf8(isccCode)!;
        unsafe
        {
            fixed (byte* pCode = nativeCode)
            {
                byte** arr = NativeMethods.iscc_decompose(pCode);
                return ConsumeNativeStringArray(arr);
            }
        }
    }

    /// <summary>Encode ISCC header fields and digest bytes into an ISCC string.</summary>
    public static string EncodeComponent(
        byte mtype, byte stype, byte version, uint bitLength,
        ReadOnlySpan<byte> digest)
    {
        unsafe
        {
            fixed (byte* pDigest = digest)
            {
                byte* result = NativeMethods.iscc_encode_component(
                    mtype, stype, version, bitLength,
                    pDigest, (nuint)digest.Length);
                return ConsumeNativeString(result);
            }
        }
    }

    /// <summary>Encode ISCC-UNIT strings into an ISCC-SEQ byte string (IEP-0020).</summary>
    /// <param name="units">One or more ISCC-UNIT strings (ISCC-CODE, ID and FLAKE are not permitted).</param>
    /// <exception cref="ArgumentNullException">Thrown when <paramref name="units"/> or an element is null.</exception>
    /// <exception cref="IsccException">Thrown when the list is empty, a unit contains an embedded NUL character, or a unit is invalid.</exception>
    public static byte[] EncodeSeq(string[] units)
    {
        using PinnedRows rows = PinnedRows.PinStrings(units, nameof(units));
        unsafe
        {
            fixed (nint* pPtrs = rows.Pointers)
            {
                IsccByteBuffer buf = NativeMethods.iscc_encode_seq((byte**)pPtrs, (nuint)rows.Count);
                return ConsumeByteBuffer(buf);
            }
        }
    }

    /// <summary>Decode an ISCC-SEQ byte string into canonical ISCC-UNIT strings (IEP-0020).</summary>
    /// <param name="data">ISCC-SEQ bytes as produced by <see cref="EncodeSeq"/>.</param>
    /// <returns><c>ISCC:</c>-prefixed upper-case unit strings in wire order.</returns>
    /// <exception cref="IsccException">Thrown when the input is empty, truncated or malformed.</exception>
    public static string[] DecodeSeq(ReadOnlySpan<byte> data)
    {
        unsafe
        {
            // An empty span passes a null pointer with length 0, which the native side
            // accepts and routes to the core's "Empty ISCC-SEQ" error.
            fixed (byte* pData = data)
            {
                byte** arr = NativeMethods.iscc_decode_seq(pData, (nuint)data.Length);
                return ConsumeNativeStringArray(arr);
            }
        }
    }

    // ── Algorithm Primitives ────────────────────────────────────────────────

    /// <summary>Compute a SimHash digest from a set of byte-array digests.</summary>
    /// <exception cref="ArgumentNullException">Thrown when <paramref name="digests"/> or a digest is null.</exception>
    public static byte[] AlgSimhash(byte[][] digests)
    {
        using PinnedRows rows = PinnedRows.Pin(digests, nameof(digests));
        unsafe
        {
            fixed (nint* pPtrs = rows.Pointers)
            fixed (nuint* pLens = rows.Lengths)
            {
                IsccByteBuffer buf = NativeMethods.iscc_alg_simhash(
                    (byte**)pPtrs, pLens, (nuint)rows.Count);
                return ConsumeByteBuffer(buf);
            }
        }
    }

    /// <summary>Compute a 256-bit MinHash digest from 32-bit integer features.</summary>
    public static byte[] AlgMinhash256(ReadOnlySpan<uint> features)
    {
        unsafe
        {
            fixed (uint* pFeatures = features)
            {
                IsccByteBuffer buf = NativeMethods.iscc_alg_minhash_256(
                    pFeatures, (nuint)features.Length);
                return ConsumeByteBuffer(buf);
            }
        }
    }

    /// <summary>Split data into content-defined chunks using gear rolling hash.</summary>
    public static byte[][] AlgCdcChunks(
        ReadOnlySpan<byte> data, bool utf32 = false, uint avgChunkSize = 1024)
    {
        unsafe
        {
            fixed (byte* pData = data)
            {
                IsccByteBufferArray arr = NativeMethods.iscc_alg_cdc_chunks(
                    pData, (nuint)data.Length, utf32, avgChunkSize);
                return ConsumeByteBufferArray(arr);
            }
        }
    }

    /// <summary>Compute a similarity-preserving hash from video frame signatures.</summary>
    /// <exception cref="ArgumentNullException">Thrown when <paramref name="frameSigs"/> or a frame signature is null.</exception>
    public static byte[] SoftHashVideoV0(int[][] frameSigs, uint bits = 64)
    {
        using PinnedRows rows = PinnedRows.Pin(frameSigs, nameof(frameSigs));
        unsafe
        {
            fixed (nint* pPtrs = rows.Pointers)
            fixed (nuint* pLens = rows.Lengths)
            {
                IsccByteBuffer buf = NativeMethods.iscc_soft_hash_video_v0(
                    (int**)pPtrs, pLens, (nuint)rows.Count, bits);
                return ConsumeByteBuffer(buf);
            }
        }
    }

    // ── Utilities ──────────────────────────────────────────────────────────

    /// <summary>Generate sliding window n-grams of the given width from a string.</summary>
    public static string[] SlidingWindow(string seq, uint width)
    {
        byte[] nativeSeq = ToNativeUtf8(seq)!;
        unsafe
        {
            fixed (byte* pSeq = nativeSeq)
            {
                byte** arr = NativeMethods.iscc_sliding_window(pSeq, width);
                return ConsumeNativeStringArray(arr);
            }
        }
    }

    // ── Diagnostics ────────────────────────────────────────────────────────

    /// <summary>Run all conformance tests against vendored test vectors.</summary>
    /// <returns>True if all conformance tests pass, false otherwise.</returns>
    public static bool ConformanceSelftest() => NativeMethods.iscc_conformance_selftest();

    // ── Private Helpers ─────────────────────────────────────────────────────

    /// <summary>Convert a C# string to a null-terminated UTF-8 byte array for native calls.</summary>
    /// <remarks>
    /// The native library reads strings up to the first NUL, so a string with an embedded NUL
    /// is rejected instead of being silently truncated.
    /// </remarks>
    /// <exception cref="IsccException">Thrown when <paramref name="s"/> contains an embedded NUL character.</exception>
    private static byte[]? ToNativeUtf8(string? s)
    {
        if (s is null)
            return null;
        if (s.Contains('\0'))
            throw new IsccException("string input contains an embedded NUL character");
        int len = Encoding.UTF8.GetByteCount(s);
        byte[] buf = new byte[len + 1];
        Encoding.UTF8.GetBytes(s, 0, s.Length, buf, 0);
        return buf;
    }

    /// <summary>Marshal a native UTF-8 string to managed, free the native pointer, or throw on null.</summary>
    internal static unsafe string ConsumeNativeString(byte* ptr)
    {
        if (ptr is null)
            throw new IsccException(GetLastError());
        string result = Marshal.PtrToStringUTF8((IntPtr)ptr) ?? string.Empty;
        NativeMethods.iscc_free_string(ptr);
        return result;
    }

    /// <summary>Marshal a native byte buffer to managed byte array, then free the buffer.</summary>
    private static unsafe byte[] ConsumeByteBuffer(IsccByteBuffer buf)
    {
        if (buf.data is null)
            throw new IsccException(GetLastError());
        try
        {
            return new Span<byte>(buf.data, (int)buf.len).ToArray();
        }
        finally
        {
            NativeMethods.iscc_free_byte_buffer(buf);
        }
    }

    /// <summary>Marshal a native byte buffer array to managed jagged array, then free.</summary>
    private static unsafe byte[][] ConsumeByteBufferArray(IsccByteBufferArray arr)
    {
        if (arr.buffers is null)
            throw new IsccException(GetLastError());
        try
        {
            byte[][] result = new byte[(int)arr.count][];
            for (int i = 0; i < (int)arr.count; i++)
            {
                IsccByteBuffer buf = arr.buffers[i];
                result[i] = new Span<byte>(buf.data, (int)buf.len).ToArray();
            }
            return result;
        }
        finally
        {
            NativeMethods.iscc_free_byte_buffer_array(arr);
        }
    }

    /// <summary>Marshal a NULL-terminated native string array to managed, then free the array.</summary>
    private static unsafe string[] ConsumeNativeStringArray(byte** arr)
    {
        if (arr is null)
            throw new IsccException(GetLastError());
        try
        {
            var list = new List<string>();
            for (int i = 0; arr[i] != null; i++)
                list.Add(Marshal.PtrToStringUTF8((IntPtr)arr[i])!);
            return list.ToArray();
        }
        finally
        {
            NativeMethods.iscc_free_string_array(arr);
        }
    }

    /// <summary>
    /// Pins the inner arrays of a jagged input and lays them out as native pointer and length
    /// arrays, so every jagged-array method marshals, validates and frees the same way.
    /// </summary>
    /// <remarks>
    /// <see cref="Pointers"/> and <see cref="Lengths"/> hold at least one slot, so <c>fixed</c>
    /// never yields a NULL pointer for an empty input and the core's own error surfaces instead of
    /// the FFI's "must not be NULL". The real element count is <see cref="Count"/>.
    /// </remarks>
    private sealed class PinnedRows : IDisposable
    {
        private readonly GCHandle[] _handles;

        /// <summary>Number of rows in the input (the count passed to the native call).</summary>
        public int Count { get; }

        /// <summary>Addresses of the pinned rows; length is <c>max(Count, 1)</c>.</summary>
        public nint[] Pointers { get; }

        /// <summary>Element counts of the pinned rows; length is <c>max(Count, 1)</c>.</summary>
        public nuint[] Lengths { get; }

        /// <summary>Allocate empty slots for <paramref name="count"/> rows.</summary>
        private PinnedRows(int count)
        {
            Count = count;
            _handles = new GCHandle[count];
            Pointers = new nint[Math.Max(count, 1)];
            Lengths = new nuint[Math.Max(count, 1)];
        }

        /// <summary>Pin every row of <paramref name="rows"/>.</summary>
        /// <exception cref="ArgumentNullException">Thrown when <paramref name="rows"/> or a row is null.</exception>
        public static PinnedRows Pin<T>(T[]?[] rows, string paramName) where T : unmanaged
        {
            ThrowIfAnyNull(rows, paramName);
            var pinned = new PinnedRows(rows.Length);
            try
            {
                for (int i = 0; i < rows.Length; i++)
                {
                    T[] row = rows[i]!;
                    pinned._handles[i] = GCHandle.Alloc(row, GCHandleType.Pinned);
                    pinned.Pointers[i] = pinned._handles[i].AddrOfPinnedObject();
                    pinned.Lengths[i] = (nuint)row.Length;
                }
            }
            catch
            {
                pinned.Dispose();
                throw;
            }
            return pinned;
        }

        /// <summary>Convert each string to NUL-terminated UTF-8 and pin the results.</summary>
        /// <exception cref="ArgumentNullException">Thrown when <paramref name="values"/> or an element is null.</exception>
        /// <exception cref="IsccException">Thrown when an element contains an embedded NUL character.</exception>
        public static PinnedRows PinStrings(string[] values, string paramName)
        {
            ThrowIfAnyNull(values, paramName);
            return Pin(Array.ConvertAll(values, ToNativeUtf8), paramName);
        }

        /// <summary>Throw when <paramref name="items"/> or any of its elements is null.</summary>
        /// <exception cref="ArgumentNullException">Names <paramref name="paramName"/> and the index of the null element.</exception>
        private static void ThrowIfAnyNull<T>(T?[] items, string paramName) where T : class
        {
            ArgumentNullException.ThrowIfNull(items, paramName);
            for (int i = 0; i < items.Length; i++)
            {
                if (items[i] is null)
                    throw new ArgumentNullException(paramName, $"{paramName}[{i}] must not be null");
            }
        }

        /// <summary>Free every handle that is still pinned; safe to call more than once.</summary>
        public void Dispose()
        {
            for (int i = 0; i < _handles.Length; i++)
            {
                if (_handles[i].IsAllocated)
                    _handles[i].Free();
            }
        }
    }

    /// <summary>Read the last error message from the native library.</summary>
    internal static unsafe string GetLastError()
    {
        byte* err = NativeMethods.iscc_last_error();
        if (err is null)
            return "Unknown ISCC error";
        return Marshal.PtrToStringUTF8((IntPtr)err) ?? "Unknown ISCC error";
    }
}
