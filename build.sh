#!/bin/bash
# build.sh - Build and prepare for Deno Deploy

# Exit on error
set -e

echo "===== Building Dioxus project ====="

# Check if dioxus-cli is installed
if ! command -v dx &> /dev/null; then
    echo "Installing dioxus-cli..."
    cargo install dioxus-cli
fi

# Build the project
echo "Building project..."
dx build --release

echo "===== Preparing Deno Deploy files ====="

# Create deploy directory
DEPLOY_DIR="./deploy"
rm -rf $DEPLOY_DIR
mkdir -p $DEPLOY_DIR

# Copy build artifacts
echo "Copying build artifacts..."
BUILD_PATH="./target/dx/always-blue/release/web/public"
if [ -d "$BUILD_PATH" ]; then
    cp -r $BUILD_PATH/* $DEPLOY_DIR/
else
    echo "Error: Build directory $BUILD_PATH does not exist"
    exit 1
fi

# Copy Deno config files
echo "Copying Deno config files..."
cp deno_deploy.ts $DEPLOY_DIR/
cp deno.json $DEPLOY_DIR/

# Copy 404 error page
if [ -f "deploy/404.html" ]; then
    cp deploy/404.html $DEPLOY_DIR/
else
    echo "Creating custom 404 page..."
    cat > $DEPLOY_DIR/404.html << 'EOL'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 - Page Not Found</title>
    <style>
        body {
            font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100vh;
            margin: 0;
            background-color: #f8f9fa;
            color: #333;
            text-align: center;
        }
        .container {
            max-width: 600px;
            padding: 2rem;
            background-color: white;
            border-radius: 8px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }
        h1 {
            font-size: 2.5rem;
            margin-bottom: 1rem;
            color: #3b82f6;
        }
        p {
            font-size: 1.2rem;
            margin-bottom: 1.5rem;
            line-height: 1.6;
        }
        .btn {
            display: inline-block;
            padding: 0.75rem 1.5rem;
            background-color: #3b82f6;
            color: white;
            text-decoration: none;
            border-radius: 4px;
            font-weight: 600;
            transition: background-color 0.3s;
        }
        .btn:hover {
            background-color: #2563eb;
        }
        .error-code {
            font-size: 1.25rem;
            color: #6b7280;
            margin-top: 1.5rem;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>Page Not Found</h1>
        <p>Sorry, the page you are looking for does not exist or has been removed.</p>
        <a href="/" class="btn">Back to Home</a>
        <div class="error-code">Error Code: 404</div>
    </div>
</body>
</html>
EOL
fi

# Try to create compressed versions for static assets
echo "Compressing static assets..."
if command -v brotli &> /dev/null; then
    # Compress JS files
    find $DEPLOY_DIR -name "*.js" -type f -exec brotli -f {} \;
    # Compress CSS files
    find $DEPLOY_DIR -name "*.css" -type f -exec brotli -f {} \;
    # Compress HTML files (except index.html, which is handled dynamically)
    find $DEPLOY_DIR -name "*.html" -not -name "index.html" -type f -exec brotli -f {} \;
    echo "Asset compression complete"
else
    echo "Warning: brotli compression tool not found, skipping compression step"
    echo "Tip: Install with 'brew install brotli' or your system's package manager"
fi

echo "===== Build complete ====="
echo "Deployment files are ready in $DEPLOY_DIR directory"
echo "You can test locally with:"
echo "cd $DEPLOY_DIR && deno task start"
echo ""
echo "To deploy to Deno Deploy, use:"
echo "deployctl deploy --project=always-blue-demo --prod $DEPLOY_DIR/deno_deploy.ts"
