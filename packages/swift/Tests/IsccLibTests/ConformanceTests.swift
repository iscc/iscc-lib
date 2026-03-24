/// Conformance tests for all 9 gen_*_v0 functions against data.json vectors.
/// Validates the Swift binding produces correct ISCC codes matching the iscc-core reference.

import Foundation
import XCTest
@testable import IsccLib

final class ConformanceTests: XCTestCase {

    /// Cached parsed data.json for all test methods.
    static var dataJson: [String: Any] = {
        let url = Bundle.module.url(forResource: "data", withExtension: "json")!
        let data = try! Data(contentsOf: url)
        return try! JSONSerialization.jsonObject(with: data) as! [String: Any]
    }()

    /// Decode a "stream:<hex>" string to Data.
    static func decodeStream(_ streamStr: String) -> Data {
        let hex = String(streamStr.dropFirst("stream:".count))
        if hex.isEmpty { return Data() }
        var bytes = [UInt8]()
        var index = hex.startIndex
        while index < hex.endIndex {
            let nextIndex = hex.index(index, offsetBy: 2)
            let byteStr = String(hex[index..<nextIndex])
            bytes.append(UInt8(byteStr, radix: 16)!)
            index = nextIndex
        }
        return Data(bytes)
    }

    /// Prepare the meta parameter from a JSON value (NSNull, String, or Dictionary).
    static func prepareMeta(_ value: Any) -> String? {
        if value is NSNull { return nil }
        if let str = value as? String { return str }
        if let dict = value as? [String: Any] {
            let data = try! JSONSerialization.data(withJSONObject: dict, options: [.sortedKeys])
            return String(data: data, encoding: .utf8)
        }
        fatalError("Unexpected meta type: \(type(of: value))")
    }

    /// Load test vectors for a given function name.
    static func vectors(for functionName: String) -> [(String, [String: Any])] {
        guard let group = dataJson[functionName] as? [String: Any] else { return [] }
        return group.sorted { $0.key < $1.key }.map { ($0.key, $0.value as! [String: Any]) }
    }

    // MARK: - gen_meta_code_v0

    func testGenMetaCodeV0() throws {
        let vectors = Self.vectors(for: "gen_meta_code_v0")
        XCTAssertEqual(vectors.count, 20, "Expected 20 meta code vectors")

        for (name, tc) in vectors {
            let inputs = tc["inputs"] as! [Any]
            let outputs = tc["outputs"] as! [String: Any]

            let nameStr = inputs[0] as! String
            let descStr = inputs[1] as! String
            let description: String? = descStr.isEmpty ? nil : descStr
            let meta = Self.prepareMeta(inputs[2])
            let bits = UInt32(inputs[3] as! Int)

            let result = try genMetaCodeV0(name: nameStr, description: description, meta: meta, bits: bits)
            XCTAssertEqual(result.iscc, outputs["iscc"] as! String, "Failed vector: \(name)")
        }
    }

    // MARK: - gen_text_code_v0

    func testGenTextCodeV0() throws {
        let vectors = Self.vectors(for: "gen_text_code_v0")
        XCTAssertEqual(vectors.count, 5, "Expected 5 text code vectors")

        for (name, tc) in vectors {
            let inputs = tc["inputs"] as! [Any]
            let outputs = tc["outputs"] as! [String: Any]

            let text = inputs[0] as! String
            let bits = UInt32(inputs[1] as! Int)

            let result = try genTextCodeV0(text: text, bits: bits)
            XCTAssertEqual(result.iscc, outputs["iscc"] as! String, "Failed vector: \(name)")
        }
    }

    // MARK: - gen_image_code_v0

    func testGenImageCodeV0() throws {
        let vectors = Self.vectors(for: "gen_image_code_v0")
        XCTAssertEqual(vectors.count, 3, "Expected 3 image code vectors")

        for (name, tc) in vectors {
            let inputs = tc["inputs"] as! [Any]
            let outputs = tc["outputs"] as! [String: Any]

            let pixelArray = inputs[0] as! [Int]
            let pixels = Data(pixelArray.map { UInt8($0) })
            let bits = UInt32(inputs[1] as! Int)

            let result = try genImageCodeV0(pixels: pixels, bits: bits)
            XCTAssertEqual(result.iscc, outputs["iscc"] as! String, "Failed vector: \(name)")
        }
    }

    // MARK: - gen_audio_code_v0

    func testGenAudioCodeV0() throws {
        let vectors = Self.vectors(for: "gen_audio_code_v0")
        XCTAssertEqual(vectors.count, 5, "Expected 5 audio code vectors")

        for (name, tc) in vectors {
            let inputs = tc["inputs"] as! [Any]
            let outputs = tc["outputs"] as! [String: Any]

            let cvArray = inputs[0] as! [Int]
            let cv = cvArray.map { Int32($0) }
            let bits = UInt32(inputs[1] as! Int)

            let result = try genAudioCodeV0(cv: cv, bits: bits)
            XCTAssertEqual(result.iscc, outputs["iscc"] as! String, "Failed vector: \(name)")
        }
    }

    // MARK: - gen_video_code_v0

    func testGenVideoCodeV0() throws {
        let vectors = Self.vectors(for: "gen_video_code_v0")
        XCTAssertEqual(vectors.count, 3, "Expected 3 video code vectors")

        for (name, tc) in vectors {
            let inputs = tc["inputs"] as! [Any]
            let outputs = tc["outputs"] as! [String: Any]

            let framesArray = inputs[0] as! [[Int]]
            let frameSigs = framesArray.map { frame in frame.map { Int32($0) } }
            let bits = UInt32(inputs[1] as! Int)

            let result = try genVideoCodeV0(frameSigs: frameSigs, bits: bits)
            XCTAssertEqual(result.iscc, outputs["iscc"] as! String, "Failed vector: \(name)")
        }
    }

    // MARK: - gen_mixed_code_v0

    func testGenMixedCodeV0() throws {
        let vectors = Self.vectors(for: "gen_mixed_code_v0")
        XCTAssertEqual(vectors.count, 2, "Expected 2 mixed code vectors")

        for (name, tc) in vectors {
            let inputs = tc["inputs"] as! [Any]
            let outputs = tc["outputs"] as! [String: Any]

            let codes = inputs[0] as! [String]
            let bits = UInt32(inputs[1] as! Int)

            let result = try genMixedCodeV0(codes: codes, bits: bits)
            XCTAssertEqual(result.iscc, outputs["iscc"] as! String, "Failed vector: \(name)")
        }
    }

    // MARK: - gen_data_code_v0

    func testGenDataCodeV0() throws {
        let vectors = Self.vectors(for: "gen_data_code_v0")
        XCTAssertEqual(vectors.count, 4, "Expected 4 data code vectors")

        for (name, tc) in vectors {
            let inputs = tc["inputs"] as! [Any]
            let outputs = tc["outputs"] as! [String: Any]

            let data = Self.decodeStream(inputs[0] as! String)
            let bits = UInt32(inputs[1] as! Int)

            let result = try genDataCodeV0(data: data, bits: bits)
            XCTAssertEqual(result.iscc, outputs["iscc"] as! String, "Failed vector: \(name)")
        }
    }

    // MARK: - gen_instance_code_v0

    func testGenInstanceCodeV0() throws {
        let vectors = Self.vectors(for: "gen_instance_code_v0")
        XCTAssertEqual(vectors.count, 3, "Expected 3 instance code vectors")

        for (name, tc) in vectors {
            let inputs = tc["inputs"] as! [Any]
            let outputs = tc["outputs"] as! [String: Any]

            let data = Self.decodeStream(inputs[0] as! String)
            let bits = UInt32(inputs[1] as! Int)

            let result = try genInstanceCodeV0(data: data, bits: bits)
            XCTAssertEqual(result.iscc, outputs["iscc"] as! String, "Failed vector: \(name)")
        }
    }

    // MARK: - gen_iscc_code_v0

    func testGenIsccCodeV0() throws {
        let vectors = Self.vectors(for: "gen_iscc_code_v0")
        XCTAssertEqual(vectors.count, 5, "Expected 5 iscc code vectors")

        for (name, tc) in vectors {
            let inputs = tc["inputs"] as! [Any]
            let outputs = tc["outputs"] as! [String: Any]

            let codes = inputs[0] as! [String]
            // gen_iscc_code_v0 vectors have no wide parameter — always pass false
            let result = try genIsccCodeV0(codes: codes, wide: false)
            XCTAssertEqual(result.iscc, outputs["iscc"] as! String, "Failed vector: \(name)")
        }
    }
}
