//! MinHash algorithm for similarity-preserving hashing.
//!
//! Implements 64-dimensional MinHash with universal hash functions and
//! bit-interleaved compression, ported from `bio-codes/iscc-sum`.

/// Maximum 64-bit unsigned value (2^64 - 1).
const MAXI64: u64 = u64::MAX;

/// Mersenne prime 2^61 - 1.
const MPRIME: u64 = (1 << 61) - 1;

/// Maximum hash value mask (2^32 - 1).
const MAXH: u64 = (1 << 32) - 1;

/// MinHash permutation parameter A (64 universal hash function multipliers).
const MPA: [u64; 64] = [
    853146490016488653,
    1849332765672628665,
    1131688930666554379,
    1936485333668353377,
    890837126813020267,
    1988249303247129861,
    1408894512544874755,
    2140251716176616185,
    1755124413189049421,
    1355916793659431597,
    546586563822844083,
    497603761441203021,
    2000709902557454173,
    1057597903350092207,
    1576204252850880253,
    2078784234495706739,
    1022616668454863635,
    2150082342606334489,
    712341150087765807,
    1511757510246096559,
    1525853819909660573,
    1263771796138990131,
    1215963627200985263,
    590069150281426443,
    130824646248385081,
    962725325544728503,
    1702561325943522847,
    296074222435072629,
    490211158716051523,
    1255327197241792767,
    699458998727907367,
    32930168991409845,
    1985097843455124585,
    362027841570125531,
    1903252144040897835,
    900391845076405289,
    547470123601853551,
    1689373724032359119,
    845594231933442371,
    400331968021206285,
    174967108345233429,
    876513700861085019,
    505848386844809885,
    1920468508342256199,
    1292611725303815789,
    963317239501343903,
    1730880032297268007,
    284614929850059717,
    1185026248283273081,
    2167288823816985197,
    1214905315086686483,
    1555253098157439857,
    1048013650291539723,
    1238618594841147605,
    1213502582686547311,
    286300733803129311,
    1250358511639043529,
    407534797452854371,
    960869149538623787,
    1722699901467253087,
    1325704236119824319,
    196979859428570839,
    1669408735473259699,
    781336617016068757,
];

/// MinHash permutation parameter B (64 universal hash function addends).
const MPB: [u64; 64] = [
    1089606993368836715,
    726972438868274737,
    66204585613901025,
    1078410179646709132,
    1343470117098523467,
    698653121981343911,
    1248486536592473639,
    1447963007834012793,
    1034598851883537815,
    1474008409379745934,
    793773480906057541,
    980501101461882479,
    963941556313537655,
    233651787311327325,
    243905121737149907,
    570269452476776142,
    297633284648631084,
    1516796967247398557,
    1494795672066692649,
    1728741177365151059,
    1029197538967983408,
    1660732464170610344,
    1399769594446678069,
    506465470557005705,
    1279720146829545181,
    860096419955634036,
    411519685280832908,
    69539191273403207,
    1960489729088056217,
    605092075716397684,
    1017496016211653149,
    1304834535101321372,
    949013511180032347,
    1142776242221098779,
    576980004709031232,
    1071272177143100544,
    1494527341093835499,
    1073290814142727850,
    1285904200674942617,
    1277176606329477335,
    343788427301735585,
    2100915269685487331,
    1227711252031557450,
    18593166391963377,
    2101884148332688233,
    191808277534686888,
    2170124912729392024,
    918430470748151293,
    1831024560113812361,
    1951365515851067694,
    744352348473654499,
    1921518311887826722,
    2020165648600700886,
    1764930142256726985,
    1903893374912839788,
    1449378957774802122,
    1435825328374066345,
    833197549717762813,
    2238991044337210799,
    748955638857938366,
    1834583747494146901,
    222012292803592982,
    901238460725547841,
    1501611130776083278,
];

/// Compute a 64-dimensional MinHash from 32-bit integer features.
///
/// Uses 64 universal hash functions parameterized by MPA/MPB to compute
/// the minimum hash value for each dimension. Returns `MAXH` for each
/// dimension when features is empty.
fn minhash(features: &[u32]) -> Vec<u64> {
    MPA.iter()
        .zip(MPB.iter())
        .map(|(&a, &b)| {
            features
                .iter()
                .map(|&f| (((a.wrapping_mul(f as u64).wrapping_add(b)) & MAXI64) % MPRIME) & MAXH)
                .min()
                .unwrap_or(MAXH)
        })
        .collect()
}

/// Compress a MinHash vector by extracting and interleaving LSB bits.
///
/// Extracts `lsb` least-significant bits from each hash value. Iterates
/// bit positions 0..lsb, then over all hash values, packing bits MSB-first
/// into output bytes.
fn minhash_compress(mhash: &[u64], lsb: u32) -> Vec<u8> {
    let total_bits = mhash.len() * lsb as usize;
    let mut bits = vec![0u8; total_bits];
    let mut bit_index = 0;
    for bitpos in 0..lsb {
        for &h in mhash {
            bits[bit_index] = ((h >> bitpos) & 1) as u8;
            bit_index += 1;
        }
    }
    let total_bytes = total_bits.div_ceil(8);
    let mut out = vec![0u8; total_bytes];
    for (i, &bit) in bits.iter().enumerate() {
        if bit != 0 {
            let byte_index = i / 8;
            let bit_in_byte = 7 - (i % 8);
            out[byte_index] |= 1 << bit_in_byte;
        }
    }
    out
}

/// Compute a 256-bit MinHash digest from 32-bit integer features.
///
/// Calls `minhash` to get 64-dimensional hash vector, then compresses
/// with `lsb=4` to produce 32 bytes (64 × 4 bits = 256 bits).
pub(crate) fn alg_minhash_256(features: &[u32]) -> Vec<u8> {
    let mhash = minhash(features);
    minhash_compress(&mhash, 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minhash_empty_features() {
        let result = minhash(&[]);
        assert_eq!(result.len(), 64);
        assert!(result.iter().all(|&v| v == MAXH));
    }

    #[test]
    fn test_minhash_single_feature() {
        let result = minhash(&[42]);
        assert_eq!(result.len(), 64);
        // Each dimension should have a specific hash value
        assert!(result.iter().all(|&v| v <= MAXH));
    }

    #[test]
    fn test_minhash_compress_basic() {
        // With lsb=1, extracting 1 bit from each of 64 hash values → 8 bytes
        let mhash = vec![0u64; 64];
        let result = minhash_compress(&mhash, 1);
        assert_eq!(result.len(), 8);
        assert!(result.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_minhash_compress_all_ones() {
        // All hash values are 0xFFFFFFFF, lsb=4 → 32 bytes, all bits set
        let mhash = vec![MAXH; 64];
        let result = minhash_compress(&mhash, 4);
        assert_eq!(result.len(), 32);
        assert!(result.iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn test_alg_minhash_256_empty() {
        let result = alg_minhash_256(&[]);
        assert_eq!(result.len(), 32);
        // Empty features → all MAXH → all bits set in LSB 4 → all 0xFF
        assert!(result.iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn test_alg_minhash_256_single() {
        let result = alg_minhash_256(&[1]);
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_alg_minhash_256_deterministic() {
        let features = vec![100, 200, 300, 400, 500];
        let result1 = alg_minhash_256(&features);
        let result2 = alg_minhash_256(&features);
        assert_eq!(result1, result2);
    }
}
