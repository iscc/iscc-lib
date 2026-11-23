// Conformance tests for GenImageCodeV0 against data.json vectors.
package iscc

import (
	"bytes"
	"testing"
)

func TestPureGoGenImageCodeV0(t *testing.T) {
	for name, vec := range conformanceSection(t, "gen_image_code_v0") {
		t.Run(name, func(t *testing.T) {
			pixels, bits, err := imageInputs(vec)
			if err != nil {
				t.Fatalf("parse inputs: %v", err)
			}
			wantISCC, err := expectedISCC(vec)
			if err != nil {
				t.Fatalf("parse iscc output: %v", err)
			}

			result, err := GenImageCodeV0(pixels, bits)
			if err != nil {
				t.Fatalf("GenImageCodeV0: %v", err)
			}

			if result.Iscc != wantISCC {
				t.Errorf("iscc: got %q, want %q", result.Iscc, wantISCC)
			}
		})
	}
}

// TestSoftHashImageV0QuadrantLayout checks that each 64-bit block of a 256-bit
// hash follows the top-left, top-right, bottom-left and bottom-right 8×8 DCT squares.
func TestSoftHashImageV0QuadrantLayout(t *testing.T) {
	pixels, _, err := imageInputs(conformanceSection(t, "gen_image_code_v0")["test_0003_img_256"])
	if err != nil {
		t.Fatalf("parse pixels: %v", err)
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

func TestGenImageCodeV0Above256(t *testing.T) {
	pixels := make([]byte, 1024)
	// softHashImageV0 rejects bits above 256 before encodeLength is reached.
	if res, err := GenImageCodeV0(pixels, 288); err == nil {
		t.Fatalf("GenImageCodeV0 288 bits: expected error, got %q", res.Iscc)
	}
	if _, err := GenImageCodeV0(pixels, 256); err != nil {
		t.Fatalf("GenImageCodeV0 256 bits: %v", err)
	}
}
