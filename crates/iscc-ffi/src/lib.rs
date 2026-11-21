//! C FFI bindings for iscc-lib.
//!
//! Exposes all 10 `gen_*_v0` functions, 4 algorithm primitives, codec
//! functions (`encode_component`, `iscc_decode`, `encode_seq`, `decode_seq`,
//! `json_to_data_url`), and 5 algorithm constants as `extern "C"` symbols for
//! integration from C, Go, Java, C#, and any other language with C interop.
//!
//! ## Memory model
//!
//! Functions return heap-allocated C strings (`*mut c_char`) via `CString`.
//! The caller must free them with `iscc_free_string()`. Algorithm primitives
//! return `IsccByteBuffer` or `IsccByteBufferArray` — callers must free these
//! with `iscc_free_byte_buffer()` or `iscc_free_byte_buffer_array()`. On
//! error, functions return `NULL` and the caller retrieves the error message
//! via `iscc_last_error()`.
//!
//! ## Safety
//!
//! All `unsafe` in this crate is confined to the FFI boundary: dereferencing
//! raw pointers from the caller, `no_mangle` for symbol export, and
//! `extern "C"` ABI. The core `iscc_lib` crate remains 100% safe Rust.

use std::cell::RefCell;
use std::ffi::{CStr, CString, c_char};
use std::ptr;

thread_local! {
    /// Thread-local storage for the last error message.
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

/// Store an error message in thread-local storage.
fn set_last_error(msg: &str) {
    LAST_ERROR.with(|e| *e.borrow_mut() = CString::new(msg).ok());
}

/// Clear the thread-local error message.
fn clear_last_error() {
    LAST_ERROR.with(|e| *e.borrow_mut() = None);
}

// --- Algorithm constants ---

/// Maximum byte length for the name field after trimming.
#[unsafe(no_mangle)]
pub extern "C" fn iscc_meta_trim_name() -> u32 {
    iscc_lib::META_TRIM_NAME as u32
}

/// Maximum byte length for the description field after trimming.
#[unsafe(no_mangle)]
pub extern "C" fn iscc_meta_trim_description() -> u32 {
    iscc_lib::META_TRIM_DESCRIPTION as u32
}

/// Maximum byte length for the meta field payload after decoding.
#[unsafe(no_mangle)]
pub extern "C" fn iscc_meta_trim_meta() -> u32 {
    iscc_lib::META_TRIM_META as u32
}

/// Default read buffer size for streaming I/O (4 MB).
#[unsafe(no_mangle)]
pub extern "C" fn iscc_io_read_size() -> u32 {
    iscc_lib::IO_READ_SIZE as u32
}

/// Sliding window width for text n-gram generation.
#[unsafe(no_mangle)]
pub extern "C" fn iscc_text_ngram_size() -> u32 {
    iscc_lib::TEXT_NGRAM_SIZE as u32
}

// --- Memory allocation helpers (for WASM host) ---

/// Allocate `size` bytes of WASM-side memory.
///
/// Returns a pointer the host can write into. The host must call
/// `iscc_dealloc` to free this memory. Returns a dangling (non-null)
/// pointer for `size == 0`.
///
/// # Safety
///
/// The caller must ensure `size` bytes are actually needed and must free
/// the allocation with `iscc_dealloc(ptr, size)`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_alloc(size: usize) -> *mut u8 {
    if size == 0 {
        return std::ptr::NonNull::dangling().as_ptr();
    }
    let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
    unsafe { std::alloc::alloc(layout) }
}

/// Free `size` bytes of WASM-side memory at `ptr`, previously allocated
/// by `iscc_alloc`.
///
/// No-op if `ptr` is null. Passing `size == 0` is a no-op because
/// `iscc_alloc(0)` returns a dangling pointer that was never allocated.
///
/// # Safety
///
/// - `ptr` must have been returned by `iscc_alloc(size)` with the same `size`.
/// - Each allocation must be freed exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_dealloc(ptr: *mut u8, size: usize) {
    if ptr.is_null() || size == 0 {
        return;
    }
    let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
    unsafe { std::alloc::dealloc(ptr, layout) };
}

/// Convert a Rust `String` result to a heap-allocated C string.
///
/// On success, returns `CString::into_raw()`. On error, sets the
/// thread-local error and returns `NULL`.
fn result_to_c_string(result: Result<String, iscc_lib::IsccError>) -> *mut c_char {
    match result {
        Ok(s) => match CString::new(s) {
            Ok(cs) => cs.into_raw(),
            Err(e) => {
                set_last_error(&e.to_string());
                ptr::null_mut()
            }
        },
        Err(e) => {
            set_last_error(&e.to_string());
            ptr::null_mut()
        }
    }
}

/// Convert a Rust `String` to a heap-allocated C string.
///
/// On success, returns `CString::into_raw()`. On failure (interior NUL byte),
/// sets the thread-local error and returns `NULL`.
fn string_to_c(s: String) -> *mut c_char {
    match CString::new(s) {
        Ok(cs) => cs.into_raw(),
        Err(e) => {
            set_last_error(&e.to_string());
            ptr::null_mut()
        }
    }
}

/// Convert a `Vec<String>` to a heap-allocated NULL-terminated array of C strings.
///
/// Each element is converted to a `CString` and its raw pointer placed in the
/// array. A NULL terminator is appended. The caller must free the result with
/// `iscc_free_string_array()`. Returns `NULL` if any element contains an
/// interior NUL byte.
fn vec_to_c_string_array(v: Vec<String>) -> *mut *mut c_char {
    let mut ptrs: Vec<*mut c_char> = Vec::with_capacity(v.len() + 1);
    for s in v {
        match CString::new(s) {
            Ok(cs) => ptrs.push(cs.into_raw()),
            Err(e) => {
                // Free already-allocated strings on failure
                for &ptr in &ptrs {
                    // SAFETY: each ptr was produced by CString::into_raw() above
                    drop(unsafe { CString::from_raw(ptr) });
                }
                set_last_error(&e.to_string());
                return ptr::null_mut();
            }
        }
    }
    ptrs.push(ptr::null_mut()); // NULL terminator
    Box::into_raw(ptrs.into_boxed_slice()) as *mut *mut c_char
}

// ── Byte buffer types ────────────────────────────────────────────────────

/// Heap-allocated byte buffer returned to C callers.
///
/// On success, `data` points to a contiguous byte array of `len` bytes.
/// On error, `data` is `NULL` and `len` is 0. Callers must free with
/// `iscc_free_byte_buffer()`.
#[repr(C)]
pub struct IsccByteBuffer {
    /// Pointer to the byte data (`NULL` if error).
    pub data: *mut u8,
    /// Number of bytes.
    pub len: usize,
}

/// Array of byte buffers (for `iscc_alg_cdc_chunks`).
///
/// Contains `count` elements pointed to by `buffers`. Callers must free
/// with `iscc_free_byte_buffer_array()`.
#[repr(C)]
pub struct IsccByteBufferArray {
    /// Pointer to array of `IsccByteBuffer` elements.
    pub buffers: *mut IsccByteBuffer,
    /// Number of buffers.
    pub count: usize,
}

/// Return a null/error `IsccByteBuffer`.
fn null_byte_buffer() -> IsccByteBuffer {
    IsccByteBuffer {
        data: ptr::null_mut(),
        len: 0,
    }
}

/// Return a null/error `IsccByteBufferArray`.
fn null_byte_buffer_array() -> IsccByteBufferArray {
    IsccByteBufferArray {
        buffers: ptr::null_mut(),
        count: 0,
    }
}

/// Convert an owned `Vec<u8>` to a C-compatible `IsccByteBuffer`.
///
/// Uses `into_boxed_slice` to guarantee capacity equals length, then
/// transfers ownership via `Box::into_raw`.
fn vec_to_byte_buffer(v: Vec<u8>) -> IsccByteBuffer {
    let len = v.len();
    let data = Box::into_raw(v.into_boxed_slice()) as *mut u8;
    IsccByteBuffer { data, len }
}

/// Convert a `*const c_char` to a `&str`, returning `NULL` on failure.
///
/// # Safety
///
/// The pointer must be valid and point to a null-terminated UTF-8 string,
/// or be null (which triggers an error return).
unsafe fn ptr_to_str<'a>(ptr: *const c_char, param_name: &str) -> Option<&'a str> {
    if ptr.is_null() {
        set_last_error(&format!("{param_name} must not be NULL"));
        return None;
    }
    // SAFETY: caller guarantees ptr is valid and null-terminated
    match unsafe { CStr::from_ptr(ptr) }.to_str() {
        Ok(s) => Some(s),
        Err(e) => {
            set_last_error(&format!("{param_name} is not valid UTF-8: {e}"));
            None
        }
    }
}

/// Borrow a pointer+length input as a slice, accepting `(NULL, 0)` as empty.
///
/// A length of 0 yields an empty slice whatever `ptr` is, so an empty input
/// reaches the core and gets the core's own error (if any). A NULL `ptr` with
/// `len > 0` sets `"<name> must not be NULL"` and returns `None`.
///
/// # Safety
///
/// If `len > 0` and `ptr` is non-null, `ptr` must be valid for reads of `len`
/// elements of `T` for the lifetime `'a`.
unsafe fn slice_or_empty<'a, T>(
    ptr: *const T,
    len: usize,
    name: impl std::fmt::Display,
) -> Option<&'a [T]> {
    if len == 0 {
        return Some(&[]);
    }
    if ptr.is_null() {
        set_last_error(&format!("{name} must not be NULL"));
        return None;
    }
    // SAFETY: ptr is non-null and the caller guarantees it is valid for len elements
    Some(unsafe { std::slice::from_raw_parts(ptr, len) })
}

/// Borrow parallel row-pointer and row-length arrays as a list of slices.
///
/// The outer arrays and every row go through `slice_or_empty`, so `count == 0`
/// yields an empty list and a row with length 0 yields an empty row whatever
/// its pointer is. Stops at the first NULL with a non-zero length; the error
/// names the parameter (`rows_name`, `lens_name`, or `rows_name[i]`).
///
/// # Safety
///
/// If `count > 0`, `rows` and `lens` must be valid for `count` elements, and
/// each non-null `rows[i]` must be valid for `lens[i]` elements of `T`.
unsafe fn rows_or_empty<'a, T>(
    rows: *const *const T,
    lens: *const usize,
    count: usize,
    rows_name: &str,
    lens_name: &str,
) -> Option<Vec<&'a [T]>> {
    // SAFETY: caller guarantees rows and lens are valid for count elements
    let row_ptrs = unsafe { slice_or_empty(rows, count, rows_name) }?;
    let row_lens = unsafe { slice_or_empty(lens, count, lens_name) }?;
    row_ptrs
        .iter()
        .zip(row_lens)
        .enumerate()
        .map(|(i, (&ptr, &len))| {
            // SAFETY: caller guarantees each non-null row is valid for its length.
            // `format_args!` builds the indexed name only if the row is rejected
            unsafe { slice_or_empty(ptr, len, format_args!("{rows_name}[{i}]")) }
        })
        .collect()
}

/// Convert an optional `*const c_char` to `Option<&str>`.
///
/// Returns `Some(None)` for null pointers, `Some(Some(s))` for valid strings,
/// and `None` on UTF-8 conversion failure.
///
/// # Safety
///
/// If non-null, the pointer must be valid and point to a null-terminated
/// UTF-8 string.
unsafe fn ptr_to_optional_str<'a>(ptr: *const c_char, param_name: &str) -> Option<Option<&'a str>> {
    if ptr.is_null() {
        return Some(None);
    }
    // SAFETY: caller guarantees non-null ptr is valid and null-terminated
    match unsafe { CStr::from_ptr(ptr) }.to_str() {
        Ok(s) => Some(Some(s)),
        Err(e) => {
            set_last_error(&format!("{param_name} is not valid UTF-8: {e}"));
            None
        }
    }
}

// ── Gen functions ───────────────────────────────────────────────────────────

/// Generate a Meta-Code from name and optional metadata.
///
/// # Parameters
///
/// - `name`: required, null-terminated UTF-8 string
/// - `description`: optional (NULL means not provided)
/// - `meta`: optional (NULL means not provided)
/// - `bits`: hash bit length (typically 64)
///
/// # Returns
///
/// Heap-allocated ISCC string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// All non-null pointer arguments must point to valid null-terminated UTF-8 strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_meta_code_v0(
    name: *const c_char,
    description: *const c_char,
    meta: *const c_char,
    bits: u32,
) -> *mut c_char {
    clear_last_error();
    let Some(name) = (unsafe { ptr_to_str(name, "name") }) else {
        return ptr::null_mut();
    };
    let Some(description) = (unsafe { ptr_to_optional_str(description, "description") }) else {
        return ptr::null_mut();
    };
    let Some(meta) = (unsafe { ptr_to_optional_str(meta, "meta") }) else {
        return ptr::null_mut();
    };
    result_to_c_string(iscc_lib::gen_meta_code_v0(name, description, meta, bits).map(|r| r.iscc))
}

/// Generate a Text-Code from plain text content.
///
/// # Parameters
///
/// - `text`: required, null-terminated UTF-8 string
/// - `bits`: hash bit length (typically 64)
///
/// # Returns
///
/// Heap-allocated ISCC string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// `text` must point to a valid null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_text_code_v0(text: *const c_char, bits: u32) -> *mut c_char {
    clear_last_error();
    let Some(text) = (unsafe { ptr_to_str(text, "text") }) else {
        return ptr::null_mut();
    };
    result_to_c_string(iscc_lib::gen_text_code_v0(text, bits).map(|r| r.iscc))
}

/// Generate an Image-Code from pixel data.
///
/// # Parameters
///
/// - `pixels`: pointer to 1024 grayscale pixel values (32x32)
/// - `pixels_len`: number of bytes in the pixel buffer
/// - `bits`: hash bit length (typically 64)
///
/// # Returns
///
/// Heap-allocated ISCC string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// `pixels` must point to a valid buffer of at least `pixels_len` bytes. It may
/// be NULL when `pixels_len` is 0; NULL with `pixels_len > 0` is an error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_image_code_v0(
    pixels: *const u8,
    pixels_len: usize,
    bits: u32,
) -> *mut c_char {
    clear_last_error();
    // SAFETY: caller guarantees pixels is valid for pixels_len bytes
    let Some(pixels) = (unsafe { slice_or_empty(pixels, pixels_len, "pixels") }) else {
        return ptr::null_mut();
    };
    result_to_c_string(iscc_lib::gen_image_code_v0(pixels, bits).map(|r| r.iscc))
}

/// Generate an Audio-Code from a Chromaprint feature vector.
///
/// # Parameters
///
/// - `cv`: pointer to signed 32-bit integer Chromaprint features
/// - `cv_len`: number of elements in the feature vector
/// - `bits`: hash bit length (typically 64)
///
/// # Returns
///
/// Heap-allocated ISCC string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// `cv` must point to a valid buffer of at least `cv_len` `i32` elements. It may
/// be NULL when `cv_len` is 0; NULL with `cv_len > 0` is an error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_audio_code_v0(
    cv: *const i32,
    cv_len: usize,
    bits: u32,
) -> *mut c_char {
    clear_last_error();
    // SAFETY: caller guarantees cv is valid for cv_len elements
    let Some(cv) = (unsafe { slice_or_empty(cv, cv_len, "cv") }) else {
        return ptr::null_mut();
    };
    result_to_c_string(iscc_lib::gen_audio_code_v0(cv, bits).map(|r| r.iscc))
}

/// Generate a Video-Code from frame signature data.
///
/// # Parameters
///
/// - `frame_sigs`: array of pointers to frame signature arrays
/// - `frame_lens`: array of lengths for each frame signature
/// - `num_frames`: number of frames
/// - `bits`: hash bit length (typically 64)
///
/// # Returns
///
/// Heap-allocated ISCC string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// - `frame_sigs` must point to an array of `num_frames` pointers
/// - `frame_lens` must point to an array of `num_frames` lengths
/// - Each `frame_sigs[i]` must be valid for `frame_lens[i]` `i32` elements
/// - Any of these pointers may be NULL when its length (`num_frames` or
///   `frame_lens[i]`) is 0; NULL with a non-zero length is an error
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_video_code_v0(
    frame_sigs: *const *const i32,
    frame_lens: *const usize,
    num_frames: usize,
    bits: u32,
) -> *mut c_char {
    clear_last_error();
    // SAFETY: caller guarantees the arrays and each row are valid for their lengths
    let Some(frames) = (unsafe {
        rows_or_empty(
            frame_sigs,
            frame_lens,
            num_frames,
            "frame_sigs",
            "frame_lens",
        )
    }) else {
        return ptr::null_mut();
    };

    result_to_c_string(iscc_lib::gen_video_code_v0(&frames, bits).map(|r| r.iscc))
}

/// Generate a Mixed-Code from multiple Content-Code strings.
///
/// # Parameters
///
/// - `codes`: array of pointers to null-terminated ISCC code strings
/// - `num_codes`: number of code strings
/// - `bits`: hash bit length (typically 64)
///
/// # Returns
///
/// Heap-allocated ISCC string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// `codes` must point to an array of `num_codes` valid null-terminated UTF-8 strings.
/// It may be NULL when `num_codes` is 0; NULL with `num_codes > 0` is an error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_mixed_code_v0(
    codes: *const *const c_char,
    num_codes: usize,
    bits: u32,
) -> *mut c_char {
    clear_last_error();
    // SAFETY: caller guarantees codes is valid for num_codes elements
    let Some(code_ptrs) = (unsafe { slice_or_empty(codes, num_codes, "codes") }) else {
        return ptr::null_mut();
    };

    let mut code_strs: Vec<&str> = Vec::with_capacity(num_codes);
    for (i, &ptr) in code_ptrs.iter().enumerate() {
        let Some(s) = (unsafe { ptr_to_str(ptr, &format!("codes[{i}]")) }) else {
            return ptr::null_mut();
        };
        code_strs.push(s);
    }

    result_to_c_string(iscc_lib::gen_mixed_code_v0(&code_strs, bits).map(|r| r.iscc))
}

/// Generate a Data-Code from raw byte data.
///
/// # Parameters
///
/// - `data`: pointer to raw byte data
/// - `data_len`: number of bytes
/// - `bits`: hash bit length (typically 64)
///
/// # Returns
///
/// Heap-allocated ISCC string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// `data` must point to a valid buffer of at least `data_len` bytes. It may be
/// NULL when `data_len` is 0; NULL with `data_len > 0` is an error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_data_code_v0(
    data: *const u8,
    data_len: usize,
    bits: u32,
) -> *mut c_char {
    clear_last_error();
    // SAFETY: caller guarantees data is valid for data_len bytes
    let Some(data) = (unsafe { slice_or_empty(data, data_len, "data") }) else {
        return ptr::null_mut();
    };
    result_to_c_string(iscc_lib::gen_data_code_v0(data, bits).map(|r| r.iscc))
}

/// Generate an Instance-Code from raw byte data.
///
/// # Parameters
///
/// - `data`: pointer to raw byte data
/// - `data_len`: number of bytes
/// - `bits`: hash bit length (typically 64)
///
/// # Returns
///
/// Heap-allocated ISCC string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// `data` must point to a valid buffer of at least `data_len` bytes. It may be
/// NULL when `data_len` is 0; NULL with `data_len > 0` is an error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_instance_code_v0(
    data: *const u8,
    data_len: usize,
    bits: u32,
) -> *mut c_char {
    clear_last_error();
    // SAFETY: caller guarantees data is valid for data_len bytes
    let Some(data) = (unsafe { slice_or_empty(data, data_len, "data") }) else {
        return ptr::null_mut();
    };
    result_to_c_string(iscc_lib::gen_instance_code_v0(data, bits).map(|r| r.iscc))
}

/// Generate a composite ISCC-CODE from individual unit codes.
///
/// # Parameters
///
/// - `codes`: array of pointers to null-terminated ISCC unit code strings
/// - `num_codes`: number of code strings
/// - `wide`: if true and exactly 2 codes (Data+Instance) with 128+ bits, produce wide output
///
/// # Returns
///
/// Heap-allocated ISCC string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// `codes` must point to an array of `num_codes` valid null-terminated UTF-8 strings.
/// It may be NULL when `num_codes` is 0; NULL with `num_codes > 0` is an error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_iscc_code_v0(
    codes: *const *const c_char,
    num_codes: usize,
    wide: bool,
) -> *mut c_char {
    clear_last_error();
    // SAFETY: caller guarantees codes is valid for num_codes elements
    let Some(code_ptrs) = (unsafe { slice_or_empty(codes, num_codes, "codes") }) else {
        return ptr::null_mut();
    };

    let mut code_strs: Vec<&str> = Vec::with_capacity(num_codes);
    for (i, &ptr) in code_ptrs.iter().enumerate() {
        let Some(s) = (unsafe { ptr_to_str(ptr, &format!("codes[{i}]")) }) else {
            return ptr::null_mut();
        };
        code_strs.push(s);
    }

    result_to_c_string(iscc_lib::gen_iscc_code_v0(&code_strs, wide).map(|r| r.iscc))
}

/// Generate an experimental ISCC-IDv1 from a timestamp, HUB-ID, and realm.
///
/// # Parameters
///
/// - `timestamp`: microseconds since the Unix epoch (must be `< 2^52`)
/// - `hub_id`: HUB identifier (must be `< 2^12`)
/// - `realm`: realm selector (`0` for testnet, `1` for the first operational mainnet)
///
/// # Returns
///
/// Heap-allocated ISCC string on success, `NULL` on error (out-of-range input).
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// Takes only scalar arguments; there are no pointer preconditions. Declared
/// `unsafe` for consistency with the other `#[unsafe(no_mangle)]` exports.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_iscc_id_v1(
    timestamp: u64,
    hub_id: u16,
    realm: u8,
) -> *mut c_char {
    clear_last_error();
    result_to_c_string(iscc_lib::gen_iscc_id_v1(timestamp, hub_id, realm).map(|r| r.iscc))
}

// ── Text utilities ──────────────────────────────────────────────────────────

/// Clean and normalize text for display.
///
/// Applies NFKC normalization, removes control characters (except newlines),
/// normalizes `\r\n` to `\n`, collapses consecutive empty lines, and strips
/// leading/trailing whitespace.
///
/// # Returns
///
/// Heap-allocated C string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// `text` must point to a valid null-terminated UTF-8 string, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_text_clean(text: *const c_char) -> *mut c_char {
    clear_last_error();
    let Some(text) = (unsafe { ptr_to_str(text, "text") }) else {
        return ptr::null_mut();
    };
    string_to_c(iscc_lib::text_clean(text))
}

/// Remove newlines and collapse whitespace to single spaces.
///
/// Converts multi-line text into a single normalized line.
///
/// # Returns
///
/// Heap-allocated C string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// `text` must point to a valid null-terminated UTF-8 string, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_text_remove_newlines(text: *const c_char) -> *mut c_char {
    clear_last_error();
    let Some(text) = (unsafe { ptr_to_str(text, "text") }) else {
        return ptr::null_mut();
    };
    string_to_c(iscc_lib::text_remove_newlines(text))
}

/// Trim text so its UTF-8 encoded size does not exceed `nbytes`.
///
/// Multi-byte characters that would be split are dropped entirely.
/// Leading/trailing whitespace is stripped from the result.
///
/// # Returns
///
/// Heap-allocated C string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// `text` must point to a valid null-terminated UTF-8 string, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_text_trim(text: *const c_char, nbytes: usize) -> *mut c_char {
    clear_last_error();
    let Some(text) = (unsafe { ptr_to_str(text, "text") }) else {
        return ptr::null_mut();
    };
    string_to_c(iscc_lib::text_trim(text, nbytes))
}

/// Normalize and simplify text for similarity hashing.
///
/// Applies NFD normalization, lowercasing, removes whitespace and characters
/// in Unicode categories C (control), M (mark), and P (punctuation), then
/// recombines with NFKC normalization.
///
/// # Returns
///
/// Heap-allocated C string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// `text` must point to a valid null-terminated UTF-8 string, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_text_collapse(text: *const c_char) -> *mut c_char {
    clear_last_error();
    let Some(text) = (unsafe { ptr_to_str(text, "text") }) else {
        return ptr::null_mut();
    };
    string_to_c(iscc_lib::text_collapse(text))
}

// ── Encoding ────────────────────────────────────────────────────────────────

/// Encode bytes as base64url (RFC 4648 section 5, no padding).
///
/// # Parameters
///
/// - `data`: pointer to raw byte data
/// - `data_len`: number of bytes
///
/// # Returns
///
/// Heap-allocated C string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// `data` must point to a valid buffer of at least `data_len` bytes. It may be
/// NULL when `data_len` is 0; NULL with `data_len > 0` is an error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_encode_base64(data: *const u8, data_len: usize) -> *mut c_char {
    clear_last_error();
    // SAFETY: caller guarantees data is valid for data_len bytes
    let Some(data) = (unsafe { slice_or_empty(data, data_len, "data") }) else {
        return ptr::null_mut();
    };
    string_to_c(iscc_lib::encode_base64(data))
}

/// Convert a JSON string into a `data:` URL with JCS canonicalization.
///
/// Uses `application/ld+json` media type when the JSON contains an `@context`
/// key, otherwise `application/json`.
///
/// # Returns
///
/// Heap-allocated C string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// `json` must point to a valid null-terminated UTF-8 string, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_json_to_data_url(json: *const c_char) -> *mut c_char {
    clear_last_error();
    let Some(s) = (unsafe { ptr_to_str(json, "json") }) else {
        return ptr::null_mut();
    };
    result_to_c_string(iscc_lib::json_to_data_url(s))
}

// ── Codec ───────────────────────────────────────────────────────────────────

/// Encode raw ISCC header components and digest into a base32 ISCC unit string.
///
/// Takes integer type identifiers and a raw digest, returns a base32-encoded
/// ISCC unit string (without "ISCC:" prefix).
///
/// # Parameters
///
/// - `mtype`: MainType enum value (0–7)
/// - `stype`: SubType enum value (0–7)
/// - `version`: Version enum value
/// - `bit_length`: bit length of the digest (multiple of 32)
/// - `digest`: pointer to raw digest bytes
/// - `digest_len`: number of bytes in the digest buffer
///
/// # Returns
///
/// Heap-allocated C string on success, `NULL` on error.
/// Caller must free with `iscc_free_string()`.
///
/// # Safety
///
/// `digest` must point to a valid buffer of at least `digest_len` bytes. It may
/// be NULL when `digest_len` is 0; NULL with `digest_len > 0` is an error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_encode_component(
    mtype: u8,
    stype: u8,
    version: u8,
    bit_length: u32,
    digest: *const u8,
    digest_len: usize,
) -> *mut c_char {
    clear_last_error();
    // SAFETY: caller guarantees digest is valid for digest_len bytes
    let Some(slice) = (unsafe { slice_or_empty(digest, digest_len, "digest") }) else {
        return ptr::null_mut();
    };
    result_to_c_string(iscc_lib::encode_component(
        mtype, stype, version, bit_length, slice,
    ))
}

/// Result of `gen_sum_code_v0` — composite Data+Instance ISCC-CODE with file metadata.
///
/// On success, `ok` is `true` and all fields are populated.
/// On error, `ok` is `false`, string pointers are `NULL`, and `filesize` is 0.
/// Caller must free with `iscc_free_sum_code_result()`.
#[repr(C)]
pub struct IsccSumCodeResult {
    /// Whether the operation succeeded.
    pub ok: bool,
    /// Composite ISCC-CODE string (heap-allocated, caller frees via `iscc_free_sum_code_result`).
    pub iscc: *mut c_char,
    /// Hex-encoded BLAKE3 multihash of the file (heap-allocated).
    pub datahash: *mut c_char,
    /// Byte length of the file.
    pub filesize: u64,
    /// NULL-terminated array of Data-Code and Instance-Code ISCC strings, or NULL.
    pub units: *mut *mut c_char,
}

/// Return a zeroed-out error `IsccSumCodeResult`.
fn null_sum_code_result() -> IsccSumCodeResult {
    IsccSumCodeResult {
        ok: false,
        iscc: ptr::null_mut(),
        datahash: ptr::null_mut(),
        filesize: 0,
        units: ptr::null_mut(),
    }
}

/// Generate a composite ISCC-CODE from a file path (Data-Code + Instance-Code).
///
/// Single-pass file I/O feeds both DataHasher and InstanceHasher, then composes
/// the ISCC-CODE internally.
///
/// # Parameters
///
/// - `path`: null-terminated UTF-8 file path
/// - `bits`: hash bit length (typically 64)
/// - `wide`: if true, use 256-bit combination for ISCC-CODE
/// - `add_units`: if true, include individual Data-Code and Instance-Code strings
///
/// # Returns
///
/// `IsccSumCodeResult` struct. Check `ok` to determine success.
/// Caller must free with `iscc_free_sum_code_result()`.
///
/// # Safety
///
/// `path` must point to a valid null-terminated UTF-8 string, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_sum_code_v0(
    path: *const c_char,
    bits: u32,
    wide: bool,
    add_units: bool,
) -> IsccSumCodeResult {
    clear_last_error();
    let Some(path_str) = (unsafe { ptr_to_str(path, "path") }) else {
        return null_sum_code_result();
    };
    match iscc_lib::gen_sum_code_v0(std::path::Path::new(path_str), bits, wide, add_units) {
        Ok(result) => {
            let iscc = string_to_c(result.iscc);
            if iscc.is_null() {
                return null_sum_code_result();
            }
            let datahash = string_to_c(result.datahash);
            if datahash.is_null() {
                unsafe { iscc_free_string(iscc) };
                return null_sum_code_result();
            }
            let units = match result.units {
                Some(v) => {
                    let arr = vec_to_c_string_array(v);
                    if arr.is_null() {
                        unsafe { iscc_free_string(iscc) };
                        unsafe { iscc_free_string(datahash) };
                        return null_sum_code_result();
                    }
                    arr
                }
                None => ptr::null_mut(),
            };
            IsccSumCodeResult {
                ok: true,
                iscc,
                datahash,
                filesize: result.filesize,
                units,
            }
        }
        Err(e) => {
            set_last_error(&e.to_string());
            null_sum_code_result()
        }
    }
}

/// Free an `IsccSumCodeResult` previously returned by `iscc_gen_sum_code_v0`.
///
/// Releases the `iscc`, `datahash` strings, and `units` array. No-op for NULL pointers.
///
/// # Safety
///
/// `result` must be a value returned by `iscc_gen_sum_code_v0`.
/// Each result must only be freed once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_free_sum_code_result(result: IsccSumCodeResult) {
    unsafe { iscc_free_string(result.iscc) };
    unsafe { iscc_free_string(result.datahash) };
    unsafe { iscc_free_string_array(result.units) };
}

/// Result of decoding an ISCC unit string.
///
/// On success, `ok` is `true` and all fields are populated.
/// On error, `ok` is `false` and all fields are zeroed (check `iscc_last_error()`).
/// Caller must free with `iscc_free_decode_result()`.
#[repr(C)]
pub struct IsccDecodeResult {
    /// Whether the decode succeeded.
    pub ok: bool,
    /// MainType enum value (0–7).
    pub maintype: u8,
    /// SubType enum value (0–7).
    pub subtype: u8,
    /// Version enum value.
    pub version: u8,
    /// Length index from the header.
    pub length: u8,
    /// Raw digest bytes truncated to the encoded bit-length.
    pub digest: IsccByteBuffer,
}

/// Decode an ISCC unit string into header components and raw digest.
///
/// Returns an `IsccDecodeResult` struct. Check `ok` to determine success.
/// Strips an optional "ISCC:" prefix before decoding.
///
/// # Returns
///
/// `IsccDecodeResult` with decoded fields. On error, `ok` is `false`.
/// Caller must free with `iscc_free_decode_result()`.
///
/// # Safety
///
/// `iscc` must point to a valid null-terminated UTF-8 string, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_decode(iscc: *const c_char) -> IsccDecodeResult {
    clear_last_error();
    let Some(s) = (unsafe { ptr_to_str(iscc, "iscc") }) else {
        return IsccDecodeResult {
            ok: false,
            maintype: 0,
            subtype: 0,
            version: 0,
            length: 0,
            digest: null_byte_buffer(),
        };
    };
    match iscc_lib::iscc_decode(s) {
        Ok((mt, st, vs, li, digest)) => IsccDecodeResult {
            ok: true,
            maintype: mt,
            subtype: st,
            version: vs,
            length: li,
            digest: vec_to_byte_buffer(digest),
        },
        Err(e) => {
            set_last_error(&e.to_string());
            IsccDecodeResult {
                ok: false,
                maintype: 0,
                subtype: 0,
                version: 0,
                length: 0,
                digest: null_byte_buffer(),
            }
        }
    }
}

/// Free an `IsccDecodeResult` previously returned by `iscc_decode`.
///
/// Releases the digest buffer if non-null. No-op if digest is already null.
///
/// # Safety
///
/// `result` must be a value returned by `iscc_decode`.
/// Each result must only be freed once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_free_decode_result(result: IsccDecodeResult) {
    if !result.digest.data.is_null() {
        unsafe { iscc_free_byte_buffer(result.digest) };
    }
}

// ── Conformance ─────────────────────────────────────────────────────────────

/// Run all conformance tests against vendored test vectors.
///
/// Returns `true` if all tests pass, `false` if any fail.
#[unsafe(no_mangle)]
pub extern "C" fn iscc_conformance_selftest() -> bool {
    iscc_lib::conformance_selftest()
}

/// Decompose a composite ISCC-CODE into individual ISCC-UNITs.
///
/// Accepts a normalized ISCC-CODE or concatenated ISCC-UNIT sequence.
/// The optional "ISCC:" prefix is stripped before decoding.
/// Returns a NULL-terminated array of heap-allocated C strings.
///
/// # Returns
///
/// NULL-terminated array on success, `NULL` on error (check `iscc_last_error()`).
/// Caller must free with `iscc_free_string_array()`.
///
/// # Safety
///
/// `iscc_code` must point to a valid null-terminated UTF-8 string, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_decompose(iscc_code: *const c_char) -> *mut *mut c_char {
    clear_last_error();
    let Some(iscc_code) = (unsafe { ptr_to_str(iscc_code, "iscc_code") }) else {
        return ptr::null_mut();
    };
    match iscc_lib::iscc_decompose(iscc_code) {
        Ok(units) => vec_to_c_string_array(units),
        Err(e) => {
            set_last_error(&e.to_string());
            ptr::null_mut()
        }
    }
}

/// Encode ISCC-UNIT strings as an ISCC-SEQ byte string (IEP-0020).
///
/// Returns the concatenated header and body bytes of each unit, in the given
/// order. Unit strings may omit the `ISCC:` prefix and may be lowercase or
/// contain hyphens. Base32 decoding is strict: no padding, zero spare bits,
/// and ASCII-only case folding.
///
/// # Parameters
///
/// - `units`: array of pointers to null-terminated ISCC-UNIT strings
/// - `num_units`: number of unit strings (must be at least 1)
///
/// # Returns
///
/// `IsccByteBuffer` with the ISCC-SEQ bytes on success. On error `.data` is
/// `NULL` (check `iscc_last_error()`): `units` is NULL with `num_units > 0`,
/// the list is empty, or a unit is NULL, not UTF-8, malformed, or of an
/// excluded MainType (ISCC-CODE, ID, FLAKE). Caller must free with
/// `iscc_free_byte_buffer()`.
///
/// # Safety
///
/// `units` must point to an array of `num_units` valid null-terminated UTF-8
/// strings. It may be NULL when `num_units` is 0.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_encode_seq(
    units: *const *const c_char,
    num_units: usize,
) -> IsccByteBuffer {
    clear_last_error();
    // SAFETY: caller guarantees units is valid for num_units elements
    let Some(unit_ptrs) = (unsafe { slice_or_empty(units, num_units, "units") }) else {
        return null_byte_buffer();
    };

    let mut unit_strs: Vec<&str> = Vec::with_capacity(num_units);
    for (i, &ptr) in unit_ptrs.iter().enumerate() {
        let Some(s) = (unsafe { ptr_to_str(ptr, &format!("units[{i}]")) }) else {
            return null_byte_buffer();
        };
        unit_strs.push(s);
    }

    match iscc_lib::encode_seq(&unit_strs) {
        Ok(bytes) => vec_to_byte_buffer(bytes),
        Err(e) => {
            set_last_error(&e.to_string());
            null_byte_buffer()
        }
    }
}

/// Decode an ISCC-SEQ byte string to canonical ISCC-UNIT strings (IEP-0020).
///
/// Strict byte-level reader: every unit must be well-formed and no trailing
/// bytes are allowed. Returns `ISCC:`-prefixed uppercase unit strings in wire
/// order; `iscc_encode_seq` of the result reproduces `data`.
///
/// # Parameters
///
/// - `data`: pointer to the ISCC-SEQ bytes
/// - `data_len`: number of bytes in `data`
///
/// # Returns
///
/// NULL-terminated array of unit strings on success, `NULL` on error (empty
/// or malformed input; check `iscc_last_error()`).
/// Caller must free with `iscc_free_string_array()`.
///
/// # Safety
///
/// `data` must point to a valid buffer of at least `data_len` bytes. It may be
/// NULL when `data_len` is 0; NULL with `data_len > 0` is an error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_decode_seq(data: *const u8, data_len: usize) -> *mut *mut c_char {
    clear_last_error();
    // SAFETY: caller guarantees data is valid for data_len bytes
    let Some(slice) = (unsafe { slice_or_empty(data, data_len, "data") }) else {
        return ptr::null_mut();
    };
    match iscc_lib::decode_seq(slice) {
        Ok(units) => vec_to_c_string_array(units),
        Err(e) => {
            set_last_error(&e.to_string());
            ptr::null_mut()
        }
    }
}

// ── Sliding window ──────────────────────────────────────────────────────────

/// Generate sliding window n-grams from a string.
///
/// Returns a NULL-terminated array of overlapping substrings of `width`
/// Unicode characters, advancing by one character at a time.
///
/// # Returns
///
/// NULL-terminated array on success, `NULL` on error (check `iscc_last_error()`).
/// Caller must free with `iscc_free_string_array()`.
///
/// # Safety
///
/// `seq` must point to a valid null-terminated UTF-8 string, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_sliding_window(seq: *const c_char, width: u32) -> *mut *mut c_char {
    clear_last_error();
    let Some(seq) = (unsafe { ptr_to_str(seq, "seq") }) else {
        return ptr::null_mut();
    };
    match iscc_lib::sliding_window(seq, width as usize) {
        Ok(v) => vec_to_c_string_array(v),
        Err(e) => {
            set_last_error(&e.to_string());
            ptr::null_mut()
        }
    }
}

// ── Algorithm primitives ─────────────────────────────────────────────────

/// Compute a SimHash digest from an array of byte digests.
///
/// The output length matches the input digest length (e.g., 4-byte digests
/// produce a 4-byte SimHash). An empty list (`num_digests == 0`) is rejected:
/// `.data` is `NULL` and `iscc_last_error()` names the empty list.
///
/// # Parameters
///
/// - `digests`: array of pointers to digest byte arrays
/// - `digest_lens`: array of lengths for each digest
/// - `num_digests`: number of digests
///
/// # Returns
///
/// `IsccByteBuffer` with the SimHash result. On error, `.data` is `NULL`.
/// Caller must free with `iscc_free_byte_buffer()`.
///
/// # Safety
///
/// - `digests` must point to an array of `num_digests` byte pointers
/// - `digest_lens` must point to an array of `num_digests` lengths
/// - Each `digests[i]` must be valid for `digest_lens[i]` bytes
/// - Any of these pointers may be NULL when its length (`num_digests` or
///   `digest_lens[i]`) is 0; NULL with a non-zero length is an error
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_alg_simhash(
    digests: *const *const u8,
    digest_lens: *const usize,
    num_digests: usize,
) -> IsccByteBuffer {
    clear_last_error();
    // SAFETY: caller guarantees the arrays and each row are valid for their lengths
    let Some(slices) =
        (unsafe { rows_or_empty(digests, digest_lens, num_digests, "digests", "digest_lens") })
    else {
        return null_byte_buffer();
    };

    match iscc_lib::alg_simhash(&slices) {
        Ok(v) => vec_to_byte_buffer(v),
        Err(e) => {
            set_last_error(&e.to_string());
            null_byte_buffer()
        }
    }
}

/// Compute a 256-bit MinHash digest from 32-bit integer features.
///
/// Uses 64 universal hash functions with bit-interleaved compression to
/// produce a 32-byte similarity-preserving digest.
///
/// # Parameters
///
/// - `features`: pointer to `u32` feature values
/// - `features_len`: number of features
///
/// # Returns
///
/// `IsccByteBuffer` with 32 bytes. On error, `.data` is `NULL`.
/// Caller must free with `iscc_free_byte_buffer()`.
///
/// # Safety
///
/// `features` must point to a valid buffer of at least `features_len` `u32` elements.
/// It may be NULL when `features_len` is 0; NULL with `features_len > 0` is an error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_alg_minhash_256(
    features: *const u32,
    features_len: usize,
) -> IsccByteBuffer {
    clear_last_error();
    // SAFETY: caller guarantees features is valid for features_len elements
    let Some(slice) = (unsafe { slice_or_empty(features, features_len, "features") }) else {
        return null_byte_buffer();
    };
    vec_to_byte_buffer(iscc_lib::alg_minhash_256(slice))
}

/// Split data into content-defined chunks using gear rolling hash.
///
/// Returns at least one chunk (empty bytes for empty input). When `utf32`
/// is true, aligns cut points to 4-byte boundaries.
///
/// # Parameters
///
/// - `data`: pointer to raw byte data
/// - `data_len`: number of bytes
/// - `utf32`: if true, align cuts to 4-byte boundaries
/// - `avg_chunk_size`: target average chunk size in bytes
///
/// # Returns
///
/// `IsccByteBufferArray` with chunks. On error, `.buffers` is `NULL`.
/// Caller must free with `iscc_free_byte_buffer_array()`.
///
/// # Safety
///
/// `data` must point to a valid buffer of at least `data_len` bytes. It may be
/// NULL when `data_len` is 0; NULL with `data_len > 0` is an error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_alg_cdc_chunks(
    data: *const u8,
    data_len: usize,
    utf32: bool,
    avg_chunk_size: u32,
) -> IsccByteBufferArray {
    clear_last_error();
    // SAFETY: caller guarantees data is valid for data_len bytes
    let Some(slice) = (unsafe { slice_or_empty(data, data_len, "data") }) else {
        return null_byte_buffer_array();
    };
    let chunks = match iscc_lib::alg_cdc_chunks(slice, utf32, avg_chunk_size) {
        Ok(c) => c,
        Err(e) => {
            set_last_error(&e.to_string());
            return null_byte_buffer_array();
        }
    };
    let buffers: Vec<IsccByteBuffer> = chunks
        .iter()
        .map(|chunk| vec_to_byte_buffer(chunk.to_vec()))
        .collect();
    let count = buffers.len();
    let ptr = Box::into_raw(buffers.into_boxed_slice()) as *mut IsccByteBuffer;
    IsccByteBufferArray {
        buffers: ptr,
        count,
    }
}

/// Compute a similarity-preserving hash from video frame signatures.
///
/// Returns raw bytes of length `bits / 8`. Errors if `frame_sigs` is empty.
///
/// # Parameters
///
/// - `frame_sigs`: array of pointers to frame signature arrays (`i32`)
/// - `frame_lens`: array of lengths for each frame signature
/// - `num_frames`: number of frames
/// - `bits`: hash bit length (typically 64)
///
/// # Returns
///
/// `IsccByteBuffer` with the hash result. On error, `.data` is `NULL`.
/// Caller must free with `iscc_free_byte_buffer()`.
///
/// # Safety
///
/// - `frame_sigs` must point to an array of `num_frames` pointers
/// - `frame_lens` must point to an array of `num_frames` lengths
/// - Each `frame_sigs[i]` must be valid for `frame_lens[i]` `i32` elements
/// - Any of these pointers may be NULL when its length (`num_frames` or
///   `frame_lens[i]`) is 0; NULL with a non-zero length is an error
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_soft_hash_video_v0(
    frame_sigs: *const *const i32,
    frame_lens: *const usize,
    num_frames: usize,
    bits: u32,
) -> IsccByteBuffer {
    clear_last_error();
    // SAFETY: caller guarantees the arrays and each row are valid for their lengths
    let Some(frames) = (unsafe {
        rows_or_empty(
            frame_sigs,
            frame_lens,
            num_frames,
            "frame_sigs",
            "frame_lens",
        )
    }) else {
        return null_byte_buffer();
    };

    match iscc_lib::soft_hash_video_v0(&frames, bits) {
        Ok(result) => vec_to_byte_buffer(result),
        Err(e) => {
            set_last_error(&e.to_string());
            null_byte_buffer()
        }
    }
}

// ── Streaming hashers ────────────────────────────────────────────────────────

/// Opaque FFI wrapper around `iscc_lib::DataHasher`.
///
/// Enforces finalize-once semantics via `Option<Inner>`. Not `#[repr(C)]` —
/// C callers interact only through function pointers.
pub struct FfiDataHasher {
    inner: Option<iscc_lib::DataHasher>,
}

/// Opaque FFI wrapper around `iscc_lib::InstanceHasher`.
///
/// Enforces finalize-once semantics via `Option<Inner>`. Not `#[repr(C)]` —
/// C callers interact only through function pointers.
pub struct FfiInstanceHasher {
    inner: Option<iscc_lib::InstanceHasher>,
}

/// Create a new streaming Data-Code hasher.
///
/// Returns an opaque pointer. The caller must eventually call
/// `iscc_data_hasher_free()` to release the memory.
#[unsafe(no_mangle)]
pub extern "C" fn iscc_data_hasher_new() -> *mut FfiDataHasher {
    clear_last_error();
    Box::into_raw(Box::new(FfiDataHasher {
        inner: Some(iscc_lib::DataHasher::new()),
    }))
}

/// Push data into a streaming DataHasher.
///
/// Returns `true` on success, `false` on error (e.g., already finalized
/// or NULL pointer). Check `iscc_last_error()` for the error message.
///
/// # Safety
///
/// - `hasher` must be a valid pointer from `iscc_data_hasher_new()`, or NULL.
/// - `data` must point to at least `data_len` valid bytes. It may be NULL when
///   `data_len` is 0; NULL with `data_len > 0` is an error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_data_hasher_update(
    hasher: *mut FfiDataHasher,
    data: *const u8,
    data_len: usize,
) -> bool {
    clear_last_error();
    if hasher.is_null() {
        set_last_error("hasher must not be NULL");
        return false;
    }
    // SAFETY: caller guarantees hasher is a valid pointer from iscc_data_hasher_new()
    let wrapper = unsafe { &mut *hasher };
    let Some(inner) = wrapper.inner.as_mut() else {
        set_last_error("DataHasher already finalized");
        return false;
    };
    // SAFETY: caller guarantees data is valid for data_len bytes
    let Some(slice) = (unsafe { slice_or_empty(data, data_len, "data") }) else {
        return false;
    };
    inner.update(slice);
    true
}

/// Finalize a streaming DataHasher and return an ISCC string.
///
/// Consumes the inner hasher state. After this call, subsequent `update`
/// or `finalize` calls will fail. The caller must still call
/// `iscc_data_hasher_free()` to release the wrapper, and `iscc_free_string()`
/// to release the returned string.
///
/// # Safety
///
/// `hasher` must be a valid pointer from `iscc_data_hasher_new()`, or NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_data_hasher_finalize(
    hasher: *mut FfiDataHasher,
    bits: u32,
) -> *mut c_char {
    clear_last_error();
    if hasher.is_null() {
        set_last_error("hasher must not be NULL");
        return ptr::null_mut();
    }
    // SAFETY: caller guarantees hasher is a valid pointer from iscc_data_hasher_new()
    let wrapper = unsafe { &mut *hasher };
    let Some(inner) = wrapper.inner.take() else {
        set_last_error("DataHasher already finalized");
        return ptr::null_mut();
    };
    result_to_c_string(inner.finalize(bits).map(|r| r.iscc))
}

/// Free a DataHasher previously created by `iscc_data_hasher_new()`.
///
/// NULL is a no-op. Each pointer must be freed exactly once.
///
/// # Safety
///
/// `hasher` must be a valid pointer from `iscc_data_hasher_new()`, or NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_data_hasher_free(hasher: *mut FfiDataHasher) {
    clear_last_error();
    if !hasher.is_null() {
        // SAFETY: hasher was produced by Box::into_raw() in iscc_data_hasher_new()
        drop(unsafe { Box::from_raw(hasher) });
    }
}

/// Create a new streaming Instance-Code hasher.
///
/// Returns an opaque pointer. The caller must eventually call
/// `iscc_instance_hasher_free()` to release the memory.
#[unsafe(no_mangle)]
pub extern "C" fn iscc_instance_hasher_new() -> *mut FfiInstanceHasher {
    clear_last_error();
    Box::into_raw(Box::new(FfiInstanceHasher {
        inner: Some(iscc_lib::InstanceHasher::new()),
    }))
}

/// Push data into a streaming InstanceHasher.
///
/// Returns `true` on success, `false` on error (e.g., already finalized
/// or NULL pointer). Check `iscc_last_error()` for the error message.
///
/// # Safety
///
/// - `hasher` must be a valid pointer from `iscc_instance_hasher_new()`, or NULL.
/// - `data` must point to at least `data_len` valid bytes. It may be NULL when
///   `data_len` is 0; NULL with `data_len > 0` is an error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_instance_hasher_update(
    hasher: *mut FfiInstanceHasher,
    data: *const u8,
    data_len: usize,
) -> bool {
    clear_last_error();
    if hasher.is_null() {
        set_last_error("hasher must not be NULL");
        return false;
    }
    // SAFETY: caller guarantees hasher is a valid pointer from iscc_instance_hasher_new()
    let wrapper = unsafe { &mut *hasher };
    let Some(inner) = wrapper.inner.as_mut() else {
        set_last_error("InstanceHasher already finalized");
        return false;
    };
    // SAFETY: caller guarantees data is valid for data_len bytes
    let Some(slice) = (unsafe { slice_or_empty(data, data_len, "data") }) else {
        return false;
    };
    inner.update(slice);
    true
}

/// Finalize a streaming InstanceHasher and return an ISCC string.
///
/// Consumes the inner hasher state. After this call, subsequent `update`
/// or `finalize` calls will fail. The caller must still call
/// `iscc_instance_hasher_free()` to release the wrapper, and
/// `iscc_free_string()` to release the returned string.
///
/// # Safety
///
/// `hasher` must be a valid pointer from `iscc_instance_hasher_new()`, or NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_instance_hasher_finalize(
    hasher: *mut FfiInstanceHasher,
    bits: u32,
) -> *mut c_char {
    clear_last_error();
    if hasher.is_null() {
        set_last_error("hasher must not be NULL");
        return ptr::null_mut();
    }
    // SAFETY: caller guarantees hasher is a valid pointer from iscc_instance_hasher_new()
    let wrapper = unsafe { &mut *hasher };
    let Some(inner) = wrapper.inner.take() else {
        set_last_error("InstanceHasher already finalized");
        return ptr::null_mut();
    };
    result_to_c_string(inner.finalize(bits).map(|r| r.iscc))
}

/// Free an InstanceHasher previously created by `iscc_instance_hasher_new()`.
///
/// NULL is a no-op. Each pointer must be freed exactly once.
///
/// # Safety
///
/// `hasher` must be a valid pointer from `iscc_instance_hasher_new()`, or NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_instance_hasher_free(hasher: *mut FfiInstanceHasher) {
    clear_last_error();
    if !hasher.is_null() {
        // SAFETY: hasher was produced by Box::into_raw() in iscc_instance_hasher_new()
        drop(unsafe { Box::from_raw(hasher) });
    }
}

// ── Memory management ───────────────────────────────────────────────────────

/// Free a string previously returned by any `iscc_gen_*` function.
///
/// # Safety
///
/// `ptr` must be a pointer returned by one of the `iscc_gen_*_v0` functions,
/// or `NULL` (which is a no-op). Each pointer must only be freed once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        // SAFETY: ptr was produced by CString::into_raw() in this crate
        drop(unsafe { CString::from_raw(ptr) });
    }
}

/// Free a NULL-terminated string array returned by `iscc_decompose`,
/// `iscc_decode_seq`, or `iscc_sliding_window`.
///
/// Walks the array, freeing each string via `CString::from_raw`, then frees
/// the array itself. `NULL` is a no-op.
///
/// # Safety
///
/// `arr` must be a pointer returned by `iscc_decompose`, `iscc_decode_seq`, or
/// `iscc_sliding_window`, or `NULL`. Each array must only be freed once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_free_string_array(arr: *mut *mut c_char) {
    if arr.is_null() {
        return;
    }
    // Count elements (excluding NULL terminator)
    let mut count = 0usize;
    // SAFETY: arr was produced by vec_to_c_string_array() and is NULL-terminated
    while !unsafe { *arr.add(count) }.is_null() {
        // SAFETY: each element was produced by CString::into_raw()
        drop(unsafe { CString::from_raw(*arr.add(count)) });
        count += 1;
    }
    // Reconstruct the boxed slice to free the array itself.
    // SAFETY: arr was produced by Box::into_raw() of a boxed slice with count + 1 elements
    drop(unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(arr, count + 1)) });
}

/// Free a byte buffer returned by `iscc_alg_simhash`, `iscc_alg_minhash_256`,
/// `iscc_soft_hash_video_v0`, or `iscc_encode_seq`.
///
/// No-op if `buf.data` is `NULL`.
///
/// # Safety
///
/// `buf` must be a value returned by one of the functions listed above.
/// Each buffer must only be freed once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_free_byte_buffer(buf: IsccByteBuffer) {
    if !buf.data.is_null() {
        // SAFETY: buf.data was produced by Box::into_raw() of a boxed slice with buf.len elements
        drop(unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(buf.data, buf.len)) });
    }
}

/// Free a byte buffer array returned by `iscc_alg_cdc_chunks`.
///
/// Frees each buffer's data, then the array itself. No-op if `arr.buffers`
/// is `NULL`.
///
/// # Safety
///
/// `arr` must be a value returned by `iscc_alg_cdc_chunks`.
/// Each array must only be freed once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_free_byte_buffer_array(arr: IsccByteBufferArray) {
    if arr.buffers.is_null() {
        return;
    }
    // SAFETY: arr.buffers was produced by Box::into_raw() of a boxed slice with arr.count elements
    let buffers =
        unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(arr.buffers, arr.count)) };
    for buf in buffers.iter() {
        if !buf.data.is_null() {
            // SAFETY: each buf.data was produced by vec_to_byte_buffer (Box::into_raw of boxed slice)
            drop(unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(buf.data, buf.len)) });
        }
    }
}

/// Return the last error message from the current thread.
///
/// Returns a pointer to a null-terminated string valid until the next
/// `iscc_gen_*` call on the same thread. Returns `NULL` if no error
/// has occurred.
///
/// The returned pointer must NOT be freed by the caller.
#[unsafe(no_mangle)]
pub extern "C" fn iscc_last_error() -> *const c_char {
    LAST_ERROR.with(|e| {
        let borrow = e.borrow();
        match borrow.as_ref() {
            Some(cs) => cs.as_ptr(),
            None => ptr::null(),
        }
    })
}

#[cfg(test)]
mod tests;
