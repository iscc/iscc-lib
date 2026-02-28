// Pure Go implementation of ISCC-CODE assembly (gen_iscc_code_v0).
// Combines individual ISCC unit codes (Meta, Content, Data, Instance) into a
// composite ISCC-CODE. Matches the Rust gen_iscc_code_v0 implementation.
package iscc

import (
	"fmt"
	"sort"
	"strings"
)

// IsccCodeResult holds the output of GenIsccCodeV0.
type IsccCodeResult struct {
	Iscc string // Composite ISCC-CODE string with "ISCC:" prefix
}

// GenIsccCodeV0 assembles individual ISCC unit codes into a composite ISCC-CODE.
// Requires at least 2 codes with Data-Code and Instance-Code mandatory.
// Optional units are Meta-Code, Semantic-Code, and Content-Code.
// The wide parameter enables 256-bit (16 bytes per unit) mode when true;
// standard mode uses 64-bit (8 bytes per unit).
// Input codes may optionally include the "ISCC:" prefix.
func GenIsccCodeV0(codes []string, wide bool) (*IsccCodeResult, error) {
	// Step 1: Clean inputs — strip "ISCC:" prefix
	cleaned := make([]string, len(codes))
	for i, c := range codes {
		cleaned[i] = strings.TrimPrefix(c, "ISCC:")
	}

	// Step 2: Validate minimum count
	if len(cleaned) < 2 {
		return nil, fmt.Errorf("at least 2 ISCC unit codes required")
	}

	// Step 3: Validate minimum length (16 base32 chars = 64-bit minimum)
	for _, code := range cleaned {
		if len(code) < 16 {
			return nil, fmt.Errorf("ISCC unit code too short (min 16 chars): %s", code)
		}
	}

	// Step 4: Decode each code
	type decodedEntry struct {
		mt   MainType
		st   SubType
		vs   Version
		len_ uint32
		body []byte
	}
	decoded := make([]decodedEntry, len(cleaned))
	for i, code := range cleaned {
		raw, err := decodeBase32(code)
		if err != nil {
			return nil, fmt.Errorf("decode code %d: %w", i, err)
		}
		mt, st, vs, ln, body, err := decodeHeader(raw)
		if err != nil {
			return nil, fmt.Errorf("decode header %d: %w", i, err)
		}
		decoded[i] = decodedEntry{mt, st, vs, ln, body}
	}

	// Step 5: Sort by MainType ascending
	sort.Slice(decoded, func(i, j int) bool {
		return decoded[i].mt < decoded[j].mt
	})

	// Step 6: Extract main types
	n := len(decoded)
	mainTypes := make([]MainType, n)
	for i, d := range decoded {
		mainTypes[i] = d.mt
	}

	// Step 7: Validate last two are Data + Instance (mandatory)
	if mainTypes[n-2] != MTData || mainTypes[n-1] != MTInstance {
		return nil, fmt.Errorf("Data-Code and Instance-Code are mandatory")
	}

	// Step 8: Determine wide composite
	isWide := wide &&
		n == 2 &&
		mainTypes[0] == MTData && mainTypes[1] == MTInstance &&
		decodeLength(decoded[0].mt, decoded[0].len_, decoded[0].st) >= 128 &&
		decodeLength(decoded[1].mt, decoded[1].len_, decoded[1].st) >= 128

	// Step 9: Determine SubType
	var st SubType
	if isWide {
		st = STWide
	} else {
		// Collect SubTypes of Semantic/Content units
		var scSubtypes []SubType
		for _, d := range decoded {
			if d.mt == MTSemantic || d.mt == MTContent {
				scSubtypes = append(scSubtypes, d.st)
			}
		}

		if len(scSubtypes) > 0 {
			// All must be the same
			first := scSubtypes[0]
			for _, s := range scSubtypes[1:] {
				if s != first {
					return nil, fmt.Errorf("mixed SubTypes among Content/Semantic units")
				}
			}
			st = first
		} else if n == 2 {
			st = STSum
		} else {
			st = STIsccNone
		}
	}

	// Step 10: Get optional MainTypes and encode
	optionalTypes := mainTypes[:n-2]
	encodedLength, err := encodeUnits(optionalTypes)
	if err != nil {
		return nil, err
	}

	// Step 11: Build digest body
	bytesPerUnit := 8
	if isWide {
		bytesPerUnit = 16
	}
	digest := make([]byte, 0, n*bytesPerUnit)
	for _, d := range decoded {
		take := min(bytesPerUnit, len(d.body))
		digest = append(digest, d.body[:take]...)
	}

	// Step 12: Encode header + digest as base32
	header, err := encodeHeader(MTIscc, st, VSV0, encodedLength)
	if err != nil {
		return nil, err
	}
	codeBytes := make([]byte, 0, len(header)+len(digest))
	codeBytes = append(codeBytes, header...)
	codeBytes = append(codeBytes, digest...)
	code := encodeBase32(codeBytes)

	// Step 13: Return with prefix
	return &IsccCodeResult{
		Iscc: "ISCC:" + code,
	}, nil
}
