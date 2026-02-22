//! Winner-Take-All Hash for video content fingerprinting.
//!
//! Implements the WTA-Hash algorithm used to produce similarity-preserving
//! hashes from MPEG-7 video frame signature vectors. Ported from the
//! `iscc-core` Python reference implementation (`wtahash.py`).

/// Permutation pairs for WTA-Hash video identification.
///
/// Each pair `(i, j)` indexes into a 380-element frame signature vector.
/// There are 256 pairs, supporting up to 256-bit output hashes.
const WTA_VIDEO_ID_PERMUTATIONS: [(usize, usize); 256] = [
    (292, 16),
    (219, 247),
    (295, 7),
    (105, 236),
    (251, 142),
    (334, 82),
    (17, 266),
    (250, 167),
    (38, 127),
    (184, 22),
    (215, 71),
    (308, 181),
    (195, 215),
    (145, 345),
    (134, 233),
    (89, 351),
    (155, 338),
    (185, 68),
    (233, 122),
    (225, 314),
    (192, 22),
    (298, 2),
    (120, 68),
    (99, 155),
    (274, 187),
    (122, 160),
    (341, 281),
    (230, 223),
    (240, 33),
    (334, 299),
    (166, 256),
    (80, 114),
    (211, 122),
    (18, 16),
    (254, 154),
    (310, 336),
    (36, 273),
    (41, 76),
    (196, 290),
    (191, 307),
    (76, 57),
    (49, 226),
    (85, 97),
    (178, 221),
    (212, 228),
    (125, 348),
    (140, 73),
    (316, 267),
    (91, 61),
    (136, 233),
    (154, 84),
    (338, 332),
    (89, 90),
    (245, 177),
    (167, 222),
    (114, 2),
    (278, 364),
    (22, 169),
    (163, 124),
    (40, 134),
    (229, 207),
    (298, 81),
    (199, 253),
    (344, 123),
    (376, 268),
    (139, 266),
    (247, 308),
    (255, 32),
    (85, 250),
    (345, 236),
    (205, 69),
    (215, 277),
    (299, 178),
    (275, 198),
    (250, 359),
    (84, 286),
    (225, 50),
    (212, 18),
    (1, 224),
    (274, 33),
    (25, 179),
    (47, 77),
    (55, 311),
    (232, 248),
    (71, 234),
    (223, 256),
    (228, 175),
    (371, 132),
    (357, 234),
    (216, 168),
    (332, 266),
    (267, 78),
    (378, 121),
    (165, 316),
    (16, 351),
    (100, 329),
    (301, 294),
    (321, 245),
    (12, 59),
    (151, 222),
    (126, 367),
    (148, 45),
    (23, 305),
    (281, 54),
    (146, 83),
    (343, 244),
    (72, 184),
    (304, 205),
    (98, 179),
    (93, 40),
    (302, 99),
    (218, 106),
    (49, 350),
    (157, 237),
    (355, 267),
    (369, 216),
    (229, 340),
    (284, 106),
    (136, 305),
    (186, 59),
    (3, 107),
    (217, 312),
    (209, 195),
    (333, 102),
    (35, 216),
    (45, 28),
    (178, 130),
    (184, 233),
    (217, 99),
    (321, 144),
    (238, 355),
    (150, 259),
    (255, 259),
    (134, 207),
    (226, 327),
    (174, 178),
    (371, 141),
    (247, 228),
    (244, 300),
    (245, 42),
    (353, 276),
    (368, 187),
    (369, 207),
    (86, 308),
    (212, 368),
    (288, 33),
    (304, 375),
    (156, 8),
    (302, 167),
    (333, 164),
    (37, 379),
    (203, 312),
    (191, 144),
    (310, 95),
    (123, 86),
    (157, 48),
    (284, 27),
    (112, 291),
    (37, 215),
    (98, 291),
    (292, 224),
    (303, 8),
    (200, 103),
    (173, 294),
    (97, 267),
    (288, 167),
    (24, 336),
    (354, 296),
    (25, 18),
    (289, 187),
    (203, 166),
    (307, 326),
    (87, 80),
    (60, 310),
    (176, 84),
    (15, 370),
    (274, 261),
    (178, 45),
    (203, 224),
    (295, 178),
    (30, 74),
    (227, 361),
    (241, 312),
    (231, 369),
    (226, 309),
    (89, 181),
    (216, 175),
    (286, 262),
    (234, 198),
    (99, 49),
    (221, 328),
    (78, 21),
    (95, 327),
    (324, 97),
    (291, 219),
    (184, 286),
    (192, 25),
    (309, 26),
    (84, 159),
    (114, 25),
    (296, 90),
    (51, 325),
    (289, 184),
    (95, 154),
    (21, 202),
    (306, 219),
    (39, 176),
    (99, 251),
    (83, 86),
    (207, 239),
    (168, 19),
    (88, 90),
    (297, 361),
    (215, 78),
    (262, 328),
    (356, 200),
    (48, 203),
    (60, 120),
    (54, 216),
    (369, 327),
    (159, 370),
    (148, 273),
    (332, 50),
    (176, 267),
    (317, 243),
    (311, 125),
    (272, 148),
    (6, 340),
    (80, 346),
    (197, 355),
    (117, 49),
    (261, 326),
    (242, 51),
    (295, 204),
    (298, 111),
    (147, 181),
    (35, 96),
    (318, 285),
    (271, 13),
    (38, 204),
    (16, 8),
    (334, 220),
    (173, 91),
    (372, 24),
    (183, 166),
    (320, 243),
    (87, 9),
    (105, 65),
    (148, 103),
    (197, 314),
    (279, 299),
    (304, 214),
    (282, 15),
    (64, 2),
    (63, 14),
    (28, 351),
];

/// Compute a WTA-Hash from an integer feature vector.
///
/// For each permutation pair `(i, j)`, compares `vec[i]` and `vec[j]`:
/// - If `vec[i] >= vec[j]`, output bit is 0
/// - Otherwise, output bit is 1
///
/// Bits are packed MSB-first into bytes, matching Python bitarray default
/// endianness. Returns `bits/8` bytes.
pub fn alg_wtahash(vec: &[i64], bits: u32) -> Vec<u8> {
    let n_bytes = (bits / 8) as usize;
    let mut result = vec![0u8; n_bytes];

    for (bit_idx, &(i, j)) in WTA_VIDEO_ID_PERMUTATIONS
        .iter()
        .enumerate()
        .take(bits as usize)
    {
        if vec[i] < vec[j] {
            // Output bit 1 when vec[i] < vec[j]
            let byte_idx = bit_idx / 8;
            result[byte_idx] |= 1 << (7 - (bit_idx % 8));
        }
        // When vec[i] >= vec[j], bit is 0 (already initialized)
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alg_wtahash_all_zeros() {
        // All-zero input should produce all-zero output (all comparisons are >=)
        let vec = vec![0i64; 380];
        let result = alg_wtahash(&vec, 64);
        assert_eq!(result, vec![0u8; 8]);
    }

    #[test]
    fn test_alg_wtahash_range() {
        // Range 0..380 should produce deterministic non-zero output
        let vec: Vec<i64> = (0..380).collect();
        let result = alg_wtahash(&vec, 64);
        assert_eq!(result.len(), 8);
        // Not all zeros since some pairs will have i < j
        assert!(result.iter().any(|&b| b != 0));
    }

    #[test]
    fn test_alg_wtahash_256_bits() {
        // 256-bit output uses all 256 permutation pairs
        let vec: Vec<i64> = (0..380).collect();
        let result = alg_wtahash(&vec, 256);
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_permutation_table_length() {
        // The permutation table has exactly 256 entries
        assert_eq!(WTA_VIDEO_ID_PERMUTATIONS.len(), 256);
    }

    #[test]
    fn test_permutation_indices_in_range() {
        // All permutation indices must be < 380 (frame signature length)
        for &(i, j) in &WTA_VIDEO_ID_PERMUTATIONS {
            assert!(i < 380, "index i={i} out of range");
            assert!(j < 380, "index j={j} out of range");
        }
    }
}
