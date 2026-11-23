// Conformance tests for GenMixedCodeV0 against data.json vectors.
package iscc

import (
	"encoding/json"
	"testing"
)

func TestPureGoGenMixedCodeV0(t *testing.T) {
	for name, vec := range conformanceSection(t, "gen_mixed_code_v0") {
		t.Run(name, func(t *testing.T) {
			codes, bits, err := mixedInputs(vec)
			if err != nil {
				t.Fatalf("parse inputs: %v", err)
			}
			wantISCC, err := expectedISCC(vec)
			if err != nil {
				t.Fatalf("parse iscc output: %v", err)
			}
			var want struct {
				Parts []string `json:"parts"`
			}
			if err := json.Unmarshal(vec.Outputs, &want); err != nil {
				t.Fatalf("parse outputs: %v", err)
			}

			result, err := GenMixedCodeV0(codes, bits)
			if err != nil {
				t.Fatalf("GenMixedCodeV0: %v", err)
			}

			if result.Iscc != wantISCC {
				t.Errorf("iscc: got %q, want %q", result.Iscc, wantISCC)
			}
			if len(result.Parts) != len(want.Parts) {
				t.Fatalf("parts length: got %d, want %d", len(result.Parts), len(want.Parts))
			}
			for i, wantPart := range want.Parts {
				if result.Parts[i] != wantPart {
					t.Errorf("parts[%d]: got %q, want %q", i, result.Parts[i], wantPart)
				}
			}
		})
	}
}

func TestGenMixedCodeV0Above256Unit(t *testing.T) {
	valid, err := GenTextCodeV0("Hello World", 64)
	if err != nil {
		t.Fatalf("GenTextCodeV0: %v", err)
	}
	codes := []string{overlongUnit(t, MTContent, STText), valid.Iscc}
	_, err = GenMixedCodeV0(codes, 64)
	assertDecodeBoundErr(t, err)
}

func TestGenMixedCodeV0BitsBelowOneByte(t *testing.T) {
	codes := []string{"EUA6GIKXN42IQV3S", "EIAUKMOUIOYZCKA5"}
	for bits := uint32(0); bits < 8; bits++ {
		if _, err := GenMixedCodeV0(codes, bits); err == nil {
			t.Errorf("bits %d: expected an error", bits)
		}
	}
}
