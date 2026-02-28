// Tests for xxh32 implementation — verifies against canonical xxHash32 test vectors.
package iscc

import "testing"

func TestXxh32EmptyZeroSeed(t *testing.T) {
	// Canonical xxHash test vector: empty input with seed 0
	got := xxh32([]byte{}, 0)
	want := uint32(0x02CC5D05)
	if got != want {
		t.Errorf("xxh32(empty, 0) = 0x%08X, want 0x%08X", got, want)
	}
}

func TestXxh32EmptyNonZeroSeed(t *testing.T) {
	// Verify non-zero seed produces different result
	got := xxh32([]byte{}, 1)
	if got == 0x02CC5D05 {
		t.Error("xxh32(empty, 1) should differ from xxh32(empty, 0)")
	}
}

func TestXxh32Deterministic(t *testing.T) {
	// Same input always produces same output
	data := []byte("test data for determinism")
	a := xxh32(data, 0)
	b := xxh32(data, 0)
	if a != b {
		t.Errorf("xxh32 not deterministic: 0x%08X != 0x%08X", a, b)
	}
}

func TestXxh32DifferentSeeds(t *testing.T) {
	// Different seeds produce different results
	data := []byte("Hello")
	a := xxh32(data, 0)
	b := xxh32(data, 42)
	if a == b {
		t.Errorf("xxh32 with seeds 0 and 42 should differ, both 0x%08X", a)
	}
}

func TestXxh32ShortInput(t *testing.T) {
	// Under 4 bytes — exercises individual byte path only
	got := xxh32([]byte{0x42}, 0)
	if got == 0 {
		t.Error("xxh32 of single byte should not be zero")
	}
}

func TestXxh32FourBytes(t *testing.T) {
	// Exactly 4 bytes — exercises 4-byte chunk path
	got := xxh32([]byte{1, 2, 3, 4}, 0)
	if got == 0 {
		t.Error("xxh32 of 4 bytes should not be zero")
	}
}

func TestXxh32SixteenBytes(t *testing.T) {
	// Exactly 16 bytes — exercises 4-lane accumulation with no remainder
	got := xxh32([]byte("0123456789abcdef"), 0)
	if got == 0 {
		t.Error("xxh32 of 16 bytes should not be zero")
	}
}

func TestXxh32LongInput(t *testing.T) {
	// 16+ bytes — exercises 4-lane accumulation with remainder
	data := []byte("Hello, World!!! Extra text here for length testing purposes")
	got := xxh32(data, 0)
	if got == 0 {
		t.Error("xxh32 of long input should not be zero")
	}
}
