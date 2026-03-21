// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "IsccLib",
    products: [
        .library(name: "IsccLib", targets: ["IsccLib"]),
    ],
    targets: [
        .target(
            name: "IsccLibFFI",
            path: "Sources/IsccLibFFI",
            publicHeadersPath: ".",
            linkerSettings: [
                .linkedLibrary("iscc_uniffi"),
            ]
        ),
        .target(
            name: "IsccLib",
            dependencies: ["IsccLibFFI"],
            path: "Sources/IsccLib"
        ),
        .testTarget(
            name: "IsccLibTests",
            dependencies: ["IsccLib"],
            path: "Tests/IsccLibTests",
            resources: [.copy("data.json")]
        ),
    ]
)
