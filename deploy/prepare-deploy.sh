#!/bin/bash

# Prepare files for Deno Deploy
# This script copies all necessary files to the deploy directory

set -e

echo "🔧 Preparing files for Deno Deploy..."

# Navigate to project root
cd "$(dirname "$0")/.."

# Create deploy directory structure
mkdir -p deploy/pkg
mkdir -p deploy/assets

echo "📋 Copying files to deploy directory..."

# Copy main files
cp index.html deploy/
echo "✓ Copied index.html"

# Copy pkg directory
cp -r pkg/* deploy/pkg/
echo "✓ Copied pkg/ directory"

# Copy assets if they exist
if [ -d "assets" ] && [ "$(ls -A assets)" ]; then
    cp -r assets/* deploy/assets/
    echo "✓ Copied assets/ directory"
else
    echo "ℹ️  No assets directory to copy"
fi

echo "📁 Files in deploy directory:"
find deploy -type f -name "*.html" -o -name "*.js" -o -name "*.wasm" | sort

echo "✅ Files prepared for deployment!"
echo ""
echo "Now you can deploy using:"
echo "deployctl deploy --project=your-project-name deploy/deno_deploy.ts"
