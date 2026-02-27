// Tests for the pure Go SimHash and SlidingWindow module.
// These tests do NOT require the WASM binary — they test pure Go functions.
package iscc

import "testing"

// ---- AlgSimhash tests ----

func TestSimhashSingleDigest(t *testing.T) {
	// Single digest → passthrough (result equals the input digest)
	digest := make([]byte, 32)
	for i := range digest {
		digest[i] = byte(i)
	}
	result, err := AlgSimhash([][]byte{digest})
	if err != nil {
		t.Fatal(err)
	}
	if len(result) != 32 {
		t.Fatalf("length: got %d, want 32", len(result))
	}
	for i := range digest {
		if result[i] != digest[i] {
			t.Errorf("byte %d: got 0x%02X, want 0x%02X", i, result[i], digest[i])
		}
	}
}

func TestSimhashEmpty(t *testing.T) {
	// Empty input → 32 zero bytes
	result, err := AlgSimhash([][]byte{})
	if err != nil {
		t.Fatal(err)
	}
	if len(result) != 32 {
		t.Fatalf("length: got %d, want 32", len(result))
	}
	for i, b := range result {
		if b != 0 {
			t.Errorf("byte %d: got 0x%02X, want 0x00", i, b)
		}
	}
}

func TestSimhashIdenticalDigests(t *testing.T) {
	// Three identical all-ones digests → all-ones result
	digest := make([]byte, 32)
	for i := range digest {
		digest[i] = 0xFF
	}
	d2 := make([]byte, 32)
	copy(d2, digest)
	d3 := make([]byte, 32)
	copy(d3, digest)
	result, err := AlgSimhash([][]byte{digest, d2, d3})
	if err != nil {
		t.Fatal(err)
	}
	for i, b := range result {
		if b != 0xFF {
			t.Errorf("byte %d: got 0x%02X, want 0xFF", i, b)
		}
	}
}

func TestSimhashOppositeDigests(t *testing.T) {
	// Two digests: all-ones and all-zeros.
	// Each bit has count 1 (from all-ones) or 0 (from all-zeros).
	// count*2 >= 2 → count >= 1. Bits from all-ones survive.
	ones := make([]byte, 32)
	for i := range ones {
		ones[i] = 0xFF
	}
	zeros := make([]byte, 32)
	result, err := AlgSimhash([][]byte{ones, zeros})
	if err != nil {
		t.Fatal(err)
	}
	for i, b := range result {
		if b != 0xFF {
			t.Errorf("byte %d: got 0x%02X, want 0xFF", i, b)
		}
	}
}

func TestSimhashMismatchedLengths(t *testing.T) {
	// Mismatched digest lengths should return error
	_, err := AlgSimhash([][]byte{{1, 2}, {1, 2, 3}})
	if err == nil {
		t.Fatal("expected error for mismatched lengths")
	}
	msg := err.Error()
	if msg == "" {
		t.Error("expected non-empty error message")
	}
}

func TestSimhash4ByteDigests(t *testing.T) {
	// SimHash with 4-byte digests (used in audio code)
	d1 := []byte{0xFF, 0x00, 0xFF, 0x00}
	d2 := []byte{0xFF, 0x00, 0xFF, 0x00}
	d3 := []byte{0x00, 0xFF, 0x00, 0xFF}
	result, err := AlgSimhash([][]byte{d1, d2, d3})
	if err != nil {
		t.Fatal(err)
	}
	if len(result) != 4 {
		t.Fatalf("length: got %d, want 4", len(result))
	}
	// Two agree on 0xFF/0x00 pattern, so majority wins
	if result[0] != 0xFF || result[1] != 0x00 || result[2] != 0xFF || result[3] != 0x00 {
		t.Errorf("result: got %v, want [0xFF 0x00 0xFF 0x00]", result)
	}
}

// ---- SlidingWindow tests ----

func TestSlidingWindowBasic(t *testing.T) {
	result, err := SlidingWindow("Hello", 4)
	if err != nil {
		t.Fatal(err)
	}
	expected := []string{"Hell", "ello"}
	if len(result) != len(expected) {
		t.Fatalf("length: got %d, want %d", len(result), len(expected))
	}
	for i := range expected {
		if result[i] != expected[i] {
			t.Errorf("window %d: got %q, want %q", i, result[i], expected[i])
		}
	}
}

func TestSlidingWindowShorterThanWidth(t *testing.T) {
	result, err := SlidingWindow("ab", 3)
	if err != nil {
		t.Fatal(err)
	}
	if len(result) != 1 {
		t.Fatalf("length: got %d, want 1", len(result))
	}
	if result[0] != "ab" {
		t.Errorf("got %q, want %q", result[0], "ab")
	}
}

func TestSlidingWindowExactWidth(t *testing.T) {
	result, err := SlidingWindow("abc", 3)
	if err != nil {
		t.Fatal(err)
	}
	if len(result) != 1 {
		t.Fatalf("length: got %d, want 1", len(result))
	}
	if result[0] != "abc" {
		t.Errorf("got %q, want %q", result[0], "abc")
	}
}

func TestSlidingWindowEmpty(t *testing.T) {
	result, err := SlidingWindow("", 3)
	if err != nil {
		t.Fatal(err)
	}
	if len(result) != 1 {
		t.Fatalf("length: got %d, want 1", len(result))
	}
	if result[0] != "" {
		t.Errorf("got %q, want %q", result[0], "")
	}
}

func TestSlidingWindowUnicode(t *testing.T) {
	result, err := SlidingWindow("äöü", 2)
	if err != nil {
		t.Fatal(err)
	}
	expected := []string{"äö", "öü"}
	if len(result) != len(expected) {
		t.Fatalf("length: got %d, want %d", len(result), len(expected))
	}
	for i := range expected {
		if result[i] != expected[i] {
			t.Errorf("window %d: got %q, want %q", i, result[i], expected[i])
		}
	}
}

func TestSlidingWindowWidthTooSmall(t *testing.T) {
	_, err := SlidingWindow("test", 1)
	if err == nil {
		t.Fatal("expected error for width < 2")
	}
}

func TestSlidingWindowWidth13(t *testing.T) {
	// Width 13 used by ISCC Text-Code
	result, err := SlidingWindow("Hello World!", 13)
	if err != nil {
		t.Fatal(err)
	}
	// 12 chars, width 13 → single element
	if len(result) != 1 {
		t.Fatalf("length: got %d, want 1", len(result))
	}
	if result[0] != "Hello World!" {
		t.Errorf("got %q, want %q", result[0], "Hello World!")
	}
}

func TestSlidingWindowLongerString(t *testing.T) {
	result, err := SlidingWindow("abcdef", 4)
	if err != nil {
		t.Fatal(err)
	}
	expected := []string{"abcd", "bcde", "cdef"}
	if len(result) != len(expected) {
		t.Fatalf("length: got %d, want %d", len(result), len(expected))
	}
	for i := range expected {
		if result[i] != expected[i] {
			t.Errorf("window %d: got %q, want %q", i, result[i], expected[i])
		}
	}
}
