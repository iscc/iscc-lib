// Pure Go implementation of ISCC Video-Code (Content-Code for video).
// Generates a WTA-Hash digest from deduplicated MPEG-7 frame signatures.
// Matches the Rust gen_video_code_v0 implementation.
package iscc

import "fmt"

// VideoCodeResult holds the output of GenVideoCodeV0.
type VideoCodeResult struct {
	Iscc string // ISCC code string with "ISCC:" prefix
}

// GenVideoCodeV0 generates an ISCC Content-Code for video content.
// Takes a sequence of MPEG-7 frame signatures (each 380 int32 values) and a
// bit length, computes a WTA-Hash digest from deduplicated column-wise sums,
// and encodes as an ISCC component.
func GenVideoCodeV0(frameSigs [][]int32, bits uint32) (*VideoCodeResult, error) {
	digest, err := SoftHashVideoV0(frameSigs, bits)
	if err != nil {
		return nil, err
	}
	component, err := EncodeComponent(
		uint8(MTContent), uint8(STVideo), uint8(VSV0), bits, digest,
	)
	if err != nil {
		return nil, err
	}
	return &VideoCodeResult{
		Iscc: "ISCC:" + component,
	}, nil
}

// SoftHashVideoV0 computes a WTA-Hash digest from video frame signatures.
// Deduplicates frame signatures, computes column-wise sums across all unique
// frames into int64 to avoid overflow, then applies WTA-Hash to produce a
// digest of bits/8 bytes.
func SoftHashVideoV0(frameSigs [][]int32, bits uint32) ([]byte, error) {
	if len(frameSigs) == 0 {
		return nil, fmt.Errorf("frame_sigs must not be empty")
	}

	// Deduplicate using a string-keyed map
	unique := make(map[string][]int32)
	for _, sig := range frameSigs {
		key := fmt.Sprintf("%v", sig)
		if _, exists := unique[key]; !exists {
			unique[key] = sig
		}
	}

	// Column-wise sum into int64 to avoid overflow
	cols := len(frameSigs[0])
	vecsum := make([]int64, cols)
	for _, sig := range unique {
		for c, val := range sig {
			vecsum[c] += int64(val)
		}
	}

	return AlgWtahash(vecsum, bits)
}
