//! Shared edge-case vectors (`tests/data-edge.json`) run against the Rust core.
//!
//! The file holds JSON-expressible edge-case inputs with their expected outcome
//! (`outputs` or `error`). This harness runs every vector, honours `except.rust`,
//! and fails on any function section it does not handle. The file is excluded
//! from the published crate, so it is read at runtime from the source tree.
//!
//! Every helper stays at a low cyclomatic complexity: integration-test code has
//! no coverage in the CI CRAP gate, so its score is complexity alone.

use serde_json::{Value, json};

/// Surface id of this harness in the `except` map.
const SURFACE: &str = "rust";

/// Result of calling a library function: the output as JSON, or the error text.
type Outcome = Result<Value, String>;

/// Runs one section's function on a vector's inputs.
///
/// The outer `Err` is a harness failure (malformed inputs); the inner
/// [`Outcome`] is what the library returned. `Ok(None)` means the function is
/// compiled out by a disabled feature flag.
type Caller = fn(&[Value]) -> Result<Option<Outcome>, String>;

/// Function sections this harness handles, each with its caller.
const SECTIONS: &[(&str, Caller)] = &[
    ("iscc_decode", call_decode),
    ("encode_seq", call_seq),
    ("gen_text_code_v0", call_text),
    ("gen_mixed_code_v0", call_mixed),
    ("gen_iscc_code_v0", call_iscc_code),
    ("alg_simhash", call_simhash),
    ("gen_meta_code_v0", call_meta),
    ("gen_video_code_v0", call_video),
    ("alg_cdc_chunks", call_cdc),
];

/// Load and parse `tests/data-edge.json` from the crate source tree.
fn load_doc() -> Value {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data-edge.json");
    let text = std::fs::read_to_string(path).expect("read data-edge.json");
    serde_json::from_str(&text).expect("parse data-edge.json")
}

/// Encode bytes as a `bytes:<hex>` fixture value.
fn bytes_value(data: &[u8]) -> Value {
    Value::String(format!("bytes:{}", hex::encode(data)))
}

/// Convert a library result into an [`Outcome`], keeping the error text.
fn to_outcome<T, E: std::fmt::Display>(result: Result<T, E>, encode: fn(T) -> Value) -> Outcome {
    result.map(encode).map_err(|e| e.to_string())
}

/// Read the input at `index` as a string.
fn str_arg(inputs: &[Value], index: usize) -> Result<&str, String> {
    inputs
        .get(index)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("input {index} is not a string"))
}

/// Read one list item as a string.
fn str_item(item: &Value, index: usize) -> Result<&str, String> {
    item.as_str()
        .ok_or_else(|| format!("input {index} holds a non-string"))
}

/// Read the input at `index` as a list of strings.
fn str_list_arg(inputs: &[Value], index: usize) -> Result<Vec<&str>, String> {
    let items = inputs
        .get(index)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("input {index} is not a list"))?;
    items.iter().map(|v| str_item(v, index)).collect()
}

/// Decode one `bytes:<hex>` list item into bytes.
fn bytes_item(item: &Value, index: usize) -> Result<Vec<u8>, String> {
    item.as_str()
        .and_then(|s| s.strip_prefix("bytes:"))
        .and_then(|h| hex::decode(h).ok())
        .ok_or_else(|| format!("input {index} holds a non-`bytes:<hex>` item"))
}

/// Read the input at `index` as one `bytes:<hex>` byte string.
fn bytes_arg(inputs: &[Value], index: usize) -> Result<Vec<u8>, String> {
    inputs
        .get(index)
        .ok_or_else(|| format!("input {index} is missing"))
        .and_then(|v| bytes_item(v, index))
}

/// Read the input at `index` as a boolean.
fn bool_arg(inputs: &[Value], index: usize) -> Result<bool, String> {
    inputs
        .get(index)
        .and_then(Value::as_bool)
        .ok_or_else(|| format!("input {index} is not a boolean"))
}

/// Read the input at `index` as a list of `bytes:<hex>` byte strings.
fn bytes_list_arg(inputs: &[Value], index: usize) -> Result<Vec<Vec<u8>>, String> {
    let items = inputs
        .get(index)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("input {index} is not a list"))?;
    items.iter().map(|v| bytes_item(v, index)).collect()
}

/// Read the input at `index` as a `meta` argument: a JSON object is serialized,
/// a string is passed as is.
#[cfg(feature = "meta-code")]
fn meta_arg(inputs: &[Value], index: usize) -> Result<String, String> {
    match inputs.get(index) {
        Some(Value::String(s)) => Ok(s.clone()),
        Some(v @ Value::Object(_)) => Ok(v.to_string()),
        _ => Err(format!("input {index} is not a meta object or string")),
    }
}

/// Read one frame-signature item as an `i32`.
fn i32_item(item: &Value, index: usize) -> Result<i32, String> {
    item.as_i64()
        .and_then(|n| i32::try_from(n).ok())
        .ok_or_else(|| format!("input {index} holds a non-i32 frame value"))
}

/// Read one frame signature (a list of integers).
fn frame_item(item: &Value, index: usize) -> Result<Vec<i32>, String> {
    let values = item
        .as_array()
        .ok_or_else(|| format!("input {index} holds a non-list frame"))?;
    values.iter().map(|v| i32_item(v, index)).collect()
}

/// Read the input at `index` as a list of frame signatures.
fn frames_arg(inputs: &[Value], index: usize) -> Result<Vec<Vec<i32>>, String> {
    let items = inputs
        .get(index)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("input {index} is not a list"))?;
    items.iter().map(|v| frame_item(v, index)).collect()
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
fn decode_json((mt, st, vs, li, digest): (u8, u8, u8, u8, Vec<u8>)) -> Value {
    json!([mt, st, vs, li, bytes_value(&digest)])
}

/// Run `iscc_decode` on the vector's string input.
fn call_decode(inputs: &[Value]) -> Result<Option<Outcome>, String> {
    let iscc = str_arg(inputs, 0)?;
    Ok(Some(to_outcome(iscc_lib::iscc_decode(iscc), decode_json)))
}

/// Run `encode_seq` on the vector's unit list.
fn call_seq(inputs: &[Value]) -> Result<Option<Outcome>, String> {
    let units = str_list_arg(inputs, 0)?;
    let encode = |data: Vec<u8>| bytes_value(&data);
    Ok(Some(to_outcome(iscc_lib::encode_seq(&units), encode)))
}

/// Run `gen_text_code_v0` and encode its result fields as JSON.
#[cfg(feature = "text-processing")]
fn call_text(inputs: &[Value]) -> Result<Option<Outcome>, String> {
    let text = str_arg(inputs, 0)?;
    let bits = u32_arg(inputs, 1)?;
    let encode = |r: iscc_lib::TextCodeResult| json!({"iscc": r.iscc, "characters": r.characters});
    Ok(Some(to_outcome(
        iscc_lib::gen_text_code_v0(text, bits),
        encode,
    )))
}

/// Report `gen_text_code_v0` as compiled out (no `text-processing` feature).
#[cfg(not(feature = "text-processing"))]
fn call_text(_inputs: &[Value]) -> Result<Option<Outcome>, String> {
    Ok(None)
}

/// Run `gen_mixed_code_v0` and encode its result fields as JSON.
fn call_mixed(inputs: &[Value]) -> Result<Option<Outcome>, String> {
    let codes = str_list_arg(inputs, 0)?;
    let bits = u32_arg(inputs, 1)?;
    let encode = |r: iscc_lib::MixedCodeResult| json!({"iscc": r.iscc, "parts": r.parts});
    Ok(Some(to_outcome(
        iscc_lib::gen_mixed_code_v0(&codes, bits),
        encode,
    )))
}

/// Run `gen_iscc_code_v0` (the optional second input is `wide`, default false).
fn call_iscc_code(inputs: &[Value]) -> Result<Option<Outcome>, String> {
    let codes = str_list_arg(inputs, 0)?;
    let wide = inputs.get(1).and_then(Value::as_bool).unwrap_or(false);
    let encode = |r: iscc_lib::IsccCodeResult| json!({"iscc": r.iscc});
    Ok(Some(to_outcome(
        iscc_lib::gen_iscc_code_v0(&codes, wide),
        encode,
    )))
}

/// Run `alg_simhash` on the vector's digest list.
fn call_simhash(inputs: &[Value]) -> Result<Option<Outcome>, String> {
    let digests = bytes_list_arg(inputs, 0)?;
    let encode = |data: Vec<u8>| bytes_value(&data);
    Ok(Some(to_outcome(iscc_lib::alg_simhash(&digests), encode)))
}

/// Encode a `gen_meta_code_v0` result as JSON (absent options become `null`).
#[cfg(feature = "meta-code")]
fn meta_json(r: iscc_lib::MetaCodeResult) -> Value {
    json!({
        "iscc": r.iscc,
        "name": r.name,
        "description": r.description,
        "meta": r.meta,
        "metahash": r.metahash,
    })
}

/// Run `gen_meta_code_v0` (an empty description means none).
#[cfg(feature = "meta-code")]
fn call_meta(inputs: &[Value]) -> Result<Option<Outcome>, String> {
    let name = str_arg(inputs, 0)?;
    let description = Some(str_arg(inputs, 1)?).filter(|d| !d.is_empty());
    let meta = meta_arg(inputs, 2)?;
    let bits = u32_arg(inputs, 3)?;
    Ok(Some(to_outcome(
        iscc_lib::gen_meta_code_v0(name, description, Some(&meta), bits),
        meta_json,
    )))
}

/// Report `gen_meta_code_v0` as compiled out (no `meta-code` feature).
#[cfg(not(feature = "meta-code"))]
fn call_meta(_inputs: &[Value]) -> Result<Option<Outcome>, String> {
    Ok(None)
}

/// Run `gen_video_code_v0` on the vector's frame signatures.
fn call_video(inputs: &[Value]) -> Result<Option<Outcome>, String> {
    let frames = frames_arg(inputs, 0)?;
    let bits = u32_arg(inputs, 1)?;
    let encode = |r: iscc_lib::VideoCodeResult| json!({"iscc": r.iscc});
    Ok(Some(to_outcome(
        iscc_lib::gen_video_code_v0(&frames, bits),
        encode,
    )))
}

/// Encode `alg_cdc_chunks` chunks as a list of `bytes:<hex>` values.
fn chunks_json(chunks: Vec<&[u8]>) -> Value {
    Value::Array(chunks.into_iter().map(bytes_value).collect())
}

/// Run `alg_cdc_chunks` on the vector's data, `utf32` flag and average size.
fn call_cdc(inputs: &[Value]) -> Result<Option<Outcome>, String> {
    let data = bytes_arg(inputs, 0)?;
    let utf32 = bool_arg(inputs, 1)?;
    let avg = u32_arg(inputs, 2)?;
    Ok(Some(to_outcome(
        iscc_lib::alg_cdc_chunks(&data, utf32, avg),
        chunks_json,
    )))
}

/// Look up the caller of a function section; `None` for an unknown section.
fn find_caller(section: &str) -> Option<Caller> {
    SECTIONS
        .iter()
        .find(|(name, _)| *name == section)
        .map(|(_, caller)| *caller)
}

/// Compare an actual output with the expected one.
///
/// For an object, only the keys present in `expected` are compared; any other
/// value must be equal as a whole. Returns a failure description on mismatch.
fn compare(expected: &Value, actual: &Value) -> Option<String> {
    let Some(fields) = expected.as_object() else {
        return (expected != actual).then(|| format!("expected {expected}, got {actual}"));
    };
    fields.iter().find_map(|(key, want)| match actual.get(key) {
        Some(got) if got == want => None,
        got => Some(format!("field {key}: expected {want}, got {got:?}")),
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
fn run_vector(caller: Caller, vector: &Value) -> Result<Option<Outcome>, String> {
    let inputs = vector
        .get("inputs")
        .and_then(Value::as_array)
        .ok_or("inputs is missing or not a list")?;
    caller(inputs)
}

/// Compare a successful output with the vector's `outputs`.
fn judge_output(vector: &Value, got: &Value) -> Option<String> {
    match vector.get("outputs") {
        Some(want) => compare(want, got),
        None => Some("vector has neither outputs nor error".into()),
    }
}

/// Judge a successful output: a failure if an error was expected, else compare.
fn judge_ok(expect_error: bool, got: &Value, vector: &Value) -> Option<String> {
    if expect_error {
        Some(format!("expected an error, got {got}"))
    } else {
        judge_output(vector, got)
    }
}

/// Judge a library error: a failure unless an error was expected.
fn judge_err(expect_error: bool, error: &str) -> Option<String> {
    (!expect_error).then(|| format!("expected an output, got error: {error}"))
}

/// Judge a library outcome against the vector's expectation.
fn judge(expect_error: bool, outcome: Outcome, vector: &Value) -> Option<String> {
    match outcome {
        Ok(got) => judge_ok(expect_error, &got, vector),
        Err(e) => judge_err(expect_error, &e),
    }
}

/// Check one vector and return a failure description, if any.
///
/// A compiled-out function (`Ok(None)` from the caller) passes silently.
fn check_vector(caller: Caller, vector: &Value) -> Option<String> {
    if exception(vector) == Some("skip") {
        return None;
    }
    match run_vector(caller, vector) {
        Ok(outcome) => outcome.and_then(|o| judge(expects_error(vector), o, vector)),
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
            check_vector(caller, vector).map(|reason| format!("{section}/{name}: {reason}"))
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

#[test]
fn test_data_edge_vectors_pass() {
    let doc = load_doc();
    let failures = check_doc(&doc);
    assert!(
        failures.is_empty(),
        "data-edge.json failures:\n{}",
        failures.join("\n")
    );
}

#[test]
fn test_data_edge_covers_every_known_section() {
    let doc = load_doc();
    for (section, _) in SECTIONS {
        let count = doc[section].as_object().map_or(0, |v| v.len());
        assert!(count > 0, "section {section} has no vectors");
    }
}

#[test]
fn test_check_doc_fails_unknown_section() {
    let doc = json!({"gen_unknown_v0": {"test_0001_x": {
        "inputs": ["x"], "outputs": "x", "note": "n"
    }}});
    let failures = check_doc(&doc);
    assert_eq!(failures.len(), 1, "{failures:?}");
    assert!(failures[0].contains("unknown section"), "{failures:?}");
}

#[test]
fn test_check_doc_fails_output_mismatch() {
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

#[test]
fn test_check_doc_fails_expected_error_that_succeeds() {
    let doc = json!({"iscc_decode": {"test_0001_x": {
        "inputs": ["ISCC:GAAAAAAAAA"], "error": "e", "note": "n"
    }}});
    let failures = check_doc(&doc);
    assert_eq!(failures.len(), 1, "{failures:?}");
    assert!(failures[0].contains("expected an error"), "{failures:?}");
}

#[test]
fn test_check_doc_fails_expected_output_that_errors() {
    let doc = json!({"iscc_decode": {"test_0001_x": {
        "inputs": ["ISCC:GAAAAAAAAB"], "outputs": [3, 0, 0, 0, "bytes:00000000"], "note": "n"
    }}});
    let failures = check_doc(&doc);
    assert_eq!(failures.len(), 1, "{failures:?}");
    assert!(failures[0].contains("expected an output"), "{failures:?}");
}

#[test]
fn test_check_doc_honours_except() {
    let doc = json!({"iscc_decode": {
        "test_0001_error": {
            "inputs": ["ISCC:GAAAAAAAAA"], "outputs": [3, 0, 0, 0, "bytes:00000000"],
            "note": "n", "except": {"rust": "error"}
        },
        "test_0002_skip": {
            "inputs": ["ISCC:GAAAAAAAAB"], "outputs": [3, 0, 0, 0, "bytes:00000000"],
            "note": "n", "except": {"rust": "skip"}
        }
    }});
    let failures = check_doc(&doc);
    assert_eq!(failures.len(), 1, "{failures:?}");
    assert!(failures[0].starts_with("iscc_decode/test_0001_error: expected an error"));
}
