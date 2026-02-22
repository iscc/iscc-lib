//! High-performance Rust implementation of ISO 24138:2024 (ISCC).
//!
//! This crate provides the core ISCC algorithm implementations. All 9 `gen_*_v0`
//! functions are the public Tier 1 API surface, designed to be compatible with
//! the `iscc-core` Python reference implementation.

pub mod codec;
pub(crate) mod simhash;
pub(crate) mod utils;

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

/// Compute a similarity-preserving 256-bit hash from metadata text.
///
/// Produces a SimHash digest from `name` n-grams. When `extra` is provided,
/// interleaves the name and extra SimHash digests in 4-byte chunks.
fn soft_hash_meta_v0(name: &str, extra: Option<&str>) -> Vec<u8> {
    let collapsed_name = utils::text_collapse(name);
    let name_ngrams = simhash::sliding_window(&collapsed_name, 3);
    let name_hashes: Vec<[u8; 32]> = name_ngrams
        .iter()
        .map(|ng| *blake3::hash(ng.as_bytes()).as_bytes())
        .collect();
    let name_simhash = simhash::alg_simhash(&name_hashes);

    match extra {
        None | Some("") => name_simhash,
        Some(extra_str) => {
            let collapsed_extra = utils::text_collapse(extra_str);
            let extra_ngrams = simhash::sliding_window(&collapsed_extra, 3);
            let extra_hashes: Vec<[u8; 32]> = extra_ngrams
                .iter()
                .map(|ng| *blake3::hash(ng.as_bytes()).as_bytes())
                .collect();
            let extra_simhash = simhash::alg_simhash(&extra_hashes);

            // Interleave first 16 bytes of each simhash in 4-byte chunks
            let mut result = vec![0u8; 32];
            for chunk in 0..4 {
                let src = chunk * 4;
                let dst_name = chunk * 8;
                let dst_extra = chunk * 8 + 4;
                result[dst_name..dst_name + 4].copy_from_slice(&name_simhash[src..src + 4]);
                result[dst_extra..dst_extra + 4].copy_from_slice(&extra_simhash[src..src + 4]);
            }
            result
        }
    }
}

/// Generate a Meta-Code from name and optional metadata.
///
/// Produces an ISCC Meta-Code by hashing the provided name, description,
/// and metadata fields using the SimHash algorithm. Returns `NotImplemented`
/// if `meta` is provided (JSON/Data-URL meta objects are not yet supported).
pub fn gen_meta_code_v0(
    name: &str,
    description: Option<&str>,
    meta: Option<&str>,
    bits: u32,
) -> IsccResult<String> {
    if meta.is_some() {
        return Err(IsccError::NotImplemented);
    }

    // Normalize name: clean → remove newlines → trim to 128 bytes
    let name = utils::text_clean(name);
    let name = utils::text_remove_newlines(&name);
    let name = utils::text_trim(&name, 128);

    if name.is_empty() {
        return Err(IsccError::InvalidInput(
            "name is empty after normalization".into(),
        ));
    }

    // Normalize description: clean → trim to 4096 bytes
    let desc_str = description.unwrap_or("");
    let desc_clean = utils::text_clean(desc_str);
    let desc_clean = utils::text_trim(&desc_clean, 4096);

    // Compute metahash from normalized payload (stored for future result struct)
    let payload = if desc_clean.is_empty() {
        name.clone()
    } else {
        format!("{} {}", name, desc_clean)
    };
    let payload = payload.trim().to_string();
    let _metahash = utils::multi_hash_blake3(payload.as_bytes());

    // Compute similarity digest
    let extra = if desc_clean.is_empty() {
        None
    } else {
        Some(desc_clean.as_str())
    };
    let meta_code_digest = soft_hash_meta_v0(&name, extra);

    // Encode as ISCC component
    let meta_code = codec::encode_component(
        codec::MainType::Meta,
        codec::SubType::None,
        codec::Version::V0,
        bits,
        &meta_code_digest,
    )?;

    Ok(format!("ISCC:{meta_code}"))
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
    fn test_gen_meta_code_v0_title_only() {
        let result = gen_meta_code_v0("Die Unendliche Geschichte", None, None, 64).unwrap();
        assert_eq!(result, "ISCC:AAAZXZ6OU74YAZIM");
    }

    #[test]
    fn test_gen_meta_code_v0_title_description() {
        let result = gen_meta_code_v0(
            "Die Unendliche Geschichte",
            Some("Von Michael Ende"),
            None,
            64,
        )
        .unwrap();
        assert_eq!(result, "ISCC:AAAZXZ6OU4E45RB5");
    }

    #[test]
    fn test_gen_meta_code_v0_meta_not_implemented() {
        assert!(matches!(
            gen_meta_code_v0("test", None, Some("{}"), 64),
            Err(IsccError::NotImplemented)
        ));
    }

    #[test]
    fn test_gen_meta_code_v0_conformance() {
        let json_str = include_str!("../tests/data.json");
        let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let section = &data["gen_meta_code_v0"];
        let cases = section.as_object().unwrap();

        let mut tested = 0;
        let mut skipped = 0;

        for (tc_name, tc) in cases {
            let inputs = tc["inputs"].as_array().unwrap();
            let input_name = inputs[0].as_str().unwrap();
            let input_desc = inputs[1].as_str().unwrap();
            let meta_val = &inputs[2];
            let bits = inputs[3].as_u64().unwrap() as u32;

            let expected_iscc = tc["outputs"]["iscc"].as_str().unwrap();
            let expected_metahash = tc["outputs"]["metahash"].as_str().unwrap();

            // Skip test cases with meta objects (deferred)
            if !meta_val.is_null() {
                eprintln!("Skipping {tc_name}: meta object support deferred");
                skipped += 1;
                continue;
            }

            let desc = if input_desc.is_empty() {
                None
            } else {
                Some(input_desc)
            };

            // Verify ISCC output
            let result = gen_meta_code_v0(input_name, desc, None, bits)
                .unwrap_or_else(|e| panic!("gen_meta_code_v0 failed for {tc_name}: {e}"));
            assert_eq!(
                result, expected_iscc,
                "ISCC mismatch in test case {tc_name}"
            );

            // Verify metahash by re-computing from normalized inputs
            let clean_name = utils::text_clean(input_name);
            let clean_name = utils::text_remove_newlines(&clean_name);
            let clean_name = utils::text_trim(&clean_name, 128);
            let clean_desc = utils::text_clean(input_desc);
            let clean_desc = utils::text_trim(&clean_desc, 4096);

            let payload = if clean_desc.is_empty() {
                clean_name.clone()
            } else {
                format!("{} {}", clean_name, clean_desc)
            };
            let payload = payload.trim().to_string();
            let metahash = utils::multi_hash_blake3(payload.as_bytes());
            assert_eq!(
                metahash, expected_metahash,
                "metahash mismatch in test case {tc_name}"
            );

            // Verify normalized name output
            if let Some(expected_name) = tc["outputs"].get("name") {
                let expected_name = expected_name.as_str().unwrap();
                assert_eq!(
                    clean_name, expected_name,
                    "name mismatch in test case {tc_name}"
                );
            }

            // Verify normalized description output
            if let Some(expected_desc) = tc["outputs"].get("description") {
                let expected_desc = expected_desc.as_str().unwrap();
                assert_eq!(
                    clean_desc, expected_desc,
                    "description mismatch in test case {tc_name}"
                );
            }

            tested += 1;
        }

        assert_eq!(tested, 13, "expected 13 conformance tests to run");
        assert_eq!(skipped, 3, "expected 3 tests to be skipped (meta objects)");
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
