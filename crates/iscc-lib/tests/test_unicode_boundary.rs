//! Data-driven Unicode 16.0.0 boundary conformance tests.
//!
//! Loads `tests/unicode_boundary.json` — the propagation source for every
//! binding's boundary conformance tests — and asserts that `text_clean` and
//! `text_collapse` enforce the declared Unicode data version (16.0.0): code
//! points unassigned in 16.0 are mapped to the noncharacter sentinel `U+FFFF`
//! before normalization (and removed by the unchanged category-`C` filter),
//! while code points assigned in 16.0 flow through the regular normalization
//! pipeline. Multi-code-point sequence vectors additionally pin the sentinel
//! map against the superseded delete-filter design.

const BOUNDARY_DATA: &str = include_str!("unicode_boundary.json");

/// The four boundary code points every section must exercise: two assigned in
/// Unicode 16.0 (`So`, `Mc`) that must survive, and two unassigned in 16.0
/// (assigned in 17.0) that the freeze rule must map to the sentinel before
/// normalization so the category filter removes them.
const BOUNDARY_CODE_POINTS: [char; 4] = ['\u{1FAE9}', '\u{113C5}', '\u{20C1}', '\u{A7F1}'];

/// Sequence boundary vectors: `(section, input, expected, delete_filter_output)`.
/// Unlike the single-code-point vectors these distinguish the `U+FFFF` sentinel
/// map from the superseded delete-filter design, so their exact strings are
/// pinned here — a fixture case silently degraded to ASCII must fail this guard.
///
/// `delete_filter_output` is what deleting the unassigned code point *before*
/// normalization yields. The `U+A7F1` row guards a second hazard on top of
/// that: `U+A7F1` gained a compatibility decomposition to `S` in Unicode 17, so
/// leaving it visible to the normalizer instead of mapping it to the sentinel
/// composes the following acute accent onto it and yields `e\u{015A}` — the
/// failure mode of the superseded category-override design.
const SEQUENCE_VECTORS: [(&str, &str, &str, &str); 4] = [
    ("text_clean", "e\u{0378}\u{0301}", "e\u{0301}", "\u{00E9}"),
    (
        "text_clean",
        "\u{1100}\u{0378}\u{1161}",
        "\u{1100}\u{1161}",
        "\u{AC00}",
    ),
    ("text_clean", "e\u{A7F1}\u{0301}", "e\u{0301}", "\u{00E9}"),
    (
        "text_collapse",
        "\u{0391}\u{03A3}\u{0378}\u{0392}",
        "\u{03B1}\u{03C2}\u{03B2}",
        "\u{03B1}\u{03C3}\u{03B2}",
    ),
];

/// Parse the vendored boundary fixture into a JSON value.
fn boundary_data() -> serde_json::Value {
    serde_json::from_str(BOUNDARY_DATA).expect("unicode_boundary.json must be valid JSON")
}

/// Anti-silent-skip guard: the fixture declares Unicode 16.0.0 and both
/// sections parse with the expected case count, covering exactly the four
/// boundary code points plus the non-ASCII characters of that section's
/// sequence inputs, even when `text-processing` is off. Without the code-point
/// check a weakened fixture (cases replaced by ASCII no-ops) would still pass
/// every vector test, because those compare the implementation against this
/// file.
#[test]
fn test_boundary_fixture_metadata() {
    let data = boundary_data();
    assert_eq!(
        data["_metadata"]["unicode_data_version"].as_str(),
        Some("16.0.0"),
        "fixture must declare Unicode data version 16.0.0"
    );
    for section in ["text_clean", "text_collapse"] {
        let cases = data[section]
            .as_object()
            .unwrap_or_else(|| panic!("section {section} must be a JSON object"));
        let rows: Vec<_> = SEQUENCE_VECTORS
            .iter()
            .filter(|(s, ..)| *s == section)
            .collect();
        assert_eq!(
            cases.len(),
            4 + rows.len(),
            "section {section} must have exactly {} cases",
            4 + rows.len()
        );
        let mut want: Vec<char> = BOUNDARY_CODE_POINTS.to_vec();
        want.extend(
            rows.iter()
                .flat_map(|(_, input, ..)| input.chars())
                .filter(|c| !c.is_ascii()),
        );
        want.sort_unstable();
        want.dedup();
        let mut covered: Vec<char> = cases
            .values()
            .flat_map(|tc| {
                tc["inputs"][0]
                    .as_str()
                    .unwrap_or_else(|| panic!("section {section} case must have a string input"))
                    .chars()
            })
            .filter(|c| !c.is_ascii())
            .collect();
        covered.sort_unstable();
        covered.dedup();
        assert_eq!(
            covered, want,
            "section {section} must cover exactly the expected non-ASCII code points"
        );
    }
}

/// Anti-degradation guard for the sequence vectors: each pinned `(input,
/// expected)` pair must appear exactly once in its fixture section, and the
/// recorded output must differ from what the superseded delete-filter design
/// would produce — so a fixture case silently rewritten to the wrong output
/// (or dropped) fails even when `text-processing` is off.
#[test]
fn test_boundary_fixture_sequence_vectors() {
    let data = boundary_data();
    for (section, input, expected, delete_filter_output) in SEQUENCE_VECTORS {
        let results: Vec<&str> = data[section]
            .as_object()
            .unwrap_or_else(|| panic!("section {section} must be a JSON object"))
            .iter()
            .filter(|(_, tc)| tc["inputs"][0].as_str() == Some(input))
            .map(|(name, tc)| {
                tc["outputs"]["result"]
                    .as_str()
                    .unwrap_or_else(|| panic!("case {section}.{name} must have a string result"))
            })
            .collect();
        assert_eq!(
            results.len(),
            1,
            "section {section} must contain exactly one case for input {input:?}"
        );
        assert_eq!(
            results[0], expected,
            "sequence vector {section} {input:?} must record the sentinel-map output"
        );
        assert_ne!(
            results[0], delete_filter_output,
            "sequence vector {section} {input:?} must not record the delete-filter output"
        );
    }
}

/// Run every case in `section` through `func`, asserting each expected
/// output, and return the number of cases executed.
#[cfg(feature = "text-processing")]
fn run_boundary_section(section: &str, func: fn(&str) -> String) -> usize {
    let data = boundary_data();
    let cases = data[section]
        .as_object()
        .unwrap_or_else(|| panic!("section {section} must be a JSON object"));
    let mut executed = 0;
    for (name, tc) in cases {
        let input = tc["inputs"][0]
            .as_str()
            .unwrap_or_else(|| panic!("case {section}.{name} must have a string input"));
        let expected = tc["outputs"]["result"]
            .as_str()
            .unwrap_or_else(|| panic!("case {section}.{name} must have a string result"));
        assert_eq!(func(input), expected, "case {section}.{name} failed");
        executed += 1;
    }
    executed
}

/// `text_clean` matches every Unicode 16.0 boundary vector.
#[cfg(feature = "text-processing")]
#[test]
fn test_text_clean_boundary_vectors() {
    let executed = run_boundary_section("text_clean", iscc_lib::text_clean);
    let sequences = SEQUENCE_VECTORS
        .iter()
        .filter(|(s, ..)| *s == "text_clean")
        .count();
    let expected = 4 + sequences;
    assert_eq!(
        executed, expected,
        "expected {expected} text_clean boundary cases to run"
    );
}

/// `text_collapse` matches every Unicode 16.0 boundary vector.
#[cfg(feature = "text-processing")]
#[test]
fn test_text_collapse_boundary_vectors() {
    let executed = run_boundary_section("text_collapse", iscc_lib::text_collapse);
    let sequences = SEQUENCE_VECTORS
        .iter()
        .filter(|(s, ..)| *s == "text_collapse")
        .count();
    let expected = 4 + sequences;
    assert_eq!(
        executed, expected,
        "expected {expected} text_collapse boundary cases to run"
    );
}
