//! Shared edge-case vectors (`crates/iscc-lib/tests/data-edge.json`) run against the WASM bindings.
//!
//! Every vector runs, `except.wasm` is the only deviation, and a function section
//! this harness does not handle fails. A rejection is asserted on the raw call
//! alone (an `Err`); the result is converted to its fixture form only after a
//! successful call.
//!
//! Runs in a Node.js WASM runtime via `wasm-pack test --node`.

use serde_json::{Value, json};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

/// Shared edge-case vectors loaded at compile time.
const DATA_EDGE_JSON: &str = include_str!("../../iscc-lib/tests/data-edge.json");

/// Surface id of this harness in the `except` map.
const SURFACE: &str = "wasm";

/// Result of calling a binding function: the converted output, or the error text.
type Outcome = Result<Value, String>;

/// Runs one section's function on a vector's inputs.
///
/// The outer `Err` is a harness failure (malformed inputs); the inner
/// [`Outcome`] is what the binding returned.
type Caller = fn(&[Value]) -> Result<Outcome, String>;

/// Function sections this harness handles, each with its caller.
const SECTIONS: &[(&str, Caller)] = &[
    ("iscc_decode", call_decode),
    ("encode_seq", call_seq),
    ("gen_text_code_v0", call_text),
    ("gen_mixed_code_v0", call_mixed),
    ("gen_iscc_code_v0", call_iscc_code),
    ("alg_simhash", call_simhash),
];

/// Parse the embedded `data-edge.json`.
fn load_doc() -> Value {
    serde_json::from_str(DATA_EDGE_JSON).expect("parse data-edge.json")
}

/// Encode bytes as a `bytes:<hex>` fixture value.
fn bytes_value(data: &[u8]) -> Value {
    Value::String(format!("bytes:{}", hex::encode(data)))
}

/// Convert a raw binding result into an [`Outcome`]; `encode` runs only on success.
fn to_outcome<T>(result: Result<T, wasm_bindgen::JsError>, encode: fn(T) -> Value) -> Outcome {
    match result {
        Ok(value) => Ok(encode(value)),
        Err(e) => Err(format!("{e:?}")),
    }
}

/// Read the input at `index` as a string.
fn str_arg(inputs: &[Value], index: usize) -> Result<&str, String> {
    inputs
        .get(index)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("input {index} is not a string"))
}

/// Read the input at `index` as a list of strings, converted to a JS array.
fn str_list_arg(inputs: &[Value], index: usize) -> Result<JsValue, String> {
    let items: Vec<String> = inputs
        .get(index)
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| format!("input {index} is not a list of strings"))?;
    serde_wasm_bindgen::to_value(&items).map_err(|e| format!("input {index}: {e}"))
}

/// Decode one `bytes:<hex>` list item into bytes.
fn bytes_item(item: &Value, index: usize) -> Result<Vec<u8>, String> {
    item.as_str()
        .and_then(|s| s.strip_prefix("bytes:"))
        .and_then(|h| hex::decode(h).ok())
        .ok_or_else(|| format!("input {index} holds a non-`bytes:<hex>` item"))
}

/// Read the input at `index` as a list of `bytes:<hex>` digests, converted to a JS array.
fn bytes_list_arg(inputs: &[Value], index: usize) -> Result<JsValue, String> {
    let items = inputs
        .get(index)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("input {index} is not a list"))?;
    let digests = items
        .iter()
        .map(|v| bytes_item(v, index))
        .collect::<Result<Vec<_>, _>>()?;
    serde_wasm_bindgen::to_value(&digests).map_err(|e| format!("input {index}: {e}"))
}

/// Read the input at `index` as a `u32`.
fn u32_arg(inputs: &[Value], index: usize) -> Result<u32, String> {
    inputs
        .get(index)
        .and_then(Value::as_u64)
        .and_then(|n| u32::try_from(n).ok())
        .ok_or_else(|| format!("input {index} is not a u32"))
}

/// Encode an `iscc_decode` result as `[mt, st, vs, li, "bytes:<hex>"]`.
fn decode_json(r: iscc_wasm::IsccDecodeResult) -> Value {
    json!([
        r.maintype,
        r.subtype,
        r.version,
        r.length,
        bytes_value(&r.digest)
    ])
}

/// Encode a bare ISCC string result (gen functions) as an `{iscc}` object.
fn iscc_json(iscc: String) -> Value {
    json!({ "iscc": iscc })
}

/// Encode `encode_seq` or `alg_simhash` bytes as a `bytes:<hex>` fixture value.
fn seq_json(data: Vec<u8>) -> Value {
    bytes_value(&data)
}

/// Run `iscc_decode` on the vector's string input.
fn call_decode(inputs: &[Value]) -> Result<Outcome, String> {
    let iscc = str_arg(inputs, 0)?;
    Ok(to_outcome(iscc_wasm::iscc_decode(iscc), decode_json))
}

/// Run `encode_seq` on the vector's unit list.
fn call_seq(inputs: &[Value]) -> Result<Outcome, String> {
    let units = str_list_arg(inputs, 0)?;
    Ok(to_outcome(iscc_wasm::encode_seq(units), seq_json))
}

/// Run `gen_text_code_v0` on the vector's text and bits.
fn call_text(inputs: &[Value]) -> Result<Outcome, String> {
    let text = str_arg(inputs, 0)?;
    let bits = u32_arg(inputs, 1)?;
    Ok(to_outcome(
        iscc_wasm::gen_text_code_v0(text, Some(bits)),
        iscc_json,
    ))
}

/// Run `gen_mixed_code_v0` on the vector's codes and bits.
fn call_mixed(inputs: &[Value]) -> Result<Outcome, String> {
    let codes = str_list_arg(inputs, 0)?;
    let bits = u32_arg(inputs, 1)?;
    Ok(to_outcome(
        iscc_wasm::gen_mixed_code_v0(codes, Some(bits)),
        iscc_json,
    ))
}

/// Run `gen_iscc_code_v0` (the optional second input is `wide`, default false).
fn call_iscc_code(inputs: &[Value]) -> Result<Outcome, String> {
    let codes = str_list_arg(inputs, 0)?;
    let wide = inputs.get(1).and_then(Value::as_bool);
    Ok(to_outcome(
        iscc_wasm::gen_iscc_code_v0(codes, wide),
        iscc_json,
    ))
}

/// Run `alg_simhash` on the vector's digest list.
fn call_simhash(inputs: &[Value]) -> Result<Outcome, String> {
    let digests = bytes_list_arg(inputs, 0)?;
    Ok(to_outcome(iscc_wasm::alg_simhash(digests), seq_json))
}

/// Look up the caller of a function section; `None` for an unknown section.
fn find_caller(section: &str) -> Option<Caller> {
    SECTIONS
        .iter()
        .find(|(name, _)| *name == section)
        .map(|(_, caller)| *caller)
}

/// Expected output keys each section's WASM result does not carry, which are
/// therefore not compared (an absent section compares every key). The gen
/// functions here return a bare ISCC string, so text drops `characters` and
/// mixed drops `parts`.
const UNCOMPARED: &[(&str, &[&str])] = &[
    ("gen_text_code_v0", &["characters"]),
    ("gen_mixed_code_v0", &["parts"]),
];

/// Look up the static uncompared output keys of a section (empty if none).
fn uncompared(section: &str) -> &'static [&'static str] {
    UNCOMPARED
        .iter()
        .find(|(name, _)| *name == section)
        .map_or(&[], |(_, keys)| keys)
}

/// Compare an actual output with the expected one.
///
/// For an object, every expected key outside the section's static uncompared
/// set must be present in the result and equal; a missing key fails. Any other
/// value must be equal as a whole. Returns a failure description on mismatch.
fn compare(section: &str, expected: &Value, actual: &Value) -> Option<String> {
    let Some(fields) = expected.as_object() else {
        return (expected != actual).then(|| format!("expected {expected}, got {actual}"));
    };
    let skip = uncompared(section);
    fields
        .iter()
        .filter(|(key, _)| !skip.contains(&key.as_str()))
        .find_map(|(key, want)| match actual.get(key) {
            None => Some(format!("missing field {key} in {actual}")),
            Some(got) => (want != got).then(|| format!("field {key}: expected {want}, got {got}")),
        })
}

/// Return this surface's `except` entry of a vector, if any.
fn exception(vector: &Value) -> Option<&str> {
    vector
        .get("except")
        .and_then(|e| e.get(SURFACE))
        .and_then(Value::as_str)
}

/// Return true if the vector expects this surface to reject its inputs.
fn expects_error(vector: &Value) -> bool {
    exception(vector) == Some("error") || vector.get("error").is_some()
}

/// Run a vector's inputs through `caller` (outer `Err` = harness failure).
fn run_vector(caller: Caller, vector: &Value) -> Result<Outcome, String> {
    let inputs = vector
        .get("inputs")
        .and_then(Value::as_array)
        .ok_or("inputs is missing or not a list")?;
    caller(inputs)
}

/// Compare a successful output with the vector's `outputs`.
fn judge_output(section: &str, vector: &Value, got: &Value) -> Option<String> {
    match vector.get("outputs") {
        Some(want) => compare(section, want, got),
        None => Some("vector has neither outputs nor error".into()),
    }
}

/// Judge a binding outcome against the vector's expectation.
fn judge(section: &str, expect_error: bool, outcome: Outcome, vector: &Value) -> Option<String> {
    match (expect_error, outcome) {
        (true, Ok(got)) => Some(format!("expected an error, got {got}")),
        (true, Err(_)) => None,
        (false, Ok(got)) => judge_output(section, vector, &got),
        (false, Err(e)) => Some(format!("expected an output, got error: {e}")),
    }
}

/// Check one vector and return a failure description, if any.
fn check_vector(section: &str, caller: Caller, vector: &Value) -> Option<String> {
    if exception(vector) == Some("skip") {
        return None;
    }
    match run_vector(caller, vector) {
        Ok(outcome) => judge(section, expects_error(vector), outcome, vector),
        Err(e) => Some(e),
    }
}

/// Check every vector of one section; an unknown section fails as a whole.
fn check_section(section: &str, vectors: &Value) -> Vec<String> {
    let Some(caller) = find_caller(section) else {
        return vec![format!("{section}: unknown section")];
    };
    let Some(vectors) = vectors.as_object() else {
        return vec![format!("{section}: section is not an object")];
    };
    vectors
        .iter()
        .filter_map(|(name, vector)| {
            check_vector(section, caller, vector)
                .map(|reason| format!("{section}/{name}: {reason}"))
        })
        .collect()
}

/// Run every vector in a parsed `data-edge.json` document.
///
/// Returns one `section/vector: reason` line per failure; an empty list means
/// every vector passed. An unknown section fails as a whole.
fn check_doc(doc: &Value) -> Vec<String> {
    let Some(sections) = doc.as_object() else {
        return vec!["document is not an object".into()];
    };
    sections
        .iter()
        .flat_map(|(section, vectors)| check_section(section, vectors))
        .collect()
}

#[wasm_bindgen_test]
fn test_data_edge_vectors_pass() {
    let doc = load_doc();
    for (section, _) in SECTIONS {
        let count = doc[section].as_object().map_or(0, |v| v.len());
        assert!(count > 0, "section {section} has no vectors");
    }
    let failures = check_doc(&doc);
    assert!(
        failures.is_empty(),
        "data-edge.json failures:\n{}",
        failures.join("\n")
    );
}

#[wasm_bindgen_test]
fn test_data_edge_fails_unknown_section() {
    let doc = json!({"gen_unknown_v0": {"test_0001_x": {
        "inputs": ["x"], "outputs": "x", "note": "n"
    }}});
    let failures = check_doc(&doc);
    assert_eq!(failures.len(), 1, "{failures:?}");
    assert!(failures[0].contains("unknown section"), "{failures:?}");
}

#[wasm_bindgen_test]
fn test_data_edge_fails_expected_error_that_succeeds() {
    // Valid inputs marked `error` prove each rejection-only caller reaches the binding.
    let doc = json!({
        "gen_mixed_code_v0": {"test_0001_x": {
            "inputs": [["EUA6GIKXN42IQV3S", "EIAUKMOUIOYZCKA5"], 64], "error": "e", "note": "n"
        }},
        "gen_iscc_code_v0": {"test_0001_x": {
            "inputs": [["GABVVC5DMJJGYKZ4ZBYVNYABFFYXG",
                "IADWIK7A7JTUAQ2D6QARX7OBEIK3OOUAM42LOBLCZ4ZOGDLRHMDL6TQ"]],
            "error": "e", "note": "n"
        }}
    });
    let failures = check_doc(&doc);
    assert_eq!(failures.len(), 2, "{failures:?}");
    assert!(
        failures.iter().all(|f| f.contains("expected an error")),
        "{failures:?}"
    );
}

#[wasm_bindgen_test]
fn test_data_edge_fails_output_mismatch() {
    let doc = json!({"encode_seq": {"test_0001_x": {
        "inputs": [["ISCC:GAAAAAAAAA"]], "outputs": "bytes:300000000001", "note": "n"
    }}});
    let failures = check_doc(&doc);
    assert_eq!(failures.len(), 1, "{failures:?}");
    assert!(
        failures[0].contains("expected \"bytes:300000000001\""),
        "{failures:?}"
    );
}

#[wasm_bindgen_test]
fn test_data_edge_honours_except() {
    let doc = json!({"iscc_decode": {
        "test_0001_error": {
            "inputs": ["ISCC:GAAAAAAAAA"], "outputs": [3, 0, 0, 0, "bytes:00000000"],
            "note": "n", "except": {"wasm": "error"}
        },
        "test_0002_skip": {
            "inputs": ["ISCC:GAAAAAAAAB"], "outputs": [3, 0, 0, 0, "bytes:00000000"],
            "note": "n", "except": {"wasm": "skip"}
        }
    }});
    let failures = check_doc(&doc);
    assert_eq!(failures.len(), 1, "{failures:?}");
    assert!(
        failures[0].starts_with("iscc_decode/test_0001_error: expected an error"),
        "{failures:?}"
    );
}

#[wasm_bindgen_test]
fn test_data_edge_compares_object_output() {
    // The data.json Hello World vector: `characters` is uncompared, `iscc` matches.
    let doc = json!({"gen_text_code_v0": {"test_0001_x": {
        "inputs": ["Hello World", 64],
        "outputs": {"iscc": "ISCC:EAASKDNZNYGUUF5A", "characters": 10}, "note": "n"
    }}});
    let failures = check_doc(&doc);
    assert!(failures.is_empty(), "{failures:?}");
}

#[wasm_bindgen_test]
fn test_data_edge_fails_key_outside_uncompared_set() {
    // `parts` is uncompared for mixed only, so a text result lacking it fails.
    let doc = json!({"gen_text_code_v0": {"test_0001_x": {
        "inputs": ["Hello World", 64],
        "outputs": {"iscc": "ISCC:EAASKDNZNYGUUF5A", "parts": []}, "note": "n"
    }}});
    let failures = check_doc(&doc);
    assert_eq!(failures.len(), 1, "{failures:?}");
    assert!(failures[0].contains("missing field parts"), "{failures:?}");
}

#[wasm_bindgen_test]
fn test_data_edge_fails_result_without_iscc() {
    let failure = compare(
        "gen_text_code_v0",
        &json!({"iscc": "ISCC:EAASKDNZNYGUUF5A", "characters": 10}),
        &json!({"characters": 10}),
    );
    let failure = failure.expect("a result without iscc must fail");
    assert!(failure.contains("missing field iscc"), "{failure}");
}
