// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "IsccLib",
    products: [
        .library(name: "IsccLib", targets: ["IsccLib"]),
    ],
    targets: [
        .target(
            name: "iscc_uniffiFFI",
            path: "packages/swift/Sources/iscc_uniffiFFI",
            publicHeadersPath: ".",
            linkerSettings: [
                .linkedLibrary("iscc_uniffi"),
            ]
        ),
        .target(
            name: "IsccLib",
            dependencies: ["iscc_uniffiFFI"],
            path: "packages/swift/Sources/IsccLib"
        ),
    ]
)
