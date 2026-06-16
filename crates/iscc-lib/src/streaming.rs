//! Streaming hash types for incremental ISCC code generation.
//!
//! Provides `DataHasher`, `InstanceHasher`, and `SumHasher` — streaming
//! counterparts to `gen_data_code_v0`, `gen_instance_code_v0`, and
//! `gen_sum_code_v0`. All follow the
//! `new() → update(&[u8]) → finalize()` pattern for incremental processing
//! of large files without loading entire contents into memory.

use crate::types::{DataCodeResult, InstanceCodeResult, SumCodeResult};
use crate::{IsccResult, cdc, codec, gen_iscc_code_v0, minhash};

/// Streaming Instance-Code generator.
///
/// Incrementally hashes data with BLAKE3 to produce an ISCC Instance-Code
/// identical to `gen_instance_code_v0` for the same byte stream.
pub struct InstanceHasher {
    hasher: blake3::Hasher,
    filesize: u64,
}

impl InstanceHasher {
    /// Create a new `InstanceHasher`.
    pub fn new() -> Self {
        Self {
            hasher: blake3::Hasher::new(),
            filesize: 0,
        }
    }

    /// Push data into the hasher.
    pub fn update(&mut self, data: &[u8]) {
        self.filesize += data.len() as u64;
        self.hasher.update(data);
    }

    /// Consume the hasher and produce an Instance-Code result.
    ///
    /// Equivalent to calling `gen_instance_code_v0` with the concatenation
    /// of all data passed to `update`.
    pub fn finalize(self, bits: u32) -> IsccResult<InstanceCodeResult> {
        let digest = self.hasher.finalize();
        let datahash = format!("1e20{}", hex::encode(digest.as_bytes()));
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
            filesize: self.filesize,
        })
    }
}

impl Default for InstanceHasher {
    /// Create a new `InstanceHasher` (delegates to `new()`).
    fn default() -> Self {
        Self::new()
    }
}

/// Streaming Data-Code generator.
///
/// Incrementally processes data with content-defined chunking (CDC) and
/// MinHash to produce an ISCC Data-Code identical to `gen_data_code_v0`
/// for the same byte stream. Uses a persistent internal buffer to avoid
/// per-call heap allocations.
pub struct DataHasher {
    chunk_features: Vec<u32>,
    buf: Vec<u8>,
}

impl DataHasher {
    /// Create a new `DataHasher`.
    pub fn new() -> Self {
        Self {
            chunk_features: Vec::new(),
            buf: Vec::new(),
        }
    }

    /// Push data into the hasher.
    ///
    /// Appends data to the internal buffer (which starts with the retained
    /// tail from the previous call), runs CDC, hashes all complete chunks,
    /// and shifts the last chunk (tail) to the front of the buffer for the
    /// next call. The buffer is reused across calls to avoid allocations.
    pub fn update(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);

        let chunks = cdc::alg_cdc_chunks_unchecked(&self.buf, false, cdc::DATA_AVG_CHUNK_SIZE);

        // Process all chunks except the last (which becomes the new tail).
        // This mirrors the Python `push()` method's `prev_chunk` pattern.
        let mut prev_chunk: Option<&[u8]> = None;
        for chunk in &chunks {
            if let Some(pc) = prev_chunk {
                self.chunk_features.push(xxhash_rust::xxh32::xxh32(pc, 0));
            }
            prev_chunk = Some(chunk);
        }

        // Extract tail length before dropping borrows on self.buf
        let tail_len = prev_chunk.map_or(0, |c| c.len());
        drop(chunks);

        // Shift tail to front of buffer, reusing existing capacity
        let tail_start = self.buf.len() - tail_len;
        self.buf.copy_within(tail_start.., 0);
        self.buf.truncate(tail_len);
    }

    /// Consume the hasher and produce a Data-Code result.
    ///
    /// Equivalent to calling `gen_data_code_v0` with the concatenation
    /// of all data passed to `update`.
    pub fn finalize(mut self, bits: u32) -> IsccResult<DataCodeResult> {
        if !self.buf.is_empty() {
            self.chunk_features
                .push(xxhash_rust::xxh32::xxh32(&self.buf, 0));
        } else if self.chunk_features.is_empty() {
            // Empty input: ensure at least one feature
            self.chunk_features.push(xxhash_rust::xxh32::xxh32(b"", 0));
        }

        let digest = minhash::alg_minhash_256(&self.chunk_features);
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
}

impl Default for DataHasher {
    /// Create a new `DataHasher` (delegates to `new()`).
    fn default() -> Self {
        Self::new()
    }
}

/// Streaming composite ISCC-CODE (Sum) generator.
///
/// Runs the Data-Code (CDC/MinHash) and Instance-Code (BLAKE3) algorithms in a
/// single pass over the input by feeding the same bytes to an inner
/// `DataHasher` and `InstanceHasher`, then composes the final ISCC-CODE.
/// Produces output identical to `gen_sum_code_v0` for the same byte stream.
pub struct SumHasher {
    data_hasher: DataHasher,
    instance_hasher: InstanceHasher,
}

impl SumHasher {
    /// Create a new `SumHasher`.
    pub fn new() -> Self {
        Self {
            data_hasher: DataHasher::new(),
            instance_hasher: InstanceHasher::new(),
        }
    }

    /// Push data into both inner hashers in a single pass.
    pub fn update(&mut self, data: &[u8]) {
        self.data_hasher.update(data);
        self.instance_hasher.update(data);
    }

    /// Consume the hasher and produce a composite ISCC-CODE result.
    ///
    /// Finalizes the inner Data-Code and Instance-Code, then composes them via
    /// `gen_iscc_code_v0`. When `add_units` is `true`, the result includes the
    /// individual Data-Code and Instance-Code ISCC strings at the requested
    /// `bits` precision. Equivalent to calling `gen_sum_code_v0` on a file
    /// containing the concatenation of all data passed to `update`.
    pub fn finalize(self, bits: u32, wide: bool, add_units: bool) -> IsccResult<SumCodeResult> {
        let data_result = self.data_hasher.finalize(bits)?;
        let instance_result = self.instance_hasher.finalize(bits)?;

        // Borrow strings for gen_iscc_code_v0 before potentially moving them into units.
        let iscc_result = gen_iscc_code_v0(&[&data_result.iscc, &instance_result.iscc], wide)?;

        let units = if add_units {
            Some(vec![data_result.iscc, instance_result.iscc])
        } else {
            None
        };

        Ok(SumCodeResult {
            iscc: iscc_result.iscc,
            datahash: instance_result.datahash,
            filesize: instance_result.filesize,
            units,
        })
    }
}

impl Default for SumHasher {
    /// Create a new `SumHasher` (delegates to `new()`).
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{gen_data_code_v0, gen_instance_code_v0, gen_iscc_code_v0, gen_sum_code_v0};

    // ---- InstanceHasher tests ----

    #[test]
    fn test_instance_hasher_empty() {
        let ih = InstanceHasher::new();
        let streaming = ih.finalize(64).unwrap();
        let oneshot = gen_instance_code_v0(b"", 64).unwrap();
        assert_eq!(streaming.iscc, oneshot.iscc);
        assert_eq!(streaming.datahash, oneshot.datahash);
        assert_eq!(streaming.filesize, oneshot.filesize);
        assert_eq!(streaming.filesize, 0);
    }

    #[test]
    fn test_instance_hasher_small_data() {
        let data = b"Hello, ISCC World!";
        let mut ih = InstanceHasher::new();
        ih.update(data);
        let streaming = ih.finalize(64).unwrap();
        let oneshot = gen_instance_code_v0(data, 64).unwrap();
        assert_eq!(streaming.iscc, oneshot.iscc);
        assert_eq!(streaming.datahash, oneshot.datahash);
        assert_eq!(streaming.filesize, oneshot.filesize);
    }

    #[test]
    fn test_instance_hasher_multi_chunk() {
        let data = b"The quick brown fox jumps over the lazy dog";
        let mut ih = InstanceHasher::new();
        ih.update(&data[..10]);
        ih.update(&data[10..25]);
        ih.update(&data[25..]);
        let streaming = ih.finalize(64).unwrap();
        let oneshot = gen_instance_code_v0(data, 64).unwrap();
        assert_eq!(streaming.iscc, oneshot.iscc);
        assert_eq!(streaming.datahash, oneshot.datahash);
        assert_eq!(streaming.filesize, oneshot.filesize);
    }

    #[test]
    fn test_instance_hasher_byte_at_a_time() {
        let data = b"streaming byte by byte";
        let mut ih = InstanceHasher::new();
        for &b in data.iter() {
            ih.update(&[b]);
        }
        let streaming = ih.finalize(128).unwrap();
        let oneshot = gen_instance_code_v0(data, 128).unwrap();
        assert_eq!(streaming.iscc, oneshot.iscc);
        assert_eq!(streaming.datahash, oneshot.datahash);
        assert_eq!(streaming.filesize, oneshot.filesize);
    }

    #[test]
    fn test_instance_hasher_default() {
        let ih = InstanceHasher::default();
        let streaming = ih.finalize(64).unwrap();
        let oneshot = gen_instance_code_v0(b"", 64).unwrap();
        assert_eq!(streaming.iscc, oneshot.iscc);
    }

    #[test]
    fn test_instance_hasher_various_bits() {
        let data = b"test various bit widths";
        for bits in [64, 128, 256] {
            let mut ih = InstanceHasher::new();
            ih.update(data);
            let streaming = ih.finalize(bits).unwrap();
            let oneshot = gen_instance_code_v0(data, bits).unwrap();
            assert_eq!(streaming.iscc, oneshot.iscc, "bits={bits}");
            assert_eq!(streaming.datahash, oneshot.datahash, "bits={bits}");
        }
    }

    #[test]
    fn test_instance_hasher_conformance() {
        let json_str = include_str!("../tests/data.json");
        let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let section = &data["gen_instance_code_v0"];
        let cases = section.as_object().unwrap();

        for (name, tc) in cases {
            let inputs = tc["inputs"].as_array().unwrap();
            let stream_str = inputs[0].as_str().unwrap();
            let bits = inputs[1].as_u64().unwrap() as u32;

            let hex_data = stream_str
                .strip_prefix("stream:")
                .unwrap_or_else(|| panic!("expected 'stream:' prefix in test case {name}"));
            let input_bytes = hex::decode(hex_data)
                .unwrap_or_else(|e| panic!("invalid hex in test case {name}: {e}"));

            // One-shot reference
            let oneshot = gen_instance_code_v0(&input_bytes, bits)
                .unwrap_or_else(|e| panic!("gen_instance_code_v0 failed for {name}: {e}"));

            // Streaming — single update
            let mut ih = InstanceHasher::new();
            ih.update(&input_bytes);
            let streaming = ih
                .finalize(bits)
                .unwrap_or_else(|e| panic!("InstanceHasher failed for {name}: {e}"));

            assert_eq!(
                streaming.iscc, oneshot.iscc,
                "ISCC mismatch in test case {name}"
            );
            assert_eq!(
                streaming.datahash, oneshot.datahash,
                "datahash mismatch in test case {name}"
            );
            assert_eq!(
                streaming.filesize, oneshot.filesize,
                "filesize mismatch in test case {name}"
            );

            // Streaming — multi-chunk (split into 256-byte chunks)
            let mut ih2 = InstanceHasher::new();
            for chunk in input_bytes.chunks(256) {
                ih2.update(chunk);
            }
            let streaming2 = ih2
                .finalize(bits)
                .unwrap_or_else(|e| panic!("InstanceHasher multi-chunk failed for {name}: {e}"));

            assert_eq!(
                streaming2.iscc, oneshot.iscc,
                "multi-chunk ISCC mismatch in test case {name}"
            );
            assert_eq!(
                streaming2.datahash, oneshot.datahash,
                "multi-chunk datahash mismatch in test case {name}"
            );
        }
    }

    // ---- DataHasher tests ----

    #[test]
    fn test_data_hasher_empty() {
        let dh = DataHasher::new();
        let streaming = dh.finalize(64).unwrap();
        let oneshot = gen_data_code_v0(b"", 64).unwrap();
        assert_eq!(streaming.iscc, oneshot.iscc);
    }

    #[test]
    fn test_data_hasher_small_data() {
        let data = b"Hello, ISCC World!";
        let mut dh = DataHasher::new();
        dh.update(data);
        let streaming = dh.finalize(64).unwrap();
        let oneshot = gen_data_code_v0(data, 64).unwrap();
        assert_eq!(streaming.iscc, oneshot.iscc);
    }

    #[test]
    fn test_data_hasher_multi_chunk_small() {
        let data = b"The quick brown fox jumps over the lazy dog";
        let mut dh = DataHasher::new();
        dh.update(&data[..10]);
        dh.update(&data[10..25]);
        dh.update(&data[25..]);
        let streaming = dh.finalize(64).unwrap();
        let oneshot = gen_data_code_v0(data, 64).unwrap();
        assert_eq!(streaming.iscc, oneshot.iscc);
    }

    #[test]
    fn test_data_hasher_byte_at_a_time() {
        // Small data that fits within a single CDC chunk
        let data = b"streaming byte by byte";
        let mut dh = DataHasher::new();
        for &b in data.iter() {
            dh.update(&[b]);
        }
        let streaming = dh.finalize(64).unwrap();
        let oneshot = gen_data_code_v0(data, 64).unwrap();
        assert_eq!(streaming.iscc, oneshot.iscc);
    }

    #[test]
    fn test_data_hasher_large_data_multi_chunk() {
        // Generate data large enough to produce multiple CDC chunks
        let data: Vec<u8> = (0..10_000).map(|i| (i % 256) as u8).collect();
        for chunk_size in [1, 256, 1024, 4096] {
            let mut dh = DataHasher::new();
            for chunk in data.chunks(chunk_size) {
                dh.update(chunk);
            }
            let streaming = dh.finalize(64).unwrap();
            let oneshot = gen_data_code_v0(&data, 64).unwrap();
            assert_eq!(
                streaming.iscc, oneshot.iscc,
                "chunk_size={chunk_size} mismatch"
            );
        }
    }

    #[test]
    fn test_data_hasher_default() {
        let dh = DataHasher::default();
        let streaming = dh.finalize(64).unwrap();
        let oneshot = gen_data_code_v0(b"", 64).unwrap();
        assert_eq!(streaming.iscc, oneshot.iscc);
    }

    #[test]
    fn test_data_hasher_various_bits() {
        let data = b"test various bit widths for data code";
        for bits in [64, 128, 256] {
            let mut dh = DataHasher::new();
            dh.update(data);
            let streaming = dh.finalize(bits).unwrap();
            let oneshot = gen_data_code_v0(data, bits).unwrap();
            assert_eq!(streaming.iscc, oneshot.iscc, "bits={bits}");
        }
    }

    #[test]
    fn test_data_hasher_conformance() {
        let json_str = include_str!("../tests/data.json");
        let data: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let section = &data["gen_data_code_v0"];
        let cases = section.as_object().unwrap();

        for (name, tc) in cases {
            let inputs = tc["inputs"].as_array().unwrap();
            let stream_str = inputs[0].as_str().unwrap();
            let bits = inputs[1].as_u64().unwrap() as u32;

            let hex_data = stream_str
                .strip_prefix("stream:")
                .unwrap_or_else(|| panic!("expected 'stream:' prefix in test case {name}"));
            let input_bytes = hex::decode(hex_data)
                .unwrap_or_else(|e| panic!("invalid hex in test case {name}: {e}"));

            // One-shot reference
            let oneshot = gen_data_code_v0(&input_bytes, bits)
                .unwrap_or_else(|e| panic!("gen_data_code_v0 failed for {name}: {e}"));

            // Streaming — single update
            let mut dh = DataHasher::new();
            dh.update(&input_bytes);
            let streaming = dh
                .finalize(bits)
                .unwrap_or_else(|e| panic!("DataHasher failed for {name}: {e}"));

            assert_eq!(
                streaming.iscc, oneshot.iscc,
                "ISCC mismatch in test case {name}"
            );

            // Streaming — 256-byte chunks
            let mut dh2 = DataHasher::new();
            for chunk in input_bytes.chunks(256) {
                dh2.update(chunk);
            }
            let streaming2 = dh2
                .finalize(bits)
                .unwrap_or_else(|e| panic!("DataHasher multi-chunk failed for {name}: {e}"));

            assert_eq!(
                streaming2.iscc, oneshot.iscc,
                "multi-chunk ISCC mismatch in test case {name}"
            );

            // Streaming — 1-byte chunks (stress test)
            let mut dh3 = DataHasher::new();
            for &b in &input_bytes {
                dh3.update(&[b]);
            }
            let streaming3 = dh3
                .finalize(bits)
                .unwrap_or_else(|e| panic!("DataHasher byte-at-a-time failed for {name}: {e}"));

            assert_eq!(
                streaming3.iscc, oneshot.iscc,
                "byte-at-a-time ISCC mismatch in test case {name}"
            );
        }
    }

    // ---- SumHasher tests ----

    /// Compose the expected composite ISCC-CODE for `data` from the one-shot
    /// Data-Code and Instance-Code functions (the streaming source of truth).
    fn expected_sum(data: &[u8], bits: u32, wide: bool) -> SumCodeResult {
        let data_result = gen_data_code_v0(data, bits).unwrap();
        let instance_result = gen_instance_code_v0(data, bits).unwrap();
        let iscc_result =
            gen_iscc_code_v0(&[&data_result.iscc, &instance_result.iscc], wide).unwrap();
        SumCodeResult {
            iscc: iscc_result.iscc,
            datahash: instance_result.datahash,
            filesize: instance_result.filesize,
            units: None,
        }
    }

    #[test]
    fn test_sum_hasher_empty() {
        let streaming = SumHasher::new().finalize(64, false, false).unwrap();
        let expected = expected_sum(b"", 64, false);

        assert_eq!(streaming.iscc, expected.iscc);
        assert_eq!(streaming.datahash, expected.datahash);
        assert_eq!(streaming.filesize, expected.filesize);
        assert_eq!(streaming.filesize, 0);
        assert_eq!(streaming.units, None);
    }

    #[test]
    fn test_sum_hasher_small_data() {
        let data = b"Hello, ISCC World!";
        let mut sh = SumHasher::new();
        sh.update(data);
        let streaming = sh.finalize(64, false, false).unwrap();
        let expected = expected_sum(data, 64, false);

        assert_eq!(streaming.iscc, expected.iscc);
        assert_eq!(streaming.datahash, expected.datahash);
        assert_eq!(streaming.filesize, expected.filesize);
    }

    #[test]
    fn test_sum_hasher_multi_update_invariance() {
        // Large enough to span multiple CDC chunks.
        let data: Vec<u8> = (0..20_000).map(|i| (i % 256) as u8).collect();

        let mut single = SumHasher::new();
        single.update(&data);
        let single_result = single.finalize(64, false, false).unwrap();

        // Split the same bytes across three update calls.
        let mut multi = SumHasher::new();
        multi.update(&data[..3000]);
        multi.update(&data[3000..12_000]);
        multi.update(&data[12_000..]);
        let multi_result = multi.finalize(64, false, false).unwrap();

        assert_eq!(single_result.iscc, multi_result.iscc);
        assert_eq!(single_result.datahash, multi_result.datahash);
        assert_eq!(single_result.filesize, multi_result.filesize);

        // And both match the one-shot composition.
        let expected = expected_sum(&data, 64, false);
        assert_eq!(multi_result.iscc, expected.iscc);
    }

    #[test]
    fn test_sum_hasher_units_toggle() {
        let data = b"unit toggle test data";

        let mut sh_on = SumHasher::new();
        sh_on.update(data);
        let with_units = sh_on.finalize(64, false, true).unwrap();

        let data_result = gen_data_code_v0(data, 64).unwrap();
        let instance_result = gen_instance_code_v0(data, 64).unwrap();
        assert_eq!(
            with_units.units,
            Some(vec![data_result.iscc, instance_result.iscc])
        );

        let mut sh_off = SumHasher::new();
        sh_off.update(data);
        let without_units = sh_off.finalize(64, false, false).unwrap();
        assert_eq!(without_units.units, None);
    }

    #[test]
    fn test_sum_hasher_wide_mode() {
        let data = b"wide mode comparison test data";

        let mut narrow = SumHasher::new();
        narrow.update(data);
        let narrow_result = narrow.finalize(128, false, false).unwrap();

        let mut wide = SumHasher::new();
        wide.update(data);
        let wide_result = wide.finalize(128, true, false).unwrap();

        // Wide mode at 128 bits produces a different (longer) composite code.
        assert_ne!(narrow_result.iscc, wide_result.iscc);
        // datahash and filesize are unaffected by wide mode.
        assert_eq!(narrow_result.datahash, wide_result.datahash);
        assert_eq!(narrow_result.filesize, wide_result.filesize);

        // Each matches its one-shot composition.
        assert_eq!(narrow_result.iscc, expected_sum(data, 128, false).iscc);
        assert_eq!(wide_result.iscc, expected_sum(data, 128, true).iscc);
    }

    #[test]
    fn test_sum_hasher_datahash_filesize_match_instance() {
        let data = b"datahash and filesize parity check";
        let mut sh = SumHasher::new();
        sh.update(data);
        let streaming = sh.finalize(64, false, false).unwrap();

        let instance_result = gen_instance_code_v0(data, 64).unwrap();
        assert_eq!(streaming.datahash, instance_result.datahash);
        assert_eq!(streaming.filesize, instance_result.filesize);
        assert_eq!(streaming.filesize, data.len() as u64);
    }

    #[test]
    fn test_sum_hasher_default() {
        let from_default = SumHasher::default().finalize(64, false, false).unwrap();
        let from_new = SumHasher::new().finalize(64, false, false).unwrap();
        assert_eq!(from_default.iscc, from_new.iscc);
    }

    #[test]
    fn test_sum_hasher_matches_gen_sum_code_v0() {
        use std::io::Write;

        // Large enough to span multiple CDC chunks.
        let data: Vec<u8> = (0..20_000).map(|i| (i % 256) as u8).collect();

        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        tmp.write_all(&data).unwrap();
        tmp.flush().unwrap();

        // Confirm SumHasher is the single source of truth across parameter
        // combinations: same iscc/datahash/filesize/units as the file-based path.
        for &(bits, wide, add_units) in &[
            (64u32, false, false),
            (64u32, false, true),
            (128u32, true, true),
        ] {
            let mut sh = SumHasher::new();
            sh.update(&data);
            let streaming = sh.finalize(bits, wide, add_units).unwrap();

            let file_based = gen_sum_code_v0(tmp.path(), bits, wide, add_units).unwrap();

            assert_eq!(
                streaming.iscc, file_based.iscc,
                "iscc mismatch (bits={bits}, wide={wide}, add_units={add_units})"
            );
            assert_eq!(streaming.datahash, file_based.datahash);
            assert_eq!(streaming.filesize, file_based.filesize);
            assert_eq!(streaming.units, file_based.units);
        }
    }
}
