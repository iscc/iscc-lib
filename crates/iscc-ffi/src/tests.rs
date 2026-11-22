//! Unit tests for the iscc-ffi C exports.

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
    let result = unsafe { iscc_gen_meta_code_v0(name.as_ptr(), desc.as_ptr(), ptr::null(), 64) };
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
fn test_gen_image_code_v0_null_empty() {
    let result = unsafe { iscc_gen_image_code_v0(ptr::null(), 0, 64) };
    assert_str_matches_core(result, iscc_lib::gen_image_code_v0(&[], 64).map(|r| r.iscc));
}

#[test]
fn test_gen_image_code_v0_null_with_len() {
    let result = unsafe { iscc_gen_image_code_v0(ptr::null(), 1024, 64) };
    assert!(result.is_null());
    assert_last_error_contains("pixels must not be NULL");
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
fn test_gen_audio_code_v0_null_empty() {
    let result = unsafe { iscc_gen_audio_code_v0(ptr::null(), 0, 64) };
    let s = unsafe { c_ptr_to_string(result) }.unwrap();
    assert_eq!(s, iscc_lib::gen_audio_code_v0(&[], 64).unwrap().iscc);
}

#[test]
fn test_gen_audio_code_v0_null_with_len() {
    let result = unsafe { iscc_gen_audio_code_v0(ptr::null(), 3, 64) };
    assert!(result.is_null());
    assert_last_error_contains("cv must not be NULL");
}

#[test]
fn test_gen_video_code_v0_null_empty() {
    // Zero frames reach the core, which rejects the empty list itself
    let result = unsafe { iscc_gen_video_code_v0(ptr::null(), ptr::null(), 0, 64) };
    let empty: &[&[i32]] = &[];
    assert_str_matches_core(
        result,
        iscc_lib::gen_video_code_v0(empty, 64).map(|r| r.iscc),
    );
}

#[test]
fn test_gen_video_code_v0_null_sigs_with_count() {
    let lens = [380usize];
    let result = unsafe { iscc_gen_video_code_v0(ptr::null(), lens.as_ptr(), 1, 64) };
    assert!(result.is_null());
    assert_last_error_contains("frame_sigs must not be NULL");
}

#[test]
fn test_gen_video_code_v0_null_lens_with_count() {
    let f1: Vec<i32> = (0..380).collect();
    let ptrs = [f1.as_ptr()];
    let result = unsafe { iscc_gen_video_code_v0(ptrs.as_ptr(), ptr::null(), 1, 64) };
    assert!(result.is_null());
    assert_last_error_contains("frame_lens must not be NULL");
}

#[test]
fn test_gen_video_code_v0_null_inner_row_with_len() {
    let f1: Vec<i32> = (0..380).collect();
    let ptrs = [f1.as_ptr(), ptr::null()];
    let lens = [380usize, 380];
    let result = unsafe { iscc_gen_video_code_v0(ptrs.as_ptr(), lens.as_ptr(), 2, 64) };
    assert!(result.is_null());
    assert_last_error_contains("frame_sigs[1] must not be NULL");
}

#[test]
fn test_gen_video_code_v0_null_inner_row_empty() {
    // A NULL row with length 0 is an empty row; the core decides what it means
    let ptrs = [ptr::null()];
    let lens = [0usize];
    let result = unsafe { iscc_gen_video_code_v0(ptrs.as_ptr(), lens.as_ptr(), 1, 64) };
    let rows: &[&[i32]] = &[&[]];
    assert_str_matches_core(
        result,
        iscc_lib::gen_video_code_v0(rows, 64).map(|r| r.iscc),
    );
}

#[test]
fn test_gen_mixed_code_v0_null_empty() {
    let result = unsafe { iscc_gen_mixed_code_v0(ptr::null(), 0, 64) };
    let empty: &[&str] = &[];
    assert_str_matches_core(
        result,
        iscc_lib::gen_mixed_code_v0(empty, 64).map(|r| r.iscc),
    );
}

#[test]
fn test_gen_mixed_code_v0_null_with_count() {
    let result = unsafe { iscc_gen_mixed_code_v0(ptr::null(), 2, 64) };
    assert!(result.is_null());
    assert_last_error_contains("codes must not be NULL");
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
fn test_gen_data_code_v0_null_empty() {
    let result = unsafe { iscc_gen_data_code_v0(ptr::null(), 0, 64) };
    let s = unsafe { c_ptr_to_string(result) }.unwrap();
    assert_eq!(s, iscc_lib::gen_data_code_v0(&[], 64).unwrap().iscc);
}

#[test]
fn test_gen_data_code_v0_null_with_len() {
    let result = unsafe { iscc_gen_data_code_v0(ptr::null(), 11, 64) };
    assert!(result.is_null());
    assert_last_error_contains("data must not be NULL");
}

#[test]
fn test_gen_instance_code_v0_basic() {
    let data = b"";
    let result = unsafe { iscc_gen_instance_code_v0(data.as_ptr(), data.len(), 64) };
    let s = unsafe { c_ptr_to_string(result) }.unwrap();
    assert_eq!(s, "ISCC:IAA26E2JXH27TING");
}

#[test]
fn test_gen_instance_code_v0_null_empty() {
    let result = unsafe { iscc_gen_instance_code_v0(ptr::null(), 0, 64) };
    let s = unsafe { c_ptr_to_string(result) }.unwrap();
    assert_eq!(s, "ISCC:IAA26E2JXH27TING");
}

#[test]
fn test_gen_instance_code_v0_null_with_len() {
    let result = unsafe { iscc_gen_instance_code_v0(ptr::null(), 4, 64) };
    assert!(result.is_null());
    assert_last_error_contains("data must not be NULL");
}

#[test]
fn test_gen_iscc_code_v0_null_empty() {
    let result = unsafe { iscc_gen_iscc_code_v0(ptr::null(), 0, false) };
    let empty: &[&str] = &[];
    assert_str_matches_core(
        result,
        iscc_lib::gen_iscc_code_v0(empty, false).map(|r| r.iscc),
    );
}

#[test]
fn test_gen_iscc_code_v0_null_with_count() {
    let result = unsafe { iscc_gen_iscc_code_v0(ptr::null(), 2, false) };
    assert!(result.is_null());
    assert_last_error_contains("codes must not be NULL");
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
fn test_encode_base64_null_empty() {
    let result = unsafe { iscc_encode_base64(ptr::null(), 0) };
    let s = unsafe { c_ptr_to_string(result) }.unwrap();
    assert_eq!(s, "");
}

#[test]
fn test_encode_base64_null_with_len() {
    let result = unsafe { iscc_encode_base64(ptr::null(), 3) };
    assert!(result.is_null());
    assert_last_error_contains("data must not be NULL");
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

// ── iscc_encode_seq / iscc_decode_seq tests ─────────────────────────────

/// Decode a `bytes:<hex>` conformance value into raw bytes.
fn hex_bytes(value: &str) -> Vec<u8> {
    let hex = value.strip_prefix("bytes:").expect("bytes: prefix");
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

/// Load one section of the vendored `data.json` conformance vectors.
fn seq_section(name: &str) -> serde_json::Map<String, serde_json::Value> {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../iscc-lib/tests/data.json")).unwrap();
    data[name].as_object().unwrap().clone()
}

/// Call `iscc_encode_seq` on Rust strings; returns the bytes or `None` on error.
fn ffi_encode_seq(units: &[&str]) -> Option<Vec<u8>> {
    let owned: Vec<CString> = units.iter().map(|u| CString::new(*u).unwrap()).collect();
    let ptrs: Vec<*const c_char> = owned.iter().map(|c| c.as_ptr()).collect();
    let buf = unsafe { iscc_encode_seq(ptrs.as_ptr(), ptrs.len()) };
    if buf.data.is_null() {
        return None;
    }
    // SAFETY: buf.data is valid for buf.len bytes until freed below
    let bytes = unsafe { std::slice::from_raw_parts(buf.data, buf.len) }.to_vec();
    unsafe { iscc_free_byte_buffer(buf) };
    Some(bytes)
}

/// Call `iscc_decode_seq` on a byte slice; returns the units or `None` on error.
fn ffi_decode_seq(data: &[u8]) -> Option<Vec<String>> {
    let arr = unsafe { iscc_decode_seq(data.as_ptr(), data.len()) };
    unsafe { c_ptr_to_string_vec(arr) }
}

/// Assert that the last FFI call set a non-NULL error message.
fn assert_last_error_set() {
    assert!(
        !iscc_last_error().is_null(),
        "iscc_last_error() must be set"
    );
}

/// Assert that the last FFI error message contains `needle`.
fn assert_last_error_contains(needle: &str) {
    assert_last_error_set();
    let msg = unsafe { CStr::from_ptr(iscc_last_error()) }
        .to_str()
        .unwrap();
    assert!(msg.contains(needle), "{msg:?} lacks {needle:?}");
}

/// Assert that an FFI string result matches the core's outcome: the same string
/// on `Ok`, NULL with the core's own message (never "must not be NULL") on `Err`.
fn assert_str_matches_core(result: *mut c_char, core: iscc_lib::IsccResult<String>) {
    let got = unsafe { c_ptr_to_string(result) };
    match core {
        Ok(expected) => assert_eq!(got.as_deref(), Some(expected.as_str())),
        Err(e) => {
            assert!(got.is_none(), "expected core error {e}, got {got:?}");
            assert_last_error_is_core(&e.to_string());
        }
    }
}

/// Assert that an FFI byte-buffer result matches the core's outcome (see
/// `assert_str_matches_core`), freeing the buffer.
fn assert_buf_matches_core(buf: IsccByteBuffer, core: iscc_lib::IsccResult<Vec<u8>>) {
    match core {
        Ok(expected) => {
            assert!(!buf.data.is_null(), "expected {expected:?}, got NULL");
            // SAFETY: buf.data is valid for buf.len bytes until freed below
            let got = unsafe { std::slice::from_raw_parts(buf.data, buf.len) }.to_vec();
            unsafe { iscc_free_byte_buffer(buf) };
            assert_eq!(got, expected);
        }
        Err(e) => {
            assert!(buf.data.is_null(), "expected core error {e}");
            assert_last_error_is_core(&e.to_string());
        }
    }
}

/// Assert that the last FFI error equals the core's message `expected`.
fn assert_last_error_is_core(expected: &str) {
    assert_last_error_set();
    let msg = unsafe { CStr::from_ptr(iscc_last_error()) }
        .to_str()
        .unwrap();
    assert_eq!(msg, expected);
    assert!(!msg.contains("must not be NULL"), "{msg:?}");
}

#[test]
fn test_encode_seq_conformance() {
    let section = seq_section("encode_seq");
    assert!(!section.is_empty());
    for (name, case) in &section {
        let units: Vec<&str> = case["inputs"][0]
            .as_array()
            .unwrap()
            .iter()
            .map(|u| u.as_str().unwrap())
            .collect();
        let expected = hex_bytes(case["outputs"].as_str().unwrap());
        let got = ffi_encode_seq(&units).unwrap_or_else(|| panic!("{name}: NULL result"));
        assert_eq!(got, expected, "{name}");
    }
}

#[test]
fn test_decode_seq_conformance() {
    let section = seq_section("decode_seq");
    assert!(!section.is_empty());
    for (name, case) in &section {
        let data = hex_bytes(case["inputs"][0].as_str().unwrap());
        let expected: Vec<String> = case["outputs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|u| u.as_str().unwrap().to_owned())
            .collect();
        let got = ffi_decode_seq(&data).unwrap_or_else(|| panic!("{name}: NULL result"));
        assert_eq!(got, expected, "{name}");
    }
}

#[test]
fn test_seq_round_trip() {
    let section = seq_section("encode_seq");
    for (name, case) in &section {
        let units: Vec<&str> = case["inputs"][0]
            .as_array()
            .unwrap()
            .iter()
            .map(|u| u.as_str().unwrap())
            .collect();
        let bytes = ffi_encode_seq(&units).unwrap();
        let decoded = ffi_decode_seq(&bytes).unwrap();
        assert_eq!(decoded, units, "{name}");
    }
}

#[test]
fn test_seq_round_trip_bytes() {
    let section = seq_section("decode_seq");
    assert!(!section.is_empty());
    for (name, case) in &section {
        let data = hex_bytes(case["inputs"][0].as_str().unwrap());
        let units = ffi_decode_seq(&data).unwrap_or_else(|| panic!("{name}: decode NULL"));
        let refs: Vec<&str> = units.iter().map(String::as_str).collect();
        let bytes = ffi_encode_seq(&refs).unwrap_or_else(|| panic!("{name}: encode NULL"));
        assert_eq!(bytes, data, "{name}");
    }
}

#[test]
fn test_encode_seq_lowercase_without_prefix() {
    let got = ffi_encode_seq(&["ead2rasiyu5iklenp2ofi4chzgrwyqcsw2wkx3y6fjgocxsynyglgbi"]);
    let expected =
        ffi_encode_seq(&["ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI"]);
    assert!(got.is_some());
    assert_eq!(got, expected);
}

#[test]
fn test_encode_seq_null_units_empty() {
    // (NULL, 0) reaches the core, which rejects the empty list itself
    let buf = unsafe { iscc_encode_seq(ptr::null(), 0) };
    assert!(buf.data.is_null());
    assert_last_error_contains("at least one ISCC-UNIT");
}

#[test]
fn test_encode_seq_null_units_with_count() {
    let buf = unsafe { iscc_encode_seq(ptr::null(), 1) };
    assert!(buf.data.is_null());
    assert_last_error_contains("units must not be NULL");
}

#[test]
fn test_encode_seq_empty_list() {
    let unit =
        CString::new("ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI").unwrap();
    let ptrs = [unit.as_ptr()];
    let buf = unsafe { iscc_encode_seq(ptrs.as_ptr(), 0) };
    assert!(buf.data.is_null());
    assert_last_error_contains("at least one ISCC-UNIT");
}

#[test]
fn test_encode_seq_null_element() {
    let unit =
        CString::new("ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI").unwrap();
    let ptrs = [unit.as_ptr(), ptr::null()];
    let buf = unsafe { iscc_encode_seq(ptrs.as_ptr(), ptrs.len()) };
    assert!(buf.data.is_null());
    assert_last_error_contains("units[1]");
}

#[test]
fn test_encode_seq_malformed_unit() {
    assert!(ffi_encode_seq(&["ISCC:NOT-A-VALID-UNIT!"]).is_none());
    assert_last_error_contains("base32 decode error");
}

#[test]
fn test_encode_seq_rejects_iscc_code_unit() {
    // Composite ISCC-CODE (MainType ISCC) is excluded from ISCC-SEQ.
    assert!(
        ffi_encode_seq(&["ISCC:KACYPXW445FTYNJ3CYSXHAFJMA2HUWULUNRFE3BLHRSCXYH2M5AEGQY"]).is_none()
    );
    assert_last_error_contains("not permitted in an ISCC-SEQ");
}

#[test]
fn test_decode_seq_empty() {
    let arr = unsafe { iscc_decode_seq(ptr::null(), 0) };
    assert!(arr.is_null());
    assert_last_error_contains("Empty ISCC-SEQ");
}

#[test]
fn test_decode_seq_truncated() {
    let section = seq_section("decode_seq");
    let first = section.values().next().unwrap();
    let data = hex_bytes(first["inputs"][0].as_str().unwrap());
    assert!(ffi_decode_seq(&data[..data.len() - 1]).is_none());
    assert_last_error_contains("Truncated");
}

#[test]
fn test_decode_seq_null_data_nonzero_len() {
    let arr = unsafe { iscc_decode_seq(ptr::null(), 4) };
    assert!(arr.is_null());
    assert_last_error_contains("data must not be NULL");
}

#[test]
fn test_seq_error_cleared_on_success() {
    assert!(ffi_decode_seq(&[]).is_none());
    assert_last_error_set();
    let data =
        hex_bytes("bytes:2007a88248c53a852c8d7e9c547047c9a36c4052b6acabef1e2a4ce15e586e0cb305");
    assert!(ffi_decode_seq(&data).is_some());
    assert!(iscc_last_error().is_null());
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
    assert_last_error_contains("digests must not be NULL");
}

#[test]
fn test_alg_simhash_null_lens() {
    let d1: [u8; 4] = [0xFF, 0x00, 0xFF, 0x00];
    let digests = [d1.as_ptr()];
    let buf = unsafe { iscc_alg_simhash(digests.as_ptr(), ptr::null(), 1) };
    assert!(buf.data.is_null());
    assert_last_error_contains("digest_lens must not be NULL");
}

#[test]
fn test_alg_simhash_null_inner_row_with_len() {
    let d1: [u8; 4] = [0xFF, 0x00, 0xFF, 0x00];
    let digests = [d1.as_ptr(), ptr::null()];
    let lens = [4usize, 4];
    let buf = unsafe { iscc_alg_simhash(digests.as_ptr(), lens.as_ptr(), 2) };
    assert!(buf.data.is_null());
    assert_last_error_contains("digests[1] must not be NULL");
}

#[test]
fn test_alg_simhash_null_inner_row_empty() {
    // A NULL row with length 0 is an empty digest; the core decides what it means
    let digests = [ptr::null()];
    let lens = [0usize];
    let buf = unsafe { iscc_alg_simhash(digests.as_ptr(), lens.as_ptr(), 1) };
    let rows: &[&[u8]] = &[&[]];
    assert_buf_matches_core(buf, iscc_lib::alg_simhash(rows));
}

#[test]
fn test_alg_simhash_empty() {
    // Zero digests forwards the core's empty-list rejection
    let buf = unsafe { iscc_alg_simhash(ptr::null(), ptr::null(), 0) };
    assert!(buf.data.is_null());
    assert_eq!(buf.len, 0);
    let err = iscc_last_error();
    assert!(!err.is_null());
    let msg = unsafe { CStr::from_ptr(err) }.to_str().unwrap();
    assert!(msg.contains("must not be empty"), "got: {msg}");
    assert!(!msg.contains("must not be NULL"), "got: {msg}");
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
    assert_last_error_contains("features must not be NULL");
}

#[test]
fn test_alg_minhash_256_null_empty() {
    // (NULL, 0) is an empty list: the core's empty-list error, not a NULL rejection
    let buf = unsafe { iscc_alg_minhash_256(ptr::null(), 0) };
    assert_buf_matches_core(buf, iscc_lib::alg_minhash_256(&[]));
    assert_last_error_contains("features must not be empty");
    let msg = unsafe { CStr::from_ptr(iscc_last_error()) }
        .to_str()
        .unwrap();
    assert!(!msg.contains("must not be NULL"), "got: {msg}");
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
    assert_last_error_contains("data must not be NULL");
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
fn test_soft_hash_video_v0_null_empty() {
    // Zero frames reach the core, which rejects the empty list itself
    let buf = unsafe { iscc_soft_hash_video_v0(ptr::null(), ptr::null(), 0, 64) };
    let empty: &[&[i32]] = &[];
    assert_buf_matches_core(buf, iscc_lib::soft_hash_video_v0(empty, 64));
}

#[test]
fn test_soft_hash_video_v0_null_sigs_with_count() {
    let lens = [380usize];
    let buf = unsafe { iscc_soft_hash_video_v0(ptr::null(), lens.as_ptr(), 1, 64) };
    assert!(buf.data.is_null());
    assert_last_error_contains("frame_sigs must not be NULL");
}

#[test]
fn test_soft_hash_video_v0_null_inner_row_with_len() {
    let f1: Vec<i32> = (0..380).collect();
    let ptrs = [f1.as_ptr(), ptr::null()];
    let lens = [380usize, 380];
    let buf = unsafe { iscc_soft_hash_video_v0(ptrs.as_ptr(), lens.as_ptr(), 2, 64) };
    assert!(buf.data.is_null());
    assert_last_error_contains("frame_sigs[1] must not be NULL");
}

#[test]
fn test_soft_hash_video_v0_null_inner_row_empty() {
    // A NULL row with length 0 is an empty row; the core decides what it means
    let ptrs = [ptr::null()];
    let lens = [0usize];
    let buf = unsafe { iscc_soft_hash_video_v0(ptrs.as_ptr(), lens.as_ptr(), 1, 64) };
    let rows: &[&[i32]] = &[&[]];
    assert_buf_matches_core(buf, iscc_lib::soft_hash_video_v0(rows, 64));
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
fn test_data_hasher_update_null() {
    let hasher = iscc_data_hasher_new();
    assert!(unsafe { iscc_data_hasher_update(hasher, ptr::null(), 0) });
    assert!(!unsafe { iscc_data_hasher_update(hasher, ptr::null(), 4) });
    assert_last_error_contains("data must not be NULL");
    unsafe { iscc_data_hasher_free(hasher) };
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
fn test_instance_hasher_update_null() {
    let hasher = iscc_instance_hasher_new();
    assert!(unsafe { iscc_instance_hasher_update(hasher, ptr::null(), 0) });
    assert!(!unsafe { iscc_instance_hasher_update(hasher, ptr::null(), 4) });
    assert_last_error_contains("data must not be NULL");
    unsafe { iscc_instance_hasher_free(hasher) };
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
    assert_last_error_contains("digest must not be NULL");
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

// ── gen_sum_code_v0 tests ─────────────────────────────────────────────

/// Write data to a unique temp file and return the path.
fn write_ffi_temp_file(name: &str, data: &[u8]) -> std::path::PathBuf {
    use std::io::Write;
    let path = std::env::temp_dir().join(format!("iscc_ffi_test_{name}"));
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(data).unwrap();
    path
}

#[test]
fn test_gen_sum_code_v0_basic() {
    let data = b"Hello World";
    let path = write_ffi_temp_file("sum_basic", data);
    let path_c = CString::new(path.to_str().unwrap()).unwrap();

    let result = unsafe { iscc_gen_sum_code_v0(path_c.as_ptr(), 64, false, false) };
    assert!(result.ok);
    assert!(!result.iscc.is_null());
    assert!(!result.datahash.is_null());
    assert_eq!(result.filesize, 11);
    assert!(result.units.is_null());

    let iscc_str = unsafe { CStr::from_ptr(result.iscc) }.to_str().unwrap();
    assert!(iscc_str.starts_with("ISCC:"));

    unsafe { iscc_free_sum_code_result(result) };
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_gen_sum_code_v0_null_path() {
    let result = unsafe { iscc_gen_sum_code_v0(ptr::null(), 64, false, false) };
    assert!(!result.ok);
    assert!(result.iscc.is_null());
    assert!(result.datahash.is_null());
    assert_eq!(result.filesize, 0);
    assert!(result.units.is_null());
}

#[test]
fn test_gen_sum_code_v0_free_null_strings() {
    // Freeing a result with null strings and null units should be a no-op
    let result = null_sum_code_result();
    assert!(result.units.is_null());
    unsafe { iscc_free_sum_code_result(result) };
}

#[test]
fn test_gen_sum_code_v0_matches_lib() {
    let data = b"Hello World";
    let path = write_ffi_temp_file("sum_matches", data);
    let path_c = CString::new(path.to_str().unwrap()).unwrap();

    // FFI result
    let ffi_result = unsafe { iscc_gen_sum_code_v0(path_c.as_ptr(), 64, false, false) };
    assert!(ffi_result.ok);

    let ffi_iscc = unsafe { CStr::from_ptr(ffi_result.iscc) }
        .to_str()
        .unwrap()
        .to_owned();
    let ffi_datahash = unsafe { CStr::from_ptr(ffi_result.datahash) }
        .to_str()
        .unwrap()
        .to_owned();
    let ffi_filesize = ffi_result.filesize;

    unsafe { iscc_free_sum_code_result(ffi_result) };

    // Direct lib result
    let lib_result = iscc_lib::gen_sum_code_v0(&path, 64, false, false).unwrap();

    assert_eq!(ffi_iscc, lib_result.iscc);
    assert_eq!(ffi_datahash, lib_result.datahash);
    assert_eq!(ffi_filesize, lib_result.filesize);

    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_gen_sum_code_v0_units_enabled() {
    let data = b"Hello World";
    let path = write_ffi_temp_file("sum_units_on", data);
    let path_c = CString::new(path.to_str().unwrap()).unwrap();

    let result = unsafe { iscc_gen_sum_code_v0(path_c.as_ptr(), 64, false, true) };
    assert!(result.ok);
    assert!(!result.units.is_null());

    // Walk the NULL-terminated array — expect exactly 2 ISCC strings
    let mut count = 0;
    while !unsafe { *result.units.add(count) }.is_null() {
        let s = unsafe { CStr::from_ptr(*result.units.add(count)) }
            .to_str()
            .unwrap();
        assert!(
            s.starts_with("ISCC:"),
            "unit {count} should start with ISCC:"
        );
        count += 1;
    }
    assert_eq!(count, 2, "expected 2 unit strings");

    unsafe { iscc_free_sum_code_result(result) };
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_gen_sum_code_v0_units_disabled() {
    let data = b"Hello World";
    let path = write_ffi_temp_file("sum_units_off", data);
    let path_c = CString::new(path.to_str().unwrap()).unwrap();

    let result = unsafe { iscc_gen_sum_code_v0(path_c.as_ptr(), 64, false, false) };
    assert!(result.ok);
    assert!(
        result.units.is_null(),
        "units should be NULL when add_units=false"
    );

    unsafe { iscc_free_sum_code_result(result) };
    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_gen_sum_code_v0_units_content() {
    let data = b"Hello World";
    let path = write_ffi_temp_file("sum_units_content", data);
    let path_c = CString::new(path.to_str().unwrap()).unwrap();

    let result = unsafe { iscc_gen_sum_code_v0(path_c.as_ptr(), 64, false, true) };
    assert!(result.ok);
    assert!(!result.units.is_null());

    // Decode both unit strings to verify maintype: Data-Code (3) and Instance-Code (4)
    let unit0 = unsafe { CStr::from_ptr(*result.units.add(0)) };
    let unit1 = unsafe { CStr::from_ptr(*result.units.add(1)) };

    let dr0 = unsafe { iscc_decode(unit0.as_ptr()) };
    assert!(dr0.ok);
    let dr1 = unsafe { iscc_decode(unit1.as_ptr()) };
    assert!(dr1.ok);

    // Collect maintypes — should have one Data (3) and one Instance (4)
    let mut maintypes = vec![dr0.maintype, dr1.maintype];
    maintypes.sort();
    assert_eq!(
        maintypes,
        vec![3, 4],
        "expected Data-Code (3) and Instance-Code (4)"
    );

    unsafe { iscc_free_decode_result(dr0) };
    unsafe { iscc_free_decode_result(dr1) };
    unsafe { iscc_free_sum_code_result(result) };
    let _ = std::fs::remove_file(&path);
}

// ── pointer+length inputs go through slice_or_empty ─────────────────────

#[test]
fn test_slices_are_built_only_in_slice_or_empty() {
    // A pointer+length input borrowed anywhere else skips the (NULL, 0) rule
    let source = include_str!("lib.rs");
    assert_eq!(source.matches("slice::from_raw_parts(").count(), 1);
}
