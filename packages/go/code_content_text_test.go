// Conformance tests for GenTextCodeV0 against data.json vectors.
package iscc

import (
	"encoding/json"
	"os"
	"testing"
)

func TestPureGoGenTextCodeV0(t *testing.T) {
	data, err := os.ReadFile("../../crates/iscc-lib/tests/data.json")
	if err != nil {
		t.Fatalf("read data.json: %v", err)
	}

	var allVectors map[string]map[string]struct {
		Inputs  []json.RawMessage      `json:"inputs"`
		Outputs map[string]interface{} `json:"outputs"`
	}
	if err := json.Unmarshal(data, &allVectors); err != nil {
		t.Fatalf("parse data.json: %v", err)
	}

	vectors, ok := allVectors["gen_text_code_v0"]
	if !ok {
		t.Fatal("gen_text_code_v0 section not found in data.json")
	}

	for name, vec := range vectors {
		t.Run(name, func(t *testing.T) {
			// Parse inputs: [text, bits]
			var text string
			if err := json.Unmarshal(vec.Inputs[0], &text); err != nil {
				t.Fatalf("parse text input: %v", err)
			}
			var bits float64
			if err := json.Unmarshal(vec.Inputs[1], &bits); err != nil {
				t.Fatalf("parse bits input: %v", err)
			}

			result, err := GenTextCodeV0(text, uint32(bits))
			if err != nil {
				t.Fatalf("GenTextCodeV0: %v", err)
			}

			// Verify ISCC code
			wantISCC, _ := vec.Outputs["iscc"].(string)
			if result.Iscc != wantISCC {
				t.Errorf("iscc: got %q, want %q", result.Iscc, wantISCC)
			}

			// Verify character count
			wantChars, _ := vec.Outputs["characters"].(float64)
			if result.Characters != int(wantChars) {
				t.Errorf("characters: got %d, want %d", result.Characters, int(wantChars))
			}
		})
	}
}
