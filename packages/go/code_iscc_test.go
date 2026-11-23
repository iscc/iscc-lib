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

// instanceUnit returns an Instance-Code of the given bits for "hello world", failing on error.
func instanceUnit(t *testing.T, bits uint32) string {
	t.Helper()
	res, err := GenInstanceCodeV0([]byte("hello world"), bits)
	if err != nil {
		t.Fatalf("GenInstanceCodeV0: %v", err)
	}
	return res.Iscc
}

func TestGenIsccCodeV0WideAbove256Data(t *testing.T) {
	codes := []string{overlongUnit(t, MTData, STNone), instanceUnit(t, 128)}
	_, err := GenIsccCodeV0(codes, true)
	assertDecodeBoundErr(t, err)
	// Standard mode reads no unit length, so the over-long Data unit passes (as iscc-core).
	if _, err := GenIsccCodeV0(codes, false); err != nil {
		t.Fatalf("GenIsccCodeV0 wide=false: %v", err)
	}
}

func TestGenIsccCodeV0WideAbove256InstanceAfterShortData(t *testing.T) {
	data, err := GenDataCodeV0([]byte("hello world"), 64)
	if err != nil {
		t.Fatalf("GenDataCodeV0: %v", err)
	}
	codes := []string{data.Iscc, overlongUnit(t, MTInstance, STNone)}
	// The 64-bit Data unit ends the wide check before the Instance length is read.
	wide, err := GenIsccCodeV0(codes, true)
	if err != nil {
		t.Fatalf("GenIsccCodeV0 wide=true: %v", err)
	}
	std, err := GenIsccCodeV0(codes, false)
	if err != nil {
		t.Fatalf("GenIsccCodeV0 wide=false: %v", err)
	}
	if wide.Iscc != std.Iscc {
		t.Errorf("wide=true got %q, want the standard code %q", wide.Iscc, std.Iscc)
	}
}

func TestGenIsccCodeV0WideAbove256InstanceAfterWideData(t *testing.T) {
	data, err := GenDataCodeV0([]byte("hello world"), 128)
	if err != nil {
		t.Fatalf("GenDataCodeV0: %v", err)
	}
	codes := []string{data.Iscc, overlongUnit(t, MTInstance, STNone)}
	// The 128-bit Data unit keeps the wide check going, so the Instance length is read.
	_, err = GenIsccCodeV0(codes, true)
	assertDecodeBoundErr(t, err)
}
