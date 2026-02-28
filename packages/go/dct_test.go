// Tests for the pure Go DCT module.
// Ported from crates/iscc-lib/src/dct.rs tests.
package iscc

import (
	"math"
	"testing"
)

func TestAlgDctEmptyError(t *testing.T) {
	_, err := algDct([]float64{})
	if err == nil {
		t.Fatal("expected error for empty input")
	}
}

func TestAlgDctOddLengthError(t *testing.T) {
	_, err := algDct([]float64{1.0, 2.0, 3.0})
	if err == nil {
		t.Fatal("expected error for odd length input")
	}
}

func TestAlgDctAllZeros(t *testing.T) {
	input := make([]float64, 64)
	result, err := algDct(input)
	if err != nil {
		t.Fatal(err)
	}
	for i, val := range result {
		if math.Abs(val) >= 1e-10 {
			t.Errorf("index %d: expected ~0, got %v", i, val)
		}
	}
}

func TestAlgDctAllOnes(t *testing.T) {
	input := make([]float64, 64)
	for i := range input {
		input[i] = 1.0
	}
	result, err := algDct(input)
	if err != nil {
		t.Fatal(err)
	}
	if math.Abs(result[0]-64.0) >= 1e-10 {
		t.Errorf("result[0]: expected 64.0, got %v", result[0])
	}
	for i := 1; i < len(result); i++ {
		if math.Abs(result[i]) >= 1e-10 {
			t.Errorf("index %d: expected ~0, got %v", i, result[i])
		}
	}
}

func TestAlgDctUniformExactZeros(t *testing.T) {
	// The Nayuki algorithm produces exact 0.0 for uniform input
	// because v[i] - v[n-1-i] = 0 exactly.
	input := make([]float64, 32)
	for i := range input {
		input[i] = 255.0
	}
	result, err := algDct(input)
	if err != nil {
		t.Fatal(err)
	}
	if result[0] != 255.0*32.0 {
		t.Errorf("result[0]: expected %v, got %v", 255.0*32.0, result[0])
	}
	for i := 1; i < len(result); i++ {
		if result[i] != 0.0 {
			t.Errorf("index %d: expected exact 0.0, got %v", i, result[i])
		}
	}
}

func TestAlgDctRange(t *testing.T) {
	input := make([]float64, 64)
	for i := range input {
		input[i] = float64(i)
	}
	result, err := algDct(input)
	if err != nil {
		t.Fatal(err)
	}
	if math.Abs(result[0]-2016.0) >= 1e-10 {
		t.Errorf("result[0]: expected ~2016.0, got %v", result[0])
	}
}

func TestAlgDctSingle(t *testing.T) {
	result, err := algDct([]float64{42.0})
	if err != nil {
		t.Fatal(err)
	}
	if math.Abs(result[0]-42.0) >= 1e-10 {
		t.Errorf("result[0]: expected 42.0, got %v", result[0])
	}
}

func TestAlgDctNonPowerOfTwoEvenError(t *testing.T) {
	// Length 6 is even but not a power of 2
	_, err := algDct([]float64{1.0, 2.0, 3.0, 4.0, 5.0, 6.0})
	if err == nil {
		t.Fatal("expected error for length 6")
	}
	// Length 10 is even but not a power of 2
	_, err = algDct(make([]float64, 10))
	if err == nil {
		t.Fatal("expected error for length 10")
	}
	// Length 12 is even but not a power of 2
	_, err = algDct(make([]float64, 12))
	if err == nil {
		t.Fatal("expected error for length 12")
	}
}

func TestAlgDctLength2Ok(t *testing.T) {
	result, err := algDct([]float64{1.0, 2.0})
	if err != nil {
		t.Fatal("expected no error for length 2")
	}
	if len(result) != 2 {
		t.Fatalf("length: got %d, want 2", len(result))
	}
}

func TestAlgDctKnownValues(t *testing.T) {
	// DCT of [1, 2, 3, 4] matches Python reference
	result, err := algDct([]float64{1.0, 2.0, 3.0, 4.0})
	if err != nil {
		t.Fatal(err)
	}
	if math.Abs(result[0]-10.0) >= 1e-10 {
		t.Errorf("result[0]: expected 10.0, got %v", result[0])
	}
	if math.Abs(result[1]-(-3.15432202989895)) >= 1e-10 {
		t.Errorf("result[1]: expected -3.15432..., got %v", result[1])
	}
	if math.Abs(result[2]) >= 1e-10 {
		t.Errorf("result[2]: expected ~0, got %v", result[2])
	}
	if math.Abs(result[3]-(-0.22417076458398263)) >= 1e-10 {
		t.Errorf("result[3]: expected -0.22417..., got %v", result[3])
	}
}
