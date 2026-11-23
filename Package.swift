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
    let releaseTag = "0.8.0"
    let releaseChecksum = "37d06e73c723c6b6f7f2f97cc20671b0d56ec740727afcd94e017dab36e408ec"
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
