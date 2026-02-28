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
type InstanceHasher struct {
	hasher   *blake3.Hasher
	filesize uint64
}

// NewInstanceHasher creates a new InstanceHasher.
func NewInstanceHasher() *InstanceHasher {
	return &InstanceHasher{
		hasher: blake3.New(),
	}
}

// Push feeds data into the BLAKE3 hasher and updates the filesize counter.
func (h *InstanceHasher) Push(data []byte) {
	h.hasher.Write(data)
	h.filesize += uint64(len(data))
}

// Finalize computes the BLAKE3 digest, builds the multihash, and encodes
// the result as an ISCC Instance-Code component.
func (h *InstanceHasher) Finalize(bits uint32) (*InstanceCodeResult, error) {
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
	h.Push(data)
	return h.Finalize(bits)
}
