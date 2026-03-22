// swift-tools-version: 5.9
import PackageDescription

// Toggle for local development vs distribution.
// Set to `true` after running `./scripts/build_xcframework.sh` locally.
// Default `false` fetches prebuilt XCFramework from GitHub Releases.
let useLocalFramework = false

let binaryTarget: Target
if useLocalFramework {
    binaryTarget = .binaryTarget(
        name: "iscc_uniffiFFI",
        path: "target/ios/IsccLib.xcframework"
    )
} else {
    let releaseTag = "0.3.1"
    let releaseChecksum = "PLACEHOLDER"
    binaryTarget = .binaryTarget(
        name: "iscc_uniffiFFI",
        url: "https://github.com/iscc/iscc-lib/releases/download/v\(releaseTag)/IsccLib.xcframework.zip",
        checksum: releaseChecksum
    )
}

let package = Package(
    name: "IsccLib",
    platforms: [.macOS(.v13), .iOS(.v16)],
    products: [
        .library(name: "IsccLib", targets: ["IsccLib"]),
    ],
    targets: [
        binaryTarget,
        .target(
            name: "IsccLib",
            dependencies: ["iscc_uniffiFFI"],
            path: "packages/swift/Sources/IsccLib"
        ),
    ]
)
