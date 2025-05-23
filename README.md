# Dioxus 0.6 Web Demo

This is a simple web application demo built with Dioxus 0.6, deployable to Deno Deploy.

## Features

- Built with Dioxus 0.6
- Responsive user interface
- Simple counter demo
- Optimized Deno Deploy TypeScript server
- Brotli-compressed static assets support

## Local Development

### Prerequisites

- Rust and Cargo
- Dioxus CLI
- Deno (for local Deno Deploy testing)

### Install Dioxus CLI

```bash
cargo install dioxus-cli
```

### Local Run

1. Clone this repository

2. Start the development server

```bash
dx serve --hot-reload
```

This will start the dev server at `http://localhost:8080`.

### Build

To build for production:

```bash
dx build --release
```

## Deploy to Deno Deploy

### One-Click Build

Run the build script:

```bash
bash build.sh
```

This will:
- Build the Dioxus project
- Prepare deployment files (in the `./deploy` directory)
- Automatically copy necessary Deno config files

### Local Test Deployment

```bash
deno task start
```

### Deploy to Deno Deploy

1. Install deployctl:

```bash
deno install --allow-read --allow-write --allow-env --allow-net --allow-run --no-check -f https://deno.land/x/deploy/deployctl.ts
```

2. Log in to Deno Deploy

3. Deploy the project:

```bash
deployctl deploy --project=always-blue-demo --prod ./deno_deploy.ts
```

## Project Structure

- `src/main.rs` - Main application code
- `index.html` - HTML template
- `build.sh` - Build and deployment preparation script
- `deno_deploy.ts` - Deno Deploy server code (TypeScript)
- `deno.json` - Deno configuration
- `deploy.json` - Deployment configuration

## Advanced Features

- **Brotli Compression Support**: Server prefers `.br` compressed files for faster loading
- **Smart Caching**: Long-term cache for static assets
- **SPA Mode Support**: Automatically fallback to index.html for non-resource requests
- **Smart Error Handling**: Friendly 404 error page

## License

MIT
