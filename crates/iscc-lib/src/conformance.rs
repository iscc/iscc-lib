//! Conformance selftest for ISO 24138:2024 (ISCC).
//!
//! Runs all 9 gen functions against vendored conformance vectors from `data.json`
//! and reports pass/fail. An application that claims ISCC conformance MUST pass
//! all tests in this suite.

use crate::{
    gen_audio_code_v0, gen_data_code_v0, gen_image_code_v0, gen_instance_code_v0, gen_iscc_code_v0,
    gen_meta_code_v0, gen_mixed_code_v0, gen_text_code_v0, gen_video_code_v0,
};

/// Embedded conformance test vectors (compile-time).
const TEST_DATA: &str = include_str!("../tests/data.json");

/// Run all conformance tests against vendored test vectors.
///
/// Iterates through all 9 `gen_*_v0` function sections in the conformance data,
/// calls each function with the specified inputs, and compares the `.iscc` field
/// of the result against expected output. Returns `true` if all tests pass,
/// `false` if any mismatch or error occurs. Does not panic — logs failures via
/// `eprintln!` and continues through all test cases.
pub fn conformance_selftest() -> bool {
    let data: serde_json::Value = match serde_json::from_str(TEST_DATA) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("FAILED: could not parse conformance data: {e}");
            return false;
        }
    };

    let mut passed = true;

    passed &= run_meta_tests(&data);
    passed &= run_text_tests(&data);
    passed &= run_image_tests(&data);
    passed &= run_audio_tests(&data);
    passed &= run_video_tests(&data);
    passed &= run_mixed_tests(&data);
    passed &= run_data_tests(&data);
    passed &= run_instance_tests(&data);
    passed &= run_iscc_tests(&data);

    passed
}

/// Run conformance tests for `gen_meta_code_v0`.
fn run_meta_tests(data: &serde_json::Value) -> bool {
    let mut passed = true;
    let section = &data["gen_meta_code_v0"];
    let cases = match section.as_object() {
        Some(c) => c,
        None => {
            eprintln!("FAILED: gen_meta_code_v0 section missing from conformance data");
            return false;
        }
    };

    for (tc_name, tc) in cases {
        let func_name = "gen_meta_code_v0";
        let result = (|| {
            let inputs = tc["inputs"].as_array()?;
            let name = inputs[0].as_str()?;
            let desc_str = inputs[1].as_str()?;
            let meta_val = &inputs[2];
            let bits = inputs[3].as_u64()? as u32;

            let meta_arg: Option<String> = match meta_val {
                serde_json::Value::Null => None,
                serde_json::Value::String(s) => Some(s.clone()),
                serde_json::Value::Object(_) => serde_json::to_string(meta_val).ok(),
                _ => None,
            };

            let desc = if desc_str.is_empty() {
                None
            } else {
                Some(desc_str)
            };

            let expected_iscc = tc["outputs"]["iscc"].as_str()?;
            match gen_meta_code_v0(name, desc, meta_arg.as_deref(), bits) {
                Ok(result) if result.iscc == expected_iscc => Some(true),
                Ok(result) => {
                    eprintln!(
                        "FAILED: {func_name}.{tc_name} — expected {expected_iscc}, got {}",
                        result.iscc
                    );
                    Some(false)
                }
                Err(e) => {
                    eprintln!("FAILED: {func_name}.{tc_name} — error: {e}");
                    Some(false)
                }
            }
        })();

        match result {
            Some(true) => {}
            Some(false) => passed = false,
            None => {
                eprintln!("FAILED: {func_name}.{tc_name} — could not parse test inputs");
                passed = false;
            }
        }
    }
    passed
}

/// Run conformance tests for `gen_text_code_v0`.
fn run_text_tests(data: &serde_json::Value) -> bool {
    let mut passed = true;
    let section = &data["gen_text_code_v0"];
    let cases = match section.as_object() {
        Some(c) => c,
        None => {
            eprintln!("FAILED: gen_text_code_v0 section missing from conformance data");
            return false;
        }
    };

    for (tc_name, tc) in cases {
        let func_name = "gen_text_code_v0";
        let result = (|| {
            let inputs = tc["inputs"].as_array()?;
            let text = inputs[0].as_str()?;
            let bits = inputs[1].as_u64()? as u32;
            let expected_iscc = tc["outputs"]["iscc"].as_str()?;

            match gen_text_code_v0(text, bits) {
                Ok(result) if result.iscc == expected_iscc => Some(true),
                Ok(result) => {
                    eprintln!(
                        "FAILED: {func_name}.{tc_name} — expected {expected_iscc}, got {}",
                        result.iscc
                    );
                    Some(false)
                }
                Err(e) => {
                    eprintln!("FAILED: {func_name}.{tc_name} — error: {e}");
                    Some(false)
                }
            }
        })();

        match result {
            Some(true) => {}
            Some(false) => passed = false,
            None => {
                eprintln!("FAILED: {func_name}.{tc_name} — could not parse test inputs");
                passed = false;
            }
        }
    }
    passed
}

/// Run conformance tests for `gen_image_code_v0`.
fn run_image_tests(data: &serde_json::Value) -> bool {
    let mut passed = true;
    let section = &data["gen_image_code_v0"];
    let cases = match section.as_object() {
        Some(c) => c,
        None => {
            eprintln!("FAILED: gen_image_code_v0 section missing from conformance data");
            return false;
        }
    };

    for (tc_name, tc) in cases {
        let func_name = "gen_image_code_v0";
        let result = (|| {
            let inputs = tc["inputs"].as_array()?;
            let pixels_json = inputs[0].as_array()?;
            let bits = inputs[1].as_u64()? as u32;
            let expected_iscc = tc["outputs"]["iscc"].as_str()?;

            let pixels: Vec<u8> = pixels_json
                .iter()
                .map(|v| v.as_u64().map(|n| n as u8))
                .collect::<Option<Vec<u8>>>()?;

            match gen_image_code_v0(&pixels, bits) {
                Ok(result) if result.iscc == expected_iscc => Some(true),
                Ok(result) => {
                    eprintln!(
                        "FAILED: {func_name}.{tc_name} — expected {expected_iscc}, got {}",
                        result.iscc
                    );
                    Some(false)
                }
                Err(e) => {
                    eprintln!("FAILED: {func_name}.{tc_name} — error: {e}");
                    Some(false)
                }
            }
        })();

        match result {
            Some(true) => {}
            Some(false) => passed = false,
            None => {
                eprintln!("FAILED: {func_name}.{tc_name} — could not parse test inputs");
                passed = false;
            }
        }
    }
    passed
}

/// Run conformance tests for `gen_audio_code_v0`.
fn run_audio_tests(data: &serde_json::Value) -> bool {
    let mut passed = true;
    let section = &data["gen_audio_code_v0"];
    let cases = match section.as_object() {
        Some(c) => c,
        None => {
            eprintln!("FAILED: gen_audio_code_v0 section missing from conformance data");
            return false;
        }
    };

    for (tc_name, tc) in cases {
        let func_name = "gen_audio_code_v0";
        let result = (|| {
            let inputs = tc["inputs"].as_array()?;
            let cv_json = inputs[0].as_array()?;
            let bits = inputs[1].as_u64()? as u32;
            let expected_iscc = tc["outputs"]["iscc"].as_str()?;

            let cv: Vec<i32> = cv_json
                .iter()
                .map(|v| v.as_i64().map(|n| n as i32))
                .collect::<Option<Vec<i32>>>()?;

            match gen_audio_code_v0(&cv, bits) {
                Ok(result) if result.iscc == expected_iscc => Some(true),
                Ok(result) => {
                    eprintln!(
                        "FAILED: {func_name}.{tc_name} — expected {expected_iscc}, got {}",
                        result.iscc
                    );
                    Some(false)
                }
                Err(e) => {
                    eprintln!("FAILED: {func_name}.{tc_name} — error: {e}");
                    Some(false)
                }
            }
        })();

        match result {
            Some(true) => {}
            Some(false) => passed = false,
            None => {
                eprintln!("FAILED: {func_name}.{tc_name} — could not parse test inputs");
                passed = false;
            }
        }
    }
    passed
}

/// Run conformance tests for `gen_video_code_v0`.
fn run_video_tests(data: &serde_json::Value) -> bool {
    let mut passed = true;
    let section = &data["gen_video_code_v0"];
    let cases = match section.as_object() {
        Some(c) => c,
        None => {
            eprintln!("FAILED: gen_video_code_v0 section missing from conformance data");
            return false;
        }
    };

    for (tc_name, tc) in cases {
        let func_name = "gen_video_code_v0";
        let result = (|| {
            let inputs = tc["inputs"].as_array()?;
            let frames_json = inputs[0].as_array()?;
            let bits = inputs[1].as_u64()? as u32;
            let expected_iscc = tc["outputs"]["iscc"].as_str()?;

            let frame_sigs: Option<Vec<Vec<i32>>> = frames_json
                .iter()
                .map(|frame| {
                    frame
                        .as_array()?
                        .iter()
                        .map(|v| v.as_i64().map(|n| n as i32))
                        .collect::<Option<Vec<i32>>>()
                })
                .collect();
            let frame_sigs = frame_sigs?;

            match gen_video_code_v0(&frame_sigs, bits) {
                Ok(result) if result.iscc == expected_iscc => Some(true),
                Ok(result) => {
                    eprintln!(
                        "FAILED: {func_name}.{tc_name} — expected {expected_iscc}, got {}",
                        result.iscc
                    );
                    Some(false)
                }
                Err(e) => {
                    eprintln!("FAILED: {func_name}.{tc_name} — error: {e}");
                    Some(false)
                }
            }
        })();

        match result {
            Some(true) => {}
            Some(false) => passed = false,
            None => {
                eprintln!("FAILED: {func_name}.{tc_name} — could not parse test inputs");
                passed = false;
            }
        }
    }
    passed
}

/// Run conformance tests for `gen_mixed_code_v0`.
fn run_mixed_tests(data: &serde_json::Value) -> bool {
    let mut passed = true;
    let section = &data["gen_mixed_code_v0"];
    let cases = match section.as_object() {
        Some(c) => c,
        None => {
            eprintln!("FAILED: gen_mixed_code_v0 section missing from conformance data");
            return false;
        }
    };

    for (tc_name, tc) in cases {
        let func_name = "gen_mixed_code_v0";
        let result = (|| {
            let inputs = tc["inputs"].as_array()?;
            let codes_json = inputs[0].as_array()?;
            let bits = inputs[1].as_u64()? as u32;
            let expected_iscc = tc["outputs"]["iscc"].as_str()?;

            let codes_owned: Vec<String> = codes_json
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();
            let codes: Vec<&str> = codes_owned.iter().map(|s| s.as_str()).collect();

            match gen_mixed_code_v0(&codes, bits) {
                Ok(result) if result.iscc == expected_iscc => Some(true),
                Ok(result) => {
                    eprintln!(
                        "FAILED: {func_name}.{tc_name} — expected {expected_iscc}, got {}",
                        result.iscc
                    );
                    Some(false)
                }
                Err(e) => {
                    eprintln!("FAILED: {func_name}.{tc_name} — error: {e}");
                    Some(false)
                }
            }
        })();

        match result {
            Some(true) => {}
            Some(false) => passed = false,
            None => {
                eprintln!("FAILED: {func_name}.{tc_name} — could not parse test inputs");
                passed = false;
            }
        }
    }
    passed
}

/// Decode a `"stream:<hex>"` value into bytes.
fn decode_stream(s: &str) -> Option<Vec<u8>> {
    let hex_data = s.strip_prefix("stream:")?;
    hex::decode(hex_data).ok()
}

/// Run conformance tests for `gen_data_code_v0`.
fn run_data_tests(data: &serde_json::Value) -> bool {
    let mut passed = true;
    let section = &data["gen_data_code_v0"];
    let cases = match section.as_object() {
        Some(c) => c,
        None => {
            eprintln!("FAILED: gen_data_code_v0 section missing from conformance data");
            return false;
        }
    };

    for (tc_name, tc) in cases {
        let func_name = "gen_data_code_v0";
        let result = (|| {
            let inputs = tc["inputs"].as_array()?;
            let stream_str = inputs[0].as_str()?;
            let bits = inputs[1].as_u64()? as u32;
            let expected_iscc = tc["outputs"]["iscc"].as_str()?;
            let input_bytes = decode_stream(stream_str)?;

            match gen_data_code_v0(&input_bytes, bits) {
                Ok(result) if result.iscc == expected_iscc => Some(true),
                Ok(result) => {
                    eprintln!(
                        "FAILED: {func_name}.{tc_name} — expected {expected_iscc}, got {}",
                        result.iscc
                    );
                    Some(false)
                }
                Err(e) => {
                    eprintln!("FAILED: {func_name}.{tc_name} — error: {e}");
                    Some(false)
                }
            }
        })();

        match result {
            Some(true) => {}
            Some(false) => passed = false,
            None => {
                eprintln!("FAILED: {func_name}.{tc_name} — could not parse test inputs");
                passed = false;
            }
        }
    }
    passed
}

/// Run conformance tests for `gen_instance_code_v0`.
fn run_instance_tests(data: &serde_json::Value) -> bool {
    let mut passed = true;
    let section = &data["gen_instance_code_v0"];
    let cases = match section.as_object() {
        Some(c) => c,
        None => {
            eprintln!("FAILED: gen_instance_code_v0 section missing from conformance data");
            return false;
        }
    };

    for (tc_name, tc) in cases {
        let func_name = "gen_instance_code_v0";
        let result = (|| {
            let inputs = tc["inputs"].as_array()?;
            let stream_str = inputs[0].as_str()?;
            let bits = inputs[1].as_u64()? as u32;
            let expected_iscc = tc["outputs"]["iscc"].as_str()?;
            let input_bytes = decode_stream(stream_str)?;

            match gen_instance_code_v0(&input_bytes, bits) {
                Ok(result) if result.iscc == expected_iscc => Some(true),
                Ok(result) => {
                    eprintln!(
                        "FAILED: {func_name}.{tc_name} — expected {expected_iscc}, got {}",
                        result.iscc
                    );
                    Some(false)
                }
                Err(e) => {
                    eprintln!("FAILED: {func_name}.{tc_name} — error: {e}");
                    Some(false)
                }
            }
        })();

        match result {
            Some(true) => {}
            Some(false) => passed = false,
            None => {
                eprintln!("FAILED: {func_name}.{tc_name} — could not parse test inputs");
                passed = false;
            }
        }
    }
    passed
}

/// Run conformance tests for `gen_iscc_code_v0`.
fn run_iscc_tests(data: &serde_json::Value) -> bool {
    let mut passed = true;
    let section = &data["gen_iscc_code_v0"];
    let cases = match section.as_object() {
        Some(c) => c,
        None => {
            eprintln!("FAILED: gen_iscc_code_v0 section missing from conformance data");
            return false;
        }
    };

    for (tc_name, tc) in cases {
        let func_name = "gen_iscc_code_v0";
        let result = (|| {
            let inputs = tc["inputs"].as_array()?;
            let codes_json = inputs[0].as_array()?;
            let expected_iscc = tc["outputs"]["iscc"].as_str()?;

            let codes_owned: Vec<String> = codes_json
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();
            let codes: Vec<&str> = codes_owned.iter().map(|s| s.as_str()).collect();

            // Conformance vectors use default (non-wide) mode
            match gen_iscc_code_v0(&codes, false) {
                Ok(result) if result.iscc == expected_iscc => Some(true),
                Ok(result) => {
                    eprintln!(
                        "FAILED: {func_name}.{tc_name} — expected {expected_iscc}, got {}",
                        result.iscc
                    );
                    Some(false)
                }
                Err(e) => {
                    eprintln!("FAILED: {func_name}.{tc_name} — error: {e}");
                    Some(false)
                }
            }
        })();

        match result {
            Some(true) => {}
            Some(false) => passed = false,
            None => {
                eprintln!("FAILED: {func_name}.{tc_name} — could not parse test inputs");
                passed = false;
            }
        }
    }
    passed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conformance_selftest_passes() {
        assert!(conformance_selftest());
    }
}
