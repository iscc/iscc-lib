// Pure Go implementation of ISCC Image-Code (Content-Code for images).
// Generates a DCT-based perceptual hash from 32×32 grayscale pixel data.
// Matches the Rust gen_image_code_v0 implementation.
package iscc

import (
	"fmt"
	"sort"
)

// ImageCodeResult holds the output of GenImageCodeV0.
type ImageCodeResult struct {
	Iscc string // ISCC code string with "ISCC:" prefix
}

// GenImageCodeV0 generates an ISCC Content-Code for image content.
// Takes 1024 grayscale pixel values (32×32, values 0-255) and a bit length,
// computes a DCT-based perceptual hash, and encodes as an ISCC component.
func GenImageCodeV0(pixels []byte, bits uint32) (*ImageCodeResult, error) {
	hashDigest, err := softHashImageV0(pixels, bits)
	if err != nil {
		return nil, err
	}
	component, err := EncodeComponent(
		uint8(MTContent), uint8(STImage), uint8(VSV0), bits, hashDigest,
	)
	if err != nil {
		return nil, err
	}
	return &ImageCodeResult{
		Iscc: "ISCC:" + component,
	}, nil
}

// softHashImageV0 computes a DCT-based perceptual hash from 32×32 grayscale pixels.
// Applies a 2D DCT, extracts four 8×8 low-frequency blocks, and generates a
// bitstring by comparing each coefficient against the block median.
func softHashImageV0(pixels []byte, bits uint32) ([]byte, error) {
	if len(pixels) != 1024 {
		return nil, fmt.Errorf("expected 1024 pixels, got %d", len(pixels))
	}
	if bits > 256 {
		return nil, fmt.Errorf("bits must be <= 256, got %d", bits)
	}

	// Step 1: Row-wise DCT (32 rows of 32 pixels)
	rows := make([][]float64, 32)
	for i := 0; i < 32; i++ {
		rowF64 := make([]float64, 32)
		for j := 0; j < 32; j++ {
			rowF64[j] = float64(pixels[i*32+j])
		}
		dctRow, err := algDct(rowF64)
		if err != nil {
			return nil, fmt.Errorf("row DCT: %w", err)
		}
		rows[i] = dctRow
	}

	// Step 2: Transpose
	transposed := transposeMatrix(rows)

	// Step 3: Column-wise DCT
	dctCols := make([][]float64, len(transposed))
	for i, col := range transposed {
		dctCol, err := algDct(col)
		if err != nil {
			return nil, fmt.Errorf("col DCT: %w", err)
		}
		dctCols[i] = dctCol
	}

	// Step 4: Transpose back
	dctMatrix := transposeMatrix(dctCols)

	// Step 5: Extract 8×8 blocks at positions (col, row): (0,0), (1,0), (0,1), (1,1)
	positions := [][2]int{{0, 0}, {1, 0}, {0, 1}, {1, 1}}
	bitstring := make([]bool, 0, 256)

	for _, pos := range positions {
		flat := flatten8x8(dctMatrix, pos[0], pos[1])
		median := computeMedian(flat)
		for _, val := range flat {
			bitstring = append(bitstring, val > median)
		}
		if len(bitstring) >= int(bits) {
			break
		}
	}

	// Step 6: Convert first `bits` bools to bytes
	return bitsToBytes(bitstring[:bits]), nil
}

// transposeMatrix swaps rows and columns of a 2D float64 matrix.
func transposeMatrix(matrix [][]float64) [][]float64 {
	nRows := len(matrix)
	if nRows == 0 {
		return nil
	}
	nCols := len(matrix[0])
	result := make([][]float64, nCols)
	for c := 0; c < nCols; c++ {
		result[c] = make([]float64, nRows)
		for r := 0; r < nRows; r++ {
			result[c][r] = matrix[r][c]
		}
	}
	return result
}

// flatten8x8 extracts an 8×8 block from a matrix at position (col, row) and flattens it.
func flatten8x8(matrix [][]float64, col, row int) []float64 {
	flat := make([]float64, 0, 64)
	for r := row; r < row+8 && r < len(matrix); r++ {
		for c := col; c < col+8 && c < len(matrix[r]); c++ {
			flat = append(flat, matrix[r][c])
		}
	}
	return flat
}

// computeMedian returns the median of a float64 slice.
// For even-length slices, returns the average of the two middle values.
func computeMedian(values []float64) float64 {
	sorted := make([]float64, len(values))
	copy(sorted, values)
	sort.Float64s(sorted)
	n := len(sorted)
	if n%2 == 1 {
		return sorted[n/2]
	}
	return (sorted[n/2-1] + sorted[n/2]) / 2.0
}
