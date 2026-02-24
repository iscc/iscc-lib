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
//! ## Error handling pattern
//!
//! For functions that return `IsccResult<T>`, errors are propagated to Java by
//! throwing `IllegalArgumentException` via `env.throw_new()` and returning a
//! type-appropriate default value. Add a `throw_and_default` helper when
//! implementing gen functions:
//!
//! ```ignore
//! fn throw_and_default<T: Default>(env: &mut JNIEnv, msg: &str) -> T {
//!     let _ = env.throw_new("java/lang/IllegalArgumentException", msg);
//!     T::default()
//! }
//!
//! // Usage in a gen binding:
//! match iscc_lib::gen_text_code_v0(&text, bits) {
//!     Ok(result) => env.new_string(result.iscc).unwrap().into_raw(),
//!     Err(e) => throw_and_default(&mut env, &e.to_string()),
//! }
//! ```
//!
//! ## Adding new bindings
//!
//! To bind a new Tier 1 function:
//! 1. Add `#[unsafe(no_mangle)] pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_<name>`
//! 2. Convert JNI types to Rust types (e.g., `JString` → `&str` via `env.get_string()`)
//! 3. Call the `iscc_lib` function
//! 4. Convert the result back to JNI types, using `throw_and_default` for errors
//! 5. Return `JObject::null().into_raw()` as the default for object-returning functions

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jboolean;

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
