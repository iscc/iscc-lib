//! C FFI bindings for iscc-lib.
//!
//! Exposes all 9 `gen_*_v0` functions, 4 algorithm primitives, codec
//! functions (`encode_component`, `iscc_decode`, `json_to_data_url`),
//! and 5 algorithm constants as `extern "C"` symbols for integration
//! from C, Go, Java, C#, and any other language with C interop.
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
use std::{mem, ptr};

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
    ptrs.shrink_to_fit();
    let ptr = ptrs.as_mut_ptr();
    mem::forget(ptrs);
    ptr
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
/// Uses `shrink_to_fit` + `as_mut_ptr` + `mem::forget` so the caller
/// can reconstruct with `Vec::from_raw_parts(data, len, len)`.
fn vec_to_byte_buffer(mut v: Vec<u8>) -> IsccByteBuffer {
    v.shrink_to_fit();
    let len = v.len();
    let data = v.as_mut_ptr();
    mem::forget(v);
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
/// `pixels` must point to a valid buffer of at least `pixels_len` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_image_code_v0(
    pixels: *const u8,
    pixels_len: usize,
    bits: u32,
) -> *mut c_char {
    clear_last_error();
    if pixels.is_null() {
        set_last_error("pixels must not be NULL");
        return ptr::null_mut();
    }
    // SAFETY: caller guarantees pixels is valid for pixels_len bytes
    let pixels = unsafe { std::slice::from_raw_parts(pixels, pixels_len) };
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
/// `cv` must point to a valid buffer of at least `cv_len` `i32` elements.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_audio_code_v0(
    cv: *const i32,
    cv_len: usize,
    bits: u32,
) -> *mut c_char {
    clear_last_error();
    if cv.is_null() {
        set_last_error("cv must not be NULL");
        return ptr::null_mut();
    }
    // SAFETY: caller guarantees cv is valid for cv_len elements
    let cv = unsafe { std::slice::from_raw_parts(cv, cv_len) };
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
/// - `frame_sigs` must point to an array of `num_frames` valid pointers
/// - `frame_lens` must point to an array of `num_frames` lengths
/// - Each `frame_sigs[i]` must be valid for `frame_lens[i]` `i32` elements
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_video_code_v0(
    frame_sigs: *const *const i32,
    frame_lens: *const usize,
    num_frames: usize,
    bits: u32,
) -> *mut c_char {
    clear_last_error();
    if frame_sigs.is_null() {
        set_last_error("frame_sigs must not be NULL");
        return ptr::null_mut();
    }
    if frame_lens.is_null() {
        set_last_error("frame_lens must not be NULL");
        return ptr::null_mut();
    }
    // SAFETY: caller guarantees pointers are valid for num_frames elements
    let sig_ptrs = unsafe { std::slice::from_raw_parts(frame_sigs, num_frames) };
    let lens = unsafe { std::slice::from_raw_parts(frame_lens, num_frames) };

    let frames: Vec<&[i32]> = sig_ptrs
        .iter()
        .zip(lens.iter())
        .map(|(&ptr, &len)| {
            // SAFETY: caller guarantees each ptr is valid for its length
            unsafe { std::slice::from_raw_parts(ptr, len) }
        })
        .collect();

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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_mixed_code_v0(
    codes: *const *const c_char,
    num_codes: usize,
    bits: u32,
) -> *mut c_char {
    clear_last_error();
    if codes.is_null() {
        set_last_error("codes must not be NULL");
        return ptr::null_mut();
    }
    // SAFETY: caller guarantees codes is valid for num_codes elements
    let code_ptrs = unsafe { std::slice::from_raw_parts(codes, num_codes) };

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
/// `data` must point to a valid buffer of at least `data_len` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_data_code_v0(
    data: *const u8,
    data_len: usize,
    bits: u32,
) -> *mut c_char {
    clear_last_error();
    if data.is_null() {
        set_last_error("data must not be NULL");
        return ptr::null_mut();
    }
    // SAFETY: caller guarantees data is valid for data_len bytes
    let data = unsafe { std::slice::from_raw_parts(data, data_len) };
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
/// `data` must point to a valid buffer of at least `data_len` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_instance_code_v0(
    data: *const u8,
    data_len: usize,
    bits: u32,
) -> *mut c_char {
    clear_last_error();
    if data.is_null() {
        set_last_error("data must not be NULL");
        return ptr::null_mut();
    }
    // SAFETY: caller guarantees data is valid for data_len bytes
    let data = unsafe { std::slice::from_raw_parts(data, data_len) };
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_iscc_code_v0(
    codes: *const *const c_char,
    num_codes: usize,
    wide: bool,
) -> *mut c_char {
    clear_last_error();
    if codes.is_null() {
        set_last_error("codes must not be NULL");
        return ptr::null_mut();
    }
    // SAFETY: caller guarantees codes is valid for num_codes elements
    let code_ptrs = unsafe { std::slice::from_raw_parts(codes, num_codes) };

    let mut code_strs: Vec<&str> = Vec::with_capacity(num_codes);
    for (i, &ptr) in code_ptrs.iter().enumerate() {
        let Some(s) = (unsafe { ptr_to_str(ptr, &format!("codes[{i}]")) }) else {
            return ptr::null_mut();
        };
        code_strs.push(s);
    }

    result_to_c_string(iscc_lib::gen_iscc_code_v0(&code_strs, wide).map(|r| r.iscc))
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
/// `data` must point to a valid buffer of at least `data_len` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_encode_base64(data: *const u8, data_len: usize) -> *mut c_char {
    clear_last_error();
    if data.is_null() {
        set_last_error("data must not be NULL");
        return ptr::null_mut();
    }
    // SAFETY: caller guarantees data is valid for data_len bytes
    let data = unsafe { std::slice::from_raw_parts(data, data_len) };
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
/// If `digest_len > 0`, `digest` must point to a valid buffer of at least
/// `digest_len` bytes.
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
    let slice = if digest_len == 0 {
        &[]
    } else if digest.is_null() {
        set_last_error("digest must not be NULL");
        return ptr::null_mut();
    } else {
        // SAFETY: caller guarantees digest is valid for digest_len bytes
        unsafe { std::slice::from_raw_parts(digest, digest_len) }
    };
    result_to_c_string(iscc_lib::encode_component(
        mtype, stype, version, bit_length, slice,
    ))
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
/// produce a 4-byte SimHash). Returns an empty 32-byte buffer for empty input.
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
/// - `digests` must point to an array of `num_digests` valid byte pointers
/// - `digest_lens` must point to an array of `num_digests` lengths
/// - Each `digests[i]` must be valid for `digest_lens[i]` bytes
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_alg_simhash(
    digests: *const *const u8,
    digest_lens: *const usize,
    num_digests: usize,
) -> IsccByteBuffer {
    clear_last_error();
    if num_digests == 0 {
        let empty: &[&[u8]] = &[];
        return match iscc_lib::alg_simhash(empty) {
            Ok(v) => vec_to_byte_buffer(v),
            Err(e) => {
                set_last_error(&e.to_string());
                null_byte_buffer()
            }
        };
    }
    if digests.is_null() {
        set_last_error("digests must not be NULL");
        return null_byte_buffer();
    }
    if digest_lens.is_null() {
        set_last_error("digest_lens must not be NULL");
        return null_byte_buffer();
    }
    // SAFETY: caller guarantees pointers are valid for num_digests elements
    let ptrs = unsafe { std::slice::from_raw_parts(digests, num_digests) };
    let lens = unsafe { std::slice::from_raw_parts(digest_lens, num_digests) };

    let slices: Vec<&[u8]> = ptrs
        .iter()
        .zip(lens.iter())
        .map(|(&ptr, &len)| {
            // SAFETY: caller guarantees each ptr is valid for its length
            unsafe { std::slice::from_raw_parts(ptr, len) }
        })
        .collect();

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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_alg_minhash_256(
    features: *const u32,
    features_len: usize,
) -> IsccByteBuffer {
    clear_last_error();
    let slice = if features_len == 0 {
        &[]
    } else {
        if features.is_null() {
            set_last_error("features must not be NULL");
            return null_byte_buffer();
        }
        // SAFETY: caller guarantees features is valid for features_len elements
        unsafe { std::slice::from_raw_parts(features, features_len) }
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
/// `data` must point to a valid buffer of at least `data_len` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_alg_cdc_chunks(
    data: *const u8,
    data_len: usize,
    utf32: bool,
    avg_chunk_size: u32,
) -> IsccByteBufferArray {
    clear_last_error();
    let slice = if data_len == 0 {
        &[]
    } else {
        if data.is_null() {
            set_last_error("data must not be NULL");
            return null_byte_buffer_array();
        }
        // SAFETY: caller guarantees data is valid for data_len bytes
        unsafe { std::slice::from_raw_parts(data, data_len) }
    };
    let chunks = iscc_lib::alg_cdc_chunks(slice, utf32, avg_chunk_size);
    let mut buffers: Vec<IsccByteBuffer> = chunks
        .iter()
        .map(|chunk| vec_to_byte_buffer(chunk.to_vec()))
        .collect();
    buffers.shrink_to_fit();
    let count = buffers.len();
    let ptr = buffers.as_mut_ptr();
    mem::forget(buffers);
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
/// - `frame_sigs` must point to an array of `num_frames` valid pointers
/// - `frame_lens` must point to an array of `num_frames` lengths
/// - Each `frame_sigs[i]` must be valid for `frame_lens[i]` `i32` elements
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_soft_hash_video_v0(
    frame_sigs: *const *const i32,
    frame_lens: *const usize,
    num_frames: usize,
    bits: u32,
) -> IsccByteBuffer {
    clear_last_error();
    if frame_sigs.is_null() {
        set_last_error("frame_sigs must not be NULL");
        return null_byte_buffer();
    }
    if frame_lens.is_null() {
        set_last_error("frame_lens must not be NULL");
        return null_byte_buffer();
    }
    // SAFETY: caller guarantees pointers are valid for num_frames elements
    let sig_ptrs = unsafe { std::slice::from_raw_parts(frame_sigs, num_frames) };
    let lens = unsafe { std::slice::from_raw_parts(frame_lens, num_frames) };

    let frames: Vec<&[i32]> = sig_ptrs
        .iter()
        .zip(lens.iter())
        .map(|(&ptr, &len)| {
            // SAFETY: caller guarantees each ptr is valid for its length
            unsafe { std::slice::from_raw_parts(ptr, len) }
        })
        .collect();

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
/// - `data` must point to at least `data_len` valid bytes.
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
    let slice = if data_len == 0 {
        &[]
    } else {
        if data.is_null() {
            set_last_error("data must not be NULL");
            return false;
        }
        unsafe { std::slice::from_raw_parts(data, data_len) }
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
/// - `data` must point to at least `data_len` valid bytes.
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
    let slice = if data_len == 0 {
        &[]
    } else {
        if data.is_null() {
            set_last_error("data must not be NULL");
            return false;
        }
        unsafe { std::slice::from_raw_parts(data, data_len) }
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

/// Free a NULL-terminated string array returned by `iscc_decompose` or
/// `iscc_sliding_window`.
///
/// Walks the array, freeing each string via `CString::from_raw`, then frees
/// the array itself. `NULL` is a no-op.
///
/// # Safety
///
/// `arr` must be a pointer returned by `iscc_decompose` or `iscc_sliding_window`,
/// or `NULL`. Each array must only be freed once.
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
    // Reconstruct the Vec to free the array itself.
    // shrink_to_fit guarantees capacity == len, and len == count + 1 (including NULL terminator)
    // SAFETY: arr was produced by Vec::as_mut_ptr() after shrink_to_fit + mem::forget
    drop(unsafe { Vec::from_raw_parts(arr, count + 1, count + 1) });
}

/// Free a byte buffer returned by `iscc_alg_simhash`, `iscc_alg_minhash_256`,
/// or `iscc_soft_hash_video_v0`.
///
/// No-op if `buf.data` is `NULL`.
///
/// # Safety
///
/// `buf` must be a value returned by one of the algorithm primitive functions.
/// Each buffer must only be freed once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_free_byte_buffer(buf: IsccByteBuffer) {
    if !buf.data.is_null() {
        // SAFETY: buf.data was produced by Vec::as_mut_ptr() after shrink_to_fit + mem::forget.
        // Capacity equals len because of shrink_to_fit.
        drop(unsafe { Vec::from_raw_parts(buf.data, buf.len, buf.len) });
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
    // SAFETY: arr.buffers was produced by Vec::as_mut_ptr() after shrink_to_fit + mem::forget
    let buffers = unsafe { Vec::from_raw_parts(arr.buffers, arr.count, arr.count) };
    for buf in buffers {
        if !buf.data.is_null() {
            // SAFETY: each buf.data was produced by vec_to_byte_buffer
            drop(unsafe { Vec::from_raw_parts(buf.data, buf.len, buf.len) });
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
mod tests {
    use super::*;
    use std::ffi::CString;

    /// Helper to call a gen function and convert the result back to a Rust String.
    unsafe fn c_ptr_to_string(ptr: *mut c_char) -> Option<String> {
        if ptr.is_null() {
            return None;
        }
        // SAFETY: ptr was produced by CString::into_raw() in this crate
        let s = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_owned();
        unsafe { iscc_free_string(ptr) };
        Some(s)
    }

    /// Walk a NULL-terminated C string array and collect into a Vec<String>.
    unsafe fn c_ptr_to_string_vec(arr: *mut *mut c_char) -> Option<Vec<String>> {
        if arr.is_null() {
            return None;
        }
        let mut result = Vec::new();
        let mut i = 0;
        while !unsafe { *arr.add(i) }.is_null() {
            let s = unsafe { CStr::from_ptr(*arr.add(i)) }
                .to_str()
                .unwrap()
                .to_owned();
            result.push(s);
            i += 1;
        }
        unsafe { iscc_free_string_array(arr) };
        Some(result)
    }

    #[test]
    fn test_gen_meta_code_v0_basic() {
        let name = CString::new("Die Unendliche Geschichte").unwrap();
        let result = unsafe { iscc_gen_meta_code_v0(name.as_ptr(), ptr::null(), ptr::null(), 64) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert_eq!(s, "ISCC:AAAZXZ6OU74YAZIM");
    }

    #[test]
    fn test_gen_meta_code_v0_with_description() {
        let name = CString::new("Die Unendliche Geschichte").unwrap();
        let desc = CString::new("Von Michael Ende").unwrap();
        let result =
            unsafe { iscc_gen_meta_code_v0(name.as_ptr(), desc.as_ptr(), ptr::null(), 64) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert_eq!(s, "ISCC:AAAZXZ6OU4E45RB5");
    }

    #[test]
    fn test_gen_meta_code_v0_null_name() {
        let result = unsafe { iscc_gen_meta_code_v0(ptr::null(), ptr::null(), ptr::null(), 64) };
        assert!(result.is_null());
        let err = iscc_last_error();
        assert!(!err.is_null());
        let msg = unsafe { CStr::from_ptr(err) }.to_str().unwrap();
        assert!(msg.contains("name"));
    }

    #[test]
    fn test_gen_text_code_v0_basic() {
        let text = CString::new("Hello World").unwrap();
        let result = unsafe { iscc_gen_text_code_v0(text.as_ptr(), 64) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert_eq!(s, "ISCC:EAASKDNZNYGUUF5A");
    }

    #[test]
    fn test_gen_text_code_v0_null() {
        let result = unsafe { iscc_gen_text_code_v0(ptr::null(), 64) };
        assert!(result.is_null());
    }

    #[test]
    fn test_gen_image_code_v0_basic() {
        let pixels = vec![0u8; 1024];
        let result = unsafe { iscc_gen_image_code_v0(pixels.as_ptr(), pixels.len(), 64) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert_eq!(s, "ISCC:EEAQAAAAAAAAAAAA");
    }

    #[test]
    fn test_gen_image_code_v0_null() {
        let result = unsafe { iscc_gen_image_code_v0(ptr::null(), 0, 64) };
        assert!(result.is_null());
    }

    #[test]
    fn test_gen_audio_code_v0_basic() {
        let cv: Vec<i32> = vec![-1, 0, 1];
        let result = unsafe { iscc_gen_audio_code_v0(cv.as_ptr(), cv.len(), 256) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert_eq!(
            s,
            "ISCC:EIDQAAAAAH777777AAAAAAAAAAAACAAAAAAP777774AAAAAAAAAAAAI"
        );
    }

    #[test]
    fn test_gen_audio_code_v0_null() {
        let result = unsafe { iscc_gen_audio_code_v0(ptr::null(), 0, 64) };
        assert!(result.is_null());
    }

    #[test]
    fn test_gen_video_code_v0_null_sigs() {
        let result = unsafe { iscc_gen_video_code_v0(ptr::null(), ptr::null(), 0, 64) };
        assert!(result.is_null());
    }

    #[test]
    fn test_gen_mixed_code_v0_null() {
        let result = unsafe { iscc_gen_mixed_code_v0(ptr::null(), 0, 64) };
        assert!(result.is_null());
    }

    #[test]
    fn test_gen_data_code_v0_basic() {
        let data = b"Hello World";
        let result = unsafe { iscc_gen_data_code_v0(data.as_ptr(), data.len(), 64) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert!(!s.is_empty());
        assert!(s.starts_with("ISCC:"));
    }

    #[test]
    fn test_gen_data_code_v0_null() {
        let result = unsafe { iscc_gen_data_code_v0(ptr::null(), 0, 64) };
        assert!(result.is_null());
    }

    #[test]
    fn test_gen_instance_code_v0_basic() {
        let data = b"";
        let result = unsafe { iscc_gen_instance_code_v0(data.as_ptr(), data.len(), 64) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert_eq!(s, "ISCC:IAA26E2JXH27TING");
    }

    #[test]
    fn test_gen_instance_code_v0_null() {
        let result = unsafe { iscc_gen_instance_code_v0(ptr::null(), 0, 64) };
        assert!(result.is_null());
    }

    #[test]
    fn test_gen_iscc_code_v0_null() {
        let result = unsafe { iscc_gen_iscc_code_v0(ptr::null(), 0, false) };
        assert!(result.is_null());
    }

    #[test]
    fn test_free_string_null() {
        // Should be a no-op, not crash
        unsafe { iscc_free_string(ptr::null_mut()) };
    }

    #[test]
    fn test_last_error_no_error() {
        clear_last_error();
        let err = iscc_last_error();
        assert!(err.is_null());
    }

    #[test]
    fn test_last_error_after_error() {
        set_last_error("test error message");
        let err = iscc_last_error();
        assert!(!err.is_null());
        let msg = unsafe { CStr::from_ptr(err) }.to_str().unwrap();
        assert_eq!(msg, "test error message");
    }

    #[test]
    fn test_error_cleared_on_success() {
        // Trigger an error first
        let result = unsafe { iscc_gen_text_code_v0(ptr::null(), 64) };
        assert!(result.is_null());
        assert!(!iscc_last_error().is_null());

        // Successful call should clear the error
        let text = CString::new("Hello World").unwrap();
        let result = unsafe { iscc_gen_text_code_v0(text.as_ptr(), 64) };
        assert!(!result.is_null());
        assert!(iscc_last_error().is_null());
        unsafe { iscc_free_string(result) };
    }

    // ── text_clean tests ────────────────────────────────────────────────────

    #[test]
    fn test_text_clean_nfkc() {
        // NFKC normalizes fi ligature (U+FB01) to "fi"
        let text = CString::new("Hel\u{FB01}").unwrap();
        let result = unsafe { iscc_text_clean(text.as_ptr()) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert_eq!(s, "Helfi");
    }

    #[test]
    fn test_text_clean_null() {
        let result = unsafe { iscc_text_clean(ptr::null()) };
        assert!(result.is_null());
    }

    // ── text_remove_newlines tests ──────────────────────────────────────────

    #[test]
    fn test_text_remove_newlines_basic() {
        let text = CString::new("line1\nline2\nline3").unwrap();
        let result = unsafe { iscc_text_remove_newlines(text.as_ptr()) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert_eq!(s, "line1 line2 line3");
    }

    #[test]
    fn test_text_remove_newlines_null() {
        let result = unsafe { iscc_text_remove_newlines(ptr::null()) };
        assert!(result.is_null());
    }

    // ── text_trim tests ─────────────────────────────────────────────────────

    #[test]
    fn test_text_trim_truncation() {
        let text = CString::new("Hello World").unwrap();
        let result = unsafe { iscc_text_trim(text.as_ptr(), 5) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert_eq!(s, "Hello");
    }

    #[test]
    fn test_text_trim_null() {
        let result = unsafe { iscc_text_trim(ptr::null(), 10) };
        assert!(result.is_null());
    }

    // ── text_collapse tests ─────────────────────────────────────────────────

    #[test]
    fn test_text_collapse_basic() {
        let text = CString::new("Hello World!").unwrap();
        let result = unsafe { iscc_text_collapse(text.as_ptr()) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert_eq!(s, "helloworld");
    }

    #[test]
    fn test_text_collapse_null() {
        let result = unsafe { iscc_text_collapse(ptr::null()) };
        assert!(result.is_null());
    }

    // ── encode_base64 tests ─────────────────────────────────────────────────

    #[test]
    fn test_encode_base64_known() {
        let data = b"\x00\x01\x02";
        let result = unsafe { iscc_encode_base64(data.as_ptr(), data.len()) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert_eq!(s, "AAEC");
    }

    #[test]
    fn test_encode_base64_null() {
        let result = unsafe { iscc_encode_base64(ptr::null(), 0) };
        assert!(result.is_null());
    }

    // ── conformance_selftest test ───────────────────────────────────────────

    #[test]
    fn test_conformance_selftest() {
        assert!(iscc_conformance_selftest());
    }

    // ── iscc_decompose tests ────────────────────────────────────────────────

    #[test]
    fn test_decompose_known() {
        // Decompose a known ISCC-CODE (single Meta-Code unit)
        let code = CString::new("ISCC:AAAYPXW445FTYNJ3").unwrap();
        let arr = unsafe { iscc_decompose(code.as_ptr()) };
        let units = unsafe { c_ptr_to_string_vec(arr) }.unwrap();
        assert_eq!(units.len(), 1);
    }

    #[test]
    fn test_decompose_invalid() {
        let code = CString::new("INVALID").unwrap();
        let arr = unsafe { iscc_decompose(code.as_ptr()) };
        assert!(arr.is_null());
    }

    #[test]
    fn test_decompose_null() {
        let arr = unsafe { iscc_decompose(ptr::null()) };
        assert!(arr.is_null());
    }

    // ── iscc_sliding_window tests ───────────────────────────────────────────

    #[test]
    fn test_sliding_window_basic() {
        let seq = CString::new("Hello").unwrap();
        let arr = unsafe { iscc_sliding_window(seq.as_ptr(), 2) };
        let ngrams = unsafe { c_ptr_to_string_vec(arr) }.unwrap();
        assert_eq!(ngrams, vec!["He", "el", "ll", "lo"]);
    }

    #[test]
    fn test_sliding_window_width_too_small() {
        let seq = CString::new("Hello").unwrap();
        let arr = unsafe { iscc_sliding_window(seq.as_ptr(), 1) };
        assert!(arr.is_null());
        let err = iscc_last_error();
        assert!(!err.is_null());
        let msg = unsafe { CStr::from_ptr(err) }.to_str().unwrap();
        assert!(msg.contains("width"));
    }

    #[test]
    fn test_sliding_window_null() {
        let arr = unsafe { iscc_sliding_window(ptr::null(), 3) };
        assert!(arr.is_null());
    }

    // ── free_string_array null safety ───────────────────────────────────────

    #[test]
    fn test_free_string_array_null() {
        // Should be a no-op, not crash
        unsafe { iscc_free_string_array(ptr::null_mut()) };
    }

    // ── alg_simhash tests ──────────────────────────────────────────────────

    #[test]
    fn test_alg_simhash_basic() {
        // Feed two 4-byte digests, output should be 4 bytes
        let d1: [u8; 4] = [0xFF, 0x00, 0xFF, 0x00];
        let d2: [u8; 4] = [0xFF, 0xFF, 0x00, 0x00];
        let digests = [d1.as_ptr(), d2.as_ptr()];
        let lens = [d1.len(), d2.len()];
        let buf = unsafe { iscc_alg_simhash(digests.as_ptr(), lens.as_ptr(), 2) };
        assert!(!buf.data.is_null());
        assert_eq!(buf.len, 4);
        unsafe { iscc_free_byte_buffer(buf) };
    }

    #[test]
    fn test_alg_simhash_null() {
        // NULL digests with count > 0 returns null buffer
        let lens = [4usize];
        let buf = unsafe { iscc_alg_simhash(ptr::null(), lens.as_ptr(), 1) };
        assert!(buf.data.is_null());
        assert_eq!(buf.len, 0);
    }

    #[test]
    fn test_alg_simhash_empty() {
        // Zero digests yields 32-byte default output
        let buf = unsafe { iscc_alg_simhash(ptr::null(), ptr::null(), 0) };
        assert!(!buf.data.is_null());
        assert_eq!(buf.len, 32);
        unsafe { iscc_free_byte_buffer(buf) };
    }

    // ── alg_minhash_256 tests ──────────────────────────────────────────────

    #[test]
    fn test_alg_minhash_256_basic() {
        let features: Vec<u32> = vec![1, 2, 3, 4, 5];
        let buf = unsafe { iscc_alg_minhash_256(features.as_ptr(), features.len()) };
        assert!(!buf.data.is_null());
        assert_eq!(buf.len, 32);
        unsafe { iscc_free_byte_buffer(buf) };
    }

    #[test]
    fn test_alg_minhash_256_null() {
        // NULL features with len > 0 returns null buffer
        let buf = unsafe { iscc_alg_minhash_256(ptr::null(), 5) };
        assert!(buf.data.is_null());
        assert_eq!(buf.len, 0);
    }

    // ── alg_cdc_chunks tests ───────────────────────────────────────────────

    #[test]
    fn test_alg_cdc_chunks_basic() {
        let data = b"Hello World";
        let arr = unsafe { iscc_alg_cdc_chunks(data.as_ptr(), data.len(), false, 1024) };
        assert!(!arr.buffers.is_null());
        assert!(arr.count >= 1);

        // Concatenate all chunks and verify they equal the original data
        let mut concatenated = Vec::new();
        for i in 0..arr.count {
            let buf = unsafe { &*arr.buffers.add(i) };
            let chunk = unsafe { std::slice::from_raw_parts(buf.data, buf.len) };
            concatenated.extend_from_slice(chunk);
        }
        assert_eq!(concatenated, data);

        unsafe { iscc_free_byte_buffer_array(arr) };
    }

    #[test]
    fn test_alg_cdc_chunks_null() {
        // NULL data with len > 0 returns null array
        let arr = unsafe { iscc_alg_cdc_chunks(ptr::null(), 10, false, 1024) };
        assert!(arr.buffers.is_null());
        assert_eq!(arr.count, 0);
    }

    #[test]
    fn test_alg_cdc_chunks_empty() {
        // Empty data returns at least one chunk
        let arr = unsafe { iscc_alg_cdc_chunks(ptr::null(), 0, false, 1024) };
        assert!(!arr.buffers.is_null());
        assert_eq!(arr.count, 1);
        let buf = unsafe { &*arr.buffers };
        assert_eq!(buf.len, 0);
        unsafe { iscc_free_byte_buffer_array(arr) };
    }

    // ── soft_hash_video_v0 tests ───────────────────────────────────────────

    #[test]
    fn test_soft_hash_video_v0_basic() {
        // WTA hash requires frames with at least 380 elements
        let f1: Vec<i32> = (0..380).collect();
        let f2: Vec<i32> = (1..381).collect();
        let ptrs = [f1.as_ptr(), f2.as_ptr()];
        let lens = [f1.len(), f2.len()];
        let buf = unsafe { iscc_soft_hash_video_v0(ptrs.as_ptr(), lens.as_ptr(), 2, 64) };
        assert!(!buf.data.is_null());
        assert_eq!(buf.len, 8); // 64 bits / 8 = 8 bytes
        unsafe { iscc_free_byte_buffer(buf) };
    }

    #[test]
    fn test_soft_hash_video_v0_null() {
        // NULL frame_sigs returns null buffer
        let buf = unsafe { iscc_soft_hash_video_v0(ptr::null(), ptr::null(), 0, 64) };
        assert!(buf.data.is_null());
        assert_eq!(buf.len, 0);
    }

    // ── free byte buffer null safety ───────────────────────────────────────

    #[test]
    fn test_free_byte_buffer_null() {
        // Should be a no-op, not crash
        unsafe { iscc_free_byte_buffer(null_byte_buffer()) };
    }

    #[test]
    fn test_free_byte_buffer_array_null() {
        // Should be a no-op, not crash
        unsafe { iscc_free_byte_buffer_array(null_byte_buffer_array()) };
    }

    // ── DataHasher streaming tests ───────────────────────────────────────

    #[test]
    fn test_data_hasher_basic() {
        let hasher = iscc_data_hasher_new();
        assert!(!hasher.is_null());
        let data = b"Hello World";
        let ok = unsafe { iscc_data_hasher_update(hasher, data.as_ptr(), data.len()) };
        assert!(ok);
        let result = unsafe { iscc_data_hasher_finalize(hasher, 64) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert!(s.starts_with("ISCC:"));
        unsafe { iscc_data_hasher_free(hasher) };
    }

    #[test]
    fn test_data_hasher_matches_gen() {
        // Streaming result must match one-shot gen_data_code_v0
        let data = b"Hello World";
        let oneshot = unsafe { iscc_gen_data_code_v0(data.as_ptr(), data.len(), 64) };
        let oneshot_str = unsafe { c_ptr_to_string(oneshot) }.unwrap();

        let hasher = iscc_data_hasher_new();
        let ok = unsafe { iscc_data_hasher_update(hasher, data.as_ptr(), data.len()) };
        assert!(ok);
        let result = unsafe { iscc_data_hasher_finalize(hasher, 64) };
        let streaming_str = unsafe { c_ptr_to_string(result) }.unwrap();
        unsafe { iscc_data_hasher_free(hasher) };

        assert_eq!(streaming_str, oneshot_str);
    }

    #[test]
    fn test_data_hasher_multi_update() {
        let data = b"Hello World";
        // Single update reference
        let hasher1 = iscc_data_hasher_new();
        unsafe { iscc_data_hasher_update(hasher1, data.as_ptr(), data.len()) };
        let result1 = unsafe { iscc_data_hasher_finalize(hasher1, 64) };
        let s1 = unsafe { c_ptr_to_string(result1) }.unwrap();
        unsafe { iscc_data_hasher_free(hasher1) };

        // Split across two updates
        let hasher2 = iscc_data_hasher_new();
        unsafe { iscc_data_hasher_update(hasher2, data[..5].as_ptr(), 5) };
        unsafe { iscc_data_hasher_update(hasher2, data[5..].as_ptr(), data.len() - 5) };
        let result2 = unsafe { iscc_data_hasher_finalize(hasher2, 64) };
        let s2 = unsafe { c_ptr_to_string(result2) }.unwrap();
        unsafe { iscc_data_hasher_free(hasher2) };

        assert_eq!(s1, s2);
    }

    #[test]
    fn test_data_hasher_empty() {
        let hasher = iscc_data_hasher_new();
        let result = unsafe { iscc_data_hasher_finalize(hasher, 64) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert!(s.starts_with("ISCC:"));
        unsafe { iscc_data_hasher_free(hasher) };
    }

    #[test]
    fn test_data_hasher_finalize_twice() {
        let hasher = iscc_data_hasher_new();
        let result1 = unsafe { iscc_data_hasher_finalize(hasher, 64) };
        assert!(!result1.is_null());
        unsafe { iscc_free_string(result1) };

        // Second finalize should return NULL with error
        let result2 = unsafe { iscc_data_hasher_finalize(hasher, 64) };
        assert!(result2.is_null());
        let err = iscc_last_error();
        assert!(!err.is_null());
        let msg = unsafe { CStr::from_ptr(err) }.to_str().unwrap();
        assert!(msg.contains("already finalized"));

        unsafe { iscc_data_hasher_free(hasher) };
    }

    #[test]
    fn test_data_hasher_free_null() {
        // Should be a no-op, not crash
        unsafe { iscc_data_hasher_free(ptr::null_mut()) };
    }

    // ── InstanceHasher streaming tests ───────────────────────────────────

    #[test]
    fn test_instance_hasher_basic() {
        let hasher = iscc_instance_hasher_new();
        assert!(!hasher.is_null());
        let data = b"Hello World";
        let ok = unsafe { iscc_instance_hasher_update(hasher, data.as_ptr(), data.len()) };
        assert!(ok);
        let result = unsafe { iscc_instance_hasher_finalize(hasher, 64) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert!(s.starts_with("ISCC:"));
        unsafe { iscc_instance_hasher_free(hasher) };
    }

    #[test]
    fn test_instance_hasher_matches_gen() {
        // Streaming result must match one-shot gen_instance_code_v0
        let data = b"Hello World";
        let oneshot = unsafe { iscc_gen_instance_code_v0(data.as_ptr(), data.len(), 64) };
        let oneshot_str = unsafe { c_ptr_to_string(oneshot) }.unwrap();

        let hasher = iscc_instance_hasher_new();
        let ok = unsafe { iscc_instance_hasher_update(hasher, data.as_ptr(), data.len()) };
        assert!(ok);
        let result = unsafe { iscc_instance_hasher_finalize(hasher, 64) };
        let streaming_str = unsafe { c_ptr_to_string(result) }.unwrap();
        unsafe { iscc_instance_hasher_free(hasher) };

        assert_eq!(streaming_str, oneshot_str);
    }

    #[test]
    fn test_instance_hasher_multi_update() {
        let data = b"Hello World";
        // Single update reference
        let hasher1 = iscc_instance_hasher_new();
        unsafe { iscc_instance_hasher_update(hasher1, data.as_ptr(), data.len()) };
        let result1 = unsafe { iscc_instance_hasher_finalize(hasher1, 64) };
        let s1 = unsafe { c_ptr_to_string(result1) }.unwrap();
        unsafe { iscc_instance_hasher_free(hasher1) };

        // Split across two updates
        let hasher2 = iscc_instance_hasher_new();
        unsafe { iscc_instance_hasher_update(hasher2, data[..5].as_ptr(), 5) };
        unsafe { iscc_instance_hasher_update(hasher2, data[5..].as_ptr(), data.len() - 5) };
        let result2 = unsafe { iscc_instance_hasher_finalize(hasher2, 64) };
        let s2 = unsafe { c_ptr_to_string(result2) }.unwrap();
        unsafe { iscc_instance_hasher_free(hasher2) };

        assert_eq!(s1, s2);
    }

    #[test]
    fn test_instance_hasher_empty() {
        // Finalize immediately with no data — matches gen_instance_code_v0(empty)
        let hasher = iscc_instance_hasher_new();
        let result = unsafe { iscc_instance_hasher_finalize(hasher, 64) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert_eq!(s, "ISCC:IAA26E2JXH27TING");
        unsafe { iscc_instance_hasher_free(hasher) };
    }

    #[test]
    fn test_instance_hasher_finalize_twice() {
        let hasher = iscc_instance_hasher_new();
        let result1 = unsafe { iscc_instance_hasher_finalize(hasher, 64) };
        assert!(!result1.is_null());
        unsafe { iscc_free_string(result1) };

        // Second finalize should return NULL with error
        let result2 = unsafe { iscc_instance_hasher_finalize(hasher, 64) };
        assert!(result2.is_null());
        let err = iscc_last_error();
        assert!(!err.is_null());
        let msg = unsafe { CStr::from_ptr(err) }.to_str().unwrap();
        assert!(msg.contains("already finalized"));

        unsafe { iscc_instance_hasher_free(hasher) };
    }

    #[test]
    fn test_instance_hasher_free_null() {
        // Should be a no-op, not crash
        unsafe { iscc_instance_hasher_free(ptr::null_mut()) };
    }

    // ── Algorithm constants tests ────────────────────────────────────────

    #[test]
    fn test_meta_trim_name() {
        assert_eq!(iscc_meta_trim_name(), 128);
    }

    #[test]
    fn test_meta_trim_description() {
        assert_eq!(iscc_meta_trim_description(), 4096);
    }

    #[test]
    fn test_meta_trim_meta() {
        assert_eq!(iscc_meta_trim_meta(), 128_000);
    }

    #[test]
    fn test_io_read_size() {
        assert_eq!(iscc_io_read_size(), 4_194_304);
    }

    #[test]
    fn test_text_ngram_size() {
        assert_eq!(iscc_text_ngram_size(), 13);
    }

    // ── json_to_data_url tests ───────────────────────────────────────────

    #[test]
    fn test_json_to_data_url_basic() {
        let json = CString::new(r#"{"key":"value"}"#).unwrap();
        let result = unsafe { iscc_json_to_data_url(json.as_ptr()) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert!(s.starts_with("data:application/json;base64,"));
    }

    #[test]
    fn test_json_to_data_url_null() {
        let result = unsafe { iscc_json_to_data_url(ptr::null()) };
        assert!(result.is_null());
    }

    // ── encode_component tests ───────────────────────────────────────────

    #[test]
    fn test_encode_component_basic() {
        // Meta-Code (mtype=0, stype=0, version=0, 64-bit, 8 zero bytes)
        let digest = [0u8; 8];
        let result = unsafe { iscc_encode_component(0, 0, 0, 64, digest.as_ptr(), digest.len()) };
        let s = unsafe { c_ptr_to_string(result) }.unwrap();
        assert!(!s.is_empty());
    }

    #[test]
    fn test_encode_component_null_digest_with_len() {
        let result = unsafe { iscc_encode_component(0, 0, 0, 64, ptr::null(), 8) };
        assert!(result.is_null());
    }

    #[test]
    fn test_encode_component_zero_len_digest() {
        // Zero-length digest with len=0 still passes the null check
        // but encode_component may reject bit_length=0 — verify it handles gracefully
        let result = unsafe { iscc_encode_component(0, 0, 0, 64, ptr::null(), 0) };
        // Empty digest with bit_length=64 is invalid — expect NULL
        assert!(result.is_null());
    }

    // ── iscc_decode tests ────────────────────────────────────────────────

    #[test]
    fn test_decode_known_meta_code() {
        // Decode known Meta-Code "AAAZXZ6OU74YAZIM"
        let iscc = CString::new("AAAZXZ6OU74YAZIM").unwrap();
        let result = unsafe { iscc_decode(iscc.as_ptr()) };
        assert!(result.ok);
        assert_eq!(result.maintype, 0); // Meta
        assert_eq!(result.subtype, 0);
        assert_eq!(result.version, 0);
        assert_eq!(result.length, 1); // 64-bit = length index 1 ((1+1)*32 = 64)
        assert!(!result.digest.data.is_null());
        assert_eq!(result.digest.len, 8); // 64 bits = 8 bytes
        unsafe { iscc_free_decode_result(result) };
    }

    #[test]
    fn test_decode_null() {
        let result = unsafe { iscc_decode(ptr::null()) };
        assert!(!result.ok);
        assert!(result.digest.data.is_null());
    }

    #[test]
    fn test_decode_invalid() {
        let iscc = CString::new("INVALID").unwrap();
        let result = unsafe { iscc_decode(iscc.as_ptr()) };
        assert!(!result.ok);
        assert!(result.digest.data.is_null());
    }

    #[test]
    fn test_decode_with_prefix() {
        // Should handle "ISCC:" prefix
        let iscc = CString::new("ISCC:AAAZXZ6OU74YAZIM").unwrap();
        let result = unsafe { iscc_decode(iscc.as_ptr()) };
        assert!(result.ok);
        assert_eq!(result.maintype, 0);
        assert_eq!(result.digest.len, 8);
        unsafe { iscc_free_decode_result(result) };
    }

    #[test]
    fn test_free_decode_result_null_digest() {
        // Should be a no-op for null digest
        let result = IsccDecodeResult {
            ok: false,
            maintype: 0,
            subtype: 0,
            version: 0,
            length: 0,
            digest: null_byte_buffer(),
        };
        unsafe { iscc_free_decode_result(result) };
    }

    // ── Roundtrip: encode_component → iscc_decode ────────────────────────

    #[test]
    fn test_encode_decode_roundtrip() {
        let digest = [0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89u8];
        let encoded = unsafe { iscc_encode_component(0, 0, 0, 64, digest.as_ptr(), digest.len()) };
        assert!(!encoded.is_null());

        // Decode the encoded result
        let decoded = unsafe { iscc_decode(encoded) };
        assert!(decoded.ok);
        assert_eq!(decoded.maintype, 0);
        assert_eq!(decoded.subtype, 0);
        assert_eq!(decoded.version, 0);
        assert_eq!(decoded.length, 1); // 64-bit = length index 1 ((1+1)*32 = 64)
        assert_eq!(decoded.digest.len, 8);

        // Verify digest matches
        let decoded_digest =
            unsafe { std::slice::from_raw_parts(decoded.digest.data, decoded.digest.len) };
        assert_eq!(decoded_digest, &digest);

        unsafe { iscc_free_string(encoded) };
        unsafe { iscc_free_decode_result(decoded) };
    }
}
