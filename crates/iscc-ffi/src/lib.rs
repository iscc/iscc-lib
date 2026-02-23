//! C FFI bindings for iscc-lib.
//!
//! Exposes all 9 `gen_*_v0` functions as `extern "C"` symbols for integration
//! from C, Go, Java, C#, and any other language with C interop.
//!
//! ## Memory model
//!
//! Functions return heap-allocated C strings (`*mut c_char`) via `CString`.
//! The caller must free them with `iscc_free_string()`. On error, functions
//! return `NULL` and the caller retrieves the error message via
//! `iscc_last_error()`.
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

    let frames: Vec<Vec<i32>> = sig_ptrs
        .iter()
        .zip(lens.iter())
        .map(|(&ptr, &len)| {
            // SAFETY: caller guarantees each ptr is valid for its length
            unsafe { std::slice::from_raw_parts(ptr, len) }.to_vec()
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
}
