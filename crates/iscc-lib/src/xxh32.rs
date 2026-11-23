//! XXH32 feature hash for Data-Code and Text-Code.
//!
//! Implements the 32-bit xxHash algorithm in safe Rust, following the xxHash
//! specification (<https://github.com/Cyan4973/xxHash/blob/dev/doc/xxhash_spec.md>).
//! `iscc-core` hashes content-defined chunks and text n-grams with XXH32 at
//! seed 0, so the digest is part of the ISCC conformance contract.

// Multiplicative constants of the XXH32 specification.
const PRIME32_1: u32 = 0x9E37_79B1;
const PRIME32_2: u32 = 0x85EB_CA77;
const PRIME32_3: u32 = 0xC2B2_AE3D;
const PRIME32_4: u32 = 0x27D4_EB2F;
const PRIME32_5: u32 = 0x1656_67B1;

/// Mix the little-endian word at `offset` of a stripe into one lane accumulator.
fn round(lane: u32, stripe: &[u8; 16], offset: usize) -> u32 {
    let word = u32::from_le_bytes([
        stripe[offset],
        stripe[offset + 1],
        stripe[offset + 2],
        stripe[offset + 3],
    ]);
    lane.wrapping_add(word.wrapping_mul(PRIME32_2))
        .rotate_left(13)
        .wrapping_mul(PRIME32_1)
}

/// Consume every complete 16-byte stripe and return the merged lanes with the unread tail.
fn consume_stripes(mut data: &[u8], seed: u32) -> (u32, &[u8]) {
    let mut v1 = seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2);
    let mut v2 = seed.wrapping_add(PRIME32_2);
    let mut v3 = seed;
    let mut v4 = seed.wrapping_sub(PRIME32_1);
    while let Some((stripe, rest)) = data.split_first_chunk::<16>() {
        v1 = round(v1, stripe, 0);
        v2 = round(v2, stripe, 4);
        v3 = round(v3, stripe, 8);
        v4 = round(v4, stripe, 12);
        data = rest;
    }
    let merged = v1
        .rotate_left(1)
        .wrapping_add(v2.rotate_left(7))
        .wrapping_add(v3.rotate_left(12))
        .wrapping_add(v4.rotate_left(18));
    (merged, data)
}

/// Spread every input bit over the whole 32-bit digest.
fn avalanche(mut acc: u32) -> u32 {
    acc ^= acc >> 15;
    acc = acc.wrapping_mul(PRIME32_2);
    acc ^= acc >> 13;
    acc = acc.wrapping_mul(PRIME32_3);
    acc ^ (acc >> 16)
}

/// Compute the XXH32 digest of `data` with `seed`.
pub(crate) fn xxh32(data: &[u8], seed: u32) -> u32 {
    let (mut acc, mut tail) = if data.len() >= 16 {
        consume_stripes(data, seed)
    } else {
        (seed.wrapping_add(PRIME32_5), data)
    };
    // The specification adds the input length modulo 2^32, so truncation is intended.
    acc = acc.wrapping_add(data.len() as u32);
    while let Some((word, rest)) = tail.split_first_chunk::<4>() {
        acc = acc
            .wrapping_add(u32::from_le_bytes(*word).wrapping_mul(PRIME32_3))
            .rotate_left(17)
            .wrapping_mul(PRIME32_4);
        tail = rest;
    }
    for &byte in tail {
        acc = acc
            .wrapping_add(u32::from(byte).wrapping_mul(PRIME32_5))
            .rotate_left(11)
            .wrapping_mul(PRIME32_1);
    }
    avalanche(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Return the first `len` bytes of the buffer the xxHash sanity check hashes.
    fn sanity_buffer(len: usize) -> Vec<u8> {
        let mut state = u64::from(PRIME32_1);
        (0..len)
            .map(|_| {
                let byte = (state >> 56) as u8;
                state = state.wrapping_mul(11_400_714_785_074_694_797);
                byte
            })
            .collect()
    }

    /// The XXH32 known answers of the xxHash sanity check: (length, seed, digest).
    #[test]
    fn test_xxh32_sanity_vectors() {
        let cases = [
            (0, 0, 0x02CC_5D05),
            (0, PRIME32_1, 0x36B7_8AE7),
            (1, 0, 0xCF65_B03E),
            (1, PRIME32_1, 0xB454_5AA4),
            (14, 0, 0x1208_E7E2),
            (14, PRIME32_1, 0x6AF1_D1FE),
            (222, 0, 0x5BD1_1DBD),
            (222, PRIME32_1, 0x5880_3C5F),
        ];
        for (len, seed, expected) in cases {
            let digest = xxh32(&sanity_buffer(len), seed);
            assert_eq!(digest, expected, "length {len}, seed {seed:#010x}");
        }
    }

    /// Every stripe count up to 16 with every tail length, for both sanity seeds.
    ///
    /// The expected value is the fold of the digests that two independent
    /// XXH32 implementations produce for the same inputs.
    #[test]
    fn test_xxh32_every_length_up_to_256() {
        let buffer = sanity_buffer(256);
        let mut folded = 0u32;
        for len in 0..=buffer.len() {
            for seed in [0, PRIME32_1] {
                let digest = xxh32(&buffer[..len], seed);
                folded = folded.wrapping_mul(31).wrapping_add(digest);
            }
        }
        assert_eq!(folded, 0x8B2B_990E);
    }
}
