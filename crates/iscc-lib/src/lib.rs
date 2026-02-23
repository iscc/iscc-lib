//! High-performance Rust implementation of ISO 24138:2024 (ISCC).
//!
//! This crate provides the core ISCC algorithm implementations. All 9 `gen_*_v0`
//! functions are the public Tier 1 API surface, designed to be compatible with
//! the `iscc-core` Python reference implementation.

pub mod cdc;
pub mod codec;
pub(crate) mod dct;
pub mod minhash;
pub mod simhash;
pub mod types;
pub mod utils;
pub(crate) mod wtahash;

pub use cdc::alg_cdc_chunks;
pub use minhash::alg_minhash_256;
pub use simhash::{alg_simhash, sliding_window};
pub use types::*;
pub use utils::{text_clean, text_collapse, text_remove_newlines, text_trim};

/// Error type for ISCC operations.
#[derive(Debug, thiserror::Error)]
pub enum IsccError {
    /// Input data is invalid.
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

/// Result type alias for ISCC operations.
pub type IsccResult<T> = Result<T, IsccError>;

/// Interleave two 32-byte SimHash digests in 4-byte chunks.
///
/// Takes the first 16 bytes of each digest and interleaves them into
/// a 32-byte result: 4 bytes from `a`, 4 bytes from `b`, alternating
/// for 4 rounds (8 chunks total).
fn interleave_digests(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = vec![0u8; 32];
    for chunk in 0..4 {
        let src = chunk * 4;
        let dst_a = chunk * 8;
        let dst_b = chunk * 8 + 4;
        result[dst_a..dst_a + 4].copy_from_slice(&a[src..src + 4]);
        result[dst_b..dst_b + 4].copy_from_slice(&b[src..src + 4]);
    }
    result
}

/// Compute a SimHash digest from the name text for meta hashing.
///
/// Applies `text_collapse`, generates width-3 sliding window n-grams,
/// hashes each with BLAKE3, and produces a SimHash.
fn meta_name_simhash(name: &str) -> Vec<u8> {
    let collapsed_name = utils::text_collapse(name);
    let name_ngrams = simhash::sliding_window(&collapsed_name, 3);
    let name_hashes: Vec<[u8; 32]> = name_ngrams
        .iter()
        .map(|ng| *blake3::hash(ng.as_bytes()).as_bytes())
        .collect();
    simhash::alg_simhash(&name_hashes)
}

/// Compute a similarity-preserving 256-bit hash from metadata text.
///
/// Produces a SimHash digest from `name` n-grams. When `extra` is provided,
/// interleaves the name and extra SimHash digests in 4-byte chunks.
fn soft_hash_meta_v0(name: &str, extra: Option<&str>) -> Vec<u8> {
    let name_simhash = meta_name_simhash(name);

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

            interleave_digests(&name_simhash, &extra_simhash)
        }
    }
}

/// Compute a similarity-preserving 256-bit hash from name text and raw bytes.
///
/// Like `soft_hash_meta_v0` but the extra data is raw bytes instead of text.
/// Uses width-4 byte n-grams (no `text_collapse`) for the bytes path,
/// and interleaves name/bytes SimHash digests in 4-byte chunks.
fn soft_hash_meta_v0_with_bytes(name: &str, extra: &[u8]) -> Vec<u8> {
    let name_simhash = meta_name_simhash(name);

    let byte_ngrams = simhash::sliding_window_bytes(extra, 4);
    let byte_hashes: Vec<[u8; 32]> = byte_ngrams
        .iter()
        .map(|ng| *blake3::hash(ng).as_bytes())
        .collect();
    let byte_simhash = simhash::alg_simhash(&byte_hashes);

    interleave_digests(&name_simhash, &byte_simhash)
}

/// Decode a Data-URL's base64 payload.
///
/// Expects a string starting with `"data:"`. Splits on the first `,` and
/// decodes the remainder as standard base64. Returns `InvalidInput` on
/// missing comma or invalid base64.
fn decode_data_url(data_url: &str) -> IsccResult<Vec<u8>> {
    let payload_b64 = data_url
        .split_once(',')
        .map(|(_, b64)| b64)
        .ok_or_else(|| IsccError::InvalidInput("Data-URL missing comma separator".into()))?;
    data_encoding::BASE64
        .decode(payload_b64.as_bytes())
        .map_err(|e| IsccError::InvalidInput(format!("invalid base64 in Data-URL: {e}")))
}

/// Parse a meta string as JSON and re-serialize to canonical bytes.
///
/// Uses `serde_json` default `BTreeMap`-based key ordering, which produces
/// sorted-key output sufficient for ASCII keys. This is NOT full RFC 8785
/// (JCS) compliance but matches iscc-core behavior for typical metadata.
fn parse_meta_json(meta_str: &str) -> IsccResult<Vec<u8>> {
    let parsed: serde_json::Value = serde_json::from_str(meta_str)
        .map_err(|e| IsccError::InvalidInput(format!("invalid JSON in meta: {e}")))?;
    serde_json::to_vec(&parsed)
        .map_err(|e| IsccError::InvalidInput(format!("JSON serialization failed: {e}")))
}

/// Build a Data-URL from canonical JSON bytes.
///
/// Uses `application/ld+json` media type if the JSON has an `@context` key,
/// otherwise `application/json`. Encodes payload as standard base64 with padding.
fn build_meta_data_url(json_bytes: &[u8], json_value: &serde_json::Value) -> String {
    let media_type = if json_value.get("@context").is_some() {
        "application/ld+json"
    } else {
        "application/json"
    };
    let b64 = data_encoding::BASE64.encode(json_bytes);
    format!("data:{media_type};base64,{b64}")
}

/// Generate a Meta-Code from name and optional metadata.
///
/// Produces an ISCC Meta-Code by hashing the provided name, description,
/// and metadata fields using the SimHash algorithm. When `meta` is provided,
/// it is treated as either a Data-URL (if starting with `"data:"`) or a JSON
/// string, and the decoded/serialized bytes are used for similarity hashing
/// and metahash computation.
pub fn gen_meta_code_v0(
    name: &str,
    description: Option<&str>,
    meta: Option<&str>,
    bits: u32,
) -> IsccResult<MetaCodeResult> {
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

    // Resolve meta payload bytes (if meta is provided)
    let meta_payload: Option<Vec<u8>> = match meta {
        Some(meta_str) if meta_str.starts_with("data:") => {
            let decoded = decode_data_url(meta_str)?;
            if decoded.is_empty() {
                None
            } else {
                Some(decoded)
            }
        }
        Some(meta_str) => Some(parse_meta_json(meta_str)?),
        None => None,
    };

    // Branch: meta bytes path vs. description text path
    if let Some(ref payload) = meta_payload {
        let meta_code_digest = soft_hash_meta_v0_with_bytes(&name, payload);
        let metahash = utils::multi_hash_blake3(payload);

        let meta_code = codec::encode_component(
            codec::MainType::Meta,
            codec::SubType::None,
            codec::Version::V0,
            bits,
            &meta_code_digest,
        )?;

        // Build the meta Data-URL for the result
        let meta_value = match meta {
            Some(meta_str) if meta_str.starts_with("data:") => meta_str.to_string(),
            Some(meta_str) => {
                let parsed: serde_json::Value = serde_json::from_str(meta_str)
                    .map_err(|e| IsccError::InvalidInput(format!("invalid JSON: {e}")))?;
                build_meta_data_url(payload, &parsed)
            }
            None => unreachable!(),
        };

        Ok(MetaCodeResult {
            iscc: format!("ISCC:{meta_code}"),
            name: name.clone(),
            description: if desc_clean.is_empty() {
                None
            } else {
                Some(desc_clean)
            },
            meta: Some(meta_value),
            metahash,
        })
    } else {
        // Compute metahash from normalized text payload
        let payload = if desc_clean.is_empty() {
            name.clone()
        } else {
            format!("{} {}", name, desc_clean)
        };
        let payload = payload.trim().to_string();
        let metahash = utils::multi_hash_blake3(payload.as_bytes());

        // Compute similarity digest
        let extra = if desc_clean.is_empty() {
            None
        } else {
            Some(desc_clean.as_str())
        };
        let meta_code_digest = soft_hash_meta_v0(&name, extra);

        let meta_code = codec::encode_component(
            codec::MainType::Meta,
            codec::SubType::None,
            codec::Version::V0,
            bits,
            &meta_code_digest,
        )?;

        Ok(MetaCodeResult {
            iscc: format!("ISCC:{meta_code}"),
            name: name.clone(),
            description: if desc_clean.is_empty() {
                None
            } else {
                Some(desc_clean)
            },
            meta: None,
            metahash,
        })
    }
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
pub fn gen_text_code_v0(text: &str, bits: u32) -> IsccResult<TextCodeResult> {
    let collapsed = utils::text_collapse(text);
    let characters = collapsed.chars().count();
    let hash_digest = soft_hash_text_v0(&collapsed);
    let component = codec::encode_component(
        codec::MainType::Content,
        codec::SubType::TEXT,
        codec::Version::V0,
        bits,
        &hash_digest,
    )?;
    Ok(TextCodeResult {
        iscc: format!("ISCC:{component}"),
        characters,
    })
}

/// Transpose a matrix represented as a Vec of Vecs.
fn transpose_matrix(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let rows = matrix.len();
    if rows == 0 {
        return vec![];
    }
    let cols = matrix[0].len();
    let mut result = vec![vec![0.0f64; rows]; cols];
    for (r, row) in matrix.iter().enumerate() {
        for (c, &val) in row.iter().enumerate() {
            result[c][r] = val;
        }
    }
    result
}

/// Extract an 8×8 block from a matrix and flatten to 64 values.
///
/// Block position `(col, row)` means the block starts at
/// `matrix[row][col]` and spans 8 rows and 8 columns.
fn flatten_8x8(matrix: &[Vec<f64>], col: usize, row: usize) -> Vec<f64> {
    let mut flat = Vec::with_capacity(64);
    for matrix_row in matrix.iter().skip(row).take(8) {
        for &val in matrix_row.iter().skip(col).take(8) {
            flat.push(val);
        }
    }
    flat
}

/// Compute the median of a slice of f64 values.
///
/// For even-length slices, returns the average of the two middle values
/// (matching Python `statistics.median` behavior).
fn compute_median(values: &[f64]) -> f64 {
    let mut sorted: Vec<f64> = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = sorted.len();
    if n % 2 == 1 {
        sorted[n / 2]
    } else {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    }
}

/// Convert a slice of bools to a byte vector (MSB first per byte).
fn bits_to_bytes(bits: &[bool]) -> Vec<u8> {
    bits.chunks(8)
        .map(|chunk| {
            let mut byte = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                if bit {
                    byte |= 1 << (7 - i);
                }
            }
            byte
        })
        .collect()
}

/// Compute a DCT-based perceptual hash from 32×32 grayscale pixels.
///
/// Applies a 2D DCT to the pixel matrix, extracts four 8×8 low-frequency
/// blocks, and generates a bitstring by comparing each coefficient against
/// the block median. Returns up to `bits` bits as a byte vector.
fn soft_hash_image_v0(pixels: &[u8], bits: u32) -> IsccResult<Vec<u8>> {
    if pixels.len() != 1024 {
        return Err(IsccError::InvalidInput(format!(
            "expected 1024 pixels, got {}",
            pixels.len()
        )));
    }
    if bits > 256 {
        return Err(IsccError::InvalidInput(format!(
            "bits must be <= 256, got {}",
            bits
        )));
    }

    // Step 1: Row-wise DCT (32 rows of 32 pixels)
    let rows: Vec<Vec<f64>> = pixels
        .chunks(32)
        .map(|row| {
            let row_f64: Vec<f64> = row.iter().map(|&p| p as f64).collect();
            dct::alg_dct(&row_f64)
        })
        .collect::<IsccResult<Vec<Vec<f64>>>>()?;

    // Step 2: Transpose
    let transposed = transpose_matrix(&rows);

    // Step 3: Column-wise DCT
    let dct_cols: Vec<Vec<f64>> = transposed
        .iter()
        .map(|col| dct::alg_dct(col))
        .collect::<IsccResult<Vec<Vec<f64>>>>()?;

    // Step 4: Transpose back → dct_matrix
    let dct_matrix = transpose_matrix(&dct_cols);

    // Step 5: Extract 8×8 blocks at positions (0,0), (1,0), (0,1), (1,1)
    let positions = [(0, 0), (1, 0), (0, 1), (1, 1)];
    let mut bitstring = Vec::<bool>::with_capacity(256);

    for (col, row) in positions {
        let flat = flatten_8x8(&dct_matrix, col, row);
        let median = compute_median(&flat);
        for val in &flat {
            bitstring.push(*val > median);
        }
        if bitstring.len() >= bits as usize {
            break;
        }
    }

    // Step 6: Convert first `bits` bools to bytes
    Ok(bits_to_bytes(&bitstring[..bits as usize]))
}

/// Generate an Image-Code from pixel data.
///
/// Produces an ISCC Content-Code for images from a sequence of 1024
/// grayscale pixel values (32×32, values 0-255) using a DCT-based
/// perceptual hash.
pub fn gen_image_code_v0(pixels: &[u8], bits: u32) -> IsccResult<ImageCodeResult> {
    let hash_digest = soft_hash_image_v0(pixels, bits)?;
    let component = codec::encode_component(
        codec::MainType::Content,
        codec::SubType::Image,
        codec::Version::V0,
        bits,
        &hash_digest,
    )?;
    Ok(ImageCodeResult {
        iscc: format!("ISCC:{component}"),
    })
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
pub fn gen_audio_code_v0(cv: &[i32], bits: u32) -> IsccResult<AudioCodeResult> {
    let hash_digest = soft_hash_audio_v0(cv);
    let component = codec::encode_component(
        codec::MainType::Content,
        codec::SubType::Audio,
        codec::Version::V0,
        bits,
        &hash_digest,
    )?;
    Ok(AudioCodeResult {
        iscc: format!("ISCC:{component}"),
    })
}

/// Compute a similarity-preserving hash from video frame signatures.
///
/// Deduplicates frame signatures, computes column-wise sums across all
/// unique frames, then applies WTA-Hash to produce a digest of `bits/8` bytes.
fn soft_hash_video_v0(frame_sigs: &[Vec<i32>], bits: u32) -> IsccResult<Vec<u8>> {
    if frame_sigs.is_empty() {
        return Err(IsccError::InvalidInput(
            "frame_sigs must not be empty".into(),
        ));
    }

    // Deduplicate using BTreeSet (Vec<i32> implements Ord)
    let unique: std::collections::BTreeSet<&Vec<i32>> = frame_sigs.iter().collect();

    // Column-wise sum into i64 to avoid overflow
    let cols = frame_sigs[0].len();
    let mut vecsum = vec![0i64; cols];
    for sig in &unique {
        for (c, &val) in sig.iter().enumerate() {
            vecsum[c] += val as i64;
        }
    }

    Ok(wtahash::alg_wtahash(&vecsum, bits))
}

/// Generate a Video-Code from frame signature data.
///
/// Produces an ISCC Content-Code for video from a sequence of MPEG-7 frame
/// signatures. Each frame signature is a 380-element integer vector.
pub fn gen_video_code_v0(frame_sigs: &[Vec<i32>], bits: u32) -> IsccResult<VideoCodeResult> {
    let digest = soft_hash_video_v0(frame_sigs, bits)?;
    let component = codec::encode_component(
        codec::MainType::Content,
        codec::SubType::Video,
        codec::Version::V0,
        bits,
        &digest,
    )?;
    Ok(VideoCodeResult {
        iscc: format!("ISCC:{component}"),
    })
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
pub fn gen_mixed_code_v0(codes: &[&str], bits: u32) -> IsccResult<MixedCodeResult> {
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

    Ok(MixedCodeResult {
        iscc: format!("ISCC:{component}"),
        parts: codes.iter().map(|s| s.to_string()).collect(),
    })
}

/// Generate a Data-Code from raw byte data.
///
/// Produces an ISCC Data-Code by splitting data into content-defined chunks,
/// hashing each chunk with xxh32, and applying MinHash to create a
/// similarity-preserving fingerprint.
pub fn gen_data_code_v0(data: &[u8], bits: u32) -> IsccResult<DataCodeResult> {
    let chunks = cdc::alg_cdc_chunks(data, false, cdc::DATA_AVG_CHUNK_SIZE);
    let mut features: Vec<u32> = chunks
        .iter()
        .map(|chunk| xxhash_rust::xxh32::xxh32(chunk, 0))
        .collect();

    // Defensive: ensure at least one feature (alg_cdc_chunks guarantees >= 1 chunk)
    if features.is_empty() {
        features.push(xxhash_rust::xxh32::xxh32(b"", 0));
    }

    let digest = minhash::alg_minhash_256(&features);
    let component = codec::encode_component(
        codec::MainType::Data,
        codec::SubType::None,
        codec::Version::V0,
        bits,
        &digest,
    )?;

    Ok(DataCodeResult {
        iscc: format!("ISCC:{component}"),
    })
}

/// Generate an Instance-Code from raw byte data.
///
/// Produces an ISCC Instance-Code by hashing the complete byte stream
/// with BLAKE3. Captures the exact binary identity of the data.
pub fn gen_instance_code_v0(data: &[u8], bits: u32) -> IsccResult<InstanceCodeResult> {
    let digest = blake3::hash(data);
    let datahash = utils::multi_hash_blake3(data);
    let filesize = data.len() as u64;
    let component = codec::encode_component(
        codec::MainType::Instance,
        codec::SubType::None,
        codec::Version::V0,
        bits,
        digest.as_bytes(),
    )?;
    Ok(InstanceCodeResult {
        iscc: format!("ISCC:{component}"),
        datahash,
        filesize,
    })
}

/// Generate a composite ISCC-CODE from individual ISCC unit codes.
///
/// Combines multiple ISCC unit codes (Meta-Code, Content-Code, Data-Code,
/// Instance-Code) into a single composite ISCC-CODE. Input codes may
/// optionally include the "ISCC:" prefix. At least Data-Code and
/// Instance-Code are required. When `wide` is true and exactly two
/// 128-bit+ codes (Data + Instance) are provided, produces a 256-bit
/// wide-mode code.
pub fn gen_iscc_code_v0(codes: &[&str], wide: bool) -> IsccResult<IsccCodeResult> {
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
    Ok(IsccCodeResult {
        iscc: format!("ISCC:{code}"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_meta_code_v0_title_only() {
        let result = gen_meta_code_v0("Die Unendliche Geschichte", None, None, 64).unwrap();
        assert_eq!(result.iscc, "ISCC:AAAZXZ6OU74YAZIM");
        assert_eq!(result.name, "Die Unendliche Geschichte");
        assert_eq!(result.description, None);
        assert_eq!(result.meta, None);
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
        assert_eq!(result.iscc, "ISCC:AAAZXZ6OU4E45RB5");
        assert_eq!(result.name, "Die Unendliche Geschichte");
        assert_eq!(result.description, Some("Von Michael Ende".to_string()));
        assert_eq!(result.meta, None);
    }

    #[test]
    fn test_gen_meta_code_v0_json_meta() {
        let result = gen_meta_code_v0("Hello", None, Some(r#"{"some":"object"}"#), 64).unwrap();
        assert_eq!(result.iscc, "ISCC:AAAWKLHFXN63LHL2");
        assert!(result.meta.is_some());
        assert!(
            result
                .meta
                .unwrap()
                .starts_with("data:application/json;base64,")
        );
    }

    #[test]
    fn test_gen_meta_code_v0_data_url_meta() {
        let result = gen_meta_code_v0(
            "Hello",
            None,
            Some("data:application/json;charset=utf-8;base64,eyJzb21lIjogIm9iamVjdCJ9"),
            64,
        )
        .unwrap();
        assert_eq!(result.iscc, "ISCC:AAAWKLHFXN43ICP2");
        // Data-URL is passed through as-is
        assert_eq!(
            result.meta,
            Some("data:application/json;charset=utf-8;base64,eyJzb21lIjogIm9iamVjdCJ9".to_string())
        );
    }

    #[test]
    fn test_gen_meta_code_v0_invalid_json() {
        assert!(matches!(
            gen_meta_code_v0("test", None, Some("not json"), 64),
            Err(IsccError::InvalidInput(_))
        ));
    }

    #[test]
    fn test_gen_meta_code_v0_invalid_data_url() {
        assert!(matches!(
            gen_meta_code_v0("test", None, Some("data:no-comma-here"), 64),
            Err(IsccError::InvalidInput(_))
        ));
    }

    #[test]
    fn test_gen_meta_code_v0_conformance() {
        let json_str = include_str!("../tests/data.json");
        let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let section = &data["gen_meta_code_v0"];
        let cases = section.as_object().unwrap();

        let mut tested = 0;

        for (tc_name, tc) in cases {
            let inputs = tc["inputs"].as_array().unwrap();
            let input_name = inputs[0].as_str().unwrap();
            let input_desc = inputs[1].as_str().unwrap();
            let meta_val = &inputs[2];
            let bits = inputs[3].as_u64().unwrap() as u32;

            let expected_iscc = tc["outputs"]["iscc"].as_str().unwrap();
            let expected_metahash = tc["outputs"]["metahash"].as_str().unwrap();

            // Dispatch meta parameter based on JSON value type
            let meta_arg: Option<String> = match meta_val {
                serde_json::Value::Null => None,
                serde_json::Value::String(s) => Some(s.clone()),
                serde_json::Value::Object(_) => Some(serde_json::to_string(meta_val).unwrap()),
                other => panic!("unexpected meta type in {tc_name}: {other:?}"),
            };

            let desc = if input_desc.is_empty() {
                None
            } else {
                Some(input_desc)
            };

            // Verify ISCC output from struct
            let result = gen_meta_code_v0(input_name, desc, meta_arg.as_deref(), bits)
                .unwrap_or_else(|e| panic!("gen_meta_code_v0 failed for {tc_name}: {e}"));
            assert_eq!(
                result.iscc, expected_iscc,
                "ISCC mismatch in test case {tc_name}"
            );

            // Verify metahash from struct
            assert_eq!(
                result.metahash, expected_metahash,
                "metahash mismatch in test case {tc_name}"
            );

            // Verify name from struct
            if let Some(expected_name) = tc["outputs"].get("name") {
                let expected_name = expected_name.as_str().unwrap();
                assert_eq!(
                    result.name, expected_name,
                    "name mismatch in test case {tc_name}"
                );
            }

            // Verify description from struct
            if let Some(expected_desc) = tc["outputs"].get("description") {
                let expected_desc = expected_desc.as_str().unwrap();
                assert_eq!(
                    result.description.as_deref(),
                    Some(expected_desc),
                    "description mismatch in test case {tc_name}"
                );
            }

            // Verify meta from struct
            if meta_arg.is_some() {
                assert!(
                    result.meta.is_some(),
                    "meta should be present in test case {tc_name}"
                );
            } else {
                assert!(
                    result.meta.is_none(),
                    "meta should be absent in test case {tc_name}"
                );
            }

            tested += 1;
        }

        assert_eq!(tested, 16, "expected 16 conformance tests to run");
    }

    #[test]
    fn test_gen_text_code_v0_empty() {
        let result = gen_text_code_v0("", 64).unwrap();
        assert_eq!(result.iscc, "ISCC:EAASL4F2WZY7KBXB");
        assert_eq!(result.characters, 0);
    }

    #[test]
    fn test_gen_text_code_v0_hello_world() {
        let result = gen_text_code_v0("Hello World", 64).unwrap();
        assert_eq!(result.iscc, "ISCC:EAASKDNZNYGUUF5A");
        assert_eq!(result.characters, 10); // "helloworld" after collapse
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

            // Verify ISCC output from struct
            let result = gen_text_code_v0(input_text, bits)
                .unwrap_or_else(|e| panic!("gen_text_code_v0 failed for {tc_name}: {e}"));
            assert_eq!(
                result.iscc, expected_iscc,
                "ISCC mismatch in test case {tc_name}"
            );

            // Verify character count from struct
            assert_eq!(
                result.characters, expected_chars,
                "character count mismatch in test case {tc_name}"
            );

            tested += 1;
        }

        assert_eq!(tested, 5, "expected 5 conformance tests to run");
    }

    #[test]
    fn test_gen_image_code_v0_all_black() {
        let pixels = vec![0u8; 1024];
        let result = gen_image_code_v0(&pixels, 64).unwrap();
        assert_eq!(result.iscc, "ISCC:EEAQAAAAAAAAAAAA");
    }

    #[test]
    fn test_gen_image_code_v0_all_white() {
        let pixels = vec![255u8; 1024];
        let result = gen_image_code_v0(&pixels, 128).unwrap();
        assert_eq!(result.iscc, "ISCC:EEBYAAAAAAAAAAAAAAAAAAAAAAAAA");
    }

    #[test]
    fn test_gen_image_code_v0_invalid_pixel_count() {
        assert!(gen_image_code_v0(&[0u8; 100], 64).is_err());
    }

    #[test]
    fn test_gen_image_code_v0_conformance() {
        let json_str = include_str!("../tests/data.json");
        let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let section = &data["gen_image_code_v0"];
        let cases = section.as_object().unwrap();

        let mut tested = 0;

        for (tc_name, tc) in cases {
            let inputs = tc["inputs"].as_array().unwrap();
            let pixels_json = inputs[0].as_array().unwrap();
            let bits = inputs[1].as_u64().unwrap() as u32;
            let expected_iscc = tc["outputs"]["iscc"].as_str().unwrap();

            let pixels: Vec<u8> = pixels_json
                .iter()
                .map(|v| v.as_u64().unwrap() as u8)
                .collect();

            let result = gen_image_code_v0(&pixels, bits)
                .unwrap_or_else(|e| panic!("gen_image_code_v0 failed for {tc_name}: {e}"));
            assert_eq!(
                result.iscc, expected_iscc,
                "ISCC mismatch in test case {tc_name}"
            );

            tested += 1;
        }

        assert_eq!(tested, 3, "expected 3 conformance tests to run");
    }

    #[test]
    fn test_gen_audio_code_v0_empty() {
        let result = gen_audio_code_v0(&[], 64).unwrap();
        assert_eq!(result.iscc, "ISCC:EIAQAAAAAAAAAAAA");
    }

    #[test]
    fn test_gen_audio_code_v0_single() {
        let result = gen_audio_code_v0(&[1], 128).unwrap();
        assert_eq!(result.iscc, "ISCC:EIBQAAAAAEAAAAABAAAAAAAAAAAAA");
    }

    #[test]
    fn test_gen_audio_code_v0_negative() {
        let result = gen_audio_code_v0(&[-1, 0, 1], 256).unwrap();
        assert_eq!(
            result.iscc,
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
                result.iscc, expected_iscc,
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
    fn test_gen_video_code_v0_empty_frames() {
        let frames: Vec<Vec<i32>> = vec![];
        assert!(matches!(
            gen_video_code_v0(&frames, 64),
            Err(IsccError::InvalidInput(_))
        ));
    }

    #[test]
    fn test_gen_video_code_v0_conformance() {
        let json_str = include_str!("../tests/data.json");
        let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let section = &data["gen_video_code_v0"];
        let cases = section.as_object().unwrap();

        let mut tested = 0;

        for (tc_name, tc) in cases {
            let inputs = tc["inputs"].as_array().unwrap();
            let frames_json = inputs[0].as_array().unwrap();
            let bits = inputs[1].as_u64().unwrap() as u32;
            let expected_iscc = tc["outputs"]["iscc"].as_str().unwrap();

            let frame_sigs: Vec<Vec<i32>> = frames_json
                .iter()
                .map(|frame| {
                    frame
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_i64().unwrap() as i32)
                        .collect()
                })
                .collect();

            let result = gen_video_code_v0(&frame_sigs, bits)
                .unwrap_or_else(|e| panic!("gen_video_code_v0 failed for {tc_name}: {e}"));
            assert_eq!(
                result.iscc, expected_iscc,
                "ISCC mismatch in test case {tc_name}"
            );

            tested += 1;
        }

        assert_eq!(tested, 3, "expected 3 conformance tests to run");
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
            let expected_parts: Vec<&str> = tc["outputs"]["parts"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();

            let codes: Vec<&str> = codes_json.iter().map(|v| v.as_str().unwrap()).collect();

            let result = gen_mixed_code_v0(&codes, bits)
                .unwrap_or_else(|e| panic!("gen_mixed_code_v0 failed for {tc_name}: {e}"));
            assert_eq!(
                result.iscc, expected_iscc,
                "ISCC mismatch in test case {tc_name}"
            );

            // Verify parts from struct match expected
            let result_parts: Vec<&str> = result.parts.iter().map(|s| s.as_str()).collect();
            assert_eq!(
                result_parts, expected_parts,
                "parts mismatch in test case {tc_name}"
            );

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
    fn test_gen_data_code_v0_conformance() {
        let json_str = include_str!("../tests/data.json");
        let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let section = &data["gen_data_code_v0"];
        let cases = section.as_object().unwrap();

        let mut tested = 0;

        for (tc_name, tc) in cases {
            let inputs = tc["inputs"].as_array().unwrap();
            let stream_str = inputs[0].as_str().unwrap();
            let bits = inputs[1].as_u64().unwrap() as u32;
            let expected_iscc = tc["outputs"]["iscc"].as_str().unwrap();

            // Parse "stream:" prefix — remainder is hex-encoded bytes
            let hex_data = stream_str
                .strip_prefix("stream:")
                .unwrap_or_else(|| panic!("expected 'stream:' prefix in test case {tc_name}"));
            let input_bytes = hex::decode(hex_data)
                .unwrap_or_else(|e| panic!("invalid hex in test case {tc_name}: {e}"));

            let result = gen_data_code_v0(&input_bytes, bits)
                .unwrap_or_else(|e| panic!("gen_data_code_v0 failed for {tc_name}: {e}"));
            assert_eq!(
                result.iscc, expected_iscc,
                "ISCC mismatch in test case {tc_name}"
            );

            tested += 1;
        }

        assert_eq!(tested, 4, "expected 4 conformance tests to run");
    }

    #[test]
    fn test_gen_instance_code_v0_empty() {
        let result = gen_instance_code_v0(b"", 64).unwrap();
        assert_eq!(result.iscc, "ISCC:IAA26E2JXH27TING");
        assert_eq!(result.filesize, 0);
        assert_eq!(
            result.datahash,
            "1e20af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
        );
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
            assert_eq!(
                result.iscc, expected_iscc,
                "ISCC mismatch in test case {name}"
            );

            // Verify datahash from struct
            if let Some(expected_datahash) = tc["outputs"].get("datahash") {
                let expected_datahash = expected_datahash.as_str().unwrap();
                assert_eq!(
                    result.datahash, expected_datahash,
                    "datahash mismatch in test case {name}"
                );
            }

            // Verify filesize from struct
            if let Some(expected_filesize) = tc["outputs"].get("filesize") {
                let expected_filesize = expected_filesize.as_u64().unwrap();
                assert_eq!(
                    result.filesize, expected_filesize,
                    "filesize mismatch in test case {name}"
                );
            }

            // Also verify filesize matches input data length
            assert_eq!(
                result.filesize,
                input_bytes.len() as u64,
                "filesize should match input length in test case {name}"
            );
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
                result.iscc, expected_iscc,
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
