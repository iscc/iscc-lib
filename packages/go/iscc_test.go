// Tests for the iscc Go package WASM bridge.
//
// Requires a prebuilt iscc_ffi.wasm binary in the package directory.
// Build with: cargo build -p iscc-ffi --target wasm32-wasip1
package iscc

import (
	"context"
	"os"
	"testing"
)

// TestMain checks for the WASM binary before running any tests.
// If missing, all tests are skipped with a clear message.
func TestMain(m *testing.M) {
	if _, err := os.Stat("iscc_ffi.wasm"); os.IsNotExist(err) {
		println("SKIP: iscc_ffi.wasm not found. Build with: cargo build -p iscc-ffi --target wasm32-wasip1")
		println("Then copy: cp target/wasm32-wasip1/debug/iscc_ffi.wasm packages/go/")
		os.Exit(0)
	}
	os.Exit(m.Run())
}

// newTestRuntime creates a Runtime for testing and registers cleanup.
func newTestRuntime(t *testing.T) *Runtime {
	t.Helper()
	ctx := context.Background()
	rt, err := NewRuntime(ctx)
	if err != nil {
		t.Fatalf("NewRuntime: %v", err)
	}
	t.Cleanup(func() {
		if err := rt.Close(ctx); err != nil {
			t.Errorf("Runtime.Close: %v", err)
		}
	})
	return rt
}

// TestRuntimeInit verifies that a Runtime can be created and closed.
func TestRuntimeInit(t *testing.T) {
	ctx := context.Background()
	rt, err := NewRuntime(ctx)
	if err != nil {
		t.Fatalf("NewRuntime failed: %v", err)
	}
	if err := rt.Close(ctx); err != nil {
		t.Fatalf("Close failed: %v", err)
	}
}

// TestConformanceSelftest calls iscc_conformance_selftest() and asserts it returns true.
// This proves the full Rust ISCC core runs correctly inside the WASM module via the Go bridge.
func TestConformanceSelftest(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)

	passed, err := rt.ConformanceSelftest(ctx)
	if err != nil {
		t.Fatalf("ConformanceSelftest error: %v", err)
	}
	if !passed {
		t.Fatal("ConformanceSelftest returned false — conformance tests failed inside WASM")
	}
}

// TestAllocDealloc allocates memory, writes bytes, reads them back, and deallocates.
func TestAllocDealloc(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)

	// Allocate 4 bytes.
	ptr, err := rt.alloc(ctx, 4)
	if err != nil {
		t.Fatalf("alloc: %v", err)
	}
	if ptr == 0 {
		t.Fatal("alloc returned null pointer")
	}

	// Write test data.
	data := []byte{0xDE, 0xAD, 0xBE, 0xEF}
	if !rt.mod.Memory().Write(ptr, data) {
		t.Fatal("Memory.Write failed")
	}

	// Read it back.
	got, ok := rt.mod.Memory().Read(ptr, 4)
	if !ok {
		t.Fatal("Memory.Read failed")
	}
	for i, b := range got {
		if b != data[i] {
			t.Fatalf("byte %d: got 0x%02X, want 0x%02X", i, b, data[i])
		}
	}

	// Deallocate.
	if err := rt.dealloc(ctx, ptr, 4); err != nil {
		t.Fatalf("dealloc: %v", err)
	}
}

// TestWriteReadString roundtrips a Unicode string through WASM memory.
func TestWriteReadString(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)

	// Use a Unicode string with multi-byte characters.
	input := "Hello, 世界! 🌍"

	ptr, size, err := rt.writeString(ctx, input)
	if err != nil {
		t.Fatalf("writeString: %v", err)
	}
	defer func() { _ = rt.dealloc(ctx, ptr, size) }()

	got, err := rt.readString(ctx, ptr)
	if err != nil {
		t.Fatalf("readString: %v", err)
	}
	if got != input {
		t.Fatalf("roundtrip mismatch: got %q, want %q", got, input)
	}
}

// TestTextClean calls iscc_text_clean via the Go bridge and verifies string marshaling.
// Tests NFKC normalization (fi ligature → "fi") and leading/trailing whitespace stripping,
// proving the full writeString → call → readString → freeString cycle works.
func TestTextClean(t *testing.T) {
	ctx := context.Background()
	rt := newTestRuntime(t)

	// NFKC normalizes the fi ligature (U+FB01) to "fi", strips leading/trailing whitespace.
	result, err := rt.TextClean(ctx, "  Hel\uFB01 World  ")
	if err != nil {
		t.Fatalf("TextClean error: %v", err)
	}
	expected := "Helfi World"
	if result != expected {
		t.Fatalf("TextClean: got %q, want %q", result, expected)
	}
}
