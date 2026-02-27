// Tests for the pure Go MinHash module.
// These tests do NOT require the WASM binary — they test pure Go functions.
package iscc

import "testing"

// ---- minhash tests ----

func TestMinhashEmptyFeatures(t *testing.T) {
	result := minhashFn([]uint32{})
	if len(result) != 64 {
		t.Fatalf("result length: got %d, want 64", len(result))
	}
	for i, v := range result {
		if v != maxH {
			t.Errorf("dimension %d: got %d, want %d (maxH)", i, v, maxH)
		}
	}
}

func TestMinhashSingleFeature(t *testing.T) {
	result := minhashFn([]uint32{42})
	if len(result) != 64 {
		t.Fatalf("result length: got %d, want 64", len(result))
	}
	// Each dimension should have a specific hash value within maxH
	for i, v := range result {
		if v > maxH {
			t.Errorf("dimension %d: value %d exceeds maxH %d", i, v, maxH)
		}
	}
}

func TestMinhashDeterministic(t *testing.T) {
	features := []uint32{100, 200, 300, 400, 500}
	r1 := minhashFn(features)
	r2 := minhashFn(features)
	for i := range r1 {
		if r1[i] != r2[i] {
			t.Errorf("dimension %d: %d != %d", i, r1[i], r2[i])
		}
	}
}

// ---- minhashCompress tests ----

func TestMinhashCompressBasic(t *testing.T) {
	// With lsb=1, extracting 1 bit from each of 64 hash values → 8 bytes
	mhash := make([]uint64, 64)
	result := minhashCompress(mhash, 1)
	if len(result) != 8 {
		t.Fatalf("compress basic length: got %d, want 8", len(result))
	}
	for i, b := range result {
		if b != 0 {
			t.Errorf("byte %d: got %d, want 0", i, b)
		}
	}
}

func TestMinhashCompressAllOnes(t *testing.T) {
	// All hash values are 0xFFFFFFFF, lsb=4 → 32 bytes, all bits set
	mhash := make([]uint64, 64)
	for i := range mhash {
		mhash[i] = maxH
	}
	result := minhashCompress(mhash, 4)
	if len(result) != 32 {
		t.Fatalf("compress all-ones length: got %d, want 32", len(result))
	}
	for i, b := range result {
		if b != 0xFF {
			t.Errorf("byte %d: got 0x%02X, want 0xFF", i, b)
		}
	}
}

// ---- AlgMinhash256 tests ----

func TestMinhash256Empty(t *testing.T) {
	result := AlgMinhash256([]uint32{})
	if len(result) != 32 {
		t.Fatalf("length: got %d, want 32", len(result))
	}
	// Empty features → all MAXH → all bits set in LSB 4 → all 0xFF
	for i, b := range result {
		if b != 0xFF {
			t.Errorf("byte %d: got 0x%02X, want 0xFF", i, b)
		}
	}
}

func TestMinhash256Single(t *testing.T) {
	result := AlgMinhash256([]uint32{1})
	if len(result) != 32 {
		t.Fatalf("length: got %d, want 32", len(result))
	}
}

func TestMinhash256Deterministic(t *testing.T) {
	features := []uint32{100, 200, 300, 400, 500}
	r1 := AlgMinhash256(features)
	r2 := AlgMinhash256(features)
	if len(r1) != len(r2) {
		t.Fatalf("length mismatch: %d vs %d", len(r1), len(r2))
	}
	for i := range r1 {
		if r1[i] != r2[i] {
			t.Errorf("byte %d: 0x%02X != 0x%02X", i, r1[i], r2[i])
		}
	}
}
