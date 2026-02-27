// Package iscc provides Go bindings for the ISCC (International Standard Content Code)
// library via a WebAssembly module powered by the wazero runtime.
//
// The package embeds a prebuilt WASM binary of iscc-ffi (Rust) and exposes a Runtime
// type with all 9 gen_*_v0 code generation functions plus utility functions. Memory
// management across the Go→WASM boundary is handled internally.
package iscc

import (
	"context"
	_ "embed"
	"encoding/binary"
	"errors"
	"fmt"
	"io"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/api"
	"github.com/tetratelabs/wazero/imports/wasi_snapshot_preview1"
)

// Algorithm configuration constants matching iscc-core core_opts.
const (
	MetaTrimName        = 128
	MetaTrimDescription = 4096
	IoReadSize          = 4_194_304
	TextNgramSize       = 13
)

//go:embed iscc_ffi.wasm
var wasmModule []byte

// DecodeResult holds the decoded header components and raw digest of an ISCC unit.
type DecodeResult struct {
	Maintype uint8
	Subtype  uint8
	Version  uint8
	Length   uint8
	Digest   []byte
}

// Runtime holds the wazero WASM runtime and the instantiated iscc-ffi module.
// Create with NewRuntime and release resources with Close.
type Runtime struct {
	runtime wazero.Runtime
	mod     api.Module
}

// NewRuntime creates a new WASM runtime, instantiates the WASI layer, and
// loads the embedded iscc-ffi module. The caller must call Close when done.
func NewRuntime(ctx context.Context) (*Runtime, error) {
	r := wazero.NewRuntime(ctx)

	// WASI is required because iscc-ffi targets wasm32-wasip1.
	wasi_snapshot_preview1.MustInstantiate(ctx, r)

	// Suppress WASI stdout/stderr noise.
	cfg := wazero.NewModuleConfig().
		WithStdout(io.Discard).
		WithStderr(io.Discard)

	mod, err := r.InstantiateWithConfig(ctx, wasmModule, cfg)
	if err != nil {
		_ = r.Close(ctx)
		return nil, fmt.Errorf("iscc: instantiate wasm module: %w", err)
	}

	return &Runtime{
		runtime: r,
		mod:     mod,
	}, nil
}

// Close releases all resources associated with the Runtime.
func (rt *Runtime) Close(ctx context.Context) error {
	var errs []error
	if rt.mod != nil {
		if err := rt.mod.Close(ctx); err != nil {
			errs = append(errs, err)
		}
	}
	if rt.runtime != nil {
		if err := rt.runtime.Close(ctx); err != nil {
			errs = append(errs, err)
		}
	}
	return errors.Join(errs...)
}

// alloc calls iscc_alloc(size) to allocate WASM-side memory.
// Returns the WASM pointer to the allocated region.
func (rt *Runtime) alloc(ctx context.Context, size uint32) (uint32, error) {
	fn := rt.mod.ExportedFunction("iscc_alloc")
	results, err := fn.Call(ctx, uint64(size))
	if err != nil {
		return 0, fmt.Errorf("iscc_alloc: %w", err)
	}
	return uint32(results[0]), nil
}

// dealloc calls iscc_dealloc(ptr, size) to free WASM-side memory.
func (rt *Runtime) dealloc(ctx context.Context, ptr, size uint32) error {
	fn := rt.mod.ExportedFunction("iscc_dealloc")
	_, err := fn.Call(ctx, uint64(ptr), uint64(size))
	if err != nil {
		return fmt.Errorf("iscc_dealloc: %w", err)
	}
	return nil
}

// writeString allocates WASM memory, writes the UTF-8 bytes of s plus a null
// terminator, and returns the WASM pointer and the total byte count (including
// the null terminator).
func (rt *Runtime) writeString(ctx context.Context, s string) (ptr, size uint32, err error) {
	b := []byte(s)
	totalSize := uint32(len(b)) + 1 // +1 for null terminator
	ptr, err = rt.alloc(ctx, totalSize)
	if err != nil {
		return 0, 0, err
	}
	// Write UTF-8 bytes + null terminator.
	buf := make([]byte, totalSize)
	copy(buf, b)
	buf[len(buf)-1] = 0 // null terminator
	if !rt.mod.Memory().Write(ptr, buf) {
		return 0, 0, fmt.Errorf("iscc: write string: out of bounds (ptr=%d, size=%d)", ptr, totalSize)
	}
	return ptr, totalSize, nil
}

// readString reads a null-terminated C string from WASM memory at the given pointer.
func (rt *Runtime) readString(_ context.Context, ptr uint32) (string, error) {
	mem := rt.mod.Memory()
	// Read bytes one at a time until we hit a null terminator.
	// For typical short ISCC strings this is efficient enough.
	var buf []byte
	for offset := ptr; ; offset++ {
		b, ok := mem.Read(offset, 1)
		if !ok {
			return "", fmt.Errorf("iscc: read string: out of bounds at offset %d", offset)
		}
		if b[0] == 0 {
			break
		}
		buf = append(buf, b[0])
	}
	return string(buf), nil
}

// freeString calls iscc_free_string(ptr) to free a string returned by the WASM module.
func (rt *Runtime) freeString(ctx context.Context, ptr uint32) error {
	fn := rt.mod.ExportedFunction("iscc_free_string")
	_, err := fn.Call(ctx, uint64(ptr))
	if err != nil {
		return fmt.Errorf("iscc_free_string: %w", err)
	}
	return nil
}

// lastError reads the error message from iscc_last_error().
// Returns an empty string if no error is stored.
func (rt *Runtime) lastError(ctx context.Context) string {
	fn := rt.mod.ExportedFunction("iscc_last_error")
	results, err := fn.Call(ctx)
	if err != nil {
		return ""
	}
	ptr := uint32(results[0])
	if ptr == 0 {
		return ""
	}
	// iscc_last_error returns a borrowed pointer — do not free.
	s, err := rt.readString(ctx, ptr)
	if err != nil {
		return ""
	}
	return s
}

// ConformanceSelftest runs the built-in conformance test suite.
// Returns true if all tests pass.
func (rt *Runtime) ConformanceSelftest(ctx context.Context) (bool, error) {
	fn := rt.mod.ExportedFunction("iscc_conformance_selftest")
	results, err := fn.Call(ctx)
	if err != nil {
		return false, fmt.Errorf("iscc_conformance_selftest: %w", err)
	}
	return results[0] != 0, nil
}

// TextClean calls iscc_text_clean on the input string and returns the cleaned result.
func (rt *Runtime) TextClean(ctx context.Context, text string) (string, error) {
	textPtr, textSize, err := rt.writeString(ctx, text)
	if err != nil {
		return "", err
	}
	defer func() { _ = rt.dealloc(ctx, textPtr, textSize) }()

	fn := rt.mod.ExportedFunction("iscc_text_clean")
	results, err := fn.Call(ctx, uint64(textPtr))
	if err != nil {
		return "", fmt.Errorf("iscc_text_clean: %w", err)
	}

	resultPtr := uint32(results[0])
	if resultPtr == 0 {
		return "", fmt.Errorf("iscc_text_clean failed: %s", rt.lastError(ctx))
	}

	result, err := rt.readString(ctx, resultPtr)
	if err != nil {
		return "", err
	}
	_ = rt.freeString(ctx, resultPtr)
	return result, nil
}

// ── Additional memory helpers ───────────────────────────────────────────────

// writeBytes allocates WASM memory and writes raw bytes (no null terminator).
// For empty data, returns a non-null dangling pointer with size=0 (Rust FFI rejects
// NULL pointers, but accepts non-null with len=0 as an empty slice).
func (rt *Runtime) writeBytes(ctx context.Context, data []byte) (ptr, size uint32, err error) {
	size = uint32(len(data))
	// alloc(0) returns a non-null dangling pointer; dealloc(ptr, 0) is a no-op.
	ptr, err = rt.alloc(ctx, size)
	if err != nil {
		return 0, 0, err
	}
	if size > 0 {
		if !rt.mod.Memory().Write(ptr, data) {
			return 0, 0, fmt.Errorf("iscc: write bytes: out of bounds (ptr=%d, size=%d)", ptr, size)
		}
	}
	return ptr, size, nil
}

// writeI32Slice allocates WASM memory and writes int32 values in little-endian format.
// For empty slices, allocates 4 bytes to ensure proper i32 alignment (the Rust FFI
// calls slice::from_raw_parts which requires alignment even for zero-length slices).
func (rt *Runtime) writeI32Slice(ctx context.Context, values []int32) (ptr, allocSize, count uint32, err error) {
	count = uint32(len(values))
	byteSize := count * 4
	// Allocate at least 4 bytes so the pointer is properly aligned for i32.
	// iscc_alloc(0) returns a dangling pointer with alignment 1, which would fail
	// the alignment check in slice::from_raw_parts for i32 (alignment 4).
	allocSize = byteSize
	if allocSize == 0 {
		allocSize = 4
	}
	ptr, err = rt.alloc(ctx, allocSize)
	if err != nil {
		return 0, 0, 0, err
	}
	if byteSize > 0 {
		buf := make([]byte, byteSize)
		for i, v := range values {
			binary.LittleEndian.PutUint32(buf[i*4:], uint32(v))
		}
		if !rt.mod.Memory().Write(ptr, buf) {
			return 0, 0, 0, fmt.Errorf("iscc: write i32 slice: out of bounds (ptr=%d, size=%d)", ptr, byteSize)
		}
	}
	return ptr, allocSize, count, nil
}

// writeStringArray allocates individual null-terminated strings plus a pointer array
// in WASM memory. Returns the WASM pointer to the pointer array, the count, and a
// cleanup function that frees all strings and the pointer array.
func (rt *Runtime) writeStringArray(ctx context.Context, strings []string) (ptrsPtr, count uint32, cleanup func(), err error) {
	count = uint32(len(strings))
	if count == 0 {
		return 0, 0, func() {}, nil
	}

	type allocEntry struct {
		ptr  uint32
		size uint32
	}
	allocs := make([]allocEntry, 0, len(strings)+1)

	doCleanup := func() {
		for _, a := range allocs {
			_ = rt.dealloc(ctx, a.ptr, a.size)
		}
	}

	// Write each string and collect WASM pointers.
	strPtrs := make([]uint32, len(strings))
	for i, s := range strings {
		sPtr, sSize, werr := rt.writeString(ctx, s)
		if werr != nil {
			doCleanup()
			return 0, 0, nil, werr
		}
		allocs = append(allocs, allocEntry{sPtr, sSize})
		strPtrs[i] = sPtr
	}

	// Build pointer array (array of uint32 WASM pointers).
	arrSize := count * 4
	arrPtr, aerr := rt.alloc(ctx, arrSize)
	if aerr != nil {
		doCleanup()
		return 0, 0, nil, aerr
	}
	allocs = append(allocs, allocEntry{arrPtr, arrSize})

	buf := make([]byte, arrSize)
	for i, sp := range strPtrs {
		binary.LittleEndian.PutUint32(buf[i*4:], sp)
	}
	if !rt.mod.Memory().Write(arrPtr, buf) {
		doCleanup()
		return 0, 0, nil, fmt.Errorf("iscc: write string array: out of bounds")
	}

	return arrPtr, count, doCleanup, nil
}

// writeI32ArrayOfArrays writes an array of i32 slices to WASM memory for video frame
// signatures. Returns pointers to the frame-pointers array and frame-lengths array,
// the frame count, and a cleanup function.
func (rt *Runtime) writeI32ArrayOfArrays(ctx context.Context, frames [][]int32) (framePtrsPtr, frameLensPtr, numFrames uint32, cleanup func(), err error) {
	numFrames = uint32(len(frames))
	if numFrames == 0 {
		return 0, 0, 0, func() {}, nil
	}

	type allocEntry struct {
		ptr  uint32
		size uint32
	}
	var allocs []allocEntry

	doCleanup := func() {
		for _, a := range allocs {
			_ = rt.dealloc(ctx, a.ptr, a.size)
		}
	}

	// Write each frame's i32 data and collect pointers and lengths.
	ptrs := make([]uint32, numFrames)
	lens := make([]uint32, numFrames)
	for i, frame := range frames {
		fPtr, fAllocSize, fCount, werr := rt.writeI32Slice(ctx, frame)
		if werr != nil {
			doCleanup()
			return 0, 0, 0, nil, werr
		}
		allocs = append(allocs, allocEntry{fPtr, fAllocSize})
		ptrs[i] = fPtr
		lens[i] = fCount
	}

	// Build frame-pointers array (uint32 per pointer).
	ptrArrSize := numFrames * 4
	ptrArrPtr, aerr := rt.alloc(ctx, ptrArrSize)
	if aerr != nil {
		doCleanup()
		return 0, 0, 0, nil, aerr
	}
	allocs = append(allocs, allocEntry{ptrArrPtr, ptrArrSize})

	ptrBuf := make([]byte, ptrArrSize)
	for i, p := range ptrs {
		binary.LittleEndian.PutUint32(ptrBuf[i*4:], p)
	}
	if !rt.mod.Memory().Write(ptrArrPtr, ptrBuf) {
		doCleanup()
		return 0, 0, 0, nil, fmt.Errorf("iscc: write frame pointers: out of bounds")
	}

	// Build frame-lengths array (uint32 per length, WASM usize is 4 bytes).
	lenArrSize := numFrames * 4
	lenArrPtr, lerr := rt.alloc(ctx, lenArrSize)
	if lerr != nil {
		doCleanup()
		return 0, 0, 0, nil, lerr
	}
	allocs = append(allocs, allocEntry{lenArrPtr, lenArrSize})

	lenBuf := make([]byte, lenArrSize)
	for i, l := range lens {
		binary.LittleEndian.PutUint32(lenBuf[i*4:], l)
	}
	if !rt.mod.Memory().Write(lenArrPtr, lenBuf) {
		doCleanup()
		return 0, 0, 0, nil, fmt.Errorf("iscc: write frame lengths: out of bounds")
	}

	return ptrArrPtr, lenArrPtr, numFrames, doCleanup, nil
}

// callStringResult calls an FFI function and handles the string result pattern:
// check NULL (ptr==0) → readString → freeString → return.
func (rt *Runtime) callStringResult(ctx context.Context, fnName string, results []uint64) (string, error) {
	resultPtr := uint32(results[0])
	if resultPtr == 0 {
		return "", fmt.Errorf("%s failed: %s", fnName, rt.lastError(ctx))
	}
	result, err := rt.readString(ctx, resultPtr)
	if err != nil {
		return "", err
	}
	_ = rt.freeString(ctx, resultPtr)
	return result, nil
}

// ── Code generation functions ───────────────────────────────────────────────

// GenMetaCodeV0 generates a Meta-Code from metadata fields.
// Pass nil for description or meta to omit those fields.
func (rt *Runtime) GenMetaCodeV0(ctx context.Context, name string, description, meta *string, bits uint32) (string, error) {
	namePtr, nameSize, err := rt.writeString(ctx, name)
	if err != nil {
		return "", err
	}
	defer func() { _ = rt.dealloc(ctx, namePtr, nameSize) }()

	// NULL pointer (0) for nil optional strings.
	var descArg, metaArg uint64
	var descPtr, descSize, metaPtr, metaSize uint32
	if description != nil {
		descPtr, descSize, err = rt.writeString(ctx, *description)
		if err != nil {
			return "", err
		}
		defer func() { _ = rt.dealloc(ctx, descPtr, descSize) }()
		descArg = uint64(descPtr)
	}
	if meta != nil {
		metaPtr, metaSize, err = rt.writeString(ctx, *meta)
		if err != nil {
			return "", err
		}
		defer func() { _ = rt.dealloc(ctx, metaPtr, metaSize) }()
		metaArg = uint64(metaPtr)
	}

	fn := rt.mod.ExportedFunction("iscc_gen_meta_code_v0")
	results, err := fn.Call(ctx, uint64(namePtr), descArg, metaArg, uint64(bits))
	if err != nil {
		return "", fmt.Errorf("iscc_gen_meta_code_v0: %w", err)
	}
	return rt.callStringResult(ctx, "iscc_gen_meta_code_v0", results)
}

// GenTextCodeV0 generates a Text-Code from text content.
func (rt *Runtime) GenTextCodeV0(ctx context.Context, text string, bits uint32) (string, error) {
	textPtr, textSize, err := rt.writeString(ctx, text)
	if err != nil {
		return "", err
	}
	defer func() { _ = rt.dealloc(ctx, textPtr, textSize) }()

	fn := rt.mod.ExportedFunction("iscc_gen_text_code_v0")
	results, err := fn.Call(ctx, uint64(textPtr), uint64(bits))
	if err != nil {
		return "", fmt.Errorf("iscc_gen_text_code_v0: %w", err)
	}
	return rt.callStringResult(ctx, "iscc_gen_text_code_v0", results)
}

// GenImageCodeV0 generates an Image-Code from 32x32 grayscale pixel data.
func (rt *Runtime) GenImageCodeV0(ctx context.Context, pixels []byte, bits uint32) (string, error) {
	pixPtr, pixSize, err := rt.writeBytes(ctx, pixels)
	if err != nil {
		return "", err
	}
	defer func() { _ = rt.dealloc(ctx, pixPtr, pixSize) }()

	fn := rt.mod.ExportedFunction("iscc_gen_image_code_v0")
	results, err := fn.Call(ctx, uint64(pixPtr), uint64(pixSize), uint64(bits))
	if err != nil {
		return "", fmt.Errorf("iscc_gen_image_code_v0: %w", err)
	}
	return rt.callStringResult(ctx, "iscc_gen_image_code_v0", results)
}

// GenAudioCodeV0 generates an Audio-Code from a Chromaprint feature vector.
func (rt *Runtime) GenAudioCodeV0(ctx context.Context, cv []int32, bits uint32) (string, error) {
	cvPtr, cvAllocSize, cvCount, err := rt.writeI32Slice(ctx, cv)
	if err != nil {
		return "", err
	}
	defer func() { _ = rt.dealloc(ctx, cvPtr, cvAllocSize) }()

	fn := rt.mod.ExportedFunction("iscc_gen_audio_code_v0")
	results, err := fn.Call(ctx, uint64(cvPtr), uint64(cvCount), uint64(bits))
	if err != nil {
		return "", fmt.Errorf("iscc_gen_audio_code_v0: %w", err)
	}
	return rt.callStringResult(ctx, "iscc_gen_audio_code_v0", results)
}

// GenVideoCodeV0 generates a Video-Code from MPEG-7 frame signatures.
func (rt *Runtime) GenVideoCodeV0(ctx context.Context, frameSigs [][]int32, bits uint32) (string, error) {
	fpPtr, flPtr, nFrames, cleanup, err := rt.writeI32ArrayOfArrays(ctx, frameSigs)
	if err != nil {
		return "", err
	}
	defer cleanup()

	fn := rt.mod.ExportedFunction("iscc_gen_video_code_v0")
	results, err := fn.Call(ctx, uint64(fpPtr), uint64(flPtr), uint64(nFrames), uint64(bits))
	if err != nil {
		return "", fmt.Errorf("iscc_gen_video_code_v0: %w", err)
	}
	return rt.callStringResult(ctx, "iscc_gen_video_code_v0", results)
}

// GenMixedCodeV0 generates a Mixed-Code from a set of ISCC unit codes.
func (rt *Runtime) GenMixedCodeV0(ctx context.Context, codes []string, bits uint32) (string, error) {
	codesPtr, codesCount, cleanup, err := rt.writeStringArray(ctx, codes)
	if err != nil {
		return "", err
	}
	defer cleanup()

	fn := rt.mod.ExportedFunction("iscc_gen_mixed_code_v0")
	results, err := fn.Call(ctx, uint64(codesPtr), uint64(codesCount), uint64(bits))
	if err != nil {
		return "", fmt.Errorf("iscc_gen_mixed_code_v0: %w", err)
	}
	return rt.callStringResult(ctx, "iscc_gen_mixed_code_v0", results)
}

// GenDataCodeV0 generates a Data-Code from raw byte data.
func (rt *Runtime) GenDataCodeV0(ctx context.Context, data []byte, bits uint32) (string, error) {
	dataPtr, dataSize, err := rt.writeBytes(ctx, data)
	if err != nil {
		return "", err
	}
	defer func() { _ = rt.dealloc(ctx, dataPtr, dataSize) }()

	fn := rt.mod.ExportedFunction("iscc_gen_data_code_v0")
	results, err := fn.Call(ctx, uint64(dataPtr), uint64(dataSize), uint64(bits))
	if err != nil {
		return "", fmt.Errorf("iscc_gen_data_code_v0: %w", err)
	}
	return rt.callStringResult(ctx, "iscc_gen_data_code_v0", results)
}

// GenInstanceCodeV0 generates an Instance-Code from raw byte data.
func (rt *Runtime) GenInstanceCodeV0(ctx context.Context, data []byte, bits uint32) (string, error) {
	dataPtr, dataSize, err := rt.writeBytes(ctx, data)
	if err != nil {
		return "", err
	}
	defer func() { _ = rt.dealloc(ctx, dataPtr, dataSize) }()

	fn := rt.mod.ExportedFunction("iscc_gen_instance_code_v0")
	results, err := fn.Call(ctx, uint64(dataPtr), uint64(dataSize), uint64(bits))
	if err != nil {
		return "", fmt.Errorf("iscc_gen_instance_code_v0: %w", err)
	}
	return rt.callStringResult(ctx, "iscc_gen_instance_code_v0", results)
}

// TextRemoveNewlines replaces newline characters with spaces.
func (rt *Runtime) TextRemoveNewlines(ctx context.Context, text string) (string, error) {
	textPtr, textSize, err := rt.writeString(ctx, text)
	if err != nil {
		return "", err
	}
	defer func() { _ = rt.dealloc(ctx, textPtr, textSize) }()

	fn := rt.mod.ExportedFunction("iscc_text_remove_newlines")
	results, err := fn.Call(ctx, uint64(textPtr))
	if err != nil {
		return "", fmt.Errorf("iscc_text_remove_newlines: %w", err)
	}
	return rt.callStringResult(ctx, "iscc_text_remove_newlines", results)
}

// TextCollapse normalizes and simplifies text for similarity hashing.
// Applies NFD normalization, lowercasing, removes whitespace and control/mark/punctuation
// characters, then recombines with NFKC normalization.
func (rt *Runtime) TextCollapse(ctx context.Context, text string) (string, error) {
	textPtr, textSize, err := rt.writeString(ctx, text)
	if err != nil {
		return "", err
	}
	defer func() { _ = rt.dealloc(ctx, textPtr, textSize) }()

	fn := rt.mod.ExportedFunction("iscc_text_collapse")
	results, err := fn.Call(ctx, uint64(textPtr))
	if err != nil {
		return "", fmt.Errorf("iscc_text_collapse: %w", err)
	}
	return rt.callStringResult(ctx, "iscc_text_collapse", results)
}

// TextTrim trims text so its UTF-8 encoded size does not exceed nbytes.
// Multi-byte characters that would be split are dropped entirely.
// Leading/trailing whitespace is stripped from the result.
func (rt *Runtime) TextTrim(ctx context.Context, text string, nbytes uint32) (string, error) {
	textPtr, textSize, err := rt.writeString(ctx, text)
	if err != nil {
		return "", err
	}
	defer func() { _ = rt.dealloc(ctx, textPtr, textSize) }()

	fn := rt.mod.ExportedFunction("iscc_text_trim")
	results, err := fn.Call(ctx, uint64(textPtr), uint64(nbytes))
	if err != nil {
		return "", fmt.Errorf("iscc_text_trim: %w", err)
	}
	return rt.callStringResult(ctx, "iscc_text_trim", results)
}

// EncodeBase64 encodes bytes as base64url (RFC 4648 section 5, no padding).
func (rt *Runtime) EncodeBase64(ctx context.Context, data []byte) (string, error) {
	dataPtr, dataSize, err := rt.writeBytes(ctx, data)
	if err != nil {
		return "", err
	}
	defer func() { _ = rt.dealloc(ctx, dataPtr, dataSize) }()

	fn := rt.mod.ExportedFunction("iscc_encode_base64")
	results, err := fn.Call(ctx, uint64(dataPtr), uint64(dataSize))
	if err != nil {
		return "", fmt.Errorf("iscc_encode_base64: %w", err)
	}
	return rt.callStringResult(ctx, "iscc_encode_base64", results)
}

// ── Codec functions ──────────────────────────────────────────────────────────

// JsonToDataUrl converts a JSON string to a data URL with base64 encoding.
// Uses application/ld+json media type when the JSON contains an @context key,
// otherwise uses application/json.
func (rt *Runtime) JsonToDataUrl(ctx context.Context, jsonStr string) (string, error) {
	jsonPtr, jsonSize, err := rt.writeString(ctx, jsonStr)
	if err != nil {
		return "", err
	}
	defer func() { _ = rt.dealloc(ctx, jsonPtr, jsonSize) }()

	fn := rt.mod.ExportedFunction("iscc_json_to_data_url")
	results, err := fn.Call(ctx, uint64(jsonPtr))
	if err != nil {
		return "", fmt.Errorf("iscc_json_to_data_url: %w", err)
	}
	return rt.callStringResult(ctx, "iscc_json_to_data_url", results)
}

// EncodeComponent encodes ISCC header components and a raw digest into an ISCC unit string.
// Parameters: mtype (MainType), stype (SubType), version, bitLength, and digest bytes.
func (rt *Runtime) EncodeComponent(ctx context.Context, mtype, stype, version uint8, bitLength uint32, digest []byte) (string, error) {
	digestPtr, digestSize, err := rt.writeBytes(ctx, digest)
	if err != nil {
		return "", err
	}
	defer func() { _ = rt.dealloc(ctx, digestPtr, digestSize) }()

	fn := rt.mod.ExportedFunction("iscc_encode_component")
	results, err := fn.Call(ctx,
		uint64(mtype), uint64(stype), uint64(version),
		uint64(bitLength), uint64(digestPtr), uint64(digestSize),
	)
	if err != nil {
		return "", fmt.Errorf("iscc_encode_component: %w", err)
	}
	return rt.callStringResult(ctx, "iscc_encode_component", results)
}

// IsccDecode decodes an ISCC unit string into its header components and raw digest.
// Strips an optional "ISCC:" prefix before decoding.
// Returns a DecodeResult with maintype, subtype, version, length index, and digest bytes.
func (rt *Runtime) IsccDecode(ctx context.Context, isccUnit string) (*DecodeResult, error) {
	// Allocate 16 bytes for sret (IsccDecodeResult struct in WASM).
	sretPtr, err := rt.alloc(ctx, 16)
	if err != nil {
		return nil, err
	}

	strPtr, strSize, err := rt.writeString(ctx, isccUnit)
	if err != nil {
		_ = rt.dealloc(ctx, sretPtr, 16)
		return nil, err
	}
	defer func() { _ = rt.dealloc(ctx, strPtr, strSize) }()

	// iscc_decode uses sret: first param is sret pointer, second is the string pointer.
	fn := rt.mod.ExportedFunction("iscc_decode")
	_, err = fn.Call(ctx, uint64(sretPtr), uint64(strPtr))
	if err != nil {
		_ = rt.dealloc(ctx, sretPtr, 16)
		return nil, fmt.Errorf("iscc_decode: %w", err)
	}

	// Read 16 bytes from sret.
	raw, ok := rt.mod.Memory().Read(sretPtr, 16)
	if !ok {
		_ = rt.dealloc(ctx, sretPtr, 16)
		return nil, fmt.Errorf("iscc_decode: read sret: out of bounds at %d", sretPtr)
	}

	// Parse struct fields.
	isOK := raw[0] != 0
	if !isOK {
		errMsg := rt.lastError(ctx)
		_ = rt.dealloc(ctx, sretPtr, 16)
		return nil, fmt.Errorf("iscc_decode failed: %s", errMsg)
	}

	maintype := raw[1]
	subtype := raw[2]
	version := raw[3]
	length := raw[4]

	// Digest: IsccByteBuffer at offset 8 (data ptr at 8, len at 12).
	dataPtr := binary.LittleEndian.Uint32(raw[8:12])
	dataLen := binary.LittleEndian.Uint32(raw[12:16])

	// Copy digest bytes from WASM memory to Go.
	var digest []byte
	if dataLen > 0 {
		digestRaw, ok := rt.mod.Memory().Read(dataPtr, dataLen)
		if !ok {
			// Free via iscc_free_decode_result then dealloc sret.
			freeFn := rt.mod.ExportedFunction("iscc_free_decode_result")
			_, _ = freeFn.Call(ctx, uint64(sretPtr))
			_ = rt.dealloc(ctx, sretPtr, 16)
			return nil, fmt.Errorf("iscc_decode: read digest: out of bounds (ptr=%d, len=%d)", dataPtr, dataLen)
		}
		digest = make([]byte, dataLen)
		copy(digest, digestRaw)
	} else {
		digest = []byte{}
	}

	// Free the decode result (frees digest buffer) then dealloc sret.
	freeFn := rt.mod.ExportedFunction("iscc_free_decode_result")
	_, _ = freeFn.Call(ctx, uint64(sretPtr))
	_ = rt.dealloc(ctx, sretPtr, 16)

	return &DecodeResult{
		Maintype: maintype,
		Subtype:  subtype,
		Version:  version,
		Length:   length,
		Digest:   digest,
	}, nil
}

// ── String array helpers ────────────────────────────────────────────────────

// readStringArray reads a null-terminated array of C string pointers from WASM memory.
// In WASM32, pointers are 4-byte little-endian uint32 values.
func (rt *Runtime) readStringArray(ctx context.Context, ptr uint32) ([]string, error) {
	mem := rt.mod.Memory()
	var result []string
	for offset := ptr; ; offset += 4 {
		raw, ok := mem.Read(offset, 4)
		if !ok {
			return nil, fmt.Errorf("iscc: read string array: out of bounds at offset %d", offset)
		}
		strPtr := binary.LittleEndian.Uint32(raw)
		if strPtr == 0 {
			break
		}
		s, err := rt.readString(ctx, strPtr)
		if err != nil {
			return nil, err
		}
		result = append(result, s)
	}
	return result, nil
}

// freeStringArray calls iscc_free_string_array(ptr) to free the entire array
// (strings + outer pointer array).
func (rt *Runtime) freeStringArray(ctx context.Context, ptr uint32) error {
	fn := rt.mod.ExportedFunction("iscc_free_string_array")
	_, err := fn.Call(ctx, uint64(ptr))
	if err != nil {
		return fmt.Errorf("iscc_free_string_array: %w", err)
	}
	return nil
}

// callStringArrayResult handles the string-array result pattern:
// check NULL (ptr==0) → readStringArray → freeStringArray → return.
func (rt *Runtime) callStringArrayResult(ctx context.Context, fnName string, results []uint64) ([]string, error) {
	resultPtr := uint32(results[0])
	if resultPtr == 0 {
		return nil, fmt.Errorf("%s failed: %s", fnName, rt.lastError(ctx))
	}
	arr, err := rt.readStringArray(ctx, resultPtr)
	if err != nil {
		return nil, err
	}
	_ = rt.freeStringArray(ctx, resultPtr)
	return arr, nil
}

// ── String array functions ──────────────────────────────────────────────────

// SlidingWindow generates overlapping substrings of width Unicode characters,
// advancing by one character at a time. Width must be >= 2.
func (rt *Runtime) SlidingWindow(ctx context.Context, seq string, width uint32) ([]string, error) {
	seqPtr, seqSize, err := rt.writeString(ctx, seq)
	if err != nil {
		return nil, err
	}
	defer func() { _ = rt.dealloc(ctx, seqPtr, seqSize) }()

	fn := rt.mod.ExportedFunction("iscc_sliding_window")
	results, err := fn.Call(ctx, uint64(seqPtr), uint64(width))
	if err != nil {
		return nil, fmt.Errorf("iscc_sliding_window: %w", err)
	}
	return rt.callStringArrayResult(ctx, "iscc_sliding_window", results)
}

// IsccDecompose decomposes a composite ISCC-CODE into individual ISCC-UNITs.
// Accepts a normalized ISCC-CODE or concatenated ISCC-UNIT sequence.
// The optional "ISCC:" prefix is stripped before decoding.
func (rt *Runtime) IsccDecompose(ctx context.Context, isccCode string) ([]string, error) {
	codePtr, codeSize, err := rt.writeString(ctx, isccCode)
	if err != nil {
		return nil, err
	}
	defer func() { _ = rt.dealloc(ctx, codePtr, codeSize) }()

	fn := rt.mod.ExportedFunction("iscc_decompose")
	results, err := fn.Call(ctx, uint64(codePtr))
	if err != nil {
		return nil, fmt.Errorf("iscc_decompose: %w", err)
	}
	return rt.callStringArrayResult(ctx, "iscc_decompose", results)
}

// ── Byte buffer helpers ─────────────────────────────────────────────────────

// readByteBuffer reads the two i32 fields of IsccByteBuffer from WASM memory at sretPtr.
// Returns (dataPtr, dataLen). If dataPtr is 0 (null = error), returns an error from lastError.
func (rt *Runtime) readByteBuffer(ctx context.Context, sretPtr uint32) (uint32, uint32, error) {
	raw, ok := rt.mod.Memory().Read(sretPtr, 8)
	if !ok {
		return 0, 0, fmt.Errorf("iscc: read byte buffer: out of bounds at %d", sretPtr)
	}
	dataPtr := binary.LittleEndian.Uint32(raw[0:4])
	dataLen := binary.LittleEndian.Uint32(raw[4:8])
	if dataPtr == 0 {
		return 0, 0, fmt.Errorf("iscc: byte buffer null: %s", rt.lastError(ctx))
	}
	return dataPtr, dataLen, nil
}

// freeByteBuffer calls iscc_free_byte_buffer passing the struct by pointer.
// The struct (data_ptr, len) must already be written at structPtr in WASM memory.
// No-op if dataPtr is 0.
func (rt *Runtime) freeByteBuffer(ctx context.Context, structPtr uint32) error {
	fn := rt.mod.ExportedFunction("iscc_free_byte_buffer")
	_, err := fn.Call(ctx, uint64(structPtr))
	if err != nil {
		return fmt.Errorf("iscc_free_byte_buffer: %w", err)
	}
	return nil
}

// callByteBufferResult orchestrates: read struct from sretPtr → check null → copy bytes
// from WASM → free byte buffer → dealloc sret → return Go []byte.
func (rt *Runtime) callByteBufferResult(ctx context.Context, fnName string, sretPtr uint32) ([]byte, error) {
	dataPtr, dataLen, err := rt.readByteBuffer(ctx, sretPtr)
	if err != nil {
		_ = rt.dealloc(ctx, sretPtr, 8)
		return nil, fmt.Errorf("%s failed: %w", fnName, err)
	}
	// Copy bytes from WASM memory to Go.
	var result []byte
	if dataLen > 0 {
		raw, ok := rt.mod.Memory().Read(dataPtr, dataLen)
		if !ok {
			_ = rt.dealloc(ctx, sretPtr, 8)
			return nil, fmt.Errorf("%s: read data: out of bounds (ptr=%d, len=%d)", fnName, dataPtr, dataLen)
		}
		result = make([]byte, dataLen)
		copy(result, raw)
	} else {
		result = []byte{}
	}
	// Free the byte buffer data (struct is still at sretPtr).
	_ = rt.freeByteBuffer(ctx, sretPtr)
	// Free the sret allocation.
	_ = rt.dealloc(ctx, sretPtr, 8)
	return result, nil
}

// readByteBufferArray reads the two i32 fields of IsccByteBufferArray from WASM memory.
// Returns (buffersPtr, count). If buffersPtr is 0 (null = error), returns an error.
func (rt *Runtime) readByteBufferArray(ctx context.Context, sretPtr uint32) (uint32, uint32, error) {
	raw, ok := rt.mod.Memory().Read(sretPtr, 8)
	if !ok {
		return 0, 0, fmt.Errorf("iscc: read byte buffer array: out of bounds at %d", sretPtr)
	}
	buffersPtr := binary.LittleEndian.Uint32(raw[0:4])
	count := binary.LittleEndian.Uint32(raw[4:8])
	if buffersPtr == 0 {
		return 0, 0, fmt.Errorf("iscc: byte buffer array null: %s", rt.lastError(ctx))
	}
	return buffersPtr, count, nil
}

// freeByteBufferArray calls iscc_free_byte_buffer_array passing the struct by pointer.
// The struct (buffers_ptr, count) must already be written at structPtr in WASM memory.
func (rt *Runtime) freeByteBufferArray(ctx context.Context, structPtr uint32) error {
	fn := rt.mod.ExportedFunction("iscc_free_byte_buffer_array")
	_, err := fn.Call(ctx, uint64(structPtr))
	if err != nil {
		return fmt.Errorf("iscc_free_byte_buffer_array: %w", err)
	}
	return nil
}

// writeU32Slice allocates WASM memory and writes uint32 values in little-endian format.
// For empty slices, allocates 4 bytes to ensure proper u32 alignment.
func (rt *Runtime) writeU32Slice(ctx context.Context, values []uint32) (ptr, allocSize, count uint32, err error) {
	count = uint32(len(values))
	byteSize := count * 4
	allocSize = byteSize
	if allocSize == 0 {
		allocSize = 4
	}
	ptr, err = rt.alloc(ctx, allocSize)
	if err != nil {
		return 0, 0, 0, err
	}
	if byteSize > 0 {
		buf := make([]byte, byteSize)
		for i, v := range values {
			binary.LittleEndian.PutUint32(buf[i*4:], v)
		}
		if !rt.mod.Memory().Write(ptr, buf) {
			return 0, 0, 0, fmt.Errorf("iscc: write u32 slice: out of bounds (ptr=%d, size=%d)", ptr, byteSize)
		}
	}
	return ptr, allocSize, count, nil
}

// writeByteArrayOfArrays writes an array of byte slices to WASM memory for functions
// that take (*const *const u8, *const usize, count). Returns pointers to the data-pointers
// array and data-lengths array, the count, and a cleanup function.
func (rt *Runtime) writeByteArrayOfArrays(ctx context.Context, digests [][]byte) (dataPtrsPtr, dataLensPtr, count uint32, cleanup func(), err error) {
	count = uint32(len(digests))
	if count == 0 {
		return 0, 0, 0, func() {}, nil
	}

	type allocEntry struct {
		ptr  uint32
		size uint32
	}
	var allocs []allocEntry

	doCleanup := func() {
		for _, a := range allocs {
			_ = rt.dealloc(ctx, a.ptr, a.size)
		}
	}

	// Write each digest and collect pointers and lengths.
	ptrs := make([]uint32, count)
	lens := make([]uint32, count)
	for i, d := range digests {
		dPtr, dSize, werr := rt.writeBytes(ctx, d)
		if werr != nil {
			doCleanup()
			return 0, 0, 0, nil, werr
		}
		allocs = append(allocs, allocEntry{dPtr, dSize})
		ptrs[i] = dPtr
		lens[i] = uint32(len(d))
	}

	// Build data-pointers array (uint32 per pointer).
	ptrArrSize := count * 4
	ptrArrPtr, aerr := rt.alloc(ctx, ptrArrSize)
	if aerr != nil {
		doCleanup()
		return 0, 0, 0, nil, aerr
	}
	allocs = append(allocs, allocEntry{ptrArrPtr, ptrArrSize})

	ptrBuf := make([]byte, ptrArrSize)
	for i, p := range ptrs {
		binary.LittleEndian.PutUint32(ptrBuf[i*4:], p)
	}
	if !rt.mod.Memory().Write(ptrArrPtr, ptrBuf) {
		doCleanup()
		return 0, 0, 0, nil, fmt.Errorf("iscc: write digest pointers: out of bounds")
	}

	// Build data-lengths array (uint32 per length, WASM usize is 4 bytes).
	lenArrSize := count * 4
	lenArrPtr, lerr := rt.alloc(ctx, lenArrSize)
	if lerr != nil {
		doCleanup()
		return 0, 0, 0, nil, lerr
	}
	allocs = append(allocs, allocEntry{lenArrPtr, lenArrSize})

	lenBuf := make([]byte, lenArrSize)
	for i, l := range lens {
		binary.LittleEndian.PutUint32(lenBuf[i*4:], l)
	}
	if !rt.mod.Memory().Write(lenArrPtr, lenBuf) {
		doCleanup()
		return 0, 0, 0, nil, fmt.Errorf("iscc: write digest lengths: out of bounds")
	}

	return ptrArrPtr, lenArrPtr, count, doCleanup, nil
}

// ── Byte buffer functions ───────────────────────────────────────────────────

// AlgSimhash computes a SimHash from a set of equal-length byte digests.
// Output length matches input digest length.
func (rt *Runtime) AlgSimhash(ctx context.Context, digests [][]byte) ([]byte, error) {
	dpPtr, dlPtr, cnt, cleanup, err := rt.writeByteArrayOfArrays(ctx, digests)
	if err != nil {
		return nil, err
	}
	defer cleanup()

	// Allocate sret for IsccByteBuffer (8 bytes).
	sretPtr, err := rt.alloc(ctx, 8)
	if err != nil {
		return nil, err
	}

	fn := rt.mod.ExportedFunction("iscc_alg_simhash")
	_, err = fn.Call(ctx, uint64(sretPtr), uint64(dpPtr), uint64(dlPtr), uint64(cnt))
	if err != nil {
		_ = rt.dealloc(ctx, sretPtr, 8)
		return nil, fmt.Errorf("iscc_alg_simhash: %w", err)
	}
	return rt.callByteBufferResult(ctx, "iscc_alg_simhash", sretPtr)
}

// AlgMinhash256 computes a 256-bit (32-byte) MinHash digest from uint32 features.
func (rt *Runtime) AlgMinhash256(ctx context.Context, features []uint32) ([]byte, error) {
	fPtr, fAllocSize, fCount, err := rt.writeU32Slice(ctx, features)
	if err != nil {
		return nil, err
	}
	defer func() { _ = rt.dealloc(ctx, fPtr, fAllocSize) }()

	sretPtr, err := rt.alloc(ctx, 8)
	if err != nil {
		return nil, err
	}

	fn := rt.mod.ExportedFunction("iscc_alg_minhash_256")
	_, err = fn.Call(ctx, uint64(sretPtr), uint64(fPtr), uint64(fCount))
	if err != nil {
		_ = rt.dealloc(ctx, sretPtr, 8)
		return nil, fmt.Errorf("iscc_alg_minhash_256: %w", err)
	}
	return rt.callByteBufferResult(ctx, "iscc_alg_minhash_256", sretPtr)
}

// AlgCdcChunks splits data into content-defined chunks using gear rolling hash.
// When utf32 is true, cut points align to 4-byte boundaries.
// Returns at least one chunk (empty bytes for empty input).
func (rt *Runtime) AlgCdcChunks(ctx context.Context, data []byte, utf32 bool, avgChunkSize uint32) ([][]byte, error) {
	dataPtr, dataSize, err := rt.writeBytes(ctx, data)
	if err != nil {
		return nil, err
	}
	defer func() { _ = rt.dealloc(ctx, dataPtr, dataSize) }()

	// Allocate sret for IsccByteBufferArray (8 bytes).
	sretPtr, err := rt.alloc(ctx, 8)
	if err != nil {
		return nil, err
	}

	var utf32Arg uint64
	if utf32 {
		utf32Arg = 1
	}

	fn := rt.mod.ExportedFunction("iscc_alg_cdc_chunks")
	_, err = fn.Call(ctx, uint64(sretPtr), uint64(dataPtr), uint64(dataSize), utf32Arg, uint64(avgChunkSize))
	if err != nil {
		_ = rt.dealloc(ctx, sretPtr, 8)
		return nil, fmt.Errorf("iscc_alg_cdc_chunks: %w", err)
	}

	buffersPtr, count, err := rt.readByteBufferArray(ctx, sretPtr)
	if err != nil {
		_ = rt.dealloc(ctx, sretPtr, 8)
		return nil, fmt.Errorf("iscc_alg_cdc_chunks failed: %w", err)
	}

	// Read each IsccByteBuffer (8 bytes per struct) from the array.
	chunks := make([][]byte, count)
	for i := uint32(0); i < count; i++ {
		bufOffset := buffersPtr + i*8
		raw, ok := rt.mod.Memory().Read(bufOffset, 8)
		if !ok {
			_ = rt.freeByteBufferArray(ctx, sretPtr)
			_ = rt.dealloc(ctx, sretPtr, 8)
			return nil, fmt.Errorf("iscc_alg_cdc_chunks: read chunk %d: out of bounds", i)
		}
		chunkPtr := binary.LittleEndian.Uint32(raw[0:4])
		chunkLen := binary.LittleEndian.Uint32(raw[4:8])
		if chunkLen > 0 {
			chunkData, ok := rt.mod.Memory().Read(chunkPtr, chunkLen)
			if !ok {
				_ = rt.freeByteBufferArray(ctx, sretPtr)
				_ = rt.dealloc(ctx, sretPtr, 8)
				return nil, fmt.Errorf("iscc_alg_cdc_chunks: read chunk %d data: out of bounds", i)
			}
			chunks[i] = make([]byte, chunkLen)
			copy(chunks[i], chunkData)
		} else {
			chunks[i] = []byte{}
		}
	}

	// Free all buffers and the array.
	_ = rt.freeByteBufferArray(ctx, sretPtr)
	_ = rt.dealloc(ctx, sretPtr, 8)
	return chunks, nil
}

// SoftHashVideoV0 computes a similarity-preserving hash from video frame signatures.
// Returns raw bytes of length bits/8. Errors if frameSigs is empty.
func (rt *Runtime) SoftHashVideoV0(ctx context.Context, frameSigs [][]int32, bits uint32) ([]byte, error) {
	fpPtr, flPtr, nFrames, cleanup, err := rt.writeI32ArrayOfArrays(ctx, frameSigs)
	if err != nil {
		return nil, err
	}
	defer cleanup()

	sretPtr, err := rt.alloc(ctx, 8)
	if err != nil {
		return nil, err
	}

	fn := rt.mod.ExportedFunction("iscc_soft_hash_video_v0")
	_, err = fn.Call(ctx, uint64(sretPtr), uint64(fpPtr), uint64(flPtr), uint64(nFrames), uint64(bits))
	if err != nil {
		_ = rt.dealloc(ctx, sretPtr, 8)
		return nil, fmt.Errorf("iscc_soft_hash_video_v0: %w", err)
	}
	return rt.callByteBufferResult(ctx, "iscc_soft_hash_video_v0", sretPtr)
}

// ── Streaming hashers ────────────────────────────────────────────────────────

// WasmDataHasher provides streaming Data-Code generation via the WASM FFI.
// Create with Runtime.NewDataHasher, feed data with Update, and retrieve the
// ISCC code with Finalize. Close releases the WASM-side memory.
type WasmDataHasher struct {
	rt  *Runtime
	ptr uint32 // opaque WASM-side FfiDataHasher pointer
}

// WasmInstanceHasher provides streaming Instance-Code generation via the WASM FFI.
// Create with Runtime.NewInstanceHasher, feed data with Update, and retrieve the
// ISCC code with Finalize. Close releases the WASM-side memory.
type WasmInstanceHasher struct {
	rt  *Runtime
	ptr uint32 // opaque WASM-side FfiInstanceHasher pointer
}

// NewDataHasher creates a streaming Data-Code hasher.
// The caller must call Close when done, even after Finalize.
func (rt *Runtime) NewDataHasher(ctx context.Context) (*WasmDataHasher, error) {
	fn := rt.mod.ExportedFunction("iscc_data_hasher_new")
	results, err := fn.Call(ctx)
	if err != nil {
		return nil, fmt.Errorf("iscc_data_hasher_new: %w", err)
	}
	ptr := uint32(results[0])
	if ptr == 0 {
		return nil, fmt.Errorf("iscc_data_hasher_new: returned NULL: %s", rt.lastError(ctx))
	}
	return &WasmDataHasher{rt: rt, ptr: ptr}, nil
}

// NewInstanceHasher creates a streaming Instance-Code hasher.
// The caller must call Close when done, even after Finalize.
func (rt *Runtime) NewInstanceHasher(ctx context.Context) (*WasmInstanceHasher, error) {
	fn := rt.mod.ExportedFunction("iscc_instance_hasher_new")
	results, err := fn.Call(ctx)
	if err != nil {
		return nil, fmt.Errorf("iscc_instance_hasher_new: %w", err)
	}
	ptr := uint32(results[0])
	if ptr == 0 {
		return nil, fmt.Errorf("iscc_instance_hasher_new: returned NULL: %s", rt.lastError(ctx))
	}
	return &WasmInstanceHasher{rt: rt, ptr: ptr}, nil
}

// Update feeds data into the DataHasher.
// Can be called multiple times before Finalize. Returns an error if the
// hasher has already been finalized.
func (h *WasmDataHasher) Update(ctx context.Context, data []byte) error {
	dataPtr, dataSize, err := h.rt.writeBytes(ctx, data)
	if err != nil {
		return err
	}
	defer func() { _ = h.rt.dealloc(ctx, dataPtr, dataSize) }()

	fn := h.rt.mod.ExportedFunction("iscc_data_hasher_update")
	results, err := fn.Call(ctx, uint64(h.ptr), uint64(dataPtr), uint64(dataSize))
	if err != nil {
		return fmt.Errorf("iscc_data_hasher_update: %w", err)
	}
	if results[0] == 0 {
		return fmt.Errorf("iscc_data_hasher_update: %s", h.rt.lastError(ctx))
	}
	return nil
}

// UpdateFrom reads all data from r and feeds it into the hasher in chunks.
// Uses 64 KiB internal buffer. Returns any read or update error.
func (h *WasmDataHasher) UpdateFrom(ctx context.Context, r io.Reader) error {
	buf := make([]byte, 64*1024)
	for {
		n, err := r.Read(buf)
		if n > 0 {
			if updateErr := h.Update(ctx, buf[:n]); updateErr != nil {
				return updateErr
			}
		}
		if err == io.EOF {
			return nil
		}
		if err != nil {
			return fmt.Errorf("iscc: read: %w", err)
		}
	}
}

// Finalize completes the hashing and returns the ISCC Data-Code string.
// After Finalize, Update and Finalize will return errors. The caller must
// still call Close to free WASM-side memory.
func (h *WasmDataHasher) Finalize(ctx context.Context, bits uint32) (string, error) {
	fn := h.rt.mod.ExportedFunction("iscc_data_hasher_finalize")
	results, err := fn.Call(ctx, uint64(h.ptr), uint64(bits))
	if err != nil {
		return "", fmt.Errorf("iscc_data_hasher_finalize: %w", err)
	}
	return h.rt.callStringResult(ctx, "iscc_data_hasher_finalize", results)
}

// Close releases the WASM-side DataHasher memory.
// Safe to call multiple times. Sets the internal pointer to 0 to prevent
// double-free.
func (h *WasmDataHasher) Close(ctx context.Context) error {
	if h.ptr == 0 {
		return nil
	}
	fn := h.rt.mod.ExportedFunction("iscc_data_hasher_free")
	_, err := fn.Call(ctx, uint64(h.ptr))
	h.ptr = 0
	if err != nil {
		return fmt.Errorf("iscc_data_hasher_free: %w", err)
	}
	return nil
}

// Update feeds data into the InstanceHasher.
// Can be called multiple times before Finalize. Returns an error if the
// hasher has already been finalized.
func (h *WasmInstanceHasher) Update(ctx context.Context, data []byte) error {
	dataPtr, dataSize, err := h.rt.writeBytes(ctx, data)
	if err != nil {
		return err
	}
	defer func() { _ = h.rt.dealloc(ctx, dataPtr, dataSize) }()

	fn := h.rt.mod.ExportedFunction("iscc_instance_hasher_update")
	results, err := fn.Call(ctx, uint64(h.ptr), uint64(dataPtr), uint64(dataSize))
	if err != nil {
		return fmt.Errorf("iscc_instance_hasher_update: %w", err)
	}
	if results[0] == 0 {
		return fmt.Errorf("iscc_instance_hasher_update: %s", h.rt.lastError(ctx))
	}
	return nil
}

// UpdateFrom reads all data from r and feeds it into the hasher in chunks.
// Uses 64 KiB internal buffer. Returns any read or update error.
func (h *WasmInstanceHasher) UpdateFrom(ctx context.Context, r io.Reader) error {
	buf := make([]byte, 64*1024)
	for {
		n, err := r.Read(buf)
		if n > 0 {
			if updateErr := h.Update(ctx, buf[:n]); updateErr != nil {
				return updateErr
			}
		}
		if err == io.EOF {
			return nil
		}
		if err != nil {
			return fmt.Errorf("iscc: read: %w", err)
		}
	}
}

// Finalize completes the hashing and returns the ISCC Instance-Code string.
// After Finalize, Update and Finalize will return errors. The caller must
// still call Close to free WASM-side memory.
func (h *WasmInstanceHasher) Finalize(ctx context.Context, bits uint32) (string, error) {
	fn := h.rt.mod.ExportedFunction("iscc_instance_hasher_finalize")
	results, err := fn.Call(ctx, uint64(h.ptr), uint64(bits))
	if err != nil {
		return "", fmt.Errorf("iscc_instance_hasher_finalize: %w", err)
	}
	return h.rt.callStringResult(ctx, "iscc_instance_hasher_finalize", results)
}

// Close releases the WASM-side InstanceHasher memory.
// Safe to call multiple times. Sets the internal pointer to 0 to prevent
// double-free.
func (h *WasmInstanceHasher) Close(ctx context.Context) error {
	if h.ptr == 0 {
		return nil
	}
	fn := h.rt.mod.ExportedFunction("iscc_instance_hasher_free")
	_, err := fn.Call(ctx, uint64(h.ptr))
	h.ptr = 0
	if err != nil {
		return fmt.Errorf("iscc_instance_hasher_free: %w", err)
	}
	return nil
}

// GenIsccCodeV0 generates a composite ISCC-CODE from individual unit codes.
func (rt *Runtime) GenIsccCodeV0(ctx context.Context, codes []string) (string, error) {
	codesPtr, codesCount, cleanup, err := rt.writeStringArray(ctx, codes)
	if err != nil {
		return "", err
	}
	defer cleanup()

	fn := rt.mod.ExportedFunction("iscc_gen_iscc_code_v0")
	// wide=false (0) — all conformance vectors use standard width.
	results, err := fn.Call(ctx, uint64(codesPtr), uint64(codesCount), 0)
	if err != nil {
		return "", fmt.Errorf("iscc_gen_iscc_code_v0: %w", err)
	}
	return rt.callStringResult(ctx, "iscc_gen_iscc_code_v0", results)
}
