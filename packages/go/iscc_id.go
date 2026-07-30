// Pure Go implementation of experimental ISCC-IDv1 minting.
// An ISCC-IDv1 is an 80-bit identifier (16-bit header + 64-bit body) whose body
// packs a 52-bit microsecond timestamp and a 12-bit HUB-ID, matching the Python
// reference in iscc-core's iscc_id.py (gen_iscc_id_v1).
package iscc

import (
	"encoding/binary"
	"fmt"
)

// IsccIdResult holds the minted ISCC-IDv1 string, mirroring iscc-core's
// {"iscc": ...} return shape.
//
// Experimental: ISCC-IDv1 is not part of ISO 24138 and may change in a minor release.
type IsccIdResult struct {
	ISCC string `json:"iscc"`
}

// GenIsccIDV1 mints an ISCC-IDv1 from timestamp, hub ID, and realm.
//
// The 64-bit body packs the 52-bit microsecond timestamp in the high bits and
// the 12-bit HUB-ID in the low 12 bits. The returned ISCC carries the "ISCC:"
// prefix. Valid inputs: timestamp < 2^52, hubID 0-4095, realm 0 (test) or 1
// (operational). The parameter order mirrors iscc-core's gen_iscc_id_v1.
//
// To recover the fields from an ISCC-IDv1, decode it with IsccDecode and unpack
// the 8-byte digest: n := binary.BigEndian.Uint64(d.Digest); timestamp := n >> 12;
// hubID := uint16(n & 0xFFF); realm := d.Subtype.
//
// Experimental: ISCC-IDv1 is not part of ISO 24138 and may change in a minor release.
func GenIsccIDV1(timestamp uint64, hubID uint16, realm uint8) (*IsccIdResult, error) {
	if timestamp >= 1<<52 {
		return nil, fmt.Errorf("iscc: timestamp overflow: %d (must be < 2^52)", timestamp)
	}
	if hubID >= 1<<12 {
		return nil, fmt.Errorf("iscc: HUB-ID overflow: %d (must be 0-4095)", hubID)
	}
	if realm > 1 {
		return nil, fmt.Errorf("iscc: Realm-ID must be 0 (test) or 1 (operational), got %d", realm)
	}

	body := (timestamp << 12) | uint64(hubID)
	var digest [8]byte
	binary.BigEndian.PutUint64(digest[:], body)

	encodedLen, err := encodeLength(MTId, 64)
	if err != nil {
		return nil, err
	}
	header, err := encodeHeader(MTId, SubType(realm), VSV1, encodedLen)
	if err != nil {
		return nil, err
	}

	component := make([]byte, 0, len(header)+len(digest))
	component = append(component, header...)
	component = append(component, digest[:]...)

	return &IsccIdResult{ISCC: "ISCC:" + encodeBase32(component)}, nil
}
