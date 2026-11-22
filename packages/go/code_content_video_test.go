// Conformance tests for GenVideoCodeV0 against data.json vectors.
package iscc

import "testing"

func TestPureGoGenVideoCodeV0(t *testing.T) {
	for name, vec := range conformanceSection(t, "gen_video_code_v0") {
		t.Run(name, func(t *testing.T) {
			frameSigs, bits, err := videoInputs(vec)
			if err != nil {
				t.Fatalf("parse inputs: %v", err)
			}
			wantISCC, err := expectedISCC(vec)
			if err != nil {
				t.Fatalf("parse iscc output: %v", err)
			}

			result, err := GenVideoCodeV0(frameSigs, bits)
			if err != nil {
				t.Fatalf("GenVideoCodeV0: %v", err)
			}

			if result.Iscc != wantISCC {
				t.Errorf("iscc: got %q, want %q", result.Iscc, wantISCC)
			}
		})
	}
}

// zeroSig returns an all-zero frame signature of length n.
func zeroSig(n int) []int32 {
	return make([]int32, n)
}

// assertSoftHashVideoErr calls SoftHashVideoV0 once and checks the exact error.
func assertSoftHashVideoErr(t *testing.T, frameSigs [][]int32, want string) {
	t.Helper()
	digest, err := SoftHashVideoV0(frameSigs, 64)
	if err == nil {
		t.Fatalf("SoftHashVideoV0: expected error, got digest %x", digest)
	}
	if err.Error() != want {
		t.Errorf("SoftHashVideoV0 error: got %q, want %q", err.Error(), want)
	}
}

func TestSoftHashVideoV0RejectsLongerLaterSig(t *testing.T) {
	assertSoftHashVideoErr(t, [][]int32{zeroSig(380), zeroSig(381)},
		"frame_sigs[1] has length 381, expected 380")
}

func TestSoftHashVideoV0RejectsShorterLaterSig(t *testing.T) {
	assertSoftHashVideoErr(t, [][]int32{zeroSig(380), zeroSig(379)},
		"frame_sigs[1] has length 379, expected 380")
}

func TestSoftHashVideoV0RejectsRaggedReversedOrder(t *testing.T) {
	assertSoftHashVideoErr(t, [][]int32{zeroSig(381), zeroSig(380)},
		"frame_sigs[1] has length 380, expected 381")
}

func TestSoftHashVideoV0ReportsFirstMismatch(t *testing.T) {
	assertSoftHashVideoErr(t, [][]int32{zeroSig(380), zeroSig(380), zeroSig(382), zeroSig(1)},
		"frame_sigs[2] has length 382, expected 380")
}

func TestGenVideoCodeV0RejectsRaggedFrames(t *testing.T) {
	result, err := GenVideoCodeV0([][]int32{zeroSig(380), zeroSig(381)}, 64)
	if err == nil {
		t.Fatalf("GenVideoCodeV0: expected error, got %q", result.Iscc)
	}
}

func TestSoftHashVideoV0EqualLengthDuplicates(t *testing.T) {
	single, err := SoftHashVideoV0([][]int32{zeroSig(380)}, 64)
	if err != nil {
		t.Fatalf("SoftHashVideoV0 single: %v", err)
	}
	doubled, err := SoftHashVideoV0([][]int32{zeroSig(380), zeroSig(380)}, 64)
	if err != nil {
		t.Fatalf("SoftHashVideoV0 doubled: %v", err)
	}
	if string(single) != string(doubled) {
		t.Errorf("duplicates changed digest: %x vs %x", single, doubled)
	}
}
