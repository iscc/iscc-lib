//! PyO3 bindings for iscc-lib, exposed as `iscc_lib._lowlevel`.
//!
//! This module provides the low-level Rust-backed functions for the `iscc_lib`
//! Python package. The pure-Python wrapper in `python/iscc_lib/__init__.py`
//! re-exports these for a Pythonic API.
//!
//! All `gen_*_v0` functions return Python `dict` objects with the same keys
//! and value types as the iscc-core reference implementation.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList};

/// Convert a Python sequence to a PyList, passing lists through unchanged.
fn to_pylist<'py>(py: Python<'py>, obj: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>> {
    if let Ok(list) = obj.downcast::<PyList>() {
        return Ok(list.clone());
    }
    unsafe {
        let ptr = pyo3::ffi::PySequence_List(obj.as_ptr());
        if ptr.is_null() {
            return Err(pyo3::PyErr::fetch(py));
        }
        Ok(Bound::from_owned_ptr(py, ptr).downcast_into_unchecked())
    }
}

/// Extract nested frame signatures from a Python sequence using CPython C API.
///
/// Accepts any sequence of sequences (lists, tuples, etc.). Uses `PyLong_AsLong`
/// to bypass PyO3 object wrapping overhead. Returns a flat `Vec<i32>` plus dimensions.
fn extract_frame_sigs(
    py: Python<'_>,
    frame_sigs: &Bound<'_, PyAny>,
) -> PyResult<(Vec<i32>, usize)> {
    let frame_list = to_pylist(py, frame_sigs)?;
    let num_frames = frame_list.len();
    if num_frames == 0 {
        return Err(PyValueError::new_err("frame_sigs must not be empty"));
    }

    // Get frame length from first inner sequence
    let first = to_pylist(py, &frame_list.get_item(0)?)?;
    let frame_len = first.len();
    if frame_len == 0 {
        return Err(PyValueError::new_err("frame_sigs frames must not be empty"));
    }

    let total = num_frames * frame_len;
    let mut flat = Vec::with_capacity(total);

    // SAFETY: frame_list is a verified PyList. Inner sequences are converted
    // to PyList as needed. PyList_GetItem returns borrowed references with
    // bounds checking. PyLong_AsLong is called with error and range checking.
    unsafe {
        let outer_ptr = frame_list.as_ptr();
        for f in 0..num_frames {
            let frame_ptr = pyo3::ffi::PyList_GetItem(outer_ptr, f as isize);
            if frame_ptr.is_null() {
                return Err(pyo3::PyErr::fetch(py));
            }
            // Convert non-list inner sequences to list for PyList_GetItem access
            let (inner_ptr, _guard) = if pyo3::ffi::PyList_Check(frame_ptr) != 0 {
                (frame_ptr, None::<Bound<'_, PyAny>>)
            } else {
                let converted = pyo3::ffi::PySequence_List(frame_ptr);
                if converted.is_null() {
                    return Err(PyValueError::new_err(format!(
                        "frame_sigs[{f}] is not a sequence"
                    )));
                }
                let guard = Bound::from_owned_ptr(py, converted);
                (converted, Some(guard))
            };
            let inner_len = pyo3::ffi::PyList_Size(inner_ptr) as usize;
            if inner_len != frame_len {
                return Err(PyValueError::new_err(format!(
                    "frame_sigs[{f}] has length {inner_len}, expected {frame_len}"
                )));
            }
            for i in 0..frame_len {
                let val_ptr = pyo3::ffi::PyList_GetItem(inner_ptr, i as isize);
                if val_ptr.is_null() {
                    return Err(pyo3::PyErr::fetch(py));
                }
                let val = pyo3::ffi::PyLong_AsLong(val_ptr);
                if val == -1 && !pyo3::ffi::PyErr_Occurred().is_null() {
                    return Err(pyo3::PyErr::fetch(py));
                }
                let val_i32 = i32::try_from(val).map_err(|_| {
                    PyValueError::new_err(format!(
                        "frame_sigs[{f}][{i}]: value {val} out of i32 range"
                    ))
                })?;
                flat.push(val_i32);
            }
        }
    }

    Ok((flat, frame_len))
}

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
/// Uses direct CPython C API for fast extraction from nested Python lists.
/// Returns a dict with key: `iscc`.
#[pyfunction]
#[pyo3(signature = (frame_sigs, bits=64))]
fn gen_video_code_v0(
    py: Python<'_>,
    frame_sigs: Bound<'_, PyAny>,
    bits: u32,
) -> PyResult<PyObject> {
    let (flat, frame_len) = extract_frame_sigs(py, &frame_sigs)?;
    let frame_slices: Vec<&[i32]> = flat.chunks_exact(frame_len).collect();
    let r = iscc_lib::gen_video_code_v0(&frame_slices, bits)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let dict = PyDict::new(py);
    dict.set_item("iscc", r.iscc)?;
    Ok(dict.into())
}

/// Generate a Video-Code from a flat byte buffer of i32 frame signatures.
///
/// Accepts pre-flattened frame data as raw bytes (native-endian i32 values)
/// for callers that already have typed data (numpy, array.array).
///
/// Returns a dict with key: `iscc`.
#[pyfunction]
#[pyo3(signature = (data, num_frames, frame_len, bits=64))]
fn gen_video_code_v0_flat(
    py: Python<'_>,
    data: &[u8],
    num_frames: usize,
    frame_len: usize,
    bits: u32,
) -> PyResult<PyObject> {
    let frames = flat_bytes_to_frames(data, num_frames, frame_len)?;
    let frame_refs: Vec<&[i32]> = frames.iter().map(|f| f.as_slice()).collect();
    let r = iscc_lib::gen_video_code_v0(&frame_refs, bits)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let dict = PyDict::new(py);
    dict.set_item("iscc", r.iscc)?;
    Ok(dict.into())
}

/// Compute a video hash from a flat byte buffer of i32 frame signatures.
///
/// Returns raw bytes of length `bits / 8`.
#[pyfunction]
#[pyo3(signature = (data, num_frames, frame_len, bits=64))]
fn soft_hash_video_v0_flat(
    py: Python<'_>,
    data: &[u8],
    num_frames: usize,
    frame_len: usize,
    bits: u32,
) -> PyResult<PyObject> {
    let frames = flat_bytes_to_frames(data, num_frames, frame_len)?;
    let frame_refs: Vec<&[i32]> = frames.iter().map(|f| f.as_slice()).collect();
    let result = iscc_lib::soft_hash_video_v0(&frame_refs, bits)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok(PyBytes::new(py, &result).into())
}

/// Reinterpret a flat byte buffer as a vector of i32 slices.
fn flat_bytes_to_frames(
    data: &[u8],
    num_frames: usize,
    frame_len: usize,
) -> PyResult<Vec<Vec<i32>>> {
    if num_frames == 0 || frame_len == 0 {
        return Err(PyValueError::new_err(
            "num_frames and frame_len must both be greater than zero",
        ));
    }
    let expected = num_frames
        .checked_mul(frame_len)
        .and_then(|n| n.checked_mul(4))
        .ok_or_else(|| PyValueError::new_err("frame dimensions overflow"))?;
    if data.len() != expected {
        return Err(PyValueError::new_err(format!(
            "buffer size mismatch: expected {} bytes ({} frames × {} elements × 4), got {}",
            expected,
            num_frames,
            frame_len,
            data.len()
        )));
    }
    let mut frames = Vec::with_capacity(num_frames);
    for f in 0..num_frames {
        let offset = f * frame_len * 4;
        let mut frame = Vec::with_capacity(frame_len);
        for i in 0..frame_len {
            let pos = offset + i * 4;
            let bytes: [u8; 4] = [data[pos], data[pos + 1], data[pos + 2], data[pos + 3]];
            frame.push(i32::from_ne_bytes(bytes));
        }
        frames.push(frame);
    }
    Ok(frames)
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

/// Encode raw digest components into a base32 ISCC unit string.
///
/// Takes integer type identifiers and a raw digest, returns a base32-encoded
/// ISCC unit string (without "ISCC:" prefix).
#[pyfunction]
#[pyo3(signature = (mtype, stype, version, bit_length, digest))]
fn encode_component(
    mtype: u8,
    stype: u8,
    version: u8,
    bit_length: u32,
    digest: &[u8],
) -> PyResult<String> {
    iscc_lib::encode_component(mtype, stype, version, bit_length, digest)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Decode an ISCC unit string into header components and raw digest.
///
/// Returns a tuple of `(maintype, subtype, version, length_index, digest)`
/// where digest is the raw bytes truncated to the encoded bit-length.
#[pyfunction]
#[pyo3(signature = (iscc))]
fn iscc_decode(py: Python<'_>, iscc: &str) -> PyResult<PyObject> {
    let (mt, st, vs, li, digest) =
        iscc_lib::iscc_decode(iscc).map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok((mt, st, vs, li, PyBytes::new(py, &digest))
        .into_pyobject(py)?
        .into())
}

/// Convert a JSON string into a `data:` URL with JCS canonicalization.
///
/// Uses `application/ld+json` media type when the JSON contains an `@context`
/// key, otherwise `application/json`.
#[pyfunction]
#[pyo3(signature = (json))]
fn json_to_data_url(json: &str) -> PyResult<String> {
    iscc_lib::json_to_data_url(json).map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Generate sliding window n-grams from a string.
///
/// Returns overlapping substrings of `width` Unicode characters, advancing
/// by one character at a time. Raises `ValueError` if width is less than 2.
#[pyfunction]
#[pyo3(signature = (seq, width))]
fn sliding_window(seq: &str, width: usize) -> PyResult<Vec<String>> {
    iscc_lib::sliding_window(seq, width).map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Compute a SimHash from a sequence of equal-length hash digests.
///
/// Returns a similarity-preserving hash as bytes. Each output bit is set
/// when its frequency meets or exceeds half the input count. Returns 32
/// zero bytes for empty input. Raises `ValueError` on mismatched digest lengths.
#[pyfunction]
fn alg_simhash(hash_digests: Vec<Vec<u8>>) -> PyResult<Vec<u8>> {
    iscc_lib::alg_simhash(&hash_digests).map_err(|e| PyValueError::new_err(e.to_string()))
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
/// Uses direct CPython C API for fast extraction from nested Python lists.
/// Returns raw bytes of length `bits / 8`.
#[pyfunction]
#[pyo3(signature = (frame_sigs, bits=64))]
fn soft_hash_video_v0(
    py: Python<'_>,
    frame_sigs: Bound<'_, PyAny>,
    bits: u32,
) -> PyResult<PyObject> {
    let (flat, frame_len) = extract_frame_sigs(py, &frame_sigs)?;
    let frame_slices: Vec<&[i32]> = flat.chunks_exact(frame_len).collect();
    let result = iscc_lib::soft_hash_video_v0(&frame_slices, bits)
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

/// Python module `iscc_lib._lowlevel` backed by Rust.
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
    m.add_function(wrap_pyfunction!(encode_component, m)?)?;
    m.add_function(wrap_pyfunction!(iscc_decode, m)?)?;
    m.add_function(wrap_pyfunction!(json_to_data_url, m)?)?;
    m.add("META_TRIM_NAME", iscc_lib::META_TRIM_NAME)?;
    m.add("META_TRIM_DESCRIPTION", iscc_lib::META_TRIM_DESCRIPTION)?;
    m.add("IO_READ_SIZE", iscc_lib::IO_READ_SIZE)?;
    m.add("TEXT_NGRAM_SIZE", iscc_lib::TEXT_NGRAM_SIZE)?;
    m.add_function(wrap_pyfunction!(sliding_window, m)?)?;
    m.add_function(wrap_pyfunction!(alg_simhash, m)?)?;
    m.add_function(wrap_pyfunction!(alg_minhash_256, m)?)?;
    m.add_function(wrap_pyfunction!(alg_cdc_chunks, m)?)?;
    m.add_function(wrap_pyfunction!(soft_hash_video_v0, m)?)?;
    m.add_function(wrap_pyfunction!(gen_video_code_v0_flat, m)?)?;
    m.add_function(wrap_pyfunction!(soft_hash_video_v0_flat, m)?)?;
    m.add_class::<PyDataHasher>()?;
    m.add_class::<PyInstanceHasher>()?;
    Ok(())
}
