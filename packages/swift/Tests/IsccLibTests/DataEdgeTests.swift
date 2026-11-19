/// Shared edge-case vectors (data-edge.json, vendored from crates/iscc-lib/tests) run against
/// the Swift binding.
///
/// Every vector runs, `except["swift"]` is the only deviation, and a function section this
/// harness does not handle fails. A rejection is asserted on the raw binding call alone (it must
/// throw `IsccUniError`); the result is converted to its fixture form only after a successful
/// call. Every expected output key must be present in the converted result and equal.

import Foundation
import XCTest
@testable import IsccLib

/// A harness error: a vector whose inputs this harness cannot read.
struct EdgeHarnessError: Error, CustomStringConvertible {
    let description: String
}

/// One handled section: `prepare` reads the inputs into a raw binding call, `convert` turns the
/// call's successful result into the fixture form.
struct EdgeCaller {
    let prepare: ([Any]) throws -> () throws -> Any
    let convert: (Any) -> Any
}

/// The vector checks behind `DataEdgeTests`, kept free of traps so synthetic docs can be fed in.
enum DataEdge {

    /// Surface id of this harness in the `except` map.
    static let surface = "swift"

    /// Read the input at `index` as type `T`, or throw a harness error.
    static func arg<T>(_ inputs: [Any], _ index: Int, as type: T.Type) throws -> T {
        guard index < inputs.count else { throw EdgeHarnessError(description: "input \(index) is missing") }
        guard let value = inputs[index] as? T else {
            throw EdgeHarnessError(description: "input \(index) is not \(T.self)")
        }
        return value
    }

    /// Read the input at `index` as a `UInt32` bit length.
    static func bits(_ inputs: [Any], _ index: Int) throws -> UInt32 {
        let value = try arg(inputs, index, as: Int.self)
        guard let bits = UInt32(exactly: value) else {
            throw EdgeHarnessError(description: "input \(index) is not a UInt32")
        }
        return bits
    }

    /// Encode bytes as a `bytes:<hex>` fixture value.
    static func bytesValue(_ data: Data) -> String {
        "bytes:" + data.map { String(format: "%02x", $0) }.joined()
    }

    /// Prepare `isccDecode` on the vector's string input.
    static func prepareDecode(_ inputs: [Any]) throws -> () throws -> Any {
        let iscc = try arg(inputs, 0, as: String.self)
        return { try isccDecode(iscc: iscc) }
    }

    /// Convert a `DecodeResult` to `[maintype, subtype, version, length, "bytes:<hex>"]`.
    static func convertDecode(_ result: Any) -> Any {
        guard let r = result as? DecodeResult else { return "not a DecodeResult: \(result)" }
        return [Int(r.maintype), Int(r.subtype), Int(r.version), Int(r.length), bytesValue(r.digest)] as [Any]
    }

    /// Prepare `encodeSeq` on the vector's unit list.
    static func prepareSeq(_ inputs: [Any]) throws -> () throws -> Any {
        let units = try arg(inputs, 0, as: [String].self)
        return { try encodeSeq(units: units) }
    }

    /// Convert `encodeSeq` bytes to a `bytes:<hex>` value.
    static func convertSeq(_ result: Any) -> Any {
        guard let data = result as? Data else { return "not Data: \(result)" }
        return bytesValue(data)
    }

    /// Prepare `genTextCodeV0` on the vector's text and bits.
    static func prepareText(_ inputs: [Any]) throws -> () throws -> Any {
        let text = try arg(inputs, 0, as: String.self)
        let bits = try bits(inputs, 1)
        return { try genTextCodeV0(text: text, bits: bits) }
    }

    /// Convert a `TextCodeResult` to a dictionary of all its fields.
    static func convertText(_ result: Any) -> Any {
        guard let r = result as? TextCodeResult else { return "not a TextCodeResult: \(result)" }
        return ["iscc": r.iscc, "characters": Int(r.characters)] as [String: Any]
    }

    /// Prepare `genMixedCodeV0` on the vector's codes and bits.
    static func prepareMixed(_ inputs: [Any]) throws -> () throws -> Any {
        let codes = try arg(inputs, 0, as: [String].self)
        let bits = try bits(inputs, 1)
        return { try genMixedCodeV0(codes: codes, bits: bits) }
    }

    /// Convert a `MixedCodeResult` to a dictionary of all its fields.
    static func convertMixed(_ result: Any) -> Any {
        guard let r = result as? MixedCodeResult else { return "not a MixedCodeResult: \(result)" }
        return ["iscc": r.iscc, "parts": r.parts] as [String: Any]
    }

    /// Prepare `genIsccCodeV0` (the optional second input is `wide`).
    static func prepareIsccCode(_ inputs: [Any]) throws -> () throws -> Any {
        let codes = try arg(inputs, 0, as: [String].self)
        let wide = inputs.count > 1 ? try arg(inputs, 1, as: Bool.self) : false
        return { try genIsccCodeV0(codes: codes, wide: wide) }
    }

    /// Convert an `IsccCodeResult` to a dictionary of all its fields.
    static func convertIsccCode(_ result: Any) -> Any {
        guard let r = result as? IsccCodeResult else { return "not an IsccCodeResult: \(result)" }
        return ["iscc": r.iscc] as [String: Any]
    }

    /// Section name -> caller for every function section this harness handles.
    static let sections: [String: EdgeCaller] = [
        "iscc_decode": EdgeCaller(prepare: prepareDecode, convert: convertDecode),
        "encode_seq": EdgeCaller(prepare: prepareSeq, convert: convertSeq),
        "gen_text_code_v0": EdgeCaller(prepare: prepareText, convert: convertText),
        "gen_mixed_code_v0": EdgeCaller(prepare: prepareMixed, convert: convertMixed),
        "gen_iscc_code_v0": EdgeCaller(prepare: prepareIsccCode, convert: convertIsccCode),
    ]

    /// Load and parse the vendored data-edge.json.
    static func loadDoc() throws -> [String: Any] {
        guard let url = Bundle.module.url(forResource: "data-edge", withExtension: "json") else {
            throw EdgeHarnessError(description: "data-edge.json is not a test resource")
        }
        let object = try JSONSerialization.jsonObject(with: Data(contentsOf: url))
        guard let doc = object as? [String: Any] else {
            throw EdgeHarnessError(description: "data-edge.json is not an object")
        }
        return doc
    }

    /// Compare an expected fixture value with an actual one through typed casts; nil means equal.
    static func compareValue(_ expected: Any, _ actual: Any) -> String? {
        if let want = expected as? String {
            return (actual as? String) == want ? nil : "expected \(want), got \(actual)"
        }
        if let want = expected as? [Any] {
            guard let got = actual as? [Any], got.count == want.count else {
                return "expected \(want), got \(actual)"
            }
            return zip(want, got).lazy.compactMap { compareValue($0, $1) }.first
        }
        if let want = expected as? Int {
            return (actual as? Int) == want ? nil : "expected \(want), got \(actual)"
        }
        return "unsupported expected value \(expected)"
    }

    /// Compare a vector's `outputs` with the converted result; every expected key must be present.
    static func compare(_ expected: Any, _ actual: Any) -> String? {
        guard let fields = expected as? [String: Any] else { return compareValue(expected, actual) }
        guard let got = actual as? [String: Any] else { return "expected an object, got \(actual)" }
        for key in fields.keys.sorted() {
            guard let value = got[key] else { return "missing field \(key) in \(got)" }
            if let reason = compareValue(fields[key]!, value) { return "field \(key): \(reason)" }
        }
        return nil
    }

    /// Judge a successful raw call against the vector's expectation.
    static func judgeOk(_ expectError: Bool, _ got: Any, _ vector: [String: Any]) -> String? {
        if expectError { return "expected an error, got \(got)" }
        guard let outputs = vector["outputs"] else { return "vector has neither outputs nor error" }
        return compare(outputs, got)
    }

    /// Check one vector and return a failure description, or nil.
    static func checkVector(_ caller: EdgeCaller, _ vector: [String: Any]) -> String? {
        let exception = (vector["except"] as? [String: Any])?[surface] as? String
        if exception == "skip" { return nil }
        let expectError = exception == "error" || vector["error"] != nil
        let call: () throws -> Any
        do {
            call = try caller.prepare(vector["inputs"] as? [Any] ?? [])
        } catch {
            return "harness: \(error)"
        }
        let result: Any
        do {
            result = try call()
        } catch is IsccUniError {
            return expectError ? nil : "expected an output, got IsccUniError"
        } catch {
            return "raw call threw \(type(of: error)): \(error)"
        }
        return judgeOk(expectError, caller.convert(result), vector)
    }

    /// Run every vector of a parsed document; return one `section/vector: reason` per failure.
    static func checkDoc(_ doc: [String: Any]) -> [String] {
        var failures: [String] = []
        for section in doc.keys.sorted() {
            guard let caller = sections[section] else {
                failures.append("\(section): unknown section")
                continue
            }
            guard let vectors = doc[section] as? [String: Any] else {
                failures.append("\(section): not an object")
                continue
            }
            for name in vectors.keys.sorted() {
                guard let vector = vectors[name] as? [String: Any] else {
                    failures.append("\(section)/\(name): not an object")
                    continue
                }
                if let reason = checkVector(caller, vector) {
                    failures.append("\(section)/\(name): \(reason)")
                }
            }
        }
        return failures
    }
}

final class DataEdgeTests: XCTestCase {

    func testDataEdgeVectorsPass() throws {
        let doc = try DataEdge.loadDoc()
        for section in DataEdge.sections.keys.sorted() {
            let vectors = doc[section] as? [String: Any] ?? [:]
            XCTAssertFalse(vectors.isEmpty, "section \(section) has no vectors")
        }
        let failures = DataEdge.checkDoc(doc)

        XCTAssertEqual(failures, [], "data-edge.json failures:\n\(failures.joined(separator: "\n"))")
    }

    func testDataEdgeFailsUnknownSection() {
        let doc: [String: Any] = [
            "gen_unknown_v0": ["test_0001_x": ["inputs": ["x"], "outputs": "x", "note": "n"]],
        ]
        let failures = DataEdge.checkDoc(doc)

        XCTAssertEqual(failures.count, 1, "\(failures)")
        XCTAssertTrue(failures.first?.contains("unknown section") ?? false, "\(failures)")
    }

    func testDataEdgeFailsExpectedErrorThatSucceeds() {
        // Valid inputs marked `error` prove each rejection-only caller reaches the binding.
        let doc: [String: Any] = [
            "gen_mixed_code_v0": ["test_0001_x": [
                "inputs": [["EUA6GIKXN42IQV3S", "EIAUKMOUIOYZCKA5"], 64], "error": "e", "note": "n",
            ] as [String: Any]],
            "gen_iscc_code_v0": ["test_0001_x": [
                "inputs": [["GABVVC5DMJJGYKZ4ZBYVNYABFFYXG",
                            "IADWIK7A7JTUAQ2D6QARX7OBEIK3OOUAM42LOBLCZ4ZOGDLRHMDL6TQ"]],
                "error": "e", "note": "n",
            ] as [String: Any]],
        ]
        let failures = DataEdge.checkDoc(doc)

        XCTAssertEqual(failures.count, 2, "\(failures)")
        XCTAssertTrue(failures.allSatisfy { $0.contains("expected an error") }, "\(failures)")
    }

    func testDataEdgeFailsOutputMismatch() {
        let doc: [String: Any] = [
            "encode_seq": ["test_0001_x": [
                "inputs": [["ISCC:GAAAAAAAAA"]], "outputs": "bytes:300000000001", "note": "n",
            ] as [String: Any]],
        ]
        let failures = DataEdge.checkDoc(doc)

        XCTAssertEqual(failures.count, 1, "\(failures)")
        XCTAssertTrue(failures.first?.contains("expected bytes:300000000001") ?? false, "\(failures)")
    }

    func testDataEdgeFailsMissingOutputKey() {
        // `iscc` is correct; the only bad case is a key the result does not have.
        let doc: [String: Any] = [
            "gen_text_code_v0": ["test_0001_x": [
                "inputs": ["Hello\u{0}World", 64],
                "outputs": ["iscc": "ISCC:EAASKDNZNYGUUF5A", "datahash": "x"], "note": "n",
            ] as [String: Any]],
        ]
        let failures = DataEdge.checkDoc(doc)

        XCTAssertEqual(failures.count, 1, "\(failures)")
        XCTAssertTrue(failures.first?.contains("missing field datahash") ?? false, "\(failures)")
    }

    func testDataEdgeHonoursExcept() {
        let outputs: [Any] = [3, 0, 0, 0, "bytes:00000000"]
        let doc: [String: Any] = [
            "iscc_decode": [
                "test_0001_error": [
                    "inputs": ["ISCC:GAAAAAAAAA"], "outputs": outputs, "note": "n",
                    "except": ["swift": "error"],
                ] as [String: Any],
                "test_0002_skip": [
                    "inputs": ["ISCC:GAAAAAAAAB"], "outputs": outputs, "note": "n",
                    "except": ["swift": "skip"],
                ] as [String: Any],
            ],
        ]
        let failures = DataEdge.checkDoc(doc)

        XCTAssertEqual(failures.count, 1, "\(failures)")
        XCTAssertTrue(
            failures.first?.hasPrefix("iscc_decode/test_0001_error: expected an error") ?? false, "\(failures)"
        )
    }
}
