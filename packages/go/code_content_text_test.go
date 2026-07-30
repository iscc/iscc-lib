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

	allVectors, err := parseConformanceData(data)
	if err != nil {
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

func TestPureGoGenTextCodeV0FinalSigma(t *testing.T) {
	// End-to-end Final_Sigma regression: Greek text with word-final capital
	// sigmas must produce the same Text-Code as the Rust core / iscc-core.
	result, err := GenTextCodeV0("ΤΟ ΓΡΗΓΟΡΟ ΚΑΦΕ ΑΛΕΠΟΥ ΠΗΔΑΕΙ ΠΑΝΩ ΑΠΟ ΤΟΝ ΤΕΜΠΕΛΗ ΣΚΥΛΟΣ", 64)
	if err != nil {
		t.Fatalf("GenTextCodeV0: %v", err)
	}
	if result.Iscc != "ISCC:EAA36O3AT3YMFRFU" {
		t.Errorf("iscc: got %q, want %q", result.Iscc, "ISCC:EAA36O3AT3YMFRFU")
	}
	if result.Characters != 48 {
		t.Errorf("characters: got %d, want 48", result.Characters)
	}
}
