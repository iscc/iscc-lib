// Conformance tests for GenInstanceCodeV0 against data.json vectors.
package iscc

import (
	"encoding/json"
	"strings"
	"testing"
)

func TestPureGoGenInstanceCodeV0(t *testing.T) {
	for name, vec := range conformanceSection(t, "gen_instance_code_v0") {
		t.Run(name, func(t *testing.T) {
			inputBytes, bits, err := streamInputs(vec)
			if err != nil {
				t.Fatalf("parse inputs: %v", err)
			}
			wantISCC, err := expectedISCC(vec)
			if err != nil {
				t.Fatalf("parse iscc output: %v", err)
			}
			var want struct {
				Datahash string `json:"datahash"`
				Filesize uint64 `json:"filesize"`
			}
			if err := json.Unmarshal(vec.Outputs, &want); err != nil {
				t.Fatalf("parse outputs: %v", err)
			}

			result, err := GenInstanceCodeV0(inputBytes, bits)
			if err != nil {
				t.Fatalf("GenInstanceCodeV0: %v", err)
			}

			if result.Iscc != wantISCC {
				t.Errorf("iscc: got %q, want %q", result.Iscc, wantISCC)
			}
			if result.Datahash != want.Datahash {
				t.Errorf("datahash: got %q, want %q", result.Datahash, want.Datahash)
			}
			if result.Filesize != want.Filesize {
				t.Errorf("filesize: got %d, want %d", result.Filesize, want.Filesize)
			}
		})
	}
}

// finalizedInstanceHasher returns an InstanceHasher whose Finalize already succeeded.
func finalizedInstanceHasher(t *testing.T) *InstanceHasher {
	t.Helper()
	h := NewInstanceHasher()
	if err := h.Push([]byte("hello world")); err != nil {
		t.Fatalf("Push: %v", err)
	}
	if _, err := h.Finalize(64); err != nil {
		t.Fatalf("Finalize: %v", err)
	}
	return h
}

func TestInstanceHasherPushAfterFinalize(t *testing.T) {
	h := finalizedInstanceHasher(t)
	err := h.Push([]byte("more"))
	if err == nil || !strings.Contains(err.Error(), "already finalized") {
		t.Fatalf("Push after Finalize: got %v, want already finalized error", err)
	}
}

func TestInstanceHasherFinalizeTwice(t *testing.T) {
	h := finalizedInstanceHasher(t)
	result, err := h.Finalize(64)
	if err == nil || !strings.Contains(err.Error(), "already finalized") {
		t.Fatalf("second Finalize: got %v, want already finalized error", err)
	}
	if result != nil {
		t.Fatalf("second Finalize: got result %+v, want nil", result)
	}
}

func TestInstanceHasherFailedFinalizeIsFinal(t *testing.T) {
	h := NewInstanceHasher()
	if err := h.Push([]byte("hello world")); err != nil {
		t.Fatalf("Push: %v", err)
	}
	if _, err := h.Finalize(63); err == nil {
		t.Fatal("Finalize(63): want invalid bits error, got nil")
	}
	result, err := h.Finalize(64)
	if err == nil || !strings.Contains(err.Error(), "already finalized") {
		t.Fatalf("Finalize after failed Finalize: got %v, want already finalized error", err)
	}
	if result != nil {
		t.Fatalf("Finalize after failed Finalize: got result %+v, want nil", result)
	}
}

func TestInstanceHasherPushSplitMatchesOneShot(t *testing.T) {
	a := []byte(strings.Repeat("ISCC streaming ", 500))
	b := []byte(strings.Repeat("blake3 instance ", 700))
	h := NewInstanceHasher()
	if err := h.Push(a); err != nil {
		t.Fatalf("Push(a): %v", err)
	}
	if err := h.Push(b); err != nil {
		t.Fatalf("Push(b): %v", err)
	}
	got, err := h.Finalize(64)
	if err != nil {
		t.Fatalf("Finalize: %v", err)
	}
	want, err := GenInstanceCodeV0(append(append([]byte{}, a...), b...), 64)
	if err != nil {
		t.Fatalf("GenInstanceCodeV0: %v", err)
	}
	if *got != *want {
		t.Fatalf("Push(a); Push(b) = %+v, GenInstanceCodeV0(a+b) = %+v", got, want)
	}
}
