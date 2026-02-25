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

//go:embed iscc_ffi.wasm
var wasmModule []byte

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
