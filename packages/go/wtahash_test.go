// Tests for the pure Go WTA-Hash module.
// Ported from crates/iscc-lib/src/wtahash.rs tests.
package iscc

import "testing"

func TestAlgWtahashAllZeros(t *testing.T) {
	// All-zero input should produce all-zero output (all comparisons are >=)
	vec := make([]int64, 380)
	result, err := AlgWtahash(vec, 64)
	if err != nil {
		t.Fatal(err)
	}
	for i, b := range result {
		if b != 0 {
			t.Errorf("byte %d: got 0x%02X, want 0x00", i, b)
		}
	}
}

func TestAlgWtahashRange(t *testing.T) {
	// Range 0..380 should produce deterministic non-zero output
	vec := make([]int64, 380)
	for i := range vec {
		vec[i] = int64(i)
	}
	result, err := AlgWtahash(vec, 64)
	if err != nil {
		t.Fatal(err)
	}
	if len(result) != 8 {
		t.Fatalf("length: got %d, want 8", len(result))
	}
	hasNonZero := false
	for _, b := range result {
		if b != 0 {
			hasNonZero = true
			break
		}
	}
	if !hasNonZero {
		t.Error("expected non-zero output for range input")
	}
}

func TestAlgWtahash256Bits(t *testing.T) {
	// 256-bit output uses all 256 permutation pairs
	vec := make([]int64, 380)
	for i := range vec {
		vec[i] = int64(i)
	}
	result, err := AlgWtahash(vec, 256)
	if err != nil {
		t.Fatal(err)
	}
	if len(result) != 32 {
		t.Fatalf("length: got %d, want 32", len(result))
	}
}

func TestAlgWtahashShortInputError(t *testing.T) {
	// Input with fewer than 380 elements must be rejected
	vec := make([]int64, 100)
	_, err := AlgWtahash(vec, 64)
	if err == nil {
		t.Fatal("expected error for short input")
	}
}

func TestAlgWtahashZeroBitsError(t *testing.T) {
	// bits = 0 is invalid
	vec := make([]int64, 380)
	_, err := AlgWtahash(vec, 0)
	if err == nil {
		t.Fatal("expected error for bits=0")
	}
}

func TestAlgWtahashNonDivisibleBitsError(t *testing.T) {
	// bits = 7 is not divisible by 8
	vec := make([]int64, 380)
	_, err := AlgWtahash(vec, 7)
	if err == nil {
		t.Fatal("expected error for bits=7")
	}
}

func TestAlgWtahashExceedsPermutationsError(t *testing.T) {
	// bits = 512 exceeds the 256-entry permutation table
	vec := make([]int64, 380)
	_, err := AlgWtahash(vec, 512)
	if err == nil {
		t.Fatal("expected error for bits=512")
	}
}

func TestPermutationTableLength(t *testing.T) {
	// The permutation table has exactly 256 entries
	if len(wtaVideoIdPermutations) != 256 {
		t.Fatalf("table length: got %d, want 256", len(wtaVideoIdPermutations))
	}
}

func TestPermutationIndicesInRange(t *testing.T) {
	// All permutation indices must be < 380 (frame signature length)
	for idx, pair := range wtaVideoIdPermutations {
		if pair[0] >= 380 {
			t.Errorf("entry %d: i=%d out of range", idx, pair[0])
		}
		if pair[1] >= 380 {
			t.Errorf("entry %d: j=%d out of range", idx, pair[1])
		}
	}
}
