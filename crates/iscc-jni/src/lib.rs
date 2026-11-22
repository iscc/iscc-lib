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
//! Each fallible function acquires an [`Env`] via `EnvUnowned::with_env` and
//! resolves the outcome with the `ThrowRuntimeExAndDefault` policy. Errors are
//! propagated to Java by `throw_and_default`, which throws via `env.throw_new()`
//! inside the closure and returns a type-appropriate default value. The error
//! type picks the class: a JVM allocation failure (`JniError::NoMemory`) throws
//! `OutOfMemoryError`, every other error `IllegalArgumentException`. When a
//! failed JNI call already left an exception pending, the helpers throw nothing
//! and Java sees that exception. The policy never overwrites a pending
//! exception, so it only fires for a genuine panic or unhandled error.
//!
//! ## Streaming hashers
//!
//! `DataHasher` and `InstanceHasher` use the opaque-pointer-as-jlong pattern:
//! `new()` allocates via `Box::into_raw()` and returns the pointer as `jlong`,
//! `update()`/`finalize()` cast back, and `free()` reclaims via `Box::from_raw()`.

use jni::errors::ThrowRuntimeExAndDefault;
use jni::objects::{JByteArray, JClass, JIntArray, JObject, JObjectArray, JString};
use jni::refs::Reference as _;
use jni::strings::JNIString;
use jni::sys::{jboolean, jint, jlong};
use jni::{Env, EnvUnowned, JValue, jni_sig, jni_str};

/// An error that a JNI bridge function reports to Java via `throw_and_default`.
///
/// The error type decides the Java exception class, so no call site can
/// misclassify an error: a JVM allocation failure maps to `OutOfMemoryError`,
/// everything else to `IllegalArgumentException` with the `Display` text.
trait JavaError: std::fmt::Display {
    /// Return `true` if the error reports a failed JVM allocation.
    fn is_out_of_memory(&self) -> bool {
        false
    }
}

impl JavaError for jni::errors::Error {
    /// jni catches and clears the JVM's `OutOfMemoryError` and returns `NoMemory`.
    fn is_out_of_memory(&self) -> bool {
        matches!(
            self,
            jni::errors::Error::JniCall(jni::errors::JniError::NoMemory)
        )
    }
}

impl JavaError for iscc_lib::IsccError {}

impl JavaError for str {}

/// Throw the Java exception for `err` and return a type-appropriate default.
///
/// Used by all fallible JNI bridge functions to propagate errors to Java
/// without panicking on the Rust side. If a Java exception is already pending
/// (a failed JNI call left it), nothing is thrown and the pending exception
/// reaches Java unchanged — `ThrowNew` over a pending exception breaks the JNI
/// spec. Otherwise a JVM allocation failure throws a fresh `OutOfMemoryError`
/// (jni already discarded the original) and every other error throws
/// `IllegalArgumentException`. The exception stays pending; returning `Ok`
/// makes `resolve()` hand the default value back to the JVM, which then raises
/// the pending exception in Java.
fn throw_and_default<T: Default, E: JavaError + ?Sized>(
    env: &mut Env,
    err: &E,
) -> jni::errors::Result<T> {
    if env.exception_check() {
        return Ok(T::default());
    }
    let class = if err.is_out_of_memory() {
        jni_str!("java/lang/OutOfMemoryError")
    } else {
        jni_str!("java/lang/IllegalArgumentException")
    };
    let _ = env.throw_new(class, JNIString::from(err.to_string()));
    Ok(T::default())
}

/// Narrow a Java `int` to `u32`, rejecting a negative value.
///
/// Java has no unsigned `int`, so a bare `as u32` would turn `-1` into
/// `4294967295`. On a negative value this throws `IllegalArgumentException`
/// naming the parameter and returns `Err` holding the default value the
/// bridge function must return.
fn non_negative<T: Default>(
    env: &mut Env,
    value: jint,
    name: &str,
) -> Result<u32, jni::errors::Result<T>> {
    u32::try_from(value)
        .map_err(|_| throw_and_default(env, format!("{name} must be non-negative").as_str()))
}

/// Throw `IllegalStateException` in Java and return a type-appropriate default.
///
/// Used for operations invalid in the current object state (e.g., calling
/// `update()` or `finalize()` on an already-finalized hasher). Like
/// `throw_and_default`, it throws nothing while a Java exception is pending.
fn throw_state_error<T: Default>(env: &mut Env, msg: &str) -> jni::errors::Result<T> {
    if env.exception_check() {
        return Ok(T::default());
    }
    let _ = env.throw_new(
        jni_str!("java/lang/IllegalStateException"),
        JNIString::from(msg),
    );
    Ok(T::default())
}

/// Extract a `Vec<i32>` from a `JIntArray`.
fn extract_int_array(env: &Env, arr: &JIntArray) -> jni::errors::Result<Vec<i32>> {
    let len = arr.len(env)?;
    let mut buf = vec![0i32; len];
    if len > 0 {
        arr.get_region(env, 0, &mut buf)?;
    }
    Ok(buf)
}

/// Extract a `Vec<Vec<i32>>` from a `JObjectArray` of `JIntArray`.
///
/// Used by gen_video_code_v0 and soft_hash_video_v0.
fn extract_int_array_2d(
    env: &mut Env,
    obj_arr: &JObjectArray<JIntArray>,
) -> jni::errors::Result<Vec<Vec<i32>>> {
    let num = obj_arr.len(env)?;
    let mut result: Vec<Vec<i32>> = Vec::with_capacity(num);
    for i in 0..num {
        // Per-iteration local frame: all local refs created within the
        // iteration are copies in a Rust-owned Vec before the frame pops.
        let ints = env.with_local_frame(16, |env| -> jni::errors::Result<Vec<i32>> {
            let int_arr = obj_arr.get_element(env, i)?;
            extract_int_array(env, &int_arr)
        })?;
        result.push(ints);
    }
    Ok(result)
}

/// Extract a `Vec<String>` from a `JObjectArray` of `JString`.
///
/// Used by gen_mixed_code_v0 and gen_iscc_code_v0.
fn extract_string_array(
    env: &mut Env,
    obj_arr: &JObjectArray<JString>,
) -> jni::errors::Result<Vec<String>> {
    let num = obj_arr.len(env)?;
    let mut result: Vec<String> = Vec::with_capacity(num);
    for i in 0..num {
        // Per-iteration local frame: the String data is copied into the
        // Rust-owned result before the frame pops.
        let s = env.with_local_frame(16, |env| -> jni::errors::Result<String> {
            let jstr = obj_arr.get_element(env, i)?;
            jstr.try_to_string(env)
        })?;
        result.push(s);
    }
    Ok(result)
}

/// Build a Java `String[]` from a slice of Rust strings.
fn build_string_array<'local>(
    env: &mut Env<'local>,
    strings: &[String],
) -> jni::errors::Result<JObjectArray<'local, JString<'local>>> {
    let arr = JObjectArray::<JString>::new(env, strings.len(), &JString::default())?;
    for (i, s) in strings.iter().enumerate() {
        // Per-iteration local frame: the string is stored into the result
        // array (a JVM-side reference) before the frame pops.
        env.with_local_frame(16, |env| -> jni::errors::Result<()> {
            let jstr = env.new_string(s)?;
            arr.set_element(env, i, &jstr)
        })?;
    }
    Ok(arr)
}

// ── Conformance ─────────────────────────────────────────────────────────────

/// Run all ISCC conformance tests against vendored test vectors.
///
/// Returns `true` (JNI_TRUE) if all tests pass, `false` (JNI_FALSE) otherwise.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_conformanceSelftest(
    _env: EnvUnowned,
    _class: JClass,
) -> jboolean {
    iscc_lib::conformance_selftest()
}

// ── Gen functions ───────────────────────────────────────────────────────────

/// Generate a Meta-Code from name and optional metadata.
///
/// Returns the ISCC string (e.g., "ISCC:AAA..."). Throws
/// `IllegalArgumentException` on invalid input.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genMetaCodeV0<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    name: JString<'local>,
    description: JString<'local>,
    meta: JString<'local>,
    bits: jint,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            let bits = match non_negative(env, bits, "bits") {
                Ok(v) => v,
                Err(default) => return default,
            };
            let name_str: String = match name.try_to_string(env) {
                Ok(s) => s,
                Err(e) => return throw_and_default(env, &e),
            };
            let desc_opt: Option<String> = if description.is_null() {
                None
            } else {
                match description.try_to_string(env) {
                    Ok(s) => Some(s),
                    Err(e) => return throw_and_default(env, &e),
                }
            };
            let meta_opt: Option<String> = if meta.is_null() {
                None
            } else {
                match meta.try_to_string(env) {
                    Ok(s) => Some(s),
                    Err(e) => return throw_and_default(env, &e),
                }
            };
            match iscc_lib::gen_meta_code_v0(
                &name_str,
                desc_opt.as_deref(),
                meta_opt.as_deref(),
                bits,
            ) {
                Ok(result) => match env.new_string(result.iscc) {
                    Ok(s) => Ok(s),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Generate a Text-Code from plain text content.
///
/// Returns the ISCC string. Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genTextCodeV0<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    text: JString<'local>,
    bits: jint,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            let bits = match non_negative(env, bits, "bits") {
                Ok(v) => v,
                Err(default) => return default,
            };
            let text_str: String = match text.try_to_string(env) {
                Ok(s) => s,
                Err(e) => return throw_and_default(env, &e),
            };
            match iscc_lib::gen_text_code_v0(&text_str, bits) {
                Ok(result) => match env.new_string(result.iscc) {
                    Ok(s) => Ok(s),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Generate an Image-Code from 1024 grayscale pixel bytes.
///
/// Returns the ISCC string. Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genImageCodeV0<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    pixels: JByteArray<'local>,
    bits: jint,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            let bits = match non_negative(env, bits, "bits") {
                Ok(v) => v,
                Err(default) => return default,
            };
            let pixel_bytes = match env.convert_byte_array(&pixels) {
                Ok(b) => b,
                Err(e) => return throw_and_default(env, &e),
            };
            match iscc_lib::gen_image_code_v0(&pixel_bytes, bits) {
                Ok(result) => match env.new_string(result.iscc) {
                    Ok(s) => Ok(s),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Generate an Audio-Code from a Chromaprint feature vector.
///
/// Takes an `int[]` of signed 32-bit features. Returns the ISCC string.
/// Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genAudioCodeV0<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    cv: JIntArray<'local>,
    bits: jint,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            let bits = match non_negative(env, bits, "bits") {
                Ok(v) => v,
                Err(default) => return default,
            };
            let buf = match extract_int_array(env, &cv) {
                Ok(b) => b,
                Err(e) => return throw_and_default(env, &e),
            };
            match iscc_lib::gen_audio_code_v0(&buf, bits) {
                Ok(result) => match env.new_string(result.iscc) {
                    Ok(s) => Ok(s),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Generate a Video-Code from frame signature data.
///
/// Takes an `int[][]` of frame signatures. Returns the ISCC string.
/// Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genVideoCodeV0<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    frame_sigs: JObjectArray<'local, JIntArray<'local>>,
    bits: jint,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            let bits = match non_negative(env, bits, "bits") {
                Ok(v) => v,
                Err(default) => return default,
            };
            let frames = match extract_int_array_2d(env, &frame_sigs) {
                Ok(f) => f,
                Err(e) => return throw_and_default(env, &e),
            };
            match iscc_lib::gen_video_code_v0(&frames, bits) {
                Ok(result) => match env.new_string(result.iscc) {
                    Ok(s) => Ok(s),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Generate a Mixed-Code from multiple Content-Code strings.
///
/// Takes a `String[]` of ISCC codes. Returns the ISCC string.
/// Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genMixedCodeV0<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    codes: JObjectArray<'local, JString<'local>>,
    bits: jint,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            let bits = match non_negative(env, bits, "bits") {
                Ok(v) => v,
                Err(default) => return default,
            };
            let code_strs = match extract_string_array(env, &codes) {
                Ok(s) => s,
                Err(e) => return throw_and_default(env, &e),
            };
            let refs: Vec<&str> = code_strs.iter().map(|s| s.as_str()).collect();
            match iscc_lib::gen_mixed_code_v0(&refs, bits) {
                Ok(result) => match env.new_string(result.iscc) {
                    Ok(s) => Ok(s),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Generate a Data-Code from raw byte data.
///
/// Returns the ISCC string. Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genDataCodeV0<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    data: JByteArray<'local>,
    bits: jint,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            let bits = match non_negative(env, bits, "bits") {
                Ok(v) => v,
                Err(default) => return default,
            };
            let bytes = match env.convert_byte_array(&data) {
                Ok(b) => b,
                Err(e) => return throw_and_default(env, &e),
            };
            match iscc_lib::gen_data_code_v0(&bytes, bits) {
                Ok(result) => match env.new_string(result.iscc) {
                    Ok(s) => Ok(s),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Generate an Instance-Code from raw byte data.
///
/// Returns the ISCC string. Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genInstanceCodeV0<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    data: JByteArray<'local>,
    bits: jint,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            let bits = match non_negative(env, bits, "bits") {
                Ok(v) => v,
                Err(default) => return default,
            };
            let bytes = match env.convert_byte_array(&data) {
                Ok(b) => b,
                Err(e) => return throw_and_default(env, &e),
            };
            match iscc_lib::gen_instance_code_v0(&bytes, bits) {
                Ok(result) => match env.new_string(result.iscc) {
                    Ok(s) => Ok(s),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Generate a composite ISCC-CODE from individual unit codes.
///
/// Takes a `String[]` of ISCC unit codes and a `wide` flag.
/// Returns the ISCC string. Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genIsccCodeV0<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    codes: JObjectArray<'local, JString<'local>>,
    wide: jboolean,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            let code_strs = match extract_string_array(env, &codes) {
                Ok(s) => s,
                Err(e) => return throw_and_default(env, &e),
            };
            let refs: Vec<&str> = code_strs.iter().map(|s| s.as_str()).collect();
            match iscc_lib::gen_iscc_code_v0(&refs, wide) {
                Ok(result) => match env.new_string(result.iscc) {
                    Ok(s) => Ok(s),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Generate an ISCC-SUM code from a file path.
///
/// Reads the file once, generating both Data-Code and Instance-Code in a
/// single pass, then composes the final ISCC-CODE. Returns a Java
/// `SumCodeResult` object with `iscc`, `datahash`, `filesize`, and
/// optionally `units` fields.
/// Throws `IllegalArgumentException` on invalid input or file I/O error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genSumCodeV0<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    path: JString<'local>,
    bits: jint,
    wide: jboolean,
    add_units: jboolean,
) -> JObject<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JObject<'local>> {
            let bits = match non_negative(env, bits, "bits") {
                Ok(v) => v,
                Err(default) => return default,
            };
            let path_str: String = match path.try_to_string(env) {
                Ok(s) => s,
                Err(e) => return throw_and_default(env, &e),
            };
            let result = match iscc_lib::gen_sum_code_v0(
                std::path::Path::new(&path_str),
                bits,
                wide,
                add_units,
            ) {
                Ok(r) => r,
                Err(e) => return throw_and_default(env, &e),
            };
            let iscc_jstr = match env.new_string(&result.iscc) {
                Ok(s) => s,
                Err(e) => return throw_and_default(env, &e),
            };
            let datahash_jstr = match env.new_string(&result.datahash) {
                Ok(s) => s,
                Err(e) => return throw_and_default(env, &e),
            };
            // Convert units: Some(Vec<String>) → String[], None → null
            let units_obj = match result.units {
                Some(units) => match build_string_array(env, &units) {
                    Ok(arr) => JObject::from(arr),
                    Err(e) => return throw_and_default(env, &e),
                },
                None => JObject::null(),
            };
            let class = match env.find_class(jni_str!("io/iscc/iscc_lib/SumCodeResult")) {
                Ok(c) => c,
                Err(e) => return throw_and_default(env, &e),
            };
            match env.new_object(
                class,
                jni_sig!("(Ljava/lang/String;Ljava/lang/String;J[Ljava/lang/String;)V"),
                &[
                    JValue::Object(&iscc_jstr),
                    JValue::Object(&datahash_jstr),
                    JValue::Long(result.filesize as jlong),
                    JValue::Object(&units_obj),
                ],
            ) {
                Ok(obj) => Ok(obj),
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Mint an experimental ISCC-IDv1 from an explicit timestamp, hub ID, and realm.
///
/// `timestamp` is microseconds since the Unix epoch (`< 2^52`), `hubId` a hub
/// identifier (0–4095), and `realm` a realm identifier (`0` = testnet,
/// `1` = mainnet). Returns the ISCC string. Throws `IllegalArgumentException`
/// on out-of-range values.
///
/// The three parameters are validated against their SEMANTIC thresholds in
/// `timestamp` → `hubId` → `realm` order — the normative cross-surface
/// validation order, where the first failing check wins — *before* narrowing
/// to the core's `u64`/`u16`/`u8`. Validating in the wrapper (rather than
/// deferring wide-narrowing guards to the core) ensures multi-invalid inputs
/// report the same field the reference implementation reports.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_genIsccIdV1<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    timestamp: jlong,
    hub_id: jint,
    realm: jint,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            if !(0..4_503_599_627_370_496).contains(&timestamp) {
                return throw_and_default(env, "timestamp must be in range 0 to 2^52 - 1");
            }
            if !(0..4096).contains(&hub_id) {
                return throw_and_default(env, "hubId (hub) must be in range 0-4095");
            }
            if !(0..2).contains(&realm) {
                return throw_and_default(env, "realm must be 0 (testnet) or 1 (mainnet)");
            }
            match iscc_lib::gen_iscc_id_v1(timestamp as u64, hub_id as u16, realm as u8) {
                Ok(result) => match env.new_string(result.iscc) {
                    Ok(s) => Ok(s),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

// ── Text utilities ──────────────────────────────────────────────────────────

/// Clean and normalize text for display.
///
/// Applies NFKC normalization, removes control characters (except newlines),
/// normalizes `\r\n` to `\n`, collapses consecutive empty lines, and strips
/// leading/trailing whitespace.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_textClean<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    text: JString<'local>,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            let text_str: String = match text.try_to_string(env) {
                Ok(s) => s,
                Err(e) => return throw_and_default(env, &e),
            };
            let result = iscc_lib::text_clean(&text_str);
            match env.new_string(result) {
                Ok(s) => Ok(s),
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Remove newlines and collapse whitespace to single spaces.
///
/// Converts multi-line text into a single normalized line.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_textRemoveNewlines<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    text: JString<'local>,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            let text_str: String = match text.try_to_string(env) {
                Ok(s) => s,
                Err(e) => return throw_and_default(env, &e),
            };
            let result = iscc_lib::text_remove_newlines(&text_str);
            match env.new_string(result) {
                Ok(s) => Ok(s),
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Trim text so its UTF-8 encoded size does not exceed `nbytes`.
///
/// Multi-byte characters that would be split are dropped entirely.
/// Leading/trailing whitespace is stripped from the result.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_textTrim<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    text: JString<'local>,
    nbytes: jint,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            let nbytes = match non_negative(env, nbytes, "nbytes") {
                Ok(v) => v as usize,
                Err(default) => return default,
            };
            let text_str: String = match text.try_to_string(env) {
                Ok(s) => s,
                Err(e) => return throw_and_default(env, &e),
            };
            let result = iscc_lib::text_trim(&text_str, nbytes);
            match env.new_string(result) {
                Ok(s) => Ok(s),
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Normalize and simplify text for similarity hashing.
///
/// Applies NFD normalization, lowercasing, removes whitespace and characters
/// in Unicode categories C (control), M (mark), and P (punctuation), then
/// recombines with NFKC normalization.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_textCollapse<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    text: JString<'local>,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            let text_str: String = match text.try_to_string(env) {
                Ok(s) => s,
                Err(e) => return throw_and_default(env, &e),
            };
            let result = iscc_lib::text_collapse(&text_str);
            match env.new_string(result) {
                Ok(s) => Ok(s),
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

// ── Encoding ────────────────────────────────────────────────────────────────

/// Encode bytes as base64url (RFC 4648 §5, no padding).
///
/// Returns a URL-safe base64 encoded string without padding characters.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_encodeBase64<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    data: JByteArray<'local>,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            let bytes = match env.convert_byte_array(&data) {
                Ok(b) => b,
                Err(e) => return throw_and_default(env, &e),
            };
            let result = iscc_lib::encode_base64(&bytes);
            match env.new_string(result) {
                Ok(s) => Ok(s),
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Convert a JSON string to a base64-encoded data URL.
///
/// Uses `application/ld+json` media type when the JSON contains an `@context`
/// key, otherwise `application/json`. Throws `IllegalArgumentException` on
/// invalid JSON input.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_jsonToDataUrl<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    json: JString<'local>,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            let json_str: String = match json.try_to_string(env) {
                Ok(s) => s,
                Err(e) => return throw_and_default(env, &e),
            };
            match iscc_lib::json_to_data_url(&json_str) {
                Ok(result) => match env.new_string(result) {
                    Ok(s) => Ok(s),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

// ── Codec ───────────────────────────────────────────────────────────────────

/// Encode header fields and a raw digest into a base32-encoded ISCC unit string.
///
/// Accepts integer type identifiers (`mtype`, `stype`, `version` in range
/// 0–255) and a `bit_length` (≥0). Returns the encoded ISCC unit string.
/// Throws `IllegalArgumentException` on invalid input or out-of-range values.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_encodeComponent<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    mtype: jint,
    stype: jint,
    version: jint,
    bit_length: jint,
    digest: JByteArray<'local>,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            // Validate jint ranges before casting
            if !(0..=255).contains(&mtype) {
                return throw_and_default(env, "mtype must be in range 0-255");
            }
            if !(0..=255).contains(&stype) {
                return throw_and_default(env, "stype must be in range 0-255");
            }
            if !(0..=255).contains(&version) {
                return throw_and_default(env, "version must be in range 0-255");
            }
            if bit_length < 0 {
                return throw_and_default(env, "bitLength must be non-negative");
            }
            let digest_bytes = match env.convert_byte_array(&digest) {
                Ok(b) => b,
                Err(e) => return throw_and_default(env, &e),
            };
            match iscc_lib::encode_component(
                mtype as u8,
                stype as u8,
                version as u8,
                bit_length as u32,
                &digest_bytes,
            ) {
                Ok(result) => match env.new_string(result) {
                    Ok(s) => Ok(s),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Decode an ISCC unit string into its header components and raw digest.
///
/// Strips an optional "ISCC:" prefix before decoding. Returns a Java
/// `IsccDecodeResult` object with `maintype`, `subtype`, `version`,
/// `length`, and `digest` fields. Throws `IllegalArgumentException` on
/// invalid input.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_isccDecode<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    iscc_unit: JString<'local>,
) -> JObject<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JObject<'local>> {
            let iscc_str: String = match iscc_unit.try_to_string(env) {
                Ok(s) => s,
                Err(e) => return throw_and_default(env, &e),
            };
            let (mt, st, vs, li, digest) = match iscc_lib::iscc_decode(&iscc_str) {
                Ok(result) => result,
                Err(e) => return throw_and_default(env, &e),
            };
            // Build a Java byte[] from the digest Vec<u8>
            let byte_array = match env.byte_array_from_slice(&digest) {
                Ok(a) => a,
                Err(e) => return throw_and_default(env, &e),
            };
            // Find the IsccDecodeResult class and construct a new instance
            let class = match env.find_class(jni_str!("io/iscc/iscc_lib/IsccDecodeResult")) {
                Ok(c) => c,
                Err(e) => return throw_and_default(env, &e),
            };
            match env.new_object(
                class,
                jni_sig!("(IIII[B)V"),
                &[
                    JValue::Int(mt as jint),
                    JValue::Int(st as jint),
                    JValue::Int(vs as jint),
                    JValue::Int(li as jint),
                    JValue::Object(&byte_array),
                ],
            ) {
                Ok(obj) => Ok(obj),
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Decompose a composite ISCC-CODE into individual ISCC-UNITs.
///
/// Returns a `String[]` of base32-encoded ISCC-UNIT strings (without prefix).
/// Throws `IllegalArgumentException` on invalid input.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_isccDecompose<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    iscc_code: JString<'local>,
) -> JObjectArray<'local, JString<'local>> {
    unowned
        .with_env(
            |env| -> jni::errors::Result<JObjectArray<'local, JString<'local>>> {
                let code_str: String = match iscc_code.try_to_string(env) {
                    Ok(s) => s,
                    Err(e) => return throw_and_default(env, &e),
                };
                let units = match iscc_lib::iscc_decompose(&code_str) {
                    Ok(u) => u,
                    Err(e) => return throw_and_default(env, &e),
                };
                match build_string_array(env, &units) {
                    Ok(arr) => Ok(arr),
                    Err(e) => throw_and_default(env, &e),
                }
            },
        )
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Encode ISCC-UNIT strings into an ISCC-SEQ byte string (IEP-0020).
///
/// Takes a `String[]` of ISCC-UNITs. Returns the concatenated raw unit bytes as
/// `byte[]`. Throws `IllegalArgumentException` on empty input or invalid units.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_encodeSeq<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    units: JObjectArray<'local, JString<'local>>,
) -> JByteArray<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JByteArray<'local>> {
            let unit_strs = match extract_string_array(env, &units) {
                Ok(s) => s,
                Err(e) => return throw_and_default(env, &e),
            };
            let refs: Vec<&str> = unit_strs.iter().map(|s| s.as_str()).collect();
            let seq = match iscc_lib::encode_seq(&refs) {
                Ok(b) => b,
                Err(e) => return throw_and_default(env, &e),
            };
            match env.byte_array_from_slice(&seq) {
                Ok(a) => Ok(a),
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Decode an ISCC-SEQ byte string into ISCC-UNIT strings (IEP-0020).
///
/// Returns a `String[]` of `ISCC:`-prefixed unit strings in wire order.
/// Throws `IllegalArgumentException` on empty or malformed input.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_decodeSeq<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    data: JByteArray<'local>,
) -> JObjectArray<'local, JString<'local>> {
    unowned
        .with_env(
            |env| -> jni::errors::Result<JObjectArray<'local, JString<'local>>> {
                let bytes = match env.convert_byte_array(&data) {
                    Ok(b) => b,
                    Err(e) => return throw_and_default(env, &e),
                };
                let units = match iscc_lib::decode_seq(&bytes) {
                    Ok(u) => u,
                    Err(e) => return throw_and_default(env, &e),
                };
                match build_string_array(env, &units) {
                    Ok(arr) => Ok(arr),
                    Err(e) => throw_and_default(env, &e),
                }
            },
        )
        .resolve::<ThrowRuntimeExAndDefault>()
}

// ── Sliding window ──────────────────────────────────────────────────────────

/// Generate sliding window n-grams from a string.
///
/// Returns a `String[]` of overlapping substrings of `width` Unicode characters.
/// Throws `IllegalArgumentException` if width is less than 2.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_slidingWindow<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    seq: JString<'local>,
    width: jint,
) -> JObjectArray<'local, JString<'local>> {
    unowned
        .with_env(
            |env| -> jni::errors::Result<JObjectArray<'local, JString<'local>>> {
                let width = match non_negative(env, width, "width") {
                    Ok(v) => v as usize,
                    Err(default) => return default,
                };
                let seq_str: String = match seq.try_to_string(env) {
                    Ok(s) => s,
                    Err(e) => return throw_and_default(env, &e),
                };
                let ngrams = match iscc_lib::sliding_window(&seq_str, width) {
                    Ok(v) => v,
                    Err(e) => return throw_and_default(env, &e),
                };
                match build_string_array(env, &ngrams) {
                    Ok(arr) => Ok(arr),
                    Err(e) => throw_and_default(env, &e),
                }
            },
        )
        .resolve::<ThrowRuntimeExAndDefault>()
}

// ── Algorithm primitives ────────────────────────────────────────────────────

/// Compute a SimHash from a sequence of equal-length hash digests.
///
/// Takes a `byte[][]` of hash digests. Returns `byte[]` with the
/// similarity-preserving hash. Throws `IllegalArgumentException` on error.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_algSimhash<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    hash_digests: JObjectArray<'local, JByteArray<'local>>,
) -> JByteArray<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JByteArray<'local>> {
            let num = match hash_digests.len(env) {
                Ok(l) => l,
                Err(e) => return throw_and_default(env, &e),
            };
            let mut digests: Vec<Vec<u8>> = Vec::with_capacity(num);
            for i in 0..num {
                // Per-iteration local frame: byte data is copied into a
                // Rust-owned Vec before the frame pops.
                let bytes = match env.with_local_frame(16, |env| -> jni::errors::Result<Vec<u8>> {
                    let elem = hash_digests.get_element(env, i)?;
                    env.convert_byte_array(&elem)
                }) {
                    Ok(b) => b,
                    Err(e) => return throw_and_default(env, &e),
                };
                digests.push(bytes);
            }
            let refs: Vec<&[u8]> = digests.iter().map(|d| d.as_slice()).collect();
            match iscc_lib::alg_simhash(&refs) {
                Ok(result) => match env.byte_array_from_slice(&result) {
                    Ok(a) => Ok(a),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Compute a 256-bit MinHash digest from 32-bit integer features.
///
/// Takes `int[]` features (Java `int` is signed, cast to `u32`).
/// Returns `byte[]` with 32-byte digest.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_algMinhash256<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    features: JIntArray<'local>,
) -> JByteArray<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JByteArray<'local>> {
            let buf = match extract_int_array(env, &features) {
                Ok(b) => b,
                Err(e) => return throw_and_default(env, &e),
            };
            // Java has no unsigned int — cast jint (i32) to u32
            let u32_features: Vec<u32> = buf.iter().map(|&v| v as u32).collect();
            let result = iscc_lib::alg_minhash_256(&u32_features);
            match env.byte_array_from_slice(&result) {
                Ok(a) => Ok(a),
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Split data into content-defined chunks using gear rolling hash.
///
/// Returns `byte[][]`. When `utf32` is true, aligns cut points to 4-byte
/// boundaries. Default `avg_chunk_size` is 1024.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_algCdcChunks<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    data: JByteArray<'local>,
    utf32: jboolean,
    avg_chunk_size: jint,
) -> JObjectArray<'local, JByteArray<'local>> {
    unowned
        .with_env(
            |env| -> jni::errors::Result<JObjectArray<'local, JByteArray<'local>>> {
                if avg_chunk_size < 2 {
                    return throw_and_default(
                        env,
                        format!("avg_chunk_size must be >= 2, got {avg_chunk_size}").as_str(),
                    );
                }
                let bytes = match env.convert_byte_array(&data) {
                    Ok(b) => b,
                    Err(e) => return throw_and_default(env, &e),
                };
                let chunks = match iscc_lib::alg_cdc_chunks(&bytes, utf32, avg_chunk_size as u32) {
                    Ok(c) => c,
                    Err(e) => return throw_and_default(env, &e),
                };
                let arr = match JObjectArray::<JByteArray>::new(
                    env,
                    chunks.len(),
                    &JByteArray::default(),
                ) {
                    Ok(a) => a,
                    Err(e) => return throw_and_default(env, &e),
                };
                for (i, chunk) in chunks.iter().enumerate() {
                    // Per-iteration local frame: the byte array is stored into
                    // the result array (a JVM-side reference) before the frame
                    // pops.
                    if let Err(e) = env.with_local_frame(16, |env| -> jni::errors::Result<()> {
                        let barr = env.byte_array_from_slice(chunk)?;
                        arr.set_element(env, i, &barr)
                    }) {
                        return throw_and_default(env, &e);
                    }
                }
                Ok(arr)
            },
        )
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Compute a similarity-preserving hash from video frame signatures.
///
/// Takes `int[][]` frame signatures and `bits`. Returns `byte[]` of length
/// `bits / 8`. Throws `IllegalArgumentException` if input is empty.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_softHashVideoV0<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    frame_sigs: JObjectArray<'local, JIntArray<'local>>,
    bits: jint,
) -> JByteArray<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JByteArray<'local>> {
            let bits = match non_negative(env, bits, "bits") {
                Ok(v) => v,
                Err(default) => return default,
            };
            let frames = match extract_int_array_2d(env, &frame_sigs) {
                Ok(f) => f,
                Err(e) => return throw_and_default(env, &e),
            };
            match iscc_lib::soft_hash_video_v0(&frames, bits) {
                Ok(result) => match env.byte_array_from_slice(&result) {
                    Ok(a) => Ok(a),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
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
    _env: EnvUnowned,
    _class: JClass,
) -> jlong {
    let wrapper = Box::new(JniDataHasher {
        inner: Some(iscc_lib::DataHasher::new()),
    });
    Box::into_raw(wrapper) as jlong
}

/// Push data into a streaming DataHasher.
///
/// Throws `IllegalArgumentException` for a zero handle and
/// `IllegalStateException` if the hasher has already been finalized.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_dataHasherUpdate<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    ptr: jlong,
    data: JByteArray<'local>,
) {
    unowned
        .with_env(|env| -> jni::errors::Result<()> {
            if ptr == 0 {
                return throw_and_default(env, "DataHasher handle is null");
            }
            let bytes = match env.convert_byte_array(&data) {
                Ok(b) => b,
                Err(e) => return throw_and_default(env, &e),
            };
            // SAFETY: ptr is non-zero (checked above) and the Java caller must
            // pass a handle from dataHasherNew that dataHasherFree has not
            // released; a stale non-zero handle cannot be detected here.
            let wrapper = unsafe { &mut *(ptr as *mut JniDataHasher) };
            let Some(inner) = wrapper.inner.as_mut() else {
                return throw_state_error(env, "DataHasher already finalized");
            };
            inner.update(&bytes);
            Ok(())
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Finalize a streaming DataHasher and return an ISCC string.
///
/// Consumes the inner hasher state, even when it throws on an invalid
/// `bits`. After this call, subsequent `update` or `finalize` calls throw
/// `IllegalStateException`; a zero handle throws
/// `IllegalArgumentException`. The caller must still call
/// `dataHasherFree` to release the wrapper.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_dataHasherFinalize<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    ptr: jlong,
    bits: jint,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            if ptr == 0 {
                return throw_and_default(env, "DataHasher handle is null");
            }
            // SAFETY: ptr is non-zero (checked above) and the Java caller must
            // pass a handle from dataHasherNew that dataHasherFree has not
            // released; a stale non-zero handle cannot be detected here.
            let wrapper = unsafe { &mut *(ptr as *mut JniDataHasher) };
            let Some(inner) = wrapper.inner.take() else {
                return throw_state_error(env, "DataHasher already finalized");
            };
            let bits = match non_negative(env, bits, "bits") {
                Ok(v) => v,
                Err(default) => return default,
            };
            match inner.finalize(bits) {
                Ok(result) => match env.new_string(result.iscc) {
                    Ok(s) => Ok(s),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Free a DataHasher previously created by `dataHasherNew`.
///
/// Zero/null handle is a no-op. Each handle must be freed exactly once.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_dataHasherFree(
    _env: EnvUnowned,
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
    _env: EnvUnowned,
    _class: JClass,
) -> jlong {
    let wrapper = Box::new(JniInstanceHasher {
        inner: Some(iscc_lib::InstanceHasher::new()),
    });
    Box::into_raw(wrapper) as jlong
}

/// Push data into a streaming InstanceHasher.
///
/// Throws `IllegalArgumentException` for a zero handle and
/// `IllegalStateException` if the hasher has already been finalized.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_instanceHasherUpdate<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    ptr: jlong,
    data: JByteArray<'local>,
) {
    unowned
        .with_env(|env| -> jni::errors::Result<()> {
            if ptr == 0 {
                return throw_and_default(env, "InstanceHasher handle is null");
            }
            let bytes = match env.convert_byte_array(&data) {
                Ok(b) => b,
                Err(e) => return throw_and_default(env, &e),
            };
            // SAFETY: ptr is non-zero (checked above) and the Java caller must
            // pass a handle from instanceHasherNew that instanceHasherFree has not
            // released; a stale non-zero handle cannot be detected here.
            let wrapper = unsafe { &mut *(ptr as *mut JniInstanceHasher) };
            let Some(inner) = wrapper.inner.as_mut() else {
                return throw_state_error(env, "InstanceHasher already finalized");
            };
            inner.update(&bytes);
            Ok(())
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Finalize a streaming InstanceHasher and return an ISCC string.
///
/// Consumes the inner hasher state, even when it throws on an invalid
/// `bits`. After this call, subsequent `update` or `finalize` calls throw
/// `IllegalStateException`; a zero handle throws
/// `IllegalArgumentException`. The caller must still call
/// `instanceHasherFree` to release the wrapper.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_instanceHasherFinalize<'local>(
    mut unowned: EnvUnowned<'local>,
    _class: JClass<'local>,
    ptr: jlong,
    bits: jint,
) -> JString<'local> {
    unowned
        .with_env(|env| -> jni::errors::Result<JString<'local>> {
            if ptr == 0 {
                return throw_and_default(env, "InstanceHasher handle is null");
            }
            // SAFETY: ptr is non-zero (checked above) and the Java caller must
            // pass a handle from instanceHasherNew that instanceHasherFree has not
            // released; a stale non-zero handle cannot be detected here.
            let wrapper = unsafe { &mut *(ptr as *mut JniInstanceHasher) };
            let Some(inner) = wrapper.inner.take() else {
                return throw_state_error(env, "InstanceHasher already finalized");
            };
            let bits = match non_negative(env, bits, "bits") {
                Ok(v) => v,
                Err(default) => return default,
            };
            match inner.finalize(bits) {
                Ok(result) => match env.new_string(result.iscc) {
                    Ok(s) => Ok(s),
                    Err(e) => throw_and_default(env, &e),
                },
                Err(e) => throw_and_default(env, &e),
            }
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

/// Free an InstanceHasher previously created by `instanceHasherNew`.
///
/// Zero/null handle is a no-op. Each handle must be freed exactly once.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_instanceHasherFree(
    _env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        // SAFETY: ptr was produced by Box::into_raw() in instanceHasherNew
        drop(unsafe { Box::from_raw(ptr as *mut JniInstanceHasher) });
    }
}
