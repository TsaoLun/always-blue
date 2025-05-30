#!/bin/bash

# Quick test script for iOS build
echo "🧪 Testing iOS build compatibility..."

# Check if we're on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "❌ iOS development requires macOS"
    exit 1
fi

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Please install Rust first."
    exit 1
fi

# Check if Dioxus CLI is installed
if ! command -v dx &> /dev/null; then
    echo "🔧 Installing Dioxus CLI..."
    cargo install dioxus-cli
fi

# Check iOS targets
echo "📱 Checking iOS targets..."
if ! rustup target list --installed | grep -q "aarch64-apple-ios"; then
    echo "🔧 Installing iOS targets..."
    rustup target add aarch64-apple-ios x86_64-apple-ios
fi

# Test compilation
echo "🔨 Testing compilation..."
if cargo check --target aarch64-apple-ios --features mobile; then
    echo "✅ iOS compilation test passed!"
else
    echo "❌ iOS compilation test failed!"
    exit 1
fi

echo ""
echo "🎉 iOS compatibility check completed successfully!"
echo ""
echo "🚀 You can now run:"
echo "  ./build-ios.sh     # Build for iOS"
echo "  dx serve --platform ios  # Run in simulator"
