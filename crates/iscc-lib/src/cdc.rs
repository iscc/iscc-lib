//! Content-Defined Chunking (CDC) for similarity-preserving data splitting.
//!
//! Implements the FastCDC-inspired gear rolling hash algorithm from iscc-core.
//! Splits byte data into variable-size chunks with content-dependent boundaries,
//! enabling similarity detection across different versions of binary data.

/// Gear rolling hash lookup table (256 entries).
///
/// Fixed constant from iscc-core `cdc_gear` option. Each byte value maps to
/// a pseudo-random u32 used in the rolling hash for chunk boundary detection.
const CDC_GEAR: [u32; 256] = [
    1553318008, 574654857, 759734804, 310648967, 1393527547, 1195718329, 694400241, 1154184075,
    1319583805, 1298164590, 122602963, 989043992, 1918895050, 933636724, 1369634190, 1963341198,
    1565176104, 1296753019, 1105746212, 1191982839, 1195494369, 29065008, 1635524067, 722221599,
    1355059059, 564669751, 1620421856, 1100048288, 1018120624, 1087284781, 1723604070, 1415454125,
    737834957, 1854265892, 1605418437, 1697446953, 973791659, 674750707, 1669838606, 320299026,
    1130545851, 1725494449, 939321396, 748475270, 554975894, 1651665064, 1695413559, 671470969,
    992078781, 1935142196, 1062778243, 1901125066, 1935811166, 1644847216, 744420649, 2068980838,
    1988851904, 1263854878, 1979320293, 111370182, 817303588, 478553825, 694867320, 685227566,
    345022554, 2095989693, 1770739427, 165413158, 1322704750, 46251975, 710520147, 700507188,
    2104251000, 1350123687, 1593227923, 1756802846, 1179873910, 1629210470, 358373501, 807118919,
    751426983, 172199468, 174707988, 1951167187, 1328704411, 2129871494, 1242495143, 1793093310,
    1721521010, 306195915, 1609230749, 1992815783, 1790818204, 234528824, 551692332, 1930351755,
    110996527, 378457918, 638641695, 743517326, 368806918, 1583529078, 1767199029, 182158924,
    1114175764, 882553770, 552467890, 1366456705, 934589400, 1574008098, 1798094820, 1548210079,
    821697741, 601807702, 332526858, 1693310695, 136360183, 1189114632, 506273277, 397438002,
    620771032, 676183860, 1747529440, 909035644, 142389739, 1991534368, 272707803, 1905681287,
    1210958911, 596176677, 1380009185, 1153270606, 1150188963, 1067903737, 1020928348, 978324723,
    962376754, 1368724127, 1133797255, 1367747748, 1458212849, 537933020, 1295159285, 2104731913,
    1647629177, 1691336604, 922114202, 170715530, 1608833393, 62657989, 1140989235, 381784875,
    928003604, 449509021, 1057208185, 1239816707, 525522922, 476962140, 102897870, 132620570,
    419788154, 2095057491, 1240747817, 1271689397, 973007445, 1380110056, 1021668229, 12064370,
    1186917580, 1017163094, 597085928, 2018803520, 1795688603, 1722115921, 2015264326, 506263638,
    1002517905, 1229603330, 1376031959, 763839898, 1970623926, 1109937345, 524780807, 1976131071,
    905940439, 1313298413, 772929676, 1578848328, 1108240025, 577439381, 1293318580, 1512203375,
    371003697, 308046041, 320070446, 1252546340, 568098497, 1341794814, 1922466690, 480833267,
    1060838440, 969079660, 1836468543, 2049091118, 2023431210, 383830867, 2112679659, 231203270,
    1551220541, 1377927987, 275637462, 2110145570, 1700335604, 738389040, 1688841319, 1506456297,
    1243730675, 258043479, 599084776, 41093802, 792486733, 1897397356, 28077829, 1520357900,
    361516586, 1119263216, 209458355, 45979201, 363681532, 477245280, 2107748241, 601938891,
    244572459, 1689418013, 1141711990, 1485744349, 1181066840, 1950794776, 410494836, 1445347454,
    2137242950, 852679640, 1014566730, 1999335993, 1871390758, 1736439305, 231222289, 603972436,
    783045542, 370384393, 184356284, 709706295, 1453549767, 591603172, 768512391, 854125182,
];

/// Default average chunk size in bytes for Data-Code generation.
pub(crate) const DATA_AVG_CHUNK_SIZE: u32 = 1024;

/// Calculate CDC parameters from target average chunk size.
///
/// Returns `(min_size, max_size, center_size, mask_s, mask_l)` where:
/// - `min_size`: minimum chunk size (avg/4)
/// - `max_size`: maximum chunk size (avg*8)
/// - `center_size`: threshold between strict and relaxed mask phases
/// - `mask_s`: strict mask for early boundary detection (harder to match)
/// - `mask_l`: relaxed mask for late boundary detection (easier to match)
pub(crate) fn alg_cdc_params(avg_size: u32) -> (usize, usize, usize, u32, u32) {
    let min_size = (avg_size / 4) as usize;
    let max_size = (avg_size * 8) as usize;
    let offset = min_size + min_size.div_ceil(2);
    let center_size = avg_size as usize - offset;
    let bits = (avg_size as f64).log2().round() as u32;
    let mask_s = (1u32 << (bits + 1)) - 1;
    let mask_l = (1u32 << (bits - 1)) - 1;
    (min_size, max_size, center_size, mask_s, mask_l)
}

/// Find the CDC cut point offset within a buffer.
///
/// Uses a gear rolling hash to scan the buffer in two phases:
/// 1. From `mi` to `cs`: strict phase using `mask_s` (harder to match)
/// 2. From `cs` to `ma`: relaxed phase using `mask_l` (easier to match)
///
/// Returns the byte offset of the cut point. For buffers smaller than
/// `mi`, returns the buffer length (entire buffer is one chunk).
pub(crate) fn alg_cdc_offset(
    buffer: &[u8],
    mi: usize,
    ma: usize,
    cs: usize,
    mask_s: u32,
    mask_l: u32,
) -> usize {
    let mut pattern: u32 = 0;
    let size = buffer.len();
    let mut i = mi.min(size);
    let mut barrier = cs.min(size);

    // Phase 1: strict mask (harder to match, produces larger chunks)
    while i < barrier {
        pattern = (pattern >> 1).wrapping_add(CDC_GEAR[buffer[i] as usize]);
        if pattern & mask_s == 0 {
            return i + 1;
        }
        i += 1;
    }

    // Phase 2: relaxed mask (easier to match, prevents overly large chunks)
    barrier = ma.min(size);
    while i < barrier {
        pattern = (pattern >> 1).wrapping_add(CDC_GEAR[buffer[i] as usize]);
        if pattern & mask_l == 0 {
            return i + 1;
        }
        i += 1;
    }

    i
}

/// Split data into content-defined chunks.
///
/// Uses the gear rolling hash CDC algorithm to find content-dependent
/// boundaries. Returns at least one chunk (empty slice for empty input).
/// When `utf32` is true, aligns cut points to 4-byte boundaries.
pub fn alg_cdc_chunks(data: &[u8], utf32: bool, avg_chunk_size: u32) -> Vec<&[u8]> {
    if data.is_empty() {
        return vec![&data[0..0]];
    }

    let (mi, ma, cs, mask_s, mask_l) = alg_cdc_params(avg_chunk_size);
    let mut chunks = Vec::new();
    let mut pos = 0;

    while pos < data.len() {
        let remaining = &data[pos..];
        let mut cut_point = alg_cdc_offset(remaining, mi, ma, cs, mask_s, mask_l);

        // Align cut points to 4-byte boundaries for UTF-32 encoded text
        if utf32 {
            cut_point -= cut_point % 4;
            if cut_point == 0 {
                cut_point = remaining.len().min(4);
            }
        }

        chunks.push(&data[pos..pos + cut_point]);
        pos += cut_point;
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_table_length() {
        assert_eq!(CDC_GEAR.len(), 256);
    }

    #[test]
    fn test_gear_table_first_last() {
        assert_eq!(CDC_GEAR[0], 1553318008);
        assert_eq!(CDC_GEAR[255], 854125182);
    }

    #[test]
    fn test_alg_cdc_params_default() {
        let (mi, ma, cs, mask_s, mask_l) = alg_cdc_params(1024);
        assert_eq!(mi, 256, "min_size");
        assert_eq!(ma, 8192, "max_size");
        assert_eq!(cs, 640, "center_size");
        assert_eq!(mask_s, 2047, "mask_s = (1 << 11) - 1");
        assert_eq!(mask_l, 511, "mask_l = (1 << 9) - 1");
    }

    #[test]
    fn test_alg_cdc_offset_small_buffer() {
        // Buffer smaller than min_size → returns buffer length
        let buf = vec![0u8; 100];
        let (mi, ma, cs, mask_s, mask_l) = alg_cdc_params(1024);
        let offset = alg_cdc_offset(&buf, mi, ma, cs, mask_s, mask_l);
        assert_eq!(offset, 100);
    }

    #[test]
    fn test_alg_cdc_offset_returns_at_most_max() {
        // Buffer larger than max_size → returns at most max_size
        let buf = vec![0xAA; 10000];
        let (mi, ma, cs, mask_s, mask_l) = alg_cdc_params(1024);
        let offset = alg_cdc_offset(&buf, mi, ma, cs, mask_s, mask_l);
        assert!(offset <= ma, "offset {offset} exceeds max_size {ma}");
        assert!(offset >= mi, "offset {offset} below min_size {mi}");
    }

    #[test]
    fn test_alg_cdc_chunks_empty() {
        let chunks = alg_cdc_chunks(b"", false, 1024);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].len(), 0);
    }

    #[test]
    fn test_alg_cdc_chunks_small_data() {
        // Data smaller than min_size → one chunk containing all data
        let data = vec![42u8; 100];
        let chunks = alg_cdc_chunks(&data, false, 1024);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].len(), 100);
    }

    #[test]
    fn test_alg_cdc_chunks_reassembly() {
        // Chunks must reassemble to original data
        let data: Vec<u8> = (0..=255).cycle().take(4096).collect();
        let chunks = alg_cdc_chunks(&data, false, 1024);
        let reassembled: Vec<u8> = chunks.iter().flat_map(|c| c.iter().copied()).collect();
        assert_eq!(reassembled, data);
    }

    #[test]
    fn test_alg_cdc_chunks_deterministic() {
        let data: Vec<u8> = (0..=255).cycle().take(4096).collect();
        let chunks1 = alg_cdc_chunks(&data, false, 1024);
        let chunks2 = alg_cdc_chunks(&data, false, 1024);
        assert_eq!(chunks1.len(), chunks2.len());
        for (a, b) in chunks1.iter().zip(chunks2.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_alg_cdc_chunks_multiple_chunks() {
        // Large data produces multiple chunks
        let data: Vec<u8> = (0..=255).cycle().take(8192).collect();
        let chunks = alg_cdc_chunks(&data, false, 1024);
        assert!(
            chunks.len() > 1,
            "expected multiple chunks, got {}",
            chunks.len()
        );
    }

    #[test]
    fn test_alg_cdc_chunks_utf32_small_buffer() {
        // 3 bytes with utf32=true must terminate and reassemble to original.
        // Primary regression test for the infinite loop bug where
        // cut_point % 4 == cut_point reduced cut_point to 0.
        let data = [0xAA, 0xBB, 0xCC];
        let chunks = alg_cdc_chunks(&data, true, 1024);
        assert!(!chunks.is_empty(), "must return at least one chunk");
        let reassembled: Vec<u8> = chunks.iter().flat_map(|c| c.iter().copied()).collect();
        assert_eq!(reassembled, data);
    }

    #[test]
    fn test_alg_cdc_chunks_utf32_exact_4_bytes() {
        // Exactly 4 bytes with utf32=true must return one 4-byte chunk.
        let data = [0x01, 0x02, 0x03, 0x04];
        let chunks = alg_cdc_chunks(&data, true, 1024);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], &data[..]);
    }

    #[test]
    fn test_alg_cdc_chunks_utf32_7_bytes() {
        // 7 bytes (4+3) with utf32=true verifies non-aligned tail handling.
        let data = [0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70];
        let chunks = alg_cdc_chunks(&data, true, 1024);
        assert!(!chunks.is_empty(), "must return at least one chunk");
        let reassembled: Vec<u8> = chunks.iter().flat_map(|c| c.iter().copied()).collect();
        assert_eq!(reassembled, data);
    }

    #[test]
    fn test_alg_cdc_chunks_utf32_reassembly() {
        // Larger 4-byte-aligned input with utf32=true must reassemble correctly,
        // and all chunks except possibly the last must be 4-byte aligned.
        let data: Vec<u8> = (0..=255).cycle().take(4096).collect();
        assert_eq!(data.len() % 4, 0, "test data must be 4-byte aligned");
        let chunks = alg_cdc_chunks(&data, true, 1024);
        let reassembled: Vec<u8> = chunks.iter().flat_map(|c| c.iter().copied()).collect();
        assert_eq!(reassembled, data);
        // All chunks except the last must be 4-byte aligned
        if chunks.len() > 1 {
            for (i, chunk) in chunks[..chunks.len() - 1].iter().enumerate() {
                assert_eq!(
                    chunk.len() % 4,
                    0,
                    "chunk {i} has length {} which is not 4-byte aligned",
                    chunk.len()
                );
            }
        }
    }

    #[test]
    fn test_alg_cdc_chunks_utf32_empty() {
        // Empty input with utf32=true must not loop and must return one empty chunk.
        let chunks = alg_cdc_chunks(b"", true, 1024);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].len(), 0);
    }
}
