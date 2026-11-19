//! Shared edge-case vectors (`tests/data-edge.json`) run against the Rust core.
//!
//! The file holds JSON-expressible edge-case inputs with their expected outcome
//! (`outputs` or `error`). This harness runs every vector, honours `except.rust`,
//! and fails on any function section it does not handle. The file is excluded
//! from the published crate, so it is read at runtime from the source tree.

use serde_json::{Value, json};

/// Surface id of this harness in the `except` map.
const SURFACE: &str = "rust";

/// Function sections this harness knows how to call.
const KNOWN_SECTIONS: &[&str] = &[
    "iscc_decode",
    "encode_seq",
    "gen_text_code_v0",
    "gen_mixed_code_v0",
    "gen_iscc_code_v0",
];

/// Result of calling a library function: the output as JSON, or the error text.
type Outcome = Result<Value, String>;

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

/// Read the input at `index` as a string.
fn str_arg(inputs: &[Value], index: usize) -> Result<&str, String> {
    inputs
        .get(index)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("input {index} is not a string"))
}

/// Read the input at `index` as a list of strings.
fn str_list_arg(inputs: &[Value], index: usize) -> Result<Vec<&str>, String> {
    let items = inputs
        .get(index)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("input {index} is not a list"))?;
    items
        .iter()
        .map(|v| {
            v.as_str()
                .ok_or_else(|| format!("input {index} holds a non-string"))
        })
        .collect()
}

/// Read the input at `index` as a `u32`.
fn u32_arg(inputs: &[Value], index: usize) -> Result<u32, String> {
    inputs
        .get(index)
        .and_then(Value::as_u64)
        .and_then(|n| u32::try_from(n).ok())
        .ok_or_else(|| format!("input {index} is not a u32"))
}

/// Run `gen_text_code_v0` and encode its result fields as JSON.
#[cfg(feature = "text-processing")]
fn call_text(inputs: &[Value]) -> Result<Outcome, String> {
    let text = str_arg(inputs, 0)?;
    let bits = u32_arg(inputs, 1)?;
    Ok(iscc_lib::gen_text_code_v0(text, bits)
        .map(|r| json!({"iscc": r.iscc, "characters": r.characters}))
        .map_err(|e| e.to_string()))
}

/// Run a vector's inputs through the function its section names.
///
/// The outer `Err` is a harness failure (unknown section or malformed inputs);
/// the inner [`Outcome`] is what the library returned. Returns `Ok(None)` when
/// the section's function is compiled out by a disabled feature flag.
fn call(section: &str, inputs: &[Value]) -> Result<Option<Outcome>, String> {
    let outcome = match section {
        "iscc_decode" => iscc_lib::iscc_decode(str_arg(inputs, 0)?)
            .map(|(mt, st, vs, li, digest)| json!([mt, st, vs, li, bytes_value(&digest)]))
            .map_err(|e| e.to_string()),
        "encode_seq" => iscc_lib::encode_seq(&str_list_arg(inputs, 0)?)
            .map(|data| bytes_value(&data))
            .map_err(|e| e.to_string()),
        #[cfg(feature = "text-processing")]
        "gen_text_code_v0" => call_text(inputs)?,
        #[cfg(not(feature = "text-processing"))]
        "gen_text_code_v0" => return Ok(None),
        "gen_mixed_code_v0" => {
            iscc_lib::gen_mixed_code_v0(&str_list_arg(inputs, 0)?, u32_arg(inputs, 1)?)
                .map(|r| json!({"iscc": r.iscc, "parts": r.parts}))
                .map_err(|e| e.to_string())
        }
        "gen_iscc_code_v0" => {
            let wide = inputs.get(1).and_then(Value::as_bool).unwrap_or(false);
            iscc_lib::gen_iscc_code_v0(&str_list_arg(inputs, 0)?, wide)
                .map(|r| json!({"iscc": r.iscc}))
                .map_err(|e| e.to_string())
        }
        _ => return Err(format!("unknown section {section}")),
    };
    Ok(Some(outcome))
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

/// Check one vector and return a failure description, if any.
fn check_vector(section: &str, vector: &Value) -> Option<String> {
    let exception = vector
        .get("except")
        .and_then(|e| e.get(SURFACE))
        .and_then(Value::as_str);
    if exception == Some("skip") {
        return None;
    }
    let Some(inputs) = vector.get("inputs").and_then(Value::as_array) else {
        return Some("inputs is missing or not a list".into());
    };
    let outcome = match call(section, inputs) {
        Ok(Some(outcome)) => outcome,
        Ok(None) => return None,
        Err(e) => return Some(e),
    };
    let expect_error = exception == Some("error") || vector.get("error").is_some();
    match (expect_error, outcome) {
        (true, Err(_)) => None,
        (true, Ok(got)) => Some(format!("expected an error, got {got}")),
        (false, Err(e)) => Some(format!("expected an output, got error: {e}")),
        (false, Ok(got)) => match vector.get("outputs") {
            Some(want) => compare(want, &got),
            None => Some("vector has neither outputs nor error".into()),
        },
    }
}

/// Run every vector in a parsed `data-edge.json` document.
///
/// Returns one `section/vector: reason` line per failure; an empty list means
/// every vector passed. An unknown section fails as a whole.
fn check_doc(doc: &Value) -> Vec<String> {
    let Some(sections) = doc.as_object() else {
        return vec!["document is not an object".into()];
    };
    let mut failures = Vec::new();
    for (section, vectors) in sections {
        if !KNOWN_SECTIONS.contains(&section.as_str()) {
            failures.push(format!("{section}: unknown section"));
            continue;
        }
        let Some(vectors) = vectors.as_object() else {
            failures.push(format!("{section}: section is not an object"));
            continue;
        };
        for (name, vector) in vectors {
            if let Some(reason) = check_vector(section, vector) {
                failures.push(format!("{section}/{name}: {reason}"));
            }
        }
    }
    failures
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
    for section in KNOWN_SECTIONS {
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
