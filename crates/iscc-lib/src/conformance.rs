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
use serde_json::{Map, Value};
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
    let data = match test_data() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("FAILED: could not parse conformance data: {e}");
            return false;
        }
    };
    if let Err(e) = check_sections(&data) {
        eprintln!("FAILED: {e}");
        return false;
    }

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

/// Parse the embedded conformance vectors.
pub(crate) fn test_data() -> serde_json::Result<Value> {
    serde_json::from_str(TEST_DATA)
}

/// Check that every top-level key except `_metadata` maps to an object of case objects.
///
/// Returns a message naming the first key that does not.
fn check_sections(data: &Value) -> Result<(), String> {
    let sections = data
        .as_object()
        .ok_or("conformance data is not an object")?;
    let bad = sections.iter().find(|(key, section)| {
        *key != "_metadata"
            && !section
                .as_object()
                .is_some_and(|cases| cases.values().all(Value::is_object))
    });
    match bad {
        Some((key, _)) => Err(format!("section {key} is not an object of cases")),
        None => Ok(()),
    }
}

/// Return the cases of one `data.json` section, keyed by case name.
pub(crate) fn cases<'a>(data: &'a Value, section: &str) -> Option<&'a Map<String, Value>> {
    data[section].as_object()
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
    let Some(section) = cases(data, func_name) else {
        eprintln!("FAILED: {func_name} section missing from conformance data");
        return false;
    };

    let mut passed = true;
    for (tc_name, tc) in section {
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
pub(crate) fn expected_iscc(tc: &Value) -> Option<String> {
    tc["outputs"]["iscc"].as_str().map(String::from)
}

/// Return the positional `inputs` list of a case.
fn inputs(tc: &Value) -> Option<&[Value]> {
    tc["inputs"].as_array().map(Vec::as_slice)
}

/// Read the `bits` argument at position `index` of an inputs list.
///
/// Returns `None` unless the value is an integer in `u32` range.
fn bits_at(inputs: &[Value], index: usize) -> Option<u32> {
    u32::try_from(inputs.get(index)?.as_u64()?).ok()
}

/// Read a JSON array of integers as `i32` values.
///
/// Returns `None` if any entry is not an integer in `i32` range.
fn i32_entries(value: &Value) -> Option<Vec<i32>> {
    value
        .as_array()?
        .iter()
        .map(|v| i32::try_from(v.as_i64()?).ok())
        .collect()
}

/// Read a JSON array of strings; `None` if it is not an array or any entry is not a string.
fn string_entries(value: &Value) -> Option<Vec<&str>> {
    value.as_array()?.iter().map(Value::as_str).collect()
}

/// Decode a `"stream:<hex>"` value into bytes.
fn decode_stream(s: &str) -> Option<Vec<u8>> {
    let hex_data = s.strip_prefix("stream:")?;
    hex::decode(hex_data).ok()
}

/// Parsed inputs of a `gen_meta_code_v0` case.
#[cfg(feature = "meta-code")]
pub(crate) struct MetaInputs<'a> {
    /// The `name` argument.
    pub(crate) name: &'a str,
    /// The `description` argument (`None` for an empty string).
    pub(crate) description: Option<&'a str>,
    /// The `meta` argument: a data URL as given, or a JSON object serialized.
    pub(crate) meta: Option<String>,
    /// The `bits` argument.
    pub(crate) bits: u32,
}

/// Convert a vector's `meta` value into the `meta` argument.
///
/// `null` is no meta (`Some(None)`), a string is a data URL as given and an
/// object is serialized. Any other JSON type is rejected (`None`).
#[cfg(feature = "meta-code")]
fn meta_arg(value: &Value) -> Option<Option<String>> {
    match value {
        Value::Null => Some(None),
        Value::String(s) => Some(Some(s.clone())),
        Value::Object(_) => serde_json::to_string(value).ok().map(Some),
        _ => None,
    }
}

/// Parse the inputs `[name, description, meta, bits]` of a `gen_meta_code_v0` case.
#[cfg(feature = "meta-code")]
pub(crate) fn meta_inputs(tc: &Value) -> Option<MetaInputs<'_>> {
    let inputs = inputs(tc)?;
    let name = inputs.first()?.as_str()?;
    let description = inputs.get(1)?.as_str()?;
    let meta = meta_arg(inputs.get(2)?)?;
    let bits = bits_at(inputs, 3)?;
    Some(MetaInputs {
        name,
        description: (!description.is_empty()).then_some(description),
        meta,
        bits,
    })
}

/// Parse the inputs `[text, bits]` of a `gen_text_code_v0` case.
#[cfg(feature = "text-processing")]
pub(crate) fn text_inputs(tc: &Value) -> Option<(&str, u32)> {
    let inputs = inputs(tc)?;
    Some((inputs.first()?.as_str()?, bits_at(inputs, 1)?))
}

/// Parse the inputs `[pixels, bits]` of a `gen_image_code_v0` case.
pub(crate) fn image_inputs(tc: &Value) -> Option<(Vec<u8>, u32)> {
    let inputs = inputs(tc)?;
    let pixels = inputs
        .first()?
        .as_array()?
        .iter()
        .map(|v| u8::try_from(v.as_u64()?).ok())
        .collect::<Option<Vec<u8>>>()?;
    Some((pixels, bits_at(inputs, 1)?))
}

/// Parse the inputs `[cv, bits]` of a `gen_audio_code_v0` case.
pub(crate) fn audio_inputs(tc: &Value) -> Option<(Vec<i32>, u32)> {
    let inputs = inputs(tc)?;
    Some((i32_entries(inputs.first()?)?, bits_at(inputs, 1)?))
}

/// Parse the inputs `[frame_sigs, bits]` of a `gen_video_code_v0` case.
pub(crate) fn video_inputs(tc: &Value) -> Option<(Vec<Vec<i32>>, u32)> {
    let inputs = inputs(tc)?;
    let frame_sigs = inputs
        .first()?
        .as_array()?
        .iter()
        .map(i32_entries)
        .collect::<Option<_>>()?;
    Some((frame_sigs, bits_at(inputs, 1)?))
}

/// Parse the inputs `[codes, bits]` of a `gen_mixed_code_v0` case.
pub(crate) fn mixed_inputs(tc: &Value) -> Option<(Vec<&str>, u32)> {
    let inputs = inputs(tc)?;
    Some((string_entries(inputs.first()?)?, bits_at(inputs, 1)?))
}

/// Parse the inputs `["stream:<hex>", bits]` of a Data-Code or Instance-Code case.
pub(crate) fn stream_inputs(tc: &Value) -> Option<(Vec<u8>, u32)> {
    let inputs = inputs(tc)?;
    let bytes = decode_stream(inputs.first()?.as_str()?)?;
    Some((bytes, bits_at(inputs, 1)?))
}

/// Parse the inputs `[codes]` of a `gen_iscc_code_v0` case.
pub(crate) fn iscc_inputs(tc: &Value) -> Option<Vec<&str>> {
    string_entries(inputs(tc)?.first()?)
}

/// Run conformance tests for `gen_meta_code_v0`.
#[cfg(feature = "meta-code")]
fn run_meta_tests(data: &Value) -> bool {
    run_section(data, "gen_meta_code_v0", |tc| {
        let m = meta_inputs(tc)?;
        let expected = expected_iscc(tc)?;
        let result = gen_meta_code_v0(m.name, m.description, m.meta.as_deref(), m.bits);
        Some((result.map(|r| r.iscc), expected))
    })
}

/// Run conformance tests for `gen_text_code_v0`.
#[cfg(feature = "text-processing")]
fn run_text_tests(data: &Value) -> bool {
    run_section(data, "gen_text_code_v0", |tc| {
        let (text, bits) = text_inputs(tc)?;
        let expected = expected_iscc(tc)?;
        Some((gen_text_code_v0(text, bits).map(|r| r.iscc), expected))
    })
}

/// Run conformance tests for `gen_image_code_v0`.
fn run_image_tests(data: &Value) -> bool {
    run_section(data, "gen_image_code_v0", |tc| {
        let (pixels, bits) = image_inputs(tc)?;
        let expected = expected_iscc(tc)?;
        Some((gen_image_code_v0(&pixels, bits).map(|r| r.iscc), expected))
    })
}

/// Run conformance tests for `gen_audio_code_v0`.
fn run_audio_tests(data: &Value) -> bool {
    run_section(data, "gen_audio_code_v0", |tc| {
        let (cv, bits) = audio_inputs(tc)?;
        let expected = expected_iscc(tc)?;
        Some((gen_audio_code_v0(&cv, bits).map(|r| r.iscc), expected))
    })
}

/// Run conformance tests for `gen_video_code_v0`.
fn run_video_tests(data: &Value) -> bool {
    run_section(data, "gen_video_code_v0", |tc| {
        let (frame_sigs, bits) = video_inputs(tc)?;
        let expected = expected_iscc(tc)?;
        let result = gen_video_code_v0(&frame_sigs, bits);
        Some((result.map(|r| r.iscc), expected))
    })
}

/// Run conformance tests for `gen_mixed_code_v0`.
fn run_mixed_tests(data: &Value) -> bool {
    run_section(data, "gen_mixed_code_v0", |tc| {
        let (codes, bits) = mixed_inputs(tc)?;
        let expected = expected_iscc(tc)?;
        Some((gen_mixed_code_v0(&codes, bits).map(|r| r.iscc), expected))
    })
}

/// Run conformance tests for `gen_data_code_v0`.
fn run_data_tests(data: &Value) -> bool {
    run_section(data, "gen_data_code_v0", |tc| {
        let (bytes, bits) = stream_inputs(tc)?;
        let expected = expected_iscc(tc)?;
        Some((gen_data_code_v0(&bytes, bits).map(|r| r.iscc), expected))
    })
}

/// Run conformance tests for `gen_instance_code_v0`.
fn run_instance_tests(data: &Value) -> bool {
    run_section(data, "gen_instance_code_v0", |tc| {
        let (bytes, bits) = stream_inputs(tc)?;
        let expected = expected_iscc(tc)?;
        Some((gen_instance_code_v0(&bytes, bits).map(|r| r.iscc), expected))
    })
}

/// Run conformance tests for `gen_iscc_code_v0`.
fn run_iscc_tests(data: &Value) -> bool {
    run_section(data, "gen_iscc_code_v0", |tc| {
        let codes = iscc_inputs(tc)?;
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
        let units = string_entries(tc["inputs"].as_array()?.first()?)?;
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
        let data = test_data().unwrap();
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
        let data = test_data().unwrap();
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
            serde_json::json!({"inputs": [], "outputs": {"iscc": "ISCC:X"}}),
            serde_json::json!({"inputs": ["stream:"], "outputs": {"iscc": "ISCC:X"}}),
        ];
        for tc in cases {
            let data = serde_json::json!({"gen_data_code_v0": {"case": tc}});
            assert!(!run_data_tests(&data), "{data}");
        }
    }

    #[test]
    fn test_gen_section_short_inputs_fail() {
        // A short inputs list is a parse failure, never a panic.
        #[cfg(feature = "meta-code")]
        {
            let tc =
                serde_json::json!({"inputs": ["Name", "", null], "outputs": {"iscc": "ISCC:X"}});
            assert!(meta_inputs(&tc).is_none());
            assert!(!run_meta_tests(
                &serde_json::json!({"gen_meta_code_v0": {"case": tc}})
            ));
        }
        let tc = serde_json::json!({"inputs": [], "outputs": {"iscc": "ISCC:X"}});
        assert!(iscc_inputs(&tc).is_none());
        assert!(!run_iscc_tests(
            &serde_json::json!({"gen_iscc_code_v0": {"case": tc}})
        ));
    }

    /// Parse and run one synthetic case of `section`; return `(parsed, passed)`.
    fn probe(section: &str, inputs: Value, iscc: &str) -> (bool, bool) {
        let tc = serde_json::json!({"inputs": inputs, "outputs": {"iscc": iscc}});
        let data = serde_json::json!({section: {"case": tc.clone()}});
        match section {
            #[cfg(feature = "meta-code")]
            "gen_meta_code_v0" => (meta_inputs(&tc).is_some(), run_meta_tests(&data)),
            #[cfg(feature = "text-processing")]
            "gen_text_code_v0" => (text_inputs(&tc).is_some(), run_text_tests(&data)),
            "gen_image_code_v0" => (image_inputs(&tc).is_some(), run_image_tests(&data)),
            "gen_audio_code_v0" => (audio_inputs(&tc).is_some(), run_audio_tests(&data)),
            "gen_video_code_v0" => (video_inputs(&tc).is_some(), run_video_tests(&data)),
            "gen_mixed_code_v0" => (mixed_inputs(&tc).is_some(), run_mixed_tests(&data)),
            "gen_data_code_v0" => (stream_inputs(&tc).is_some(), run_data_tests(&data)),
            "gen_iscc_code_v0" => (iscc_inputs(&tc).is_some(), run_iscc_tests(&data)),
            _ => panic!("no runner for {section}"),
        }
    }

    /// Return 1024 pixel values, all zero except the first, which is `first`.
    fn pixels_with(first: Value) -> Value {
        let mut pixels = vec![Value::from(0); 1024];
        pixels[0] = first;
        Value::from(pixels)
    }

    /// Return one 380-entry frame signature, all zero except the first, which is `first`.
    fn frame_with(first: Value) -> Value {
        let mut frame = vec![Value::from(0); 380];
        frame[0] = first;
        Value::from(frame)
    }

    /// One synthetic case: `(section, inputs, expected iscc)`.
    type Row = (&'static str, Value, String);

    /// Assert that every `bad` row fails the parse arm alone and every `good` row passes.
    ///
    /// Each bad row's output is what the coerced, truncated or skipped reading
    /// would produce, so only the parse arm can fail it.
    fn assert_rows(bad: Vec<Row>, good: Vec<Row>) {
        for (section, inputs, iscc) in bad {
            let shown: String = inputs.to_string().chars().take(60).collect();
            assert_eq!(
                probe(section, inputs, &iscc),
                (false, false),
                "{section} {shown}"
            );
        }
        for (section, inputs, iscc) in good {
            let shown: String = inputs.to_string().chars().take(60).collect();
            assert_eq!(
                probe(section, inputs, &iscc),
                (true, true),
                "{section} {shown}"
            );
        }
    }

    #[test]
    fn test_wrong_type_vector_values_are_unparsable() {
        // One bad case per call: every row must fail the parse arm alone.
        use serde_json::json;
        let data64 = gen_data_code_v0(&[], 64).unwrap().iscc;
        let image = gen_image_code_v0(&[0; 1024], 64).unwrap().iscc;
        let audio = gen_audio_code_v0(&[i32::MIN], 64).unwrap().iscc;
        let mut frame = vec![0; 380];
        frame[0] = i32::MIN;
        let video = gen_video_code_v0(&[frame], 64).unwrap().iscc;
        let audio_ok = gen_audio_code_v0(&[1, 2, 3], 64).unwrap().iscc;
        let mixed = gen_mixed_code_v0(&[&image, &audio_ok], 64).unwrap().iscc;
        let instance = gen_instance_code_v0(&[], 64).unwrap().iscc;
        let iscc = gen_iscc_code_v0(&[&data64, &instance], false).unwrap().iscc;
        let wrapped = json!(2147483648_i64);
        let bad = vec![
            ("gen_data_code_v0", json!(["stream:", 64.0]), data64.clone()),
            ("gen_data_code_v0", json!(["stream:", 64.5]), data64.clone()),
            ("gen_data_code_v0", json!(["stream:", -1]), data64.clone()),
            (
                "gen_data_code_v0",
                json!(["stream:", 4294967296_u64]),
                data64.clone(),
            ),
            (
                "gen_mixed_code_v0",
                json!([[&image, &audio_ok, 1], 64]),
                mixed.clone(),
            ),
            (
                "gen_mixed_code_v0",
                json!([[&image, &audio_ok, null], 64]),
                mixed.clone(),
            ),
            (
                "gen_iscc_code_v0",
                json!([[&data64, &instance, 1]]),
                iscc.clone(),
            ),
            (
                "gen_image_code_v0",
                json!([pixels_with(json!(0.5)), 64]),
                image.clone(),
            ),
            (
                "gen_image_code_v0",
                json!([pixels_with(json!(-256)), 64]),
                image.clone(),
            ),
            (
                "gen_image_code_v0",
                json!([pixels_with(json!(256)), 64]),
                image.clone(),
            ),
            ("gen_audio_code_v0", json!([[wrapped.clone()], 64]), audio),
            (
                "gen_video_code_v0",
                json!([[frame_with(wrapped)], 64]),
                video,
            ),
        ];
        let good = vec![
            ("gen_data_code_v0", json!(["stream:", 64]), data64.clone()),
            ("gen_mixed_code_v0", json!([[&image, &audio_ok], 64]), mixed),
            ("gen_iscc_code_v0", json!([[&data64, &instance]]), iscc),
            (
                "gen_image_code_v0",
                json!([pixels_with(json!(0)), 64]),
                image,
            ),
            ("gen_audio_code_v0", json!([[1, 2, 3], 64]), audio_ok),
        ];
        assert_rows(bad, good);
    }

    #[cfg(feature = "meta-code")]
    #[test]
    fn test_wrong_type_meta_values_are_unparsable() {
        // A null name or description and a number or array meta fail the parse arm.
        use serde_json::json;
        let meta = gen_meta_code_v0("Name", None, None, 64).unwrap().iscc;
        let with_meta = gen_meta_code_v0("Name", None, Some(r#"{"a":1}"#), 64);
        let bad = vec![
            (
                "gen_meta_code_v0",
                json!(["Name", null, null, 64]),
                meta.clone(),
            ),
            (
                "gen_meta_code_v0",
                json!([null, "", null, 64]),
                meta.clone(),
            ),
            ("gen_meta_code_v0", json!(["Name", "", 5, 64]), meta.clone()),
            (
                "gen_meta_code_v0",
                json!(["Name", "", [], 64]),
                meta.clone(),
            ),
        ];
        let good = vec![
            ("gen_meta_code_v0", json!(["Name", "", null, 64]), meta),
            (
                "gen_meta_code_v0",
                json!(["Name", "", {"a": 1}, 64]),
                with_meta.unwrap().iscc,
            ),
        ];
        assert_rows(bad, good);
    }

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_wrong_type_text_value_is_unparsable() {
        // A null text fails the parse arm; an empty text parses.
        use serde_json::json;
        let text = gen_text_code_v0("", 64).unwrap().iscc;
        let bad = vec![("gen_text_code_v0", json!([null, 64]), text.clone())];
        let good = vec![("gen_text_code_v0", json!(["", 64]), text)];
        assert_rows(bad, good);
    }

    #[test]
    fn test_seq_wrong_type_values_are_unparsable() {
        // A non-string unit fails encode_seq; a null output unit fails decode_seq.
        let encode = serde_json::json!({"inputs": [["ISCC:GAAAAAAAAA", null]],
            "outputs": "bytes:300000000000"});
        assert!(!run_encode_seq_tests(
            &serde_json::json!({"encode_seq": {"case": encode}})
        ));
        let decode = serde_json::json!({"inputs": ["bytes:300000000000"],
            "outputs": ["ISCC:GAAAAAAAAA", null]});
        assert!(!run_decode_seq_tests(
            &serde_json::json!({"decode_seq": {"case": decode}})
        ));
    }

    #[test]
    fn test_check_sections() {
        // Every key but `_metadata` must map to an object of case objects.
        assert_eq!(check_sections(&test_data().unwrap()), Ok(()));
        let ok = serde_json::json!({"_metadata": 5, "encode_seq": {"case": {}}});
        assert_eq!(check_sections(&ok), Ok(()));
        let bad = [
            serde_json::json!({"gen_data_code_v0": 5}),
            serde_json::json!({"gen_data_code_v0": null}),
            serde_json::json!({"gen_data_code_v0": {"case": 5}}),
            serde_json::json!([]),
        ];
        for data in bad {
            assert!(check_sections(&data).is_err(), "{data}");
        }
    }
}
