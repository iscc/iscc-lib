// Pure Go implementation of the fast Discrete Cosine Transform (Nayuki algorithm) for ISCC.
// Used by image content hashing. Ported from crates/iscc-lib/src/dct.rs.
// See: https://www.nayuki.io/page/fast-discrete-cosine-transform-algorithms
package iscc

import (
	"fmt"
	"math"
)

// algDct computes the fast Discrete Cosine Transform using the Nayuki algorithm.
// Input length must be a power of 2 and > 0. Returns f64 values matching the
// reference implementation's floating-point behavior.
func algDct(v []float64) ([]float64, error) {
	n := len(v)
	if n == 0 || n&(n-1) != 0 {
		return nil, fmt.Errorf("DCT input length must be a power of 2, got %d", n)
	}
	return dctRecursive(v), nil
}

// dctRecursive implements the recursive fast DCT divide-and-conquer.
func dctRecursive(v []float64) []float64 {
	n := len(v)
	if n == 1 {
		return []float64{v[0]}
	}

	half := n / 2

	// Split into symmetric (alpha) and antisymmetric (beta) parts
	alpha := make([]float64, half)
	beta := make([]float64, half)
	for i := 0; i < half; i++ {
		alpha[i] = v[i] + v[n-1-i]
		beta[i] = (v[i] - v[n-1-i]) / (math.Cos((float64(i)+0.5)*math.Pi/float64(n)) * 2.0)
	}

	alpha = dctRecursive(alpha)
	beta = dctRecursive(beta)

	// Interleave results
	result := make([]float64, 0, n)
	for i := 0; i < half-1; i++ {
		result = append(result, alpha[i])
		result = append(result, beta[i]+beta[i+1])
	}
	result = append(result, alpha[half-1])
	result = append(result, beta[half-1])

	return result
}
