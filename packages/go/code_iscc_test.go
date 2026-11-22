// Conformance tests for GenIsccCodeV0 against data.json vectors.
package iscc

import "testing"

func TestPureGoGenIsccCodeV0(t *testing.T) {
	for name, vec := range conformanceSection(t, "gen_iscc_code_v0") {
		t.Run(name, func(t *testing.T) {
			codes, err := isccInputs(vec)
			if err != nil {
				t.Fatalf("parse inputs: %v", err)
			}
			wantISCC, err := expectedISCC(vec)
			if err != nil {
				t.Fatalf("parse iscc output: %v", err)
			}

			result, err := GenIsccCodeV0(codes, false)
			if err != nil {
				t.Fatalf("GenIsccCodeV0: %v", err)
			}

			if result.Iscc != wantISCC {
				t.Errorf("iscc: got %q, want %q", result.Iscc, wantISCC)
			}
		})
	}
}
