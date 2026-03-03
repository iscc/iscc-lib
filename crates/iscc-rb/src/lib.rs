//! Magnus bridge for iscc-lib Ruby bindings.
//!
//! Exposes a subset of Tier 1 ISCC functions as Ruby module functions
//! under the `IsccLib` module. The pure Ruby wrapper in `lib/iscc_lib.rb`
//! provides idiomatic result classes and keyword arguments.
//!
//! Symbols (32 of 32):
//! - `gen_meta_code_v0`, `gen_text_code_v0`, `gen_image_code_v0`, `gen_audio_code_v0`
//! - `gen_video_code_v0`, `gen_mixed_code_v0`, `gen_data_code_v0`
//! - `gen_instance_code_v0`, `gen_iscc_code_v0`, `gen_sum_code_v0`
//! - `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`
//! - `encode_base64`, `iscc_decompose`, `encode_component`, `iscc_decode`
//! - `json_to_data_url`, `conformance_selftest`
//! - `sliding_window`, `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
//!   `soft_hash_video_v0`
//! - `DataHasher`, `InstanceHasher` (streaming classes)
//! - Constants: META_TRIM_NAME, META_TRIM_DESCRIPTION, META_TRIM_META,
//!   IO_READ_SIZE, TEXT_NGRAM_SIZE

use magnus::{Error, RArray, RHash, RString, Ruby, TryConvert, function, method, prelude::*};
use std::cell::RefCell;

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

/// Generate sliding window n-grams from a string.
///
/// Returns overlapping substrings of `width` Unicode characters.
/// Raises `RuntimeError` if `width < 2`.
fn sliding_window(seq: String, width: usize) -> Result<Vec<String>, Error> {
    iscc_lib::sliding_window(&seq, width).map_err(to_magnus_err)
}

/// Compute a SimHash from a sequence of equal-length hash digests.
///
/// Accepts a Ruby Array of binary Strings, returns a binary String.
/// Raises `RuntimeError` on mismatched digest lengths.
fn alg_simhash(hash_digests: RArray) -> Result<RString, Error> {
    let digests: Vec<Vec<u8>> = hash_digests
        .into_iter()
        .map(|val| {
            let s: RString = TryConvert::try_convert(val)?;
            // Safety: we copy the bytes immediately before any Ruby API calls.
            let bytes = unsafe { s.as_slice() }.to_vec();
            Ok(bytes)
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let result = iscc_lib::alg_simhash(&digests).map_err(to_magnus_err)?;
    Ok(RString::from_slice(&result))
}

/// Compute a 256-bit MinHash digest from 32-bit integer features.
///
/// Returns a 32-byte binary String.
fn alg_minhash_256(features: Vec<u32>) -> RString {
    let result = iscc_lib::alg_minhash_256(&features);
    RString::from_slice(&result)
}

/// Split data into content-defined chunks using gear rolling hash.
///
/// Accepts a binary Ruby String, a `utf32` flag, and an `avg_chunk_size`.
/// Returns a Ruby Array of binary Strings (one per chunk).
fn alg_cdc_chunks(data: RString, utf32: bool, avg_chunk_size: u32) -> Result<RArray, Error> {
    // Safety: the slice is passed directly to a pure Rust function
    // and not held across any Ruby API calls that could trigger GC.
    let bytes = unsafe { data.as_slice() };
    let chunks = iscc_lib::alg_cdc_chunks(bytes, utf32, avg_chunk_size);
    let ruby = Ruby::get().expect("called from Ruby");
    let arr = ruby.ary_new_capa(chunks.len());
    for chunk in chunks {
        arr.push(RString::from_slice(chunk))?;
    }
    Ok(arr)
}

/// Compute a similarity-preserving hash from video frame signatures.
///
/// Accepts a Ruby Array of Arrays of integers (nested `i32` frame signatures)
/// and a bit length. Returns a binary String of length `bits / 8`.
fn soft_hash_video_v0(frame_sigs: RArray, bits: u32) -> Result<RString, Error> {
    let frames: Vec<Vec<i32>> = frame_sigs
        .into_iter()
        .map(|frame| {
            let arr: Vec<i32> = TryConvert::try_convert(frame)?;
            Ok(arr)
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let result = iscc_lib::soft_hash_video_v0(&frames, bits).map_err(to_magnus_err)?;
    Ok(RString::from_slice(&result))
}

/// Streaming Data-Code generator for Ruby.
///
/// Wraps `iscc_lib::DataHasher` with `RefCell<Option<...>>` for one-shot
/// finalize semantics (Magnus instance methods receive `&self`, not `&mut self`).
#[magnus::wrap(class = "IsccLib::DataHasher")]
struct RbDataHasher {
    inner: RefCell<Option<iscc_lib::DataHasher>>,
}

impl RbDataHasher {
    /// Create a new `RbDataHasher`.
    fn rb_new() -> Self {
        Self {
            inner: RefCell::new(Some(iscc_lib::DataHasher::new())),
        }
    }

    /// Push binary data into the hasher.
    ///
    /// Raises `RuntimeError` if called after `finalize`.
    fn update(&self, data: RString) -> Result<(), Error> {
        let mut inner = self.inner.borrow_mut();
        let hasher = inner.as_mut().ok_or_else(|| {
            Error::new(
                magnus::exception::runtime_error(),
                "DataHasher already finalized",
            )
        })?;
        // Safety: the slice is passed directly to a pure Rust function
        // and not held across any Ruby API calls.
        let bytes = unsafe { data.as_slice() };
        hasher.update(bytes);
        Ok(())
    }

    /// Consume the hasher and produce a Data-Code result hash.
    ///
    /// Returns an `RHash` with key `"iscc"`. Raises `RuntimeError` if
    /// called more than once.
    fn finalize(&self, bits: u32) -> Result<RHash, Error> {
        let hasher = self.inner.borrow_mut().take().ok_or_else(|| {
            Error::new(
                magnus::exception::runtime_error(),
                "DataHasher already finalized",
            )
        })?;
        let r = hasher.finalize(bits).map_err(to_magnus_err)?;
        let ruby = Ruby::get().expect("called from Ruby");
        let hash = ruby.hash_new();
        hash.aset("iscc", r.iscc)?;
        Ok(hash)
    }
}

/// Streaming Instance-Code generator for Ruby.
///
/// Wraps `iscc_lib::InstanceHasher` with `RefCell<Option<...>>` for one-shot
/// finalize semantics.
#[magnus::wrap(class = "IsccLib::InstanceHasher")]
struct RbInstanceHasher {
    inner: RefCell<Option<iscc_lib::InstanceHasher>>,
}

impl RbInstanceHasher {
    /// Create a new `RbInstanceHasher`.
    fn rb_new() -> Self {
        Self {
            inner: RefCell::new(Some(iscc_lib::InstanceHasher::new())),
        }
    }

    /// Push binary data into the hasher.
    ///
    /// Raises `RuntimeError` if called after `finalize`.
    fn update(&self, data: RString) -> Result<(), Error> {
        let mut inner = self.inner.borrow_mut();
        let hasher = inner.as_mut().ok_or_else(|| {
            Error::new(
                magnus::exception::runtime_error(),
                "InstanceHasher already finalized",
            )
        })?;
        // Safety: the slice is passed directly to a pure Rust function
        // and not held across any Ruby API calls.
        let bytes = unsafe { data.as_slice() };
        hasher.update(bytes);
        Ok(())
    }

    /// Consume the hasher and produce an Instance-Code result hash.
    ///
    /// Returns an `RHash` with keys `"iscc"`, `"datahash"`, `"filesize"`.
    /// Raises `RuntimeError` if called more than once.
    fn finalize(&self, bits: u32) -> Result<RHash, Error> {
        let hasher = self.inner.borrow_mut().take().ok_or_else(|| {
            Error::new(
                magnus::exception::runtime_error(),
                "InstanceHasher already finalized",
            )
        })?;
        let r = hasher.finalize(bits).map_err(to_magnus_err)?;
        let ruby = Ruby::get().expect("called from Ruby");
        let hash = ruby.hash_new();
        hash.aset("iscc", r.iscc)?;
        hash.aset("datahash", r.datahash)?;
        hash.aset("filesize", r.filesize)?;
        Ok(hash)
    }
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

    // Algorithm primitives
    module.define_module_function("sliding_window", function!(sliding_window, 2))?;
    module.define_module_function("alg_simhash", function!(alg_simhash, 1))?;
    module.define_module_function("alg_minhash_256", function!(alg_minhash_256, 1))?;
    module.define_module_function("alg_cdc_chunks", function!(alg_cdc_chunks, 3))?;
    module.define_module_function("soft_hash_video_v0", function!(soft_hash_video_v0, 2))?;

    // Streaming hasher classes (Ruby wrapper reopens to add defaults + result wrapping)
    let data_hasher = module.define_class("DataHasher", ruby.class_object())?;
    data_hasher.define_singleton_method("new", function!(RbDataHasher::rb_new, 0))?;
    data_hasher.define_method("_update", method!(RbDataHasher::update, 1))?;
    data_hasher.define_method("_finalize", method!(RbDataHasher::finalize, 1))?;

    let instance_hasher = module.define_class("InstanceHasher", ruby.class_object())?;
    instance_hasher.define_singleton_method("new", function!(RbInstanceHasher::rb_new, 0))?;
    instance_hasher.define_method("_update", method!(RbInstanceHasher::update, 1))?;
    instance_hasher.define_method("_finalize", method!(RbInstanceHasher::finalize, 1))?;

    // Constants
    module.const_set("META_TRIM_NAME", iscc_lib::META_TRIM_NAME)?;
    module.const_set("META_TRIM_DESCRIPTION", iscc_lib::META_TRIM_DESCRIPTION)?;
    module.const_set("META_TRIM_META", iscc_lib::META_TRIM_META)?;
    module.const_set("IO_READ_SIZE", iscc_lib::IO_READ_SIZE)?;
    module.const_set("TEXT_NGRAM_SIZE", iscc_lib::TEXT_NGRAM_SIZE)?;

    Ok(())
}
