// Streaming Instance-Code hasher — wraps the native FfiInstanceHasher via SafeHandle for safe disposal.

using System.Runtime.InteropServices;

namespace Iscc.Lib;

/// <summary>Streaming hasher for generating ISCC Instance-Codes incrementally.</summary>
/// <remarks>This type is not thread-safe. Do not call methods from multiple threads concurrently.</remarks>
public sealed class IsccInstanceHasher : IDisposable
{
    private readonly InstanceHasherHandle _handle;
    private bool _finalized;

    /// <summary>Create a new streaming Instance-Code hasher.</summary>
    public IsccInstanceHasher()
    {
        unsafe
        {
            _handle = new InstanceHasherHandle(NativeMethods.iscc_instance_hasher_new());
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
                bool ok = NativeMethods.iscc_instance_hasher_update(
                    (FfiInstanceHasher*)(void*)_handle.DangerousGetHandle(),
                    pData, (nuint)data.Length);
                if (!ok)
                    throw new IsccException(IsccLib.GetLastError());
            }
        }
    }

    /// <summary>Finalize the hasher and return the ISCC Instance-Code result.</summary>
    public InstanceCodeResult Finalize(uint bits = 64)
    {
        ObjectDisposedException.ThrowIf(_handle.IsInvalid || _handle.IsClosed, this);
        if (_finalized)
            throw new InvalidOperationException("Hasher already finalized");

        _finalized = true;
        unsafe
        {
            byte* result = NativeMethods.iscc_instance_hasher_finalize(
                (FfiInstanceHasher*)(void*)_handle.DangerousGetHandle(), bits);
            return new InstanceCodeResult(IsccLib.ConsumeNativeString(result));
        }
    }

    /// <summary>Release the native hasher resources.</summary>
    public void Dispose()
    {
        _handle.Dispose();
    }

    /// <summary>SafeHandle wrapper for the opaque FfiInstanceHasher pointer.</summary>
    private sealed class InstanceHasherHandle : SafeHandle
    {
        /// <summary>Wrap a native FfiInstanceHasher pointer for safe disposal.</summary>
        public unsafe InstanceHasherHandle(FfiInstanceHasher* ptr)
            : base(IntPtr.Zero, ownsHandle: true)
        {
            SetHandle((IntPtr)ptr);
        }

        /// <summary>Whether the handle is invalid (null pointer).</summary>
        public override bool IsInvalid => handle == IntPtr.Zero;

        /// <summary>Free the native FfiInstanceHasher.</summary>
        protected override bool ReleaseHandle()
        {
            unsafe
            {
                NativeMethods.iscc_instance_hasher_free(
                    (FfiInstanceHasher*)(void*)handle);
            }
            return true;
        }
    }
}
