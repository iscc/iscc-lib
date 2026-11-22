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

/// Outcome of one parsed conformance case: `Err` holds the failure description.
type Verdict = Result<(), String>;

/// Parse one case, call the function under test and judge its result.
///
/// `None` means the case's inputs or expected outputs could not be parsed.
type CaseCheck = fn(&Value) -> Option<Verdict>;

/// Every `data.json` section, each with the check that runs one of its cases.
///
/// This table is the only list of sections: `conformance_selftest` runs every
/// row, and a test compares the row names with the `data.json` keys. Rows whose
/// feature flag is disabled are left out, so their sections are skipped.
const SECTIONS: &[(&str, CaseCheck)] = &[
    #[cfg(feature = "meta-code")]
    ("gen_meta_code_v0", meta_case),
    #[cfg(feature = "text-processing")]
    ("gen_text_code_v0", text_case),
    ("gen_image_code_v0", image_case),
    ("gen_audio_code_v0", audio_case),
    ("gen_video_code_v0", video_case),
    ("gen_mixed_code_v0", mixed_case),
    ("gen_data_code_v0", data_case),
    ("gen_instance_code_v0", instance_case),
    ("gen_iscc_code_v0", iscc_case),
    ("encode_seq", encode_seq_case),
    ("decode_seq", decode_seq_case),
];

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
    for &(name, check) in SECTIONS {
        passed &= run_section(&data, name, check);
    }
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

/// Run every case of one `data.json` section and report each failure.
///
/// Returns `false` if the section is missing or any case fails.
fn run_section(data: &Value, func_name: &str, check: CaseCheck) -> bool {
    let Some(section) = cases(data, func_name) else {
        eprintln!("FAILED: {func_name} section missing from conformance data");
        return false;
    };

    let mut passed = true;
    for (tc_name, tc) in section {
        passed &= report_case(func_name, tc_name, check(tc));
    }
    passed
}

/// Report the outcome of one conformance case; returns whether it passed.
fn report_case(func_name: &str, tc_name: &str, verdict: Option<Verdict>) -> bool {
    match verdict {
        None => {
            eprintln!("FAILED: {func_name}.{tc_name} — could not parse test inputs");
            false
        }
        Some(Ok(())) => true,
        Some(Err(reason)) => {
            eprintln!("FAILED: {func_name}.{tc_name} — {reason}");
            false
        }
    }
}

/// Judge a function result against the expected output.
fn judge<T: PartialEq + Debug>(result: IsccResult<T>, expected: T) -> Verdict {
    match result {
        Ok(actual) if actual == expected => Ok(()),
        Ok(actual) => Err(format!("expected {expected:?}, got {actual:?}")),
        Err(e) => Err(format!("error: {e}")),
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
fn audio_inputs(tc: &Value) -> Option<(Vec<i32>, u32)> {
    let inputs = inputs(tc)?;
    Some((i32_entries(inputs.first()?)?, bits_at(inputs, 1)?))
}

/// Parse the inputs `[frame_sigs, bits]` of a `gen_video_code_v0` case.
fn video_inputs(tc: &Value) -> Option<(Vec<Vec<i32>>, u32)> {
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

/// Check one `gen_meta_code_v0` case.
#[cfg(feature = "meta-code")]
fn meta_case(tc: &Value) -> Option<Verdict> {
    let m = meta_inputs(tc)?;
    let expected = expected_iscc(tc)?;
    let result = gen_meta_code_v0(m.name, m.description, m.meta.as_deref(), m.bits);
    Some(judge(result.map(|r| r.iscc), expected))
}

/// Check one `gen_text_code_v0` case.
#[cfg(feature = "text-processing")]
fn text_case(tc: &Value) -> Option<Verdict> {
    let (text, bits) = text_inputs(tc)?;
    let expected = expected_iscc(tc)?;
    Some(judge(
        gen_text_code_v0(text, bits).map(|r| r.iscc),
        expected,
    ))
}

/// Check one `gen_image_code_v0` case.
fn image_case(tc: &Value) -> Option<Verdict> {
    let (pixels, bits) = image_inputs(tc)?;
    let expected = expected_iscc(tc)?;
    Some(judge(
        gen_image_code_v0(&pixels, bits).map(|r| r.iscc),
        expected,
    ))
}

/// Check one `gen_audio_code_v0` case.
fn audio_case(tc: &Value) -> Option<Verdict> {
    let (cv, bits) = audio_inputs(tc)?;
    let expected = expected_iscc(tc)?;
    Some(judge(
        gen_audio_code_v0(&cv, bits).map(|r| r.iscc),
        expected,
    ))
}

/// Check one `gen_video_code_v0` case.
fn video_case(tc: &Value) -> Option<Verdict> {
    let (frame_sigs, bits) = video_inputs(tc)?;
    let expected = expected_iscc(tc)?;
    let result = gen_video_code_v0(&frame_sigs, bits);
    Some(judge(result.map(|r| r.iscc), expected))
}

/// Check one `gen_mixed_code_v0` case.
fn mixed_case(tc: &Value) -> Option<Verdict> {
    let (codes, bits) = mixed_inputs(tc)?;
    let expected = expected_iscc(tc)?;
    Some(judge(
        gen_mixed_code_v0(&codes, bits).map(|r| r.iscc),
        expected,
    ))
}

/// Check one `gen_data_code_v0` case.
fn data_case(tc: &Value) -> Option<Verdict> {
    let (bytes, bits) = stream_inputs(tc)?;
    let expected = expected_iscc(tc)?;
    Some(judge(
        gen_data_code_v0(&bytes, bits).map(|r| r.iscc),
        expected,
    ))
}

/// Check one `gen_instance_code_v0` case.
fn instance_case(tc: &Value) -> Option<Verdict> {
    let (bytes, bits) = stream_inputs(tc)?;
    let expected = expected_iscc(tc)?;
    Some(judge(
        gen_instance_code_v0(&bytes, bits).map(|r| r.iscc),
        expected,
    ))
}

/// Check one `gen_iscc_code_v0` case.
fn iscc_case(tc: &Value) -> Option<Verdict> {
    let codes = iscc_inputs(tc)?;
    let expected = expected_iscc(tc)?;
    // Conformance vectors use default (non-wide) mode
    Some(judge(
        gen_iscc_code_v0(&codes, false).map(|r| r.iscc),
        expected,
    ))
}

/// Decode a `"bytes:<hex>"` value into bytes.
fn decode_bytes(s: &str) -> Option<Vec<u8>> {
    let hex_data = s.strip_prefix("bytes:")?;
    hex::decode(hex_data).ok()
}

/// Check one `encode_seq` case (inputs `[[str…]]`, output `"bytes:<hex>"`).
///
/// Both sides are compared as lower-case hex so a mismatch reads as hex.
fn encode_seq_case(tc: &Value) -> Option<Verdict> {
    let units = string_entries(tc["inputs"].as_array()?.first()?)?;
    let expected = hex::encode(decode_bytes(tc["outputs"].as_str()?)?);
    Some(judge(encode_seq(&units).map(hex::encode), expected))
}

/// Check one `decode_seq` case (inputs `["bytes:<hex>"]`, output `[str…]`).
fn decode_seq_case(tc: &Value) -> Option<Verdict> {
    let seq = decode_bytes(tc["inputs"].as_array()?.first()?.as_str()?)?;
    let expected: Vec<String> = tc["outputs"]
        .as_array()?
        .iter()
        .map(|v| v.as_str().map(String::from))
        .collect::<Option<_>>()?;
    Some(judge(decode_seq(&seq), expected))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Look up the case check of `section` in [`SECTIONS`]; panics if it has no row.
    fn check_of(section: &str) -> CaseCheck {
        SECTIONS
            .iter()
            .find(|(name, _)| *name == section)
            .map(|&(_, check)| check)
            .unwrap_or_else(|| panic!("no runner for {section}"))
    }

    /// Run the `section` cases of `data` through that section's table row.
    fn run(data: &Value, section: &str) -> bool {
        run_section(data, section, check_of(section))
    }

    #[test]
    fn test_conformance_selftest_passes() {
        assert!(conformance_selftest());
    }

    #[cfg(feature = "meta-code")]
    #[test]
    fn test_sections_match_data_json() {
        // The table must name every data.json section exactly once, and nothing else.
        use std::collections::BTreeSet;
        let data = test_data().unwrap();
        let keys: BTreeSet<&str> = data
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .filter(|key| *key != "_metadata")
            .collect();
        let names: BTreeSet<&str> = SECTIONS.iter().map(|&(name, _)| name).collect();
        assert_eq!(names.len(), SECTIONS.len(), "a section has two table rows");
        assert!(!keys.is_empty(), "data.json has no sections");
        assert_eq!(names, keys, "table rows differ from data.json sections");
    }

    #[test]
    fn test_sections_have_cases() {
        // An empty section passes the selftest, so every table row needs at least one case.
        let data = test_data().unwrap();
        for &(name, _) in SECTIONS {
            let count = cases(&data, name).map_or(0, Map::len);
            assert!(count > 0, "section {name} has no cases");
        }
    }

    #[test]
    fn test_seq_section_counts() {
        let data = test_data().unwrap();
        assert_eq!(data["encode_seq"].as_object().unwrap().len(), 4);
        assert_eq!(data["decode_seq"].as_object().unwrap().len(), 4);
    }

    #[test]
    fn test_seq_sections_missing_fail() {
        let empty = serde_json::json!({});
        assert!(!run(&empty, "encode_seq"));
        assert!(!run(&empty, "decode_seq"));
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
            assert!(!run(&data, "encode_seq"), "{data}");
        }
        let decode_cases = [
            serde_json::json!({"inputs": ["bytes:300000000000"], "outputs": ["ISCC:X"]}),
            serde_json::json!({"inputs": ["bytes:"], "outputs": []}),
            serde_json::json!({"inputs": ["300000000000"], "outputs": []}),
        ];
        for tc in decode_cases {
            let data = serde_json::json!({"decode_seq": {"case": tc}});
            assert!(!run(&data, "decode_seq"), "{data}");
        }
    }

    #[test]
    fn test_gen_section_missing_fails() {
        let empty = serde_json::json!({});
        assert!(!run(&empty, "gen_data_code_v0"));
        assert!(!run(&empty, "gen_iscc_code_v0"));
    }

    #[test]
    fn test_gen_section_positive_case() {
        // Guards the failure-arm test: the same shape with a correct output passes.
        let iscc = gen_data_code_v0(&[], 64).unwrap().iscc;
        let tc = serde_json::json!({"inputs": ["stream:", 64], "outputs": {"iscc": iscc}});
        assert!(run(
            &serde_json::json!({"gen_data_code_v0": {"case": tc}}),
            "gen_data_code_v0"
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
            assert!(!run(&data, "gen_data_code_v0"), "{data}");
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
            assert!(!run(
                &serde_json::json!({"gen_meta_code_v0": {"case": tc}}),
                "gen_meta_code_v0"
            ));
        }
        let tc = serde_json::json!({"inputs": [], "outputs": {"iscc": "ISCC:X"}});
        assert!(iscc_inputs(&tc).is_none());
        assert!(!run(
            &serde_json::json!({"gen_iscc_code_v0": {"case": tc}}),
            "gen_iscc_code_v0"
        ));
    }

    /// Parse and run one synthetic case of `section`; return `(parsed, passed)`.
    fn probe(section: &str, inputs: Value, iscc: &str) -> (bool, bool) {
        let tc = serde_json::json!({"inputs": inputs, "outputs": {"iscc": iscc}});
        let data = serde_json::json!({section: {"case": tc.clone()}});
        let check = check_of(section);
        (check(&tc).is_some(), run_section(&data, section, check))
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
        assert!(!run(
            &serde_json::json!({"encode_seq": {"case": encode}}),
            "encode_seq"
        ));
        let decode = serde_json::json!({"inputs": ["bytes:300000000000"],
            "outputs": ["ISCC:GAAAAAAAAA", null]});
        assert!(!run(
            &serde_json::json!({"decode_seq": {"case": decode}}),
            "decode_seq"
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
