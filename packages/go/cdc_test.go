// Tests for the pure Go CDC (Content-Defined Chunking) module.
// These tests do NOT require the WASM binary — they test pure Go functions.
package iscc

import "testing"

// ---- Gear table tests ----

func TestCdcGearTableLength(t *testing.T) {
	if len(cdcGear) != 256 {
		t.Errorf("gear table length: got %d, want 256", len(cdcGear))
	}
}

func TestCdcGearTableFirstLast(t *testing.T) {
	if cdcGear[0] != 1553318008 {
		t.Errorf("gear[0]: got %d, want 1553318008", cdcGear[0])
	}
	if cdcGear[255] != 854125182 {
		t.Errorf("gear[255]: got %d, want 854125182", cdcGear[255])
	}
}

// ---- algCdcParams tests ----

func TestCdcParamsDefault(t *testing.T) {
	mi, ma, cs, maskS, maskL := algCdcParams(1024)
	if mi != 256 {
		t.Errorf("min_size: got %d, want 256", mi)
	}
	if ma != 8192 {
		t.Errorf("max_size: got %d, want 8192", ma)
	}
	if cs != 640 {
		t.Errorf("center_size: got %d, want 640", cs)
	}
	if maskS != 2047 {
		t.Errorf("mask_s: got %d, want 2047 ((1 << 11) - 1)", maskS)
	}
	if maskL != 511 {
		t.Errorf("mask_l: got %d, want 511 ((1 << 9) - 1)", maskL)
	}
}

// ---- algCdcOffset tests ----

func TestCdcOffsetSmallBuffer(t *testing.T) {
	// Buffer smaller than min_size → returns buffer length
	buf := make([]byte, 100)
	mi, ma, cs, maskS, maskL := algCdcParams(1024)
	offset := algCdcOffset(buf, mi, ma, cs, maskS, maskL)
	if offset != 100 {
		t.Errorf("small buffer offset: got %d, want 100", offset)
	}
}

func TestCdcOffsetReturnsAtMostMax(t *testing.T) {
	// Buffer larger than max_size → returns at most max_size
	buf := make([]byte, 10000)
	for i := range buf {
		buf[i] = 0xAA
	}
	mi, ma, cs, maskS, maskL := algCdcParams(1024)
	offset := algCdcOffset(buf, mi, ma, cs, maskS, maskL)
	if offset > ma {
		t.Errorf("offset %d exceeds max_size %d", offset, ma)
	}
	if offset < mi {
		t.Errorf("offset %d below min_size %d", offset, mi)
	}
}

// ---- AlgCdcChunks tests ----

func TestCdcChunksEmpty(t *testing.T) {
	chunks := AlgCdcChunks([]byte{}, false, 1024)
	if len(chunks) != 1 {
		t.Fatalf("empty input: got %d chunks, want 1", len(chunks))
	}
	if len(chunks[0]) != 0 {
		t.Errorf("empty input chunk length: got %d, want 0", len(chunks[0]))
	}
}

func TestCdcChunksSmallData(t *testing.T) {
	// Data smaller than min_size → one chunk containing all data
	data := make([]byte, 100)
	for i := range data {
		data[i] = 42
	}
	chunks := AlgCdcChunks(data, false, 1024)
	if len(chunks) != 1 {
		t.Fatalf("small data: got %d chunks, want 1", len(chunks))
	}
	if len(chunks[0]) != 100 {
		t.Errorf("small data chunk length: got %d, want 100", len(chunks[0]))
	}
}

func TestCdcChunksReassembly(t *testing.T) {
	// Chunks must reassemble to original data
	data := make([]byte, 4096)
	for i := range data {
		data[i] = byte(i % 256)
	}
	chunks := AlgCdcChunks(data, false, 1024)
	var reassembled []byte
	for _, c := range chunks {
		reassembled = append(reassembled, c...)
	}
	if len(reassembled) != len(data) {
		t.Fatalf("reassembly length: got %d, want %d", len(reassembled), len(data))
	}
	for i := range data {
		if reassembled[i] != data[i] {
			t.Fatalf("reassembly mismatch at byte %d", i)
		}
	}
}

func TestCdcChunksDeterministic(t *testing.T) {
	data := make([]byte, 4096)
	for i := range data {
		data[i] = byte(i % 256)
	}
	chunks1 := AlgCdcChunks(data, false, 1024)
	chunks2 := AlgCdcChunks(data, false, 1024)
	if len(chunks1) != len(chunks2) {
		t.Fatalf("determinism: got %d and %d chunks", len(chunks1), len(chunks2))
	}
	for i := range chunks1 {
		if len(chunks1[i]) != len(chunks2[i]) {
			t.Errorf("chunk %d length mismatch: %d vs %d", i, len(chunks1[i]), len(chunks2[i]))
		}
	}
}

func TestCdcChunksMultipleChunks(t *testing.T) {
	// Large data produces multiple chunks
	data := make([]byte, 8192)
	for i := range data {
		data[i] = byte(i % 256)
	}
	chunks := AlgCdcChunks(data, false, 1024)
	if len(chunks) <= 1 {
		t.Errorf("expected multiple chunks, got %d", len(chunks))
	}
}

// ---- UTF-32 alignment tests ----

func TestCdcChunksUtf32SmallBuffer(t *testing.T) {
	// 3 bytes with utf32=true must terminate and reassemble to original.
	// Regression test for the infinite loop bug.
	data := []byte{0xAA, 0xBB, 0xCC}
	chunks := AlgCdcChunks(data, true, 1024)
	if len(chunks) == 0 {
		t.Fatal("must return at least one chunk")
	}
	var reassembled []byte
	for _, c := range chunks {
		reassembled = append(reassembled, c...)
	}
	if len(reassembled) != len(data) {
		t.Fatalf("reassembly length: got %d, want %d", len(reassembled), len(data))
	}
	for i := range data {
		if reassembled[i] != data[i] {
			t.Fatalf("reassembly mismatch at byte %d", i)
		}
	}
}

func TestCdcChunksUtf32Exact4Bytes(t *testing.T) {
	// Exactly 4 bytes with utf32=true must return one 4-byte chunk.
	data := []byte{0x01, 0x02, 0x03, 0x04}
	chunks := AlgCdcChunks(data, true, 1024)
	if len(chunks) != 1 {
		t.Fatalf("got %d chunks, want 1", len(chunks))
	}
	if len(chunks[0]) != 4 {
		t.Errorf("chunk length: got %d, want 4", len(chunks[0]))
	}
}

func TestCdcChunksUtf327Bytes(t *testing.T) {
	// 7 bytes (4+3) with utf32=true verifies non-aligned tail handling.
	data := []byte{0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70}
	chunks := AlgCdcChunks(data, true, 1024)
	if len(chunks) == 0 {
		t.Fatal("must return at least one chunk")
	}
	var reassembled []byte
	for _, c := range chunks {
		reassembled = append(reassembled, c...)
	}
	if len(reassembled) != len(data) {
		t.Fatalf("reassembly length: got %d, want %d", len(reassembled), len(data))
	}
	for i := range data {
		if reassembled[i] != data[i] {
			t.Fatalf("reassembly mismatch at byte %d", i)
		}
	}
}

func TestCdcChunksUtf32Reassembly(t *testing.T) {
	// Larger 4-byte-aligned input with utf32=true must reassemble correctly,
	// and all chunks except possibly the last must be 4-byte aligned.
	data := make([]byte, 4096)
	for i := range data {
		data[i] = byte(i % 256)
	}
	if len(data)%4 != 0 {
		t.Fatal("test data must be 4-byte aligned")
	}
	chunks := AlgCdcChunks(data, true, 1024)
	var reassembled []byte
	for _, c := range chunks {
		reassembled = append(reassembled, c...)
	}
	if len(reassembled) != len(data) {
		t.Fatalf("reassembly length: got %d, want %d", len(reassembled), len(data))
	}
	for i := range data {
		if reassembled[i] != data[i] {
			t.Fatalf("reassembly mismatch at byte %d", i)
		}
	}
	// All chunks except the last must be 4-byte aligned
	if len(chunks) > 1 {
		for i, chunk := range chunks[:len(chunks)-1] {
			if len(chunk)%4 != 0 {
				t.Errorf("chunk %d has length %d which is not 4-byte aligned", i, len(chunk))
			}
		}
	}
}

func TestCdcChunksUtf32Empty(t *testing.T) {
	// Empty input with utf32=true must not loop and must return one empty chunk.
	chunks := AlgCdcChunks([]byte{}, true, 1024)
	if len(chunks) != 1 {
		t.Fatalf("got %d chunks, want 1", len(chunks))
	}
	if len(chunks[0]) != 0 {
		t.Errorf("chunk length: got %d, want 0", len(chunks[0]))
	}
}
