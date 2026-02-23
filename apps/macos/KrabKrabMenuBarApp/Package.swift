// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "openkrabMenuBarApp",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .executable(
            name: "openkrabMenuBarApp",
            targets: ["openkrabMenuBarApp"]
        )
    ],
    targets: [
        .executableTarget(
            name: "openkrabMenuBarApp",
            path: "Sources"
        )
    ]
)

