# Always Blue

A personal website built with Rust and Slint, featuring a hidden cross-platform memory matching game (Easter Egg) supporting desktop and WebAssembly (WASM) deployment.

[中文文档](README_CN.md)

## 🎮 Introduction

Always Blue is a personal showcase site that includes an ocean-themed memory matching game as an Easter Egg. The project features:

- **8 Cute Ocean Creatures**: Fish, Octopus, Crab, Jellyfish, Starfish, Sea Turtle, Whale, Seahorse
- **Cross-Platform Support**: Desktop application and Web version
- **Audio System**: Background music and sound effects
- **Responsive UI**: Adapted for different screen sizes

## 🚀 Quick Start

### Desktop Version

```bash
# Clone the repository
git clone <repository-url>
cd always-blue

# Build and run (defaults to desktop feature)
cargo run
# Or explicitly specify the feature
cargo run --features desktop
```

### Web Version

```bash
# Build WASM (uses wasm feature)
./build.sh

# Start local server
deno task start
# Or use any other static file server
```

Then visit `http://localhost:8000` in your browser.

## 🛠️ Tech Stack

- **Rust** - Systems programming language
- **Slint** - Declarative UI framework
- **WASM** - WebAssembly support
- **Rodio** - Desktop audio library
- **Web Audio API** - Web audio

## 📁 Project Structure

```
always-blue/
├── src/
│   ├── main.rs          # Desktop entry point
│   ├── lib.rs           # WASM entry point
│   ├── audio.rs         # Cross-platform audio management
│   └── game/            # Game logic
│       ├── mod.rs       # Game initialization
│       └── tiles.rs     # Tile generation
├── ui/
│   ├── main-window.slint # Main window definition
│   ├── app-window.slint  # Game window definition
│   └── icons/           # Game icons
├── raw/                 # Audio resources
├── pkg/                 # WASM build output
└── build.sh             # Build script
```

## 🔧 Build Instructions

### Development Build

```bash
# Desktop version
cargo build --features desktop

# WASM version (local check)
cargo build --features wasm --no-default-features
```

### Release Build

```bash
# Desktop version
cargo build --release --features desktop

# WASM version (using build.sh script)
./build.sh
```

### WASM Build details

```bash
# Full build (includes optimization and compression)
./build.sh

# Or manual build
wasm-pack build --target web --out-dir pkg --release -- --features wasm --no-default-features
```

## 🌐 Deploy to Web

After building, the `pkg` directory contains:

- `always_blue_wasm.js` - JavaScript glue code
- `always_blue_wasm_bg.wasm` - WASM binary file
- `always_blue_wasm.js.br` / `always_blue_wasm_bg.wasm.br` - Brotli compressed versions

Deploy the `pkg` directory, `index.html`, and `raw` directory together to a static file server.

## 🎵 Audio Features

The game includes a complete audio system:

- **Background Music**: Plays while the game is running
- **Match Sound**: Plays when a pair is successfully matched
- **Cross-Platform Support**:
  - Desktop: Uses Rodio library
  - Web: Uses Web Audio API

## 🎨 UI Features

- **Responsive Design**: Adapts to different screen sizes
- **Smooth Animations**: Card flipping and state transitions
- **Theme System**: Blue ocean theme
- **Interactive Feedback**: Hover effects and click feedback

## 📱 Platform Support

- **Desktop**: Windows, macOS, Linux
- **Web**: Modern browsers (Chrome, Firefox, Safari, Edge)
- **Mobile**: Supported via browser

## 🔍 Development Framework

### Adding New Cards

1. Add new PNG icon to `ui/icons/` directory
2. Add new `TileData` entry in `src/game/tiles.rs`
3. Ensure icon path is correct

### Modifying Audio

1. Place audio file in `raw/` directory
2. Update file path in `src/audio.rs`
3. Supported formats: MP3, WAV, OGG

### Customizing Theme

Edit `.slint` files in `ui/` directory to modify:
- Color schemes
- Layout
- Fonts
- Animations

## 🐛 Troubleshooting

### Common Issues

1. **WASM Build Failed**
   - Ensure `wasm-pack` is installed
   - Check Rust toolchain: `rustup target add wasm32-unknown-unknown`

2. **Audio Not Playing**
   - Check audio file paths
   - Web version requires HTTPS or localhost to play audio

3. **Icons Not Showing**
   - Check if icon files exist
   - Confirm file path case correctness

### Debugging

```bash
# Verbose build output
cargo build --verbose --features desktop

# WASM debug build
wasm-pack build --target web --out-dir pkg --dev -- --features wasm --no-default-features

# Check WASM code (without actual WASM build)
cargo check --features wasm --no-default-features
```

## 📄 License

This project is open sourced under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Issues and Pull Requests are welcome!

### Development Guidelines

The project uses features to distinguish between platforms:
- `desktop`: Desktop version features (enabled by default)
- `wasm`: WebAssembly version features

When developing, please note:
1. Use `#[cfg(feature = "desktop")]` and `#[cfg(feature = "wasm")]` for conditional compilation
2. Desktop and WASM versions share most game logic
3. Platform-specific code (like audio, file access) needs separate implementations

### Contribution Steps
1. Fork the project
2. Create a feature branch
3. Commit changes
4. Push to the branch
5. Create a Pull Request

## 🙏 Acknowledgements

- [Slint UI](https://slint.rs/) - Excellent Rust UI framework
- [Rust Community](https://www.rust-lang.org/) - Powerful tools and libraries
- All contributors and users

---

**Ocean world awaits your exploration!** 🌊🐠
