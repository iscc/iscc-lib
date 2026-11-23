//! Strict vector readers shared by the WASM conformance and data-edge harnesses.
//!
//! Each reader takes a vector's positional `inputs` list and returns an `Err`
//! naming the input for a missing value, a wrong JSON type or an out-of-range
//! number. No reader casts, truncates or coerces a value.

use serde_json::Value;
use wasm_bindgen::JsValue;

/// Read the input at `index` as a string.
pub fn str_arg(inputs: &[Value], index: usize) -> Result<&str, String> {
    inputs
        .get(index)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("input {index} is not a string"))
}

/// Read the input at `index` as a list of strings, converted to a JS array.
pub fn str_list_arg(inputs: &[Value], index: usize) -> Result<JsValue, String> {
    let items: Vec<String> = inputs
        .get(index)
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| format!("input {index} is not a list of strings"))?;
    serde_wasm_bindgen::to_value(&items).map_err(|e| format!("input {index}: {e}"))
}

/// Decode one `bytes:<hex>` list item into bytes.
pub fn bytes_item(item: &Value, index: usize) -> Result<Vec<u8>, String> {
    item.as_str()
        .and_then(|s| s.strip_prefix("bytes:"))
        .and_then(|h| hex::decode(h).ok())
        .ok_or_else(|| format!("input {index} holds a non-`bytes:<hex>` item"))
}

/// Read the input at `index` as one `bytes:<hex>` byte string.
pub fn bytes_arg(inputs: &[Value], index: usize) -> Result<Vec<u8>, String> {
    inputs
        .get(index)
        .ok_or_else(|| format!("input {index} is missing"))
        .and_then(|v| bytes_item(v, index))
}

/// Read the input at `index` as a list of integer frame signatures, converted to a JS array.
pub fn frames_arg(inputs: &[Value], index: usize) -> Result<JsValue, String> {
    let frames: Vec<Vec<i32>> = inputs
        .get(index)
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .ok_or_else(|| format!("input {index} is not a list of i32 lists"))?;
    serde_wasm_bindgen::to_value(&frames).map_err(|e| format!("input {index}: {e}"))
}

/// Read the input at `index` as a `u32`.
pub fn u32_arg(inputs: &[Value], index: usize) -> Result<u32, String> {
    inputs
        .get(index)
        .and_then(Value::as_u64)
        .and_then(|n| u32::try_from(n).ok())
        .ok_or_else(|| format!("input {index} is not a u32"))
}
