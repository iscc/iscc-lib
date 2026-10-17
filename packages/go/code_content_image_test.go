// Conformance tests for GenImageCodeV0 against data.json vectors.
package iscc

import (
	"bytes"
	"encoding/json"
	"os"
	"testing"
)

func TestPureGoGenImageCodeV0(t *testing.T) {
	data, err := os.ReadFile("../../crates/iscc-lib/tests/data.json")
	if err != nil {
		t.Fatalf("read data.json: %v", err)
	}

	allVectors, err := parseConformanceData(data)
	if err != nil {
		t.Fatalf("parse data.json: %v", err)
	}

	vectors, ok := allVectors["gen_image_code_v0"]
	if !ok {
		t.Fatal("gen_image_code_v0 section not found in data.json")
	}

	for name, vec := range vectors {
		t.Run(name, func(t *testing.T) {
			// Parse inputs: [pixels_array, bits]
			var pixelsFloat []float64
			if err := json.Unmarshal(vec.Inputs[0], &pixelsFloat); err != nil {
				t.Fatalf("parse pixels input: %v", err)
			}

			// Convert float64 JSON numbers to []byte
			pixels := make([]byte, len(pixelsFloat))
			for i, v := range pixelsFloat {
				pixels[i] = byte(v)
			}

			var bits float64
			if err := json.Unmarshal(vec.Inputs[1], &bits); err != nil {
				t.Fatalf("parse bits input: %v", err)
			}

			result, err := GenImageCodeV0(pixels, uint32(bits))
			if err != nil {
				t.Fatalf("GenImageCodeV0: %v", err)
			}

			// Verify ISCC code
			wantISCC, _ := vec.Outputs["iscc"].(string)
			if result.Iscc != wantISCC {
				t.Errorf("iscc: got %q, want %q", result.Iscc, wantISCC)
			}
		})
	}
}

// TestSoftHashImageV0QuadrantLayout checks that each 64-bit block of a 256-bit
// hash follows the top-left, top-right, bottom-left and bottom-right 8×8 DCT squares.
func TestSoftHashImageV0QuadrantLayout(t *testing.T) {
	data, err := os.ReadFile("../../crates/iscc-lib/tests/data.json")
	if err != nil {
		t.Fatalf("read data.json: %v", err)
	}
	allVectors, err := parseConformanceData(data)
	if err != nil {
		t.Fatalf("parse data.json: %v", err)
	}
	var pixelsFloat []float64
	if err := json.Unmarshal(allVectors["gen_image_code_v0"]["test_0003_img_256"].Inputs[0], &pixelsFloat); err != nil {
		t.Fatalf("parse pixels: %v", err)
	}
	pixels := make([]byte, len(pixelsFloat))
	for i, v := range pixelsFloat {
		pixels[i] = byte(v)
	}
	digest, err := softHashImageV0(pixels, 256)
	if err != nil {
		t.Fatalf("softHashImageV0: %v", err)
	}

	rows := make([][]float64, 32)
	for r := range rows {
		row := make([]float64, 32)
		for c := range row {
			row[c] = float64(pixels[r*32+c])
		}
		rows[r], _ = algDct(row)
	}
	cols := transposeMatrix(rows)
	for i, col := range cols {
		cols[i], _ = algDct(col)
	}
	matrix := transposeMatrix(cols)

	for i, pos := range [][2]int{{0, 0}, {8, 0}, {0, 8}, {8, 8}} {
		x, y := pos[0], pos[1]
		square := make([]float64, 0, 64)
		for r := y; r < y+8; r++ {
			square = append(square, matrix[r][x:x+8]...)
		}
		median := computeMedian(square)
		bits := make([]bool, len(square))
		for j, v := range square {
			bits[j] = v > median
		}
		if !bytes.Equal(digest[i*8:(i+1)*8], bitsToBytes(bits)) {
			t.Errorf("block %d does not match the 8×8 square at (%d, %d)", i, x, y)
		}
	}
}
