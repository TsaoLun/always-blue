#!/bin/bash

# Build and Deploy Script for Slint Rust Template
# This script ensures all files are built and ready for Deno Deploy

set -e  # Exit on any error

echo "🔧 Starting build and deploy process..."

# Navigate to project root
cd "$(dirname "$0")/.."

echo "📦 Building WebAssembly package..."
# Build the project for WebAssembly
cargo build --release --target wasm32-unknown-unknown

echo "🔄 Generating WASM bindings..."
# Generate WASM bindings
wasm-pack build --target web --out-dir pkg --release

echo "✅ Verifying required files exist..."
# Check if all required files exist
required_files=(
    "index.html"
    "pkg/slint_rust_template.js"
    "pkg/slint_rust_template_bg.wasm"
    "deploy/deno_deploy.ts"
)

for file in "${required_files[@]}"; do
    if [[ ! -f "$file" ]]; then
        echo "❌ Required file missing: $file"
        exit 1
    else
        echo "✓ Found: $file"
    fi
done

echo "📁 Checking file sizes..."
echo "  - JavaScript: $(du -h pkg/slint_rust_template.js | cut -f1)"
echo "  - WASM: $(du -h pkg/slint_rust_template_bg.wasm | cut -f1)"

echo "🚀 Ready for deployment!"
echo ""
echo "To deploy to Deno Deploy:"
echo "1. Make sure you have deployctl installed: deno install --allow-all --no-check -r -f https://deno.land/x/deploy/deployctl.ts"
echo "2. Run: deployctl deploy --project=always-blue-demo deploy/deno_deploy.ts"
echo ""
echo "Or use the Deno Deploy dashboard to deploy from GitHub."
