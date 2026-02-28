// Conformance tests for GenVideoCodeV0 against data.json vectors.
package iscc

import (
	"encoding/json"
	"os"
	"testing"
)

func TestPureGoGenVideoCodeV0(t *testing.T) {
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

	vectors, ok := allVectors["gen_video_code_v0"]
	if !ok {
		t.Fatal("gen_video_code_v0 section not found in data.json")
	}

	for name, vec := range vectors {
		t.Run(name, func(t *testing.T) {
			// Parse inputs: [frame_sigs_array, bits]
			var rawFrameSigs [][]float64
			if err := json.Unmarshal(vec.Inputs[0], &rawFrameSigs); err != nil {
				t.Fatalf("parse frame_sigs input: %v", err)
			}

			// Convert float64 JSON numbers to [][]int32
			frameSigs := make([][]int32, len(rawFrameSigs))
			for i, rawSig := range rawFrameSigs {
				sig := make([]int32, len(rawSig))
				for j, v := range rawSig {
					sig[j] = int32(v)
				}
				frameSigs[i] = sig
			}

			var bits float64
			if err := json.Unmarshal(vec.Inputs[1], &bits); err != nil {
				t.Fatalf("parse bits input: %v", err)
			}

			result, err := GenVideoCodeV0(frameSigs, uint32(bits))
			if err != nil {
				t.Fatalf("GenVideoCodeV0: %v", err)
			}

			// Verify ISCC code
			wantISCC, _ := vec.Outputs["iscc"].(string)
			if result.Iscc != wantISCC {
				t.Errorf("iscc: got %q, want %q", result.Iscc, wantISCC)
			}
		})
	}
}
