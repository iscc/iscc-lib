//! PyO3 bindings for iscc-lib, exposed as `iscc._lowlevel`.
//!
//! This module provides the low-level Rust-backed functions for the `iscc`
//! Python package. The pure-Python wrapper in `python/iscc/__init__.py`
//! re-exports these for a Pythonic API.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Generate an Instance-Code from raw byte data.
///
/// Produces an ISCC Instance-Code by hashing the complete byte stream
/// with BLAKE3. Returns the ISCC string with "ISCC:" prefix.
#[pyfunction]
#[pyo3(signature = (data, bits=64))]
fn gen_instance_code_v0(data: &[u8], bits: u32) -> PyResult<String> {
    iscc_lib::gen_instance_code_v0(data, bits).map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Python module `iscc._lowlevel` backed by Rust.
#[pymodule(name = "_lowlevel")]
fn iscc_lowlevel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gen_instance_code_v0, m)?)?;
    Ok(())
}
