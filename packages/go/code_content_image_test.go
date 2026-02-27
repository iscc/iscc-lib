// Conformance tests for GenImageCodeV0 against data.json vectors.
package iscc

import (
	"encoding/json"
	"os"
	"testing"
)

func TestPureGoGenImageCodeV0(t *testing.T) {
	data, err := os.ReadFile("../../crates/iscc-lib/tests/data.json")
	if err != nil {
		t.Fatalf("read data.json: %v", err)
	}

	var allVectors map[string]map[string]struct {
		Inputs  []json.RawMessage     `json:"inputs"`
		Outputs map[string]interface{} `json:"outputs"`
	}
	if err := json.Unmarshal(data, &allVectors); err != nil {
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
