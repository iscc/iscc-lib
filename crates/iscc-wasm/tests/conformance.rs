//! Conformance tests for all 9 gen_*_v0 WASM-exported functions and `encode_seq` /
//! `decode_seq` against data.json vectors.
//!
//! Vector values are read through strict readers: a wrong JSON type or an
//! out-of-range number fails the test instead of being cast or truncated.
//! Each section's case count comes from `data.json`, and an empty section fails.
//!
//! Runs in a Node.js WASM runtime via `wasm-pack test --node`.
//! Mirrors the Rust core and Node.js conformance test patterns.

mod common;

use common::{bytes_arg, bytes_item, frames_arg, str_arg, str_list_arg, u32_arg};
use serde_json::{Map, Value};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

/// Vendored conformance vectors loaded at compile time.
const DATA_JSON: &str = include_str!("../../iscc-lib/tests/data.json");

/// Runs one gen case's inputs through a binding function and returns its ISCC.
///
/// `Err` is an unparsable input or the binding's error.
type GenCaller = fn(&[Value]) -> Result<String, String>;

/// Return the cases of one `data.json` section; panics if it is missing or empty.
fn section_cases(data: &Value, section: &str) -> Map<String, Value> {
    let cases = data[section]
        .as_object()
        .unwrap_or_else(|| panic!("section {section} missing from data.json"))
        .clone();
    assert!(!cases.is_empty(), "section {section} has no cases");
    cases
}

/// Return the positional `inputs` list of a case; panics if it is not a list.
fn case_inputs<'a>(tc_name: &str, tc: &'a Value) -> &'a [Value] {
    tc["inputs"]
        .as_array()
        .unwrap_or_else(|| panic!("inputs of {tc_name} is not a list"))
}

/// Convert a binding error into its debug text.
fn err_text(e: wasm_bindgen::JsError) -> String {
    format!("{e:?}")
}

/// Run every case of a gen section through `call` and compare its `.iscc` output.
///
/// Asserts that the number of cases run equals the section's count in `data.json`.
fn check_gen_section(section: &str, call: GenCaller) {
    let data: Value = serde_json::from_str(DATA_JSON).unwrap();
    let cases = section_cases(&data, section);
    let mut tested = 0;

    for (tc_name, tc) in &cases {
        let expected = tc["outputs"]["iscc"]
            .as_str()
            .unwrap_or_else(|| panic!("outputs.iscc of {tc_name} is not a string"));
        let result = call(case_inputs(tc_name, tc))
            .unwrap_or_else(|e| panic!("{section} failed for {tc_name}: {e}"));
        assert_eq!(result, expected, "ISCC mismatch in test case {tc_name}");
        tested += 1;
    }

    assert_eq!(tested, cases.len(), "{section}: not every case ran");
}

/// Read the input at `index` as a `meta` argument: `null` is none, a string is
/// passed as is and a JSON object is serialized.
fn opt_meta_arg(inputs: &[Value], index: usize) -> Result<Option<String>, String> {
    match inputs.get(index) {
        Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => Ok(Some(s.clone())),
        Some(v @ Value::Object(_)) => Ok(Some(v.to_string())),
        _ => Err(format!(
            "input {index} is not null, a meta object or a string"
        )),
    }
}

/// Read the input at `index` as a list of `u8` pixel values.
fn pixels_arg(inputs: &[Value], index: usize) -> Result<Vec<u8>, String> {
    inputs
        .get(index)
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| format!("input {index} is not a list of u8"))
}

/// Read the input at `index` as a list of `i32` values, converted to a JS array.
fn i32_list_arg(inputs: &[Value], index: usize) -> Result<JsValue, String> {
    let values: Vec<i32> = inputs
        .get(index)
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| format!("input {index} is not a list of i32"))?;
    serde_wasm_bindgen::to_value(&values).map_err(|e| format!("input {index}: {e}"))
}

/// Read the input at `index` as a `stream:<hex>` byte string (an empty hex is no bytes).
fn stream_arg(inputs: &[Value], index: usize) -> Result<Vec<u8>, String> {
    inputs
        .get(index)
        .and_then(Value::as_str)
        .and_then(|s| s.strip_prefix("stream:"))
        .and_then(|h| hex::decode(h).ok())
        .ok_or_else(|| format!("input {index} is not a `stream:<hex>` string"))
}

/// Run `gen_meta_code_v0` (an empty description means none).
fn call_meta(inputs: &[Value]) -> Result<String, String> {
    let name = str_arg(inputs, 0)?;
    let description = Some(str_arg(inputs, 1)?)
        .filter(|d| !d.is_empty())
        .map(str::to_owned);
    let meta = opt_meta_arg(inputs, 2)?;
    let bits = u32_arg(inputs, 3)?;
    iscc_wasm::gen_meta_code_v0(name, description, meta, JsValue::from(bits)).map_err(err_text)
}

/// Run `gen_text_code_v0` on the case's text and bits.
fn call_text(inputs: &[Value]) -> Result<String, String> {
    let text = str_arg(inputs, 0)?;
    let bits = u32_arg(inputs, 1)?;
    iscc_wasm::gen_text_code_v0(text, JsValue::from(bits)).map_err(err_text)
}

/// Run `gen_image_code_v0` on the case's pixels and bits.
fn call_image(inputs: &[Value]) -> Result<String, String> {
    let pixels = pixels_arg(inputs, 0)?;
    let bits = u32_arg(inputs, 1)?;
    iscc_wasm::gen_image_code_v0(&pixels, JsValue::from(bits)).map_err(err_text)
}

/// Run `gen_audio_code_v0` on the case's chromaprint vector and bits.
fn call_audio(inputs: &[Value]) -> Result<String, String> {
    let cv = i32_list_arg(inputs, 0)?;
    let bits = u32_arg(inputs, 1)?;
    iscc_wasm::gen_audio_code_v0(cv, JsValue::from(bits)).map_err(err_text)
}

/// Run `gen_video_code_v0` on the case's frame signatures and bits.
fn call_video(inputs: &[Value]) -> Result<String, String> {
    let frames = frames_arg(inputs, 0)?;
    let bits = u32_arg(inputs, 1)?;
    iscc_wasm::gen_video_code_v0(frames, JsValue::from(bits)).map_err(err_text)
}

/// Run `gen_mixed_code_v0` on the case's codes and bits.
fn call_mixed(inputs: &[Value]) -> Result<String, String> {
    let codes = str_list_arg(inputs, 0)?;
    let bits = u32_arg(inputs, 1)?;
    iscc_wasm::gen_mixed_code_v0(codes, JsValue::from(bits)).map_err(err_text)
}

/// Run `gen_data_code_v0` on the case's stream and bits.
fn call_data(inputs: &[Value]) -> Result<String, String> {
    let bytes = stream_arg(inputs, 0)?;
    let bits = u32_arg(inputs, 1)?;
    iscc_wasm::gen_data_code_v0(&bytes, JsValue::from(bits)).map_err(err_text)
}

/// Run `gen_instance_code_v0` on the case's stream and bits.
fn call_instance(inputs: &[Value]) -> Result<String, String> {
    let bytes = stream_arg(inputs, 0)?;
    let bits = u32_arg(inputs, 1)?;
    iscc_wasm::gen_instance_code_v0(&bytes, JsValue::from(bits)).map_err(err_text)
}

/// Run `gen_iscc_code_v0` on the case's codes (conformance vectors use non-wide mode).
fn call_iscc(inputs: &[Value]) -> Result<String, String> {
    let codes = str_list_arg(inputs, 0)?;
    iscc_wasm::gen_iscc_code_v0(codes, JsValue::UNDEFINED).map_err(err_text)
}

#[wasm_bindgen_test]
fn test_gen_meta_code_v0_conformance() {
    check_gen_section("gen_meta_code_v0", call_meta);
}

#[wasm_bindgen_test]
fn test_gen_text_code_v0_conformance() {
    check_gen_section("gen_text_code_v0", call_text);
}

#[wasm_bindgen_test]
fn test_gen_image_code_v0_conformance() {
    check_gen_section("gen_image_code_v0", call_image);
}

#[wasm_bindgen_test]
fn test_gen_audio_code_v0_conformance() {
    check_gen_section("gen_audio_code_v0", call_audio);
}

#[wasm_bindgen_test]
fn test_gen_video_code_v0_conformance() {
    check_gen_section("gen_video_code_v0", call_video);
}

#[wasm_bindgen_test]
fn test_gen_mixed_code_v0_conformance() {
    check_gen_section("gen_mixed_code_v0", call_mixed);
}

#[wasm_bindgen_test]
fn test_gen_data_code_v0_conformance() {
    check_gen_section("gen_data_code_v0", call_data);
}

#[wasm_bindgen_test]
fn test_gen_instance_code_v0_conformance() {
    check_gen_section("gen_instance_code_v0", call_instance);
}

#[wasm_bindgen_test]
fn test_gen_iscc_code_v0_conformance() {
    check_gen_section("gen_iscc_code_v0", call_iscc);
}

#[wasm_bindgen_test]
fn test_encode_seq_conformance() {
    let data: Value = serde_json::from_str(DATA_JSON).unwrap();
    let cases = section_cases(&data, "encode_seq");
    let mut tested = 0;

    for (tc_name, tc) in &cases {
        let units = str_list_arg(case_inputs(tc_name, tc), 0)
            .unwrap_or_else(|e| panic!("encode_seq inputs of {tc_name}: {e}"));
        let expected = bytes_item(&tc["outputs"], 0)
            .unwrap_or_else(|_| panic!("outputs of {tc_name} is not `bytes:<hex>`"));

        let result = iscc_wasm::encode_seq(units)
            .unwrap_or_else(|e| panic!("encode_seq failed for {tc_name}: {e:?}"));
        assert_eq!(result, expected, "ISCC-SEQ mismatch in test case {tc_name}");
        tested += 1;
    }

    assert_eq!(tested, cases.len(), "encode_seq: not every case ran");
}

#[wasm_bindgen_test]
fn test_decode_seq_conformance() {
    let data: Value = serde_json::from_str(DATA_JSON).unwrap();
    let cases = section_cases(&data, "decode_seq");
    let mut tested = 0;

    for (tc_name, tc) in &cases {
        let seq = bytes_arg(case_inputs(tc_name, tc), 0)
            .unwrap_or_else(|e| panic!("decode_seq inputs of {tc_name}: {e}"));
        let expected: Vec<String> = serde_json::from_value(tc["outputs"].clone())
            .unwrap_or_else(|e| panic!("outputs of {tc_name} is not a list of strings: {e}"));

        let result = iscc_wasm::decode_seq(&seq)
            .unwrap_or_else(|e| panic!("decode_seq failed for {tc_name}: {e:?}"));
        assert_eq!(result, expected, "unit mismatch in test case {tc_name}");

        let units_js: JsValue = serde_wasm_bindgen::to_value(&result).unwrap();
        let round_trip = iscc_wasm::encode_seq(units_js)
            .unwrap_or_else(|e| panic!("encode_seq round trip failed for {tc_name}: {e:?}"));
        assert_eq!(
            round_trip, seq,
            "round trip mismatch in test case {tc_name}"
        );
        tested += 1;
    }

    assert_eq!(tested, cases.len(), "decode_seq: not every case ran");
}
