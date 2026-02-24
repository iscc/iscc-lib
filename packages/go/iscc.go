// Package iscc provides Go bindings for the ISCC (International Standard Content Code)
// library via a WebAssembly module powered by the wazero runtime.
//
// The package embeds a prebuilt WASM binary of iscc-ffi (Rust) and exposes a Runtime
// type with low-level memory helpers for passing strings and byte slices across the
// Go→WASM boundary. Higher-level ISCC function wrappers are added in a subsequent step.
package iscc

import (
	"context"
	_ "embed"
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
