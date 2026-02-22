//! High-performance Rust implementation of ISO 24138:2024 (ISCC).
//!
//! This crate provides the core ISCC algorithm implementations. All 9 `gen_*_v0`
//! functions are the public Tier 1 API surface, designed to be compatible with
//! the `iscc-core` Python reference implementation.

pub mod codec;

/// Error type for ISCC operations.
#[derive(Debug, thiserror::Error)]
pub enum IsccError {
    /// Operation is not yet implemented.
    #[error("not implemented")]
    NotImplemented,
    /// Input data is invalid.
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

/// Result type alias for ISCC operations.
pub type IsccResult<T> = Result<T, IsccError>;

/// Generate a Meta-Code from name and optional metadata.
///
/// Produces an ISCC Meta-Code by hashing the provided name, description,
/// and metadata fields using the SimHash algorithm.
pub fn gen_meta_code_v0(
    _name: &str,
    _description: Option<&str>,
    _meta: Option<&str>,
    _bits: u32,
) -> IsccResult<String> {
    Err(IsccError::NotImplemented)
}

/// Generate a Text-Code from plain text content.
///
/// Produces an ISCC Content-Code for text by extracting and hashing
/// text features using the SimHash algorithm.
pub fn gen_text_code_v0(_text: &str, _bits: u32) -> IsccResult<String> {
    Err(IsccError::NotImplemented)
}

/// Generate an Image-Code from pixel data.
///
/// Produces an ISCC Content-Code for images from a sequence of pixel
/// values (grayscale, 0-255).
pub fn gen_image_code_v0(_pixels: &[u8], _bits: u32) -> IsccResult<String> {
    Err(IsccError::NotImplemented)
}

/// Generate an Audio-Code from a Chromaprint feature vector.
///
/// Produces an ISCC Content-Code for audio from a Chromaprint integer
/// fingerprint vector.
pub fn gen_audio_code_v0(_cv: &[u32], _bits: u32) -> IsccResult<String> {
    Err(IsccError::NotImplemented)
}

/// Generate a Video-Code from frame signature data.
///
/// Produces an ISCC Content-Code for video from a sequence of frame
/// signatures (each frame signature is a byte vector).
pub fn gen_video_code_v0(_frame_sigs: &[Vec<u8>], _bits: u32) -> IsccResult<String> {
    Err(IsccError::NotImplemented)
}

/// Generate a Mixed-Code from multiple Content-Code strings.
///
/// Produces a Mixed Content-Code by combining multiple ISCC Content-Codes
/// of different types (text, image, audio, video).
pub fn gen_mixed_code_v0(_codes: &[&str], _bits: u32) -> IsccResult<String> {
    Err(IsccError::NotImplemented)
}

/// Generate a Data-Code from raw byte data.
///
/// Produces an ISCC Data-Code by content-defined chunking and MinHash
/// fingerprinting of the input byte stream.
pub fn gen_data_code_v0(_data: &[u8], _bits: u32) -> IsccResult<String> {
    Err(IsccError::NotImplemented)
}

/// Generate an Instance-Code from raw byte data.
///
/// Produces an ISCC Instance-Code by hashing the complete byte stream
/// with BLAKE3. Captures the exact binary identity of the data.
pub fn gen_instance_code_v0(data: &[u8], bits: u32) -> IsccResult<String> {
    let digest = blake3::hash(data);
    let component = codec::encode_component(
        codec::MainType::Instance,
        codec::SubType::None,
        codec::Version::V0,
        bits,
        digest.as_bytes(),
    )?;
    Ok(format!("ISCC:{component}"))
}

/// Generate a composite ISCC-CODE from individual ISCC unit codes.
///
/// Combines multiple ISCC unit codes (Meta-Code, Content-Code, Data-Code,
/// Instance-Code) into a single composite ISCC-CODE. When `wide` is true,
/// produces a 256-bit code instead of the standard 128-bit.
pub fn gen_iscc_code_v0(_codes: &[&str], _wide: bool) -> IsccResult<String> {
    Err(IsccError::NotImplemented)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_meta_code_v0_stub() {
        assert!(matches!(
            gen_meta_code_v0("test", None, None, 64),
            Err(IsccError::NotImplemented)
        ));
    }

    #[test]
    fn test_gen_text_code_v0_stub() {
        assert!(matches!(
            gen_text_code_v0("hello world", 64),
            Err(IsccError::NotImplemented)
        ));
    }

    #[test]
    fn test_gen_image_code_v0_stub() {
        assert!(matches!(
            gen_image_code_v0(&[0u8; 100], 64),
            Err(IsccError::NotImplemented)
        ));
    }

    #[test]
    fn test_gen_audio_code_v0_stub() {
        assert!(matches!(
            gen_audio_code_v0(&[0u32; 10], 64),
            Err(IsccError::NotImplemented)
        ));
    }

    #[test]
    fn test_gen_video_code_v0_stub() {
        let frames: Vec<Vec<u8>> = vec![vec![0; 10]];
        assert!(matches!(
            gen_video_code_v0(&frames, 64),
            Err(IsccError::NotImplemented)
        ));
    }

    #[test]
    fn test_gen_mixed_code_v0_stub() {
        assert!(matches!(
            gen_mixed_code_v0(&["ISCC:AAA"], 64),
            Err(IsccError::NotImplemented)
        ));
    }

    #[test]
    fn test_gen_data_code_v0_stub() {
        assert!(matches!(
            gen_data_code_v0(&[1, 2, 3], 64),
            Err(IsccError::NotImplemented)
        ));
    }

    #[test]
    fn test_gen_instance_code_v0_empty() {
        let result = gen_instance_code_v0(b"", 64).unwrap();
        assert_eq!(result, "ISCC:IAA26E2JXH27TING");
    }

    #[test]
    fn test_gen_instance_code_v0_conformance() {
        let json_str = include_str!("../tests/data.json");
        let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let section = &data["gen_instance_code_v0"];
        let cases = section.as_object().unwrap();

        for (name, tc) in cases {
            let inputs = tc["inputs"].as_array().unwrap();
            let stream_str = inputs[0].as_str().unwrap();
            let bits = inputs[1].as_u64().unwrap() as u32;
            let expected_iscc = tc["outputs"]["iscc"].as_str().unwrap();

            // Parse "stream:" prefix — remainder is hex-encoded bytes
            let hex_data = stream_str
                .strip_prefix("stream:")
                .unwrap_or_else(|| panic!("expected 'stream:' prefix in test case {name}"));
            let input_bytes = hex::decode(hex_data)
                .unwrap_or_else(|e| panic!("invalid hex in test case {name}: {e}"));

            let result = gen_instance_code_v0(&input_bytes, bits)
                .unwrap_or_else(|e| panic!("gen_instance_code_v0 failed for {name}: {e}"));
            assert_eq!(result, expected_iscc, "mismatch in test case {name}");
        }
    }

    #[test]
    fn test_gen_iscc_code_v0_stub() {
        assert!(matches!(
            gen_iscc_code_v0(&["ISCC:AAA"], false),
            Err(IsccError::NotImplemented)
        ));
    }
}
