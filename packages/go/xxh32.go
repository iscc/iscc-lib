// Pure Go implementation of xxHash32 (XXH32) for ISCC text code generation.
// Implements the xxHash specification for 32-bit hashing with a seed parameter.
// Reference: https://github.com/Cyan4973/xxHash/blob/dev/doc/xxhash_spec.md
package iscc

// xxh32 prime constants.
const (
	xxh32Prime1 uint32 = 0x9E3779B1
	xxh32Prime2 uint32 = 0x85EBCA77
	xxh32Prime3 uint32 = 0xC2B2AE3D
	xxh32Prime4 uint32 = 0x27D4EB2F
	xxh32Prime5 uint32 = 0x165667B1
)

// xxh32 computes the 32-bit xxHash of data with the given seed.
func xxh32(data []byte, seed uint32) uint32 {
	length := len(data)
	var h uint32

	if length >= 16 {
		// Four-lane accumulation
		v1 := seed + xxh32Prime1 + xxh32Prime2
		v2 := seed + xxh32Prime2
		v3 := seed
		v4 := seed - xxh32Prime1

		for len(data) >= 16 {
			v1 = xxh32Round(v1, readU32LE(data[0:4]))
			v2 = xxh32Round(v2, readU32LE(data[4:8]))
			v3 = xxh32Round(v3, readU32LE(data[8:12]))
			v4 = xxh32Round(v4, readU32LE(data[12:16]))
			data = data[16:]
		}

		h = rotl32(v1, 1) + rotl32(v2, 7) + rotl32(v3, 12) + rotl32(v4, 18)
	} else {
		h = seed + xxh32Prime5
	}

	h += uint32(length)

	// Process remaining bytes in 4-byte chunks
	for len(data) >= 4 {
		h += readU32LE(data[0:4]) * xxh32Prime3
		h = rotl32(h, 17) * xxh32Prime4
		data = data[4:]
	}

	// Process remaining individual bytes
	for _, b := range data {
		h += uint32(b) * xxh32Prime5
		h = rotl32(h, 11) * xxh32Prime1
	}

	// Final avalanche mixing
	h ^= h >> 15
	h *= xxh32Prime2
	h ^= h >> 13
	h *= xxh32Prime3
	h ^= h >> 16

	return h
}

// xxh32Round processes one 32-bit input in the accumulation phase.
func xxh32Round(acc, input uint32) uint32 {
	acc += input * xxh32Prime2
	acc = rotl32(acc, 13)
	acc *= xxh32Prime1
	return acc
}

// rotl32 performs a 32-bit left rotation.
func rotl32(val uint32, n uint) uint32 {
	return (val << n) | (val >> (32 - n))
}

// readU32LE reads a little-endian uint32 from a byte slice.
func readU32LE(b []byte) uint32 {
	return uint32(b[0]) | uint32(b[1])<<8 | uint32(b[2])<<16 | uint32(b[3])<<24
}
