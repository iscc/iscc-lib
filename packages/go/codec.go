// Pure Go implementation of the ISCC codec: type enums, varnibble header
// encoding/decoding, base32/base64, component encoding, ISCC decompose/decode and
// the ISCC-SEQ codec (EncodeSeq/DecodeSeq).
// This module has zero external dependencies — it uses only the Go standard library.
package iscc

import (
	"bytes"
	"encoding/base32"
	"encoding/base64"
	"fmt"
	"strings"
	"unicode"
)

// ---- Algorithm Configuration Constants ----

// Algorithm configuration constants matching iscc-core core_opts.
const (
	MetaTrimName        = 128
	MetaTrimDescription = 4096
	MetaTrimMeta        = 128_000
	IoReadSize          = 4_194_304
	TextNgramSize       = 13
)

// ---- Type Enums (Go typed constants) ----

// MainType identifies the primary ISCC unit type.
type MainType uint8

const (
	MTMeta     MainType = iota // 0
	MTSemantic                 // 1
	MTContent                  // 2
	MTData                     // 3
	MTInstance                 // 4
	MTIscc                     // 5
	MTId                       // 6
	MTFlake                    // 7
)

// SubType identifies the ISCC sub-type (interpretation depends on MainType context).
type SubType uint8

const (
	STNone     SubType = iota // 0 (also TEXT in Content-Code context)
	STImage                   // 1
	STAudio                   // 2
	STVideo                   // 3
	STMixed                   // 4
	STSum                     // 5
	STIsccNone                // 6
	STWide                    // 7
)

// STText is an alias for STNone (value 0) in Content-Code / Semantic-Code context.
const STText = STNone

// Version identifies the ISCC algorithm version.
type Version uint8

// VSV0 is ISCC version 0.
const VSV0 Version = 0

// VSV1 is ISCC version 1. Currently only valid for MainType ID (ISCC-IDv1).
const VSV1 Version = 1

// ---- Bit Manipulation Helpers (unexported) ----

// getBit reads the bit at position bitPos from data (MSB-first ordering).
func getBit(data []byte, bitPos int) bool {
	byteIdx := bitPos / 8
	bitIdx := 7 - (bitPos % 8)
	return (data[byteIdx]>>uint(bitIdx))&1 == 1
}

// extractBits extracts count bits starting at bitPos as a uint32 (MSB-first).
func extractBits(data []byte, bitPos, count int) uint32 {
	var value uint32
	for i := 0; i < count; i++ {
		value = (value << 1)
		if getBit(data, bitPos+i) {
			value |= 1
		}
	}
	return value
}

// bitsToBytes converts a bit slice to bytes, padding with zero bits on the right.
func bitsToBytes(bits []bool) []byte {
	nBytes := (len(bits) + 7) / 8
	result := make([]byte, nBytes)
	for i, b := range bits {
		if b {
			byteIdx := i / 8
			bitIdx := 7 - (i % 8)
			result[byteIdx] |= 1 << uint(bitIdx)
		}
	}
	return result
}

// ---- Varnibble Encoding (unexported) ----

// encodeVarnibble encodes an integer as a variable-length nibble bit sequence.
//
// Encoding scheme:
//   - 0xxx (4 bits): values 0–7
//   - 10xxxxxx (8 bits): values 8–71
//   - 110xxxxxxxxx (12 bits): values 72–583
//   - 1110xxxxxxxxxxxx (16 bits): values 584–4679
func encodeVarnibble(value uint32) ([]bool, error) {
	switch {
	case value <= 7:
		bits := make([]bool, 4)
		for i := 0; i < 4; i++ {
			bits[i] = (value>>(3-uint(i)))&1 == 1
		}
		return bits, nil
	case value <= 71:
		v := value - 8
		bits := make([]bool, 8)
		bits[0] = true  // prefix: 10
		bits[1] = false //
		for i := 0; i < 6; i++ {
			bits[2+i] = (v>>(5-uint(i)))&1 == 1
		}
		return bits, nil
	case value <= 583:
		v := value - 72
		bits := make([]bool, 12)
		bits[0] = true  // prefix: 110
		bits[1] = true  //
		bits[2] = false //
		for i := 0; i < 9; i++ {
			bits[3+i] = (v>>(8-uint(i)))&1 == 1
		}
		return bits, nil
	case value <= 4679:
		v := value - 584
		bits := make([]bool, 16)
		bits[0] = true  // prefix: 1110
		bits[1] = true  //
		bits[2] = true  //
		bits[3] = false //
		for i := 0; i < 12; i++ {
			bits[4+i] = (v>>(11-uint(i)))&1 == 1
		}
		return bits, nil
	default:
		return nil, fmt.Errorf("iscc: varnibble value out of range (0-4679): %d", value)
	}
}

// decodeVarnibbleFromBytes decodes the first varnibble from data at the given bit position.
// Returns the decoded value and the number of bits consumed.
func decodeVarnibbleFromBytes(data []byte, bitPos int) (uint32, int, error) {
	available := len(data)*8 - bitPos
	if available < 4 {
		return 0, 0, fmt.Errorf("iscc: insufficient bits for varnibble")
	}

	if !getBit(data, bitPos) {
		// 0xxx — 4 bits, values 0–7
		return extractBits(data, bitPos, 4), 4, nil
	}
	if available >= 8 && !getBit(data, bitPos+1) {
		// 10xxxxxx — 8 bits, values 8–71
		return extractBits(data, bitPos+2, 6) + 8, 8, nil
	}
	if available >= 12 && !getBit(data, bitPos+2) {
		// 110xxxxxxxxx — 12 bits, values 72–583
		return extractBits(data, bitPos+3, 9) + 72, 12, nil
	}
	if available >= 16 && !getBit(data, bitPos+3) {
		// 1110xxxxxxxxxxxx — 16 bits, values 584–4679
		return extractBits(data, bitPos+4, 12) + 584, 16, nil
	}
	return 0, 0, fmt.Errorf("iscc: invalid varnibble prefix or insufficient bits")
}

// ---- Header Encoding (unexported) ----

// encodeHeader encodes ISCC header fields into bytes.
// Concatenates varnibble-encoded MainType, SubType, Version, and length,
// then pads to byte boundary with zero bits on the right.
func encodeHeader(mtype MainType, stype SubType, version Version, length uint32) ([]byte, error) {
	return encodeHeaderRaw(uint32(mtype), uint32(stype), uint32(version), length)
}

// encodeHeaderRaw encodes raw header field values without type gates.
// Each field must fit a varnibble (0–4679). Used by the typed encodeHeader and
// by the ISCC-SEQ codec, which must re-encode unknown SubType/Version values.
func encodeHeaderRaw(mtype, stype, version, length uint32) ([]byte, error) {
	var bits []bool
	for _, field := range [4]uint32{mtype, stype, version, length} {
		fieldBits, err := encodeVarnibble(field)
		if err != nil {
			return nil, err
		}
		bits = append(bits, fieldBits...)
	}

	// Pad to byte boundary with zero bits
	remainder := len(bits) % 8
	if remainder != 0 {
		padding := make([]bool, 8-remainder)
		bits = append(bits, padding...)
	}

	return bitsToBytes(bits), nil
}

// decodeHeaderRaw decodes the four raw header fields without type gates.
// Returns (MainType, SubType, Version, length, tailStart) where tailStart is
// the byte offset where the body begins. A zero 4-bit padding nibble is
// skipped; a non-zero one is left in place (callers needing canonical headers
// must re-encode and compare).
func decodeHeaderRaw(data []byte) (uint32, uint32, uint32, uint32, int, error) {
	names := [4]string{"maintype", "subtype", "version", "length"}
	var fields [4]uint32
	bitPos := 0
	for i, name := range names {
		value, consumed, err := decodeVarnibbleFromBytes(data, bitPos)
		if err != nil {
			return 0, 0, 0, 0, 0, fmt.Errorf("iscc: decode header %s: %w", name, err)
		}
		fields[i] = value
		bitPos += consumed
	}

	// Strip 4-bit zero padding if header bits are not byte-aligned
	if bitPos%8 != 0 && bitPos+4 <= len(data)*8 && extractBits(data, bitPos, 4) == 0 {
		bitPos += 4
	}

	return fields[0], fields[1], fields[2], fields[3], (bitPos + 7) / 8, nil
}

// decodeHeader decodes ISCC header from bytes.
// Returns (MainType, SubType, Version, length, tail) where tail contains
// remaining data after the header.
func decodeHeader(data []byte) (MainType, SubType, Version, uint32, []byte, error) {
	mtypeVal, stypeVal, versionVal, length, tailByteStart, err := decodeHeaderRaw(data)
	if err != nil {
		return 0, 0, 0, 0, nil, err
	}

	// Copy the bytes after the header boundary as the tail
	var tail []byte
	if tailByteStart < len(data) {
		tail = make([]byte, len(data)-tailByteStart)
		copy(tail, data[tailByteStart:])
	}

	if mtypeVal > 7 {
		return 0, 0, 0, 0, nil, fmt.Errorf("iscc: invalid MainType: %d", mtypeVal)
	}
	if stypeVal > 7 {
		return 0, 0, 0, 0, nil, fmt.Errorf("iscc: invalid SubType: %d", stypeVal)
	}
	// Version 1 is only valid for ISCC-IDv1 (MainType ID); all other MainTypes require Version 0.
	if versionVal > 0 && (MainType(mtypeVal) != MTId || versionVal != uint32(VSV1)) {
		return 0, 0, 0, 0, nil, fmt.Errorf("iscc: invalid Version: %d", versionVal)
	}

	return MainType(mtypeVal), SubType(stypeVal), Version(versionVal), length, tail, nil
}

// ---- Length Encoding (unexported) ----

// encodeLength encodes bit length to header length field value.
//
// Semantics depend on MainType:
//   - META/SEMANTIC/CONTENT/DATA/INSTANCE/FLAKE: (bitLength / 32) - 1, for a
//     multiple of 32 from 32 to 256 (an ISCC-UNIT is at most 256 bits)
//   - ISCC: pass-through (0–7, unit composition flags)
//   - ID: (bitLength - 64) / 8
func encodeLength(mtype MainType, length uint32) (uint32, error) {
	switch mtype {
	case MTMeta, MTSemantic, MTContent, MTData, MTInstance, MTFlake:
		if length >= 32 && length <= 256 && length%32 == 0 {
			return length/32 - 1, nil
		}
		return 0, fmt.Errorf("iscc: invalid length %d for MainType %d (must be a multiple of 32 from 32 to 256)", length, mtype)
	case MTIscc:
		if length <= 7 {
			return length, nil
		}
		return 0, fmt.Errorf("iscc: invalid length %d for ISCC (must be 0-7)", length)
	case MTId:
		if length >= 64 && length <= 96 && (length-64)%8 == 0 {
			return (length - 64) / 8, nil
		}
		return 0, fmt.Errorf("iscc: invalid length %d for ID (must be 64-96, step 8)", length)
	default:
		return 0, fmt.Errorf("iscc: unknown MainType: %d", mtype)
	}
}

// decodeLength decodes header length field to actual bit length.
//
// Inverse of encodeLength. Returns the number of bits in the digest.
//   - META/SEMANTIC/CONTENT/DATA/INSTANCE/FLAKE: (length + 1) * 32
//   - ISCC + Wide: 256
//   - ISCC + other: popcount(length) * 64 + 128
//   - ID: length * 8 + 64
//
// The length field is not range-checked: a META..FLAKE index above 7 decodes
// to more than 256 bits, which is not a valid ISCC-UNIT. Decoders call
// unitBitLength, which applies that rule.
func decodeLength(mtype MainType, length uint32, stype SubType) uint32 {
	switch mtype {
	case MTMeta, MTSemantic, MTContent, MTData, MTInstance, MTFlake:
		return (length + 1) * 32
	case MTIscc:
		if stype == STWide {
			return 256
		}
		return popcount(length)*64 + 128
	case MTId:
		return length*8 + 64
	default:
		return 0
	}
}

// unitBitLength decodes a header length field to the bit length of a valid ISCC header.
//
// Same result as decodeLength, but a META..FLAKE length index above 7 (more
// than 256 bits) is an error, mirroring iscc-core's LN lookup. ISCC and ID
// lengths decode unchanged.
func unitBitLength(mtype MainType, length uint32, stype SubType) (uint32, error) {
	bits := decodeLength(mtype, length, stype)
	if bits > 256 && mtype != MTIscc && mtype != MTId {
		return 0, fmt.Errorf("iscc: invalid length %d bits for MainType %d (an ISCC-UNIT is at most 256 bits)", bits, mtype)
	}
	return bits, nil
}

// popcount returns the number of 1-bits in v.
func popcount(v uint32) uint32 {
	var count uint32
	for v != 0 {
		count += v & 1
		v >>= 1
	}
	return count
}

// ---- Unit Encoding (unexported) ----

// encodeUnits encodes optional ISCC-UNIT MainTypes as a unit combination index (0–7).
// Maps optional units (Meta, Semantic, Content) to a bitfield:
// bit 0 = Content, bit 1 = Semantic, bit 2 = Meta.
func encodeUnits(mainTypes []MainType) (uint32, error) {
	var result uint32
	for _, mt := range mainTypes {
		switch mt {
		case MTContent:
			result |= 1
		case MTSemantic:
			result |= 2
		case MTMeta:
			result |= 4
		default:
			return 0, fmt.Errorf("iscc: %d is not a valid optional unit type", mt)
		}
	}
	return result, nil
}

// decodeUnits decodes a unit combination index (0–7) to a sorted list of optional MainTypes.
// Inverse of encodeUnits. Results in MainType order: Meta, Semantic, Content.
func decodeUnits(unitID uint32) ([]MainType, error) {
	if unitID > 7 {
		return nil, fmt.Errorf("iscc: invalid unit_id: %d (must be 0-7)", unitID)
	}
	var result []MainType
	if unitID&4 != 0 {
		result = append(result, MTMeta)
	}
	if unitID&2 != 0 {
		result = append(result, MTSemantic)
	}
	if unitID&1 != 0 {
		result = append(result, MTContent)
	}
	return result, nil
}

// ---- Base32/Base64 Encoding (unexported helpers + public functions) ----

// b32Encoding is the RFC 4648 base32 encoder with no padding.
var b32Encoding = base32.StdEncoding.WithPadding(base32.NoPadding)

// b64Encoding is the RFC 4648 §5 URL-safe base64 encoder with no padding.
var b64Encoding = base64.RawURLEncoding

// encodeBase32 encodes bytes as base32 (RFC 4648, uppercase, no padding).
func encodeBase32(data []byte) string {
	return b32Encoding.EncodeToString(data)
}

// decodeBase32 decodes a base32 string to bytes (ASCII case-insensitive, no padding).
//
// Decoding is strict, matching the Rust core: the input must be the canonical
// encoding of its bytes. Padding, non-zero unused trailing bits and the
// carriage returns or newlines that encoding/base32 silently skips are all
// rejected, by re-encoding the decoded bytes and comparing against the
// ASCII upper-cased input.
func decodeBase32(code string) ([]byte, error) {
	upper := asciiUpper(code)
	decoded, err := b32Encoding.DecodeString(upper)
	if err != nil {
		return nil, fmt.Errorf("iscc: base32 decode error: %w", err)
	}
	if encodeBase32(decoded) != upper {
		return nil, fmt.Errorf("iscc: base32 decode error: non-canonical encoding")
	}
	return decoded, nil
}

// validPrefixes holds the valid two-character ISCC prefixes, mirroring
// iscc_core.constants.PREFIXES. Note: MA and ME are ambiguous between
// ID-V0 and ID-V1.
var validPrefixes = map[string]struct{}{
	"AA": {}, // META-NONE
	"CA": {}, // SEMANTIC-TEXT
	"CE": {}, // SEMANTIC-IMAGE
	"CI": {}, // SEMANTIC-AUDIO
	"CM": {}, // SEMANTIC-VIDEO
	"CQ": {}, // SEMANTIC-MIXED
	"EA": {}, // CONTENT-TEXT
	"EE": {}, // CONTENT-IMAGE
	"EI": {}, // CONTENT-AUDIO
	"EM": {}, // CONTENT-VIDEO
	"EQ": {}, // CONTENT-MIXED
	"GA": {}, // DATA-NONE
	"IA": {}, // INSTANCE-NONE
	"KA": {}, // ISCC-TEXT
	"KE": {}, // ISCC-IMAGE
	"KI": {}, // ISCC-AUDIO
	"KM": {}, // ISCC-VIDEO
	"KQ": {}, // ISCC-MIXED
	"KU": {}, // ISCC-SUM
	"KY": {}, // ISCC-NONE
	"K4": {}, // ISCC-WIDE
	"MA": {}, // ID-PRIVATE-V0 / ID-REALM_0-V1 (ambiguous)
	"ME": {}, // ID-BITCOIN-V0 / ID-REALM_1-V1 (ambiguous)
	"MI": {}, // ID-ETHEREUM-V0
	"MM": {}, // ID-POLYGON-V0
	"OA": {}, // FLAKE-NONE
}

// checkPrefix validates the two-character prefix of a cleaned ISCC code
// against the reference allow-list, exactly as iscc_core.codec.iscc_normalize
// does before any decoding. Without this, a structurally decodable header
// with an invalid (MainType, SubType) combination (e.g. MQ = ID subtype 4)
// would be accepted.
func checkPrefix(clean string) error {
	prefix := []rune(asciiUpper(clean))
	if len(prefix) > 2 {
		prefix = prefix[:2]
	}
	if _, ok := validPrefixes[string(prefix)]; !ok {
		return fmt.Errorf("iscc: ISCC starts with invalid prefix %s", string(prefix))
	}
	return nil
}

// EncodeBase64 encodes bytes as base64url (RFC 4648 §5, no padding).
func EncodeBase64(data []byte) string {
	return b64Encoding.EncodeToString(data)
}

// JsonToDataUrl converts a JSON string to a data-URL with base64-encoded canonical JSON.
//
// Parses and canonicalizes the JSON (sorted keys, compact format). If the JSON
// contains an "@context" key, uses "application/ld+json" media type; otherwise
// uses "application/json".
func JsonToDataUrl(jsonStr string) (string, error) {
	canonical, err := parseMetaJSON(jsonStr)
	if err != nil {
		return "", err
	}
	hasContext := jsonHasContext(jsonStr)
	return buildMetaDataURL(canonical, hasContext), nil
}

// ---- Component Encoding (public) ----

// EncodeComponent encodes an ISCC-UNIT with header and body as a base32 string.
// Produces the base32-encoded string (without "ISCC:" prefix).
// Returns an error if a type value is out of range, if mtype is MTIscc (use
// gen_iscc_code_v0 instead), if bitLength is not a multiple of 32 from 32 to 256
// (an ISCC-UNIT is at most 256 bits), or if digest is shorter than bitLength/8
// bytes.
func EncodeComponent(mtype, stype uint8, version uint8, bitLength uint32, digest []byte) (string, error) {
	mt := MainType(mtype)
	st := SubType(stype)
	vs := Version(version)

	if mt > MTFlake {
		return "", fmt.Errorf("iscc: invalid MainType: %d", mtype)
	}
	if st > STWide {
		return "", fmt.Errorf("iscc: invalid SubType: %d", stype)
	}
	// Version 1 is only valid for ISCC-IDv1 (MainType ID); all other MainTypes require Version 0.
	if vs > VSV0 && (mt != MTId || vs != VSV1) {
		return "", fmt.Errorf("iscc: invalid Version: %d", version)
	}
	if len(digest) < int(bitLength/8) {
		return "", fmt.Errorf("iscc: digest length %d < bit_length/8 (%d)", len(digest), bitLength/8)
	}
	return encodeComponentInternal(mt, st, vs, bitLength, digest)
}

// encodeComponentInternal encodes an ISCC-UNIT from typed enums for the Gen*
// functions and IsccDecompose. It rejects MTIscc but performs no range or
// digest-length checks: the body is digest truncated to bitLength/8 bytes, or the
// whole digest when it is shorter (as iscc-core's encode_component slices).
func encodeComponentInternal(mtype MainType, stype SubType, version Version, bitLength uint32, digest []byte) (string, error) {
	if mtype == MTIscc {
		return "", fmt.Errorf("iscc: ISCC MainType is not a unit; use gen_iscc_code_v0 instead")
	}

	encodedLength, err := encodeLength(mtype, bitLength)
	if err != nil {
		return "", err
	}
	header, err := encodeHeader(mtype, stype, version, encodedLength)
	if err != nil {
		return "", err
	}

	bodyLen := int(bitLength / 8)
	if bodyLen > len(digest) {
		bodyLen = len(digest)
	}

	component := make([]byte, 0, len(header)+bodyLen)
	component = append(component, header...)
	component = append(component, digest[:bodyLen]...)

	return encodeBase32(component), nil
}

// ---- ISCC Clean (unexported) ----

// isPyStripChar reports whether Python's str.strip() (no arguments) removes r.
//
// Python's whitespace set is the Unicode White_Space set (unicode.IsSpace)
// plus the four information separators U+001C..U+001F.
func isPyStripChar(r rune) bool {
	return unicode.IsSpace(r) || (r >= 0x1C && r <= 0x1F)
}

// isIsccScheme reports whether scheme equals "iscc" under ASCII-only case
// folding. Unicode-aware folding would accept "iſcc" or "İSCC", which the
// reference rejects.
func isIsccScheme(scheme string) bool {
	const want = "iscc"
	if len(scheme) != len(want) {
		return false
	}
	for i := 0; i < len(want); i++ {
		if scheme[i]|0x20 != want[i] {
			return false
		}
	}
	return true
}

// asciiUpper upper-cases the ASCII letters a-z and copies every other byte
// unchanged. Unicode-aware upper-casing would map "ı" or "ſ" to ASCII letters,
// aliasing codes that the reference's ASCII-only base32 decoder rejects.
// Returns s itself, without allocating, when it holds no lowercase ASCII letter.
func asciiUpper(s string) string {
	first := 0
	for first < len(s) && (s[first] < 'a' || s[first] > 'z') {
		first++
	}
	if first == len(s) {
		return s
	}
	b := []byte(s)
	for i := first; i < len(b); i++ {
		if c := b[i]; c >= 'a' && c <= 'z' {
			b[i] = c - ('a' - 'A')
		}
	}
	return string(b)
}

// isccClean cleans up an ISCC string to its bare base32 form.
//
// Mirrors iscc_core.codec.iscc_clean (and the Rust core's iscc_clean): trims
// surrounding whitespace (the str.strip() set, see isPyStripChar), removes an
// optional scheme prefix matched ASCII case-insensitively against "iscc", and
// strips the dash group separators of the display form. A single-part input
// whose first character is a multibase prefix (f, b, v, z, u) keeps its dashes.
//
// Returns an error when a two-part input uses a scheme other than "iscc", when
// the input contains more than one colon, or when the cleaned result is empty
// (decodeBase32("") succeeds, so an empty code would otherwise be accepted).
func isccClean(iscc string) (string, error) {
	trimmed := strings.TrimFunc(iscc, isPyStripChar)
	var code string
	scheme, rest, found := strings.Cut(trimmed, ":")
	if !found {
		code = trimmed
		if code != "" && !strings.ContainsRune("fbvzu", rune(code[0])) {
			code = strings.ReplaceAll(code, "-", "")
		}
	} else {
		scheme = strings.TrimFunc(scheme, isPyStripChar)
		code = strings.TrimFunc(rest, isPyStripChar)
		if strings.Contains(code, ":") {
			return "", fmt.Errorf("iscc: malformed ISCC string: more than one colon")
		}
		if !isIsccScheme(scheme) {
			return "", fmt.Errorf("iscc: invalid scheme: expected \"iscc\"")
		}
		code = strings.ReplaceAll(code, "-", "")
	}
	if code == "" {
		return "", fmt.Errorf("iscc: empty ISCC string")
	}
	return code, nil
}

// ---- ISCC Decompose (public) ----

// IsccDecompose decomposes a composite ISCC-CODE or ISCC sequence into individual ISCC-UNITs.
// The input is cleaned first like iscc_core.codec.iscc_clean (surrounding
// whitespace, an optional case-insensitive "ISCC:" scheme and dashes are
// removed). Returns a list of base32-encoded ISCC-UNIT strings (without
// "ISCC:" prefix).
func IsccDecompose(isccCode string) ([]string, error) {
	clean, err := isccClean(isccCode)
	if err != nil {
		return nil, err
	}
	rawCode, err := decodeBase32(clean)
	if err != nil {
		return nil, err
	}

	var components []string
	for len(rawCode) > 0 {
		mt, st, vs, ln, body, err := decodeHeader(rawCode)
		if err != nil {
			return nil, err
		}

		// Standard ISCC-UNIT with tail continuation
		if mt != MTIscc {
			lnBits, err := unitBitLength(mt, ln, st)
			if err != nil {
				return nil, err
			}
			nbytes := int(lnBits / 8)
			if len(body) < nbytes {
				return nil, fmt.Errorf("iscc: truncated ISCC body: expected %d bytes, got %d", nbytes, len(body))
			}
			code, err := encodeComponentInternal(mt, st, vs, lnBits, body[:nbytes])
			if err != nil {
				return nil, err
			}
			components = append(components, code)
			rawCode = body[nbytes:]
			continue
		}

		// ISCC-CODE: decode into constituent units
		mainTypes, err := decodeUnits(ln)
		if err != nil {
			return nil, err
		}

		// Wide mode: 128-bit Data-Code + 128-bit Instance-Code
		if st == STWide {
			if len(body) < 32 {
				return nil, fmt.Errorf("iscc: truncated ISCC body: expected 32 bytes, got %d", len(body))
			}
			dataCode, err := encodeComponentInternal(MTData, STNone, vs, 128, body[:16])
			if err != nil {
				return nil, err
			}
			instanceCode, err := encodeComponentInternal(MTInstance, STNone, vs, 128, body[16:32])
			if err != nil {
				return nil, err
			}
			components = append(components, dataCode, instanceCode)
			break
		}

		// Non-wide ISCC-CODE: total body = dynamic units × 8 + Data 8 + Instance 8
		expectedBody := len(mainTypes)*8 + 16
		if len(body) < expectedBody {
			return nil, fmt.Errorf("iscc: truncated ISCC body: expected %d bytes, got %d", expectedBody, len(body))
		}

		// Rebuild dynamic units (Meta, Semantic, Content)
		for idx, mtype := range mainTypes {
			unitStype := st
			if mtype == MTMeta {
				unitStype = STNone
			}
			code, err := encodeComponentInternal(mtype, unitStype, vs, 64, body[idx*8:])
			if err != nil {
				return nil, err
			}
			components = append(components, code)
		}

		// Rebuild static units (Data-Code, Instance-Code)
		dataCode, err := encodeComponentInternal(MTData, STNone, vs, 64, body[len(body)-16:len(body)-8])
		if err != nil {
			return nil, err
		}
		instanceCode, err := encodeComponentInternal(MTInstance, STNone, vs, 64, body[len(body)-8:])
		if err != nil {
			return nil, err
		}
		components = append(components, dataCode, instanceCode)
		break
	}

	return components, nil
}

// ---- ISCC Decode (public) ----

// DecodeResult holds the decoded header components and raw digest of an ISCC unit.
type DecodeResult struct {
	Maintype uint8
	Subtype  uint8
	Version  uint8
	Length   uint8
	Digest   []byte
}

// isccNormalize returns the canonical shortest form of an ISCC, without the
// "ISCC:" prefix.
//
// Mirrors iscc_core.codec.iscc_normalize: decomposes the input into ISCC-UNITs
// and recomposes them into a single ISCC-CODE when two or more are present,
// otherwise returns the sole unit unchanged. This is what lets a concatenated
// unit sequence and a composite ISCC-CODE normalize to the same string.
//
// Multiformat (multibase-prefixed) inputs are not supported; the reference
// handles those in normalize_multiformat before this step.
func isccNormalize(iscc string) (string, error) {
	// Wide-mode detection reads the original header, before decomposition.
	clean, err := isccClean(iscc)
	if err != nil {
		return "", err
	}
	if err := checkPrefix(clean); err != nil {
		return "", err
	}
	raw, err := decodeBase32(clean)
	if err != nil {
		return "", err
	}
	mt, st, _, _, _, err := decodeHeader(raw)
	if err != nil {
		return "", err
	}
	isWide := mt == MTIscc && st == STWide

	decomposed, err := IsccDecompose(clean)
	if err != nil {
		return "", err
	}
	if len(decomposed) >= 2 {
		composed, err := GenIsccCodeV0(decomposed, isWide)
		if err != nil {
			return "", err
		}
		return strings.TrimPrefix(composed.Iscc, "ISCC:"), nil
	}
	if len(decomposed) == 0 {
		return "", fmt.Errorf("iscc: decomposed to zero ISCC-UNITs")
	}
	return decomposed[0], nil
}

// IsccDecode decodes an ISCC string into its header components and raw digest.
//
// Normalizes the input to its canonical form first (matching
// iscc_core.codec.iscc_decode), then base32-decodes it and parses the header.
// A concatenated sequence of ISCC-UNITs is therefore composed into a single
// ISCC-CODE before decoding, so a sequence and its composite form decode
// identically. The input is cleaned first like iscc_core.codec.iscc_clean:
// surrounding whitespace, an optional case-insensitive "ISCC:" scheme and
// dashes are removed.
func IsccDecode(iscc string) (*DecodeResult, error) {
	normalized, err := isccNormalize(iscc)
	if err != nil {
		return nil, err
	}
	raw, err := decodeBase32(normalized)
	if err != nil {
		return nil, err
	}
	mt, st, vs, lengthIndex, tail, err := decodeHeader(raw)
	if err != nil {
		return nil, err
	}
	bitLength, err := unitBitLength(mt, lengthIndex, st)
	if err != nil {
		return nil, err
	}
	nbytes := int(bitLength / 8)
	// No length check here: normalization guarantees an exact body. IsccDecompose
	// rejects a short unit body and truncates each unit to its encoded length, and
	// a recomposed ISCC-CODE is well-formed by construction.
	digest := make([]byte, nbytes)
	copy(digest, tail[:nbytes])

	return &DecodeResult{
		Maintype: uint8(mt),
		Subtype:  uint8(st),
		Version:  uint8(vs),
		Length:   uint8(lengthIndex),
		Digest:   digest,
	}, nil
}

// ---- ISCC-SEQ (IEP-0020) ----

// seqMainTypes lists the MainTypes permitted in an ISCC-SEQ (IEP-0020):
// META, SEMANTIC, CONTENT, DATA and INSTANCE. ISCC-CODE, ID and FLAKE units
// are rejected by readUnit, EncodeSeq and DecodeSeq.
var seqMainTypes = [...]MainType{MTMeta, MTSemantic, MTContent, MTData, MTInstance}

// headerMax is the upper bound of an encoded ISCC-HEADER in bytes (four 16-bit varnibbles).
const headerMax = 8

// isSeqMainType reports whether the raw MainType value mt is permitted in an ISCC-SEQ.
func isSeqMainType(mt uint32) bool {
	for _, m := range seqMainTypes {
		if uint32(m) == mt {
			return true
		}
	}
	return false
}

// readUnit validates one strictly encoded ISCC-UNIT starting at offset in an
// ISCC-SEQ byte string and returns the offset where it ends (the start of the
// next unit). Mirrors iscc_core.codec.read_unit. Unknown SubType and Version
// values are accepted.
//
// Returns an error (checked in this order) if the header is incomplete
// (including an offset outside data), the header is non-canonical (e.g. a
// non-zero padding nibble), the MainType is not permitted in an ISCC-SEQ, the
// Length field is above 7, or the body runs past the end of data.
func readUnit(data []byte, offset int) (int, error) {
	// An out-of-range offset yields an empty window, which fails as a malformed
	// header (as in the reference) instead of panicking on the slice.
	var window []byte
	if offset >= 0 && offset < len(data) {
		window = data[offset:min(offset+headerMax, len(data))]
	}
	mt, st, vs, ln, _, err := decodeHeaderRaw(window)
	if err != nil {
		return 0, fmt.Errorf("iscc: Malformed ISCC-HEADER at offset %d: %w", offset, err)
	}
	header, err := encodeHeaderRaw(mt, st, vs, ln)
	if err != nil {
		return 0, err
	}
	if !bytes.HasPrefix(window, header) {
		return 0, fmt.Errorf("iscc: Non-canonical ISCC-HEADER at offset %d", offset)
	}
	if !isSeqMainType(mt) {
		return 0, fmt.Errorf("iscc: MainType %d is not permitted in an ISCC-SEQ (offset %d)", mt, offset)
	}
	if ln > 7 {
		return 0, fmt.Errorf("iscc: Unsupported Length field %d at offset %d", ln, offset)
	}
	end := offset + len(header) + 4*(int(ln)+1)
	if end > len(data) {
		return 0, fmt.Errorf("iscc: Truncated ISCC-UNIT at offset %d", offset)
	}
	return end, nil
}

// EncodeSeq encodes ISCC-UNIT strings as an ISCC-SEQ byte string (IEP-0020).
//
// Returns the concatenation of the header and body bytes of each unit, in the
// given order (never sorted, deduplicated or dropped). Unit strings may omit
// the "ISCC:" prefix and may be lowercase or contain hyphens. Mirrors
// iscc_core.codec.encode_seq, except that base32 input is strict: padding,
// non-zero spare bits and non-ASCII case-fold aliases (e.g. "ſ") are rejected.
//
// Any 32 to 256-bit unit of any SubType and Version is accepted. A C2PA
// io.iscc.v0 soft binding value additionally requires Version 0 units with
// 256-bit bodies; callers producing such values must enforce that.
//
// Returns an error if units is empty, or a unit is empty, malformed,
// multiformat-encoded, of a MainType outside META, SEMANTIC, CONTENT, DATA and
// INSTANCE (ISCC-CODE, ID, FLAKE), or carries trailing data (e.g. several units
// in one string).
func EncodeSeq(units []string) ([]byte, error) {
	if len(units) == 0 {
		return nil, fmt.Errorf("iscc: ISCC-SEQ requires at least one ISCC-UNIT")
	}
	var seq []byte
	for _, unit := range units {
		if strings.TrimFunc(unit, isPyStripChar) == "" {
			return nil, fmt.Errorf("iscc: ISCC-UNIT must not be empty")
		}
		// ASCII-uppercase first: isccClean keeps hyphens in bare strings that start
		// with a multibase prefix character, and bare lowercase units with
		// SubType >= 8 start with "b" or "f".
		clean, err := isccClean(asciiUpper(unit))
		if err != nil {
			return nil, err
		}
		raw, err := decodeBase32(clean)
		if err != nil {
			return nil, err
		}
		end, err := readUnit(raw, 0)
		if err != nil {
			return nil, err
		}
		if end != len(raw) {
			return nil, fmt.Errorf("iscc: Trailing data in ISCC-UNIT %q", unit)
		}
		seq = append(seq, raw...)
	}
	return seq, nil
}

// DecodeSeq decodes an ISCC-SEQ byte string to canonical ISCC-UNIT strings (IEP-0020).
//
// Strict byte-level reader: applies every readUnit rule to each unit and
// rejects trailing bytes. Unknown SubType and Version values are preserved.
// Returns "ISCC:"-prefixed unit strings in wire order; EncodeSeq of the result
// reproduces data. Mirrors iscc_core.codec.decode_seq.
//
// Returns an error if data is empty or any unit is malformed.
func DecodeSeq(data []byte) ([]string, error) {
	if len(data) == 0 {
		return nil, fmt.Errorf("iscc: Empty ISCC-SEQ")
	}
	var units []string
	offset := 0
	for offset < len(data) {
		end, err := readUnit(data, offset)
		if err != nil {
			return nil, err
		}
		units = append(units, "ISCC:"+encodeBase32(data[offset:end]))
		offset = end
	}
	return units, nil
}
