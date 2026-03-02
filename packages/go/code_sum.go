// Pure Go implementation of ISCC-SUM generation (gen_sum_code_v0).
// Single-pass file I/O feeds both DataHasher (CDC/MinHash) and InstanceHasher
// (BLAKE3), then composes an ISCC-CODE. Matches the Rust gen_sum_code_v0.
package iscc

import (
	"fmt"
	"io"
	"os"
)

// SumCodeResult holds the output of GenSumCodeV0.
type SumCodeResult struct {
	Iscc     string // Composite ISCC-CODE string with "ISCC:" prefix
	Datahash string // Hex-encoded BLAKE3 multihash (prefix "1e20" + 32-byte digest)
	Filesize uint64 // Size of input file in bytes
}

// GenSumCodeV0 generates a composite ISCC-CODE from a file path using a single
// read pass. Internally creates a DataHasher and InstanceHasher, feeds both from
// the same buffer, finalizes each, and assembles the ISCC-CODE via GenIsccCodeV0.
func GenSumCodeV0(path string, bits uint32, wide bool) (*SumCodeResult, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("iscc: open file: %w", err)
	}
	defer f.Close()

	dh := NewDataHasher()
	ih := NewInstanceHasher()

	buf := make([]byte, IoReadSize)
	for {
		n, err := f.Read(buf)
		if n > 0 {
			dh.Push(buf[:n])
			ih.Push(buf[:n])
		}
		if err == io.EOF {
			break
		}
		if err != nil {
			return nil, fmt.Errorf("iscc: read file: %w", err)
		}
	}

	dataResult, err := dh.Finalize(bits)
	if err != nil {
		return nil, fmt.Errorf("iscc: finalize data code: %w", err)
	}

	instanceResult, err := ih.Finalize(bits)
	if err != nil {
		return nil, fmt.Errorf("iscc: finalize instance code: %w", err)
	}

	isccResult, err := GenIsccCodeV0(
		[]string{dataResult.Iscc, instanceResult.Iscc}, wide,
	)
	if err != nil {
		return nil, fmt.Errorf("iscc: compose iscc code: %w", err)
	}

	return &SumCodeResult{
		Iscc:     isccResult.Iscc,
		Datahash: instanceResult.Datahash,
		Filesize: instanceResult.Filesize,
	}, nil
}
