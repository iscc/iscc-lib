//! napi-rs bindings for iscc-lib, exposed as `@iscc/lib` native Node.js addon.
//!
//! This module provides the native Rust-backed functions for the `@iscc/lib`
//! npm package. Each function is a thin wrapper around the corresponding
//! `iscc_lib` API, converting napi types to Rust types and mapping errors.

use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

/// Generate a Meta-Code from name and optional metadata.
///
/// Produces an ISCC Meta-Code by hashing the provided name, description,
/// and metadata fields using the SimHash algorithm.
#[napi(js_name = "gen_meta_code_v0")]
pub fn gen_meta_code_v0(
    name: String,
    description: Option<String>,
    meta: Option<String>,
    bits: Option<u32>,
) -> napi::Result<String> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_meta_code_v0(&name, description.as_deref(), meta.as_deref(), bits)
        .map(|r| r.iscc)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Generate a Text-Code from plain text content.
///
/// Produces an ISCC Content-Code for text using MinHash-based
/// similarity hashing.
#[napi(js_name = "gen_text_code_v0")]
pub fn gen_text_code_v0(text: String, bits: Option<u32>) -> napi::Result<String> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_text_code_v0(&text, bits)
        .map(|r| r.iscc)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Generate an Image-Code from pixel data.
///
/// Produces an ISCC Content-Code for images from 1024 grayscale pixels
/// (32×32) using a DCT-based perceptual hash.
#[napi(js_name = "gen_image_code_v0")]
pub fn gen_image_code_v0(pixels: Buffer, bits: Option<u32>) -> napi::Result<String> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_image_code_v0(pixels.as_ref(), bits)
        .map(|r| r.iscc)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Generate an Audio-Code from a Chromaprint feature vector.
///
/// Produces an ISCC Content-Code for audio from signed integer
/// Chromaprint fingerprint features using multi-stage SimHash.
#[napi(js_name = "gen_audio_code_v0")]
pub fn gen_audio_code_v0(cv: Vec<i32>, bits: Option<u32>) -> napi::Result<String> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_audio_code_v0(&cv, bits)
        .map(|r| r.iscc)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Generate a Video-Code from frame signature data.
///
/// Produces an ISCC Content-Code for video from MPEG-7 frame
/// signature vectors using WTA-Hash.
#[napi(js_name = "gen_video_code_v0")]
pub fn gen_video_code_v0(frame_sigs: Vec<Vec<i32>>, bits: Option<u32>) -> napi::Result<String> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_video_code_v0(&frame_sigs, bits)
        .map(|r| r.iscc)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Generate a Mixed-Code from multiple Content-Code strings.
///
/// Produces a Mixed Content-Code by combining multiple ISCC Content-Codes
/// of different types using SimHash.
#[napi(js_name = "gen_mixed_code_v0")]
pub fn gen_mixed_code_v0(codes: Vec<String>, bits: Option<u32>) -> napi::Result<String> {
    let bits = bits.unwrap_or(64);
    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
    iscc_lib::gen_mixed_code_v0(&refs, bits)
        .map(|r| r.iscc)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Generate a Data-Code from raw byte data.
///
/// Produces an ISCC Data-Code by splitting data into content-defined
/// chunks and applying MinHash for similarity hashing.
#[napi(js_name = "gen_data_code_v0")]
pub fn gen_data_code_v0(data: Buffer, bits: Option<u32>) -> napi::Result<String> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_data_code_v0(data.as_ref(), bits)
        .map(|r| r.iscc)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Generate an Instance-Code from raw byte data.
///
/// Produces an ISCC Instance-Code by hashing the complete byte stream
/// with BLAKE3. Returns the ISCC string with "ISCC:" prefix.
#[napi(js_name = "gen_instance_code_v0")]
pub fn gen_instance_code_v0(data: Buffer, bits: Option<u32>) -> napi::Result<String> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_instance_code_v0(data.as_ref(), bits)
        .map(|r| r.iscc)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Generate a composite ISCC-CODE from individual unit codes.
///
/// Combines multiple ISCC unit codes into a single composite ISCC-CODE.
/// Requires at least Data-Code and Instance-Code.
#[napi(js_name = "gen_iscc_code_v0")]
pub fn gen_iscc_code_v0(codes: Vec<String>, wide: Option<bool>) -> napi::Result<String> {
    let wide = wide.unwrap_or(false);
    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
    iscc_lib::gen_iscc_code_v0(&refs, wide)
        .map(|r| r.iscc)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Clean and normalize text for display.
///
/// Applies NFKC normalization, removes control characters (except newlines),
/// normalizes `\r\n` to `\n`, collapses consecutive empty lines, and strips
/// leading/trailing whitespace.
#[napi(js_name = "text_clean")]
pub fn text_clean(text: String) -> String {
    iscc_lib::text_clean(&text)
}

/// Remove newlines and collapse whitespace to single spaces.
///
/// Converts multi-line text into a single normalized line.
#[napi(js_name = "text_remove_newlines")]
pub fn text_remove_newlines(text: String) -> String {
    iscc_lib::text_remove_newlines(&text)
}

/// Trim text so its UTF-8 encoded size does not exceed `nbytes`.
///
/// Multi-byte characters that would be split are dropped entirely.
/// Leading/trailing whitespace is stripped from the result.
#[napi(js_name = "text_trim")]
pub fn text_trim(text: String, nbytes: u32) -> String {
    iscc_lib::text_trim(&text, nbytes as usize)
}

/// Normalize and simplify text for similarity hashing.
///
/// Applies NFD normalization, lowercasing, removes whitespace and characters
/// in Unicode categories C (control), M (mark), and P (punctuation), then
/// recombines with NFKC normalization.
#[napi(js_name = "text_collapse")]
pub fn text_collapse(text: String) -> String {
    iscc_lib::text_collapse(&text)
}

/// Encode bytes as base64url (RFC 4648 §5, no padding).
///
/// Returns a URL-safe base64 encoded string without padding characters.
#[napi(js_name = "encode_base64")]
pub fn encode_base64(data: Buffer) -> String {
    iscc_lib::encode_base64(data.as_ref())
}

/// Decompose a composite ISCC-CODE into individual ISCC-UNITs.
///
/// Accepts a normalized ISCC-CODE or concatenated ISCC-UNIT sequence.
/// The optional "ISCC:" prefix is stripped before decoding.
/// Returns an array of base32-encoded ISCC-UNIT strings (without prefix).
#[napi(js_name = "iscc_decompose")]
pub fn iscc_decompose(iscc_code: String) -> napi::Result<Vec<String>> {
    iscc_lib::iscc_decompose(&iscc_code).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Run all conformance tests against vendored test vectors.
///
/// Returns `true` if all tests pass, `false` if any fail.
#[napi(js_name = "conformance_selftest")]
pub fn conformance_selftest() -> bool {
    iscc_lib::conformance_selftest()
}

/// Generate sliding window n-grams from a string.
///
/// Returns overlapping substrings of `width` Unicode characters, advancing
/// by one character at a time. Throws if width is less than 2.
#[napi(js_name = "sliding_window")]
pub fn sliding_window(seq: String, width: u32) -> napi::Result<Vec<String>> {
    if width < 2 {
        return Err(napi::Error::from_reason(
            "Sliding window width must be 2 or bigger.",
        ));
    }
    Ok(iscc_lib::sliding_window(&seq, width as usize))
}

// ── Algorithm primitives ─────────────────────────────────────────────────────

/// Compute a SimHash from a sequence of equal-length hash digests.
///
/// Returns a similarity-preserving hash as a Buffer. Each output bit is set
/// when its frequency meets or exceeds half the input count. Returns 32
/// zero bytes for empty input.
#[napi(js_name = "alg_simhash")]
pub fn alg_simhash(hash_digests: Vec<Buffer>) -> Buffer {
    iscc_lib::alg_simhash(&hash_digests).into()
}

/// Compute a 256-bit MinHash digest from 32-bit integer features.
///
/// Uses 64 universal hash functions with bit-interleaved compression to
/// produce a 32-byte similarity-preserving digest.
#[napi(js_name = "alg_minhash_256")]
pub fn alg_minhash_256(features: Vec<u32>) -> Buffer {
    iscc_lib::alg_minhash_256(&features).into()
}

/// Split data into content-defined chunks using gear rolling hash.
///
/// Returns at least one chunk (empty bytes for empty input). When `utf32`
/// is true, aligns cut points to 4-byte boundaries. Default
/// `avg_chunk_size` is 1024.
#[napi(js_name = "alg_cdc_chunks")]
pub fn alg_cdc_chunks(data: Buffer, utf32: bool, avg_chunk_size: Option<u32>) -> Vec<Buffer> {
    let avg = avg_chunk_size.unwrap_or(1024);
    iscc_lib::alg_cdc_chunks(data.as_ref(), utf32, avg)
        .iter()
        .map(|c| Buffer::from(c.to_vec()))
        .collect()
}

/// Compute a similarity-preserving hash from video frame signatures.
///
/// Returns raw bytes of length `bits / 8`. Default `bits` is 64.
/// Throws if `frame_sigs` is empty.
#[napi(js_name = "soft_hash_video_v0")]
pub fn soft_hash_video_v0(frame_sigs: Vec<Vec<i32>>, bits: Option<u32>) -> napi::Result<Buffer> {
    let bits = bits.unwrap_or(64);
    iscc_lib::soft_hash_video_v0(&frame_sigs, bits)
        .map(|r| r.into())
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}
