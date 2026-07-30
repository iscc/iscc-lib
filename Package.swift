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
    let releaseTag = "0.6.0"
    let releaseChecksum = "0cb2244ff2ab17ab9d20be5d9996d4d5841fd9406d4e23111a5d57c5c238ee63"
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
