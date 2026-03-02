//! Structured result types for ISCC code generation functions.
//!
//! Each `gen_*_v0` function returns a dedicated result struct carrying the ISCC
//! code string plus any additional fields (metahash, name, characters, etc.)
//! that match the `iscc-core` Python reference implementation's dict returns.

/// Result of [`gen_meta_code_v0`](crate::gen_meta_code_v0).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MetaCodeResult {
    /// ISCC code string (e.g., `"ISCC:AAAZXZ6OU74YAZIM"`).
    pub iscc: String,
    /// Normalized name after cleaning, newline removal, and trimming.
    pub name: String,
    /// Normalized description (present only when description was non-empty).
    pub description: Option<String>,
    /// Metadata as a Data-URL string (present only when meta was provided).
    pub meta: Option<String>,
    /// Hex-encoded BLAKE3 multihash (`"1e20..."`) of the metadata payload.
    pub metahash: String,
}

/// Result of [`gen_text_code_v0`](crate::gen_text_code_v0).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct TextCodeResult {
    /// ISCC code string.
    pub iscc: String,
    /// Character count after `text_collapse`.
    pub characters: usize,
}

/// Result of [`gen_image_code_v0`](crate::gen_image_code_v0).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ImageCodeResult {
    /// ISCC code string.
    pub iscc: String,
}

/// Result of [`gen_audio_code_v0`](crate::gen_audio_code_v0).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct AudioCodeResult {
    /// ISCC code string.
    pub iscc: String,
}

/// Result of [`gen_video_code_v0`](crate::gen_video_code_v0).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct VideoCodeResult {
    /// ISCC code string.
    pub iscc: String,
}

/// Result of [`gen_mixed_code_v0`](crate::gen_mixed_code_v0).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MixedCodeResult {
    /// ISCC code string.
    pub iscc: String,
    /// Input Content-Code strings (passed through unchanged).
    pub parts: Vec<String>,
}

/// Result of [`gen_data_code_v0`](crate::gen_data_code_v0).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct DataCodeResult {
    /// ISCC code string.
    pub iscc: String,
}

/// Result of [`gen_instance_code_v0`](crate::gen_instance_code_v0).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct InstanceCodeResult {
    /// ISCC code string.
    pub iscc: String,
    /// Hex-encoded BLAKE3 multihash (`"1e20..."`) of the input data.
    pub datahash: String,
    /// Byte length of the input data.
    pub filesize: u64,
}

/// Result of [`gen_iscc_code_v0`](crate::gen_iscc_code_v0).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct IsccCodeResult {
    /// ISCC code string.
    pub iscc: String,
}

/// Result of [`gen_sum_code_v0`](crate::gen_sum_code_v0).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct SumCodeResult {
    /// Composite ISCC-CODE string (e.g., `"ISCC:KAC..."`).
    pub iscc: String,
    /// Hex-encoded BLAKE3 multihash (`"1e20..."`) of the file.
    pub datahash: String,
    /// Byte length of the file.
    pub filesize: u64,
}
