#!/bin/bash
set -e

# Build script for coder-lifecycle-wasm extension
# Compiles to WASM component model format

echo "Building coder-lifecycle-wasm extension..."

# Add WASM target if not already added
rustup target add wasm32-wasip1 2>/dev/null || true

# Build in release mode
cargo build --target wasm32-wasip1 --release

# Create output directory
mkdir -p wasm-output

# Copy the WASM file
cp target/wasm32-wasip1/release/coder_lifecycle_wasm.wasm wasm-output/

# Also copy extension.toml for distribution
cp extension.toml wasm-output/

echo "Build complete!"
echo "Output: wasm-output/coder_lifecycle_wasm.wasm"

# Show file size
ls -lh wasm-output/coder_lifecycle_wasm.wasm
