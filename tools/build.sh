#!/bin/bash
# Build script for bux (Unix)

set -e

TARGET="${1:-release}"
CLEAN="${2:-false}"

if [ "$CLEAN" = "clean" ]; then
    echo "Cleaning build artifacts..."
    cargo clean
fi

echo "Building bux ($TARGET)..."

if [ "$TARGET" = "release" ]; then
    cargo build --release
else
    cargo build
fi

echo "Build complete!"

