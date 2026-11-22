// Conformance tests for GenTextCodeV0 against data.json vectors.
package iscc

import (
	"encoding/json"
	"testing"
)

func TestPureGoGenTextCodeV0(t *testing.T) {
	for name, vec := range conformanceSection(t, "gen_text_code_v0") {
		t.Run(name, func(t *testing.T) {
			text, bits, err := textInputs(vec)
			if err != nil {
				t.Fatalf("parse inputs: %v", err)
			}
			wantISCC, err := expectedISCC(vec)
			if err != nil {
				t.Fatalf("parse iscc output: %v", err)
			}
			var want struct {
				Characters int `json:"characters"`
			}
			if err := json.Unmarshal(vec.Outputs, &want); err != nil {
				t.Fatalf("parse outputs: %v", err)
			}

			result, err := GenTextCodeV0(text, bits)
			if err != nil {
				t.Fatalf("GenTextCodeV0: %v", err)
			}

			if result.Iscc != wantISCC {
				t.Errorf("iscc: got %q, want %q", result.Iscc, wantISCC)
			}
			if result.Characters != want.Characters {
				t.Errorf("characters: got %d, want %d", result.Characters, want.Characters)
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

func TestGenTextCodeV0Above256(t *testing.T) {
	_, err := GenTextCodeV0("Hello World", 288)
	assertEncodeBoundErr(t, err)
	if _, err := GenTextCodeV0("Hello World", 256); err != nil {
		t.Fatalf("GenTextCodeV0 256 bits: %v", err)
	}
}
