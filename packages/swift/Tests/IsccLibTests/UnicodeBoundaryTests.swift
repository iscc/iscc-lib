/// Unicode 16.0.0 boundary vector tests for textClean / textCollapse.
/// Gates the declared-Unicode-version freeze rule (code points unassigned in Unicode 16.0.0
/// map to the U+FFFF sentinel before normalization) against the vendored
/// unicode_boundary.json fixture — 7 text_clean + 5 text_collapse vectors.

import Foundation
import XCTest
@testable import IsccLib

final class UnicodeBoundaryTests: XCTestCase {

    /// Declared Unicode data version pinned by the fixture metadata.
    static let expectedUnicodeVersion = "16.0.0"

    /// Expected number of text_clean boundary vectors.
    static let expectedTextCleanCount = 7

    /// Expected number of text_collapse boundary vectors.
    static let expectedTextCollapseCount = 5

    /// Cached parsed unicode_boundary.json for all test methods.
    static var fixture: [String: Any] = {
        let url = Bundle.module.url(forResource: "unicode_boundary", withExtension: "json")!
        let data = try! Data(contentsOf: url)
        return try! JSONSerialization.jsonObject(with: data) as! [String: Any]
    }()

    /// Load boundary vectors for a given fixture section, sorted by case name.
    static func vectors(for section: String) -> [(String, [String: Any])] {
        guard let group = fixture[section] as? [String: Any] else { return [] }
        return group.sorted { $0.key < $1.key }.map { ($0.key, $0.value as! [String: Any]) }
    }

    /// Unicode scalar values of a string, for exact code-point comparison.
    ///
    /// Swift's `String ==` (and therefore a bare `XCTAssertEqual` on strings) folds
    /// canonical equivalence: "e" + U+0301 == U+00E9 is true. That would make the
    /// sequence vectors vacuous — they exist precisely to distinguish the U+FFFF
    /// sentinel map from a delete filter, and both designs produce canonically
    /// equivalent strings for three of the four sequences. Always compare scalar
    /// arrays here, never strings directly.
    static func scalars(_ text: String) -> [UInt32] {
        return text.unicodeScalars.map { $0.value }
    }

    /// Run every vector in a section through a transform and assert scalar-exact output.
    static func assertVectors(
        section: String,
        transform: (String) -> String,
        file: StaticString = #filePath,
        line: UInt = #line
    ) {
        for (name, tc) in vectors(for: section) {
            let inputs = tc["inputs"] as! [Any]
            let outputs = tc["outputs"] as! [String: Any]
            let input = inputs[0] as! String
            let expected = outputs["result"] as! String
            let actual = transform(input)
            XCTAssertEqual(
                scalars(actual), scalars(expected),
                "Failed vector: \(section)/\(name)", file: file, line: line)
        }
    }

    /// Verify the fixture pins the declared Unicode version and both vector counts.
    func testFixtureMetadata() {
        let metadata = Self.fixture["_metadata"] as! [String: Any]
        XCTAssertEqual(
            metadata["unicode_data_version"] as! String, Self.expectedUnicodeVersion,
            "Fixture Unicode data version drifted")
        XCTAssertEqual(
            Self.vectors(for: "text_clean").count, Self.expectedTextCleanCount,
            "Unexpected text_clean vector count")
        XCTAssertEqual(
            Self.vectors(for: "text_collapse").count, Self.expectedTextCollapseCount,
            "Unexpected text_collapse vector count")
    }

    /// All 7 text_clean boundary vectors must pass with scalar-exact output.
    func testTextCleanBoundaryVectors() {
        Self.assertVectors(section: "text_clean", transform: { textClean(text: $0) })
    }

    /// All 5 text_collapse boundary vectors must pass with scalar-exact output.
    func testTextCollapseBoundaryVectors() {
        Self.assertVectors(section: "text_collapse", transform: { textCollapse(text: $0) })
    }
}
