// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "KrabKrabMenuBarApp",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .executable(
            name: "KrabKrabMenuBarApp",
            targets: ["KrabKrabMenuBarApp"]
        )
    ],
    targets: [
        .executableTarget(
            name: "KrabKrabMenuBarApp",
            path: "Sources"
        )
    ]
)
