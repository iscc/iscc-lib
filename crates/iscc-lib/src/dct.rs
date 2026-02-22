//! Discrete Cosine Transform (DCT) for image content hashing.
//!
//! Implements the fast recursive DCT from Nayuki, matching the
//! reference implementation in `iscc-core/dct.py`.
//!
//! See: <https://www.nayuki.io/page/fast-discrete-cosine-transform-algorithms>

use std::f64::consts::PI;

use crate::IsccResult;

/// Compute the fast Discrete Cosine Transform (Nayuki algorithm).
///
/// Uses a recursive divide-and-conquer approach. Input length must be
/// a power of 2 (or 1). Returns f64 values matching the reference
/// implementation's floating-point behavior exactly.
pub(crate) fn alg_dct(v: &[f64]) -> IsccResult<Vec<f64>> {
    let n = v.len();
    if n == 0 || (n > 1 && n % 2 != 0) {
        return Err(crate::IsccError::InvalidInput(
            "DCT input must be non-empty with even length (or 1)".into(),
        ));
    }
    Ok(dct_recursive(v))
}

/// Recursive fast DCT implementation.
fn dct_recursive(v: &[f64]) -> Vec<f64> {
    let n = v.len();
    if n == 1 {
        return v.to_vec();
    }

    let half = n / 2;

    // Split into symmetric (alpha) and antisymmetric (beta) parts
    let alpha: Vec<f64> = (0..half).map(|i| v[i] + v[n - 1 - i]).collect();
    let beta: Vec<f64> = (0..half)
        .map(|i| (v[i] - v[n - 1 - i]) / ((i as f64 + 0.5) * PI / n as f64).cos() / 2.0)
        .collect();

    let alpha = dct_recursive(&alpha);
    let beta = dct_recursive(&beta);

    // Interleave results
    let mut result = Vec::with_capacity(n);
    for i in 0..half - 1 {
        result.push(alpha[i]);
        result.push(beta[i] + beta[i + 1]);
    }
    result.push(alpha[half - 1]);
    result.push(beta[half - 1]);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alg_dct_empty_error() {
        assert!(alg_dct(&[]).is_err());
    }

    #[test]
    fn test_alg_dct_odd_length_error() {
        assert!(alg_dct(&[1.0, 2.0, 3.0]).is_err());
    }

    #[test]
    fn test_alg_dct_all_zeros() {
        let input = vec![0.0f64; 64];
        let result = alg_dct(&input).unwrap();
        for val in &result {
            assert!(val.abs() < 1e-10, "expected ~0, got {val}");
        }
    }

    #[test]
    fn test_alg_dct_all_ones() {
        let input = vec![1.0f64; 64];
        let result = alg_dct(&input).unwrap();
        assert!((result[0] - 64.0).abs() < 1e-10);
        for &val in &result[1..] {
            assert!(val.abs() < 1e-10, "expected ~0, got {val}");
        }
    }

    #[test]
    fn test_alg_dct_uniform_exact_zeros() {
        // The Nayuki algorithm produces exact 0.0 for uniform input
        // because v[i] - v[n-1-i] = 0 exactly.
        let input = vec![255.0f64; 32];
        let result = alg_dct(&input).unwrap();
        assert_eq!(result[0], 255.0 * 32.0);
        for &val in &result[1..] {
            assert_eq!(val, 0.0, "expected exact 0.0, got {val}");
        }
    }

    #[test]
    fn test_alg_dct_range() {
        let input: Vec<f64> = (0..64).map(|i| i as f64).collect();
        let result = alg_dct(&input).unwrap();
        assert!((result[0] - 2016.0).abs() < 1e-10);
    }

    #[test]
    fn test_alg_dct_single() {
        let result = alg_dct(&[42.0]).unwrap();
        assert!((result[0] - 42.0).abs() < 1e-10);
    }

    #[test]
    fn test_alg_dct_known_values() {
        // DCT of [1, 2, 3, 4] matches Python reference
        let result = alg_dct(&[1.0, 2.0, 3.0, 4.0]).unwrap();
        assert!((result[0] - 10.0).abs() < 1e-10);
        assert!((result[1] - (-3.15432202989895)).abs() < 1e-10);
        assert!(result[2].abs() < 1e-10);
        assert!((result[3] - (-0.22417076458398263)).abs() < 1e-10);
    }
}
