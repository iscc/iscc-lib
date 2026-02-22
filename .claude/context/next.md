# Next Work Package

## Step: Implement gen_video_code_v0 with WTA-Hash

## Goal

Implement the last remaining stub `gen_video_code_v0` using the WTA-Hash algorithm, completing all 9
gen functions and achieving full conformance with the iscc-core reference implementation.

## Scope

- **Create**: `crates/iscc-lib/src/wtahash.rs` -- WTA-Hash module with permutation table and
    algorithm
- **Modify**: `crates/iscc-lib/src/lib.rs` -- add mod wtahash, fix function signature, implement
    gen_video_code_v0 and soft_hash_video_v0, add conformance tests
- **Reference**: iscc-core wtahash.py (algorithm), iscc-core code_content_video.py (gen function),
    `crates/iscc-lib/tests/data.json` (conformance vectors)

## Implementation Notes

### Function signature change

The current stub is `gen_video_code_v0(_frame_sigs: &[Vec<u8>], _bits: u32)`. This is wrong -- frame
signatures are 380-element integer vectors (values can exceed 255, e.g., the range test goes 0-379).
Change to `&[Vec<i32>]`.

### wtahash.rs module

Create `pub(crate) mod wtahash` containing:

**WTA_VIDEO_ID_PERMUTATIONS** -- a static array of 256 (usize, usize) pairs. Full table:

```text
(292, 16), (219, 247), (295, 7), (105, 236), (251, 142), (334, 82), (17, 266), (250, 167),
(38, 127), (184, 22), (215, 71), (308, 181), (195, 215), (145, 345), (134, 233), (89, 351),
(155, 338), (185, 68), (233, 122), (225, 314), (192, 22), (298, 2), (120, 68), (99, 155),
(274, 187), (122, 160), (341, 281), (230, 223), (240, 33), (334, 299), (166, 256), (80, 114),
(211, 122), (18, 16), (254, 154), (310, 336), (36, 273), (41, 76), (196, 290), (191, 307),
(76, 57), (49, 226), (85, 97), (178, 221), (212, 228), (125, 348), (140, 73), (316, 267),
(91, 61), (136, 233), (154, 84), (338, 332), (89, 90), (245, 177), (167, 222), (114, 2),
(278, 364), (22, 169), (163, 124), (40, 134), (229, 207), (298, 81), (199, 253), (344, 123),
(376, 268), (139, 266), (247, 308), (255, 32), (85, 250), (345, 236), (205, 69), (215, 277),
(299, 178), (275, 198), (250, 359), (84, 286), (225, 50), (212, 18), (1, 224), (274, 33),
(25, 179), (47, 77), (55, 311), (232, 248), (71, 234), (223, 256), (228, 175), (371, 132),
(357, 234), (216, 168), (332, 266), (267, 78), (378, 121), (165, 316), (16, 351), (100, 329),
(301, 294), (321, 245), (12, 59), (151, 222), (126, 367), (148, 45), (23, 305), (281, 54),
(146, 83), (343, 244), (72, 184), (304, 205), (98, 179), (93, 40), (302, 99), (218, 106),
(49, 350), (157, 237), (355, 267), (369, 216), (229, 340), (284, 106), (136, 305), (186, 59),
(3, 107), (217, 312), (209, 195), (333, 102), (35, 216), (45, 28), (178, 130), (184, 233),
(217, 99), (321, 144), (238, 355), (150, 259), (255, 259), (134, 207), (226, 327), (174, 178),
(371, 141), (247, 228), (244, 300), (245, 42), (353, 276), (368, 187), (369, 207), (86, 308),
(212, 368), (288, 33), (304, 375), (156, 8), (302, 167), (333, 164), (37, 379), (203, 312),
(191, 144), (310, 95), (123, 86), (157, 48), (284, 27), (112, 291), (37, 215), (98, 291),
(292, 224), (303, 8), (200, 103), (173, 294), (97, 267), (288, 167), (24, 336), (354, 296),
(25, 18), (289, 187), (203, 166), (307, 326), (87, 80), (60, 310), (176, 84), (15, 370),
(274, 261), (178, 45), (203, 224), (295, 178), (30, 74), (227, 361), (241, 312), (231, 369),
(226, 309), (89, 181), (216, 175), (286, 262), (234, 198), (99, 49), (221, 328), (78, 21),
(95, 327), (324, 97), (291, 219), (184, 286), (192, 25), (309, 26), (84, 159), (114, 25),
(296, 90), (51, 325), (289, 184), (95, 154), (21, 202), (306, 219), (39, 176), (99, 251),
(83, 86), (207, 239), (168, 19), (88, 90), (297, 361), (215, 78), (262, 328), (356, 200),
(48, 203), (60, 120), (54, 216), (369, 327), (159, 370), (148, 273), (332, 50), (176, 267),
(317, 243), (311, 125), (272, 148), (6, 340), (80, 346), (197, 355), (117, 49), (261, 326),
(242, 51), (295, 204), (298, 111), (147, 181), (35, 96), (318, 285), (271, 13), (38, 204),
(16, 8), (334, 220), (173, 91), (372, 24), (183, 166), (320, 243), (87, 9), (105, 65),
(148, 103), (197, 314), (279, 299), (304, 214), (282, 15), (64, 2), (63, 14), (28, 351)
```

**alg_wtahash(vec: &[i64], bits: u32) -> Vec\<u8>** -- WTA-Hash algorithm:

- For each permutation pair (i, j) in WTA_VIDEO_ID_PERMUTATIONS (up to bits pairs): if vec[i] >=
    vec[j], output bit 0; otherwise output bit 1
- Pack bits into bytes MSB-first (matching Python bitarray default endianness)
- Return bits/8 bytes

### soft_hash_video_v0

Add to lib.rs as a private function with signature
`fn soft_hash_video_v0(frame_sigs: &[Vec<i32>], bits: u32) -> IsccResult<Vec<u8>>`

Algorithm:

1. Validate frame_sigs is non-empty
2. Deduplicate frame signatures using BTreeSet (Vec implements Ord)
3. Column-wise sum into Vec of i64 (i64 to avoid overflow)
4. Call wtahash::alg_wtahash on the sum vector and return the digest

Python uses set() on tuples for dedup. The column-wise sum is commutative, so dedup order is
irrelevant -- the sum is identical regardless of iteration order.

### gen_video_code_v0

Replace the NotImplemented stub:

1. Call soft_hash_video_v0(frame_sigs, bits)
2. Call codec::encode_component(MainType::Content, SubType::Video, 0, bits, &digest)
3. Return "ISCC:" plus the component

### Conformance tests

Parse from data.json under gen_video_code_v0. Three test vectors:

- test_0000_one_zero_frame_64: 1 frame of 380 zeros, bits=64, expect ISCC:EMAQAAAAAAAAAAAA
- test_0001_multiple_frames_128: 2 frames, bits=128, expect ISCC:EMBZEMGSDFIB4AHUEZSLJPJANMAAY
- test_0003_range_256: 1 frame of 0..379, bits=256, expect
    ISCC:EMDVFD4RIMPXYSWSNEZPYBZ2FDFMSPZBUMDRUFJPYKJFXWXNDUMQAYI

Remove the existing test_gen_video_code_v0_stub test and replace with proper conformance tests.

### WTA-Hash bit packing detail

Python bitarray packs MSB-first by default. Bit index 0 goes to the MSB of byte 0. Rust packing:
`byte |= (bit_value << (7 - (bit_index % 8)))`.

## Verification

- `cargo test -p iscc-lib` passes (all existing 128 tests + new video conformance tests, minus
    removed stub test)
- All 3 gen_video_code_v0 conformance vectors produce exact matching ISCC strings
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo fmt -p iscc-lib --check` clean
- No unsafe code
- The old stub test is replaced with proper conformance tests

## Done When

All 9 gen functions pass their conformance vectors, including the 3 new video tests, with clean
clippy and fmt.
