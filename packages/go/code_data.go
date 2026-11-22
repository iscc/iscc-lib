// Pure Go implementation of ISCC Data-Code generation.
// Produces a similarity-preserving fingerprint from binary data using
// Content-Defined Chunking (CDC) and MinHash. Matches the Rust gen_data_code_v0.
package iscc

import "fmt"

// DataCodeResult holds the output of GenDataCodeV0.
type DataCodeResult struct {
	Iscc string // ISCC code string with "ISCC:" prefix
}

// DataHasher incrementally hashes binary data to produce an ISCC Data-Code.
// Uses CDC to split data into content-defined chunks, xxh32-hashes each chunk,
// and applies MinHash to produce a similarity-preserving digest.
//
// A DataHasher is single-use: the first Finalize call consumes it, whether
// it succeeds or fails, and every later Push or Finalize returns an
// "already finalized" error.
type DataHasher struct {
	chunkFeatures []uint32
	tail          []byte
	finalized     bool
}

// NewDataHasher creates a new DataHasher.
func NewDataHasher() *DataHasher {
	return &DataHasher{}
}

// Push appends data to the hasher, running CDC on accumulated bytes.
// All complete chunks are xxh32-hashed; the last chunk is retained as tail
// for the next Push call (mirrors the Python prev_chunk pattern).
// Returns an error without touching any state once Finalize has been called.
func (h *DataHasher) Push(data []byte) error {
	if h.finalized {
		return fmt.Errorf("iscc: DataHasher already finalized")
	}

	// Prepend any retained tail from previous Push
	if len(h.tail) > 0 {
		combined := make([]byte, len(h.tail)+len(data))
		copy(combined, h.tail)
		copy(combined[len(h.tail):], data)
		data = combined
	}

	chunks := algCdcChunksUnchecked(data, false, 1024)

	// Process all chunks except the last (which becomes the new tail).
	// This mirrors the Python push() method's prev_chunk pattern.
	var prevChunk []byte
	for _, chunk := range chunks {
		if prevChunk != nil {
			h.chunkFeatures = append(h.chunkFeatures, xxh32(prevChunk, 0))
		}
		prevChunk = chunk
	}

	// Retain the last chunk as tail for the next Push
	if prevChunk != nil {
		h.tail = make([]byte, len(prevChunk))
		copy(h.tail, prevChunk)
	} else {
		h.tail = []byte{}
	}
	return nil
}

// Finalize flushes any remaining tail data, computes the MinHash digest,
// and encodes the result as an ISCC Data-Code component.
// The first call consumes the hasher even when it fails (for example on an
// invalid bits value); every later call returns an "already finalized" error.
func (h *DataHasher) Finalize(bits uint32) (*DataCodeResult, error) {
	if h.finalized {
		return nil, fmt.Errorf("iscc: DataHasher already finalized")
	}
	h.finalized = true

	if len(h.tail) > 0 {
		// Hash non-empty tail
		h.chunkFeatures = append(h.chunkFeatures, xxh32(h.tail, 0))
	} else if len(h.chunkFeatures) == 0 {
		// Empty input: ensure at least one feature
		h.chunkFeatures = append(h.chunkFeatures, xxh32([]byte{}, 0))
	}

	digest := algMinhash256(h.chunkFeatures)
	component, err := encodeComponentInternal(
		MTData, STNone, VSV0, bits, digest,
	)
	if err != nil {
		return nil, fmt.Errorf("iscc: encode data code: %w", err)
	}

	return &DataCodeResult{
		Iscc: "ISCC:" + component,
	}, nil
}

// GenDataCodeV0 generates an ISCC Data-Code from binary data.
// Splits data into content-defined chunks, hashes each chunk with xxh32,
// and applies MinHash to produce a similarity-preserving fingerprint.
func GenDataCodeV0(data []byte, bits uint32) (*DataCodeResult, error) {
	h := NewDataHasher()
	if err := h.Push(data); err != nil {
		return nil, fmt.Errorf("iscc: push data code: %w", err)
	}
	return h.Finalize(bits)
}
