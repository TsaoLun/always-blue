#!/bin/bash

# Build script for iOS deployment

set -e

echo "🍎 Building Always Blue for iOS..."

# Check if Dioxus CLI is installed
if ! command -v dx &> /dev/null; then
    echo "❌ Dioxus CLI not found. Installing..."
    cargo install dioxus-cli
fi

# Check if iOS targets are installed
if ! rustup target list --installed | grep -q "aarch64-apple-ios"; then
    echo "📱 Installing iOS targets..."
    rustup target add aarch64-apple-ios
    rustup target add x86_64-apple-ios
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Build for iOS
echo "🔨 Building for iOS..."
dx build --platform ios --release

echo "✅ iOS build completed successfully!"
echo ""
echo "📋 Next steps:"
echo "1. Open the generated Xcode project"
echo "2. Configure your Apple Developer Team ID in Xcode"
echo "3. Build and run on device or simulator"
echo ""
echo "🔗 Useful commands:"
echo "  dx serve --platform ios    # Run in iOS simulator"
echo "  dx build --platform ios    # Build for iOS"
