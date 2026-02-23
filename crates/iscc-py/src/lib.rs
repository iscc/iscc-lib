//! PyO3 bindings for iscc-lib, exposed as `iscc._lowlevel`.
//!
//! This module provides the low-level Rust-backed functions for the `iscc`
//! Python package. The pure-Python wrapper in `python/iscc/__init__.py`
//! re-exports these for a Pythonic API.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Generate a Meta-Code from name and optional metadata.
///
/// Produces an ISCC Meta-Code by hashing the provided name, description,
/// and metadata fields using the SimHash algorithm.
#[pyfunction]
#[pyo3(signature = (name, description=None, meta=None, bits=64))]
fn gen_meta_code_v0(
    name: &str,
    description: Option<&str>,
    meta: Option<&str>,
    bits: u32,
) -> PyResult<String> {
    iscc_lib::gen_meta_code_v0(name, description, meta, bits)
        .map(|r| r.iscc)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Generate a Text-Code from plain text content.
///
/// Produces an ISCC Content-Code for text using MinHash-based
/// similarity hashing.
#[pyfunction]
#[pyo3(signature = (text, bits=64))]
fn gen_text_code_v0(text: &str, bits: u32) -> PyResult<String> {
    iscc_lib::gen_text_code_v0(text, bits)
        .map(|r| r.iscc)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Generate an Image-Code from pixel data.
///
/// Produces an ISCC Content-Code for images from 1024 grayscale pixels
/// (32×32) using a DCT-based perceptual hash.
#[pyfunction]
#[pyo3(signature = (pixels, bits=64))]
fn gen_image_code_v0(pixels: &[u8], bits: u32) -> PyResult<String> {
    iscc_lib::gen_image_code_v0(pixels, bits)
        .map(|r| r.iscc)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Generate an Audio-Code from a Chromaprint feature vector.
///
/// Produces an ISCC Content-Code for audio from signed integer
/// Chromaprint fingerprint features using multi-stage SimHash.
#[pyfunction]
#[pyo3(signature = (cv, bits=64))]
fn gen_audio_code_v0(cv: Vec<i32>, bits: u32) -> PyResult<String> {
    iscc_lib::gen_audio_code_v0(&cv, bits)
        .map(|r| r.iscc)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Generate a Video-Code from frame signature data.
///
/// Produces an ISCC Content-Code for video from MPEG-7 frame
/// signature vectors using WTA-Hash.
#[pyfunction]
#[pyo3(signature = (frame_sigs, bits=64))]
fn gen_video_code_v0(frame_sigs: Vec<Vec<i32>>, bits: u32) -> PyResult<String> {
    iscc_lib::gen_video_code_v0(&frame_sigs, bits)
        .map(|r| r.iscc)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Generate a Mixed-Code from multiple Content-Code strings.
///
/// Produces a Mixed Content-Code by combining multiple ISCC Content-Codes
/// of different types using SimHash.
#[pyfunction]
#[pyo3(signature = (codes, bits=64))]
fn gen_mixed_code_v0(codes: Vec<String>, bits: u32) -> PyResult<String> {
    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
    iscc_lib::gen_mixed_code_v0(&refs, bits)
        .map(|r| r.iscc)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Generate a Data-Code from raw byte data.
///
/// Produces an ISCC Data-Code by splitting data into content-defined
/// chunks and applying MinHash for similarity hashing.
#[pyfunction]
#[pyo3(signature = (data, bits=64))]
fn gen_data_code_v0(data: &[u8], bits: u32) -> PyResult<String> {
    iscc_lib::gen_data_code_v0(data, bits)
        .map(|r| r.iscc)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Generate an Instance-Code from raw byte data.
///
/// Produces an ISCC Instance-Code by hashing the complete byte stream
/// with BLAKE3. Returns the ISCC string with "ISCC:" prefix.
#[pyfunction]
#[pyo3(signature = (data, bits=64))]
fn gen_instance_code_v0(data: &[u8], bits: u32) -> PyResult<String> {
    iscc_lib::gen_instance_code_v0(data, bits)
        .map(|r| r.iscc)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Generate a composite ISCC-CODE from individual unit codes.
///
/// Combines multiple ISCC unit codes into a single composite ISCC-CODE.
/// Requires at least Data-Code and Instance-Code.
#[pyfunction]
#[pyo3(signature = (codes, wide=false))]
fn gen_iscc_code_v0(codes: Vec<String>, wide: bool) -> PyResult<String> {
    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
    iscc_lib::gen_iscc_code_v0(&refs, wide)
        .map(|r| r.iscc)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Python module `iscc._lowlevel` backed by Rust.
#[pymodule(name = "_lowlevel")]
fn iscc_lowlevel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gen_meta_code_v0, m)?)?;
    m.add_function(wrap_pyfunction!(gen_text_code_v0, m)?)?;
    m.add_function(wrap_pyfunction!(gen_image_code_v0, m)?)?;
    m.add_function(wrap_pyfunction!(gen_audio_code_v0, m)?)?;
    m.add_function(wrap_pyfunction!(gen_video_code_v0, m)?)?;
    m.add_function(wrap_pyfunction!(gen_mixed_code_v0, m)?)?;
    m.add_function(wrap_pyfunction!(gen_data_code_v0, m)?)?;
    m.add_function(wrap_pyfunction!(gen_instance_code_v0, m)?)?;
    m.add_function(wrap_pyfunction!(gen_iscc_code_v0, m)?)?;
    Ok(())
}
