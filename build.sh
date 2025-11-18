#!/bin/bash
set -euo pipefail

echo "Building Rust/WASM System Monitor..."

# Check for wasm-pack
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack not found"
    echo "Install with: cargo install wasm-pack"
    exit 1
fi

# Build WASM package
echo "Building WASM package..."
wasm-pack build --target web --out-dir pkg

echo "Build complete!"
echo ""
echo "Output: pkg/"
echo ""
echo "To use:"
echo "  1. Open demo.html in a browser (use a local web server)"
echo "  2. Or import from TypeScript: import { SystemMonitor } from './pkg/rust_wasm_monitor'"
echo ""
echo "Run demo server: python3 -m http.server 8080"
