//! Conformance selftest for ISO 24138:2024 (ISCC).
//!
//! Runs gen functions and the ISCC-SEQ codec (`encode_seq` / `decode_seq`)
//! against vendored conformance vectors from `data.json` and reports
//! pass/fail. Code types whose feature flags are disabled are skipped (not
//! failed). With all features enabled, all 9 gen functions are tested; the
//! ISCC-SEQ sections run under every feature set. An application claiming ISCC
//! conformance MUST pass all tests in this suite.

#[cfg(feature = "meta-code")]
use crate::gen_meta_code_v0;
#[cfg(feature = "text-processing")]
use crate::gen_text_code_v0;
use crate::{
    IsccResult, decode_seq, encode_seq, gen_audio_code_v0, gen_data_code_v0, gen_image_code_v0,
    gen_instance_code_v0, gen_iscc_code_v0, gen_mixed_code_v0, gen_video_code_v0,
};
use serde_json::Value;
use std::fmt::Debug;

/// Embedded conformance test vectors (compile-time).
const TEST_DATA: &str = include_str!("../tests/data.json");

/// Run conformance tests against vendored test vectors.
///
/// Tests each `gen_*_v0` section (comparing the result's `.iscc` field) and the
/// `encode_seq` / `decode_seq` sections (comparing bytes / unit lists) against
/// the expected outputs. Code types whose feature flags are disabled are
/// skipped. Returns `true` if all enabled tests pass, `false` if any
/// mismatch or error occurs. Does not panic — logs failures via `eprintln!`
/// and continues through all test cases.
pub fn conformance_selftest() -> bool {
    let data: Value = match serde_json::from_str(TEST_DATA) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("FAILED: could not parse conformance data: {e}");
            return false;
        }
    };

    let mut passed = true;

    #[cfg(feature = "meta-code")]
    {
        passed &= run_meta_tests(&data);
    }
    #[cfg(feature = "text-processing")]
    {
        passed &= run_text_tests(&data);
    }
    passed &= run_image_tests(&data);
    passed &= run_audio_tests(&data);
    passed &= run_video_tests(&data);
    passed &= run_mixed_tests(&data);
    passed &= run_data_tests(&data);
    passed &= run_instance_tests(&data);
    passed &= run_iscc_tests(&data);
    passed &= run_encode_seq_tests(&data);
    passed &= run_decode_seq_tests(&data);

    passed
}

/// One parsed conformance case: the function's result and the expected output.
///
/// `None` means the case's inputs or expected outputs could not be parsed.
type Case<T> = Option<(IsccResult<T>, T)>;

/// Run every case of one `data.json` section and report each failure.
///
/// `run_case` parses a case, calls the function under test and returns its
/// result with the expected output. Returns `false` if the section is missing
/// or any case fails.
fn run_section<T, F>(data: &Value, func_name: &str, run_case: F) -> bool
where
    T: PartialEq + Debug,
    F: Fn(&Value) -> Case<T>,
{
    let Some(cases) = data[func_name].as_object() else {
        eprintln!("FAILED: {func_name} section missing from conformance data");
        return false;
    };

    let mut passed = true;
    for (tc_name, tc) in cases {
        passed &= report_case(func_name, tc_name, run_case(tc));
    }
    passed
}

/// Report the outcome of one conformance case; returns whether it passed.
fn report_case<T: PartialEq + Debug>(func_name: &str, tc_name: &str, case: Case<T>) -> bool {
    let Some((result, expected)) = case else {
        eprintln!("FAILED: {func_name}.{tc_name} — could not parse test inputs");
        return false;
    };
    match result {
        Ok(actual) if actual == expected => true,
        Ok(actual) => {
            eprintln!("FAILED: {func_name}.{tc_name} — expected {expected:?}, got {actual:?}");
            false
        }
        Err(e) => {
            eprintln!("FAILED: {func_name}.{tc_name} — error: {e}");
            false
        }
    }
}

/// Read the expected `.iscc` output of a `gen_*_v0` case.
fn expected_iscc(tc: &Value) -> Option<String> {
    tc["outputs"]["iscc"].as_str().map(String::from)
}

/// Run conformance tests for `gen_meta_code_v0`.
#[cfg(feature = "meta-code")]
fn run_meta_tests(data: &Value) -> bool {
    run_section(data, "gen_meta_code_v0", |tc| {
        let inputs = tc["inputs"].as_array()?;
        let name = inputs[0].as_str()?;
        let desc_str = inputs[1].as_str()?;
        let meta_val = &inputs[2];
        let bits = inputs[3].as_u64()? as u32;
        let expected = expected_iscc(tc)?;

        let meta_arg: Option<String> = match meta_val {
            Value::Null => None,
            Value::String(s) => Some(s.clone()),
            Value::Object(_) => serde_json::to_string(meta_val).ok(),
            _ => None,
        };
        let desc = (!desc_str.is_empty()).then_some(desc_str);

        let result = gen_meta_code_v0(name, desc, meta_arg.as_deref(), bits);
        Some((result.map(|r| r.iscc), expected))
    })
}

/// Run conformance tests for `gen_text_code_v0`.
#[cfg(feature = "text-processing")]
fn run_text_tests(data: &Value) -> bool {
    run_section(data, "gen_text_code_v0", |tc| {
        let inputs = tc["inputs"].as_array()?;
        let text = inputs[0].as_str()?;
        let bits = inputs[1].as_u64()? as u32;
        let expected = expected_iscc(tc)?;
        Some((gen_text_code_v0(text, bits).map(|r| r.iscc), expected))
    })
}

/// Run conformance tests for `gen_image_code_v0`.
fn run_image_tests(data: &Value) -> bool {
    run_section(data, "gen_image_code_v0", |tc| {
        let inputs = tc["inputs"].as_array()?;
        let pixels_json = inputs[0].as_array()?;
        let bits = inputs[1].as_u64()? as u32;
        let expected = expected_iscc(tc)?;

        let pixels: Vec<u8> = pixels_json
            .iter()
            .map(|v| v.as_u64().map(|n| n as u8))
            .collect::<Option<Vec<u8>>>()?;

        Some((gen_image_code_v0(&pixels, bits).map(|r| r.iscc), expected))
    })
}

/// Run conformance tests for `gen_audio_code_v0`.
fn run_audio_tests(data: &Value) -> bool {
    run_section(data, "gen_audio_code_v0", |tc| {
        let inputs = tc["inputs"].as_array()?;
        let cv_json = inputs[0].as_array()?;
        let bits = inputs[1].as_u64()? as u32;
        let expected = expected_iscc(tc)?;

        let cv: Vec<i32> = cv_json
            .iter()
            .map(|v| v.as_i64().map(|n| n as i32))
            .collect::<Option<Vec<i32>>>()?;

        Some((gen_audio_code_v0(&cv, bits).map(|r| r.iscc), expected))
    })
}

/// Run conformance tests for `gen_video_code_v0`.
fn run_video_tests(data: &Value) -> bool {
    run_section(data, "gen_video_code_v0", |tc| {
        let inputs = tc["inputs"].as_array()?;
        let frames_json = inputs[0].as_array()?;
        let bits = inputs[1].as_u64()? as u32;
        let expected = expected_iscc(tc)?;

        let frame_sigs: Vec<Vec<i32>> = frames_json
            .iter()
            .map(|frame| {
                frame
                    .as_array()?
                    .iter()
                    .map(|v| v.as_i64().map(|n| n as i32))
                    .collect::<Option<Vec<i32>>>()
            })
            .collect::<Option<_>>()?;

        Some((
            gen_video_code_v0(&frame_sigs, bits).map(|r| r.iscc),
            expected,
        ))
    })
}

/// Collect the string entries of a JSON array, skipping any non-string entry.
fn string_entries(values: &[Value]) -> Vec<&str> {
    values.iter().filter_map(Value::as_str).collect()
}

/// Run conformance tests for `gen_mixed_code_v0`.
fn run_mixed_tests(data: &Value) -> bool {
    run_section(data, "gen_mixed_code_v0", |tc| {
        let inputs = tc["inputs"].as_array()?;
        let codes = string_entries(inputs[0].as_array()?);
        let bits = inputs[1].as_u64()? as u32;
        let expected = expected_iscc(tc)?;
        Some((gen_mixed_code_v0(&codes, bits).map(|r| r.iscc), expected))
    })
}

/// Decode a `"stream:<hex>"` value into bytes.
fn decode_stream(s: &str) -> Option<Vec<u8>> {
    let hex_data = s.strip_prefix("stream:")?;
    hex::decode(hex_data).ok()
}

/// Run conformance tests for `gen_data_code_v0`.
fn run_data_tests(data: &Value) -> bool {
    run_section(data, "gen_data_code_v0", |tc| {
        let inputs = tc["inputs"].as_array()?;
        let input_bytes = decode_stream(inputs[0].as_str()?)?;
        let bits = inputs[1].as_u64()? as u32;
        let expected = expected_iscc(tc)?;
        Some((
            gen_data_code_v0(&input_bytes, bits).map(|r| r.iscc),
            expected,
        ))
    })
}

/// Run conformance tests for `gen_instance_code_v0`.
fn run_instance_tests(data: &Value) -> bool {
    run_section(data, "gen_instance_code_v0", |tc| {
        let inputs = tc["inputs"].as_array()?;
        let input_bytes = decode_stream(inputs[0].as_str()?)?;
        let bits = inputs[1].as_u64()? as u32;
        let expected = expected_iscc(tc)?;
        let result = gen_instance_code_v0(&input_bytes, bits);
        Some((result.map(|r| r.iscc), expected))
    })
}

/// Run conformance tests for `gen_iscc_code_v0`.
fn run_iscc_tests(data: &Value) -> bool {
    run_section(data, "gen_iscc_code_v0", |tc| {
        let codes = string_entries(tc["inputs"].as_array()?.first()?.as_array()?);
        let expected = expected_iscc(tc)?;
        // Conformance vectors use default (non-wide) mode
        Some((gen_iscc_code_v0(&codes, false).map(|r| r.iscc), expected))
    })
}

/// Decode a `"bytes:<hex>"` value into bytes.
fn decode_bytes(s: &str) -> Option<Vec<u8>> {
    let hex_data = s.strip_prefix("bytes:")?;
    hex::decode(hex_data).ok()
}

/// Run conformance tests for `encode_seq` (inputs `[[str…]]`, output `"bytes:<hex>"`).
///
/// Both sides are compared as lower-case hex so a mismatch reads as hex.
fn run_encode_seq_tests(data: &Value) -> bool {
    run_section(data, "encode_seq", |tc| {
        let units_json = tc["inputs"].as_array()?.first()?.as_array()?;
        let units: Vec<&str> = units_json
            .iter()
            .map(Value::as_str)
            .collect::<Option<_>>()?;
        let expected = hex::encode(decode_bytes(tc["outputs"].as_str()?)?);
        Some((encode_seq(&units).map(hex::encode), expected))
    })
}

/// Run conformance tests for `decode_seq` (inputs `["bytes:<hex>"]`, output `[str…]`).
fn run_decode_seq_tests(data: &Value) -> bool {
    run_section(data, "decode_seq", |tc| {
        let seq = decode_bytes(tc["inputs"].as_array()?.first()?.as_str()?)?;
        let expected: Vec<String> = tc["outputs"]
            .as_array()?
            .iter()
            .map(|v| v.as_str().map(String::from))
            .collect::<Option<_>>()?;
        Some((decode_seq(&seq), expected))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conformance_selftest_passes() {
        assert!(conformance_selftest());
    }

    #[test]
    fn test_seq_sections_run() {
        let data: serde_json::Value = serde_json::from_str(TEST_DATA).unwrap();
        assert_eq!(data["encode_seq"].as_object().unwrap().len(), 4);
        assert_eq!(data["decode_seq"].as_object().unwrap().len(), 4);
        assert!(run_encode_seq_tests(&data));
        assert!(run_decode_seq_tests(&data));
    }

    #[test]
    fn test_seq_sections_missing_fail() {
        let empty = serde_json::json!({});
        assert!(!run_encode_seq_tests(&empty));
        assert!(!run_decode_seq_tests(&empty));
    }

    #[test]
    fn test_seq_mismatch_and_parse_failures_fail() {
        // One case per section, so each failure arm (mismatch, error, parse) is proven alone.
        let encode_cases = [
            serde_json::json!({"inputs": [["ISCC:GAAAAAAAAA"]], "outputs": "bytes:00"}),
            serde_json::json!({"inputs": [[""]], "outputs": "bytes:00"}),
            serde_json::json!({"inputs": [[1]], "outputs": "bytes:00"}),
        ];
        for tc in encode_cases {
            let data = serde_json::json!({"encode_seq": {"case": tc}});
            assert!(!run_encode_seq_tests(&data), "{data}");
        }
        let decode_cases = [
            serde_json::json!({"inputs": ["bytes:300000000000"], "outputs": ["ISCC:X"]}),
            serde_json::json!({"inputs": ["bytes:"], "outputs": []}),
            serde_json::json!({"inputs": ["300000000000"], "outputs": []}),
        ];
        for tc in decode_cases {
            let data = serde_json::json!({"decode_seq": {"case": tc}});
            assert!(!run_decode_seq_tests(&data), "{data}");
        }
    }

    #[test]
    fn test_gen_sections_run() {
        let data: Value = serde_json::from_str(TEST_DATA).unwrap();
        assert!(run_image_tests(&data));
        assert!(run_audio_tests(&data));
        assert!(run_video_tests(&data));
        assert!(run_mixed_tests(&data));
        assert!(run_data_tests(&data));
        assert!(run_instance_tests(&data));
        assert!(run_iscc_tests(&data));
    }

    #[test]
    fn test_gen_section_missing_fails() {
        let empty = serde_json::json!({});
        assert!(!run_data_tests(&empty));
        assert!(!run_iscc_tests(&empty));
    }

    #[test]
    fn test_gen_section_positive_case() {
        // Guards the failure-arm test: the same shape with a correct output passes.
        let iscc = gen_data_code_v0(&[], 64).unwrap().iscc;
        let tc = serde_json::json!({"inputs": ["stream:", 64], "outputs": {"iscc": iscc}});
        assert!(run_data_tests(
            &serde_json::json!({"gen_data_code_v0": {"case": tc}})
        ));
    }

    #[test]
    fn test_gen_section_failure_arms_fail() {
        // One case per call, so each failure arm (mismatch, error, parse) is proven alone.
        assert!(
            gen_data_code_v0(&[], 7).is_err(),
            "bits 7 must reach the error arm"
        );
        let cases = [
            serde_json::json!({"inputs": ["stream:", 64], "outputs": {"iscc": "ISCC:X"}}),
            serde_json::json!({"inputs": ["stream:", 7], "outputs": {"iscc": "ISCC:X"}}),
            serde_json::json!({"inputs": ["00ff", 64], "outputs": {"iscc": "ISCC:X"}}),
            serde_json::json!({"inputs": ["stream:", 64], "outputs": {}}),
        ];
        for tc in cases {
            let data = serde_json::json!({"gen_data_code_v0": {"case": tc}});
            assert!(!run_data_tests(&data), "{data}");
        }
    }
}
