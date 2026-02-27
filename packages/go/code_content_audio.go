// Pure Go implementation of ISCC Audio-Code (Content-Code for audio).
// Generates a multi-stage SimHash digest from Chromaprint feature vectors.
// Matches the Rust gen_audio_code_v0 implementation.
package iscc

import (
	"encoding/binary"
	"sort"
)

// AudioCodeResult holds the output of GenAudioCodeV0.
type AudioCodeResult struct {
	Iscc string // ISCC code string with "ISCC:" prefix
}

// GenAudioCodeV0 generates an ISCC Content-Code for audio content.
// Takes a Chromaprint signed integer fingerprint vector and a bit length,
// computes a multi-stage SimHash digest, and encodes as an ISCC component.
func GenAudioCodeV0(cv []int32, bits uint32) (*AudioCodeResult, error) {
	hashDigest := softHashAudioV0(cv)
	component, err := EncodeComponent(
		uint8(MTContent), uint8(STAudio), uint8(VSV0), bits, hashDigest,
	)
	if err != nil {
		return nil, err
	}
	return &AudioCodeResult{
		Iscc: "ISCC:" + component,
	}, nil
}

// softHashAudioV0 computes a multi-stage SimHash digest from Chromaprint features.
// Builds a 32-byte digest by concatenating 4-byte SimHash chunks:
//   - Stage 1: overall SimHash of all features (4 bytes)
//   - Stage 2: SimHash of each quarter of features (4 × 4 = 16 bytes)
//   - Stage 3: SimHash of each third of sorted features (3 × 4 = 12 bytes)
func softHashAudioV0(cv []int32) []byte {
	// Convert each int32 to 4-byte big-endian digest
	digests := make([][]byte, len(cv))
	for i, v := range cv {
		buf := make([]byte, 4)
		binary.BigEndian.PutUint32(buf, uint32(v))
		digests[i] = buf
	}

	if len(digests) == 0 {
		return make([]byte, 32)
	}

	// Stage 1: overall SimHash (4 bytes)
	parts := make([]byte, 0, 32)
	result, _ := AlgSimhash(digests)
	parts = append(parts, result[:4]...)

	// Stage 2: quarter-based SimHashes (4 × 4 = 16 bytes)
	quarters := arraySplit(digests, 4)
	for _, quarter := range quarters {
		if len(quarter) == 0 {
			parts = append(parts, 0, 0, 0, 0)
		} else {
			sh, _ := AlgSimhash(quarter)
			parts = append(parts, sh[:4]...)
		}
	}

	// Stage 3: sorted-third-based SimHashes (3 × 4 = 12 bytes)
	sortedValues := make([]int32, len(cv))
	copy(sortedValues, cv)
	sort.Slice(sortedValues, func(i, j int) bool { return sortedValues[i] < sortedValues[j] })

	sortedDigests := make([][]byte, len(sortedValues))
	for i, v := range sortedValues {
		buf := make([]byte, 4)
		binary.BigEndian.PutUint32(buf, uint32(v))
		sortedDigests[i] = buf
	}

	thirds := arraySplit(sortedDigests, 3)
	for _, third := range thirds {
		if len(third) == 0 {
			parts = append(parts, 0, 0, 0, 0)
		} else {
			sh, _ := AlgSimhash(third)
			parts = append(parts, sh[:4]...)
		}
	}

	return parts
}

// arraySplit distributes a slice into n parts, with the first len%n parts
// getting one extra element. Returns empty sub-slices for excess parts.
// Equivalent to Python's more_itertools.divide / numpy.array_split.
func arraySplit[T any](slice []T, n int) [][]T {
	if n == 0 {
		return nil
	}
	length := len(slice)
	base := length / n
	remainder := length % n
	parts := make([][]T, n)
	offset := 0
	for i := 0; i < n; i++ {
		size := base
		if i < remainder {
			size++
		}
		parts[i] = slice[offset : offset+size]
		offset += size
	}
	return parts
}
