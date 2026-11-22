// Conformance tests for GenAudioCodeV0 against data.json vectors.
package iscc

import "testing"

func TestPureGoGenAudioCodeV0(t *testing.T) {
	for name, vec := range conformanceSection(t, "gen_audio_code_v0") {
		t.Run(name, func(t *testing.T) {
			cv, bits, err := audioInputs(vec)
			if err != nil {
				t.Fatalf("parse inputs: %v", err)
			}
			wantISCC, err := expectedISCC(vec)
			if err != nil {
				t.Fatalf("parse iscc output: %v", err)
			}

			result, err := GenAudioCodeV0(cv, bits)
			if err != nil {
				t.Fatalf("GenAudioCodeV0: %v", err)
			}

			if result.Iscc != wantISCC {
				t.Errorf("iscc: got %q, want %q", result.Iscc, wantISCC)
			}
		})
	}
}
