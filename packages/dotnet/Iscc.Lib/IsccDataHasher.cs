// Streaming Data-Code hasher — wraps the native FfiDataHasher via SafeHandle for safe disposal.

using System.Runtime.InteropServices;

namespace Iscc.Lib;

/// <summary>Streaming hasher for generating ISCC Data-Codes incrementally.</summary>
/// <remarks>This type is not thread-safe. Do not call methods from multiple threads concurrently.</remarks>
public sealed class IsccDataHasher : IDisposable
{
    private readonly DataHasherHandle _handle;
    private bool _finalized;

    /// <summary>Create a new streaming Data-Code hasher.</summary>
    public IsccDataHasher()
    {
        unsafe
        {
            _handle = new DataHasherHandle(NativeMethods.iscc_data_hasher_new());
        }
    }

    /// <summary>Feed data into the hasher. May be called multiple times before Finalize.</summary>
    public void Update(ReadOnlySpan<byte> data)
    {
        ObjectDisposedException.ThrowIf(_handle.IsInvalid || _handle.IsClosed, this);
        if (_finalized)
            throw new InvalidOperationException("Hasher already finalized");

        unsafe
        {
            fixed (byte* pData = data)
            {
                bool ok = NativeMethods.iscc_data_hasher_update(
                    (FfiDataHasher*)(void*)_handle.DangerousGetHandle(),
                    pData, (nuint)data.Length);
                if (!ok)
                    throw new IsccException(IsccLib.GetLastError());
            }
        }
    }

    /// <summary>Finalize the hasher and return the ISCC Data-Code result.</summary>
    public DataCodeResult Finalize(uint bits = 64)
    {
        ObjectDisposedException.ThrowIf(_handle.IsInvalid || _handle.IsClosed, this);
        if (_finalized)
            throw new InvalidOperationException("Hasher already finalized");

        _finalized = true;
        unsafe
        {
            byte* result = NativeMethods.iscc_data_hasher_finalize(
                (FfiDataHasher*)(void*)_handle.DangerousGetHandle(), bits);
            return new DataCodeResult(IsccLib.ConsumeNativeString(result));
        }
    }

    /// <summary>Release the native hasher resources.</summary>
    public void Dispose()
    {
        _handle.Dispose();
    }

    /// <summary>SafeHandle wrapper for the opaque FfiDataHasher pointer.</summary>
    private sealed class DataHasherHandle : SafeHandle
    {
        /// <summary>Wrap a native FfiDataHasher pointer for safe disposal.</summary>
        public unsafe DataHasherHandle(FfiDataHasher* ptr)
            : base(IntPtr.Zero, ownsHandle: true)
        {
            SetHandle((IntPtr)ptr);
        }

        /// <summary>Whether the handle is invalid (null pointer).</summary>
        public override bool IsInvalid => handle == IntPtr.Zero;

        /// <summary>Free the native FfiDataHasher.</summary>
        protected override bool ReleaseHandle()
        {
            unsafe
            {
                NativeMethods.iscc_data_hasher_free(
                    (FfiDataHasher*)(void*)handle);
            }
            return true;
        }
    }
}
