//! wasm-bindgen bindings for iscc-lib, exposed as `@iscc/wasm` npm package.
//!
//! This module provides browser-compatible WASM bindings for the `iscc-lib`
//! core. Each function is a thin wrapper around the corresponding `iscc_lib`
//! API, converting wasm-bindgen types to Rust types and mapping errors to
//! `JsError`.

use wasm_bindgen::prelude::*;

// ── Constants ────────────────────────────────────────────────────────────────

/// Maximum byte length for the name field after trimming.
#[wasm_bindgen(js_name = "META_TRIM_NAME")]
pub fn meta_trim_name() -> u32 {
    iscc_lib::META_TRIM_NAME as u32
}

/// Maximum byte length for the description field after trimming.
#[wasm_bindgen(js_name = "META_TRIM_DESCRIPTION")]
pub fn meta_trim_description() -> u32 {
    iscc_lib::META_TRIM_DESCRIPTION as u32
}

/// Default read buffer size for streaming I/O (4 MB).
#[wasm_bindgen(js_name = "IO_READ_SIZE")]
pub fn io_read_size() -> u32 {
    iscc_lib::IO_READ_SIZE as u32
}

/// Sliding window width for text n-gram generation.
#[wasm_bindgen(js_name = "TEXT_NGRAM_SIZE")]
pub fn text_ngram_size() -> u32 {
    iscc_lib::TEXT_NGRAM_SIZE as u32
}

/// Generate a Meta-Code from name and optional metadata.
///
/// Produces an ISCC Meta-Code by hashing the provided name, description,
/// and metadata fields using the SimHash algorithm.
#[wasm_bindgen]
pub fn gen_meta_code_v0(
    name: &str,
    description: Option<String>,
    meta: Option<String>,
    bits: Option<u32>,
) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_meta_code_v0(name, description.as_deref(), meta.as_deref(), bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate a Text-Code from plain text content.
///
/// Produces an ISCC Content-Code for text using MinHash-based
/// similarity hashing.
#[wasm_bindgen]
pub fn gen_text_code_v0(text: &str, bits: Option<u32>) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_text_code_v0(text, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate an Image-Code from pixel data.
///
/// Produces an ISCC Content-Code for images from 1024 grayscale pixels
/// (32x32) using a DCT-based perceptual hash.
#[wasm_bindgen]
pub fn gen_image_code_v0(pixels: &[u8], bits: Option<u32>) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_image_code_v0(pixels, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate an Audio-Code from a Chromaprint feature vector.
///
/// Produces an ISCC Content-Code for audio from signed integer
/// Chromaprint fingerprint features using multi-stage SimHash.
#[wasm_bindgen]
pub fn gen_audio_code_v0(cv: Vec<i32>, bits: Option<u32>) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_audio_code_v0(&cv, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate a Video-Code from frame signature data.
///
/// Produces an ISCC Content-Code for video from MPEG-7 frame
/// signature vectors using WTA-Hash. Accepts a JS array of arrays of i32.
#[wasm_bindgen]
pub fn gen_video_code_v0(frame_sigs: JsValue, bits: Option<u32>) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    let frame_sigs: Vec<Vec<i32>> =
        serde_wasm_bindgen::from_value(frame_sigs).map_err(|e| JsError::new(&e.to_string()))?;
    iscc_lib::gen_video_code_v0(&frame_sigs, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate a Mixed-Code from multiple Content-Code strings.
///
/// Produces a Mixed Content-Code by combining multiple ISCC Content-Codes
/// of different types using SimHash. Accepts a JS array of strings.
#[wasm_bindgen]
pub fn gen_mixed_code_v0(codes: JsValue, bits: Option<u32>) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    let codes: Vec<String> =
        serde_wasm_bindgen::from_value(codes).map_err(|e| JsError::new(&e.to_string()))?;
    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
    iscc_lib::gen_mixed_code_v0(&refs, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate a Data-Code from raw byte data.
///
/// Produces an ISCC Data-Code by splitting data into content-defined
/// chunks and applying MinHash for similarity hashing.
#[wasm_bindgen]
pub fn gen_data_code_v0(data: &[u8], bits: Option<u32>) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_data_code_v0(data, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate an Instance-Code from raw byte data.
///
/// Produces an ISCC Instance-Code by hashing the complete byte stream
/// with BLAKE3. Returns the ISCC string with "ISCC:" prefix.
#[wasm_bindgen]
pub fn gen_instance_code_v0(data: &[u8], bits: Option<u32>) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_instance_code_v0(data, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate a composite ISCC-CODE from individual unit codes.
///
/// Combines multiple ISCC unit codes into a single composite ISCC-CODE.
/// Requires at least Data-Code and Instance-Code. Accepts a JS array of strings.
#[wasm_bindgen]
pub fn gen_iscc_code_v0(codes: JsValue, wide: Option<bool>) -> Result<String, JsError> {
    let wide = wide.unwrap_or(false);
    let codes: Vec<String> =
        serde_wasm_bindgen::from_value(codes).map_err(|e| JsError::new(&e.to_string()))?;
    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
    iscc_lib::gen_iscc_code_v0(&refs, wide)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

// ── Text utilities ──────────────────────────────────────────────────────────

/// Clean and normalize text for display.
///
/// Applies NFKC normalization, removes control characters (except newlines),
/// normalizes `\r\n` to `\n`, collapses consecutive empty lines, and strips
/// leading/trailing whitespace.
#[wasm_bindgen]
pub fn text_clean(text: &str) -> String {
    iscc_lib::text_clean(text)
}

/// Remove newlines and collapse whitespace to single spaces.
///
/// Converts multi-line text into a single normalized line.
#[wasm_bindgen]
pub fn text_remove_newlines(text: &str) -> String {
    iscc_lib::text_remove_newlines(text)
}

/// Trim text so its UTF-8 encoded size does not exceed `nbytes`.
///
/// Multi-byte characters that would be split are dropped entirely.
/// Leading/trailing whitespace is stripped from the result.
#[wasm_bindgen]
pub fn text_trim(text: &str, nbytes: u32) -> String {
    iscc_lib::text_trim(text, nbytes as usize)
}

/// Normalize and simplify text for similarity hashing.
///
/// Applies NFD normalization, lowercasing, removes whitespace and characters
/// in Unicode categories C (control), M (mark), and P (punctuation), then
/// recombines with NFKC normalization.
#[wasm_bindgen]
pub fn text_collapse(text: &str) -> String {
    iscc_lib::text_collapse(text)
}

// ── Encoding ────────────────────────────────────────────────────────────────

/// Convert a JSON string into a `data:` URL with JCS canonicalization.
///
/// Uses `application/ld+json` media type when the JSON contains an `@context`
/// key, otherwise `application/json`.
#[wasm_bindgen]
pub fn json_to_data_url(json: &str) -> Result<String, JsError> {
    iscc_lib::json_to_data_url(json).map_err(|e| JsError::new(&e.to_string()))
}

/// Encode bytes as base64url (RFC 4648 §5, no padding).
///
/// Returns a URL-safe base64 encoded string without padding characters.
#[wasm_bindgen]
pub fn encode_base64(data: &[u8]) -> String {
    iscc_lib::encode_base64(data)
}

// ── Codec ───────────────────────────────────────────────────────────────────

/// Encode raw ISCC header components and digest into a base32 ISCC unit string.
///
/// Takes integer type identifiers and a raw digest, returns a base32-encoded
/// ISCC unit string (without "ISCC:" prefix).
#[wasm_bindgen]
pub fn encode_component(
    mtype: u8,
    stype: u8,
    version: u8,
    bit_length: u32,
    digest: &[u8],
) -> Result<String, JsError> {
    iscc_lib::encode_component(mtype, stype, version, bit_length, digest)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Result of decoding an ISCC unit string.
#[wasm_bindgen(getter_with_clone)]
pub struct IsccDecodeResult {
    /// MainType enum value (0–7).
    pub maintype: u8,
    /// SubType enum value (0–7).
    pub subtype: u8,
    /// Version enum value.
    pub version: u8,
    /// Length index from the header.
    pub length: u8,
    /// Raw digest bytes truncated to the encoded bit-length.
    pub digest: Vec<u8>,
}

/// Decode an ISCC unit string into header components and raw digest.
///
/// Returns an object with `maintype`, `subtype`, `version`, `length`, and
/// `digest` fields. Strips an optional "ISCC:" prefix before decoding.
#[wasm_bindgen]
pub fn iscc_decode(iscc: &str) -> Result<IsccDecodeResult, JsError> {
    let (mt, st, vs, li, digest) =
        iscc_lib::iscc_decode(iscc).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(IsccDecodeResult {
        maintype: mt,
        subtype: st,
        version: vs,
        length: li,
        digest,
    })
}

/// Decompose a composite ISCC-CODE into individual ISCC-UNITs.
///
/// Accepts a normalized ISCC-CODE or concatenated ISCC-UNIT sequence.
/// The optional "ISCC:" prefix is stripped before decoding.
/// Returns an array of base32-encoded ISCC-UNIT strings (without prefix).
#[wasm_bindgen]
pub fn iscc_decompose(iscc_code: &str) -> Result<Vec<String>, JsError> {
    iscc_lib::iscc_decompose(iscc_code).map_err(|e| JsError::new(&e.to_string()))
}

// ── Conformance ─────────────────────────────────────────────────────────────

/// Run all conformance tests against vendored test vectors.
///
/// Returns `true` if all tests pass, `false` if any fail.
#[cfg(feature = "conformance")]
#[wasm_bindgen]
pub fn conformance_selftest() -> bool {
    iscc_lib::conformance_selftest()
}

// ── Sliding window ──────────────────────────────────────────────────────────

/// Generate sliding window n-grams from a string.
///
/// Returns overlapping substrings of `width` Unicode characters, advancing
/// by one character at a time. Throws if width is less than 2.
#[wasm_bindgen]
pub fn sliding_window(seq: &str, width: u32) -> Result<Vec<String>, JsError> {
    iscc_lib::sliding_window(seq, width as usize).map_err(|e| JsError::new(&e.to_string()))
}

// ── Algorithm primitives ─────────────────────────────────────────────────────

/// Compute a SimHash from a sequence of equal-length hash digests.
///
/// Accepts a JS array of `Uint8Array` values. Returns a similarity-preserving
/// hash whose length matches the input digest length. Returns 32 zero bytes
/// for empty input. Throws on mismatched digest lengths.
#[wasm_bindgen]
pub fn alg_simhash(hash_digests: JsValue) -> Result<Vec<u8>, JsError> {
    let digests: Vec<Vec<u8>> =
        serde_wasm_bindgen::from_value(hash_digests).map_err(|e| JsError::new(&e.to_string()))?;
    iscc_lib::alg_simhash(&digests).map_err(|e| JsError::new(&e.to_string()))
}

/// Compute a 256-bit MinHash digest from 32-bit integer features.
///
/// Uses 64 universal hash functions with bit-interleaved compression to
/// produce a 32-byte similarity-preserving digest.
#[wasm_bindgen]
pub fn alg_minhash_256(features: Vec<u32>) -> Vec<u8> {
    iscc_lib::alg_minhash_256(&features)
}

/// Split data into content-defined chunks using gear rolling hash.
///
/// Returns a JS array of `Uint8Array` chunks. At least one chunk is always
/// returned (empty bytes for empty input). When `utf32` is true, aligns cut
/// points to 4-byte boundaries. Default `avg_chunk_size` is 1024.
#[wasm_bindgen]
pub fn alg_cdc_chunks(
    data: &[u8],
    utf32: bool,
    avg_chunk_size: Option<u32>,
) -> Result<JsValue, JsError> {
    let avg = avg_chunk_size.unwrap_or(1024);
    let chunks: Vec<Vec<u8>> = iscc_lib::alg_cdc_chunks(data, utf32, avg)
        .iter()
        .map(|c| c.to_vec())
        .collect();
    serde_wasm_bindgen::to_value(&chunks).map_err(|e| JsError::new(&e.to_string()))
}

/// Compute a similarity-preserving hash from video frame signatures.
///
/// Accepts a JS array of arrays of `i32`. Returns raw bytes of length
/// `bits / 8`. Default `bits` is 64. Throws if `frame_sigs` is empty.
#[wasm_bindgen]
pub fn soft_hash_video_v0(frame_sigs: JsValue, bits: Option<u32>) -> Result<Vec<u8>, JsError> {
    let bits = bits.unwrap_or(64);
    let frame_sigs: Vec<Vec<i32>> =
        serde_wasm_bindgen::from_value(frame_sigs).map_err(|e| JsError::new(&e.to_string()))?;
    iscc_lib::soft_hash_video_v0(&frame_sigs, bits).map_err(|e| JsError::new(&e.to_string()))
}

// ── Streaming hashers ─────────────────────────────────────────────────────────

/// Streaming Data-Code generator.
///
/// Incrementally processes data with content-defined chunking and MinHash
/// to produce results identical to `gen_data_code_v0`. Follows the
/// `new() → update() → finalize()` pattern.
#[wasm_bindgen]
pub struct DataHasher {
    inner: Option<iscc_lib::DataHasher>,
}

impl Default for DataHasher {
    /// Create a new `DataHasher` (delegates to `new()`).
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl DataHasher {
    /// Create a new `DataHasher`.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Some(iscc_lib::DataHasher::new()),
        }
    }

    /// Push data into the hasher.
    pub fn update(&mut self, data: &[u8]) -> Result<(), JsError> {
        self.inner
            .as_mut()
            .ok_or_else(|| JsError::new("DataHasher already finalized"))
            .map(|h| h.update(data))
    }

    /// Consume the hasher and produce a Data-Code ISCC string.
    ///
    /// After calling `finalize`, subsequent calls to `update` or `finalize`
    /// will throw. Default `bits` is 64.
    pub fn finalize(&mut self, bits: Option<u32>) -> Result<String, JsError> {
        let hasher = self
            .inner
            .take()
            .ok_or_else(|| JsError::new("DataHasher already finalized"))?;
        hasher
            .finalize(bits.unwrap_or(64))
            .map(|r| r.iscc)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// Streaming Instance-Code generator.
///
/// Incrementally hashes data with BLAKE3 to produce results identical
/// to `gen_instance_code_v0`. Follows the
/// `new() → update() → finalize()` pattern.
#[wasm_bindgen]
pub struct InstanceHasher {
    inner: Option<iscc_lib::InstanceHasher>,
}

impl Default for InstanceHasher {
    /// Create a new `InstanceHasher` (delegates to `new()`).
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl InstanceHasher {
    /// Create a new `InstanceHasher`.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Some(iscc_lib::InstanceHasher::new()),
        }
    }

    /// Push data into the hasher.
    pub fn update(&mut self, data: &[u8]) -> Result<(), JsError> {
        self.inner
            .as_mut()
            .ok_or_else(|| JsError::new("InstanceHasher already finalized"))
            .map(|h| h.update(data))
    }

    /// Consume the hasher and produce an Instance-Code ISCC string.
    ///
    /// After calling `finalize`, subsequent calls to `update` or `finalize`
    /// will throw. Default `bits` is 64.
    pub fn finalize(&mut self, bits: Option<u32>) -> Result<String, JsError> {
        let hasher = self
            .inner
            .take()
            .ok_or_else(|| JsError::new("InstanceHasher already finalized"))?;
        hasher
            .finalize(bits.unwrap_or(64))
            .map(|r| r.iscc)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}
