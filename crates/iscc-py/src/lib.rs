//! PyO3 bindings for iscc-lib, exposed as `iscc._lowlevel`.
//!
//! This module provides the low-level Rust-backed functions for the `iscc`
//! Python package. The pure-Python wrapper in `python/iscc/__init__.py`
//! re-exports these for a Pythonic API.
//!
//! All `gen_*_v0` functions return Python `dict` objects with the same keys
//! and value types as the iscc-core reference implementation.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};

/// Generate a Meta-Code from name and optional metadata.
///
/// Returns a dict with keys: `iscc`, `name`, `metahash`, and optionally
/// `description` and `meta`.
#[pyfunction]
#[pyo3(signature = (name, description=None, meta=None, bits=64))]
fn gen_meta_code_v0(
    py: Python<'_>,
    name: &str,
    description: Option<&str>,
    meta: Option<&str>,
    bits: u32,
) -> PyResult<PyObject> {
    let r = iscc_lib::gen_meta_code_v0(name, description, meta, bits)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let dict = PyDict::new(py);
    dict.set_item("iscc", r.iscc)?;
    dict.set_item("name", r.name)?;
    dict.set_item("metahash", r.metahash)?;
    if let Some(desc) = r.description {
        dict.set_item("description", desc)?;
    }
    if let Some(meta) = r.meta {
        dict.set_item("meta", meta)?;
    }
    Ok(dict.into())
}

/// Generate a Text-Code from plain text content.
///
/// Returns a dict with keys: `iscc`, `characters`.
#[pyfunction]
#[pyo3(signature = (text, bits=64))]
fn gen_text_code_v0(py: Python<'_>, text: &str, bits: u32) -> PyResult<PyObject> {
    let r =
        iscc_lib::gen_text_code_v0(text, bits).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let dict = PyDict::new(py);
    dict.set_item("iscc", r.iscc)?;
    dict.set_item("characters", r.characters)?;
    Ok(dict.into())
}

/// Generate an Image-Code from pixel data.
///
/// Returns a dict with key: `iscc`.
#[pyfunction]
#[pyo3(signature = (pixels, bits=64))]
fn gen_image_code_v0(py: Python<'_>, pixels: &[u8], bits: u32) -> PyResult<PyObject> {
    let r = iscc_lib::gen_image_code_v0(pixels, bits)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let dict = PyDict::new(py);
    dict.set_item("iscc", r.iscc)?;
    Ok(dict.into())
}

/// Generate an Audio-Code from a Chromaprint feature vector.
///
/// Returns a dict with key: `iscc`.
#[pyfunction]
#[pyo3(signature = (cv, bits=64))]
fn gen_audio_code_v0(py: Python<'_>, cv: Vec<i32>, bits: u32) -> PyResult<PyObject> {
    let r =
        iscc_lib::gen_audio_code_v0(&cv, bits).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let dict = PyDict::new(py);
    dict.set_item("iscc", r.iscc)?;
    Ok(dict.into())
}

/// Generate a Video-Code from frame signature data.
///
/// Returns a dict with key: `iscc`.
#[pyfunction]
#[pyo3(signature = (frame_sigs, bits=64))]
fn gen_video_code_v0(py: Python<'_>, frame_sigs: Vec<Vec<i32>>, bits: u32) -> PyResult<PyObject> {
    let r = iscc_lib::gen_video_code_v0(&frame_sigs, bits)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let dict = PyDict::new(py);
    dict.set_item("iscc", r.iscc)?;
    Ok(dict.into())
}

/// Generate a Mixed-Code from multiple Content-Code strings.
///
/// Returns a dict with keys: `iscc`, `parts`.
#[pyfunction]
#[pyo3(signature = (codes, bits=64))]
fn gen_mixed_code_v0(py: Python<'_>, codes: Vec<String>, bits: u32) -> PyResult<PyObject> {
    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
    let r = iscc_lib::gen_mixed_code_v0(&refs, bits)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let dict = PyDict::new(py);
    dict.set_item("iscc", r.iscc)?;
    dict.set_item("parts", r.parts)?;
    Ok(dict.into())
}

/// Generate a Data-Code from raw byte data.
///
/// Returns a dict with key: `iscc`.
#[pyfunction]
#[pyo3(signature = (data, bits=64))]
fn gen_data_code_v0(py: Python<'_>, data: &[u8], bits: u32) -> PyResult<PyObject> {
    let r =
        iscc_lib::gen_data_code_v0(data, bits).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let dict = PyDict::new(py);
    dict.set_item("iscc", r.iscc)?;
    Ok(dict.into())
}

/// Generate an Instance-Code from raw byte data.
///
/// Returns a dict with keys: `iscc`, `datahash`, `filesize`.
#[pyfunction]
#[pyo3(signature = (data, bits=64))]
fn gen_instance_code_v0(py: Python<'_>, data: &[u8], bits: u32) -> PyResult<PyObject> {
    let r = iscc_lib::gen_instance_code_v0(data, bits)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let dict = PyDict::new(py);
    dict.set_item("iscc", r.iscc)?;
    dict.set_item("datahash", r.datahash)?;
    dict.set_item("filesize", r.filesize)?;
    Ok(dict.into())
}

/// Generate a composite ISCC-CODE from individual unit codes.
///
/// Returns a dict with key: `iscc`.
#[pyfunction]
#[pyo3(signature = (codes, wide=false))]
fn gen_iscc_code_v0(py: Python<'_>, codes: Vec<String>, wide: bool) -> PyResult<PyObject> {
    let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();
    let r = iscc_lib::gen_iscc_code_v0(&refs, wide)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let dict = PyDict::new(py);
    dict.set_item("iscc", r.iscc)?;
    Ok(dict.into())
}

/// Run all conformance tests against vendored test vectors.
///
/// Returns `True` if all tests pass, `False` if any fail.
#[pyfunction]
fn conformance_selftest() -> bool {
    iscc_lib::conformance_selftest()
}

/// Clean and normalize text for display.
///
/// Applies NFKC normalization, removes control characters (except newlines),
/// normalizes `\r\n` to `\n`, collapses consecutive empty lines, and strips
/// leading/trailing whitespace.
#[pyfunction]
fn text_clean(text: &str) -> String {
    iscc_lib::text_clean(text)
}

/// Remove newlines and collapse whitespace to single spaces.
///
/// Converts multi-line text into a single normalized line.
#[pyfunction]
fn text_remove_newlines(text: &str) -> String {
    iscc_lib::text_remove_newlines(text)
}

/// Trim text so its UTF-8 encoded size does not exceed `nbytes`.
///
/// Multi-byte characters that would be split are dropped entirely.
/// Leading/trailing whitespace is stripped from the result.
#[pyfunction]
#[pyo3(signature = (text, nbytes))]
fn text_trim(text: &str, nbytes: usize) -> String {
    iscc_lib::text_trim(text, nbytes)
}

/// Normalize and simplify text for similarity hashing.
///
/// Applies NFD normalization, lowercasing, removes whitespace and characters
/// in Unicode categories C (control), M (mark), and P (punctuation), then
/// recombines with NFKC normalization.
#[pyfunction]
fn text_collapse(text: &str) -> String {
    iscc_lib::text_collapse(text)
}

/// Encode bytes as base64url (RFC 4648 §5, no padding).
///
/// Returns a URL-safe base64 encoded string without padding characters.
#[pyfunction]
fn encode_base64(data: &[u8]) -> String {
    iscc_lib::encode_base64(data)
}

/// Decompose a composite ISCC-CODE into individual ISCC-UNITs.
///
/// Accepts a normalized ISCC-CODE or concatenated ISCC-UNIT sequence.
/// The optional "ISCC:" prefix is stripped before decoding.
/// Returns a list of base32-encoded ISCC-UNIT strings (without prefix).
#[pyfunction]
#[pyo3(signature = (iscc_code))]
fn iscc_decompose(iscc_code: &str) -> PyResult<Vec<String>> {
    iscc_lib::iscc_decompose(iscc_code).map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Generate sliding window n-grams from a string.
///
/// Returns overlapping substrings of `width` Unicode characters, advancing
/// by one character at a time. Raises `ValueError` if width is less than 2.
#[pyfunction]
#[pyo3(signature = (seq, width))]
fn sliding_window(seq: &str, width: usize) -> PyResult<Vec<String>> {
    if width < 2 {
        return Err(PyValueError::new_err(
            "Sliding window width must be 2 or bigger.",
        ));
    }
    Ok(iscc_lib::sliding_window(seq, width))
}

/// Compute a SimHash from a sequence of equal-length hash digests.
///
/// Returns a similarity-preserving hash as bytes. Each output bit is set
/// when its frequency meets or exceeds half the input count. Returns 32
/// zero bytes for empty input.
#[pyfunction]
fn alg_simhash(hash_digests: Vec<Vec<u8>>) -> Vec<u8> {
    iscc_lib::alg_simhash(&hash_digests)
}

/// Compute a 256-bit MinHash digest from 32-bit integer features.
///
/// Uses 64 universal hash functions with bit-interleaved compression to
/// produce a 32-byte similarity-preserving digest.
#[pyfunction]
fn alg_minhash_256(features: Vec<u32>) -> Vec<u8> {
    iscc_lib::alg_minhash_256(&features)
}

/// Split data into content-defined chunks using gear rolling hash.
///
/// Returns at least one chunk (empty bytes for empty input). When `utf32`
/// is true, aligns cut points to 4-byte boundaries.
#[pyfunction]
#[pyo3(signature = (data, utf32, avg_chunk_size=1024))]
fn alg_cdc_chunks(data: &[u8], utf32: bool, avg_chunk_size: u32) -> Vec<Vec<u8>> {
    iscc_lib::alg_cdc_chunks(data, utf32, avg_chunk_size)
        .into_iter()
        .map(|c| c.to_vec())
        .collect()
}

/// Compute a similarity-preserving hash from video frame signatures.
///
/// Returns raw bytes of length `bits / 8`.
#[pyfunction]
#[pyo3(signature = (frame_sigs, bits=64))]
fn soft_hash_video_v0(py: Python<'_>, frame_sigs: Vec<Vec<i32>>, bits: u32) -> PyResult<PyObject> {
    let result = iscc_lib::soft_hash_video_v0(&frame_sigs, bits)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok(PyBytes::new(py, &result).into())
}

/// Streaming Data-Code generator.
///
/// Incrementally processes data with content-defined chunking and MinHash
/// to produce results identical to `gen_data_code_v0`.
#[pyclass(name = "DataHasher")]
struct PyDataHasher {
    inner: Option<iscc_lib::DataHasher>,
}

#[pymethods]
impl PyDataHasher {
    /// Create a new `DataHasher`.
    #[new]
    fn new() -> Self {
        Self {
            inner: Some(iscc_lib::DataHasher::new()),
        }
    }

    /// Push data into the hasher.
    fn update(&mut self, data: &[u8]) -> PyResult<()> {
        self.inner
            .as_mut()
            .ok_or_else(|| PyValueError::new_err("DataHasher already finalized"))?
            .update(data);
        Ok(())
    }

    /// Consume the hasher and produce a Data-Code result dict.
    #[pyo3(signature = (bits=64))]
    fn finalize(&mut self, py: Python<'_>, bits: u32) -> PyResult<PyObject> {
        let hasher = self
            .inner
            .take()
            .ok_or_else(|| PyValueError::new_err("DataHasher already finalized"))?;
        let r = hasher
            .finalize(bits)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let dict = PyDict::new(py);
        dict.set_item("iscc", r.iscc)?;
        Ok(dict.into())
    }
}

/// Streaming Instance-Code generator.
///
/// Incrementally hashes data with BLAKE3 to produce results identical
/// to `gen_instance_code_v0`.
#[pyclass(name = "InstanceHasher")]
struct PyInstanceHasher {
    inner: Option<iscc_lib::InstanceHasher>,
}

#[pymethods]
impl PyInstanceHasher {
    /// Create a new `InstanceHasher`.
    #[new]
    fn new() -> Self {
        Self {
            inner: Some(iscc_lib::InstanceHasher::new()),
        }
    }

    /// Push data into the hasher.
    fn update(&mut self, data: &[u8]) -> PyResult<()> {
        self.inner
            .as_mut()
            .ok_or_else(|| PyValueError::new_err("InstanceHasher already finalized"))?
            .update(data);
        Ok(())
    }

    /// Consume the hasher and produce an Instance-Code result dict.
    #[pyo3(signature = (bits=64))]
    fn finalize(&mut self, py: Python<'_>, bits: u32) -> PyResult<PyObject> {
        let hasher = self
            .inner
            .take()
            .ok_or_else(|| PyValueError::new_err("InstanceHasher already finalized"))?;
        let r = hasher
            .finalize(bits)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let dict = PyDict::new(py);
        dict.set_item("iscc", r.iscc)?;
        dict.set_item("datahash", r.datahash)?;
        dict.set_item("filesize", r.filesize)?;
        Ok(dict.into())
    }
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
    m.add_function(wrap_pyfunction!(conformance_selftest, m)?)?;
    m.add_function(wrap_pyfunction!(text_clean, m)?)?;
    m.add_function(wrap_pyfunction!(text_remove_newlines, m)?)?;
    m.add_function(wrap_pyfunction!(text_trim, m)?)?;
    m.add_function(wrap_pyfunction!(text_collapse, m)?)?;
    m.add_function(wrap_pyfunction!(encode_base64, m)?)?;
    m.add_function(wrap_pyfunction!(iscc_decompose, m)?)?;
    m.add_function(wrap_pyfunction!(sliding_window, m)?)?;
    m.add_function(wrap_pyfunction!(alg_simhash, m)?)?;
    m.add_function(wrap_pyfunction!(alg_minhash_256, m)?)?;
    m.add_function(wrap_pyfunction!(alg_cdc_chunks, m)?)?;
    m.add_function(wrap_pyfunction!(soft_hash_video_v0, m)?)?;
    m.add_class::<PyDataHasher>()?;
    m.add_class::<PyInstanceHasher>()?;
    Ok(())
}
