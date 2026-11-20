// Pure Go implementation of SimHash and sliding window utilities for ISCC.
// SimHash computes a similarity-preserving hash from equal-length digests.
// SlidingWindow generates overlapping n-gram substrings from Unicode text.
package iscc

import "fmt"

// AlgSimhash computes a SimHash from a sequence of equal-length hash digests.
// Returns an error for an empty list or mismatched digest lengths, then
// delegates to algSimhash. Uses MSB-first bit ordering and majority-vote
// aggregation.
func AlgSimhash(hashDigests [][]byte) ([]byte, error) {
	if len(hashDigests) == 0 {
		return nil, fmt.Errorf("hash_digests must not be empty")
	}
	expectedLen := len(hashDigests[0])
	for i := 1; i < len(hashDigests); i++ {
		if len(hashDigests[i]) != expectedLen {
			return nil, fmt.Errorf(
				"all hash digests must have equal length (expected %d, got %d at index %d)",
				expectedLen, len(hashDigests[i]), i,
			)
		}
	}
	return algSimhash(hashDigests), nil
}

// algSimhash computes a SimHash without validation. Callers must guarantee
// all digests have equal length. Returns 32 zero bytes for empty input,
// which the generators rely on.
func algSimhash(hashDigests [][]byte) []byte {
	if len(hashDigests) == 0 {
		return make([]byte, 32)
	}

	nBytes := len(hashDigests[0])
	nBits := nBytes * 8
	vector := make([]uint32, nBits)

	// Count bits across all digests
	for _, digest := range hashDigests {
		for i := 0; i < nBits; i++ {
			byteIdx := i / 8
			bitIdx := uint(7 - (i % 8)) // MSB first
			if (digest[byteIdx]>>bitIdx)&1 == 1 {
				vector[i]++
			}
		}
	}

	// Threshold: count * 2 >= n (matches Python's count >= n / 2.0)
	n := uint32(len(hashDigests))
	result := make([]byte, nBytes)
	for i, count := range vector {
		if count*2 >= n {
			byteIdx := i / 8
			bitIdx := uint(7 - (i % 8))
			result[byteIdx] |= 1 << bitIdx
		}
	}

	return result
}

// SlidingWindow generates overlapping substrings of width runes from a string.
// Returns an error if width < 2. If input is shorter than width, returns a
// single element with the full input. Uses []rune conversion for proper
// Unicode character counting.
func SlidingWindow(seq string, width int) ([]string, error) {
	if width < 2 {
		return nil, fmt.Errorf("sliding window width must be 2 or bigger")
	}
	runes := []rune(seq)
	runeLen := len(runes)

	// Range: max(len - width + 1, 1) windows
	rangeVal := runeLen - width + 1
	if rangeVal < 1 {
		rangeVal = 1
	}

	result := make([]string, rangeVal)
	for i := 0; i < rangeVal; i++ {
		end := i + width
		if end > runeLen {
			end = runeLen
		}
		result[i] = string(runes[i:end])
	}

	return result, nil
}
