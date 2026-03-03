//! Magnus bridge for iscc-lib Ruby bindings.
//!
//! Exposes a subset of Tier 1 ISCC functions as Ruby module functions
//! under the `IsccLib` module. The pure Ruby wrapper in `lib/iscc_lib.rb`
//! provides idiomatic result classes and keyword arguments.
//!
//! Symbols (25 of 32):
//! - `gen_meta_code_v0`, `gen_text_code_v0`, `gen_image_code_v0`, `gen_audio_code_v0`
//! - `gen_video_code_v0`, `gen_mixed_code_v0`, `gen_data_code_v0`
//! - `gen_instance_code_v0`, `gen_iscc_code_v0`, `gen_sum_code_v0`
//! - `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`
//! - `encode_base64`, `iscc_decompose`, `encode_component`, `iscc_decode`
//! - `json_to_data_url`, `conformance_selftest`
//! - Constants: META_TRIM_NAME, META_TRIM_DESCRIPTION, META_TRIM_META,
//!   IO_READ_SIZE, TEXT_NGRAM_SIZE

use magnus::{Error, RArray, RHash, RString, Ruby, TryConvert, function, prelude::*};

/// Map an `IsccError` to a Magnus `RuntimeError`.
fn to_magnus_err(e: iscc_lib::IsccError) -> Error {
    Error::new(magnus::exception::runtime_error(), e.to_string())
}

/// Generate a Meta-Code from name and optional metadata.
///
/// Returns a Ruby Hash with keys: `iscc`, `name`, `metahash`, and optionally
/// `description` and `meta`.
fn gen_meta_code_v0(
    name: String,
    description: Option<String>,
    meta: Option<String>,
    bits: u32,
) -> Result<RHash, Error> {
    let r = iscc_lib::gen_meta_code_v0(&name, description.as_deref(), meta.as_deref(), bits)
        .map_err(to_magnus_err)?;
    let ruby = Ruby::get().expect("called from Ruby");
    let hash = ruby.hash_new();
    hash.aset("iscc", r.iscc)?;
    hash.aset("name", r.name)?;
    hash.aset("metahash", r.metahash)?;
    if let Some(desc) = r.description {
        hash.aset("description", desc)?;
    }
    if let Some(meta) = r.meta {
        hash.aset("meta", meta)?;
    }
    Ok(hash)
}

/// Generate a Text-Code from plain text content.
///
/// Returns a Ruby Hash with keys: `iscc`, `characters`.
fn gen_text_code_v0(text: String, bits: u32) -> Result<RHash, Error> {
    let r = iscc_lib::gen_text_code_v0(&text, bits).map_err(to_magnus_err)?;
    let ruby = Ruby::get().expect("called from Ruby");
    let hash = ruby.hash_new();
    hash.aset("iscc", r.iscc)?;
    hash.aset("characters", r.characters)?;
    Ok(hash)
}

/// Generate an Image-Code from pixel data.
///
/// Accepts a binary Ruby String of raw pixel bytes.
/// Returns a Ruby Hash with key: `iscc`.
fn gen_image_code_v0(pixels: RString, bits: u32) -> Result<RHash, Error> {
    // Safety: the slice is passed directly to a pure Rust function
    // and not held across any Ruby API calls.
    let bytes = unsafe { pixels.as_slice() };
    let r = iscc_lib::gen_image_code_v0(bytes, bits).map_err(to_magnus_err)?;
    let ruby = Ruby::get().expect("called from Ruby");
    let hash = ruby.hash_new();
    hash.aset("iscc", r.iscc)?;
    Ok(hash)
}

/// Generate an Audio-Code from a Chromaprint feature vector.
///
/// Accepts a Ruby Array of integers (i32 Chromaprint fingerprints).
/// Returns a Ruby Hash with key: `iscc`.
fn gen_audio_code_v0(cv: Vec<i32>, bits: u32) -> Result<RHash, Error> {
    let r = iscc_lib::gen_audio_code_v0(&cv, bits).map_err(to_magnus_err)?;
    let ruby = Ruby::get().expect("called from Ruby");
    let hash = ruby.hash_new();
    hash.aset("iscc", r.iscc)?;
    Ok(hash)
}

/// Generate a Video-Code from frame signature vectors.
///
/// Accepts a Ruby Array of Arrays of integers (nested `i32` frame signatures).
/// Returns a Ruby Hash with key: `iscc`.
fn gen_video_code_v0(frame_sigs: RArray, bits: u32) -> Result<RHash, Error> {
    let frames: Vec<Vec<i32>> = frame_sigs
        .into_iter()
        .map(|frame| {
            let arr: Vec<i32> = TryConvert::try_convert(frame)?;
            Ok(arr)
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let r = iscc_lib::gen_video_code_v0(&frames, bits).map_err(to_magnus_err)?;
    let ruby = Ruby::get().expect("called from Ruby");
    let hash = ruby.hash_new();
    hash.aset("iscc", r.iscc)?;
    Ok(hash)
}

/// Generate a Mixed-Code from multiple ISCC content code strings.
///
/// Accepts a Ruby Array of ISCC unit strings.
/// Returns a Ruby Hash with keys: `iscc`, `parts`.
fn gen_mixed_code_v0(codes: Vec<String>, bits: u32) -> Result<RHash, Error> {
    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
    let r = iscc_lib::gen_mixed_code_v0(&refs, bits).map_err(to_magnus_err)?;
    let ruby = Ruby::get().expect("called from Ruby");
    let hash = ruby.hash_new();
    hash.aset("iscc", r.iscc)?;
    hash.aset("parts", r.parts)?;
    Ok(hash)
}

/// Generate a Data-Code from binary data.
///
/// Accepts a binary Ruby String of raw bytes.
/// Returns a Ruby Hash with key: `iscc`.
fn gen_data_code_v0(data: RString, bits: u32) -> Result<RHash, Error> {
    // Safety: the slice is passed directly to a pure Rust function
    // and not held across any Ruby API calls.
    let bytes = unsafe { data.as_slice() };
    let r = iscc_lib::gen_data_code_v0(bytes, bits).map_err(to_magnus_err)?;
    let ruby = Ruby::get().expect("called from Ruby");
    let hash = ruby.hash_new();
    hash.aset("iscc", r.iscc)?;
    Ok(hash)
}

/// Generate an Instance-Code from binary data.
///
/// Accepts a binary Ruby String of raw bytes.
/// Returns a Ruby Hash with keys: `iscc`, `datahash`, `filesize`.
fn gen_instance_code_v0(data: RString, bits: u32) -> Result<RHash, Error> {
    // Safety: the slice is passed directly to a pure Rust function
    // and not held across any Ruby API calls.
    let bytes = unsafe { data.as_slice() };
    let r = iscc_lib::gen_instance_code_v0(bytes, bits).map_err(to_magnus_err)?;
    let ruby = Ruby::get().expect("called from Ruby");
    let hash = ruby.hash_new();
    hash.aset("iscc", r.iscc)?;
    hash.aset("datahash", r.datahash)?;
    hash.aset("filesize", r.filesize)?;
    Ok(hash)
}

/// Generate a composite ISCC-CODE from individual unit codes.
///
/// Accepts a Ruby Array of ISCC unit strings and a wide flag.
/// Returns a Ruby Hash with key: `iscc`.
fn gen_iscc_code_v0(codes: Vec<String>, wide: bool) -> Result<RHash, Error> {
    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
    let r = iscc_lib::gen_iscc_code_v0(&refs, wide).map_err(to_magnus_err)?;
    let ruby = Ruby::get().expect("called from Ruby");
    let hash = ruby.hash_new();
    hash.aset("iscc", r.iscc)?;
    Ok(hash)
}

/// Generate a composite ISCC-CODE from a file in a single pass.
///
/// Accepts a file path string, bit length, wide flag, and add_units flag.
/// Returns a Ruby Hash with keys: `iscc`, `datahash`, `filesize`, and
/// optionally `units` when `add_units` is true.
fn gen_sum_code_v0(path: String, bits: u32, wide: bool, add_units: bool) -> Result<RHash, Error> {
    let r = iscc_lib::gen_sum_code_v0(std::path::Path::new(&path), bits, wide, add_units)
        .map_err(to_magnus_err)?;
    let ruby = Ruby::get().expect("called from Ruby");
    let hash = ruby.hash_new();
    hash.aset("iscc", r.iscc)?;
    hash.aset("datahash", r.datahash)?;
    hash.aset("filesize", r.filesize)?;
    if let Some(units) = r.units {
        hash.aset("units", units)?;
    }
    Ok(hash)
}

/// Clean and normalize text for display.
///
/// Applies NFKC normalization, removes control characters (except newlines),
/// normalizes `\r\n` to `\n`, collapses consecutive empty lines, and strips
/// leading/trailing whitespace.
fn text_clean(text: String) -> String {
    iscc_lib::text_clean(&text)
}

/// Remove newlines and collapse whitespace to single spaces.
///
/// Converts multi-line text into a single normalized line.
fn text_remove_newlines(text: String) -> String {
    iscc_lib::text_remove_newlines(&text)
}

/// Trim text so its UTF-8 encoded size does not exceed `nbytes`.
///
/// Multi-byte characters that would be split are dropped entirely.
/// Leading/trailing whitespace is stripped from the result.
fn text_trim(text: String, nbytes: usize) -> String {
    iscc_lib::text_trim(&text, nbytes)
}

/// Normalize and simplify text for similarity hashing.
///
/// Applies NFD normalization, lowercasing, removes whitespace and certain
/// Unicode categories, then recombines with NFKC normalization.
fn text_collapse(text: String) -> String {
    iscc_lib::text_collapse(&text)
}

/// Encode bytes as base64url (RFC 4648 §5, no padding).
///
/// Accepts a Ruby String (binary data) and returns a URL-safe base64 string.
fn encode_base64(data: RString) -> String {
    // Safety: the slice is passed directly to a pure Rust function
    // and not held across any Ruby API calls.
    let bytes = unsafe { data.as_slice() };
    iscc_lib::encode_base64(bytes)
}

/// Decompose a composite ISCC-CODE into individual ISCC-UNITs.
///
/// Returns a Ruby Array of base32-encoded ISCC-UNIT strings (without prefix).
fn iscc_decompose(iscc_code: String) -> Result<Vec<String>, Error> {
    iscc_lib::iscc_decompose(&iscc_code).map_err(to_magnus_err)
}

/// Encode raw digest components into a base32 ISCC unit string.
///
/// Takes integer type identifiers (mtype, stype, version), a bit_length,
/// and a binary digest String. Returns a base32-encoded ISCC unit string.
fn encode_component(
    mtype: u8,
    stype: u8,
    version: u8,
    bit_length: u32,
    digest: RString,
) -> Result<String, Error> {
    // Safety: we copy the bytes immediately and do not hold the slice
    // across any Ruby API calls.
    let bytes = unsafe { digest.as_slice() }.to_vec();
    iscc_lib::encode_component(mtype, stype, version, bit_length, &bytes).map_err(to_magnus_err)
}

/// Decode an ISCC unit string into header components and raw digest.
///
/// Returns a 5-element Ruby Array: `[maintype, subtype, version, length_index, digest_bytes]`
/// where digest_bytes is a binary Ruby String.
fn iscc_decode(iscc: String) -> Result<RArray, Error> {
    let (mt, st, vs, li, digest) = iscc_lib::iscc_decode(&iscc).map_err(to_magnus_err)?;
    let ruby = Ruby::get().expect("called from Ruby");
    let arr = ruby.ary_new_capa(5);
    arr.push(mt)?;
    arr.push(st)?;
    arr.push(vs)?;
    arr.push(li)?;
    arr.push(RString::from_slice(&digest))?;
    Ok(arr)
}

/// Convert a JSON string into a `data:` URL with JCS canonicalization.
///
/// Uses `application/ld+json` media type when the JSON contains an `@context`
/// key, otherwise `application/json`.
fn json_to_data_url(json: String) -> Result<String, Error> {
    iscc_lib::json_to_data_url(&json).map_err(to_magnus_err)
}

/// Run conformance self-test against vendored test vectors.
///
/// Returns `true` if all tests pass, `false` if any fail.
fn conformance_selftest() -> bool {
    iscc_lib::conformance_selftest()
}

/// Initialize the IsccLib Ruby module with all bridge functions and constants.
#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("IsccLib")?;

    // Gen functions (prefixed with _ for Ruby wrapper layer)
    module.define_module_function("_gen_meta_code_v0", function!(gen_meta_code_v0, 4))?;
    module.define_module_function("_gen_text_code_v0", function!(gen_text_code_v0, 2))?;
    module.define_module_function("_gen_image_code_v0", function!(gen_image_code_v0, 2))?;
    module.define_module_function("_gen_audio_code_v0", function!(gen_audio_code_v0, 2))?;
    module.define_module_function("_gen_video_code_v0", function!(gen_video_code_v0, 2))?;
    module.define_module_function("_gen_mixed_code_v0", function!(gen_mixed_code_v0, 2))?;
    module.define_module_function("_gen_data_code_v0", function!(gen_data_code_v0, 2))?;
    module.define_module_function("_gen_instance_code_v0", function!(gen_instance_code_v0, 2))?;
    module.define_module_function("_gen_iscc_code_v0", function!(gen_iscc_code_v0, 2))?;
    module.define_module_function("_gen_sum_code_v0", function!(gen_sum_code_v0, 4))?;

    // Text utility functions
    module.define_module_function("text_clean", function!(text_clean, 1))?;
    module.define_module_function("text_remove_newlines", function!(text_remove_newlines, 1))?;
    module.define_module_function("text_trim", function!(text_trim, 2))?;
    module.define_module_function("text_collapse", function!(text_collapse, 1))?;

    // Codec and encoding functions
    module.define_module_function("encode_base64", function!(encode_base64, 1))?;
    module.define_module_function("iscc_decompose", function!(iscc_decompose, 1))?;
    module.define_module_function("encode_component", function!(encode_component, 5))?;
    module.define_module_function("iscc_decode", function!(iscc_decode, 1))?;
    module.define_module_function("json_to_data_url", function!(json_to_data_url, 1))?;
    module.define_module_function("conformance_selftest", function!(conformance_selftest, 0))?;

    // Constants
    module.const_set("META_TRIM_NAME", iscc_lib::META_TRIM_NAME)?;
    module.const_set("META_TRIM_DESCRIPTION", iscc_lib::META_TRIM_DESCRIPTION)?;
    module.const_set("META_TRIM_META", iscc_lib::META_TRIM_META)?;
    module.const_set("IO_READ_SIZE", iscc_lib::IO_READ_SIZE)?;
    module.const_set("TEXT_NGRAM_SIZE", iscc_lib::TEXT_NGRAM_SIZE)?;

    Ok(())
}
