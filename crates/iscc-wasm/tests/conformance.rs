//! Conformance tests for all 9 gen_*_v0 WASM-exported functions and `encode_seq` /
//! `decode_seq` against data.json vectors.
//!
//! Runs in a Node.js WASM runtime via `wasm-pack test --node`.
//! Mirrors the Rust core and Node.js conformance test patterns.

use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

/// Vendored conformance vectors loaded at compile time.
const DATA_JSON: &str = include_str!("../../iscc-lib/tests/data.json");

/// Stringify a JSON object with sorted keys (matching Python json.dumps(sort_keys=True)).
fn json_sorted_stringify(val: &serde_json::Value) -> String {
    serde_json::to_string(val).unwrap()
}

/// Convert meta input from JSON value to Option<String> for gen_meta_code_v0.
fn prepare_meta_arg(val: &serde_json::Value) -> Option<String> {
    match val {
        serde_json::Value::Null => None,
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Object(_) => Some(json_sorted_stringify(val)),
        other => panic!("unexpected meta type: {other:?}"),
    }
}

/// Decode "bytes:<hex>" value to Vec<u8>.
fn decode_bytes(bytes_str: &str) -> Vec<u8> {
    let hex_data = bytes_str
        .strip_prefix("bytes:")
        .unwrap_or_else(|| panic!("expected 'bytes:' prefix, got: {bytes_str}"));
    hex::decode(hex_data).unwrap_or_else(|e| panic!("invalid hex: {e}"))
}

/// Decode "stream:<hex>" input to Vec<u8>.
fn decode_stream(stream_str: &str) -> Vec<u8> {
    let hex_data = stream_str
        .strip_prefix("stream:")
        .unwrap_or_else(|| panic!("expected 'stream:' prefix, got: {stream_str}"));
    if hex_data.is_empty() {
        return Vec::new();
    }
    hex::decode(hex_data).unwrap_or_else(|e| panic!("invalid hex: {e}"))
}

// ── gen_meta_code_v0 ─────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_gen_meta_code_v0_conformance() {
    let data: serde_json::Value = serde_json::from_str(DATA_JSON).unwrap();
    let cases = data["gen_meta_code_v0"].as_object().unwrap();
    let mut tested = 0;

    for (tc_name, tc) in cases {
        let inputs = tc["inputs"].as_array().unwrap();
        let name_arg = inputs[0].as_str().unwrap();
        let description = inputs[1].as_str().unwrap();
        let meta = prepare_meta_arg(&inputs[2]);
        let bits = inputs[3].as_u64().unwrap() as f64;
        let expected = tc["outputs"]["iscc"].as_str().unwrap();

        let desc = if description.is_empty() {
            None
        } else {
            Some(description.to_string())
        };

        let result = iscc_wasm::gen_meta_code_v0(name_arg, desc, meta, JsValue::from(bits))
            .unwrap_or_else(|e| panic!("gen_meta_code_v0 failed for {tc_name}: {e:?}"));
        assert_eq!(result, expected, "ISCC mismatch in test case {tc_name}");
        tested += 1;
    }

    assert_eq!(tested, 20, "expected 20 conformance tests to run");
}

// ── gen_text_code_v0 ─────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_gen_text_code_v0_conformance() {
    let data: serde_json::Value = serde_json::from_str(DATA_JSON).unwrap();
    let cases = data["gen_text_code_v0"].as_object().unwrap();
    let mut tested = 0;

    for (tc_name, tc) in cases {
        let inputs = tc["inputs"].as_array().unwrap();
        let text = inputs[0].as_str().unwrap();
        let bits = inputs[1].as_u64().unwrap() as f64;
        let expected = tc["outputs"]["iscc"].as_str().unwrap();

        let result = iscc_wasm::gen_text_code_v0(text, JsValue::from(bits))
            .unwrap_or_else(|e| panic!("gen_text_code_v0 failed for {tc_name}: {e:?}"));
        assert_eq!(result, expected, "ISCC mismatch in test case {tc_name}");
        tested += 1;
    }

    assert_eq!(tested, 5, "expected 5 conformance tests to run");
}

// ── gen_image_code_v0 ────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_gen_image_code_v0_conformance() {
    let data: serde_json::Value = serde_json::from_str(DATA_JSON).unwrap();
    let cases = data["gen_image_code_v0"].as_object().unwrap();
    let mut tested = 0;

    for (tc_name, tc) in cases {
        let inputs = tc["inputs"].as_array().unwrap();
        let pixels_json = inputs[0].as_array().unwrap();
        let bits = inputs[1].as_u64().unwrap() as f64;
        let expected = tc["outputs"]["iscc"].as_str().unwrap();

        let pixels: Vec<u8> = pixels_json
            .iter()
            .map(|v| v.as_u64().unwrap() as u8)
            .collect();

        let result = iscc_wasm::gen_image_code_v0(&pixels, JsValue::from(bits))
            .unwrap_or_else(|e| panic!("gen_image_code_v0 failed for {tc_name}: {e:?}"));
        assert_eq!(result, expected, "ISCC mismatch in test case {tc_name}");
        tested += 1;
    }

    assert_eq!(tested, 3, "expected 3 conformance tests to run");
}

// ── gen_audio_code_v0 ────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_gen_audio_code_v0_conformance() {
    let data: serde_json::Value = serde_json::from_str(DATA_JSON).unwrap();
    let cases = data["gen_audio_code_v0"].as_object().unwrap();
    let mut tested = 0;

    for (tc_name, tc) in cases {
        let inputs = tc["inputs"].as_array().unwrap();
        let cv_json = inputs[0].as_array().unwrap();
        let bits = inputs[1].as_u64().unwrap() as f64;
        let expected = tc["outputs"]["iscc"].as_str().unwrap();

        let cv: js_sys::Array = cv_json
            .iter()
            .map(|v| JsValue::from(v.as_i64().unwrap() as f64))
            .collect();

        let result = iscc_wasm::gen_audio_code_v0(cv.into(), JsValue::from(bits))
            .unwrap_or_else(|e| panic!("gen_audio_code_v0 failed for {tc_name}: {e:?}"));
        assert_eq!(result, expected, "ISCC mismatch in test case {tc_name}");
        tested += 1;
    }

    assert_eq!(tested, 5, "expected 5 conformance tests to run");
}

// ── gen_video_code_v0 ────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_gen_video_code_v0_conformance() {
    let data: serde_json::Value = serde_json::from_str(DATA_JSON).unwrap();
    let cases = data["gen_video_code_v0"].as_object().unwrap();
    let mut tested = 0;

    for (tc_name, tc) in cases {
        let inputs = tc["inputs"].as_array().unwrap();
        let frames_json = inputs[0].as_array().unwrap();
        let bits = inputs[1].as_u64().unwrap() as f64;
        let expected = tc["outputs"]["iscc"].as_str().unwrap();

        let frame_sigs: Vec<Vec<i32>> = frames_json
            .iter()
            .map(|frame| {
                frame
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_i64().unwrap() as i32)
                    .collect()
            })
            .collect();

        let frame_sigs_js: JsValue = serde_wasm_bindgen::to_value(&frame_sigs).unwrap();
        let result = iscc_wasm::gen_video_code_v0(frame_sigs_js, JsValue::from(bits))
            .unwrap_or_else(|e| panic!("gen_video_code_v0 failed for {tc_name}: {e:?}"));
        assert_eq!(result, expected, "ISCC mismatch in test case {tc_name}");
        tested += 1;
    }

    assert_eq!(tested, 3, "expected 3 conformance tests to run");
}

// ── gen_mixed_code_v0 ────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_gen_mixed_code_v0_conformance() {
    let data: serde_json::Value = serde_json::from_str(DATA_JSON).unwrap();
    let cases = data["gen_mixed_code_v0"].as_object().unwrap();
    let mut tested = 0;

    for (tc_name, tc) in cases {
        let inputs = tc["inputs"].as_array().unwrap();
        let codes_json = inputs[0].as_array().unwrap();
        let bits = inputs[1].as_u64().unwrap() as f64;
        let expected = tc["outputs"]["iscc"].as_str().unwrap();

        let codes: Vec<String> = codes_json
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();

        let codes_js: JsValue = serde_wasm_bindgen::to_value(&codes).unwrap();
        let result = iscc_wasm::gen_mixed_code_v0(codes_js, JsValue::from(bits))
            .unwrap_or_else(|e| panic!("gen_mixed_code_v0 failed for {tc_name}: {e:?}"));
        assert_eq!(result, expected, "ISCC mismatch in test case {tc_name}");
        tested += 1;
    }

    assert_eq!(tested, 2, "expected 2 conformance tests to run");
}

// ── gen_data_code_v0 ─────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_gen_data_code_v0_conformance() {
    let data: serde_json::Value = serde_json::from_str(DATA_JSON).unwrap();
    let cases = data["gen_data_code_v0"].as_object().unwrap();
    let mut tested = 0;

    for (tc_name, tc) in cases {
        let inputs = tc["inputs"].as_array().unwrap();
        let stream_str = inputs[0].as_str().unwrap();
        let bits = inputs[1].as_u64().unwrap() as f64;
        let expected = tc["outputs"]["iscc"].as_str().unwrap();

        let input_bytes = decode_stream(stream_str);

        let result = iscc_wasm::gen_data_code_v0(&input_bytes, JsValue::from(bits))
            .unwrap_or_else(|e| panic!("gen_data_code_v0 failed for {tc_name}: {e:?}"));
        assert_eq!(result, expected, "ISCC mismatch in test case {tc_name}");
        tested += 1;
    }

    assert_eq!(tested, 4, "expected 4 conformance tests to run");
}

// ── gen_instance_code_v0 ─────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_gen_instance_code_v0_conformance() {
    let data: serde_json::Value = serde_json::from_str(DATA_JSON).unwrap();
    let cases = data["gen_instance_code_v0"].as_object().unwrap();
    let mut tested = 0;

    for (tc_name, tc) in cases {
        let inputs = tc["inputs"].as_array().unwrap();
        let stream_str = inputs[0].as_str().unwrap();
        let bits = inputs[1].as_u64().unwrap() as f64;
        let expected = tc["outputs"]["iscc"].as_str().unwrap();

        let input_bytes = decode_stream(stream_str);

        let result = iscc_wasm::gen_instance_code_v0(&input_bytes, JsValue::from(bits))
            .unwrap_or_else(|e| panic!("gen_instance_code_v0 failed for {tc_name}: {e:?}"));
        assert_eq!(result, expected, "ISCC mismatch in test case {tc_name}");
        tested += 1;
    }

    assert_eq!(tested, 3, "expected 3 conformance tests to run");
}

// ── gen_iscc_code_v0 ─────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_gen_iscc_code_v0_conformance() {
    let data: serde_json::Value = serde_json::from_str(DATA_JSON).unwrap();
    let cases = data["gen_iscc_code_v0"].as_object().unwrap();
    let mut tested = 0;

    for (tc_name, tc) in cases {
        let inputs = tc["inputs"].as_array().unwrap();
        let codes_json = inputs[0].as_array().unwrap();
        let expected = tc["outputs"]["iscc"].as_str().unwrap();

        let codes: Vec<String> = codes_json
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();

        let codes_js: JsValue = serde_wasm_bindgen::to_value(&codes).unwrap();
        let result = iscc_wasm::gen_iscc_code_v0(codes_js, JsValue::UNDEFINED)
            .unwrap_or_else(|e| panic!("gen_iscc_code_v0 failed for {tc_name}: {e:?}"));
        assert_eq!(result, expected, "ISCC mismatch in test case {tc_name}");
        tested += 1;
    }

    assert_eq!(tested, 5, "expected 5 conformance tests to run");
}

// ── encode_seq ───────────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_encode_seq_conformance() {
    let data: serde_json::Value = serde_json::from_str(DATA_JSON).unwrap();
    let cases = data["encode_seq"].as_object().unwrap();
    let mut tested = 0;

    for (tc_name, tc) in cases {
        let units: Vec<String> = tc["inputs"][0]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        let expected = decode_bytes(tc["outputs"].as_str().unwrap());

        let units_js: JsValue = serde_wasm_bindgen::to_value(&units).unwrap();
        let result = iscc_wasm::encode_seq(units_js)
            .unwrap_or_else(|e| panic!("encode_seq failed for {tc_name}: {e:?}"));
        assert_eq!(result, expected, "ISCC-SEQ mismatch in test case {tc_name}");
        tested += 1;
    }

    assert_eq!(tested, 4, "expected 4 conformance tests to run");
}

// ── decode_seq ───────────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_decode_seq_conformance() {
    let data: serde_json::Value = serde_json::from_str(DATA_JSON).unwrap();
    let cases = data["decode_seq"].as_object().unwrap();
    let mut tested = 0;

    for (tc_name, tc) in cases {
        let seq = decode_bytes(tc["inputs"][0].as_str().unwrap());
        let expected: Vec<String> = tc["outputs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();

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

    assert_eq!(tested, 4, "expected 4 conformance tests to run");
}
