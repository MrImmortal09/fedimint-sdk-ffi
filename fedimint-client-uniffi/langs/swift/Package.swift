// swift-tools-version:5.5
import PackageDescription

let package = Package(
    name: "Fedimint",
    platforms: [
        .macOS("13.0"),
        .iOS(.v13),
    ],
    products: [
        .library(name: "Fedimint", targets: ["FedimintFFI", "Fedimint"])
    ],
    targets: [
        .binaryTarget(name: "FedimintFFI", path: "./FedimintFFI.xcframework"),
        .target(
            name: "Fedimint",
            dependencies: [
                "FedimintFFI"
            ]),
    ]
)
