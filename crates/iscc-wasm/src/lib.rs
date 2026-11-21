//! wasm-bindgen bindings for iscc-lib, exposed as `@iscc/wasm` npm package.
//!
//! This module provides browser-compatible WASM bindings for the `iscc-lib`
//! core. Each function is a thin wrapper around the corresponding `iscc_lib`
//! API, converting wasm-bindgen types to Rust types and mapping errors to
//! `JsError`.

use std::fmt::Display;
use wasm_bindgen::prelude::*;

// ── Integer narrowing ─────────────────────────────────────────────────────────

/// Validate a JS number before narrowing it to an integer type.
///
/// The wasm-bindgen glue wraps a JS number passed as an integer type (`>>> 0`
/// for `u32`, typed-array copies for `Vec<i32>`/`Vec<u32>`), so every numeric
/// parameter is taken as `f64` and checked here. Rejects non-finite,
/// non-integral and out-of-range (`< min` or `>= max_exclusive`) values so an
/// invalid JS number throws instead of turning into a different valid integer.
/// `-0` passes as 0. The `name` is formatted only on rejection.
fn checked(value: f64, min: f64, max_exclusive: f64, name: impl Display) -> Result<f64, JsError> {
    if !value.is_finite() || value.fract() != 0.0 || value < min || value >= max_exclusive {
        return Err(JsError::new(&format!("Invalid {name}: {value}")));
    }
    Ok(value)
}

/// Narrow a JS number to `u8`, rejecting anything outside \[0, 2^8).
fn to_u8(value: f64, name: impl Display) -> Result<u8, JsError> {
    checked(value, 0.0, 256.0, name).map(|v| v as u8)
}

/// Narrow a JS number to `u32`, rejecting anything outside \[0, 2^32).
fn to_u32(value: f64, name: impl Display) -> Result<u32, JsError> {
    checked(value, 0.0, 4_294_967_296.0, name).map(|v| v as u32)
}

/// Narrow a JS number to `i32`, rejecting anything outside \[-2^31, 2^31).
fn to_i32(value: f64, name: impl Display) -> Result<i32, JsError> {
    checked(value, -2_147_483_648.0, 2_147_483_648.0, name).map(|v| v as i32)
}

/// Narrow an optional JS number to `u32`, using `default` for `undefined`/`null`.
fn opt_u32(value: Option<f64>, default: u32, name: &str) -> Result<u32, JsError> {
    value.map_or(Ok(default), |v| to_u32(v, name))
}

/// Narrow every element of a JS number array to `u32`, naming a bad one by index.
fn to_u32_vec(values: &[f64], name: &str) -> Result<Vec<u32>, JsError> {
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| to_u32(v, format_args!("{name}[{i}]")))
        .collect()
}

/// Narrow every element of a JS number array to `i32`, naming a bad one by index.
fn to_i32_vec(values: &[f64], name: &str) -> Result<Vec<i32>, JsError> {
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| to_i32(v, format_args!("{name}[{i}]")))
        .collect()
}

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

/// Maximum byte length for the meta field payload after decoding.
#[wasm_bindgen(js_name = "META_TRIM_META")]
pub fn meta_trim_meta() -> u32 {
    iscc_lib::META_TRIM_META as u32
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
    bits: Option<f64>,
) -> Result<String, JsError> {
    let bits = opt_u32(bits, 64, "bits")?;
    iscc_lib::gen_meta_code_v0(name, description.as_deref(), meta.as_deref(), bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate a Text-Code from plain text content.
///
/// Produces an ISCC Content-Code for text using MinHash-based
/// similarity hashing.
#[wasm_bindgen]
pub fn gen_text_code_v0(text: &str, bits: Option<f64>) -> Result<String, JsError> {
    let bits = opt_u32(bits, 64, "bits")?;
    iscc_lib::gen_text_code_v0(text, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate an Image-Code from pixel data.
///
/// Produces an ISCC Content-Code for images from 1024 grayscale pixels
/// (32x32) using a DCT-based perceptual hash.
#[wasm_bindgen]
pub fn gen_image_code_v0(pixels: &[u8], bits: Option<f64>) -> Result<String, JsError> {
    let bits = opt_u32(bits, 64, "bits")?;
    iscc_lib::gen_image_code_v0(pixels, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate an Audio-Code from a Chromaprint feature vector.
///
/// Produces an ISCC Content-Code for audio from signed integer
/// Chromaprint fingerprint features using multi-stage SimHash.
#[wasm_bindgen]
pub fn gen_audio_code_v0(
    #[wasm_bindgen(unchecked_param_type = "ArrayLike<number>")] cv: Vec<f64>,
    bits: Option<f64>,
) -> Result<String, JsError> {
    let cv = to_i32_vec(&cv, "cv")?;
    let bits = opt_u32(bits, 64, "bits")?;
    iscc_lib::gen_audio_code_v0(&cv, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate a Video-Code from frame signature data.
///
/// Produces an ISCC Content-Code for video from MPEG-7 frame
/// signature vectors using WTA-Hash. Accepts a JS array of arrays of i32.
#[wasm_bindgen]
pub fn gen_video_code_v0(frame_sigs: JsValue, bits: Option<f64>) -> Result<String, JsError> {
    let bits = opt_u32(bits, 64, "bits")?;
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
pub fn gen_mixed_code_v0(codes: JsValue, bits: Option<f64>) -> Result<String, JsError> {
    let bits = opt_u32(bits, 64, "bits")?;
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
pub fn gen_data_code_v0(data: &[u8], bits: Option<f64>) -> Result<String, JsError> {
    let bits = opt_u32(bits, 64, "bits")?;
    iscc_lib::gen_data_code_v0(data, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate an Instance-Code from raw byte data.
///
/// Produces an ISCC Instance-Code by hashing the complete byte stream
/// with BLAKE3. Returns the ISCC string with "ISCC:" prefix.
#[wasm_bindgen]
pub fn gen_instance_code_v0(data: &[u8], bits: Option<f64>) -> Result<String, JsError> {
    let bits = opt_u32(bits, 64, "bits")?;
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

/// Result of [`gen_sum_code_v0`], containing the composite ISCC-CODE, data hash, and file size.
#[wasm_bindgen]
pub struct WasmSumCodeResult {
    /// Composite ISCC-CODE string (e.g., `"ISCC:KAC..."`).
    #[wasm_bindgen(getter_with_clone)]
    pub iscc: String,
    /// Hex-encoded BLAKE3 multihash (`"1e20..."`) of the data.
    #[wasm_bindgen(getter_with_clone)]
    pub datahash: String,
    /// Byte length of the input data (as `f64` for JS `number` compatibility).
    pub filesize: f64,
    /// Data-Code and Instance-Code ISCC strings (when `add_units` is true).
    #[wasm_bindgen(getter_with_clone)]
    pub units: Option<Vec<String>>,
}

/// Generate a composite ISCC-CODE from raw byte data in a single pass.
///
/// Feeds both `DataHasher` (CDC/MinHash) and `InstanceHasher` (BLAKE3) from
/// the same byte slice, then composes the final ISCC-CODE via
/// `gen_iscc_code_v0`. WASM-compatible alternative to the file-based core API.
///
/// When `add_units` is `true`, the result includes the individual Data-Code
/// and Instance-Code ISCC strings.
#[wasm_bindgen]
pub fn gen_sum_code_v0(
    data: &[u8],
    bits: Option<f64>,
    wide: Option<bool>,
    add_units: Option<bool>,
) -> Result<WasmSumCodeResult, JsError> {
    let bits = opt_u32(bits, 64, "bits")?;
    let wide = wide.unwrap_or(false);
    let add_units = add_units.unwrap_or(false);

    let mut data_hasher = iscc_lib::DataHasher::new();
    let mut instance_hasher = iscc_lib::InstanceHasher::new();

    data_hasher.update(data);
    instance_hasher.update(data);

    let data_result = data_hasher
        .finalize(bits)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let instance_result = instance_hasher
        .finalize(bits)
        .map_err(|e| JsError::new(&e.to_string()))?;

    // Borrow strings for gen_iscc_code_v0 before potentially moving them into units.
    let iscc_result = iscc_lib::gen_iscc_code_v0(&[&data_result.iscc, &instance_result.iscc], wide)
        .map_err(|e| JsError::new(&e.to_string()))?;

    let units = if add_units {
        Some(vec![data_result.iscc, instance_result.iscc])
    } else {
        None
    };

    Ok(WasmSumCodeResult {
        iscc: iscc_result.iscc,
        datahash: instance_result.datahash,
        filesize: instance_result.filesize as f64,
        units,
    })
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
pub fn text_trim(text: &str, nbytes: f64) -> Result<String, JsError> {
    let nbytes = to_u32(nbytes, "nbytes")?;
    Ok(iscc_lib::text_trim(text, nbytes as usize))
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
    mtype: f64,
    stype: f64,
    version: f64,
    bit_length: f64,
    digest: &[u8],
) -> Result<String, JsError> {
    iscc_lib::encode_component(
        to_u8(mtype, "mtype")?,
        to_u8(stype, "stype")?,
        to_u8(version, "version")?,
        to_u32(bit_length, "bit_length")?,
        digest,
    )
    .map_err(|e| JsError::new(&e.to_string()))
}

/// Result of decoding an ISCC unit string.
#[wasm_bindgen]
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
    #[wasm_bindgen(getter_with_clone)]
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

/// Encode ISCC-UNIT strings as an ISCC-SEQ byte string (IEP-0020).
///
/// Accepts an array of unit strings and returns a `Uint8Array` with the
/// concatenated header and body bytes of each unit, in order. Base32 decoding
/// is strict: `=` padding, non-zero spare bits and non-ASCII case-fold aliases
/// (`ſ`, `ß`) are rejected, although iscc-core accepts them. Throws on a bare
/// string, an empty array, an empty or malformed unit, or a unit of an
/// unsupported MainType (ISCC-CODE, ID, FLAKE).
#[wasm_bindgen]
pub fn encode_seq(units: JsValue) -> Result<Vec<u8>, JsError> {
    let units: Vec<String> =
        serde_wasm_bindgen::from_value(units).map_err(|e| JsError::new(&e.to_string()))?;
    let refs: Vec<&str> = units.iter().map(String::as_str).collect();
    iscc_lib::encode_seq(&refs).map_err(|e| JsError::new(&e.to_string()))
}

/// Decode an ISCC-SEQ byte string to canonical `ISCC:`-prefixed unit strings.
///
/// Decoding is strict: throws on empty input, malformed units or trailing bytes.
#[wasm_bindgen]
pub fn decode_seq(data: &[u8]) -> Result<Vec<String>, JsError> {
    iscc_lib::decode_seq(data).map_err(|e| JsError::new(&e.to_string()))
}

/// Generate an ISCC-IDv1 from a timestamp and a HUB-ID (experimental).
///
/// Packs a 52-bit microsecond UTC `timestamp` into the high bits and a 12-bit
/// `hub_id` (0-4095) into the low bits, encoding the result as an ISCC-ID unit
/// with `realm` (0 = testnet, 1 = mainnet) as SubType and Version `V1`.
///
/// All three parameters are JS `number`s (valid `timestamp` values are `< 2^52`,
/// exactly representable in `f64`). Throws if any parameter is non-finite,
/// non-integral, or negative, or if `timestamp >= 2^52`, `hub_id >= 2^12`, or
/// `realm` is not `0` or `1`.
///
/// **Experimental:** the ISCC-IDv1 format is not yet part of ISO 24138 and may
/// change. There is no dedicated decoder — use `iscc_decode` to decode.
#[wasm_bindgen]
pub fn gen_iscc_id_v1(timestamp: f64, hub_id: f64, realm: f64) -> Result<String, JsError> {
    let timestamp = checked(timestamp, 0.0, 4_503_599_627_370_496.0, "timestamp")?; // 2^52
    let hub_id = checked(hub_id, 0.0, 4096.0, "hub_id")?;
    let realm = checked(realm, 0.0, 2.0, "realm")?;
    iscc_lib::gen_iscc_id_v1(timestamp as u64, hub_id as u16, realm as u8)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
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
pub fn sliding_window(seq: &str, width: f64) -> Result<Vec<String>, JsError> {
    let width = to_u32(width, "width")?;
    iscc_lib::sliding_window(seq, width as usize).map_err(|e| JsError::new(&e.to_string()))
}

// ── Algorithm primitives ─────────────────────────────────────────────────────

/// Compute a SimHash from a sequence of equal-length hash digests.
///
/// Accepts a JS array of `Uint8Array` values. Returns a similarity-preserving
/// hash whose length matches the input digest length. Throws on an empty list
/// or mismatched digest lengths.
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
pub fn alg_minhash_256(
    #[wasm_bindgen(unchecked_param_type = "ArrayLike<number>")] features: Vec<f64>,
) -> Result<Vec<u8>, JsError> {
    let features = to_u32_vec(&features, "features")?;
    Ok(iscc_lib::alg_minhash_256(&features))
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
    avg_chunk_size: Option<f64>,
) -> Result<Vec<js_sys::Uint8Array>, JsError> {
    let avg = opt_u32(avg_chunk_size, 1024, "avg_chunk_size")?;
    let chunks = iscc_lib::alg_cdc_chunks(data, utf32, avg)
        .map_err(|e| JsError::new(&e.to_string()))?
        .into_iter()
        .map(js_sys::Uint8Array::from)
        .collect();
    Ok(chunks)
}

/// Compute a similarity-preserving hash from video frame signatures.
///
/// Accepts a JS array of arrays of `i32`. Returns raw bytes of length
/// `bits / 8`. Default `bits` is 64. Throws if `frame_sigs` is empty.
#[wasm_bindgen]
pub fn soft_hash_video_v0(frame_sigs: JsValue, bits: Option<f64>) -> Result<Vec<u8>, JsError> {
    let bits = opt_u32(bits, 64, "bits")?;
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
    pub fn finalize(&mut self, bits: Option<f64>) -> Result<String, JsError> {
        let bits = opt_u32(bits, 64, "bits")?;
        let hasher = self
            .inner
            .take()
            .ok_or_else(|| JsError::new("DataHasher already finalized"))?;
        hasher
            .finalize(bits)
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
    pub fn finalize(&mut self, bits: Option<f64>) -> Result<String, JsError> {
        let bits = opt_u32(bits, 64, "bits")?;
        let hasher = self
            .inner
            .take()
            .ok_or_else(|| JsError::new("InstanceHasher already finalized"))?;
        hasher
            .finalize(bits)
            .map(|r| r.iscc)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// Streaming composite ISCC-CODE (Sum) generator.
///
/// Runs the Data-Code (CDC/MinHash) and Instance-Code (BLAKE3) algorithms in a
/// single pass over the input, then composes the final ISCC-CODE. Produces
/// results identical to `gen_sum_code_v0` for the same byte stream, but lets
/// callers feed data in chunks. Follows the
/// `new() → update() → finalize()` pattern.
#[wasm_bindgen]
pub struct SumHasher {
    inner: Option<iscc_lib::streaming::SumHasher>,
}

impl Default for SumHasher {
    /// Create a new `SumHasher` (delegates to `new()`).
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl SumHasher {
    /// Create a new `SumHasher`.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Some(iscc_lib::streaming::SumHasher::new()),
        }
    }

    /// Push data into the hasher.
    pub fn update(&mut self, data: &[u8]) -> Result<(), JsError> {
        self.inner
            .as_mut()
            .ok_or_else(|| JsError::new("SumHasher already finalized"))
            .map(|h| h.update(data))
    }

    /// Consume the hasher and produce a composite ISCC-CODE result.
    ///
    /// After calling `finalize`, subsequent calls to `update` or `finalize`
    /// will throw. Defaults: `bits` 64, `wide` false, `add_units` false. When
    /// `add_units` is `true`, the result includes the individual Data-Code and
    /// Instance-Code ISCC strings.
    pub fn finalize(
        &mut self,
        bits: Option<f64>,
        wide: Option<bool>,
        add_units: Option<bool>,
    ) -> Result<WasmSumCodeResult, JsError> {
        let bits = opt_u32(bits, 64, "bits")?;
        let hasher = self
            .inner
            .take()
            .ok_or_else(|| JsError::new("SumHasher already finalized"))?;
        let result = hasher
            .finalize(bits, wide.unwrap_or(false), add_units.unwrap_or(false))
            .map_err(|e| JsError::new(&e.to_string()))?;
        Ok(WasmSumCodeResult {
            iscc: result.iscc,
            datahash: result.datahash,
            filesize: result.filesize as f64,
            units: result.units,
        })
    }
}
