//! Integration tests for the public algorithm primitive API.
//!
//! Verifies that `sliding_window`, `alg_simhash`, `alg_minhash_256`,
//! `alg_cdc_chunks`, and `soft_hash_video_v0` are accessible from the crate
//! root and produce correct results for a variety of inputs.

// ---- sliding_window tests ----

#[test]
fn test_sliding_window_basic_ngrams() {
    // "hello" with width 3 → ["hel", "ell", "llo"]
    let result = iscc_lib::sliding_window("hello", 3);
    assert_eq!(
        result,
        vec!["hel".to_string(), "ell".to_string(), "llo".to_string()]
    );
}

#[test]
fn test_sliding_window_input_shorter_than_width() {
    // Input shorter than width returns a single element with the full input
    let result = iscc_lib::sliding_window("ab", 5);
    assert_eq!(result, vec!["ab".to_string()]);
}

#[test]
fn test_sliding_window_unicode_cjk() {
    // CJK characters are single Unicode code points, width 2
    let result = iscc_lib::sliding_window("你好世界", 2);
    assert_eq!(
        result,
        vec!["你好".to_string(), "好世".to_string(), "世界".to_string()]
    );
}

#[test]
fn test_sliding_window_width_2_minimum() {
    let result = iscc_lib::sliding_window("abcde", 2);
    assert_eq!(
        result,
        vec![
            "ab".to_string(),
            "bc".to_string(),
            "cd".to_string(),
            "de".to_string()
        ]
    );
}

#[test]
fn test_sliding_window_empty_input() {
    // Empty input returns a single empty string element
    let result = iscc_lib::sliding_window("", 3);
    assert_eq!(result, vec!["".to_string()]);
}

// ---- alg_simhash tests ----

#[test]
fn test_alg_simhash_empty_input() {
    // Empty input returns 32 zero bytes
    let empty: Vec<Vec<u8>> = vec![];
    let result = iscc_lib::alg_simhash(&empty);
    assert_eq!(result, vec![0u8; 32]);
}

#[test]
fn test_alg_simhash_single_digest_returns_itself() {
    let digest = vec![0xAB, 0xCD, 0xEF, 0x01];
    let result = iscc_lib::alg_simhash(std::slice::from_ref(&digest));
    assert_eq!(result, digest);
}

#[test]
fn test_alg_simhash_identical_digests() {
    // Multiple identical digests should return the same digest
    let digest = vec![0x42u8; 16];
    let result = iscc_lib::alg_simhash(&[digest.clone(), digest.clone(), digest.clone()]);
    assert_eq!(result, digest);
}

#[test]
fn test_alg_simhash_different_digests_meaningful_hash() {
    // Two opposing digests: simhash with threshold count*2 >= n
    // For n=2, bits present in either digest survive (OR behavior)
    let d1 = vec![0xF0, 0x0F, 0xF0, 0x0F];
    let d2 = vec![0x0F, 0xF0, 0x0F, 0xF0];
    let result = iscc_lib::alg_simhash(&[d1, d2]);
    assert_eq!(result.len(), 4);
    // With 2 digests, threshold is count >= 1, so all set bits from
    // both survive → all 0xFF
    assert_eq!(result, vec![0xFF, 0xFF, 0xFF, 0xFF]);
}

#[test]
fn test_alg_simhash_4byte_digests() {
    // alg_simhash output length matches input digest length
    // 4-byte digests in → 4-byte SimHash out (used by audio code)
    let digests: Vec<[u8; 4]> = vec![[1, 2, 3, 4], [5, 6, 7, 8]];
    let result = iscc_lib::alg_simhash(&digests);
    assert_eq!(result.len(), 4);
}

// ---- alg_minhash_256 tests ----

#[test]
fn test_alg_minhash_256_empty_features() {
    // Empty features produces 32 bytes (all 0xFF due to MAXH fallback)
    let result = iscc_lib::alg_minhash_256(&[]);
    assert_eq!(result.len(), 32);
    assert!(result.iter().all(|&b| b == 0xFF));
}

#[test]
fn test_alg_minhash_256_single_feature() {
    let result = iscc_lib::alg_minhash_256(&[42]);
    assert_eq!(result.len(), 32);
}

#[test]
fn test_alg_minhash_256_deterministic() {
    // Same input always produces same output
    let features = vec![100, 200, 300, 400, 500];
    let result1 = iscc_lib::alg_minhash_256(&features);
    let result2 = iscc_lib::alg_minhash_256(&features);
    assert_eq!(result1, result2);
}

#[test]
fn test_alg_minhash_256_different_features_different_digests() {
    let result_a = iscc_lib::alg_minhash_256(&[1, 2, 3]);
    let result_b = iscc_lib::alg_minhash_256(&[100, 200, 300]);
    assert_ne!(result_a, result_b);
}

// ---- alg_cdc_chunks tests ----

#[test]
fn test_alg_cdc_chunks_basic_chunking() {
    // Known data of sufficient size produces multiple chunks
    let data: Vec<u8> = (0..=255).cycle().take(8192).collect();
    let chunks = iscc_lib::alg_cdc_chunks(&data, false, 1024);
    assert!(
        chunks.len() > 1,
        "expected multiple chunks, got {}",
        chunks.len()
    );
}

#[test]
fn test_alg_cdc_chunks_empty_input() {
    // Empty input returns a single empty chunk
    let chunks = iscc_lib::alg_cdc_chunks(b"", false, 1024);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].len(), 0);
}

#[test]
fn test_alg_cdc_chunks_reassembly() {
    // Concatenating chunks must reproduce the original data
    let data: Vec<u8> = (0..=255).cycle().take(4096).collect();
    let chunks = iscc_lib::alg_cdc_chunks(&data, false, 1024);
    let reassembled: Vec<u8> = chunks.iter().flat_map(|c| c.iter().copied()).collect();
    assert_eq!(reassembled, data);
}

#[test]
fn test_alg_cdc_chunks_utf32_alignment() {
    // With utf32=true, all chunk boundaries must be 4-byte aligned
    let data: Vec<u8> = (0..=255).cycle().take(8192).collect();
    let chunks = iscc_lib::alg_cdc_chunks(&data, true, 1024);
    let mut offset = 0;
    for chunk in &chunks {
        offset += chunk.len();
        assert_eq!(
            offset % 4,
            0,
            "chunk boundary at offset {offset} is not 4-byte aligned"
        );
    }
}

#[test]
fn test_alg_cdc_chunks_smaller_avg_more_chunks() {
    // Smaller avg_chunk_size produces more chunks for the same data
    let data: Vec<u8> = (0..=255).cycle().take(16384).collect();
    let chunks_large = iscc_lib::alg_cdc_chunks(&data, false, 2048);
    let chunks_small = iscc_lib::alg_cdc_chunks(&data, false, 512);
    assert!(
        chunks_small.len() > chunks_large.len(),
        "smaller avg_chunk_size ({}) should produce more chunks than larger ({})",
        chunks_small.len(),
        chunks_large.len()
    );
}

// ---- soft_hash_video_v0 tests ----

#[test]
fn test_soft_hash_video_v0_basic_64bit() {
    // Single all-zero 380-element frame with bits=64 → 8-byte digest
    let frame = vec![0i32; 380];
    let result = iscc_lib::soft_hash_video_v0(&[frame], 64).unwrap();
    assert_eq!(result.len(), 8, "bits=64 should produce 8-byte digest");
}

#[test]
fn test_soft_hash_video_v0_256bit() {
    // Single frame with bits=256 → 32-byte digest
    let frame: Vec<i32> = (0..380).collect();
    let result = iscc_lib::soft_hash_video_v0(&[frame], 256).unwrap();
    assert_eq!(result.len(), 32, "bits=256 should produce 32-byte digest");
}

#[test]
fn test_soft_hash_video_v0_deduplication() {
    // Duplicate frame signatures should not change the result
    let frame: Vec<i32> = (0..380).map(|i| i % 3).collect();
    let single = iscc_lib::soft_hash_video_v0(std::slice::from_ref(&frame), 64).unwrap();
    let doubled = iscc_lib::soft_hash_video_v0(&[frame.clone(), frame], 64).unwrap();
    assert_eq!(single, doubled, "duplicates should be deduplicated");
}

#[test]
fn test_soft_hash_video_v0_empty_input_error() {
    // Empty frame_sigs slice returns InvalidInput error
    let empty: &[Vec<i32>] = &[];
    let err = iscc_lib::soft_hash_video_v0(empty, 64).unwrap_err();
    assert!(
        err.to_string().contains("must not be empty"),
        "expected 'must not be empty' error, got: {err}"
    );
}

#[test]
fn test_soft_hash_video_v0_consistency_with_gen_video_code() {
    // soft_hash_video_v0 digest should match the body of the gen_video_code_v0 output
    // Using conformance vector test_0000: single all-zero 380-element frame, bits=64
    let frame = vec![0i32; 380];
    let digest = iscc_lib::soft_hash_video_v0(std::slice::from_ref(&frame), 64).unwrap();
    let code_result = iscc_lib::gen_video_code_v0(std::slice::from_ref(&frame), 64).unwrap();
    // Strip "ISCC:" prefix and decode base32 to get raw bytes
    let component = code_result.iscc.strip_prefix("ISCC:").unwrap();
    let raw = iscc_lib::codec::decode_base32(component).unwrap();
    // decode_header returns (mtype, stype, version, bits, body)
    let (_mt, _st, _v, _bits, body) = iscc_lib::codec::decode_header(&raw).unwrap();
    assert_eq!(
        digest, body,
        "soft_hash_video_v0 digest should match gen_video_code_v0 body"
    );
}

// ---- Import path verification ----

#[test]
fn test_flat_crate_root_imports() {
    // Verify all 5 algorithm primitives are callable via iscc_lib::<fn>
    let _ = iscc_lib::sliding_window("test", 2);
    let empty: &[Vec<u8>] = &[];
    let _ = iscc_lib::alg_simhash(empty);
    let _ = iscc_lib::alg_minhash_256(&[]);
    let _ = iscc_lib::alg_cdc_chunks(b"", false, 1024);
    let frame = vec![0i32; 380];
    let _ = iscc_lib::soft_hash_video_v0(&[frame], 64);
}

#[test]
fn test_module_path_imports_simhash() {
    // Verify functions are accessible via iscc_lib::simhash::<fn>
    let _ = iscc_lib::simhash::sliding_window("test", 2);
    let empty: &[Vec<u8>] = &[];
    let _ = iscc_lib::simhash::alg_simhash(empty);
}

#[test]
fn test_module_path_imports_minhash() {
    // Verify function is accessible via iscc_lib::minhash::<fn>
    let _ = iscc_lib::minhash::alg_minhash_256(&[]);
}

#[test]
fn test_module_path_imports_cdc() {
    // Verify function is accessible via iscc_lib::cdc::<fn>
    let _ = iscc_lib::cdc::alg_cdc_chunks(b"", false, 1024);
}
