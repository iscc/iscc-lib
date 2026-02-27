// Pure Go implementation of ISCC Mixed-Code (Content-Code for mixed content).
// Combines multiple Content-Code digests into a single similarity hash using SimHash.
// Matches the Rust gen_mixed_code_v0 implementation.
package iscc

import (
	"fmt"
	"strings"
)

// MixedCodeResult holds the output of GenMixedCodeV0.
type MixedCodeResult struct {
	Iscc  string   // ISCC code string with "ISCC:" prefix
	Parts []string // Input ISCC codes that were combined
}

// GenMixedCodeV0 generates a Mixed Content-Code from multiple Content-Code strings.
// Combines multiple ISCC Content-Codes of different types (text, image, audio,
// video) using SimHash. Input codes may optionally include the "ISCC:" prefix.
func GenMixedCodeV0(codes []string, bits uint32) (*MixedCodeResult, error) {
	decoded := make([][]byte, len(codes))
	for i, code := range codes {
		clean := strings.TrimPrefix(code, "ISCC:")
		raw, err := decodeBase32(clean)
		if err != nil {
			return nil, fmt.Errorf("decode code %d: %w", i, err)
		}
		decoded[i] = raw
	}

	digest, err := softHashCodesV0(decoded, bits)
	if err != nil {
		return nil, err
	}

	component, err := EncodeComponent(
		uint8(MTContent), uint8(STMixed), uint8(VSV0), bits, digest,
	)
	if err != nil {
		return nil, err
	}

	parts := make([]string, len(codes))
	copy(parts, codes)

	return &MixedCodeResult{
		Iscc:  "ISCC:" + component,
		Parts: parts,
	}, nil
}

// softHashCodesV0 combines multiple Content-Code digests into a single SimHash.
// Takes raw decoded ISCC bytes (header + body) for each Content-Code and
// produces a SimHash digest. Each input is trimmed to bits/8 bytes by keeping
// the first header byte (encodes type info) plus nbytes-1 body bytes.
// Requires at least 2 codes, all of MainType Content.
func softHashCodesV0(ccDigests [][]byte, bits uint32) ([]byte, error) {
	if len(ccDigests) < 2 {
		return nil, fmt.Errorf("at least 2 Content-Codes required for mixing")
	}

	nbytes := int(bits / 8)
	prepared := make([][]byte, 0, len(ccDigests))

	for _, raw := range ccDigests {
		mtype, stype, _, blen, body, err := decodeHeader(raw)
		if err != nil {
			return nil, err
		}
		if mtype != MTContent {
			return nil, fmt.Errorf("all codes must be Content-Codes")
		}
		unitBits := decodeLength(mtype, blen, stype)
		if unitBits < bits {
			return nil, fmt.Errorf(
				"Content-Code too short for %d-bit length (has %d bits)", bits, unitBits,
			)
		}

		entry := make([]byte, nbytes)
		entry[0] = raw[0] // first byte preserves type info
		take := min(nbytes-1, len(body))
		copy(entry[1:], body[:take])
		// Remaining bytes are already zero from make()

		prepared = append(prepared, entry)
	}

	// All entries have identical length (nbytes), so AlgSimhash validation passes
	result, _ := AlgSimhash(prepared)
	return result, nil
}
