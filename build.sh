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
echo "Running wasm-pack build..."
wasm-pack build --target web --out-dir pkg

# 检查构建结果
if [ ! -f "pkg/slint_rust_template.js" ]; then
    echo "Error: Build failed - JavaScript file not found"
    exit 1
fi

if [ ! -f "pkg/slint_rust_template_bg.wasm" ]; then
    echo "Error: Build failed - WASM file not found"
    exit 1
fi

# 添加构建时间到文件中，帮助调试
echo "// Built at $(date)" >> pkg/slint_rust_template.js

echo "=== Build complete! ==="
echo "Files updated with timestamp: $(date)"
echo "Generated files:"
ls -la pkg/
