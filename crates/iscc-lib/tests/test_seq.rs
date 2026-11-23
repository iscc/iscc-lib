//! Tests for the ISCC-SEQ codec (IEP-0020, C2PA soft binding values).
//!
//! Port of iscc-core 1.5.0 `tests/test_seq.py` (Python-only type/bytes-like
//! cases omitted), plus the IEP-0020 decoder vectors and guards that the typed
//! codec (`iscc_decode` / `iscc_decompose`) keeps rejecting what `decode_seq`
//! accepts.

use iscc_lib::codec::{
    MainType, SEQ_MAINTYPES, SubType, Version, decode_base32, encode_base32, encode_header,
    read_unit,
};
use iscc_lib::{decode_seq, encode_seq, gen_data_code_v0, gen_instance_code_v0};

const UNITS: [&str; 4] = [
    "ISCC:AADVPDD4R6733NMPFH3D57VTQ4KVH6VHL74XIWGUT37V5ZG56NV3NSI",
    "ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI",
    "ISCC:GADQLNA7GRZESMRF2J7NZPNWGI3II2ST5YUN5SS6GVQ2ZQGJXPPYDNI",
    "ISCC:IAD2KIVPJIWJZP3KQCESJL6SVT5APEZUPOJWM6HVTAXCF7OT3VFA4NY",
];

const HEX: &str = concat!(
    "0007578c7c8fbfbdb58f29f63efeb3871553faa75ff97458d49eff5ee4ddf36bb6c92007a88248c53a852c8d",
    "7e9c547047c9a36c4052b6acabef1e2a4ce15e586e0cb305300705b41f3472493225d27edcbdb63236846a53",
    "ee28deca5e3561acc0c9bbdf81b54007a522af4a2c9cbf6a808924afd2acfa0793347b936678f5982e22fdd3",
    "dd4a0e37",
);

const B64: &str = concat!(
    "AAdXjHyPv721jyn2Pv6zhxVT+qdf+XRY1J7/XuTd82u2ySAHqIJIxTqFLI1+nFRwR8mjbEBStqyr7x4qTOFeWG4M",
    "swUwBwW0HzRySTIl0n7cvbYyNoRqU+4o3speNWGswMm734G1QAelIq9KLJy/aoCJJK/SrPoHkzR7k2Z49ZguIv3T",
    "3UoONw==",
);

const TEXT: &str = concat!(
    "ISCC:AADVPDD4R6733NMPFH3D57VTQ4KVH6VHL74XIWGUT37V5ZG56NV3NSJAA6UIESGFHKCSZDL6TRKHAR6JUNWE",
    "AUVWVSV66HRKJTQV4WDOBSZQKMAHAW2B6NDSJEZCLUT63S63MMRWQRVFH3RI33FF4NLBVTAMTO67QG2UAB5FEKXUU",
    "LE4X5VIBCJEV7JKZ6QHSM2HXE3GPD2ZQLRC7XJ52SQOG4",
);

const SINGLE_HEX: &str = "2007a88248c53a852c8d7e9c547047c9a36c4052b6acabef1e2a4ce15e586e0cb305";

const UNITS_128: [&str; 2] = [
    "ISCC:GABQLNA7GRZESMRF2J7NZPNWGI3II",
    "ISCC:IAB2KIVPJIWJZP3KQCESJL6SVT5AO",
];
const HEX_128: &str = "300305b41f3472493225d27edcbdb63236844003a522af4a2c9cbf6a808924afd2acfa07";

const UNITS_MIXED: [&str; 3] = [
    "ISCC:EEAYYNMFG5XOELK2",
    "ISCC:EIBZ6B7T76PAAYP7H334X77754376",
    "ISCC:GADQLNA7GRZESMRF2J7NZPNWGI3II2ST5YUN5SS6GVQ2ZQGJXPPYDNI",
];
const HEX_MIXED: &str = concat!(
    "21018c3585376ee22d5a22039f07f3ff9e0061ff3ef7cbffffef37ff300705b41f3472493225d27edcbdb632",
    "36846a53ee28deca5e3561acc0c9bbdf81b5",
);

const ISCC_CODE_HEX: &str = "5005578c7c8fbfbdb58fa88248c53a852c8d05b41f3472493225a522af4a2c9cbf6a";
const ISCC_ID_HEX: &str = "60100000000000000000";

/// Multicodec prefix of ISCC (`0xcc01`), used to build multiformat inputs.
const MC_PREFIX: [u8; 2] = [0xcc, 0x01];

/// Content-Code header bytes for unknown SubType/Version values with Length 7,
/// as produced by iscc-core 1.5.0 `encode_header(MT.CONTENT, st, vs, 7)`.
const UNKNOWN_ST_HEADERS: [(u32, u32, &str); 14] = [
    (7, 0, "2707"),
    (7, 1, "2717"),
    (8, 0, "280070"),
    (8, 1, "280170"),
    (71, 0, "2bf070"),
    (71, 1, "2bf170"),
    (72, 0, "2c0007"),
    (72, 1, "2c0017"),
    (583, 0, "2dff07"),
    (583, 1, "2dff17"),
    (584, 0, "2e000070"),
    (584, 1, "2e000170"),
    (4679, 0, "2efff070"),
    (4679, 1, "2efff170"),
];

/// Decode a hex test constant into bytes.
fn hx(s: &str) -> Vec<u8> {
    hex::decode(s).unwrap()
}

/// Return the owned unit strings of a `&[&str]` constant.
fn owned(units: &[&str]) -> Vec<String> {
    units.iter().map(|u| u.to_string()).collect()
}

/// Borrow a list of owned strings as `&str` for `encode_seq`.
fn refs(units: &[String]) -> Vec<&str> {
    units.iter().map(String::as_str).collect()
}

/// Assert `result` is an error whose message contains `needle`.
fn assert_err_contains<T: std::fmt::Debug>(result: iscc_lib::IsccResult<T>, needle: &str) {
    match result {
        Ok(v) => panic!("expected error containing {needle:?}, got Ok({v:?})"),
        Err(e) => {
            let msg = e.to_string();
            assert!(msg.contains(needle), "error {msg:?} lacks {needle:?}");
        }
    }
}

/// Typed header for a SEQ-permitted MainType with SubType 0, Version 0.
fn header(mt: MainType, ln: u32) -> Vec<u8> {
    encode_header(mt, SubType::None, Version::V0, ln).unwrap()
}

// ---- Positive cases ----

#[test]
fn test_iep_example() {
    let value = encode_seq(&UNITS).unwrap();
    let query = data_encoding::BASE64.encode(&value);
    assert_eq!(decode_seq(&value).unwrap(), owned(&UNITS));
    let back = data_encoding::BASE64.decode(query.as_bytes()).unwrap();
    assert_eq!(decode_seq(&back).unwrap(), owned(&UNITS));
    let text = format!("ISCC:{}", encode_base32(&value));
    assert_eq!(text, TEXT);
}

#[test]
fn test_encode_seq_vector_1() {
    let value = encode_seq(&UNITS).unwrap();
    assert_eq!(value.len(), 136);
    assert_eq!(hex::encode(&value), HEX);
    assert_eq!(data_encoding::BASE64.encode(&value), B64);
    assert_eq!(encode_base32(&value).len(), 218);
}

#[test]
fn test_decode_seq_vector_1() {
    assert_eq!(decode_seq(&hx(HEX)).unwrap(), owned(&UNITS));
    let raw = data_encoding::BASE64.decode(B64.as_bytes()).unwrap();
    assert_eq!(decode_seq(&raw).unwrap(), owned(&UNITS));
}

#[test]
fn test_seq_single_unit() {
    let unit = UNITS[1];
    let value = encode_seq(&[unit]).unwrap();
    assert_eq!(
        value,
        decode_base32(unit.strip_prefix("ISCC:").unwrap()).unwrap()
    );
    assert_eq!(hex::encode(&value), SINGLE_HEX);
    assert_eq!(decode_seq(&value).unwrap(), vec![unit.to_string()]);
}

#[test]
fn test_seq_two_128_bit_units() {
    let value = encode_seq(&UNITS_128).unwrap();
    assert_eq!(hex::encode(&value), HEX_128);
    assert_eq!(decode_seq(&value).unwrap(), owned(&UNITS_128));
}

#[test]
fn test_seq_mixed_lengths_vector_2() {
    let value = encode_seq(&UNITS_MIXED).unwrap();
    assert_eq!(hex::encode(&value), HEX_MIXED);
    assert_eq!(decode_seq(&value).unwrap(), owned(&UNITS_MIXED));
}

#[test]
fn test_encode_seq_tolerant_input() {
    let bare: Vec<String> = UNITS
        .iter()
        .map(|u| u.strip_prefix("ISCC:").unwrap().to_string())
        .collect();
    let lower: Vec<String> = UNITS.iter().map(|u| u.to_lowercase()).collect();
    let bare_lower: Vec<String> = bare.iter().map(|u| u.to_lowercase()).collect();
    let hyphen: Vec<String> = UNITS
        .iter()
        .map(|u| format!("{}-{}", &u[..9], &u[9..]))
        .collect();
    for variant in [bare, lower, bare_lower, hyphen] {
        assert_eq!(hex::encode(encode_seq(&refs(&variant)).unwrap()), HEX);
    }
}

#[test]
fn test_encode_seq_multibase_like_content() {
    for unit in [
        "ISCC:FAAAAAAAAAAA",
        "iscc:faaa-aaaaaaaa",
        "FAAA-AAAAAAAA",
        "faaaaaaaaaaa",
        "faaa-aaaaaaaa",
    ] {
        assert_eq!(
            hex::encode(encode_seq(&[unit]).unwrap()),
            "28000000000000",
            "{unit}"
        );
    }
}

#[test]
fn test_encode_seq_multibase_like_meta() {
    assert_eq!(
        hex::encode(encode_seq(&["baaa-aaaaaaaa"]).unwrap()),
        "08000000000000"
    );
}

#[test]
fn test_seq_all_lengths_roundtrip() {
    for &mt in SEQ_MAINTYPES {
        for ln in 0..8u32 {
            let mut raw = header(mt, ln);
            raw.extend((0..4 * (ln + 1)).map(|b| b as u8));
            let units = decode_seq(&raw).unwrap();
            assert_eq!(units.len(), 1);
            assert_eq!(encode_seq(&refs(&units)).unwrap(), raw, "{mt:?} ln={ln}");
        }
    }
}

#[test]
fn test_seq_unknown_subtype_version_roundtrip() {
    for (st, vs, header_hex) in UNKNOWN_ST_HEADERS {
        let mut raw = hx(header_hex);
        raw.extend(0u8..32);
        let units = decode_seq(&raw).unwrap();
        assert_eq!(units.len(), 1, "st={st} vs={vs}");
        assert_eq!(encode_seq(&refs(&units)).unwrap(), raw, "st={st} vs={vs}");
        let bare = units[0].strip_prefix("ISCC:").unwrap();
        let hyphen = format!("{}-{}", &bare[..4], &bare[4..]);
        for variant in [
            bare.to_string(),
            bare.to_lowercase(),
            hyphen.clone(),
            hyphen.to_lowercase(),
        ] {
            assert_eq!(encode_seq(&[&variant]).unwrap(), raw, "{variant}");
        }
    }
}

#[test]
fn test_seq_unknown_subtype_canonical_strings() {
    // Exact unit strings from iscc-core 1.5.0 `decode_seq` (header + bytes 0..32).
    let cases = [
        (
            "2707",
            "ISCC:E4DQAAICAMCAKBQHBAEQUCYMBUHA6EARCIJRIFIWC4MBSGQ3DQOR4HY",
        ),
        (
            "280070",
            "ISCC:FAAHAAABAIBQIBIGA4EASCQLBQGQ4DYQCEJBGFAVCYLRQGI2DMOB2HQ7",
        ),
        (
            "2c0017",
            "ISCC:FQABOAABAIBQIBIGA4EASCQLBQGQ4DYQCEJBGFAVCYLRQGI2DMOB2HQ7",
        ),
        (
            "2efff170",
            "ISCC:F377C4AAAEBAGBAFAYDQQCIKBMGA2DQPCAIREEYUCULBOGAZDINRYHI6D4",
        ),
    ];
    for (header_hex, expected) in cases {
        let mut raw = hx(header_hex);
        raw.extend(0u8..32);
        assert_eq!(decode_seq(&raw).unwrap(), vec![expected.to_string()]);
    }
}

#[test]
fn test_decode_seq_undefined_subtype() {
    assert_eq!(
        decode_seq(&hx("270000000000")).unwrap(),
        vec!["ISCC:E4AAAAAAAA".to_string()]
    );
}

#[test]
fn test_seq_interop_with_iscc_decompose() {
    let value = encode_seq(&UNITS).unwrap();
    let decomposed = iscc_lib::iscc_decompose(&format!("ISCC:{}", encode_base32(&value))).unwrap();
    let expected: Vec<String> = UNITS
        .iter()
        .map(|u| u.strip_prefix("ISCC:").unwrap().to_string())
        .collect();
    assert_eq!(decomposed, expected);
}

#[test]
fn test_seq_repeated_keys_keep_order() {
    let data_64 = gen_data_code_v0(b"a", 64).unwrap().iscc;
    let data_256 = gen_data_code_v0(b"a", 256).unwrap().iscc;
    let inst_64 = gen_instance_code_v0(b"a", 64).unwrap().iscc;
    let inst_256 = gen_instance_code_v0(b"a", 256).unwrap().iscc;
    let units = vec![data_256, inst_64, data_64, inst_256];
    assert_eq!(
        decode_seq(&encode_seq(&refs(&units)).unwrap()).unwrap(),
        units
    );
}

// ---- decode_seq negative cases ----

#[test]
fn test_decode_seq_empty() {
    assert_err_contains(decode_seq(b""), "Empty");
}

#[test]
fn test_decode_seq_incomplete_header() {
    assert_err_contains(decode_seq(&hx("28")), "Malformed ISCC-HEADER at offset 0");
}

#[test]
fn test_decode_seq_nonzero_header_padding() {
    assert_eq!(
        decode_seq(&hx("28000000000000")).unwrap(),
        vec!["ISCC:FAAAAAAAAAAA".to_string()]
    );
    assert_err_contains(
        decode_seq(&hx("28000100000000")),
        "Non-canonical ISCC-HEADER at offset 0",
    );
}

#[test]
fn test_decode_seq_nonzero_header_padding_second_unit() {
    let raw = hx(&format!("{SINGLE_HEX}28000100000000"));
    let offset = hx(SINGLE_HEX).len();
    assert_err_contains(
        decode_seq(&raw),
        &format!("Non-canonical ISCC-HEADER at offset {offset}"),
    );
}

#[test]
fn test_decode_seq_length_above_7() {
    for ln in [8u32, 9] {
        let mut raw = header(MainType::Content, ln);
        raw.extend(vec![0u8; 4 * (ln as usize + 1)]);
        assert_err_contains(decode_seq(&raw), &format!("Unsupported Length field {ln}"));
    }
}

#[test]
fn test_decode_seq_truncated() {
    let raw = hx(HEX);
    assert_err_contains(
        decode_seq(&raw[..raw.len() - 1]),
        "Truncated ISCC-UNIT at offset 102",
    );
}

#[test]
fn test_decode_seq_trailing_byte() {
    let mut raw = hx(HEX);
    raw.push(0);
    assert_err_contains(decode_seq(&raw), "offset 136");
}

#[test]
fn test_decode_seq_rejects_maintype() {
    let mut flake = header(MainType::Flake, 1);
    flake.extend([0u8; 8]);
    for raw in [hx(ISCC_CODE_HEX), hx(ISCC_ID_HEX), flake] {
        assert_err_contains(decode_seq(&raw), "not permitted");
    }
}

// ---- encode_seq negative cases ----

#[test]
fn test_encode_seq_empty() {
    assert_err_contains(encode_seq(&[]), "at least one");
}

#[test]
fn test_encode_seq_empty_unit() {
    for unit in ["", "  "] {
        assert_err_contains(encode_seq(&[unit]), "must not be empty");
    }
}

#[test]
fn test_encode_seq_rejects_maintype() {
    for hex_code in [ISCC_CODE_HEX, ISCC_ID_HEX] {
        let unit = format!("ISCC:{}", encode_base32(&hx(hex_code)));
        assert_err_contains(encode_seq(&[&unit]), "not permitted");
    }
}

#[test]
fn test_encode_seq_rejects_flake() {
    // gen_flake_code_v0 is not ported; build a 64-bit FLAKE unit directly.
    let mut raw = header(MainType::Flake, 1);
    raw.extend([0x12u8; 8]);
    let flake = format!("ISCC:{}", encode_base32(&raw));
    assert_err_contains(encode_seq(&[&flake]), "not permitted");
}

#[test]
fn test_encode_seq_rejects_multiformat() {
    let raw = decode_base32(UNITS[0].strip_prefix("ISCC:").unwrap()).unwrap();
    let mut prefixed = MC_PREFIX.to_vec();
    prefixed.extend(&raw);
    let base16 = format!("f{}", hex::encode(&prefixed));
    let base32 = format!("b{}", encode_base32(&prefixed).to_lowercase());
    for mf in [base16, base32] {
        assert!(encode_seq(&[&mf]).is_err(), "{mf}");
    }
}

#[test]
fn test_encode_seq_trailing_data() {
    let unit = encode_base32(&[0u8; 8]);
    assert_err_contains(encode_seq(&[&unit]), "Trailing data");
}

#[test]
fn test_encode_seq_multiple_units_in_one_string() {
    assert_err_contains(encode_seq(&[TEXT]), "Trailing data");
}

// ---- read_unit ----

#[test]
fn test_read_unit_default_offset() {
    let raw = hx(SINGLE_HEX);
    assert_eq!(read_unit(&raw, 0).unwrap(), raw.len());
}

#[test]
fn test_read_unit_sequential_offsets() {
    let raw = hx(HEX);
    let mut offsets = vec![0usize];
    while *offsets.last().unwrap() < raw.len() {
        offsets.push(read_unit(&raw, *offsets.last().unwrap()).unwrap());
    }
    assert_eq!(offsets, vec![0, 34, 68, 102, 136]);
}

#[test]
fn test_read_unit_offsets_128() {
    let raw = hx(HEX_128);
    assert_eq!(read_unit(&raw, 0).unwrap(), 18);
    assert_eq!(read_unit(&raw, 18).unwrap(), raw.len());
}

#[test]
fn test_read_unit_truncated_at_offset() {
    let raw = hx(HEX_128);
    assert_err_contains(
        read_unit(&raw[..raw.len() - 1], 18),
        "Truncated ISCC-UNIT at offset 18",
    );
}

#[test]
fn test_read_unit_offset_out_of_range() {
    // Rust slices panic where Python is lenient: these must be errors, not panics.
    let raw = hx("300000000000");
    for offset in [raw.len(), raw.len() + 1, 100, usize::MAX] {
        assert_err_contains(
            read_unit(&raw, offset),
            &format!("Malformed ISCC-HEADER at offset {offset}"),
        );
    }
}

#[test]
fn test_seq_maintypes() {
    assert_eq!(
        SEQ_MAINTYPES,
        &[
            MainType::Meta,
            MainType::Semantic,
            MainType::Content,
            MainType::Data,
            MainType::Instance
        ]
    );
}

// ---- IEP-0020 decoder vectors (spec "Verified when") ----

#[test]
fn test_iep0020_accepts_subtype_7() {
    assert_eq!(
        decode_seq(&hx("270000000000")).unwrap(),
        vec!["ISCC:E4AAAAAAAA".to_string()]
    );
}

#[test]
fn test_iep0020_accepts_data_version_1_then_0() {
    assert_eq!(
        decode_seq(&hx("301000000000300000000000")).unwrap(),
        vec!["ISCC:GAIAAAAAAA".to_string(), "ISCC:GAAAAAAAAA".to_string()]
    );
}

#[test]
fn test_iep0020_accepts_four_unit_vector() {
    assert_eq!(decode_seq(&hx(HEX)).unwrap(), owned(&UNITS));
}

#[test]
fn test_iep0020_encode_multibase_like_lowercase() {
    assert_eq!(
        encode_seq(&["faaa-aaaaaaaa"]).unwrap(),
        hx("28000000000000")
    );
}

#[test]
fn test_iep0020_table2_rejects() {
    let long_length = format!("200800{}", "00".repeat(36));
    let cases: [(&str, &str); 7] = [
        ("", "Empty ISCC-SEQ"),
        ("28000100000000", "Non-canonical ISCC-HEADER at offset 0"),
        (&long_length, "Unsupported Length field 8 at offset 0"),
        ("3000000000", "Truncated ISCC-UNIT at offset 0"),
        ("30000000000000", "Malformed ISCC-HEADER at offset 6"),
        (ISCC_CODE_HEX, "MainType 5 is not permitted"),
        ("60100000000000000000", "MainType 6 is not permitted"),
    ];
    for (input, needle) in cases {
        assert_err_contains(decode_seq(&hx(input)), needle);
    }
}

#[test]
fn test_roundtrip_conformance_decode_vectors() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("data.json")).expect("valid data.json");
    let cases = data["decode_seq"].as_object().expect("decode_seq section");
    assert_eq!(cases.len(), 4);
    for (name, tc) in cases {
        let input = tc["inputs"][0].as_str().unwrap();
        let raw = hx(input.strip_prefix("bytes:").unwrap());
        let units = decode_seq(&raw).unwrap();
        assert_eq!(encode_seq(&refs(&units)).unwrap(), raw, "{name}");
    }
}

// ---- Typed codec keeps rejecting what decode_seq accepts ----

#[test]
fn test_typed_codec_rejects_data_version_1() {
    assert!(decode_seq(&hx("301000000000")).is_ok());
    assert!(iscc_lib::iscc_decode("ISCC:GAIAAAAAAA").is_err());
    assert!(iscc_lib::iscc_decompose("ISCC:GAIAAAAAAA").is_err());
}

#[test]
fn test_typed_codec_rejects_unknown_subtype() {
    let mut raw = hx("280070"); // Content, SubType 8, Version 0, Length 7
    raw.extend(0u8..32);
    let units = decode_seq(&raw).unwrap();
    assert!(iscc_lib::iscc_decode(&units[0]).is_err());
    assert!(iscc_lib::iscc_decompose(&units[0]).is_err());
}
