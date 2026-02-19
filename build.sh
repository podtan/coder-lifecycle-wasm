#!/bin/bash
set -e

# Build script for coder-lifecycle-wasm extension
# Compiles to WASM component model format

echo "🔨 Building coder-lifecycle-wasm extension..."

# Add WASM target if not already added
rustup target add wasm32-wasip1 2>/dev/null || true

# Check if wasm-tools is available
if ! command -v wasm-tools &> /dev/null; then
    echo "❌ wasm-tools not found!"
    echo "   Install with: cargo install wasm-tools"
    exit 1
fi

# Build in release mode
echo "🏗️  Compiling to WASM module..."
cargo build --target wasm32-wasip1 --release

# Create output directory
mkdir -p wasm-output

# Convert WASM module to component
echo "🔄 Converting WASM module to component..."
WASM_MODULE="target/wasm32-wasip1/release/coder_lifecycle_wasm.wasm"
WASM_COMPONENT="wasm-output/coder_lifecycle_wasm.wasm"

# Download WASI adapter if needed
ADAPTER_URL="https://github.com/bytecodealliance/wasmtime/releases/download/v25.0.3/wasi_snapshot_preview1.reactor.wasm"
ADAPTER_FILE="target/wasm32-wasip1/release/wasi_snapshot_preview1.reactor.wasm"

if [ ! -f "$ADAPTER_FILE" ]; then
    echo "📥 Downloading WASI adapter..."
    mkdir -p "$(dirname "$ADAPTER_FILE")"
    curl -L -o "$ADAPTER_FILE" "$ADAPTER_URL"
fi

wasm-tools component new "$WASM_MODULE" --adapt "wasi_snapshot_preview1=$ADAPTER_FILE" -o "$WASM_COMPONENT"

# Also copy extension.toml for distribution
cp extension.toml wasm-output/

echo "✅ Build complete!"
echo "Output: wasm-output/coder_lifecycle_wasm.wasm"

# Show file size
ls -lh wasm-output/coder_lifecycle_wasm.wasm
