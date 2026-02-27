// Pure Go implementation of ISCC Meta-Code generation.
// Produces similarity-preserving fingerprints from metadata (name, description,
// structured JSON) using SimHash and BLAKE3. Matches the Rust gen_meta_code_v0.
package iscc

import (
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/zeebo/blake3"
)

// MetaCodeResult holds the output of GenMetaCodeV0.
type MetaCodeResult struct {
	Iscc        string // ISCC code string with "ISCC:" prefix
	Name        string // Normalized name
	Description string // Normalized description (empty if none)
	Meta        string // Data-URL of structured metadata (empty if none)
	Metahash    string // Hex-encoded multihash (BLAKE3) of metadata payload
}

// GenMetaCodeV0 generates an ISCC Meta-Code from name and optional metadata.
// Hashes name, description, and meta fields using SimHash to produce a
// similarity-preserving fingerprint. Meta can be a JSON string or data-URL.
func GenMetaCodeV0(name string, description, meta *string, bits uint32) (*MetaCodeResult, error) {
	// Normalize name: clean → remove newlines → trim to 128 bytes
	cleanName := TextClean(name)
	cleanName = TextRemoveNewlines(cleanName)
	cleanName = TextTrim(cleanName, MetaTrimName)

	if cleanName == "" {
		return nil, fmt.Errorf("iscc: name is empty after normalization")
	}

	// Normalize description: clean → trim to 4096 bytes
	descStr := ""
	if description != nil {
		descStr = *description
	}
	descClean := TextClean(descStr)
	descClean = TextTrim(descClean, MetaTrimDescription)

	// Resolve meta payload bytes (if meta is provided)
	var metaPayload []byte
	var metaIsDataURL bool
	if meta != nil {
		metaStr := *meta
		if strings.HasPrefix(metaStr, "data:") {
			payload, err := decodeDataURL(metaStr)
			if err != nil {
				return nil, err
			}
			metaPayload = payload
			metaIsDataURL = true
		} else {
			payload, err := parseMetaJSON(metaStr)
			if err != nil {
				return nil, err
			}
			metaPayload = payload
		}
	}

	// Branch: meta bytes path vs. description text path
	if metaPayload != nil {
		metaCodeDigest := softHashMetaV0WithBytes(cleanName, metaPayload)
		metahash := multiHashBlake3(metaPayload)

		component, err := EncodeComponent(
			uint8(MTMeta), uint8(STNone), uint8(VSV0), bits, metaCodeDigest,
		)
		if err != nil {
			return nil, err
		}

		// Build the meta Data-URL for the result
		var metaValue string
		if metaIsDataURL {
			metaValue = *meta
		} else {
			hasContext := jsonHasContext(*meta)
			metaValue = buildMetaDataURL(metaPayload, hasContext)
		}

		return &MetaCodeResult{
			Iscc:        "ISCC:" + component,
			Name:        cleanName,
			Description: descClean,
			Meta:        metaValue,
			Metahash:    metahash,
		}, nil
	}

	// No meta — compute metahash from normalized text payload
	payload := cleanName
	if descClean != "" {
		payload = cleanName + " " + descClean
	}
	payload = strings.TrimSpace(payload)
	metahash := multiHashBlake3([]byte(payload))

	// Compute similarity digest
	var extra *string
	if descClean != "" {
		extra = &descClean
	}
	metaCodeDigest := softHashMetaV0(cleanName, extra)

	component, err := EncodeComponent(
		uint8(MTMeta), uint8(STNone), uint8(VSV0), bits, metaCodeDigest,
	)
	if err != nil {
		return nil, err
	}

	return &MetaCodeResult{
		Iscc:        "ISCC:" + component,
		Name:        cleanName,
		Description: descClean,
		Meta:        "",
		Metahash:    metahash,
	}, nil
}

// metaNameSimhash computes a SimHash digest from name text for meta hashing.
// Applies TextCollapse, generates width-3 sliding window n-grams,
// hashes each with BLAKE3, and produces a SimHash.
func metaNameSimhash(name string) []byte {
	collapsed := TextCollapse(name)
	ngrams, _ := SlidingWindow(collapsed, 3)
	hashes := make([][]byte, len(ngrams))
	for i, ng := range ngrams {
		digest := blake3.Sum256([]byte(ng))
		hashes[i] = digest[:]
	}
	result, _ := AlgSimhash(hashes)
	return result
}

// softHashMetaV0 computes a similarity-preserving 256-bit hash from metadata text.
// Produces a SimHash digest from name n-grams. When extra is non-nil and non-empty,
// interleaves the name and extra SimHash digests in 4-byte chunks.
func softHashMetaV0(name string, extra *string) []byte {
	nameSimhash := metaNameSimhash(name)

	if extra == nil || *extra == "" {
		return nameSimhash
	}

	collapsedExtra := TextCollapse(*extra)
	extraNgrams, _ := SlidingWindow(collapsedExtra, 3)
	extraHashes := make([][]byte, len(extraNgrams))
	for i, ng := range extraNgrams {
		digest := blake3.Sum256([]byte(ng))
		extraHashes[i] = digest[:]
	}
	extraSimhash, _ := AlgSimhash(extraHashes)

	return interleaveDigests(nameSimhash, extraSimhash)
}

// softHashMetaV0WithBytes computes a similarity-preserving hash from name text and raw bytes.
// Uses width-4 byte sliding windows for the bytes path and interleaves
// name/bytes SimHash digests in 4-byte chunks.
func softHashMetaV0WithBytes(name string, extra []byte) []byte {
	nameSimhash := metaNameSimhash(name)

	if len(extra) == 0 {
		return nameSimhash
	}

	byteNgrams := slidingWindowBytes(extra, 4)
	byteHashes := make([][]byte, len(byteNgrams))
	for i, ng := range byteNgrams {
		digest := blake3.Sum256(ng)
		byteHashes[i] = digest[:]
	}
	byteSimhash, _ := AlgSimhash(byteHashes)

	return interleaveDigests(nameSimhash, byteSimhash)
}

// interleaveDigests interleaves two 32-byte SimHash digests in 4-byte chunks.
// Takes the first 16 bytes of each and interleaves: a[0:4] || b[0:4] ||
// a[4:8] || b[4:8] || ... || a[12:16] || b[12:16] → 32 bytes.
func interleaveDigests(a, b []byte) []byte {
	result := make([]byte, 32)
	for chunk := 0; chunk < 4; chunk++ {
		src := chunk * 4
		dstA := chunk * 8
		dstB := chunk*8 + 4
		copy(result[dstA:dstA+4], a[src:src+4])
		copy(result[dstB:dstB+4], b[src:src+4])
	}
	return result
}

// slidingWindowBytes generates overlapping byte sub-slices of the given width.
// If data is shorter than width, returns a single element with the full data.
func slidingWindowBytes(data []byte, width int) [][]byte {
	dataLen := len(data)
	rangeVal := dataLen - width + 1
	if rangeVal < 1 {
		rangeVal = 1
	}
	result := make([][]byte, rangeVal)
	for i := 0; i < rangeVal; i++ {
		end := i + width
		if end > dataLen {
			end = dataLen
		}
		result[i] = data[i:end]
	}
	return result
}

// decodeDataURL decodes a data-URL's base64 payload.
// Splits on the first comma and decodes the remainder as standard base64.
func decodeDataURL(dataURL string) ([]byte, error) {
	idx := strings.Index(dataURL, ",")
	if idx < 0 {
		return nil, fmt.Errorf("iscc: data-URL missing comma separator")
	}
	payload := dataURL[idx+1:]
	decoded, err := base64.StdEncoding.DecodeString(payload)
	if err != nil {
		return nil, fmt.Errorf("iscc: invalid base64 in data-URL: %w", err)
	}
	return decoded, nil
}

// parseMetaJSON parses a JSON string and re-serializes to RFC 8785 (JCS) canonical bytes.
// Go's json.Marshal produces sorted keys and compact format, which matches JCS for
// string/boolean/null values. This covers all ISCC conformance vectors.
func parseMetaJSON(metaStr string) ([]byte, error) {
	var parsed interface{}
	if err := json.Unmarshal([]byte(metaStr), &parsed); err != nil {
		return nil, fmt.Errorf("iscc: invalid JSON in meta: %w", err)
	}
	canonical, err := json.Marshal(parsed)
	if err != nil {
		return nil, fmt.Errorf("iscc: JSON canonicalization failed: %w", err)
	}
	return canonical, nil
}

// jsonHasContext checks if a JSON string contains an "@context" key at the top level.
func jsonHasContext(jsonStr string) bool {
	var obj map[string]interface{}
	if err := json.Unmarshal([]byte(jsonStr), &obj); err != nil {
		return false
	}
	_, ok := obj["@context"]
	return ok
}

// buildMetaDataURL formats canonical JSON bytes as a data-URL.
// Uses application/ld+json media type if hasContext is true,
// otherwise application/json.
func buildMetaDataURL(jsonBytes []byte, hasContext bool) string {
	mediaType := "application/json"
	if hasContext {
		mediaType = "application/ld+json"
	}
	b64 := base64.StdEncoding.EncodeToString(jsonBytes)
	return fmt.Sprintf("data:%s;base64,%s", mediaType, b64)
}

// multiHashBlake3 computes a BLAKE3 multihash and returns it as hex string.
// Prepends the BLAKE3 multicodec prefix (0x1e) and digest length (0x20 = 32).
func multiHashBlake3(data []byte) string {
	digest := blake3.Sum256(data)
	result := make([]byte, 34)
	result[0] = 0x1e // BLAKE3 multicodec
	result[1] = 0x20 // 32 bytes length
	copy(result[2:], digest[:])
	return hex.EncodeToString(result)
}
