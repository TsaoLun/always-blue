#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

echo "=== Building WASM ==="
echo "Starting build at $(date)"

# Clean up previous build
if [ -d "pkg" ]; then
    echo "Cleaning previous build..."
    rm -rf pkg/*
fi

# Build WASM
echo "Running wasm-pack build with optimizations..."
wasm-pack build --target web --out-dir pkg --release -- --features wasm --no-default-features
rm -f pkg/.gitignore

# Copy audio files to pkg directory
echo "Copying audio files..."
if [ ! -d "pkg/raw" ]; then
    mkdir -p pkg/raw
fi
cp raw/moments.mp3 pkg/raw/
cp raw/beep.wav pkg/raw/

# Optimize WASM file size
echo "Optimizing WASM file size..."
if command -v wasm-opt >/dev/null 2>&1; then
    echo "Running wasm-opt for further optimization..."
    wasm-opt -Oz --enable-mutable-globals pkg/always_blue_wasm_bg.wasm -o pkg/always_blue_wasm_bg.wasm
else
    echo "wasm-opt not found, skipping additional optimization"
fi

# Add Brotli compression
echo "Creating Brotli compressed versions..."
if command -v brotli >/dev/null 2>&1; then
    echo "Compressing WASM with Brotli..."
    brotli -q 11 -o pkg/always_blue_wasm_bg.wasm.br pkg/always_blue_wasm_bg.wasm
    
    echo "Compressing JS with Brotli..."
    brotli -q 11 -o pkg/always_blue_wasm.js.br pkg/always_blue_wasm.js
    
    echo "Compression results:"
    echo "Original WASM: $(ls -lh pkg/always_blue_wasm_bg.wasm | awk '{print $5}')"
    echo "Brotli WASM:   $(ls -lh pkg/always_blue_wasm_bg.wasm.br | awk '{print $5}')"
    echo "Original JS:   $(ls -lh pkg/always_blue_wasm.js | awk '{print $5}')"
    echo "Brotli JS:     $(ls -lh pkg/always_blue_wasm.js.br | awk '{print $5}')"
else
    echo "brotli not found, skipping compression"
fi

# Display file size
echo "WASM file size:"
ls -lh pkg/always_blue_wasm_bg.wasm

# Check build results
if [ ! -f "pkg/always_blue_wasm.js" ]; then
    echo "Error: Build failed - JavaScript file not found"
    exit 1
fi

if [ ! -f "pkg/always_blue_wasm_bg.wasm" ]; then
    echo "Error: Build failed - WASM file not found"
    exit 1
fi

# Add build timestamp to file to help debugging
echo "// Built at $(date)" >> pkg/always_blue_wasm.js

echo "=== Build complete! ==="
echo "Files updated with timestamp: $(date)"
echo "Generated files:"
ls -la pkg/
