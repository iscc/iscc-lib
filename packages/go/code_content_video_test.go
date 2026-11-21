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

	allVectors, err := parseConformanceData(data)
	if err != nil {
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
