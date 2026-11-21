// Pure Go implementation of ISCC Instance-Code generation.
// Produces a cryptographic hash (BLAKE3) of binary data for exact content
// identification. Matches the Rust gen_instance_code_v0.
package iscc

import (
	"encoding/hex"
	"fmt"

	"github.com/zeebo/blake3"
)

// InstanceCodeResult holds the output of GenInstanceCodeV0.
type InstanceCodeResult struct {
	Iscc     string // ISCC code string with "ISCC:" prefix
	Datahash string // Hex-encoded BLAKE3 multihash (prefix "1e20" + 32-byte digest)
	Filesize uint64 // Size of input data in bytes
}

// InstanceHasher incrementally hashes binary data with BLAKE3 to produce
// an ISCC Instance-Code identical to GenInstanceCodeV0 for the same byte stream.
//
// An InstanceHasher is single-use: the first Finalize call consumes it,
// whether it succeeds or fails, and every later Push or Finalize returns an
// "already finalized" error.
type InstanceHasher struct {
	hasher    *blake3.Hasher
	filesize  uint64
	finalized bool
}

// NewInstanceHasher creates a new InstanceHasher.
func NewInstanceHasher() *InstanceHasher {
	return &InstanceHasher{
		hasher: blake3.New(),
	}
}

// Push feeds data into the BLAKE3 hasher and updates the filesize counter.
// Returns an error without touching any state once Finalize has been called.
func (h *InstanceHasher) Push(data []byte) error {
	if h.finalized {
		return fmt.Errorf("iscc: InstanceHasher already finalized")
	}
	h.hasher.Write(data)
	h.filesize += uint64(len(data))
	return nil
}

// Finalize computes the BLAKE3 digest, builds the multihash, and encodes
// the result as an ISCC Instance-Code component.
// The first call consumes the hasher even when it fails (for example on an
// invalid bits value); every later call returns an "already finalized" error.
func (h *InstanceHasher) Finalize(bits uint32) (*InstanceCodeResult, error) {
	if h.finalized {
		return nil, fmt.Errorf("iscc: InstanceHasher already finalized")
	}
	h.finalized = true

	digest := h.hasher.Sum(nil)
	datahash := "1e20" + hex.EncodeToString(digest)

	component, err := EncodeComponent(
		uint8(MTInstance), uint8(STNone), uint8(VSV0), bits, digest,
	)
	if err != nil {
		return nil, fmt.Errorf("iscc: encode instance code: %w", err)
	}

	return &InstanceCodeResult{
		Iscc:     "ISCC:" + component,
		Datahash: datahash,
		Filesize: h.filesize,
	}, nil
}

// GenInstanceCodeV0 generates an ISCC Instance-Code from binary data.
// Computes a BLAKE3 hash of the input and encodes it as an ISCC component.
// Returns the ISCC code, BLAKE3 multihash, and file size.
func GenInstanceCodeV0(data []byte, bits uint32) (*InstanceCodeResult, error) {
	h := NewInstanceHasher()
	if err := h.Push(data); err != nil {
		return nil, fmt.Errorf("iscc: push instance code: %w", err)
	}
	return h.Finalize(bits)
}
