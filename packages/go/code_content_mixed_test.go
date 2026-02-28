// Conformance tests for GenMixedCodeV0 against data.json vectors.
package iscc

import (
	"encoding/json"
	"os"
	"testing"
)

func TestPureGoGenMixedCodeV0(t *testing.T) {
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

	vectors, ok := allVectors["gen_mixed_code_v0"]
	if !ok {
		t.Fatal("gen_mixed_code_v0 section not found in data.json")
	}

	for name, vec := range vectors {
		t.Run(name, func(t *testing.T) {
			// Parse inputs: [codes_array, bits]
			var codes []string
			if err := json.Unmarshal(vec.Inputs[0], &codes); err != nil {
				t.Fatalf("parse codes input: %v", err)
			}

			var bits float64
			if err := json.Unmarshal(vec.Inputs[1], &bits); err != nil {
				t.Fatalf("parse bits input: %v", err)
			}

			result, err := GenMixedCodeV0(codes, uint32(bits))
			if err != nil {
				t.Fatalf("GenMixedCodeV0: %v", err)
			}

			// Verify ISCC code
			wantISCC, _ := vec.Outputs["iscc"].(string)
			if result.Iscc != wantISCC {
				t.Errorf("iscc: got %q, want %q", result.Iscc, wantISCC)
			}

			// Verify parts
			wantParts, _ := vec.Outputs["parts"].([]interface{})
			if len(result.Parts) != len(wantParts) {
				t.Fatalf("parts length: got %d, want %d", len(result.Parts), len(wantParts))
			}
			for i, wp := range wantParts {
				wantStr, _ := wp.(string)
				if result.Parts[i] != wantStr {
					t.Errorf("parts[%d]: got %q, want %q", i, result.Parts[i], wantStr)
				}
			}
		})
	}
}
