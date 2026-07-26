// Unicode 16.0.0 boundary conformance tests for TextClean and TextCollapse,
// run against the vendored copy of the canonical fixture
// crates/iscc-lib/tests/unicode_boundary.json. Cases the pure-Go package
// cannot pass on Go 1.26's Unicode 15.0 tables are skipped per the 2026-07-26
// ruling (decisions.md): wait for go1.27 instead of vendoring the 15.0-to-16.0
// assigned delta.
package iscc

import (
	_ "embed"
	"encoding/json"
	"strings"
	"testing"
)

//go:embed testdata/unicode_boundary.json
var unicodeBoundaryData string

// unicodeBoundarySkips lists the boundary cases that depend on Unicode 16.0
// assignment data the Go stdlib does not ship yet. Keys are "<section>/<case>".
var unicodeBoundarySkips = map[string]string{
	"text_clean/test_0000_u1fae9_assigned_so_retained": "go1.27 (~Aug 2026) brings Unicode 16/17 tables to the stdlib and x/text; " +
		"Go 1.26 classifies U+1FAE9 as unassigned and drops it. " +
		"Ruled 2026-07-26 (decisions.md): skip, do not vendor the 15.0-to-16.0 delta.",
	"text_clean/test_0001_u113c5_assigned_mc_retained": "go1.27 (~Aug 2026) brings Unicode 16/17 tables to the stdlib and x/text; " +
		"Go 1.26 classifies U+113C5 as unassigned and drops it. " +
		"Ruled 2026-07-26 (decisions.md): skip, do not vendor the 15.0-to-16.0 delta.",
	"text_collapse/test_0000_u1fae9_assigned_so_retained": "go1.27 (~Aug 2026) brings Unicode 16/17 tables to the stdlib and x/text; " +
		"Go 1.26 classifies U+1FAE9 as unassigned and drops it. " +
		"Ruled 2026-07-26 (decisions.md): skip, do not vendor the 15.0-to-16.0 delta.",
}

// runUnicodeBoundarySection runs every case of the named fixture section
// through fn, honouring the ruled skip list.
func runUnicodeBoundarySection(t *testing.T, section string, fn func(string) string) {
	t.Helper()
	data, err := parseConformanceData([]byte(unicodeBoundaryData))
	if err != nil {
		t.Fatalf("parse unicode_boundary.json: %v", err)
	}
	cases, ok := data[section]
	if !ok {
		t.Fatalf("%s section not found in unicode_boundary.json", section)
	}
	for name, vec := range cases {
		t.Run(name, func(t *testing.T) {
			if reason, skip := unicodeBoundarySkips[section+"/"+name]; skip {
				t.Skipf("%s", reason)
			}
			var input string
			if err := json.Unmarshal(vec.Inputs[0], &input); err != nil {
				t.Fatalf("parse input: %v", err)
			}
			want, _ := vec.Outputs["result"].(string)
			if got := fn(input); got != want {
				t.Errorf("got %q, want %q", got, want)
			}
		})
	}
}

func TestPureGoUnicodeBoundaryTextClean(t *testing.T) {
	runUnicodeBoundarySection(t, "text_clean", TextClean)
}

func TestPureGoUnicodeBoundaryTextCollapse(t *testing.T) {
	runUnicodeBoundarySection(t, "text_collapse", TextCollapse)
}

// TestPureGoUnicodeBoundaryFixtureMetadata guards against a truncated or
// renamed fixture silently degrading to zero cases, and against a stale skip
// entry masking a renamed vector.
func TestPureGoUnicodeBoundaryFixtureMetadata(t *testing.T) {
	var meta struct {
		Metadata struct {
			UnicodeDataVersion string `json:"unicode_data_version"`
		} `json:"_metadata"`
	}
	if err := json.Unmarshal([]byte(unicodeBoundaryData), &meta); err != nil {
		t.Fatalf("parse unicode_boundary.json: %v", err)
	}
	if meta.Metadata.UnicodeDataVersion != "16.0.0" {
		t.Errorf("unicode_data_version: got %q, want %q", meta.Metadata.UnicodeDataVersion, "16.0.0")
	}

	data, err := parseConformanceData([]byte(unicodeBoundaryData))
	if err != nil {
		t.Fatalf("parse unicode_boundary.json: %v", err)
	}
	if got := len(data["text_clean"]); got != 7 {
		t.Errorf("text_clean case count: got %d, want 7", got)
	}
	if got := len(data["text_collapse"]); got != 5 {
		t.Errorf("text_collapse case count: got %d, want 5", got)
	}

	for key := range unicodeBoundarySkips {
		section, name, ok := strings.Cut(key, "/")
		if !ok {
			t.Errorf("skip key %q is not of the form <section>/<case>", key)
			continue
		}
		if _, exists := data[section][name]; !exists {
			t.Errorf("skip key %q names a case missing from the fixture", key)
		}
	}
}
