//! ISCC codec: type enums, header encoding/decoding, base32, and component encoding.
//!
//! Provides the foundational encoding primitives that all `gen_*_v0` functions
//! depend on to produce ISCC-encoded output strings. This is a Tier 2 module —
//! available to Rust consumers but not exposed through FFI bindings.

use crate::{IsccError, IsccResult};

// ---- Type Enums ----

/// ISCC MainType identifier.
///
/// Integer values match the `iscc-core` Python reference (MT enum).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MainType {
    Meta = 0,
    Semantic = 1,
    Content = 2,
    Data = 3,
    Instance = 4,
    Iscc = 5,
    Id = 6,
    Flake = 7,
}

impl TryFrom<u8> for MainType {
    type Error = IsccError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Meta),
            1 => Ok(Self::Semantic),
            2 => Ok(Self::Content),
            3 => Ok(Self::Data),
            4 => Ok(Self::Instance),
            5 => Ok(Self::Iscc),
            6 => Ok(Self::Id),
            7 => Ok(Self::Flake),
            _ => Err(IsccError::InvalidInput(format!(
                "invalid MainType: {value}"
            ))),
        }
    }
}

/// ISCC SubType identifier.
///
/// A unified enum covering all subtype contexts (ST, ST_CC, ST_ISCC).
/// The interpretation depends on the MainType context. Integer values
/// match the `iscc-core` Python reference.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubType {
    /// No specific subtype (general) / Text content (ST_CC context).
    None = 0,
    /// Image content.
    Image = 1,
    /// Audio content.
    Audio = 2,
    /// Video content.
    Video = 3,
    /// Mixed content.
    Mixed = 4,
    /// ISCC composite summary (only 2 mandatory units, no optional).
    Sum = 5,
    /// ISCC no specific content type (3+ units, mixed subtypes).
    IsccNone = 6,
    /// ISCC wide mode (256-bit Data+Instance composite).
    Wide = 7,
}

impl SubType {
    /// Alias for `None` (value 0) in Content-Code / Semantic-Code context.
    pub const TEXT: Self = Self::None;
}

impl TryFrom<u8> for SubType {
    type Error = IsccError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Image),
            2 => Ok(Self::Audio),
            3 => Ok(Self::Video),
            4 => Ok(Self::Mixed),
            5 => Ok(Self::Sum),
            6 => Ok(Self::IsccNone),
            7 => Ok(Self::Wide),
            _ => Err(IsccError::InvalidInput(format!("invalid SubType: {value}"))),
        }
    }
}

/// ISCC version identifier.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    V0 = 0,
}

impl TryFrom<u8> for Version {
    type Error = IsccError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::V0),
            _ => Err(IsccError::InvalidInput(format!("invalid Version: {value}"))),
        }
    }
}

// ---- Bit Manipulation Helpers ----

/// Convert a bit slice (big-endian, MSB first) to a u32.
fn bits_to_u32(bits: &[bool]) -> u32 {
    bits.iter().fold(0u32, |acc, &b| (acc << 1) | u32::from(b))
}

/// Convert bytes to a bit vector (big-endian, MSB first).
fn bytes_to_bits(bytes: &[u8]) -> Vec<bool> {
    bytes
        .iter()
        .flat_map(|&byte| (0..8).rev().map(move |i| (byte >> i) & 1 == 1))
        .collect()
}

/// Convert a bit vector to bytes, padding with zero bits on the right.
fn bits_to_bytes(bits: &[bool]) -> Vec<u8> {
    bits.chunks(8)
        .map(|chunk| {
            chunk.iter().enumerate().fold(
                0u8,
                |byte, (i, &bit)| if bit { byte | (1 << (7 - i)) } else { byte },
            )
        })
        .collect()
}

// ---- Varnibble Encoding ----

/// Encode an integer as a variable-length nibble (varnibble) bit sequence.
///
/// Encoding scheme:
/// - `0xxx` (4 bits, 1 nibble): values 0–7
/// - `10xxxxxx` (8 bits, 2 nibbles): values 8–71
/// - `110xxxxxxxxx` (12 bits, 3 nibbles): values 72–583
/// - `1110xxxxxxxxxxxx` (16 bits, 4 nibbles): values 584–4679
fn encode_varnibble(value: u32) -> IsccResult<Vec<bool>> {
    match value {
        0..=7 => {
            // 4 bits: value fits directly (leading 0 implicit in 4-bit encoding)
            Ok((0..4).rev().map(|i| (value >> i) & 1 == 1).collect())
        }
        8..=71 => {
            // 8 bits: prefix 10 + 6 data bits for (value - 8)
            let v = value - 8;
            let mut bits = vec![true, false];
            bits.extend((0..6).rev().map(|i| (v >> i) & 1 == 1));
            Ok(bits)
        }
        72..=583 => {
            // 12 bits: prefix 110 + 9 data bits for (value - 72)
            let v = value - 72;
            let mut bits = vec![true, true, false];
            bits.extend((0..9).rev().map(|i| (v >> i) & 1 == 1));
            Ok(bits)
        }
        584..=4679 => {
            // 16 bits: prefix 1110 + 12 data bits for (value - 584)
            let v = value - 584;
            let mut bits = vec![true, true, true, false];
            bits.extend((0..12).rev().map(|i| (v >> i) & 1 == 1));
            Ok(bits)
        }
        _ => Err(IsccError::InvalidInput(format!(
            "varnibble value out of range (0-4679): {value}"
        ))),
    }
}

/// Decode the first varnibble from a bit slice.
///
/// Returns the decoded integer and the number of bits consumed.
fn decode_varnibble(bits: &[bool]) -> IsccResult<(u32, usize)> {
    if bits.len() < 4 {
        return Err(IsccError::InvalidInput(
            "insufficient bits for varnibble".into(),
        ));
    }

    if !bits[0] {
        // 0xxx — 4 bits, values 0–7
        Ok((bits_to_u32(&bits[..4]), 4))
    } else if bits.len() >= 8 && !bits[1] {
        // 10xxxxxx — 8 bits, values 8–71
        Ok((bits_to_u32(&bits[2..8]) + 8, 8))
    } else if bits.len() >= 12 && !bits[2] {
        // 110xxxxxxxxx — 12 bits, values 72–583
        Ok((bits_to_u32(&bits[3..12]) + 72, 12))
    } else if bits.len() >= 16 && !bits[3] {
        // 1110xxxxxxxxxxxx — 16 bits, values 584–4679
        Ok((bits_to_u32(&bits[4..16]) + 584, 16))
    } else {
        Err(IsccError::InvalidInput(
            "invalid varnibble prefix or insufficient bits".into(),
        ))
    }
}

// ---- Header Encoding ----

/// Encode ISCC header fields into bytes.
///
/// Concatenates varnibble-encoded MainType, SubType, Version, and length,
/// then pads to byte boundary with zero bits on the right.
/// Result is 2 bytes minimum (typical case), up to 8 bytes maximum.
pub fn encode_header(
    mtype: MainType,
    stype: SubType,
    version: Version,
    length: u32,
) -> IsccResult<Vec<u8>> {
    let mut bits = Vec::new();
    bits.extend(encode_varnibble(mtype as u32)?);
    bits.extend(encode_varnibble(stype as u32)?);
    bits.extend(encode_varnibble(version as u32)?);
    bits.extend(encode_varnibble(length)?);

    // Pad to byte boundary with zero bits (equivalent to bitarray.fill())
    let remainder = bits.len() % 8;
    if remainder != 0 {
        bits.resize(bits.len() + (8 - remainder), false);
    }

    Ok(bits_to_bytes(&bits))
}

/// Decode ISCC header from bytes.
///
/// Returns `(MainType, SubType, Version, length, tail_bytes)` where
/// `tail_bytes` contains any remaining data after the header.
pub fn decode_header(data: &[u8]) -> IsccResult<(MainType, SubType, Version, u32, Vec<u8>)> {
    let bits = bytes_to_bits(data);
    let mut pos = 0;

    let (mtype_val, consumed) = decode_varnibble(&bits[pos..])?;
    pos += consumed;

    let (stype_val, consumed) = decode_varnibble(&bits[pos..])?;
    pos += consumed;

    let (version_val, consumed) = decode_varnibble(&bits[pos..])?;
    pos += consumed;

    let (length, consumed) = decode_varnibble(&bits[pos..])?;
    pos += consumed;

    // Strip 4-bit zero padding if header bits are not byte-aligned.
    // Since each varnibble is a multiple of 4 bits, misalignment is always 4 bits.
    if pos % 8 != 0 {
        let pad_end = pos + 4;
        if pad_end <= bits.len() && bits[pos..pad_end].iter().all(|&b| !b) {
            pos = pad_end;
        }
    }

    let tail = if pos < bits.len() {
        bits_to_bytes(&bits[pos..])
    } else {
        vec![]
    };

    let mtype = MainType::try_from(mtype_val as u8)?;
    let stype = SubType::try_from(stype_val as u8)?;
    let version = Version::try_from(version_val as u8)?;

    Ok((mtype, stype, version, length, tail))
}

// ---- Length Encoding ----

/// Encode bit length to header length field value.
///
/// Semantics depend on MainType:
/// - META/SEMANTIC/CONTENT/DATA/INSTANCE/FLAKE: `(bit_length / 32) - 1`
/// - ISCC: pass-through (0–7, unit composition flags)
/// - ID: `(bit_length - 64) / 8`
pub fn encode_length(mtype: MainType, length: u32) -> IsccResult<u32> {
    match mtype {
        MainType::Meta
        | MainType::Semantic
        | MainType::Content
        | MainType::Data
        | MainType::Instance
        | MainType::Flake => {
            if length >= 32 && length % 32 == 0 {
                Ok(length / 32 - 1)
            } else {
                Err(IsccError::InvalidInput(format!(
                    "invalid length {length} for {mtype:?} (must be multiple of 32, >= 32)"
                )))
            }
        }
        MainType::Iscc => {
            if length <= 7 {
                Ok(length)
            } else {
                Err(IsccError::InvalidInput(format!(
                    "invalid length {length} for ISCC (must be 0-7)"
                )))
            }
        }
        MainType::Id => {
            if (64..=96).contains(&length) && (length - 64) % 8 == 0 {
                Ok((length - 64) / 8)
            } else {
                Err(IsccError::InvalidInput(format!(
                    "invalid length {length} for ID (must be 64-96, step 8)"
                )))
            }
        }
    }
}

/// Decode header length field to actual bit length.
///
/// Inverse of `encode_length`. Returns the number of bits in the digest.
/// - META/SEMANTIC/CONTENT/DATA/INSTANCE/FLAKE: `(length + 1) * 32`
/// - ISCC + Wide: 256
/// - ISCC + other: `popcount(length) * 64 + 128`
/// - ID: `length * 8 + 64`
pub fn decode_length(mtype: MainType, length: u32, stype: SubType) -> u32 {
    match mtype {
        MainType::Meta
        | MainType::Semantic
        | MainType::Content
        | MainType::Data
        | MainType::Instance
        | MainType::Flake => (length + 1) * 32,
        MainType::Iscc => {
            if stype == SubType::Wide {
                256
            } else {
                length.count_ones() * 64 + 128
            }
        }
        MainType::Id => length * 8 + 64,
    }
}

// ---- Unit Encoding ----

/// Encode optional ISCC-UNIT MainTypes as a unit combination index (0–7).
///
/// Maps the optional units (Meta, Semantic, Content) present in a composite
/// ISCC-CODE to a bitfield index. Data and Instance are mandatory and must
/// not be included. The bitfield pattern is:
/// bit 0 = Content, bit 1 = Semantic, bit 2 = Meta.
pub fn encode_units(main_types: &[MainType]) -> IsccResult<u32> {
    let mut result = 0u32;
    for &mt in main_types {
        match mt {
            MainType::Content => result |= 1,
            MainType::Semantic => result |= 2,
            MainType::Meta => result |= 4,
            _ => {
                return Err(IsccError::InvalidInput(format!(
                    "{mt:?} is not a valid optional unit type"
                )));
            }
        }
    }
    Ok(result)
}

// ---- Base32 Encoding ----

/// Encode bytes as base32 (RFC 4648, uppercase, no padding).
pub fn encode_base32(data: &[u8]) -> String {
    data_encoding::BASE32_NOPAD.encode(data)
}

/// Decode base32 string to bytes (case-insensitive, no padding expected).
pub fn decode_base32(code: &str) -> IsccResult<Vec<u8>> {
    let upper = code.to_uppercase();
    data_encoding::BASE32_NOPAD
        .decode(upper.as_bytes())
        .map_err(|e| IsccError::InvalidInput(format!("base32 decode error: {e}")))
}

// ---- Base64 Encoding ----

/// Encode bytes as base64url (RFC 4648 §5, no padding).
pub fn encode_base64(data: &[u8]) -> String {
    data_encoding::BASE64URL_NOPAD.encode(data)
}

// ---- Component Encoding ----

/// Encode an ISCC-UNIT with header and body as a base32 string.
///
/// Produces the base32-encoded string (without "ISCC:" prefix). Callers
/// add the prefix when constructing the final ISCC string.
///
/// Note: ISCC-CODEs (MainType::Iscc) are not encoded via this function —
/// `gen_iscc_code_v0` constructs the composite header directly.
pub fn encode_component(
    mtype: MainType,
    stype: SubType,
    version: Version,
    bit_length: u32,
    digest: &[u8],
) -> IsccResult<String> {
    if mtype == MainType::Iscc {
        return Err(IsccError::InvalidInput(
            "ISCC MainType is not a unit; use gen_iscc_code_v0 instead".into(),
        ));
    }

    let encoded_length = encode_length(mtype, bit_length)?;
    let nbytes = (bit_length / 8) as usize;
    let header = encode_header(mtype, stype, version, encoded_length)?;
    let body = &digest[..nbytes.min(digest.len())];

    let mut component = header;
    component.extend_from_slice(body);

    Ok(encode_base32(&component))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Varnibble roundtrip tests ----

    #[test]
    fn test_varnibble_roundtrip() {
        let test_values = [0, 1, 7, 8, 71, 72, 583, 584, 4679];
        for &value in &test_values {
            let bits = encode_varnibble(value).unwrap();
            let (decoded, consumed) = decode_varnibble(&bits).unwrap();
            assert_eq!(decoded, value, "roundtrip failed for value {value}");
            assert_eq!(consumed, bits.len(), "consumed mismatch for value {value}");
        }
    }

    #[test]
    fn test_varnibble_bit_lengths() {
        // 0-7: 4 bits (1 nibble)
        assert_eq!(encode_varnibble(0).unwrap().len(), 4);
        assert_eq!(encode_varnibble(7).unwrap().len(), 4);
        // 8-71: 8 bits (2 nibbles)
        assert_eq!(encode_varnibble(8).unwrap().len(), 8);
        assert_eq!(encode_varnibble(71).unwrap().len(), 8);
        // 72-583: 12 bits (3 nibbles)
        assert_eq!(encode_varnibble(72).unwrap().len(), 12);
        assert_eq!(encode_varnibble(583).unwrap().len(), 12);
        // 584-4679: 16 bits (4 nibbles)
        assert_eq!(encode_varnibble(584).unwrap().len(), 16);
        assert_eq!(encode_varnibble(4679).unwrap().len(), 16);
    }

    #[test]
    fn test_varnibble_out_of_range() {
        assert!(encode_varnibble(4680).is_err());
    }

    #[test]
    fn test_varnibble_boundary_values() {
        // Verify exact bit patterns at boundaries
        let bits_0 = encode_varnibble(0).unwrap();
        assert_eq!(bits_0, vec![false, false, false, false]); // 0000

        let bits_7 = encode_varnibble(7).unwrap();
        assert_eq!(bits_7, vec![false, true, true, true]); // 0111

        let bits_8 = encode_varnibble(8).unwrap();
        assert_eq!(
            bits_8,
            vec![true, false, false, false, false, false, false, false]
        ); // 10 000000
    }

    // ---- Header encoding tests ----

    #[test]
    fn test_encode_header_meta_v0() {
        // encode_header(META=0, NONE=0, V0=0, length=1) → 2 bytes
        let header = encode_header(MainType::Meta, SubType::None, Version::V0, 1).unwrap();
        assert_eq!(header, vec![0x00, 0x01]);
    }

    #[test]
    fn test_encode_header_with_padding() {
        // encode_header(META=0, NONE=0, V0=0, length=8)
        // varnibble(0)=4b + varnibble(0)=4b + varnibble(0)=4b + varnibble(8)=8b = 20 bits
        // Padded to 24 bits = 3 bytes
        let header = encode_header(MainType::Meta, SubType::None, Version::V0, 8).unwrap();
        assert_eq!(header.len(), 3);
        // bits: 0000 0000 0000 10|000000 0000
        //       ^^^^ ^^^^ ^^^^ ^^^^^^^^ ^^^^(pad)
        assert_eq!(header, vec![0x00, 0x08, 0x00]);
    }

    #[test]
    fn test_encode_header_data_type() {
        // DATA=3, NONE=0, V0=0, length=1
        let header = encode_header(MainType::Data, SubType::None, Version::V0, 1).unwrap();
        // varnibble(3)=0011, varnibble(0)=0000, varnibble(0)=0000, varnibble(1)=0001
        // bits: 0011 0000 0000 0001
        assert_eq!(header, vec![0x30, 0x01]);
    }

    #[test]
    fn test_encode_header_instance_type() {
        // INSTANCE=4, NONE=0, V0=0, length=1
        let header = encode_header(MainType::Instance, SubType::None, Version::V0, 1).unwrap();
        // varnibble(4)=0100, varnibble(0)=0000, varnibble(0)=0000, varnibble(1)=0001
        // bits: 0100 0000 0000 0001
        assert_eq!(header, vec![0x40, 0x01]);
    }

    #[test]
    fn test_decode_header_roundtrip_all_main_types() {
        let main_types = [
            MainType::Meta,
            MainType::Semantic,
            MainType::Content,
            MainType::Data,
            MainType::Instance,
            MainType::Iscc,
            MainType::Id,
            MainType::Flake,
        ];

        for &mtype in &main_types {
            let header = encode_header(mtype, SubType::None, Version::V0, 1).unwrap();
            let (dec_mtype, dec_stype, dec_version, dec_length, tail) =
                decode_header(&header).unwrap();
            assert_eq!(dec_mtype, mtype, "MainType mismatch for {mtype:?}");
            assert_eq!(dec_stype, SubType::None);
            assert_eq!(dec_version, Version::V0);
            assert_eq!(dec_length, 1);
            assert!(tail.is_empty(), "unexpected tail for {mtype:?}");
        }
    }

    #[test]
    fn test_decode_header_with_tail() {
        // Simulate header + 8 bytes body
        let header = encode_header(MainType::Meta, SubType::None, Version::V0, 1).unwrap();
        let body = vec![0xAA, 0xBB, 0xCC, 0xDD, 0x11, 0x22, 0x33, 0x44];
        let mut data = header;
        data.extend_from_slice(&body);

        let (mtype, stype, version, length, tail) = decode_header(&data).unwrap();
        assert_eq!(mtype, MainType::Meta);
        assert_eq!(stype, SubType::None);
        assert_eq!(version, Version::V0);
        assert_eq!(length, 1);
        assert_eq!(tail, body);
    }

    #[test]
    fn test_decode_header_with_padding_and_tail() {
        // Header with padding (3 bytes) + body
        let header = encode_header(MainType::Meta, SubType::None, Version::V0, 8).unwrap();
        assert_eq!(header.len(), 3); // 20 bits padded to 24

        let body = vec![0xFF, 0xEE];
        let mut data = header;
        data.extend_from_slice(&body);

        let (mtype, _stype, _version, length, tail) = decode_header(&data).unwrap();
        assert_eq!(mtype, MainType::Meta);
        assert_eq!(length, 8);
        assert_eq!(tail, body);
    }

    #[test]
    fn test_decode_header_subtypes() {
        // Test with non-zero subtype
        let header = encode_header(MainType::Content, SubType::Image, Version::V0, 1).unwrap();
        let (mtype, stype, version, length, _tail) = decode_header(&header).unwrap();
        assert_eq!(mtype, MainType::Content);
        assert_eq!(stype, SubType::Image);
        assert_eq!(version, Version::V0);
        assert_eq!(length, 1);
    }

    // ---- Length encoding tests ----

    #[test]
    fn test_encode_length_standard_types() {
        // (bit_length / 32) - 1
        assert_eq!(encode_length(MainType::Meta, 32).unwrap(), 0);
        assert_eq!(encode_length(MainType::Meta, 64).unwrap(), 1);
        assert_eq!(encode_length(MainType::Meta, 96).unwrap(), 2);
        assert_eq!(encode_length(MainType::Meta, 128).unwrap(), 3);
        assert_eq!(encode_length(MainType::Meta, 256).unwrap(), 7);
        assert_eq!(encode_length(MainType::Data, 64).unwrap(), 1);
        assert_eq!(encode_length(MainType::Instance, 64).unwrap(), 1);
    }

    #[test]
    fn test_encode_length_iscc() {
        // Pass-through for ISCC (0-7)
        for i in 0..=7 {
            assert_eq!(encode_length(MainType::Iscc, i).unwrap(), i);
        }
        assert!(encode_length(MainType::Iscc, 8).is_err());
    }

    #[test]
    fn test_encode_length_id() {
        // (bit_length - 64) / 8
        assert_eq!(encode_length(MainType::Id, 64).unwrap(), 0);
        assert_eq!(encode_length(MainType::Id, 72).unwrap(), 1);
        assert_eq!(encode_length(MainType::Id, 80).unwrap(), 2);
        assert_eq!(encode_length(MainType::Id, 96).unwrap(), 4);
    }

    #[test]
    fn test_encode_length_invalid() {
        // Not a multiple of 32
        assert!(encode_length(MainType::Meta, 48).is_err());
        // Too small
        assert!(encode_length(MainType::Meta, 0).is_err());
        // ID out of range
        assert!(encode_length(MainType::Id, 63).is_err());
        assert!(encode_length(MainType::Id, 97).is_err());
    }

    #[test]
    fn test_decode_length_standard_types() {
        // (length + 1) * 32
        assert_eq!(decode_length(MainType::Meta, 0, SubType::None), 32);
        assert_eq!(decode_length(MainType::Meta, 1, SubType::None), 64);
        assert_eq!(decode_length(MainType::Meta, 7, SubType::None), 256);
        assert_eq!(decode_length(MainType::Data, 1, SubType::None), 64);
    }

    #[test]
    fn test_decode_length_iscc() {
        // Wide → 256
        assert_eq!(decode_length(MainType::Iscc, 0, SubType::Wide), 256);
        // Non-wide → popcount(length) * 64 + 128
        assert_eq!(decode_length(MainType::Iscc, 0, SubType::Sum), 128); // 0 optional units
        assert_eq!(decode_length(MainType::Iscc, 1, SubType::None), 192); // 1 optional unit
        assert_eq!(decode_length(MainType::Iscc, 3, SubType::None), 256); // 2 optional units
        assert_eq!(decode_length(MainType::Iscc, 7, SubType::None), 320); // 3 optional units
    }

    #[test]
    fn test_decode_length_id() {
        // length * 8 + 64
        assert_eq!(decode_length(MainType::Id, 0, SubType::None), 64);
        assert_eq!(decode_length(MainType::Id, 1, SubType::None), 72);
        assert_eq!(decode_length(MainType::Id, 4, SubType::None), 96);
    }

    #[test]
    fn test_encode_decode_length_roundtrip() {
        for &mtype in &[
            MainType::Meta,
            MainType::Data,
            MainType::Instance,
            MainType::Content,
        ] {
            for bit_length in (32..=256).step_by(32) {
                let encoded = encode_length(mtype, bit_length).unwrap();
                let decoded = decode_length(mtype, encoded, SubType::None);
                assert_eq!(
                    decoded, bit_length,
                    "roundtrip failed for {mtype:?} bit_length={bit_length}"
                );
            }
        }
    }

    // ---- Base32 tests ----

    #[test]
    fn test_base32_roundtrip() {
        let test_data: &[&[u8]] = &[
            &[0x00],
            &[0xFF],
            &[0x00, 0x01, 0x02, 0x03],
            &[0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE],
            &[0; 10],
            &[0xFF; 10],
        ];

        for data in test_data {
            let encoded = encode_base32(data);
            let decoded = decode_base32(&encoded).unwrap();
            assert_eq!(&decoded, data, "base32 roundtrip failed for {data:?}");
        }
    }

    #[test]
    fn test_base32_no_padding() {
        let encoded = encode_base32(&[0x00, 0x01]);
        assert!(!encoded.contains('='), "base32 should not contain padding");
    }

    #[test]
    fn test_base32_case_insensitive_decode() {
        let data = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let encoded = encode_base32(&data);
        let lower = encoded.to_lowercase();
        let decoded = decode_base32(&lower).unwrap();
        assert_eq!(decoded, data);
    }

    // ---- Base64 encoding tests ----

    #[test]
    fn test_encode_base64_empty() {
        assert_eq!(encode_base64(&[]), "");
    }

    #[test]
    fn test_encode_base64_known_value() {
        // Python: base64.urlsafe_b64encode(bytes([0,1,2,3])).decode().rstrip("=") == "AAECAw"
        assert_eq!(encode_base64(&[0, 1, 2, 3]), "AAECAw");
    }

    #[test]
    fn test_encode_base64_roundtrip() {
        let data: &[&[u8]] = &[
            &[0xFF],
            &[0xDE, 0xAD, 0xBE, 0xEF],
            &[0; 10],
            &[0xFF; 10],
            b"Hello World",
        ];
        for input in data {
            let encoded = encode_base64(input);
            let decoded = data_encoding::BASE64URL_NOPAD
                .decode(encoded.as_bytes())
                .unwrap();
            assert_eq!(&decoded, input, "base64 roundtrip failed for {input:?}");
        }
    }

    #[test]
    fn test_encode_base64_no_padding() {
        // Various lengths that would normally produce padding
        for len in 1..=10 {
            let data = vec![0xABu8; len];
            let encoded = encode_base64(&data);
            assert!(
                !encoded.contains('='),
                "base64 output must not contain padding for len={len}"
            );
        }
    }

    // ---- encode_component tests ----

    #[test]
    fn test_encode_component_meta_known_vector() {
        // gen_meta_code_v0("Hello World") → "ISCC:AAAWKLHFPV6OPKDG"
        // Decode the known output to extract the digest, then re-encode
        let known_code = "AAAWKLHFPV6OPKDG";
        let raw = decode_base32(known_code).unwrap();
        assert_eq!(raw.len(), 10); // 2 header bytes + 8 digest bytes

        // Verify header decodes correctly
        let (mtype, stype, version, length, tail) = decode_header(&raw).unwrap();
        assert_eq!(mtype, MainType::Meta);
        assert_eq!(stype, SubType::None);
        assert_eq!(version, Version::V0);
        assert_eq!(length, 1); // encode_length(META, 64) = 1
        assert_eq!(tail.len(), 8); // 64-bit digest

        // Re-encode from extracted digest
        let result =
            encode_component(MainType::Meta, SubType::None, Version::V0, 64, &tail).unwrap();
        assert_eq!(result, known_code);
    }

    #[test]
    fn test_encode_component_rejects_iscc_maintype() {
        assert!(
            encode_component(MainType::Iscc, SubType::Sum, Version::V0, 128, &[0; 16],).is_err()
        );
    }

    #[test]
    fn test_encode_component_data_type() {
        // Encode a Data-Code component and verify roundtrip
        let digest = [0xAA; 32];
        let code =
            encode_component(MainType::Data, SubType::None, Version::V0, 64, &digest).unwrap();

        // Decode and verify
        let raw = decode_base32(&code).unwrap();
        let (mtype, stype, version, length, tail) = decode_header(&raw).unwrap();
        assert_eq!(mtype, MainType::Data);
        assert_eq!(stype, SubType::None);
        assert_eq!(version, Version::V0);
        assert_eq!(length, 1); // encode_length(DATA, 64) = 1
        assert_eq!(tail, &digest[..8]); // 64 bits = 8 bytes
    }

    #[test]
    fn test_encode_component_content_image() {
        let digest = [0x55; 16];
        let code =
            encode_component(MainType::Content, SubType::Image, Version::V0, 128, &digest).unwrap();

        let raw = decode_base32(&code).unwrap();
        let (mtype, stype, _version, length, tail) = decode_header(&raw).unwrap();
        assert_eq!(mtype, MainType::Content);
        assert_eq!(stype, SubType::Image);
        assert_eq!(length, 3); // encode_length(CONTENT, 128) = 3
        assert_eq!(tail, &digest[..]); // 128 bits = 16 bytes
    }

    // ---- TryFrom tests ----

    #[test]
    fn test_maintype_try_from() {
        for v in 0..=7u8 {
            assert!(MainType::try_from(v).is_ok());
        }
        assert!(MainType::try_from(8).is_err());
    }

    #[test]
    fn test_subtype_try_from() {
        for v in 0..=7u8 {
            assert!(SubType::try_from(v).is_ok());
        }
        assert!(SubType::try_from(8).is_err());
    }

    #[test]
    fn test_version_try_from() {
        assert!(Version::try_from(0).is_ok());
        assert!(Version::try_from(1).is_err());
    }

    #[test]
    fn test_subtype_text_alias() {
        assert_eq!(SubType::TEXT, SubType::None);
        assert_eq!(SubType::TEXT as u8, 0);
    }

    // ---- Bit helper tests ----

    #[test]
    fn test_bits_to_u32() {
        assert_eq!(bits_to_u32(&[false, false, false, false]), 0);
        assert_eq!(bits_to_u32(&[false, true, true, true]), 7);
        assert_eq!(bits_to_u32(&[true, false, false, false]), 8);
        assert_eq!(bits_to_u32(&[true, true, true, true]), 15);
    }

    #[test]
    fn test_bytes_bits_roundtrip() {
        let data = vec![0x00, 0x01, 0xFF, 0xAB];
        let bits = bytes_to_bits(&data);
        assert_eq!(bits.len(), 32);
        let bytes = bits_to_bytes(&bits);
        assert_eq!(bytes, data);
    }

    // ---- encode_units tests ----

    #[test]
    fn test_encode_units_empty() {
        assert_eq!(encode_units(&[]).unwrap(), 0);
    }

    #[test]
    fn test_encode_units_content_only() {
        assert_eq!(encode_units(&[MainType::Content]).unwrap(), 1);
    }

    #[test]
    fn test_encode_units_semantic_only() {
        assert_eq!(encode_units(&[MainType::Semantic]).unwrap(), 2);
    }

    #[test]
    fn test_encode_units_semantic_content() {
        assert_eq!(
            encode_units(&[MainType::Semantic, MainType::Content]).unwrap(),
            3
        );
    }

    #[test]
    fn test_encode_units_meta_only() {
        assert_eq!(encode_units(&[MainType::Meta]).unwrap(), 4);
    }

    #[test]
    fn test_encode_units_meta_content() {
        assert_eq!(
            encode_units(&[MainType::Meta, MainType::Content]).unwrap(),
            5
        );
    }

    #[test]
    fn test_encode_units_meta_semantic() {
        assert_eq!(
            encode_units(&[MainType::Meta, MainType::Semantic]).unwrap(),
            6
        );
    }

    #[test]
    fn test_encode_units_all_optional() {
        assert_eq!(
            encode_units(&[MainType::Meta, MainType::Semantic, MainType::Content]).unwrap(),
            7
        );
    }

    #[test]
    fn test_encode_units_rejects_data() {
        assert!(encode_units(&[MainType::Data]).is_err());
    }

    #[test]
    fn test_encode_units_rejects_instance() {
        assert!(encode_units(&[MainType::Instance]).is_err());
    }

    #[test]
    fn test_encode_units_rejects_iscc() {
        assert!(encode_units(&[MainType::Iscc]).is_err());
    }
}
