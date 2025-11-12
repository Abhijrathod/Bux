#!/bin/bash
# Linux installer script for bux

set -e

INSTALL_PATH="${HOME}/.local/bin"

echo "Installing bux..."

# Create install directory
mkdir -p "$INSTALL_PATH"

# Build bux
echo "Building bux..."
cargo build --release

# Copy binary
if [ -f "target/release/bux" ]; then
    cp target/release/bux "$INSTALL_PATH/bux"
    chmod +x "$INSTALL_PATH/bux"
    echo "Installed to $INSTALL_PATH/bux"
else
    echo "Binary not found! Build failed."
    exit 1
fi

# Add to PATH if not already there
if [[ ":$PATH:" != *":$INSTALL_PATH:"* ]]; then
    echo "Adding $INSTALL_PATH to PATH..."
    if [ -f ~/.bashrc ]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
        echo "Added to PATH in ~/.bashrc"
    elif [ -f ~/.zshrc ]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
        echo "Added to PATH in ~/.zshrc"
    fi
    echo "Restart your terminal or run: source ~/.bashrc (or ~/.zshrc)"
fi

echo "Installation complete!"

