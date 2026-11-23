// Tests for the pure Go CDC (Content-Defined Chunking) module.
package iscc

import (
	"math"
	"strconv"
	"testing"
)

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

// largeAvgParams holds the reference (min, max, center, maskS, maskL) for large
// avgChunkSize values; maskS for 4294967295 is the reference's 2^33-1 saturated.
var largeAvgParams = []struct {
	avg          uint32
	mi, ma, cs   uint64
	maskS, maskL uint32
}{
	{536870912, 134217728, 4294967296, 335544320, 1073741823, 268435455},
	{2147483647, 536870911, 17179869176, 1342177280, 4294967295, 1073741823},
	{2147483648, 536870912, 17179869184, 1342177280, 4294967295, 1073741823},
	{4294967295, 1073741823, 34359738360, 2684354560, 4294967295, 2147483647},
}

func TestCdcParamsLargeAvgMatchesReference(t *testing.T) {
	for _, tc := range largeAvgParams {
		mi, ma, cs, maskS, maskL := algCdcParams(tc.avg)
		if uint64(mi) != tc.mi {
			t.Errorf("avg %d min_size: got %d, want %d", tc.avg, mi, tc.mi)
		}
		if uint64(ma) != min(tc.ma, uint64(math.MaxInt)) {
			t.Errorf("avg %d max_size: got %d, want %d", tc.avg, ma, tc.ma)
		}
		if uint64(cs) != min(tc.cs, uint64(math.MaxInt)) {
			t.Errorf("avg %d center_size: got %d, want %d", tc.avg, cs, tc.cs)
		}
		if maskS != tc.maskS {
			t.Errorf("avg %d mask_s: got %d, want %d", tc.avg, maskS, tc.maskS)
		}
		if maskL != tc.maskL {
			t.Errorf("avg %d mask_l: got %d, want %d", tc.avg, maskL, tc.maskL)
		}
	}
}

func TestCdcParamsLargeAvgExactOn64Bit(t *testing.T) {
	if strconv.IntSize != 64 {
		t.Skip("exact max/center sizes need a 64-bit int")
	}
	for _, tc := range largeAvgParams {
		_, ma, cs, _, _ := algCdcParams(tc.avg)
		if uint64(ma) != tc.ma || uint64(cs) != tc.cs {
			t.Errorf("avg %d: got (max %d, center %d), want (%d, %d)", tc.avg, ma, cs, tc.ma, tc.cs)
		}
	}
}

func TestCdcChunksLargeAvgSingleChunk(t *testing.T) {
	data := make([]byte, 64)
	for i := range data {
		data[i] = byte(i)
	}
	for _, tc := range largeAvgParams {
		chunks, err := AlgCdcChunks(data, false, tc.avg)
		if err != nil {
			t.Fatalf("avg %d: unexpected error: %v", tc.avg, err)
		}
		if len(chunks) != 1 || !byteSliceEqual(chunks[0], data) {
			t.Errorf("avg %d: got %d chunks, want one chunk equal to the input", tc.avg, len(chunks))
		}
	}
}

func TestClampIntSaturates(t *testing.T) {
	if got := clampInt(42); got != 42 {
		t.Errorf("clampInt(42): got %d", got)
	}
	if got := clampInt(math.MaxUint64); got != math.MaxInt {
		t.Errorf("clampInt(MaxUint64): got %d, want %d", got, math.MaxInt)
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
	chunks, err := AlgCdcChunks([]byte{}, false, 1024)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
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
	chunks, err := AlgCdcChunks(data, false, 1024)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
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
	chunks, err := AlgCdcChunks(data, false, 1024)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
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

func TestCdcChunksDeterministic(t *testing.T) {
	data := make([]byte, 4096)
	for i := range data {
		data[i] = byte(i % 256)
	}
	chunks1, err := AlgCdcChunks(data, false, 1024)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	chunks2, err := AlgCdcChunks(data, false, 1024)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
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
	chunks, err := AlgCdcChunks(data, false, 1024)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(chunks) <= 1 {
		t.Errorf("expected multiple chunks, got %d", len(chunks))
	}
}

// ---- AlgCdcChunks validation tests ----

func TestCdcChunksInvalidAvgChunkSize(t *testing.T) {
	data := []byte{1, 2, 3, 4}

	// avgChunkSize=0 must return error
	_, err := AlgCdcChunks(data, false, 0)
	if err == nil {
		t.Error("avgChunkSize=0: expected error, got nil")
	}

	// avgChunkSize=1 must return error
	_, err = AlgCdcChunks(data, false, 1)
	if err == nil {
		t.Error("avgChunkSize=1: expected error, got nil")
	}

	// avgChunkSize=2 is the boundary — must succeed
	chunks, err := AlgCdcChunks(data, false, 2)
	if err != nil {
		t.Errorf("avgChunkSize=2: unexpected error: %v", err)
	}
	if len(chunks) == 0 {
		t.Error("avgChunkSize=2: expected at least one chunk")
	}
}

// ---- UTF-32 alignment tests ----

func TestCdcChunksUtf32SmallBuffer(t *testing.T) {
	// 3 bytes with utf32=true must terminate and reassemble to original.
	// Regression test for the infinite loop bug.
	data := []byte{0xAA, 0xBB, 0xCC}
	chunks, err := AlgCdcChunks(data, true, 1024)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
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
	chunks, err := AlgCdcChunks(data, true, 1024)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
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
	chunks, err := AlgCdcChunks(data, true, 1024)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
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
	chunks, err := AlgCdcChunks(data, true, 1024)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
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
	chunks, err := AlgCdcChunks([]byte{}, true, 1024)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(chunks) != 1 {
		t.Fatalf("got %d chunks, want 1", len(chunks))
	}
	if len(chunks[0]) != 0 {
		t.Errorf("chunk length: got %d, want 0", len(chunks[0]))
	}
}
