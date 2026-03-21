#!/bin/bash
# Build an XCFramework containing libiscc_uniffi for all Apple platforms.
#
# Usage: ./scripts/build_xcframework.sh [--release|--debug]
# Output: target/ios/IsccLib.xcframework.zip
# Prints: SHA256 checksum (via swift package compute-checksum)
#
# Requires: Rust toolchain with Apple targets, Xcode command-line tools.

set -euo pipefail

# --- Configuration -----------------------------------------------------------

PROFILE_FLAG="${1:---release}"

if [[ "$PROFILE_FLAG" == "--debug" ]]; then
    PROFILE_DIR="debug"
else
    PROFILE_FLAG="--release"
    PROFILE_DIR="release"
fi

TARGETS=(
    aarch64-apple-darwin
    x86_64-apple-darwin
    aarch64-apple-ios
    aarch64-apple-ios-sim
    x86_64-apple-ios
)

LIB_NAME="libiscc_uniffi.a"
OUTPUT_DIR="target/ios"
STAGING_DIR="target/xcframework-staging"
MACOS_FAT_DIR="target/macos-fat"
IOS_SIM_FAT_DIR="target/ios-simulator-fat"
HEADERS_DIR="$STAGING_DIR/headers"

# --- 1. Cross-compile for all Apple targets ----------------------------------

echo "==> Cross-compiling iscc-uniffi for ${#TARGETS[@]} targets ($PROFILE_DIR)..."
for target in "${TARGETS[@]}"; do
    echo "  -> $target"
    cargo build -p iscc-uniffi $PROFILE_FLAG --target "$target"
done

# --- 2. Create fat binaries with lipo ---------------------------------------

echo "==> Creating fat binaries..."

mkdir -p "$MACOS_FAT_DIR" "$IOS_SIM_FAT_DIR"

# macOS: arm64 + x86_64
lipo -create \
    "target/aarch64-apple-darwin/$PROFILE_DIR/$LIB_NAME" \
    "target/x86_64-apple-darwin/$PROFILE_DIR/$LIB_NAME" \
    -output "$MACOS_FAT_DIR/$LIB_NAME"

# iOS simulator: arm64 + x86_64
lipo -create \
    "target/aarch64-apple-ios-sim/$PROFILE_DIR/$LIB_NAME" \
    "target/x86_64-apple-ios/$PROFILE_DIR/$LIB_NAME" \
    -output "$IOS_SIM_FAT_DIR/$LIB_NAME"

# --- 3. Stage headers -------------------------------------------------------

echo "==> Staging headers..."
mkdir -p "$HEADERS_DIR"

cp packages/swift/Sources/iscc_uniffiFFI/iscc_uniffiFFI.h "$HEADERS_DIR/"
cp packages/swift/Sources/iscc_uniffiFFI/module.modulemap "$HEADERS_DIR/"

# --- 4. Assemble XCFramework ------------------------------------------------

echo "==> Assembling XCFramework..."

# Clean previous output
rm -rf "$OUTPUT_DIR/IsccLib.xcframework"

xcodebuild -create-xcframework \
    -library "$MACOS_FAT_DIR/$LIB_NAME" \
    -headers "$HEADERS_DIR" \
    -library "target/aarch64-apple-ios/$PROFILE_DIR/$LIB_NAME" \
    -headers "$HEADERS_DIR" \
    -library "$IOS_SIM_FAT_DIR/$LIB_NAME" \
    -headers "$HEADERS_DIR" \
    -output "$OUTPUT_DIR/IsccLib.xcframework"

# --- 5. Zip with ditto (preserves resource forks) ---------------------------

echo "==> Zipping XCFramework..."

rm -f "$OUTPUT_DIR/IsccLib.xcframework.zip"

ditto -c -k --sequesterRsrc --keepParent \
    "$OUTPUT_DIR/IsccLib.xcframework" \
    "$OUTPUT_DIR/IsccLib.xcframework.zip"

# --- 6. Compute checksum ----------------------------------------------------

echo "==> Checksum:"
swift package compute-checksum "$OUTPUT_DIR/IsccLib.xcframework.zip"
