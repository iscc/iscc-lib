//! UniFFI scaffolding for iscc-lib — shared interface for Swift and Kotlin bindings.
//!
//! Exposes all 32 Tier 1 symbols from `iscc-lib` via UniFFI proc macros. Uses owned
//! types (`String`, `Vec<u8>`) because UniFFI requires values, not borrowed references.
//! Constants are exposed as getter functions (UniFFI doesn't support `const` exports).
//!
//! Streaming types (`DataHasher`, `InstanceHasher`) use `Mutex<Option<Inner>>` for
//! thread-safe one-shot finalization as UniFFI Objects.

uniffi::setup_scaffolding!();

use std::sync::Mutex;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// UniFFI-compatible error type wrapping `iscc_lib::IsccError`.
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum IsccUniError {
    /// ISCC operation error with a descriptive message.
    #[error("{msg}")]
    IsccError {
        /// Error description.
        msg: String,
    },
}

impl From<iscc_lib::IsccError> for IsccUniError {
    /// Convert an `iscc_lib::IsccError` into `IsccUniError`.
    fn from(e: iscc_lib::IsccError) -> Self {
        IsccUniError::IsccError { msg: e.to_string() }
    }
}

// ---------------------------------------------------------------------------
// Result records
// ---------------------------------------------------------------------------

/// Result of `gen_meta_code_v0`.
#[derive(Debug, uniffi::Record)]
pub struct MetaCodeResult {
    /// ISCC code string (e.g., `"ISCC:AAAZXZ6OU74YAZIM"`).
    pub iscc: String,
    /// Normalized name after cleaning, newline removal, and trimming.
    pub name: String,
    /// Normalized description (present only when description was non-empty).
    pub description: Option<String>,
    /// Metadata as a Data-URL string (present only when meta was provided).
    pub meta: Option<String>,
    /// Hex-encoded BLAKE3 multihash of the metadata payload.
    pub metahash: String,
}

/// Result of `gen_text_code_v0`.
#[derive(Debug, uniffi::Record)]
pub struct TextCodeResult {
    /// ISCC code string.
    pub iscc: String,
    /// Character count after `text_collapse` (u64 because UniFFI doesn't support usize).
    pub characters: u64,
}

/// Result of `gen_image_code_v0`.
#[derive(Debug, uniffi::Record)]
pub struct ImageCodeResult {
    /// ISCC code string.
    pub iscc: String,
}

/// Result of `gen_audio_code_v0`.
#[derive(Debug, uniffi::Record)]
pub struct AudioCodeResult {
    /// ISCC code string.
    pub iscc: String,
}

/// Result of `gen_video_code_v0`.
#[derive(Debug, uniffi::Record)]
pub struct VideoCodeResult {
    /// ISCC code string.
    pub iscc: String,
}

/// Result of `gen_mixed_code_v0`.
#[derive(Debug, uniffi::Record)]
pub struct MixedCodeResult {
    /// ISCC code string.
    pub iscc: String,
    /// Input Content-Code strings (passed through unchanged).
    pub parts: Vec<String>,
}

/// Result of `gen_data_code_v0`.
#[derive(Debug, uniffi::Record)]
pub struct DataCodeResult {
    /// ISCC code string.
    pub iscc: String,
}

/// Result of `gen_instance_code_v0`.
#[derive(Debug, uniffi::Record)]
pub struct InstanceCodeResult {
    /// ISCC code string.
    pub iscc: String,
    /// Hex-encoded BLAKE3 multihash of the input data.
    pub datahash: String,
    /// Byte length of the input data.
    pub filesize: u64,
}

/// Result of `gen_iscc_code_v0`.
#[derive(Debug, uniffi::Record)]
pub struct IsccCodeResult {
    /// ISCC code string.
    pub iscc: String,
}

/// Result of `gen_sum_code_v0`.
#[derive(Debug, uniffi::Record)]
pub struct SumCodeResult {
    /// Composite ISCC-CODE string.
    pub iscc: String,
    /// Hex-encoded BLAKE3 multihash of the file.
    pub datahash: String,
    /// Byte length of the file.
    pub filesize: u64,
    /// Individual ISCC strings at the requested precision (when `add_units` is true).
    pub units: Option<Vec<String>>,
}

/// Result of `iscc_decode`.
#[derive(Debug, uniffi::Record)]
pub struct DecodeResult {
    /// Main type identifier.
    pub maintype: u8,
    /// Sub type identifier.
    pub subtype: u8,
    /// Version number.
    pub version: u8,
    /// Bit length indicator.
    pub length: u8,
    /// Raw digest bytes.
    pub digest: Vec<u8>,
}

// ---------------------------------------------------------------------------
// Constants (as getter functions — UniFFI doesn't support const exports)
// ---------------------------------------------------------------------------

/// Max UTF-8 byte length for name metadata trimming.
#[uniffi::export]
pub fn meta_trim_name() -> u32 {
    iscc_lib::META_TRIM_NAME as u32
}

/// Max UTF-8 byte length for description metadata trimming.
#[uniffi::export]
pub fn meta_trim_description() -> u32 {
    iscc_lib::META_TRIM_DESCRIPTION as u32
}

/// Max decoded payload size in bytes for the meta element.
#[uniffi::export]
pub fn meta_trim_meta() -> u32 {
    iscc_lib::META_TRIM_META as u32
}

/// Buffer size in bytes for streaming file reads (4 MB).
#[uniffi::export]
pub fn io_read_size() -> u32 {
    iscc_lib::IO_READ_SIZE as u32
}

/// Character n-gram width for text content features.
#[uniffi::export]
pub fn text_ngram_size() -> u32 {
    iscc_lib::TEXT_NGRAM_SIZE as u32
}

// ---------------------------------------------------------------------------
// Gen functions
// ---------------------------------------------------------------------------

/// Generate a Meta-Code from name, optional description, and optional metadata.
#[uniffi::export]
pub fn gen_meta_code_v0(
    name: String,
    description: Option<String>,
    meta: Option<String>,
    bits: u32,
) -> Result<MetaCodeResult, IsccUniError> {
    let result = iscc_lib::gen_meta_code_v0(&name, description.as_deref(), meta.as_deref(), bits)?;
    Ok(MetaCodeResult {
        iscc: result.iscc,
        name: result.name,
        description: result.description,
        meta: result.meta,
        metahash: result.metahash,
    })
}

/// Generate a Text-Code from text content.
#[uniffi::export]
pub fn gen_text_code_v0(text: String, bits: u32) -> Result<TextCodeResult, IsccUniError> {
    let result = iscc_lib::gen_text_code_v0(&text, bits)?;
    Ok(TextCodeResult {
        iscc: result.iscc,
        characters: result.characters as u64,
    })
}

/// Generate an Image-Code from raw pixel data.
#[uniffi::export]
pub fn gen_image_code_v0(pixels: Vec<u8>, bits: u32) -> Result<ImageCodeResult, IsccUniError> {
    let result = iscc_lib::gen_image_code_v0(&pixels, bits)?;
    Ok(ImageCodeResult { iscc: result.iscc })
}

/// Generate an Audio-Code from a chromaprint fingerprint.
#[uniffi::export]
pub fn gen_audio_code_v0(cv: Vec<i32>, bits: u32) -> Result<AudioCodeResult, IsccUniError> {
    let result = iscc_lib::gen_audio_code_v0(&cv, bits)?;
    Ok(AudioCodeResult { iscc: result.iscc })
}

/// Generate a Video-Code from per-frame signatures.
#[uniffi::export]
pub fn gen_video_code_v0(
    frame_sigs: Vec<Vec<i32>>,
    bits: u32,
) -> Result<VideoCodeResult, IsccUniError> {
    let result = iscc_lib::gen_video_code_v0(&frame_sigs, bits)?;
    Ok(VideoCodeResult { iscc: result.iscc })
}

/// Generate a Mixed-Code from multiple content code strings.
#[uniffi::export]
pub fn gen_mixed_code_v0(codes: Vec<String>, bits: u32) -> Result<MixedCodeResult, IsccUniError> {
    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
    let result = iscc_lib::gen_mixed_code_v0(&refs, bits)?;
    Ok(MixedCodeResult {
        iscc: result.iscc,
        parts: result.parts,
    })
}

/// Generate a Data-Code from raw byte data.
#[uniffi::export]
pub fn gen_data_code_v0(data: Vec<u8>, bits: u32) -> Result<DataCodeResult, IsccUniError> {
    let result = iscc_lib::gen_data_code_v0(&data, bits)?;
    Ok(DataCodeResult { iscc: result.iscc })
}

/// Generate an Instance-Code from raw byte data.
#[uniffi::export]
pub fn gen_instance_code_v0(data: Vec<u8>, bits: u32) -> Result<InstanceCodeResult, IsccUniError> {
    let result = iscc_lib::gen_instance_code_v0(&data, bits)?;
    Ok(InstanceCodeResult {
        iscc: result.iscc,
        datahash: result.datahash,
        filesize: result.filesize,
    })
}

/// Generate a composite ISCC-CODE from multiple code strings.
#[uniffi::export]
pub fn gen_iscc_code_v0(codes: Vec<String>, wide: bool) -> Result<IsccCodeResult, IsccUniError> {
    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
    let result = iscc_lib::gen_iscc_code_v0(&refs, wide)?;
    Ok(IsccCodeResult { iscc: result.iscc })
}

/// Generate a complete ISCC-SUM from a file path.
#[uniffi::export]
pub fn gen_sum_code_v0(
    path: String,
    bits: u32,
    wide: bool,
    add_units: bool,
) -> Result<SumCodeResult, IsccUniError> {
    let result = iscc_lib::gen_sum_code_v0(std::path::Path::new(&path), bits, wide, add_units)?;
    Ok(SumCodeResult {
        iscc: result.iscc,
        datahash: result.datahash,
        filesize: result.filesize,
        units: result.units,
    })
}

// ---------------------------------------------------------------------------
// Text utility functions
// ---------------------------------------------------------------------------

/// Clean text by applying NFKC normalization and control-character removal.
#[uniffi::export]
pub fn text_clean(text: String) -> String {
    iscc_lib::text_clean(&text)
}

/// Remove newlines and collapse whitespace runs to single spaces.
#[uniffi::export]
pub fn text_remove_newlines(text: String) -> String {
    iscc_lib::text_remove_newlines(&text)
}

/// Trim text to a maximum number of UTF-8 bytes on a word boundary.
#[uniffi::export]
pub fn text_trim(text: String, nbytes: u64) -> String {
    iscc_lib::text_trim(&text, nbytes as usize)
}

/// Collapse text for similarity processing (NFD, lowercase, filter).
#[uniffi::export]
pub fn text_collapse(text: String) -> String {
    iscc_lib::text_collapse(&text)
}

// ---------------------------------------------------------------------------
// Algorithm primitives
// ---------------------------------------------------------------------------

/// Generate character n-grams from text using a sliding window.
#[uniffi::export]
pub fn sliding_window(seq: String, width: u32) -> Result<Vec<String>, IsccUniError> {
    Ok(iscc_lib::sliding_window(&seq, width as usize)?)
}

/// Compute a 256-bit MinHash from a set of u32 features.
#[uniffi::export]
pub fn alg_minhash_256(features: Vec<u32>) -> Vec<u8> {
    iscc_lib::alg_minhash_256(&features)
}

/// Split data into content-defined chunks.
#[uniffi::export]
pub fn alg_cdc_chunks(
    data: Vec<u8>,
    utf32: bool,
    avg_chunk_size: u32,
) -> Result<Vec<Vec<u8>>, IsccUniError> {
    let chunks = iscc_lib::alg_cdc_chunks(&data, utf32, avg_chunk_size)?;
    Ok(chunks.into_iter().map(|c| c.to_vec()).collect())
}

/// Compute a SimHash from a collection of hash digests.
#[uniffi::export]
pub fn alg_simhash(hash_digests: Vec<Vec<u8>>) -> Result<Vec<u8>, IsccUniError> {
    Ok(iscc_lib::alg_simhash(&hash_digests)?)
}

/// Compute a similarity-preserving video hash from per-frame signatures.
#[uniffi::export]
pub fn soft_hash_video_v0(frame_sigs: Vec<Vec<i32>>, bits: u32) -> Result<Vec<u8>, IsccUniError> {
    Ok(iscc_lib::soft_hash_video_v0(&frame_sigs, bits)?)
}

// ---------------------------------------------------------------------------
// Encoding / codec utilities
// ---------------------------------------------------------------------------

/// Encode binary data as URL-safe base64 (no padding).
#[uniffi::export]
pub fn encode_base64(data: Vec<u8>) -> String {
    iscc_lib::encode_base64(&data)
}

/// Convert a JSON string to a Data-URL with JCS canonicalization.
#[uniffi::export]
pub fn json_to_data_url(json: String) -> Result<String, IsccUniError> {
    Ok(iscc_lib::json_to_data_url(&json)?)
}

/// Decompose an ISCC-CODE into individual unit strings.
#[uniffi::export]
pub fn iscc_decompose(iscc_code: String) -> Result<Vec<String>, IsccUniError> {
    Ok(iscc_lib::iscc_decompose(&iscc_code)?)
}

/// Encode header and digest into an ISCC component string.
#[uniffi::export]
pub fn encode_component(
    mtype: u8,
    stype: u8,
    version: u8,
    bit_length: u32,
    digest: Vec<u8>,
) -> Result<String, IsccUniError> {
    Ok(iscc_lib::encode_component(
        mtype, stype, version, bit_length, &digest,
    )?)
}

/// Decode an ISCC unit string into header components and raw digest.
#[uniffi::export]
pub fn iscc_decode(iscc: String) -> Result<DecodeResult, IsccUniError> {
    let (maintype, subtype, version, length, digest) = iscc_lib::iscc_decode(&iscc)?;
    Ok(DecodeResult {
        maintype,
        subtype,
        version,
        length,
        digest,
    })
}

// ---------------------------------------------------------------------------
// Diagnostics
// ---------------------------------------------------------------------------

/// Run the conformance self-test against vendored test vectors.
#[uniffi::export]
pub fn conformance_selftest() -> bool {
    iscc_lib::conformance_selftest()
}

// ---------------------------------------------------------------------------
// Streaming types
// ---------------------------------------------------------------------------

/// Streaming Data-Code generator.
///
/// Incrementally processes data with content-defined chunking and MinHash.
/// Call `update()` with data chunks, then `finalize()` to produce the code.
#[derive(uniffi::Object)]
pub struct DataHasher {
    /// Inner hasher behind a mutex for thread-safe one-shot finalization.
    inner: Mutex<Option<iscc_lib::DataHasher>>,
}

impl Default for DataHasher {
    /// Create a new `DataHasher` (delegates to `new()`).
    fn default() -> Self {
        Self::new()
    }
}

#[uniffi::export]
impl DataHasher {
    /// Create a new `DataHasher`.
    #[uniffi::constructor]
    pub fn new() -> Self {
        DataHasher {
            inner: Mutex::new(Some(iscc_lib::DataHasher::new())),
        }
    }

    /// Push data into the hasher.
    pub fn update(&self, data: Vec<u8>) -> Result<(), IsccUniError> {
        let mut guard = self.inner.lock().unwrap();
        match guard.as_mut() {
            Some(hasher) => {
                hasher.update(&data);
                Ok(())
            }
            None => Err(IsccUniError::IsccError {
                msg: "DataHasher already finalized".into(),
            }),
        }
    }

    /// Consume the inner hasher and produce a Data-Code result.
    pub fn finalize(&self, bits: u32) -> Result<DataCodeResult, IsccUniError> {
        let mut guard = self.inner.lock().unwrap();
        let hasher = guard.take().ok_or(IsccUniError::IsccError {
            msg: "DataHasher already finalized".into(),
        })?;
        let result = hasher.finalize(bits)?;
        Ok(DataCodeResult { iscc: result.iscc })
    }
}

/// Streaming Instance-Code generator.
///
/// Incrementally hashes data with BLAKE3 to produce an Instance-Code.
/// Call `update()` with data chunks, then `finalize()` to produce the code.
#[derive(uniffi::Object)]
pub struct InstanceHasher {
    /// Inner hasher behind a mutex for thread-safe one-shot finalization.
    inner: Mutex<Option<iscc_lib::InstanceHasher>>,
}

impl Default for InstanceHasher {
    /// Create a new `InstanceHasher` (delegates to `new()`).
    fn default() -> Self {
        Self::new()
    }
}

#[uniffi::export]
impl InstanceHasher {
    /// Create a new `InstanceHasher`.
    #[uniffi::constructor]
    pub fn new() -> Self {
        InstanceHasher {
            inner: Mutex::new(Some(iscc_lib::InstanceHasher::new())),
        }
    }

    /// Push data into the hasher.
    pub fn update(&self, data: Vec<u8>) -> Result<(), IsccUniError> {
        let mut guard = self.inner.lock().unwrap();
        match guard.as_mut() {
            Some(hasher) => {
                hasher.update(&data);
                Ok(())
            }
            None => Err(IsccUniError::IsccError {
                msg: "InstanceHasher already finalized".into(),
            }),
        }
    }

    /// Consume the inner hasher and produce an Instance-Code result.
    pub fn finalize(&self, bits: u32) -> Result<InstanceCodeResult, IsccUniError> {
        let mut guard = self.inner.lock().unwrap();
        let hasher = guard.take().ok_or(IsccUniError::IsccError {
            msg: "InstanceHasher already finalized".into(),
        })?;
        let result = hasher.finalize(bits)?;
        Ok(InstanceCodeResult {
            iscc: result.iscc,
            datahash: result.datahash,
            filesize: result.filesize,
        })
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(meta_trim_name(), 128);
        assert_eq!(meta_trim_description(), 4096);
        assert_eq!(meta_trim_meta(), 128_000);
        assert_eq!(io_read_size(), 4_194_304);
        assert_eq!(text_ngram_size(), 13);
    }

    #[test]
    fn test_gen_meta_code_v0() {
        let result = gen_meta_code_v0("Test Title".into(), None, None, 64).unwrap();
        assert!(result.iscc.starts_with("ISCC:"));
        assert_eq!(result.name, "Test Title");
    }

    #[test]
    fn test_gen_text_code_v0() {
        let result = gen_text_code_v0("Hello World".into(), 64).unwrap();
        assert!(result.iscc.starts_with("ISCC:"));
        assert!(result.characters > 0);
    }

    #[test]
    fn test_gen_data_code_v0() {
        let result = gen_data_code_v0(b"hello world".to_vec(), 64).unwrap();
        assert!(result.iscc.starts_with("ISCC:"));
    }

    #[test]
    fn test_gen_instance_code_v0() {
        let result = gen_instance_code_v0(b"hello world".to_vec(), 64).unwrap();
        assert!(result.iscc.starts_with("ISCC:"));
        assert!(!result.datahash.is_empty());
        assert_eq!(result.filesize, 11);
    }

    #[test]
    fn test_gen_iscc_code_v0() {
        let data = gen_data_code_v0(b"hello world".to_vec(), 64).unwrap();
        let inst = gen_instance_code_v0(b"hello world".to_vec(), 64).unwrap();
        let result = gen_iscc_code_v0(vec![data.iscc, inst.iscc], false).unwrap();
        assert!(result.iscc.starts_with("ISCC:"));
    }

    #[test]
    fn test_text_utilities() {
        let cleaned = text_clean("Hello\x00World".into());
        assert_eq!(cleaned, "HelloWorld");

        let no_nl = text_remove_newlines("Hello\nWorld".into());
        assert_eq!(no_nl, "Hello World");

        let trimmed = text_trim("Hello World".into(), 5);
        assert_eq!(trimmed, "Hello");

        let collapsed = text_collapse("Hello World".into());
        assert!(!collapsed.is_empty());
    }

    #[test]
    fn test_sliding_window() {
        let result = sliding_window("abcdef".into(), 3).unwrap();
        assert_eq!(result, vec!["abc", "bcd", "cde", "def"]);
    }

    #[test]
    fn test_alg_minhash_256() {
        let result = alg_minhash_256(vec![1, 2, 3]);
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_encode_base64() {
        let result = encode_base64(b"hello".to_vec());
        assert_eq!(result, "aGVsbG8");
    }

    #[test]
    fn test_iscc_decode() {
        let data = gen_data_code_v0(b"hello world".to_vec(), 64).unwrap();
        let decoded = iscc_decode(data.iscc).unwrap();
        assert_eq!(decoded.maintype, 3); // MainType::Data
    }

    #[test]
    fn test_iscc_decompose() {
        let data = gen_data_code_v0(b"hello world".to_vec(), 64).unwrap();
        let inst = gen_instance_code_v0(b"hello world".to_vec(), 64).unwrap();
        let iscc = gen_iscc_code_v0(vec![data.iscc, inst.iscc], false).unwrap();
        let parts = iscc_decompose(iscc.iscc).unwrap();
        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn test_encode_component() {
        let digest = vec![0u8; 32];
        let result = encode_component(3, 0, 0, 64, digest).unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_conformance_selftest() {
        assert!(conformance_selftest());
    }

    #[test]
    fn test_data_hasher_streaming() {
        let hasher = DataHasher::new();
        hasher.update(b"hello ".to_vec()).unwrap();
        hasher.update(b"world".to_vec()).unwrap();
        let streaming = hasher.finalize(64).unwrap();

        let oneshot = gen_data_code_v0(b"hello world".to_vec(), 64).unwrap();
        assert_eq!(streaming.iscc, oneshot.iscc);
    }

    #[test]
    fn test_data_hasher_double_finalize() {
        let hasher = DataHasher::new();
        hasher.update(b"hello".to_vec()).unwrap();
        let _ = hasher.finalize(64).unwrap();
        let err = hasher.finalize(64).unwrap_err();
        assert!(err.to_string().contains("already finalized"));
    }

    #[test]
    fn test_instance_hasher_streaming() {
        let hasher = InstanceHasher::new();
        hasher.update(b"hello ".to_vec()).unwrap();
        hasher.update(b"world".to_vec()).unwrap();
        let streaming = hasher.finalize(64).unwrap();

        let oneshot = gen_instance_code_v0(b"hello world".to_vec(), 64).unwrap();
        assert_eq!(streaming.iscc, oneshot.iscc);
        assert_eq!(streaming.datahash, oneshot.datahash);
        assert_eq!(streaming.filesize, oneshot.filesize);
    }

    #[test]
    fn test_instance_hasher_double_finalize() {
        let hasher = InstanceHasher::new();
        hasher.update(b"hello".to_vec()).unwrap();
        let _ = hasher.finalize(64).unwrap();
        let err = hasher.finalize(64).unwrap_err();
        assert!(err.to_string().contains("already finalized"));
    }

    #[test]
    fn test_json_to_data_url() {
        let result = json_to_data_url(r#"{"key":"value"}"#.into()).unwrap();
        assert!(result.starts_with("data:"));
    }

    #[test]
    fn test_alg_simhash() {
        let digests = vec![vec![0u8; 4], vec![255u8; 4]];
        let result = alg_simhash(digests).unwrap();
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_alg_cdc_chunks() {
        let data = vec![0u8; 1000];
        let chunks = alg_cdc_chunks(data, false, 256).unwrap();
        assert!(!chunks.is_empty());
    }
}
