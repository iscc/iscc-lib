//! napi-rs bindings for iscc-lib, exposed as `@iscc/lib` native Node.js addon.
//!
//! This module provides the native Rust-backed functions for the `@iscc/lib`
//! npm package. Each function is a thin wrapper around the corresponding
//! `iscc_lib` API, converting napi types to Rust types and mapping errors.

use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

// ── Algorithm constants ───────────────────────────────────────────────────────

/// Maximum byte length for the name field after trimming.
#[napi(js_name = "META_TRIM_NAME")]
pub const META_TRIM_NAME: u32 = iscc_lib::META_TRIM_NAME as u32;

/// Maximum byte length for the description field after trimming.
#[napi(js_name = "META_TRIM_DESCRIPTION")]
pub const META_TRIM_DESCRIPTION: u32 = iscc_lib::META_TRIM_DESCRIPTION as u32;

/// Maximum byte length for the meta field payload after decoding.
#[napi(js_name = "META_TRIM_META")]
pub const META_TRIM_META: u32 = iscc_lib::META_TRIM_META as u32;

/// Default read buffer size for streaming I/O (4 MB).
#[napi(js_name = "IO_READ_SIZE")]
pub const IO_READ_SIZE: u32 = iscc_lib::IO_READ_SIZE as u32;

/// Sliding window width for text n-gram generation.
#[napi(js_name = "TEXT_NGRAM_SIZE")]
pub const TEXT_NGRAM_SIZE: u32 = iscc_lib::TEXT_NGRAM_SIZE as u32;

// ── Codec functions ──────────────────────────────────────────────────────────

/// Encode raw ISCC header components and digest into a base32 ISCC unit string.
///
/// Takes integer type identifiers and a raw digest, returns a base32-encoded
/// ISCC unit string (without "ISCC:" prefix).
#[napi(js_name = "encode_component")]
pub fn encode_component(
    mtype: u8,
    stype: u8,
    version: u8,
    bit_length: u32,
    digest: Buffer,
) -> napi::Result<String> {
    iscc_lib::encode_component(mtype, stype, version, bit_length, digest.as_ref())
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Result of decoding an ISCC unit string.
#[napi(object)]
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
    pub digest: Buffer,
}

/// Decode an ISCC unit string into header components and raw digest.
///
/// Returns an object with `maintype`, `subtype`, `version`, `length`, and
/// `digest` fields. Strips an optional "ISCC:" prefix before decoding.
#[napi(js_name = "iscc_decode")]
pub fn iscc_decode(iscc: String) -> napi::Result<IsccDecodeResult> {
    let (mt, st, vs, li, digest) =
        iscc_lib::iscc_decode(&iscc).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(IsccDecodeResult {
        maintype: mt,
        subtype: st,
        version: vs,
        length: li,
        digest: digest.into(),
    })
}

/// Convert a JSON string into a `data:` URL with JCS canonicalization.
///
/// Uses `application/ld+json` media type when the JSON contains an `@context`
/// key, otherwise `application/json`.
#[napi(js_name = "json_to_data_url")]
pub fn json_to_data_url(json: String) -> napi::Result<String> {
    iscc_lib::json_to_data_url(&json).map_err(|e| napi::Error::from_reason(e.to_string()))
}

// ── Code generators ──────────────────────────────────────────────────────────

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

/// Result of generating a composite ISCC-CODE from a file in a single pass.
#[napi(object)]
pub struct NapiSumCodeResult {
    /// Composite ISCC-CODE string.
    pub iscc: String,
    /// Hex-encoded BLAKE3 multihash of the file.
    pub datahash: String,
    /// Byte length of the file.
    pub filesize: i64,
}

/// Generate a composite ISCC-CODE from a file in a single pass.
///
/// Reads the file at `path` and produces Data-Code + Instance-Code in one
/// pass, then combines them into an ISCC-CODE. Returns an object with
/// `iscc`, `datahash`, and `filesize` fields.
#[napi(js_name = "gen_sum_code_v0")]
pub fn gen_sum_code_v0(
    path: String,
    bits: Option<u32>,
    wide: Option<bool>,
) -> napi::Result<NapiSumCodeResult> {
    let bits = bits.unwrap_or(64);
    let wide = wide.unwrap_or(false);
    let result = iscc_lib::gen_sum_code_v0(std::path::Path::new(&path), bits, wide)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(NapiSumCodeResult {
        iscc: result.iscc,
        datahash: result.datahash,
        filesize: result.filesize as i64,
    })
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
    iscc_lib::sliding_window(&seq, width as usize)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

// ── Algorithm primitives ─────────────────────────────────────────────────────

/// Compute a SimHash from a sequence of equal-length hash digests.
///
/// Returns a similarity-preserving hash as a Buffer. Each output bit is set
/// when its frequency meets or exceeds half the input count. Returns 32
/// zero bytes for empty input. Throws on mismatched digest lengths.
#[napi(js_name = "alg_simhash")]
pub fn alg_simhash(hash_digests: Vec<Buffer>) -> napi::Result<Buffer> {
    iscc_lib::alg_simhash(&hash_digests)
        .map(|v| v.into())
        .map_err(|e| napi::Error::from_reason(e.to_string()))
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
        .into_iter()
        .map(Buffer::from)
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

// ── Streaming hashers ─────────────────────────────────────────────────────────

/// Streaming Data-Code generator.
///
/// Incrementally processes data with content-defined chunking and MinHash
/// to produce results identical to `gen_data_code_v0`. Follows the
/// `new() → update() → finalize()` pattern.
#[napi(js_name = "DataHasher")]
pub struct NapiDataHasher {
    inner: Option<iscc_lib::DataHasher>,
}

impl Default for NapiDataHasher {
    /// Create a new `NapiDataHasher` (delegates to `new()`).
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
impl NapiDataHasher {
    /// Create a new `DataHasher`.
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Some(iscc_lib::DataHasher::new()),
        }
    }

    /// Push data into the hasher.
    #[napi]
    pub fn update(&mut self, data: Buffer) -> napi::Result<()> {
        self.inner
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("DataHasher already finalized"))
            .map(|h| h.update(&data))
    }

    /// Consume the hasher and produce a Data-Code ISCC string.
    ///
    /// After calling `finalize`, subsequent calls to `update` or `finalize`
    /// will throw. Default `bits` is 64.
    #[napi(js_name = "finalize")]
    pub fn finalize_code(&mut self, bits: Option<u32>) -> napi::Result<String> {
        let hasher = self
            .inner
            .take()
            .ok_or_else(|| napi::Error::from_reason("DataHasher already finalized"))?;
        hasher
            .finalize(bits.unwrap_or(64))
            .map(|r| r.iscc)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }
}

/// Streaming Instance-Code generator.
///
/// Incrementally hashes data with BLAKE3 to produce results identical
/// to `gen_instance_code_v0`. Follows the
/// `new() → update() → finalize()` pattern.
#[napi(js_name = "InstanceHasher")]
pub struct NapiInstanceHasher {
    inner: Option<iscc_lib::InstanceHasher>,
}

impl Default for NapiInstanceHasher {
    /// Create a new `NapiInstanceHasher` (delegates to `new()`).
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
impl NapiInstanceHasher {
    /// Create a new `InstanceHasher`.
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Some(iscc_lib::InstanceHasher::new()),
        }
    }

    /// Push data into the hasher.
    #[napi]
    pub fn update(&mut self, data: Buffer) -> napi::Result<()> {
        self.inner
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("InstanceHasher already finalized"))
            .map(|h| h.update(&data))
    }

    /// Consume the hasher and produce an Instance-Code ISCC string.
    ///
    /// After calling `finalize`, subsequent calls to `update` or `finalize`
    /// will throw. Default `bits` is 64.
    #[napi(js_name = "finalize")]
    pub fn finalize_code(&mut self, bits: Option<u32>) -> napi::Result<String> {
        let hasher = self
            .inner
            .take()
            .ok_or_else(|| napi::Error::from_reason("InstanceHasher already finalized"))?;
        hasher
            .finalize(bits.unwrap_or(64))
            .map(|r| r.iscc)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }
}
