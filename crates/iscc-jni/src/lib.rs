//! JNI bindings for iscc-lib.
//!
//! Exposes all Tier 1 ISCC symbols as JNI-compatible `extern "system"` functions
//! for integration from Java/Kotlin via `System.loadLibrary()`.
//!
//! ## JNI function naming
//!
//! JNI requires specific mangled names that encode the Java package, class, and
//! method. For a Java class `io.iscc.iscc_lib.IsccLib`, the package `iscc_lib`
//! contains an underscore, which JNI encodes as `_1`. So the full prefix is:
//! `Java_io_iscc_iscc_1lib_IsccLib_<methodName>`.
//!
//! ## Error handling
//!
//! For fallible functions, errors are propagated to Java by throwing
//! `IllegalArgumentException` via `env.throw_new()` and returning a
//! type-appropriate default value using `throw_and_default`.
//!
//! ## Streaming hashers
//!
//! `DataHasher` and `InstanceHasher` use the opaque-pointer-as-jlong pattern:
//! `new()` allocates via `Box::into_raw()` and returns the pointer as `jlong`,
//! `update()`/`finalize()` cast back, and `free()` reclaims via `Box::from_raw()`.

use jni::JNIEnv;
use jni::objects::{JByteArray, JClass, JIntArray, JObject, JObjectArray, JString};
use jni::sys::{jboolean, jbyteArray, jint, jintArray, jlong, jobjectArray, jstring};

/// Throw `IllegalArgumentException` in Java and return a type-appropriate default.
///
/// Used by all fallible JNI bridge functions to propagate errors to Java
/// without panicking on the Rust side.
fn throw_and_default<T: Default>(env: &mut JNIEnv, msg: &str) -> T {
    let _ = env.throw_new("java/lang/IllegalArgumentException", msg);
    T::default()
}

/// Throw `IllegalStateException` in Java and return a type-appropriate default.
///
/// Used for operations invalid in the current object state (e.g., calling
/// `update()` or `finalize()` on an already-finalized hasher).
fn throw_state_error<T: Default>(env: &mut JNIEnv, msg: &str) -> T {
    let _ = env.throw_new("java/lang/IllegalStateException", msg);
    T::default()
}

/// Convert a raw `jintArray` to a `Vec<i32>` via typed JNI wrapper.
///
/// Returns the extracted integer array, or an error string on failure.
fn extract_int_array(env: &mut JNIEnv, raw: jintArray) -> Result<Vec<i32>, String> {
    // SAFETY: raw is a valid jintArray from the JVM
    let arr = unsafe { JIntArray::from_raw(raw) };
    let len = env.get_array_length(&arr).map_err(|e| e.to_string())? as usize;
    let mut buf = vec![0i32; len];
    if len > 0 {
        env.get_int_array_region(&arr, 0, &mut buf)
            .map_err(|e| e.to_string())?;
    }
    Ok(buf)
}

/// Convert a raw `jbyteArray` to a `Vec<u8>` via typed JNI wrapper.
///
/// Returns the extracted byte array, or an error string on failure.
fn extract_byte_array(env: &JNIEnv, raw: jbyteArray) -> Result<Vec<u8>, String> {
    // SAFETY: raw is a valid jbyteArray from the JVM
    let arr = unsafe { JByteArray::from_raw(raw) };
    env.convert_byte_array(arr).map_err(|e| e.to_string())
}

/// Extract a `Vec<Vec<i32>>` from a JObjectArray of jintArray.
///
/// Used by gen_video_code_v0 and soft_hash_video_v0.
fn extract_int_array_2d(env: &mut JNIEnv, obj_arr: &JObjectArray) -> Result<Vec<Vec<i32>>, String> {
    let num = env.get_array_length(obj_arr).map_err(|e| e.to_string())? as usize;
    let mut result: Vec<Vec<i32>> = Vec::with_capacity(num);
    for i in 0..num {
        env.push_local_frame(16).map_err(|e| e.to_string())?;
        let obj = env
            .get_object_array_element(obj_arr, i as i32)
            .map_err(|e| e.to_string())?;
        let int_arr: jintArray = obj.as_raw();
        let ints = extract_int_array(env, int_arr)?;
        result.push(ints);
        // SAFETY: frame was pushed at the start of this iteration; all local
        // refs created within the iteration are copies in Rust-owned Vec.
        unsafe {
            env.pop_local_frame(&JObject::null())
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(result)
}

/// Extract a `Vec<String>` from a JObjectArray of String.
///
/// Used by gen_mixed_code_v0 and gen_iscc_code_v0.
fn extract_string_array(env: &mut JNIEnv, obj_arr: &JObjectArray) -> Result<Vec<String>, String> {
    let num = env.get_array_length(obj_arr).map_err(|e| e.to_string())? as usize;
    let mut result: Vec<String> = Vec::with_capacity(num);
    for i in 0..num {
        env.push_local_frame(16).map_err(|e| e.to_string())?;
        let obj = env
            .get_object_array_element(obj_arr, i as i32)
            .map_err(|e| e.to_string())?;
        let jstr = JString::from(obj);
        let s: String = env.get_string(&jstr).map_err(|e| e.to_string())?.into();
        result.push(s);
        // SAFETY: frame was pushed at the start of this iteration; the String
        // data has been copied into Rust-owned `result`.
        unsafe {
            env.pop_local_frame(&JObject::null())
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(result)
}

/// Build a Java `String[]` (jobjectArray) from a `Vec<String>`.
fn build_string_array(env: &mut JNIEnv, strings: &[String]) -> Result<jobjectArray, String> {
    let string_class = env
        .find_class("java/lang/String")
        .map_err(|e| e.to_string())?;
    let arr = env
        .new_object_array(strings.len() as i32, &string_class, JObject::null())
        .map_err(|e| e.to_string())?;
    for (i, s) in strings.iter().enumerate() {
        env.push_local_frame(16).map_err(|e| e.to_string())?;
        let jstr = env.new_string(s).map_err(|e| e.to_string())?;
        env.set_object_array_element(&arr, i as i32, jstr)
            .map_err(|e| e.to_string())?;
        // SAFETY: frame was pushed at the start of this iteration; the string
        // has been set into the result array before popping.
        unsafe {
            env.pop_local_frame(&JObject::null())
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(arr.into_raw())
}

// ── Conformance ─────────────────────────────────────────────────────────────

/// Run all ISCC conformance tests against vendored test vectors.
///
/// Returns `true` (JNI_TRUE) if all tests pass, `false` (JNI_FALSE) otherwise.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_conformanceSelftest(
    _env: JNIEnv,
    _class: JClass,
) -> jboolean {
    let result = iscc_lib::conformance_selftest();
    result as jboolean
}

// ── Gen functions ───────────────────────────────────────────────────────────

/// Generate a Meta-Code from name and optional metadata.
///
/// Returns the ISCC string (e.g., "ISCC:AAA..."). Throws
/// `IllegalArgumentException` on invalid input.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genMetaCodeV0(
    mut env: JNIEnv,
    _class: JClass,
    name: JString,
    description: JString,
    meta: JString,
    bits: jint,
) -> jstring {
    let name_str: String = match env.get_string(&name) {
        Ok(s) => s.into(),
        Err(e) => return throw_and_default(&mut env, &e.to_string()),
    };
    let desc_opt: Option<String> = if description.is_null() {
        None
    } else {
        match env.get_string(&description) {
            Ok(s) => Some(s.into()),
            Err(e) => return throw_and_default(&mut env, &e.to_string()),
        }
    };
    let meta_opt: Option<String> = if meta.is_null() {
        None
    } else {
        match env.get_string(&meta) {
            Ok(s) => Some(s.into()),
            Err(e) => return throw_and_default(&mut env, &e.to_string()),
        }
    };
    match iscc_lib::gen_meta_code_v0(
        &name_str,
        desc_opt.as_deref(),
        meta_opt.as_deref(),
        bits as u32,
    ) {
        Ok(result) => match env.new_string(result.iscc) {
            Ok(s) => s.into_raw(),
            Err(e) => throw_and_default(&mut env, &e.to_string()),
        },
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

/// Generate a Text-Code from plain text content.
///
/// Returns the ISCC string. Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genTextCodeV0(
    mut env: JNIEnv,
    _class: JClass,
    text: JString,
    bits: jint,
) -> jstring {
    let text_str: String = match env.get_string(&text) {
        Ok(s) => s.into(),
        Err(e) => return throw_and_default(&mut env, &e.to_string()),
    };
    match iscc_lib::gen_text_code_v0(&text_str, bits as u32) {
        Ok(result) => match env.new_string(result.iscc) {
            Ok(s) => s.into_raw(),
            Err(e) => throw_and_default(&mut env, &e.to_string()),
        },
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

/// Generate an Image-Code from 1024 grayscale pixel bytes.
///
/// Returns the ISCC string. Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genImageCodeV0(
    mut env: JNIEnv,
    _class: JClass,
    pixels: jbyteArray,
    bits: jint,
) -> jstring {
    let pixel_bytes = match extract_byte_array(&env, pixels) {
        Ok(b) => b,
        Err(e) => return throw_and_default(&mut env, &e),
    };
    match iscc_lib::gen_image_code_v0(&pixel_bytes, bits as u32) {
        Ok(result) => match env.new_string(result.iscc) {
            Ok(s) => s.into_raw(),
            Err(e) => throw_and_default(&mut env, &e.to_string()),
        },
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

/// Generate an Audio-Code from a Chromaprint feature vector.
///
/// Takes a `jintArray` of signed 32-bit features. Returns the ISCC string.
/// Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genAudioCodeV0(
    mut env: JNIEnv,
    _class: JClass,
    cv: jintArray,
    bits: jint,
) -> jstring {
    let buf = match extract_int_array(&mut env, cv) {
        Ok(b) => b,
        Err(e) => return throw_and_default(&mut env, &e),
    };
    match iscc_lib::gen_audio_code_v0(&buf, bits as u32) {
        Ok(result) => match env.new_string(result.iscc) {
            Ok(s) => s.into_raw(),
            Err(e) => throw_and_default(&mut env, &e.to_string()),
        },
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

/// Generate a Video-Code from frame signature data.
///
/// Takes a `jobjectArray` of `jintArray` frame signatures. Returns the ISCC string.
/// Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genVideoCodeV0(
    mut env: JNIEnv,
    _class: JClass,
    frame_sigs: JObjectArray,
    bits: jint,
) -> jstring {
    let frames = match extract_int_array_2d(&mut env, &frame_sigs) {
        Ok(f) => f,
        Err(e) => return throw_and_default(&mut env, &e),
    };
    match iscc_lib::gen_video_code_v0(&frames, bits as u32) {
        Ok(result) => match env.new_string(result.iscc) {
            Ok(s) => s.into_raw(),
            Err(e) => throw_and_default(&mut env, &e.to_string()),
        },
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

/// Generate a Mixed-Code from multiple Content-Code strings.
///
/// Takes a `jobjectArray` of `String` ISCC codes. Returns the ISCC string.
/// Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genMixedCodeV0(
    mut env: JNIEnv,
    _class: JClass,
    codes: JObjectArray,
    bits: jint,
) -> jstring {
    let code_strs = match extract_string_array(&mut env, &codes) {
        Ok(s) => s,
        Err(e) => return throw_and_default(&mut env, &e),
    };
    let refs: Vec<&str> = code_strs.iter().map(|s| s.as_str()).collect();
    match iscc_lib::gen_mixed_code_v0(&refs, bits as u32) {
        Ok(result) => match env.new_string(result.iscc) {
            Ok(s) => s.into_raw(),
            Err(e) => throw_and_default(&mut env, &e.to_string()),
        },
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

/// Generate a Data-Code from raw byte data.
///
/// Returns the ISCC string. Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genDataCodeV0(
    mut env: JNIEnv,
    _class: JClass,
    data: jbyteArray,
    bits: jint,
) -> jstring {
    let bytes = match extract_byte_array(&env, data) {
        Ok(b) => b,
        Err(e) => return throw_and_default(&mut env, &e),
    };
    match iscc_lib::gen_data_code_v0(&bytes, bits as u32) {
        Ok(result) => match env.new_string(result.iscc) {
            Ok(s) => s.into_raw(),
            Err(e) => throw_and_default(&mut env, &e.to_string()),
        },
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

/// Generate an Instance-Code from raw byte data.
///
/// Returns the ISCC string. Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genInstanceCodeV0(
    mut env: JNIEnv,
    _class: JClass,
    data: jbyteArray,
    bits: jint,
) -> jstring {
    let bytes = match extract_byte_array(&env, data) {
        Ok(b) => b,
        Err(e) => return throw_and_default(&mut env, &e),
    };
    match iscc_lib::gen_instance_code_v0(&bytes, bits as u32) {
        Ok(result) => match env.new_string(result.iscc) {
            Ok(s) => s.into_raw(),
            Err(e) => throw_and_default(&mut env, &e.to_string()),
        },
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

/// Generate a composite ISCC-CODE from individual unit codes.
///
/// Takes a `jobjectArray` of `String` ISCC unit codes and a `wide` flag.
/// Returns the ISCC string. Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genIsccCodeV0(
    mut env: JNIEnv,
    _class: JClass,
    codes: JObjectArray,
    wide: jboolean,
) -> jstring {
    let code_strs = match extract_string_array(&mut env, &codes) {
        Ok(s) => s,
        Err(e) => return throw_and_default(&mut env, &e),
    };
    let refs: Vec<&str> = code_strs.iter().map(|s| s.as_str()).collect();
    match iscc_lib::gen_iscc_code_v0(&refs, wide != 0) {
        Ok(result) => match env.new_string(result.iscc) {
            Ok(s) => s.into_raw(),
            Err(e) => throw_and_default(&mut env, &e.to_string()),
        },
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

// ── Text utilities ──────────────────────────────────────────────────────────

/// Clean and normalize text for display.
///
/// Applies NFKC normalization, removes control characters (except newlines),
/// normalizes `\r\n` to `\n`, collapses consecutive empty lines, and strips
/// leading/trailing whitespace.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_textClean(
    mut env: JNIEnv,
    _class: JClass,
    text: JString,
) -> jstring {
    let text_str: String = match env.get_string(&text) {
        Ok(s) => s.into(),
        Err(e) => return throw_and_default(&mut env, &e.to_string()),
    };
    let result = iscc_lib::text_clean(&text_str);
    match env.new_string(result) {
        Ok(s) => s.into_raw(),
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

/// Remove newlines and collapse whitespace to single spaces.
///
/// Converts multi-line text into a single normalized line.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_textRemoveNewlines(
    mut env: JNIEnv,
    _class: JClass,
    text: JString,
) -> jstring {
    let text_str: String = match env.get_string(&text) {
        Ok(s) => s.into(),
        Err(e) => return throw_and_default(&mut env, &e.to_string()),
    };
    let result = iscc_lib::text_remove_newlines(&text_str);
    match env.new_string(result) {
        Ok(s) => s.into_raw(),
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

/// Trim text so its UTF-8 encoded size does not exceed `nbytes`.
///
/// Multi-byte characters that would be split are dropped entirely.
/// Leading/trailing whitespace is stripped from the result.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_textTrim(
    mut env: JNIEnv,
    _class: JClass,
    text: JString,
    nbytes: jint,
) -> jstring {
    if nbytes < 0 {
        return throw_and_default(&mut env, "nbytes must be non-negative");
    }
    let text_str: String = match env.get_string(&text) {
        Ok(s) => s.into(),
        Err(e) => return throw_and_default(&mut env, &e.to_string()),
    };
    let result = iscc_lib::text_trim(&text_str, nbytes as usize);
    match env.new_string(result) {
        Ok(s) => s.into_raw(),
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

/// Normalize and simplify text for similarity hashing.
///
/// Applies NFD normalization, lowercasing, removes whitespace and characters
/// in Unicode categories C (control), M (mark), and P (punctuation), then
/// recombines with NFKC normalization.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_textCollapse(
    mut env: JNIEnv,
    _class: JClass,
    text: JString,
) -> jstring {
    let text_str: String = match env.get_string(&text) {
        Ok(s) => s.into(),
        Err(e) => return throw_and_default(&mut env, &e.to_string()),
    };
    let result = iscc_lib::text_collapse(&text_str);
    match env.new_string(result) {
        Ok(s) => s.into_raw(),
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

// ── Encoding ────────────────────────────────────────────────────────────────

/// Encode bytes as base64url (RFC 4648 §5, no padding).
///
/// Returns a URL-safe base64 encoded string without padding characters.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_encodeBase64(
    mut env: JNIEnv,
    _class: JClass,
    data: jbyteArray,
) -> jstring {
    let bytes = match extract_byte_array(&env, data) {
        Ok(b) => b,
        Err(e) => return throw_and_default(&mut env, &e),
    };
    let result = iscc_lib::encode_base64(&bytes);
    match env.new_string(result) {
        Ok(s) => s.into_raw(),
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

// ── Codec ───────────────────────────────────────────────────────────────────

/// Decompose a composite ISCC-CODE into individual ISCC-UNITs.
///
/// Returns a `String[]` of base32-encoded ISCC-UNIT strings (without prefix).
/// Throws `IllegalArgumentException` on invalid input.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_isccDecompose(
    mut env: JNIEnv,
    _class: JClass,
    iscc_code: JString,
) -> jobjectArray {
    let code_str: String = match env.get_string(&iscc_code) {
        Ok(s) => s.into(),
        Err(e) => return throw_and_default(&mut env, &e.to_string()),
    };
    let units = match iscc_lib::iscc_decompose(&code_str) {
        Ok(u) => u,
        Err(e) => return throw_and_default(&mut env, &e.to_string()),
    };
    match build_string_array(&mut env, &units) {
        Ok(arr) => arr,
        Err(e) => throw_and_default(&mut env, &e),
    }
}

// ── Sliding window ──────────────────────────────────────────────────────────

/// Generate sliding window n-grams from a string.
///
/// Returns a `String[]` of overlapping substrings of `width` Unicode characters.
/// Throws `IllegalArgumentException` if width is less than 2.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_slidingWindow(
    mut env: JNIEnv,
    _class: JClass,
    seq: JString,
    width: jint,
) -> jobjectArray {
    if width < 0 {
        return throw_and_default(&mut env, "width must be non-negative");
    }
    let seq_str: String = match env.get_string(&seq) {
        Ok(s) => s.into(),
        Err(e) => return throw_and_default(&mut env, &e.to_string()),
    };
    let ngrams = match iscc_lib::sliding_window(&seq_str, width as usize) {
        Ok(v) => v,
        Err(e) => return throw_and_default(&mut env, &e.to_string()),
    };
    match build_string_array(&mut env, &ngrams) {
        Ok(arr) => arr,
        Err(e) => throw_and_default(&mut env, &e),
    }
}

// ── Algorithm primitives ────────────────────────────────────────────────────

/// Compute a SimHash from a sequence of equal-length hash digests.
///
/// Takes a `byte[][]` (jobjectArray of jbyteArray). Returns `byte[]` with the
/// similarity-preserving hash. Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_algSimhash(
    mut env: JNIEnv,
    _class: JClass,
    hash_digests: JObjectArray,
) -> jbyteArray {
    let num = match env.get_array_length(&hash_digests) {
        Ok(l) => l as usize,
        Err(e) => return throw_and_default(&mut env, &e.to_string()),
    };
    let mut digests: Vec<Vec<u8>> = Vec::with_capacity(num);
    for i in 0..num {
        if env.push_local_frame(16).is_err() {
            return throw_and_default(&mut env, "failed to push local frame");
        }
        let obj = match env.get_object_array_element(&hash_digests, i as i32) {
            Ok(o) => o,
            Err(e) => return throw_and_default(&mut env, &e.to_string()),
        };
        let bytes = match extract_byte_array(&env, obj.as_raw()) {
            Ok(b) => b,
            Err(e) => return throw_and_default(&mut env, &e),
        };
        digests.push(bytes);
        // SAFETY: frame was pushed at the start of this iteration; byte data
        // has been copied into Rust-owned Vec.
        let _ = unsafe { env.pop_local_frame(&JObject::null()) };
    }
    let refs: Vec<&[u8]> = digests.iter().map(|d| d.as_slice()).collect();
    match iscc_lib::alg_simhash(&refs) {
        Ok(result) => match env.byte_array_from_slice(&result) {
            Ok(a) => a.into_raw(),
            Err(e) => throw_and_default(&mut env, &e.to_string()),
        },
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

/// Compute a 256-bit MinHash digest from 32-bit integer features.
///
/// Takes `int[]` features (Java `int` is signed, cast to `u32`).
/// Returns `byte[]` with 32-byte digest.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_algMinhash256(
    mut env: JNIEnv,
    _class: JClass,
    features: jintArray,
) -> jbyteArray {
    let buf = match extract_int_array(&mut env, features) {
        Ok(b) => b,
        Err(e) => return throw_and_default(&mut env, &e),
    };
    // Java has no unsigned int — cast jint (i32) to u32
    let u32_features: Vec<u32> = buf.iter().map(|&v| v as u32).collect();
    let result = iscc_lib::alg_minhash_256(&u32_features);
    match env.byte_array_from_slice(&result) {
        Ok(a) => a.into_raw(),
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

/// Split data into content-defined chunks using gear rolling hash.
///
/// Returns `byte[][]`. When `utf32` is true, aligns cut points to 4-byte
/// boundaries. Default `avg_chunk_size` is 1024.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_algCdcChunks(
    mut env: JNIEnv,
    _class: JClass,
    data: jbyteArray,
    utf32: jboolean,
    avg_chunk_size: jint,
) -> jobjectArray {
    if avg_chunk_size < 0 {
        return throw_and_default(&mut env, "avg_chunk_size must be non-negative");
    }
    let bytes = match extract_byte_array(&env, data) {
        Ok(b) => b,
        Err(e) => return throw_and_default(&mut env, &e),
    };
    let chunks = iscc_lib::alg_cdc_chunks(&bytes, utf32 != 0, avg_chunk_size as u32);
    let byte_array_class = match env.find_class("[B") {
        Ok(c) => c,
        Err(e) => return throw_and_default(&mut env, &e.to_string()),
    };
    let arr = match env.new_object_array(chunks.len() as i32, &byte_array_class, JObject::null()) {
        Ok(a) => a,
        Err(e) => return throw_and_default(&mut env, &e.to_string()),
    };
    for (i, chunk) in chunks.iter().enumerate() {
        if env.push_local_frame(16).is_err() {
            return throw_and_default(&mut env, "failed to push local frame");
        }
        let barr = match env.byte_array_from_slice(chunk) {
            Ok(a) => a,
            Err(e) => return throw_and_default(&mut env, &e.to_string()),
        };
        if let Err(e) = env.set_object_array_element(&arr, i as i32, &barr) {
            return throw_and_default(&mut env, &e.to_string());
        }
        // SAFETY: frame was pushed at the start of this iteration; the byte
        // array has been set into the result array before popping.
        let _ = unsafe { env.pop_local_frame(&JObject::null()) };
    }
    arr.into_raw()
}

/// Compute a similarity-preserving hash from video frame signatures.
///
/// Takes `int[][]` frame signatures and `bits`. Returns `byte[]` of length
/// `bits / 8`. Throws `IllegalArgumentException` if input is empty.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_softHashVideoV0(
    mut env: JNIEnv,
    _class: JClass,
    frame_sigs: JObjectArray,
    bits: jint,
) -> jbyteArray {
    let frames = match extract_int_array_2d(&mut env, &frame_sigs) {
        Ok(f) => f,
        Err(e) => return throw_and_default(&mut env, &e),
    };
    match iscc_lib::soft_hash_video_v0(&frames, bits as u32) {
        Ok(result) => match env.byte_array_from_slice(&result) {
            Ok(a) => a.into_raw(),
            Err(e) => throw_and_default(&mut env, &e.to_string()),
        },
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

// ── Streaming hashers ───────────────────────────────────────────────────────

/// JNI wrapper around `iscc_lib::DataHasher` with finalize-once semantics.
struct JniDataHasher {
    inner: Option<iscc_lib::DataHasher>,
}

/// JNI wrapper around `iscc_lib::InstanceHasher` with finalize-once semantics.
struct JniInstanceHasher {
    inner: Option<iscc_lib::InstanceHasher>,
}

/// Create a new streaming Data-Code hasher.
///
/// Returns an opaque `jlong` handle. The caller must eventually call
/// `dataHasherFree` to release the memory.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_dataHasherNew(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let wrapper = Box::new(JniDataHasher {
        inner: Some(iscc_lib::DataHasher::new()),
    });
    Box::into_raw(wrapper) as jlong
}

/// Push data into a streaming DataHasher.
///
/// Throws `IllegalStateException` if the hasher has already been finalized.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_dataHasherUpdate(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    data: jbyteArray,
) {
    let bytes = match extract_byte_array(&env, data) {
        Ok(b) => b,
        Err(e) => {
            throw_and_default::<()>(&mut env, &e);
            return;
        }
    };
    // SAFETY: ptr was produced by Box::into_raw() in dataHasherNew
    let wrapper = unsafe { &mut *(ptr as *mut JniDataHasher) };
    let Some(inner) = wrapper.inner.as_mut() else {
        throw_state_error::<()>(&mut env, "DataHasher already finalized");
        return;
    };
    inner.update(&bytes);
}

/// Finalize a streaming DataHasher and return an ISCC string.
///
/// Consumes the inner hasher state. After this call, subsequent `update`
/// or `finalize` calls will throw. The caller must still call
/// `dataHasherFree` to release the wrapper.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_dataHasherFinalize(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    bits: jint,
) -> jstring {
    // SAFETY: ptr was produced by Box::into_raw() in dataHasherNew
    let wrapper = unsafe { &mut *(ptr as *mut JniDataHasher) };
    let Some(inner) = wrapper.inner.take() else {
        return throw_state_error(&mut env, "DataHasher already finalized");
    };
    match inner.finalize(bits as u32) {
        Ok(result) => match env.new_string(result.iscc) {
            Ok(s) => s.into_raw(),
            Err(e) => throw_and_default(&mut env, &e.to_string()),
        },
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

/// Free a DataHasher previously created by `dataHasherNew`.
///
/// Zero/null handle is a no-op. Each handle must be freed exactly once.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_dataHasherFree(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        // SAFETY: ptr was produced by Box::into_raw() in dataHasherNew
        drop(unsafe { Box::from_raw(ptr as *mut JniDataHasher) });
    }
}

/// Create a new streaming Instance-Code hasher.
///
/// Returns an opaque `jlong` handle. The caller must eventually call
/// `instanceHasherFree` to release the memory.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_instanceHasherNew(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let wrapper = Box::new(JniInstanceHasher {
        inner: Some(iscc_lib::InstanceHasher::new()),
    });
    Box::into_raw(wrapper) as jlong
}

/// Push data into a streaming InstanceHasher.
///
/// Throws `IllegalStateException` if the hasher has already been finalized.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_instanceHasherUpdate(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    data: jbyteArray,
) {
    let bytes = match extract_byte_array(&env, data) {
        Ok(b) => b,
        Err(e) => {
            throw_and_default::<()>(&mut env, &e);
            return;
        }
    };
    // SAFETY: ptr was produced by Box::into_raw() in instanceHasherNew
    let wrapper = unsafe { &mut *(ptr as *mut JniInstanceHasher) };
    let Some(inner) = wrapper.inner.as_mut() else {
        throw_state_error::<()>(&mut env, "InstanceHasher already finalized");
        return;
    };
    inner.update(&bytes);
}

/// Finalize a streaming InstanceHasher and return an ISCC string.
///
/// Consumes the inner hasher state. After this call, subsequent `update`
/// or `finalize` calls will throw. The caller must still call
/// `instanceHasherFree` to release the wrapper.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_instanceHasherFinalize(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    bits: jint,
) -> jstring {
    // SAFETY: ptr was produced by Box::into_raw() in instanceHasherNew
    let wrapper = unsafe { &mut *(ptr as *mut JniInstanceHasher) };
    let Some(inner) = wrapper.inner.take() else {
        return throw_state_error(&mut env, "InstanceHasher already finalized");
    };
    match inner.finalize(bits as u32) {
        Ok(result) => match env.new_string(result.iscc) {
            Ok(s) => s.into_raw(),
            Err(e) => throw_and_default(&mut env, &e.to_string()),
        },
        Err(e) => throw_and_default(&mut env, &e.to_string()),
    }
}

/// Free an InstanceHasher previously created by `instanceHasherNew`.
///
/// Zero/null handle is a no-op. Each handle must be freed exactly once.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_instanceHasherFree(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        // SAFETY: ptr was produced by Box::into_raw() in instanceHasherNew
        drop(unsafe { Box::from_raw(ptr as *mut JniInstanceHasher) });
    }
}
