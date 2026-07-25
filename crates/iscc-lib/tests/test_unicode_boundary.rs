//! Data-driven Unicode 16.0.0 boundary conformance tests.
//!
//! Loads `tests/unicode_boundary.json` — the propagation source for every
//! binding's boundary conformance tests — and asserts that `text_clean` and
//! `text_collapse` enforce the declared Unicode data version (16.0.0): code
//! points unassigned in 16.0 are stripped before normalization, while code
//! points assigned in 16.0 flow through the regular normalization pipeline.

const BOUNDARY_DATA: &str = include_str!("unicode_boundary.json");

/// Parse the vendored boundary fixture into a JSON value.
fn boundary_data() -> serde_json::Value {
    serde_json::from_str(BOUNDARY_DATA).expect("unicode_boundary.json must be valid JSON")
}

/// Anti-silent-skip guard: the fixture declares Unicode 16.0.0 and both
/// sections parse with exactly 4 cases, even when `text-processing` is off.
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
        assert_eq!(
            cases.len(),
            4,
            "section {section} must have exactly 4 cases"
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
    assert_eq!(executed, 4, "expected 4 text_clean boundary cases to run");
}

/// `text_collapse` matches every Unicode 16.0 boundary vector.
#[cfg(feature = "text-processing")]
#[test]
fn test_text_collapse_boundary_vectors() {
    let executed = run_boundary_section("text_collapse", iscc_lib::text_collapse);
    assert_eq!(
        executed, 4,
        "expected 4 text_collapse boundary cases to run"
    );
}
