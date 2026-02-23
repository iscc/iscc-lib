//! wasm-bindgen bindings for iscc-lib, exposed as `@iscc/wasm` npm package.
//!
//! This module provides browser-compatible WASM bindings for the `iscc-lib`
//! core. Each function is a thin wrapper around the corresponding `iscc_lib`
//! API, converting wasm-bindgen types to Rust types and mapping errors to
//! `JsError`.

use wasm_bindgen::prelude::*;

/// Generate a Meta-Code from name and optional metadata.
///
/// Produces an ISCC Meta-Code by hashing the provided name, description,
/// and metadata fields using the SimHash algorithm.
#[wasm_bindgen]
pub fn gen_meta_code_v0(
    name: &str,
    description: Option<String>,
    meta: Option<String>,
    bits: Option<u32>,
) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_meta_code_v0(name, description.as_deref(), meta.as_deref(), bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate a Text-Code from plain text content.
///
/// Produces an ISCC Content-Code for text using MinHash-based
/// similarity hashing.
#[wasm_bindgen]
pub fn gen_text_code_v0(text: &str, bits: Option<u32>) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_text_code_v0(text, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate an Image-Code from pixel data.
///
/// Produces an ISCC Content-Code for images from 1024 grayscale pixels
/// (32x32) using a DCT-based perceptual hash.
#[wasm_bindgen]
pub fn gen_image_code_v0(pixels: &[u8], bits: Option<u32>) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_image_code_v0(pixels, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate an Audio-Code from a Chromaprint feature vector.
///
/// Produces an ISCC Content-Code for audio from signed integer
/// Chromaprint fingerprint features using multi-stage SimHash.
#[wasm_bindgen]
pub fn gen_audio_code_v0(cv: Vec<i32>, bits: Option<u32>) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_audio_code_v0(&cv, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate a Video-Code from frame signature data.
///
/// Produces an ISCC Content-Code for video from MPEG-7 frame
/// signature vectors using WTA-Hash. Accepts a JS array of arrays of i32.
#[wasm_bindgen]
pub fn gen_video_code_v0(frame_sigs: JsValue, bits: Option<u32>) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    let frame_sigs: Vec<Vec<i32>> =
        serde_wasm_bindgen::from_value(frame_sigs).map_err(|e| JsError::new(&e.to_string()))?;
    iscc_lib::gen_video_code_v0(&frame_sigs, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate a Mixed-Code from multiple Content-Code strings.
///
/// Produces a Mixed Content-Code by combining multiple ISCC Content-Codes
/// of different types using SimHash. Accepts a JS array of strings.
#[wasm_bindgen]
pub fn gen_mixed_code_v0(codes: JsValue, bits: Option<u32>) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    let codes: Vec<String> =
        serde_wasm_bindgen::from_value(codes).map_err(|e| JsError::new(&e.to_string()))?;
    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
    iscc_lib::gen_mixed_code_v0(&refs, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate a Data-Code from raw byte data.
///
/// Produces an ISCC Data-Code by splitting data into content-defined
/// chunks and applying MinHash for similarity hashing.
#[wasm_bindgen]
pub fn gen_data_code_v0(data: &[u8], bits: Option<u32>) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_data_code_v0(data, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate an Instance-Code from raw byte data.
///
/// Produces an ISCC Instance-Code by hashing the complete byte stream
/// with BLAKE3. Returns the ISCC string with "ISCC:" prefix.
#[wasm_bindgen]
pub fn gen_instance_code_v0(data: &[u8], bits: Option<u32>) -> Result<String, JsError> {
    let bits = bits.unwrap_or(64);
    iscc_lib::gen_instance_code_v0(data, bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Generate a composite ISCC-CODE from individual unit codes.
///
/// Combines multiple ISCC unit codes into a single composite ISCC-CODE.
/// Requires at least Data-Code and Instance-Code. Accepts a JS array of strings.
#[wasm_bindgen]
pub fn gen_iscc_code_v0(codes: JsValue, wide: Option<bool>) -> Result<String, JsError> {
    let wide = wide.unwrap_or(false);
    let codes: Vec<String> =
        serde_wasm_bindgen::from_value(codes).map_err(|e| JsError::new(&e.to_string()))?;
    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
    iscc_lib::gen_iscc_code_v0(&refs, wide)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}
