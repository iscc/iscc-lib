//! High-performance Rust implementation of ISO 24138:2024 (ISCC).
//!
//! This crate provides the core ISCC algorithm implementations. All 9 `gen_*_v0`
//! functions are the public Tier 1 API surface, designed to be compatible with
//! the `iscc-core` Python reference implementation.

pub mod codec;
pub(crate) mod minhash;
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

/// Compute a 256-bit similarity-preserving hash from collapsed text.
///
/// Generates character n-grams with a sliding window of width 13,
/// hashes each with xxh32, then applies MinHash to produce a 32-byte digest.
fn soft_hash_text_v0(text: &str) -> Vec<u8> {
    let ngrams = simhash::sliding_window(text, 13);
    let features: Vec<u32> = ngrams
        .iter()
        .map(|ng| xxhash_rust::xxh32::xxh32(ng.as_bytes(), 0))
        .collect();
    minhash::alg_minhash_256(&features)
}

/// Generate a Text-Code from plain text content.
///
/// Produces an ISCC Content-Code for text by collapsing the input,
/// extracting character n-gram features, and applying MinHash to
/// create a similarity-preserving fingerprint.
pub fn gen_text_code_v0(text: &str, bits: u32) -> IsccResult<String> {
    let collapsed = utils::text_collapse(text);
    let _characters = collapsed.chars().count();
    let hash_digest = soft_hash_text_v0(&collapsed);
    let component = codec::encode_component(
        codec::MainType::Content,
        codec::SubType::TEXT,
        codec::Version::V0,
        bits,
        &hash_digest,
    )?;
    Ok(format!("ISCC:{component}"))
}

/// Generate an Image-Code from pixel data.
///
/// Produces an ISCC Content-Code for images from a sequence of pixel
/// values (grayscale, 0-255).
pub fn gen_image_code_v0(_pixels: &[u8], _bits: u32) -> IsccResult<String> {
    Err(IsccError::NotImplemented)
}

/// Split a slice into `n` parts, distributing remainder across first chunks.
///
/// Equivalent to `numpy.array_split` / `more_itertools.divide`:
/// each part gets `len / n` elements, and the first `len % n` parts
/// get one extra element. Returns empty slices for excess parts.
fn array_split<T>(slice: &[T], n: usize) -> Vec<&[T]> {
    if n == 0 {
        return vec![];
    }
    let len = slice.len();
    let base = len / n;
    let remainder = len % n;
    let mut parts = Vec::with_capacity(n);
    let mut offset = 0;
    for i in 0..n {
        let size = base + if i < remainder { 1 } else { 0 };
        parts.push(&slice[offset..offset + size]);
        offset += size;
    }
    parts
}

/// Compute a multi-stage SimHash digest from Chromaprint features.
///
/// Builds a 32-byte digest by concatenating 4-byte SimHash chunks:
/// - Stage 1: overall SimHash of all features (4 bytes)
/// - Stage 2: SimHash of each quarter of features (4 × 4 = 16 bytes)
/// - Stage 3: SimHash of each third of sorted features (3 × 4 = 12 bytes)
fn soft_hash_audio_v0(cv: &[i32]) -> Vec<u8> {
    // Convert each i32 to 4-byte big-endian digest
    let digests: Vec<[u8; 4]> = cv.iter().map(|&v| v.to_be_bytes()).collect();

    if digests.is_empty() {
        return vec![0u8; 32];
    }

    // Stage 1: overall SimHash (4 bytes)
    let mut parts: Vec<u8> = simhash::alg_simhash(&digests);

    // Stage 2: quarter-based SimHashes (4 × 4 = 16 bytes)
    let quarters = array_split(&digests, 4);
    for quarter in &quarters {
        if quarter.is_empty() {
            parts.extend_from_slice(&[0u8; 4]);
        } else {
            parts.extend_from_slice(&simhash::alg_simhash(quarter));
        }
    }

    // Stage 3: sorted-third-based SimHashes (3 × 4 = 12 bytes)
    let mut sorted_values: Vec<i32> = cv.to_vec();
    sorted_values.sort();
    let sorted_digests: Vec<[u8; 4]> = sorted_values.iter().map(|&v| v.to_be_bytes()).collect();
    let thirds = array_split(&sorted_digests, 3);
    for third in &thirds {
        if third.is_empty() {
            parts.extend_from_slice(&[0u8; 4]);
        } else {
            parts.extend_from_slice(&simhash::alg_simhash(third));
        }
    }

    parts
}

/// Generate an Audio-Code from a Chromaprint feature vector.
///
/// Produces an ISCC Content-Code for audio from a Chromaprint signed
/// integer fingerprint vector using multi-stage SimHash.
pub fn gen_audio_code_v0(cv: &[i32], bits: u32) -> IsccResult<String> {
    let hash_digest = soft_hash_audio_v0(cv);
    let component = codec::encode_component(
        codec::MainType::Content,
        codec::SubType::Audio,
        codec::Version::V0,
        bits,
        &hash_digest,
    )?;
    Ok(format!("ISCC:{component}"))
}

/// Generate a Video-Code from frame signature data.
///
/// Produces an ISCC Content-Code for video from a sequence of frame
/// signatures (each frame signature is a byte vector).
pub fn gen_video_code_v0(_frame_sigs: &[Vec<u8>], _bits: u32) -> IsccResult<String> {
    Err(IsccError::NotImplemented)
}

/// Combine multiple Content-Code digests into a single similarity hash.
///
/// Takes raw decoded ISCC bytes (header + body) for each Content-Code and
/// produces a SimHash digest. Each input is trimmed to `bits/8` bytes by
/// keeping the first header byte (encodes type info) plus `nbytes-1` body bytes.
/// Requires at least 2 codes, all of MainType::Content.
fn soft_hash_codes_v0(cc_digests: &[Vec<u8>], bits: u32) -> IsccResult<Vec<u8>> {
    if cc_digests.len() < 2 {
        return Err(IsccError::InvalidInput(
            "at least 2 Content-Codes required for mixing".into(),
        ));
    }

    let nbytes = (bits / 8) as usize;
    let mut prepared: Vec<Vec<u8>> = Vec::with_capacity(cc_digests.len());

    for raw in cc_digests {
        let (mtype, _stype, _ver, _blen, body) = codec::decode_header(raw)?;
        if mtype != codec::MainType::Content {
            return Err(IsccError::InvalidInput(
                "all codes must be Content-Codes".into(),
            ));
        }
        let mut entry = Vec::with_capacity(nbytes);
        entry.push(raw[0]); // first byte preserves type info
        let take = std::cmp::min(nbytes - 1, body.len());
        entry.extend_from_slice(&body[..take]);
        // Pad with zeros if body is shorter than nbytes-1
        while entry.len() < nbytes {
            entry.push(0);
        }
        prepared.push(entry);
    }

    Ok(simhash::alg_simhash(&prepared))
}

/// Generate a Mixed-Code from multiple Content-Code strings.
///
/// Produces a Mixed Content-Code by combining multiple ISCC Content-Codes
/// of different types (text, image, audio, video) using SimHash. Input codes
/// may optionally include the "ISCC:" prefix.
pub fn gen_mixed_code_v0(codes: &[&str], bits: u32) -> IsccResult<String> {
    let decoded: Vec<Vec<u8>> = codes
        .iter()
        .map(|code| {
            let clean = code.strip_prefix("ISCC:").unwrap_or(code);
            codec::decode_base32(clean)
        })
        .collect::<IsccResult<Vec<Vec<u8>>>>()?;

    let digest = soft_hash_codes_v0(&decoded, bits)?;

    let component = codec::encode_component(
        codec::MainType::Content,
        codec::SubType::Mixed,
        codec::Version::V0,
        bits,
        &digest,
    )?;

    Ok(format!("ISCC:{component}"))
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
/// Instance-Code) into a single composite ISCC-CODE. Input codes may
/// optionally include the "ISCC:" prefix. At least Data-Code and
/// Instance-Code are required. When `wide` is true and exactly two
/// 128-bit+ codes (Data + Instance) are provided, produces a 256-bit
/// wide-mode code.
pub fn gen_iscc_code_v0(codes: &[&str], wide: bool) -> IsccResult<String> {
    // Step 1: Clean inputs — strip "ISCC:" prefix
    let cleaned: Vec<&str> = codes
        .iter()
        .map(|c| c.strip_prefix("ISCC:").unwrap_or(c))
        .collect();

    // Step 2: Validate minimum count
    if cleaned.len() < 2 {
        return Err(IsccError::InvalidInput(
            "at least 2 ISCC unit codes required".into(),
        ));
    }

    // Step 3: Validate minimum length (16 base32 chars = 64-bit minimum)
    for code in &cleaned {
        if code.len() < 16 {
            return Err(IsccError::InvalidInput(format!(
                "ISCC unit code too short (min 16 chars): {}",
                code
            )));
        }
    }

    // Step 4: Decode each code
    let mut decoded: Vec<(
        codec::MainType,
        codec::SubType,
        codec::Version,
        u32,
        Vec<u8>,
    )> = Vec::with_capacity(cleaned.len());
    for code in &cleaned {
        let raw = codec::decode_base32(code)?;
        let header = codec::decode_header(&raw)?;
        decoded.push(header);
    }

    // Step 5: Sort by MainType (ascending)
    decoded.sort_by_key(|&(mt, ..)| mt);

    // Step 6: Extract main_types
    let main_types: Vec<codec::MainType> = decoded.iter().map(|&(mt, ..)| mt).collect();

    // Step 7: Validate last two are Data + Instance (mandatory)
    let n = main_types.len();
    if main_types[n - 2] != codec::MainType::Data || main_types[n - 1] != codec::MainType::Instance
    {
        return Err(IsccError::InvalidInput(
            "Data-Code and Instance-Code are mandatory".into(),
        ));
    }

    // Step 8: Determine wide composite
    let is_wide = wide
        && decoded.len() == 2
        && main_types == [codec::MainType::Data, codec::MainType::Instance]
        && decoded
            .iter()
            .all(|&(mt, st, _, len, _)| codec::decode_length(mt, len, st) >= 128);

    // Step 9: Determine SubType
    let st = if is_wide {
        codec::SubType::Wide
    } else {
        // Collect SubTypes of Semantic/Content units
        let sc_subtypes: Vec<codec::SubType> = decoded
            .iter()
            .filter(|&&(mt, ..)| mt == codec::MainType::Semantic || mt == codec::MainType::Content)
            .map(|&(_, st, ..)| st)
            .collect();

        if !sc_subtypes.is_empty() {
            // All must be the same
            let first = sc_subtypes[0];
            if sc_subtypes.iter().all(|&s| s == first) {
                first
            } else {
                return Err(IsccError::InvalidInput(
                    "mixed SubTypes among Content/Semantic units".into(),
                ));
            }
        } else if decoded.len() == 2 {
            codec::SubType::Sum
        } else {
            codec::SubType::IsccNone
        }
    };

    // Step 10–11: Get optional MainTypes and encode
    let optional_types = &main_types[..n - 2];
    let encoded_length = codec::encode_units(optional_types)?;

    // Step 12: Build digest body
    let bytes_per_unit = if is_wide { 16 } else { 8 };
    let mut digest = Vec::with_capacity(decoded.len() * bytes_per_unit);
    for (_, _, _, _, tail) in &decoded {
        let take = bytes_per_unit.min(tail.len());
        digest.extend_from_slice(&tail[..take]);
    }

    // Step 13–14: Encode header + digest as base32
    let header = codec::encode_header(
        codec::MainType::Iscc,
        st,
        codec::Version::V0,
        encoded_length,
    )?;
    let mut code_bytes = header;
    code_bytes.extend_from_slice(&digest);
    let code = codec::encode_base32(&code_bytes);

    // Step 15: Return with prefix
    Ok(format!("ISCC:{code}"))
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
    fn test_gen_text_code_v0_empty() {
        let result = gen_text_code_v0("", 64).unwrap();
        assert_eq!(result, "ISCC:EAASL4F2WZY7KBXB");
    }

    #[test]
    fn test_gen_text_code_v0_hello_world() {
        let result = gen_text_code_v0("Hello World", 64).unwrap();
        assert_eq!(result, "ISCC:EAASKDNZNYGUUF5A");
    }

    #[test]
    fn test_gen_text_code_v0_conformance() {
        let json_str = include_str!("../tests/data.json");
        let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let section = &data["gen_text_code_v0"];
        let cases = section.as_object().unwrap();

        let mut tested = 0;

        for (tc_name, tc) in cases {
            let inputs = tc["inputs"].as_array().unwrap();
            let input_text = inputs[0].as_str().unwrap();
            let bits = inputs[1].as_u64().unwrap() as u32;

            let expected_iscc = tc["outputs"]["iscc"].as_str().unwrap();
            let expected_chars = tc["outputs"]["characters"].as_u64().unwrap() as usize;

            // Verify ISCC output
            let result = gen_text_code_v0(input_text, bits)
                .unwrap_or_else(|e| panic!("gen_text_code_v0 failed for {tc_name}: {e}"));
            assert_eq!(
                result, expected_iscc,
                "ISCC mismatch in test case {tc_name}"
            );

            // Verify character count independently
            let collapsed = utils::text_collapse(input_text);
            let characters = collapsed.chars().count();
            assert_eq!(
                characters, expected_chars,
                "character count mismatch in test case {tc_name}"
            );

            tested += 1;
        }

        assert_eq!(tested, 5, "expected 5 conformance tests to run");
    }

    #[test]
    fn test_gen_image_code_v0_stub() {
        assert!(matches!(
            gen_image_code_v0(&[0u8; 100], 64),
            Err(IsccError::NotImplemented)
        ));
    }

    #[test]
    fn test_gen_audio_code_v0_empty() {
        let result = gen_audio_code_v0(&[], 64).unwrap();
        assert_eq!(result, "ISCC:EIAQAAAAAAAAAAAA");
    }

    #[test]
    fn test_gen_audio_code_v0_single() {
        let result = gen_audio_code_v0(&[1], 128).unwrap();
        assert_eq!(result, "ISCC:EIBQAAAAAEAAAAABAAAAAAAAAAAAA");
    }

    #[test]
    fn test_gen_audio_code_v0_negative() {
        let result = gen_audio_code_v0(&[-1, 0, 1], 256).unwrap();
        assert_eq!(
            result,
            "ISCC:EIDQAAAAAH777777AAAAAAAAAAAACAAAAAAP777774AAAAAAAAAAAAI"
        );
    }

    #[test]
    fn test_gen_audio_code_v0_conformance() {
        let json_str = include_str!("../tests/data.json");
        let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let section = &data["gen_audio_code_v0"];
        let cases = section.as_object().unwrap();

        let mut tested = 0;

        for (tc_name, tc) in cases {
            let inputs = tc["inputs"].as_array().unwrap();
            let cv_json = inputs[0].as_array().unwrap();
            let bits = inputs[1].as_u64().unwrap() as u32;
            let expected_iscc = tc["outputs"]["iscc"].as_str().unwrap();

            let cv: Vec<i32> = cv_json.iter().map(|v| v.as_i64().unwrap() as i32).collect();

            let result = gen_audio_code_v0(&cv, bits)
                .unwrap_or_else(|e| panic!("gen_audio_code_v0 failed for {tc_name}: {e}"));
            assert_eq!(
                result, expected_iscc,
                "ISCC mismatch in test case {tc_name}"
            );

            tested += 1;
        }

        assert_eq!(tested, 5, "expected 5 conformance tests to run");
    }

    #[test]
    fn test_array_split_even() {
        let data = vec![1, 2, 3, 4];
        let parts = array_split(&data, 4);
        assert_eq!(parts, vec![&[1][..], &[2][..], &[3][..], &[4][..]]);
    }

    #[test]
    fn test_array_split_remainder() {
        let data = vec![1, 2, 3, 4, 5];
        let parts = array_split(&data, 3);
        assert_eq!(parts, vec![&[1, 2][..], &[3, 4][..], &[5][..]]);
    }

    #[test]
    fn test_array_split_more_parts_than_elements() {
        let data = vec![1, 2];
        let parts = array_split(&data, 4);
        assert_eq!(
            parts,
            vec![&[1][..], &[2][..], &[][..] as &[i32], &[][..] as &[i32]]
        );
    }

    #[test]
    fn test_array_split_empty() {
        let data: Vec<i32> = vec![];
        let parts = array_split(&data, 3);
        assert_eq!(
            parts,
            vec![&[][..] as &[i32], &[][..] as &[i32], &[][..] as &[i32]]
        );
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
    fn test_gen_mixed_code_v0_conformance() {
        let json_str = include_str!("../tests/data.json");
        let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let section = &data["gen_mixed_code_v0"];
        let cases = section.as_object().unwrap();

        let mut tested = 0;

        for (tc_name, tc) in cases {
            let inputs = tc["inputs"].as_array().unwrap();
            let codes_json = inputs[0].as_array().unwrap();
            let bits = inputs[1].as_u64().unwrap() as u32;
            let expected_iscc = tc["outputs"]["iscc"].as_str().unwrap();
            let expected_parts = tc["outputs"]["parts"].as_array().unwrap();

            let codes: Vec<&str> = codes_json.iter().map(|v| v.as_str().unwrap()).collect();

            let result = gen_mixed_code_v0(&codes, bits)
                .unwrap_or_else(|e| panic!("gen_mixed_code_v0 failed for {tc_name}: {e}"));
            assert_eq!(
                result, expected_iscc,
                "ISCC mismatch in test case {tc_name}"
            );

            // Verify parts match input codes
            let parts: Vec<&str> = expected_parts.iter().map(|v| v.as_str().unwrap()).collect();
            assert_eq!(codes, parts, "parts mismatch in test case {tc_name}");

            tested += 1;
        }

        assert_eq!(tested, 2, "expected 2 conformance tests to run");
    }

    #[test]
    fn test_gen_mixed_code_v0_too_few_codes() {
        assert!(matches!(
            gen_mixed_code_v0(&["EUA6GIKXN42IQV3S"], 64),
            Err(IsccError::InvalidInput(_))
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
    fn test_gen_iscc_code_v0_conformance() {
        let json_str = include_str!("../tests/data.json");
        let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let section = &data["gen_iscc_code_v0"];
        let cases = section.as_object().unwrap();

        let mut tested = 0;

        for (tc_name, tc) in cases {
            let inputs = tc["inputs"].as_array().unwrap();
            let codes_json = inputs[0].as_array().unwrap();
            let expected_iscc = tc["outputs"]["iscc"].as_str().unwrap();

            let codes: Vec<&str> = codes_json.iter().map(|v| v.as_str().unwrap()).collect();

            let result = gen_iscc_code_v0(&codes, false)
                .unwrap_or_else(|e| panic!("gen_iscc_code_v0 failed for {tc_name}: {e}"));
            assert_eq!(
                result, expected_iscc,
                "ISCC mismatch in test case {tc_name}"
            );

            tested += 1;
        }

        assert_eq!(tested, 5, "expected 5 conformance tests to run");
    }

    #[test]
    fn test_gen_iscc_code_v0_too_few_codes() {
        assert!(matches!(
            gen_iscc_code_v0(&["AAAWKLHFPV6OPKDG"], false),
            Err(IsccError::InvalidInput(_))
        ));
    }

    #[test]
    fn test_gen_iscc_code_v0_missing_instance() {
        // Two Meta codes — missing Data and Instance
        assert!(matches!(
            gen_iscc_code_v0(&["AAAWKLHFPV6OPKDG", "AAAWKLHFPV6OPKDG"], false),
            Err(IsccError::InvalidInput(_))
        ));
    }

    #[test]
    fn test_gen_iscc_code_v0_short_code() {
        // Code too short (< 16 chars)
        assert!(matches!(
            gen_iscc_code_v0(&["AAAWKLHFPV6", "AAAWKLHFPV6OPKDG"], false),
            Err(IsccError::InvalidInput(_))
        ));
    }
}
