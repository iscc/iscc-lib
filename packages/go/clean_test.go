// Tests for isccClean and its routing through the four Go codec input sites
// (IsccDecompose, IsccDecode, GenIsccCodeV0, GenMixedCodeV0). Each display
// form must produce the same result as the canonical input, mirroring
// iscc_core.codec.iscc_clean (probed on iscc_core 1.5.0 / CPython 3.13).
package iscc

import (
	"reflect"
	"strings"
	"testing"
)

// Canonical inputs from data.json (gen_iscc_code_v0 test_0000_standard and
// gen_mixed_code_v0 test_0000_std_64).
var (
	cleanIsccCode  = "KACYPXW445FTYNJ3CYSXHAFJMA2HUWULUNRFE3BLHRSCXYH2M5AEGQY"
	cleanIsccUnits = []string{
		"AAAYPXW445FTYNJ3", "EAARMJLTQCUWAND2", "GABVVC5DMJJGYKZ4ZBYVNYABFFYXG",
		"IADWIK7A7JTUAQ2D6QARX7OBEIK3OOUAM42LOBLCZ4ZOGDLRHMDL6TQ",
	}
	cleanMixedUnits = []string{
		"EUA6GIKXN42IQV3S", "EIAUKMOUIOYZCKA5", "EQA6JK5IEKO6E732", "EIAU2XRWOT4AKMTZ",
	}
)

// dashed inserts a dash after every 4 characters of code.
func dashed(code string) string {
	var b strings.Builder
	for i, r := range code {
		if i > 0 && i%4 == 0 {
			b.WriteByte('-')
		}
		b.WriteRune(r)
	}
	return b.String()
}

// cleanForms maps a form name to a function rendering a bare code in that form.
var cleanForms = map[string]func(string) string{
	"canonical":        func(c string) string { return "ISCC:" + c },
	"bare":             func(c string) string { return c },
	"dashed":           func(c string) string { return "ISCC:" + dashed(c) },
	"dashed bare":      func(c string) string { return dashed(c) },
	"whitespace":       func(c string) string { return " \t ISCC : " + c + "  " },
	"info separators":  func(c string) string { return "\x1c\x1dISCC:" + c + "\x1e\x1f" },
	"inner separators": func(c string) string { return "ISCC\x1c:\x1f" + c },
	"lowercase":        func(c string) string { return "iscc:" + c },
	"mixed case":       func(c string) string { return "IsCc:" + c },
	"trailing LF":      func(c string) string { return "ISCC:" + c + "\n" },
	"trailing CRLF":    func(c string) string { return "ISCC:" + c + "\r\n" },
}

// mapForm applies form to every code in codes.
func mapForm(form func(string) string, codes []string) []string {
	out := make([]string, len(codes))
	for i, c := range codes {
		out[i] = form(c)
	}
	return out
}

func TestIsccCleanUnit(t *testing.T) {
	cases := []struct{ in, want string }{
		{"ISCC:KACY-PXW4-45FT-YNJ3", "KACYPXW445FTYNJ3"},
		{"iscc:KACYPXW445FTYNJ3", "KACYPXW445FTYNJ3"},
		{"Iscc:KACYPXW445FTYNJ3", "KACYPXW445FTYNJ3"},
		{"  ISCC: KACY-PXW4  ", "KACYPXW4"},
		{"KACY-PXW4-45FT-YNJ3", "KACYPXW445FTYNJ3"},
		{"ISCC:GAAAAAAAAA\n", "GAAAAAAAAA"},
		{"\x1cISCC:GAAAAAAAAA\x1f", "GAAAAAAAAA"},
		{" ISCC : GAAAAAAAAA ", "GAAAAAAAAA"},
		{" GAAAAAAAAA　", "GAAAAAAAAA"},
		// A multibase-led bare string keeps its dashes.
		{"bAAA-BBB", "bAAA-BBB"},
		{"z-abc", "z-abc"},
		{"fAB-CD", "fAB-CD"},
		{"vAB-CD", "vAB-CD"},
		{"uAB-CD", "uAB-CD"},
		// With a scheme, dashes are always stripped.
		{"ISCC:bAAA-BBB", "bAAABBB"},
	}
	for _, tc := range cases {
		t.Run(tc.in, func(t *testing.T) {
			got, err := isccClean(tc.in)
			if err != nil {
				t.Fatalf("isccClean(%q): %v", tc.in, err)
			}
			if got != tc.want {
				t.Errorf("isccClean(%q) = %q, want %q", tc.in, got, tc.want)
			}
		})
	}
}

func TestIsccCleanUnitRejects(t *testing.T) {
	cases := map[string]struct{ in, msg string }{
		"empty":            {"", "empty ISCC string"},
		"dash only":        {"-", "empty ISCC string"},
		"blank":            {"   ", "empty ISCC string"},
		"scheme only":      {"iscc:", "empty ISCC string"},
		"scheme and dash":  {"ISCC: - ", "empty ISCC string"},
		"two colons":       {"a:b:c", "malformed ISCC string"},
		"two colons iscc":  {"ISCC:GAAA:AAAAAA", "malformed ISCC string"},
		"long s scheme":    {"iſcc:GAAAAAAAAA", "invalid scheme"},
		"dotted I scheme":  {"İSCC:GAAAAAAAAA", "invalid scheme"},
		"other scheme":     {"isbn:GAAAAAAAAA", "invalid scheme"},
		"empty scheme":     {":GAAAAAAAAA", "invalid scheme"},
		"scheme too short": {"isc:GAAAAAAAAA", "invalid scheme"},
	}
	for name, tc := range cases {
		t.Run(name, func(t *testing.T) {
			_, err := isccClean(tc.in)
			if err == nil {
				t.Fatalf("isccClean(%q): expected error", tc.in)
			}
			if !strings.Contains(err.Error(), tc.msg) {
				t.Errorf("isccClean(%q): got %q, want %q", tc.in, err, tc.msg)
			}
		})
	}
}

func TestIsccCleanIsccDecompose(t *testing.T) {
	want, err := IsccDecompose("ISCC:" + cleanIsccCode)
	if err != nil {
		t.Fatalf("canonical: %v", err)
	}
	units := []string{"AAAYPXW445FTYNJ3", "EAARMJLTQCUWAND2", "GAAVVC5DMJJGYKZ4", "IAAWIK7A7JTUAQ2D"}
	if !reflect.DeepEqual(want, units) {
		t.Fatalf("canonical: got %v, want %v", want, units)
	}
	for name, form := range cleanForms {
		t.Run(name, func(t *testing.T) {
			in := form(cleanIsccCode)
			got, err := IsccDecompose(in)
			if err != nil {
				t.Fatalf("IsccDecompose(%q): %v", in, err)
			}
			if !reflect.DeepEqual(got, want) {
				t.Errorf("IsccDecompose(%q) = %v, want %v", in, got, want)
			}
		})
	}
}

func TestIsccCleanIsccDecomposeTrailingNewlineUnit(t *testing.T) {
	got, err := IsccDecompose("ISCC:GAAAAAAAAA\n")
	if err != nil {
		t.Fatalf("IsccDecompose: %v", err)
	}
	if !reflect.DeepEqual(got, []string{"GAAAAAAAAA"}) {
		t.Errorf("got %v, want [GAAAAAAAAA]", got)
	}
}

func TestIsccCleanIsccDecode(t *testing.T) {
	for _, code := range []string{cleanIsccCode, "GAAAAAAAAA", cleanMixedUnits[1]} {
		want, err := IsccDecode("ISCC:" + code)
		if err != nil {
			t.Fatalf("canonical %s: %v", code, err)
		}
		for name, form := range cleanForms {
			in := form(code)
			t.Run(code+"/"+name, func(t *testing.T) {
				got, err := IsccDecode(in)
				if err != nil {
					t.Fatalf("IsccDecode(%q): %v", in, err)
				}
				if !reflect.DeepEqual(got, want) {
					t.Errorf("IsccDecode(%q) = %+v, want %+v", in, got, want)
				}
			})
		}
	}
}

func TestIsccCleanIsccDecodeReferenceFacts(t *testing.T) {
	for _, in := range []string{
		"ISCC:GAAAAAAAAA\n", "\x1cISCC:GAAAAAAAAA\x1f", "iscc:GAAAAAAAAA", " ISCC : GAAAAAAAAA ",
	} {
		t.Run(in, func(t *testing.T) {
			r, err := IsccDecode(in)
			if err != nil {
				t.Fatalf("IsccDecode(%q): %v", in, err)
			}
			if r.Maintype != 3 || r.Subtype != 0 || r.Version != 0 {
				t.Errorf("IsccDecode(%q): got (%d,%d,%d), want (3,0,0)", in, r.Maintype, r.Subtype, r.Version)
			}
		})
	}
}

func TestIsccCleanGenIsccCodeV0(t *testing.T) {
	want := "ISCC:" + cleanIsccCode
	for name, form := range cleanForms {
		t.Run(name, func(t *testing.T) {
			in := mapForm(form, cleanIsccUnits)
			got, err := GenIsccCodeV0(in, false)
			if err != nil {
				t.Fatalf("GenIsccCodeV0(%q): %v", in, err)
			}
			if got.Iscc != want {
				t.Errorf("GenIsccCodeV0(%q) = %s, want %s", in, got.Iscc, want)
			}
		})
	}
}

func TestIsccCleanGenMixedCodeV0(t *testing.T) {
	want := "ISCC:EQASNZJ36ZT33AL7"
	for name, form := range cleanForms {
		t.Run(name, func(t *testing.T) {
			in := mapForm(form, cleanMixedUnits)
			got, err := GenMixedCodeV0(in, 64)
			if err != nil {
				t.Fatalf("GenMixedCodeV0(%q): %v", in, err)
			}
			if got.Iscc != want {
				t.Errorf("GenMixedCodeV0(%q) = %s, want %s", in, got.Iscc, want)
			}
			// Parts keep the caller's strings, as iscc_core returns parts=codes.
			if !reflect.DeepEqual(got.Parts, in) {
				t.Errorf("GenMixedCodeV0(%q).Parts = %q, want the raw inputs", in, got.Parts)
			}
		})
	}
}

// cleanRejectInputs holds one bad input per subtest with the expected error text.
var cleanRejectInputs = map[string]struct{ in, msg string }{
	"empty":         {"", "empty ISCC string"},
	"dash only":     {"-", "empty ISCC string"},
	"blank":         {"   ", "empty ISCC string"},
	"scheme only":   {"iscc:", "empty ISCC string"},
	"two colons":    {"a:b:c", "malformed ISCC string"},
	"long s scheme": {"iſcc:GAAAAAAAAA", "invalid scheme"},
}

func TestIsccCleanRejectsIsccDecompose(t *testing.T) {
	for name, tc := range cleanRejectInputs {
		t.Run(name, func(t *testing.T) {
			got, err := IsccDecompose(tc.in)
			if err == nil {
				t.Fatalf("IsccDecompose(%q) = %v: expected error", tc.in, got)
			}
			if !strings.Contains(err.Error(), tc.msg) {
				t.Errorf("IsccDecompose(%q): got %q, want %q", tc.in, err, tc.msg)
			}
		})
	}
}

func TestIsccCleanRejectsIsccDecode(t *testing.T) {
	for name, tc := range cleanRejectInputs {
		t.Run(name, func(t *testing.T) {
			_, err := IsccDecode(tc.in)
			if err == nil {
				t.Fatalf("IsccDecode(%q): expected error", tc.in)
			}
			if !strings.Contains(err.Error(), tc.msg) {
				t.Errorf("IsccDecode(%q): got %q, want %q", tc.in, err, tc.msg)
			}
		})
	}
}

func TestIsccCleanRejectsGenIsccCodeV0(t *testing.T) {
	for name, tc := range cleanRejectInputs {
		t.Run(name, func(t *testing.T) {
			// Only the last code is bad; its index must appear in the error.
			in := append(append([]string{}, cleanIsccUnits...), tc.in)
			_, err := GenIsccCodeV0(in, false)
			if err == nil {
				t.Fatalf("GenIsccCodeV0(%q): expected error", in)
			}
			if !strings.Contains(err.Error(), tc.msg) || !strings.Contains(err.Error(), "code 4") {
				t.Errorf("GenIsccCodeV0(%q): got %q, want %q at code 4", in, err, tc.msg)
			}
		})
	}
}

func TestIsccCleanRejectsGenMixedCodeV0(t *testing.T) {
	for name, tc := range cleanRejectInputs {
		t.Run(name, func(t *testing.T) {
			in := []string{cleanMixedUnits[0], tc.in}
			_, err := GenMixedCodeV0(in, 64)
			if err == nil {
				t.Fatalf("GenMixedCodeV0(%q): expected error", in)
			}
			if !strings.Contains(err.Error(), tc.msg) || !strings.Contains(err.Error(), "code 1") {
				t.Errorf("GenMixedCodeV0(%q): got %q, want %q at code 1", in, err, tc.msg)
			}
		})
	}
}
