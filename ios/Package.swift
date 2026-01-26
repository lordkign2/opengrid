// swift-tools-version: 5.7
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "OpenGridIOS",
    platforms: [
        .iOS(.v14)
    ],
    products: [
        .library(
            name: "OpenGridIOS",
            targets: ["OpenGridIOS"]),
    ],
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        // .package(url: /* package url */, from: "1.0.0"),
    ],
    targets: [
        .target(
            name: "OpenGridIOS",
            dependencies: []),
        .testTarget(
            name: "OpenGridIOSTests",
            dependencies: ["OpenGridIOS"]),
    ]
)