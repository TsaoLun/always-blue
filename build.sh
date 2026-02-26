#!/bin/bash

set -e  # 出错时立即退出

echo "=== Building WASM ==="
echo "Starting build at $(date)"

# 清理之前的构建
if [ -d "pkg" ]; then
    echo "Cleaning previous build..."
    rm -rf pkg/*
fi

# 构建 WASM
echo "Running wasm-pack build with optimizations..."
wasm-pack build --target web --out-dir pkg --release -- --features wasm

# 复制音频文件到pkg目录
echo "Copying audio files..."
if [ ! -d "pkg/raw" ]; then
    mkdir -p pkg/raw
fi
cp raw/moments.mp3 pkg/raw/
cp raw/beep.wav pkg/raw/

# 优化WASM文件大小
echo "Optimizing WASM file size..."
if command -v wasm-opt >/dev/null 2>&1; then
    echo "Running wasm-opt for further optimization..."
    wasm-opt -Oz --enable-mutable-globals pkg/always_blue_wasm_bg.wasm -o pkg/always_blue_wasm_bg.wasm
else
    echo "wasm-opt not found, skipping additional optimization"
fi

# 添加Brotli压缩
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

# 显示文件大小
echo "WASM file size:"
ls -lh pkg/always_blue_wasm_bg.wasm

# 检查构建结果
if [ ! -f "pkg/always_blue_wasm.js" ]; then
    echo "Error: Build failed - JavaScript file not found"
    exit 1
fi

if [ ! -f "pkg/always_blue_wasm_bg.wasm" ]; then
    echo "Error: Build failed - WASM file not found"
    exit 1
fi

# 添加构建时间到文件中，帮助调试
echo "// Built at $(date)" >> pkg/always_blue_wasm.js

echo "=== Build complete! ==="
echo "Files updated with timestamp: $(date)"
echo "Generated files:"
ls -la pkg/
